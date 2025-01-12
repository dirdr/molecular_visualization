use std::fs;

use glium::{
    glutin::surface::WindowSurface, implement_vertex, index::PrimitiveType, program, IndexBuffer,
    Program, VertexBuffer,
};
use nalgebra::{Point3, Point4};

use crate::geometry::quad::{Quad, QuadVertex};

#[derive(Copy, Clone, Debug)]
pub struct CylinderInstanceData {
    pub instance_start_pos: [f32; 3],
    pub instance_end_pos: [f32; 3],
    pub instance_color: [f32; 4],
    pub instance_radius: f32,

    pub original_start_pos: Point3<f32>,
    pub original_end_pos: Point3<f32>,
}

implement_vertex!(
    CylinderInstanceData,
    instance_start_pos,
    instance_end_pos,
    instance_color,
    instance_radius
);

pub struct CylinderBatch {
    pub vertex_buffer: VertexBuffer<QuadVertex>,
    pub index_buffer: IndexBuffer<u16>,
    pub instance_buffer: VertexBuffer<CylinderInstanceData>,
    pub instances: Vec<CylinderInstanceData>,
}

impl CylinderInstanceData {
    pub fn new(
        start_pos: Point3<f32>,
        end_pos: Point3<f32>,
        color: Point4<f32>,
        radius: f32,
    ) -> Self {
        Self {
            instance_start_pos: start_pos.into(),
            instance_end_pos: end_pos.into(),
            instance_color: color.into(),
            instance_radius: radius,
            original_start_pos: start_pos,
            original_end_pos: end_pos,
        }
    }
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
        let vertex_shader = fs::read_to_string("./resources/shaders/cylinder_imposter.vert")?;
        let fragment_shader = fs::read_to_string("./resources/shaders/cylinder_imposter.frag")?;

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
