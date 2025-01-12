use glium::glutin::surface::WindowSurface;
use nalgebra::{Matrix4, Point3, Point4};

use crate::sphere_batch::{SphereBatch, SphereInstanceData};

pub struct Molecule {
    pub atoms: SphereBatch,
}

impl Molecule {
    pub fn initialize_instances(display: &glium::Display<WindowSurface>) -> anyhow::Result<Self> {
        Ok(Self {
            atoms: SphereBatch::new(display)?,
        })
    }

    /// Rotate the molecule model
    pub fn rotate(&mut self, rotation_matrix: Matrix4<f32>) {
        for instance in self.atoms.instances.iter_mut() {
            let rotated = rotation_matrix * instance.original_pos.to_homogeneous();
            instance.instance_pos = [rotated.x, rotated.y, rotated.z];
        }
    }

    pub fn sync_buffers(&mut self, display: &glium::Display<WindowSurface>) -> anyhow::Result<()> {
        self.atoms.sync_buffer(display)?;
        Ok(())
    }

    pub fn init_molecule(&mut self, display: &glium::Display<WindowSurface>) -> anyhow::Result<()> {
        let instances = vec![
            SphereInstanceData::new(
                Point3::new(0.0, 0.0, 0.0),
                Point4::new(1.0, 0.0, 0.0, 1.0),
                0.1,
            ),
            SphereInstanceData::new(
                Point3::new(0.5, 0.0, 0.0),
                Point4::new(0.0, 0.0, 1.0, 1.0),
                0.1,
            ),
            SphereInstanceData::new(
                Point3::new(1.0, 0.0, 1.0),
                Point4::new(0.0, 0.0, 1.0, 1.0),
                0.1,
            ),
        ];

        self.atoms.update_instances(display, &instances)?;
        Ok(())
    }
}
