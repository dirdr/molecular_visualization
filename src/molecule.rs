use glium::glutin::surface::WindowSurface;
use nalgebra::{Matrix4, Point3, Point4};
use pdbtbx::Element;

use crate::{
    cylinder_batch::{CylinderBatch, CylinderInstanceData},
    sphere_batch::{SphereBatch, SphereInstanceData},
};

pub struct Molecule {
    pub atoms: SphereBatch,
    pub bonds: CylinderBatch,
}

impl Molecule {
    pub fn initialize_instances(display: &glium::Display<WindowSurface>) -> anyhow::Result<Self> {
        Ok(Self {
            atoms: SphereBatch::new(display)?,
            bonds: CylinderBatch::new(display)?,
        })
    }

    /// Rotate the molecule model
    pub fn rotate(&mut self, rotation_matrix: Matrix4<f32>) {
        for atom in self.atoms.instances.iter_mut() {
            let rotated = rotation_matrix * atom.original_pos.to_homogeneous();
            atom.instance_pos = [rotated.x, rotated.y, rotated.z];
        }
        for bond in self.bonds.instances.iter_mut() {
            let rotated_start_pos = rotation_matrix * bond.original_start_pos.to_homogeneous();
            let rotated_end_pos = rotation_matrix * bond.original_end_pos.to_homogeneous();

            bond.instance_start_pos = [
                rotated_start_pos.x,
                rotated_start_pos.y,
                rotated_start_pos.z,
            ];
            bond.instance_end_pos = [rotated_end_pos.x, rotated_end_pos.y, rotated_end_pos.z];
        }
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
                let color = match atom.element().unwrap() {
                    Element::C => Point4::new(0.0, 1.0, 0.0, 1.0), // Carbon: green
                    Element::O => Point4::new(1.0, 0.0, 0.0, 1.0), // Oxygen: red
                    Element::H => Point4::new(1.0, 1.0, 1.0, 1.0), // Hydrogen: white
                    Element::N => Point4::new(0.0, 0.0, 1.0, 1.0), // Nitrogen: blue
                    _ => Point4::new(0.5, 0.5, 0.5, 1.0),          // Default: gray
                };
                let radius = match atom.element().unwrap() {
                    Element::C => 0.17,
                    Element::O => 0.15,
                    Element::H => 0.12,
                    Element::N => 0.16,
                    _ => 0.14,
                };

                atom_instances.push(SphereInstanceData::new(position, color, radius));
            }
        }

        let mut bond_instances = Vec::new();
        let bond_radius = 0.1; // Default bond thickness

        for (i, atom1) in pdb.models().flat_map(|m| m.atoms()).enumerate() {
            let pos1 = Point3::new(atom1.x() as f32, atom1.y() as f32, atom1.z() as f32);
            for atom2 in pdb.models().flat_map(|m| m.atoms()).skip(i + 1) {
                let pos2 = Point3::new(atom2.x() as f32, atom2.y() as f32, atom2.z() as f32);
                let distance = (pos2 - pos1).norm();

                let max_bond_distance = 1.6;
                if distance <= max_bond_distance {
                    let bond_color = Point4::new(1.0, 1.0, 0.0, 1.0); // Default bond color (yellow)
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
}
