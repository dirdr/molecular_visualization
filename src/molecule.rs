use glium::glutin::surface::WindowSurface;

use crate::sphere_batch::{SphereBatch, SphereInstanceData};

pub struct Molecule {
    pub sphere_instances: SphereBatch,
}

impl Molecule {
    pub fn initialize_instances(display: &glium::Display<WindowSurface>) -> anyhow::Result<Self> {
        Ok(Self {
            sphere_instances: SphereBatch::new(display)?,
        })
    }

    pub fn init_molecule(&mut self, display: &glium::Display<WindowSurface>) -> anyhow::Result<()> {
        let instances = vec![
            SphereInstanceData {
                instance_pos: [0.0, 0.0, 0.0],
                instance_color: [1.0, 0.0, 0.0, 1.0],
                instance_radius: 1.0,
            },
            SphereInstanceData {
                instance_pos: [2.0, 0.0, 0.0],
                instance_color: [0.0, 0.0, 1.0, 1.0],
                instance_radius: 0.8,
            },
        ];

        self.sphere_instances
            .update_instances(display, &instances)?;
        Ok(())
    }
}
