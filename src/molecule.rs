use glium::glutin::surface::WindowSurface;
use nalgebra::{Matrix4, Point3, Point4};

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
        let atom_instances = vec![
            SphereInstanceData::new(
                Point3::new(0.3, 0.0, 0.0),
                Point4::new(1.0, 0.0, 0.0, 1.0),
                0.1,
            ),
            SphereInstanceData::new(
                Point3::new(0.7, 0.0, 0.0),
                Point4::new(0.0, 0.0, 1.0, 1.0),
                0.1,
            ),
        ];

        let bond_instances = vec![CylinderInstanceData::new(
            Point3::new(0.4, 0.0, 0.0),
            Point3::new(0.65, 0.0, 0.0),
            Point4::new(1.0, 0.0, 1.0, 1.0),
            0.08,
        )];

        self.atoms.update_instances(&atom_instances);
        self.bonds.update_instances(&bond_instances);
        self.sync_buffers(display)?;
        Ok(())
    }
}
