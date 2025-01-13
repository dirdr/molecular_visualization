use glium::glutin::surface::WindowSurface;
use nalgebra::{Matrix4, Point3, Point4, Vector3};
use pdbtbx::{Atom, Element};

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
        let pdb = pdbtbx::open("./resources/pdb/liquid.pdb").unwrap().0;
        let mut atom_instances = Vec::new();

        for model in pdb.models() {
            for atom in model.atoms() {
                let position = Point3::new(atom.x() as f32, atom.y() as f32, atom.z() as f32);
                let color = Self::atom_color(atom);
                let radius = Self::atom_size(atom);
                atom_instances.push(SphereInstanceData::new(position, color, radius));
            }
        }

        // HACK THIS IS NOT THE MOELCULE REALITY!!! AD-HOC CODE TO SHOW THE BOND IN THE SIMULATIOn
        let bond_radius = 0.1;
        let mut bond_instances = vec![];
        for (i, atom1) in pdb.models().flat_map(|m| m.atoms()).enumerate() {
            let pos1 = Point3::new(atom1.x() as f32, atom1.y() as f32, atom1.z() as f32);
            for atom2 in pdb.models().flat_map(|m| m.atoms()).skip(i + 1) {
                let pos2 = Point3::new(atom2.x() as f32, atom2.y() as f32, atom2.z() as f32);
                let distance = (pos2 - pos1).norm();

                let max_bond_distance = 1.0;
                if distance <= max_bond_distance {
                    let bond_color = Point4::new(0.65, 0.65, 0.65, 1.0);
                    bond_instances.push(CylinderInstanceData::new(
                        pos1,
                        pos2,
                        bond_color,
                        bond_radius,
                    ));
                }
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
