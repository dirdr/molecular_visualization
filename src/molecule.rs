use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use glium::glutin::surface::WindowSurface;
use nalgebra::{Matrix4, Point3, Point4};
use pdbtbx::{Atom, Element, PDB};

use crate::{
    cylinder_batch::{CylinderBatch, CylinderInstanceData},
    geometry::{Model, Rotate, Scale, Translate},
    sphere_batch::{SphereBatch, SphereInstanceData},
    ARGS,
};

pub struct Molecule {
    pub atoms: SphereBatch,
    pub bonds: CylinderBatch,
    model_matrix: Matrix4<f32>,
    pub show_silhouette: bool,
    pub scale_factor: f32,
}

impl Molecule {
    pub fn initialize_instances(display: &glium::Display<WindowSurface>) -> anyhow::Result<Self> {
        Ok(Self {
            atoms: SphereBatch::new(display)?,
            bonds: CylinderBatch::new(display)?,
            model_matrix: Matrix4::<f32>::identity(),
            show_silhouette: false,
            scale_factor: 1.0,
        })
    }

    pub fn sync_buffers(&mut self, display: &glium::Display<WindowSurface>) -> anyhow::Result<()> {
        self.atoms.sync_buffer(display)?;
        self.bonds.sync_buffer(display)?;
        Ok(())
    }

    pub fn init_molecule(&mut self) -> anyhow::Result<()> {
        let mut atom_map = HashMap::new();
        let filename = format!("./resources/pdb/{}", &ARGS.file);

        let pdb = pdbtbx::open(&filename).unwrap().0;

        let bonds = parse_bonds(&filename)?;
        let molecule_center = Self::calculate_molecule_center(&pdb);
        let bounding_box = pdb.bounding_box();
        let bottom_left = Point3::<f32>::new(
            bounding_box.0 .0 as f32,
            bounding_box.0 .1 as f32,
            bounding_box.0 .2 as f32,
        );

        let top_right = Point3::<f32>::new(
            bounding_box.1 .0 as f32,
            bounding_box.1 .1 as f32,
            bounding_box.1 .2 as f32,
        );

        let dimension = top_right - bottom_left;
        // Get maximum dimension
        let max_dimension = dimension.x.max(dimension.y).max(dimension.z);

        // Define your target size (how big you want the molecule to appear in your scene)
        let target_size = 5.0; // Adjust this value based on your needs

        // Calculate scale factor
        let scale_factor = if max_dimension > target_size {
            target_size / max_dimension
        } else {
            1.0 // Don't scale up if molecule is smaller than target
        };

        self.scale_factor = scale_factor;

        let atom_instances = Self::create_atom_instances(&pdb, &mut atom_map, molecule_center);
        let cylinder_instances = Self::create_bond_instances(&bonds, &atom_map, molecule_center);

        self.atoms.update_instances(&atom_instances);
        self.bonds.update_instances(&cylinder_instances);

        Ok(())
    }

    fn create_atom_instances<'a>(
        pdb: &'a PDB,
        atom_map: &mut HashMap<usize, &'a Atom>,
        molecule_center: Point3<f32>,
    ) -> Vec<SphereInstanceData> {
        let mut atom_instances = Vec::new();

        for model in pdb.models() {
            for atom in model.atoms() {
                atom_map.insert(atom.serial_number(), atom);

                let position = Point3::new(
                    atom.x() as f32 - molecule_center.x,
                    atom.y() as f32 - molecule_center.y,
                    atom.z() as f32 - molecule_center.z,
                );
                let color = Self::atom_color(atom);
                let radius = Self::atom_size(atom);
                atom_instances.push(SphereInstanceData::new(position, color, radius));
            }
        }
        atom_instances
    }

    fn create_bond_instances<'a>(
        bonds: &[ConectRecord],
        atom_map: &'a HashMap<usize, &'a Atom>,
        molecule_center: Point3<f32>,
    ) -> Vec<CylinderInstanceData> {
        let mut cylinder_instances = vec![];
        let mut already_connected = HashSet::new();

        for bond in bonds {
            let start = match atom_map.get(&(bond.source_atom)) {
                Some(atom) => atom,
                None => continue,
            };

            for &connected in &bond.bonded_atoms {
                let end = match atom_map.get(&(connected)) {
                    Some(atom) => atom,
                    None => continue,
                };

                if already_connected.contains(&(start.serial_number(), end.serial_number()))
                    || already_connected.contains(&(end.serial_number(), start.serial_number()))
                {
                    continue;
                }

                let start_pos = [
                    start.x() as f32 - molecule_center.x,
                    start.y() as f32 - molecule_center.y,
                    start.z() as f32 - molecule_center.z,
                ];
                let end_pos = [
                    end.x() as f32 - molecule_center.x,
                    end.y() as f32 - molecule_center.y,
                    end.z() as f32 - molecule_center.z,
                ];

                cylinder_instances.push(CylinderInstanceData {
                    instance_start_pos: start_pos,
                    instance_end_pos: end_pos,
                    instance_color_first_half: Self::atom_color(start).into(),
                    instance_color_second_half: Self::atom_color(end).into(),
                    instance_radius: 0.15,
                });

                already_connected.insert((start.serial_number(), end.serial_number()));
            }
        }
        cylinder_instances
    }

    pub fn toggle_silhouette(&mut self) {
        self.show_silhouette = !self.show_silhouette;
    }

    /// Take a reference to a `Atom` and return a normalized RGBA color
    /// according to the CPK coloring.
    fn atom_color(atom: &Atom) -> Point4<f32> {
        match atom.element().unwrap() {
            Element::H => Point4::new(1.0, 1.0, 1.0, 1.0), // 255/255
            Element::He => Point4::new(0.851, 1.0, 1.0, 1.0), // 217/255
            Element::Li => Point4::new(0.8, 0.502, 1.0, 1.0), // 204/255, 128/255
            Element::Be => Point4::new(0.761, 1.0, 0.0, 1.0), // 194/255
            Element::B => Point4::new(1.0, 0.710, 0.710, 1.0), // 181/255
            Element::C => Point4::new(0.565, 0.565, 0.565, 1.0), // 144/255
            Element::N => Point4::new(0.188, 0.314, 0.973, 1.0), // 48/255, 80/255, 248/255
            Element::O => Point4::new(1.0, 0.051, 0.051, 1.0), // 13/255
            Element::F => Point4::new(0.565, 0.878, 0.314, 1.0), // 144/255, 224/255, 80/255
            Element::Ne => Point4::new(0.702, 0.890, 0.961, 1.0), // 179/255, 227/255, 245/255
            Element::Na => Point4::new(0.671, 0.361, 0.949, 1.0), // 171/255, 92/255, 242/255
            Element::Mg => Point4::new(0.541, 1.0, 0.0, 1.0), // 138/255
            Element::Al => Point4::new(0.749, 0.651, 0.651, 1.0), // 191/255, 166/255
            Element::Si => Point4::new(0.941, 0.784, 0.627, 1.0), // 240/255, 200/255, 160/255
            Element::P => Point4::new(1.0, 0.502, 0.0, 1.0), // 128/255
            Element::S => Point4::new(1.0, 1.0, 0.188, 1.0), // 48/255
            Element::Cl => Point4::new(0.122, 0.941, 0.122, 1.0), // 31/255, 240/255
            Element::Ar => Point4::new(0.502, 0.820, 0.890, 1.0), // 128/255, 209/255, 227/255
            Element::K => Point4::new(0.561, 0.251, 0.831, 1.0), // 143/255, 64/255, 212/255
            Element::Ca => Point4::new(0.239, 1.0, 0.0, 1.0), // 61/255
            Element::Fe => Point4::new(0.878, 0.400, 0.200, 1.0), // 224/255, 102/255, 51/255
            Element::Cu => Point4::new(0.784, 0.502, 0.200, 1.0), // 200/255, 128/255, 51/255
            Element::Zn => Point4::new(0.490, 0.502, 0.690, 1.0), // 125/255, 128/255, 176/255
            Element::Br => Point4::new(0.651, 0.161, 0.161, 1.0), // 166/255, 41/255
            Element::Ag => Point4::new(0.753, 0.753, 0.753, 1.0), // 192/255
            Element::I => Point4::new(0.580, 0.0, 0.580, 1.0), // 148/255
            Element::Au => Point4::new(1.0, 0.820, 0.137, 1.0), // 255/255, 209/255, 35/255
            Element::Pb => Point4::new(0.341, 0.349, 0.380, 1.0), // 87/255, 89/255, 97/255
            Element::U => Point4::new(0.0, 0.561, 1.0, 1.0), // 0, 143/255, 255/255
            _ => Point4::new(1.0, 0.078, 0.576, 1.0),      // 255/255, 20/255, 147/255 (pink)
        }
    }

    /// Assign a normalized atomic size based on the CPK radii.
    /// The sizes are scaled to fit OpenGL rendering.
    fn atom_size(atom: &Atom) -> f32 {
        let cpk_radii = match atom.element().unwrap() {
            Element::H => 1.20,
            Element::C => 1.70,
            Element::N => 1.55,
            Element::O => 1.52,
            Element::F => 1.47,
            Element::Cl => 1.75,
            Element::Br => 1.85,
            Element::I => 1.98,
            Element::He => 1.40,
            Element::Ne => 1.54,
            Element::Ar => 1.88,
            Element::P => 1.80,
            Element::S => 1.80,
            Element::B => 2.00,
            Element::Li => 1.82,
            Element::Na => 2.27,
            Element::K => 2.75,
            Element::Rb => 3.03,
            Element::Cs => 3.43,
            Element::Fr => 3.48,
            Element::Be => 1.53,
            Element::Mg => 1.73,
            Element::Ca => 2.31,
            Element::Sr => 2.49,
            Element::Ba => 2.68,
            Element::Ra => 2.83,
            Element::Ti => 1.60,
            Element::Fe => 1.52,
            _ => 1.75,
        };

        let scale: f32 = 0.25;

        cpk_radii as f32 * scale
    }

    pub fn calculate_molecule_center(pdb: &PDB) -> Point3<f32> {
        let mut total_x = 0.0;
        let mut total_y = 0.0;
        let mut total_z = 0.0;
        let mut atom_count = 0;

        for model in pdb.models() {
            for atom in model.atoms() {
                total_x += atom.x() as f32;
                total_y += atom.y() as f32;
                total_z += atom.z() as f32;
                atom_count += 1;
            }
        }

        if atom_count > 0 {
            return Point3::new(
                total_x / atom_count as f32,
                total_y / atom_count as f32,
                total_z / atom_count as f32,
            );
        }

        Point3::origin()
    }
}

impl Rotate for Molecule {
    fn rotate(&mut self, rotation_matrix: Matrix4<f32>) {
        self.model_matrix = rotation_matrix * self.model_matrix;
    }
}

impl Scale for Molecule {
    fn scale(&mut self, scale_matrix: Matrix4<f32>) {
        self.model_matrix = scale_matrix * self.model_matrix;
    }
}

impl Translate for Molecule {
    fn translate(&mut self, translate_matrix: Matrix4<f32>) {
        self.model_matrix = translate_matrix * self.model_matrix;
    }
}

impl Model for Molecule {
    fn model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix
    }

    fn reset_model_matrix(&mut self) {
        self.model_matrix = Matrix4::<f32>::identity();
    }
}

/// Represents a CONECT record from a PDB file
#[derive(Debug, Clone)]
struct ConectRecord {
    source_atom: usize,
    bonded_atoms: Vec<usize>,
}

impl ConectRecord {
    /// Parse a CONECT line into a ConectRecord
    fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // CONECT records must have at least 2 numbers
        if parts.len() < 3 || !parts[0].starts_with("CONECT") {
            return None;
        }

        // Parse source atom
        let source_atom = parts[1].parse().ok()?;

        // Parse bonded atoms
        let bonded_atoms = parts[2..]
            .iter()
            .filter_map(|s| s.parse().ok())
            .filter(|&num| num != source_atom) // Avoid self-bonds
            .collect();

        Some(ConectRecord {
            source_atom,
            bonded_atoms,
        })
    }
}

fn parse_bonds(file_path: &str) -> anyhow::Result<Vec<ConectRecord>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let bonds = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| ConectRecord::from_line(&line))
        .collect::<Vec<_>>();

    Ok(bonds)
}
