use glium::glutin::surface::WindowSurface;
use nalgebra::Matrix4;

use crate::{
    cylinder_batch::CylinderBatch,
    geometry::{Model, Rotate, Scale, Translate},
    sphere_batch::SphereBatch,
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
        let text = std::fs::read_to_string(&ARGS.file)?;
        let data = crate::molecule_data::MoleculeData::from_pdb(&text)?;
        self.scale_factor = data.scale_factor;
        self.atoms.update_instances(data.atoms);
        self.bonds.update_instances(data.bonds);
        Ok(())
    }

    pub fn toggle_silhouette(&mut self) {
        self.show_silhouette = !self.show_silhouette;
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
