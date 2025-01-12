use std::fs;

use glium::{
    glutin::surface::WindowSurface, implement_vertex, index::PrimitiveType, program, IndexBuffer,
    Program, VertexBuffer,
};

use crate::geometry::quad::{Quad, QuadVertex};

/// This struct hold a instancing imposter sphere batch informations.
/// `vertex_buffer` hold the quad geometry for the sphere imposter, and `index_buffer` contains the
/// quad indices describing the quad triangle decomposition, see `Quad` static accessor.
///
/// `instance_buffer` contains the **per_instance** data, for each of the sphere imposer.
pub struct SphereBatch {
    pub vertex_buffer: VertexBuffer<QuadVertex>,
    pub index_buffer: IndexBuffer<u16>,
    pub instance_buffer: VertexBuffer<SphereInstanceData>,
}

/// Sphere imposter instance data,
/// The instances can have different variant of each of the field in this struct.
#[derive(Copy, Clone, Debug)]
pub struct SphereInstanceData {
    pub instance_pos: [f32; 3],
    pub instance_color: [f32; 4],
    pub instance_radius: f32,
}

implement_vertex!(
    SphereInstanceData,
    instance_pos,
    instance_color,
    instance_radius
);

impl SphereBatch {
    pub fn new(display: &glium::Display<WindowSurface>) -> anyhow::Result<Self> {
        let vertices = Quad::get_vertices_vertices();
        let indices = Quad::get_billboard_indices();

        Ok(Self {
            vertex_buffer: VertexBuffer::new(display, &vertices)?,
            index_buffer: IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices)?,
            instance_buffer: VertexBuffer::empty_dynamic(display, 0)?,
        })
    }

    pub fn update_instances(
        &mut self,
        display: &glium::Display<WindowSurface>,
        instances: &[SphereInstanceData],
    ) -> anyhow::Result<()> {
        self.instance_buffer = VertexBuffer::dynamic(display, instances)?;
        Ok(())
    }

    /// Build the sphere imposter GLSL Program and return it.
    pub fn build_program(display: &glium::Display<WindowSurface>) -> anyhow::Result<Program> {
        let vertex_shader = fs::read_to_string("./resources/shaders/sphere_imposter.vert")?;
        let fragment_shader = fs::read_to_string("./resources/shaders/sphere_imposter.frag")?;

        let program = program!(display,
            410 => {
                vertex: &vertex_shader,
                fragment: &fragment_shader,
            },
        )?;

        Ok(program)
    }
}
