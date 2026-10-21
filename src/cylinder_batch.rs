use glium::{
    glutin::surface::WindowSurface, index::PrimitiveType, program, IndexBuffer, Program,
    VertexBuffer,
};

pub use crate::instance::CylinderInstanceData;

use crate::geometry::quad::{Quad, QuadVertex};

/// This struct hold a instancing imposter cylinder batch informations.
/// `vertex_buffer` hold the quad geometry for the imposter, and `index_buffer` contains the
/// quad indices describing the quad triangle decomposition, see `Quad` static accessor.
///
/// `instance_buffer` contains the **per_instance** data, for each of the imposer.
pub struct CylinderBatch {
    pub vertex_buffer: VertexBuffer<QuadVertex>,
    pub index_buffer: IndexBuffer<u16>,
    pub instance_buffer: VertexBuffer<CylinderInstanceData>,
    pub instances: Vec<CylinderInstanceData>,
}

impl CylinderBatch {
    pub fn new(display: &glium::Display<WindowSurface>) -> anyhow::Result<Self> {
        let vertices = Quad::get_vertices_vertices();
        let indices = Quad::get_billboard_indices();

        Ok(Self {
            vertex_buffer: VertexBuffer::new(display, &vertices)?,
            index_buffer: IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices)?,
            instance_buffer: VertexBuffer::empty_dynamic(display, 0)?,
            instances: Vec::new(),
        })
    }

    pub fn update_instances(&mut self, instances: &[CylinderInstanceData]) {
        self.instances = instances.to_vec();
    }

    pub fn get_instance(&self, index: usize) -> Option<&CylinderInstanceData> {
        self.instances.get(index)
    }

    pub fn get_instance_mut(&mut self, index: usize) -> Option<&mut CylinderInstanceData> {
        self.instances.get_mut(index)
    }

    pub fn sync_buffer(&mut self, display: &glium::Display<WindowSurface>) -> anyhow::Result<()> {
        self.instance_buffer = VertexBuffer::dynamic(display, &self.instances)?;
        Ok(())
    }

    /// Build the cylinder imposter GLSL Program and return it.
    pub fn build_program(display: &glium::Display<WindowSurface>) -> anyhow::Result<Program> {
        let vertex_shader = crate::shaders::CYLINDER_VERTEX;
        let fragment_shader = crate::shaders::CYLINDER_FRAGMENT;
        let program = program!(display,
            410 => {
                vertex: vertex_shader,
                fragment: fragment_shader,
            },
        )?;
        Ok(program)
    }
}
