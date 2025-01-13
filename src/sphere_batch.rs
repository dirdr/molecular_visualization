use std::fs;

use glium::{
    glutin::surface::WindowSurface, implement_vertex, index::PrimitiveType, program, IndexBuffer,
    Program, VertexBuffer,
};
use nalgebra::{Point3, Point4};

use crate::geometry::quad::{Quad, QuadVertex};

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

/// Sphere imposter instance data,
/// To update the instances in the application, you must call `update_instances` to synchronize the
/// instancing buffer.
#[derive(Copy, Clone, Debug)]
pub struct SphereInstanceData {
    pub instance_pos: [f32; 3],
    pub instance_color: [f32; 4],
    pub instance_radius: f32,
}

impl SphereInstanceData {
    pub fn new(pos: Point3<f32>, color: Point4<f32>, radius: f32) -> Self {
        Self {
            instance_pos: pos.into(),
            instance_color: color.into(),
            instance_radius: radius,
        }
    }
}

// Implement the `glium` `Vertex` trait, effectively biding
// The SphereInstanceData specified fields to the vertex shader.
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
            instances: Vec::new(),
        })
    }

    pub fn update_instances(&mut self, instances: &[SphereInstanceData]) {
        self.instances = instances.to_vec();
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
        let vertex_shader = fs::read_to_string("./resources/shaders/sphere_imposter.vert")?;
        let fragment_shader = fs::read_to_string("./resources/shaders/sphere_imposter.frag")?;
        if vertex_shader.is_empty() || fragment_shader.is_empty() {
            return Err(anyhow::format_err!(
                "Fragment or Vertex shader file are empty"
            ));
        }
        let program = program!(display,
            410 => {
                vertex: &vertex_shader,
                fragment: &fragment_shader,
            },
        )?;
        Ok(program)
    }
}
