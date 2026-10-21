use glium::{
    glutin::surface::WindowSurface, index::PrimitiveType, program, IndexBuffer, Program,
    VertexBuffer,
};

use crate::{
    geometry::quad::{Quad, QuadVertex},
    instance::SphereInstanceData,
};

/// This struct hold a instancing imposter sphere batch informations.
/// `vertex_buffer` hold the quad geometry for the sphere imposter, and `index_buffer` contains the
/// quad indices describing the quad triangle decomposition, see `Quad` static accessor.
///
/// `instance_buffer` contains the **per_instance** data, for each of the imposer.
pub struct SphereBatch {
    pub vertex_buffer: VertexBuffer<QuadVertex>,
    pub index_buffer: IndexBuffer<u16>,
    pub instance_buffer: VertexBuffer<SphereInstanceData>,
    pub instances: Vec<SphereInstanceData>,
}

impl SphereBatch {
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

    pub fn update_instances(&mut self, instances: Vec<SphereInstanceData>) {
        self.instances = instances;
    }

    pub fn get_instance(&self, index: usize) -> Option<&SphereInstanceData> {
        self.instances.get(index)
    }

    pub fn get_instance_mut(&mut self, index: usize) -> Option<&mut SphereInstanceData> {
        self.instances.get_mut(index)
    }

    pub fn sync_buffer(&mut self, display: &glium::Display<WindowSurface>) -> anyhow::Result<()> {
        self.instance_buffer = VertexBuffer::dynamic(display, &self.instances)?;
        Ok(())
    }

    /// Build the sphere imposter GLSL Program and return it.
    pub fn build_program(display: &glium::Display<WindowSurface>) -> anyhow::Result<Program> {
        let vertex_shader = crate::shaders::SPHERE_VERTEX;
        let fragment_shader = crate::shaders::SPHERE_FRAGMENT;
        let program = program!(display,
            410 => {
                vertex: vertex_shader,
                fragment: fragment_shader,
            },
        )?;
        Ok(program)
    }
}
