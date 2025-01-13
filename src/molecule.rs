use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

use glium::glutin::surface::WindowSurface;
use nalgebra::{Matrix4, Point3, Point4, Vector3};
use pdbtbx::{Atom, Bond, Element, PDB};

use crate::{
    cylinder_batch::{CylinderBatch, CylinderInstanceData},
    sphere_batch::{SphereBatch, SphereInstanceData},
};

/// A molecule that is capable of applying a rotation matrix on the CPU
pub trait Rotate {
    fn rotate(&mut self, rotation_matrix: Matrix4<f32>);
}

/// A molecule that is capable of applying a scaling matrix on the CPU
pub trait Scale {
    fn scale(&mut self, scale_matrix: Matrix4<f32>);
}

/// A molecule that is capable of applying a translation matrix on the CPU
pub trait Translate {
    fn translate(&mut self, translate_matrix: Matrix4<f32>);
}

pub trait Model {
    fn model_matrix(&self) -> Matrix4<f32>;

    fn reset_model_matrix(&mut self);
}

pub struct Molecule {
    pub atoms: SphereBatch,
    pub bonds: CylinderBatch,
    model_matrix: Matrix4<f32>,
}

impl Molecule {
    pub fn initialize_instances(display: &glium::Display<WindowSurface>) -> anyhow::Result<Self> {
        Ok(Self {
            atoms: SphereBatch::new(display)?,
            bonds: CylinderBatch::new(display)?,
            model_matrix: Matrix4::<f32>::identity(),
        })
    }

    pub fn sync_buffers(&mut self, display: &glium::Display<WindowSurface>) -> anyhow::Result<()> {
        self.atoms.sync_buffer(display)?;
        self.bonds.sync_buffer(display)?;
        Ok(())
    }

    pub fn init_molecule(&mut self, display: &glium::Display<WindowSurface>) -> anyhow::Result<()> {
        let mut pdb = pdbtbx::open("./resources/pdb/c2h5oh.pdb").unwrap().0;
        let mut atom_instances = Vec::new();

        for model in pdb.models() {
            println!("{:?}", model);
            for atom in model.atoms() {
                let position = Point3::new(atom.x() as f32, atom.y() as f32, atom.z() as f32);
                let color = Self::atom_color(atom);
                let radius = Self::atom_size(atom);
                atom_instances.push(SphereInstanceData::new(position, color, radius));
            }
        }

        let bonds = parse_bonds("./resources/pdb/c2h5oh.pdb")?;
        let mut bond_instances = vec![];
        let mut already_connected = HashSet::new();

        for bond in bonds {
            println!(
                "Atom : {:?}, connecté aux atomes : {:?}",
                bond.source_atom, bond.bonded_atoms,
            );
            let start = pdb.atom(bond.source_atom);
            if start.is_none() {
                continue;
            }
            let start = start.unwrap();
            for connected in bond.bonded_atoms {
                let end = pdb.atom(connected);

                if end.is_none() {
                    continue;
                }

                let end = end.unwrap();

                let start_pos = [start.x() as f32, start.y() as f32, start.z() as f32];
                let end_pos = [end.x() as f32, end.y() as f32, end.z() as f32];

                let bond_color = [0.8, 0.8, 0.8, 1.0];
                let bond_radius = 0.1;

                if already_connected.contains(&(start.serial_number(), end.serial_number()))
                    || already_connected.contains(&(end.serial_number(), start.serial_number()))
                {
                    continue;
                }

                bond_instances.push(CylinderInstanceData {
                    instance_start_pos: start_pos,
                    instance_end_pos: end_pos,
                    instance_color: bond_color,
                    instance_radius: bond_radius,
                });
                already_connected.insert((start.serial_number(), end.serial_number()));
            }
        }
        self.atoms.update_instances(&atom_instances);
        self.bonds.update_instances(&bond_instances);
        self.sync_buffers(display)?;
        Ok(())
    }

    /// Take a reference to a `Atom` and return a normalized RGBA color
    /// according to the CPK coloring.
    fn atom_color(atom: &Atom) -> Point4<f32> {
        match atom.element().unwrap() {
            Element::H => Point4::new(1.0, 1.0, 1.0, 1.0),
            Element::C => Point4::new(0.0, 0.0, 0.0, 1.0),
            Element::N => Point4::new(0.0, 0.0, 1.0, 1.0),
            Element::O => Point4::new(1.0, 0.0, 0.0, 1.0),
            Element::F | Element::Cl => Point4::new(0.0, 1.0, 0.0, 1.0),
            Element::Br => Point4::new(0.6, 0.0, 0.0, 1.0),
            Element::I => Point4::new(0.5, 0.0, 0.5, 1.0),
            Element::He | Element::Ne | Element::Ar | Element::Kr | Element::Xe | Element::Rn => {
                Point4::new(0.0, 1.0, 1.0, 1.0)
            }
            Element::P => Point4::new(1.0, 0.5, 0.0, 1.0),
            Element::S => Point4::new(1.0, 1.0, 0.0, 1.0),
            Element::B => Point4::new(0.9, 0.8, 0.6, 1.0),
            Element::Li | Element::Na | Element::K | Element::Rb | Element::Cs | Element::Fr => {
                Point4::new(0.5, 0.0, 0.5, 1.0)
            }
            Element::Be | Element::Mg | Element::Ca | Element::Sr | Element::Ba | Element::Ra => {
                Point4::new(0.0, 0.5, 0.0, 1.0)
            }
            Element::Ti => Point4::new(0.5, 0.5, 0.5, 1.0),
            Element::Fe => Point4::new(0.8, 0.4, 0.0, 1.0),
            _ => Point4::new(1.0, 0.5, 0.8, 1.0),
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

        // Scaling factor to convert Angstroms to our rendering scale
        let scale: f32 = 0.2;

        cpk_radii as f32 * scale
    }
}

impl Rotate for Molecule {
    fn rotate(&mut self, rotation_matrix: Matrix4<f32>) {
        self.model_matrix = rotation_matrix * self.model_matrix;
    }
}

impl Scale for Molecule {
    /// Apply the `scale_matrix` to all the atoms and bonds of `self`.
    fn scale(&mut self, scale_matrix: Matrix4<f32>) {
        self.model_matrix = scale_matrix * self.model_matrix;
    }
}

impl Translate for Molecule {
    /// Apply the `translate_matrix` to all the atoms and bonds of `self`.
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
#[derive(Debug)]
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

    Ok(reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| ConectRecord::from_line(&line))
        .collect::<Vec<_>>())
}
