use std::{
    collections::{HashMap, HashSet},
    io::BufReader,
};

use crate::instance::{CylinderInstanceData, SphereInstanceData};
use nalgebra::{Point3, Point4};
use pdbtbx::{Atom, Element, Format, ReadOptions, StrictnessLevel, PDB};

pub struct MoleculeData {
    pub atoms: Vec<SphereInstanceData>,
    pub bonds: Vec<CylinderInstanceData>,
    pub scale_factor: f32,
    pub radius: f32,
}

impl MoleculeData {
    pub fn from_pdb(text: &str) -> anyhow::Result<Self> {
        let (pdb, _) = ReadOptions::new()
            .set_format(Format::Pdb)
            .set_level(StrictnessLevel::Loose)
            .read_raw(BufReader::new(text.as_bytes()))
            .map_err(|errors| anyhow::anyhow!("Could not parse PDB: {errors:?}"))?;
        anyhow::ensure!(pdb.atom_count() > 0, "No atoms found in the PDB file");
        let center = Self::calculate_molecule_center(&pdb);
        let mut atom_map = HashMap::new();
        let atoms = Self::create_atom_instances(&pdb, &mut atom_map, center);
        anyhow::ensure!(
            atoms
                .iter()
                .all(|atom| atom.instance_pos.iter().all(|v| v.is_finite())),
            "Non-finite atom coordinates"
        );
        let records: Vec<_> = text.lines().filter_map(ConectRecord::from_line).collect();
        let bonds = Self::create_bond_instances(&records, &atom_map, center);
        let bounds = pdb.bounding_box();
        let dimension = [
            (bounds.1 .0 - bounds.0 .0) as f32,
            (bounds.1 .1 - bounds.0 .1) as f32,
            (bounds.1 .2 - bounds.0 .2) as f32,
        ];
        let max_dimension = dimension.into_iter().fold(0.0_f32, f32::max);
        let scale_factor = if max_dimension > 5.0 {
            5.0 / max_dimension
        } else {
            1.0
        };
        let radius = atoms
            .iter()
            .map(|atom| {
                (nalgebra::Vector3::from(atom.instance_pos).norm() + atom.instance_radius)
                    * scale_factor
            })
            .fold(0.0_f32, f32::max);
        Ok(Self {
            atoms,
            bonds,
            scale_factor,
            radius,
        })
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

    /// Take a reference to a `Atom` and return a normalized RGBA color
    /// according to the CPK coloring.
    fn atom_color(atom: &Atom) -> Point4<f32> {
        match atom.element().copied() {
            Some(Element::H) => Point4::new(1.0, 1.0, 1.0, 1.0), // 255/255
            Some(Element::He) => Point4::new(0.851, 1.0, 1.0, 1.0), // 217/255
            Some(Element::Li) => Point4::new(0.8, 0.502, 1.0, 1.0), // 204/255, 128/255
            Some(Element::Be) => Point4::new(0.761, 1.0, 0.0, 1.0), // 194/255
            Some(Element::B) => Point4::new(1.0, 0.710, 0.710, 1.0), // 181/255
            Some(Element::C) => Point4::new(0.565, 0.565, 0.565, 1.0), // 144/255
            Some(Element::N) => Point4::new(0.188, 0.314, 0.973, 1.0), // 48/255, 80/255, 248/255
            Some(Element::O) => Point4::new(1.0, 0.051, 0.051, 1.0), // 13/255
            Some(Element::F) => Point4::new(0.565, 0.878, 0.314, 1.0), // 144/255, 224/255, 80/255
            Some(Element::Ne) => Point4::new(0.702, 0.890, 0.961, 1.0), // 179/255, 227/255, 245/255
            Some(Element::Na) => Point4::new(0.671, 0.361, 0.949, 1.0), // 171/255, 92/255, 242/255
            Some(Element::Mg) => Point4::new(0.541, 1.0, 0.0, 1.0), // 138/255
            Some(Element::Al) => Point4::new(0.749, 0.651, 0.651, 1.0), // 191/255, 166/255
            Some(Element::Si) => Point4::new(0.941, 0.784, 0.627, 1.0), // 240/255, 200/255, 160/255
            Some(Element::P) => Point4::new(1.0, 0.502, 0.0, 1.0), // 128/255
            Some(Element::S) => Point4::new(1.0, 1.0, 0.188, 1.0), // 48/255
            Some(Element::Cl) => Point4::new(0.122, 0.941, 0.122, 1.0), // 31/255, 240/255
            Some(Element::Ar) => Point4::new(0.502, 0.820, 0.890, 1.0), // 128/255, 209/255, 227/255
            Some(Element::K) => Point4::new(0.561, 0.251, 0.831, 1.0), // 143/255, 64/255, 212/255
            Some(Element::Ca) => Point4::new(0.239, 1.0, 0.0, 1.0), // 61/255
            Some(Element::Fe) => Point4::new(0.878, 0.400, 0.200, 1.0), // 224/255, 102/255, 51/255
            Some(Element::Cu) => Point4::new(0.784, 0.502, 0.200, 1.0), // 200/255, 128/255, 51/255
            Some(Element::Zn) => Point4::new(0.490, 0.502, 0.690, 1.0), // 125/255, 128/255, 176/255
            Some(Element::Br) => Point4::new(0.651, 0.161, 0.161, 1.0), // 166/255, 41/255
            Some(Element::Ag) => Point4::new(0.753, 0.753, 0.753, 1.0), // 192/255
            Some(Element::I) => Point4::new(0.580, 0.0, 0.580, 1.0), // 148/255
            Some(Element::Au) => Point4::new(1.0, 0.820, 0.137, 1.0), // 255/255, 209/255, 35/255
            Some(Element::Pb) => Point4::new(0.341, 0.349, 0.380, 1.0), // 87/255, 89/255, 97/255
            Some(Element::U) => Point4::new(0.0, 0.561, 1.0, 1.0), // 0, 143/255, 255/255
            _ => Point4::new(1.0, 0.078, 0.576, 1.0),            // 255/255, 20/255, 147/255 (pink)
        }
    }

    /// Assign a normalized atomic size based on the CPK radii.
    /// The sizes are scaled to fit OpenGL rendering.
    fn atom_size(atom: &Atom) -> f32 {
        let cpk_radii = match atom.element().copied() {
            Some(Element::H) => 1.20,
            Some(Element::C) => 1.70,
            Some(Element::N) => 1.55,
            Some(Element::O) => 1.52,
            Some(Element::F) => 1.47,
            Some(Element::Cl) => 1.75,
            Some(Element::Br) => 1.85,
            Some(Element::I) => 1.98,
            Some(Element::He) => 1.40,
            Some(Element::Ne) => 1.54,
            Some(Element::Ar) => 1.88,
            Some(Element::P) => 1.80,
            Some(Element::S) => 1.80,
            Some(Element::B) => 2.00,
            Some(Element::Li) => 1.82,
            Some(Element::Na) => 2.27,
            Some(Element::K) => 2.75,
            Some(Element::Rb) => 3.03,
            Some(Element::Cs) => 3.43,
            Some(Element::Fr) => 3.48,
            Some(Element::Be) => 1.53,
            Some(Element::Mg) => 1.73,
            Some(Element::Ca) => 2.31,
            Some(Element::Sr) => 2.49,
            Some(Element::Ba) => 2.68,
            Some(Element::Ra) => 2.83,
            Some(Element::Ti) => 1.60,
            Some(Element::Fe) => 1.52,
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

/// Represents a CONECT record from a PDB file
#[derive(Debug, Clone)]
struct ConectRecord {
    source_atom: usize,
    bonded_atoms: Vec<usize>,
}

impl ConectRecord {
    /// Parse a CONECT line into a ConectRecord
    fn from_line(line: &str) -> Option<Self> {
        if !line.is_ascii() || !line.starts_with("CONECT") {
            return None;
        }
        let fields: Vec<_> = line
            .as_bytes()
            .get(6..)?
            .chunks(5)
            .filter_map(|field| {
                std::str::from_utf8(field)
                    .ok()?
                    .trim()
                    .parse::<usize>()
                    .ok()
            })
            .collect();
        let (&source_atom, bonded) = fields.split_first()?;
        let bonded_atoms = bonded
            .iter()
            .copied()
            .filter(|&id| id != source_atom)
            .collect();

        Some(ConectRecord {
            source_atom,
            bonded_atoms,
        })
    }
}
