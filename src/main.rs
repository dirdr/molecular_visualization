#[macro_use]
extern crate glium;

use std::fs;

use glium::{glutin::surface::WindowSurface, Surface};
use molecular_visualization::{
    backend::{ApplicationContext, State},
    teapot::{self},
    utils::Vertex,
};

struct Application {
    pub vertex_buffer: glium::VertexBuffer<teapot::Vertex>,
    pub normals_buffer: glium::VertexBuffer<teapot::Normal>,
    pub index_buffer: glium::IndexBuffer<u16>,
    pub program: glium::Program,
}

impl ApplicationContext for Application {
    const WINDOW_TITLE: &'static str = "PDB Viewer - Adrien Pelfresne - FIB 2025";

    fn new(display: &glium::Display<WindowSurface>) -> Self {
        let positions = glium::VertexBuffer::new(display, &teapot::VERTICES).unwrap();
        let normals = glium::VertexBuffer::new(display, &teapot::NORMALS).unwrap();
        let indices = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &teapot::INDICES,
        )
        .unwrap();

        let fragment_shader = fs::read_to_string("./resources/shaders/gouraud.frag")
            .expect("Failed to read fragment shader");

        let program = program!(display,
            410 => {
                vertex: "
                    #version 410 core

                    in vec3 position;
                    in vec3 normal;

                    out vec3 v_normal;

                    uniform mat4 model;

                    void main() {
                        v_normal = transpose(inverse(mat3(model))) * normal;
                        gl_Position = model * vec4(position, 1.0);
                    }
                ",

                fragment: &fragment_shader,
            },
        )
        .unwrap();

        Self {
            vertex_buffer: positions,
            normals_buffer: normals,
            index_buffer: indices,
            program,
        }
    }

    fn draw_frame(&mut self, display: &glium::Display<WindowSurface>) {
        let mut frame = display.draw();
        let uniforms = uniform! {
            model: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            u_light: [-1.0, 0.4, 0.9f32],
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        frame
            .draw(
                (&self.vertex_buffer, &self.normals_buffer),
                &self.index_buffer,
                &self.program,
                &uniforms,
                &params,
            )
            .unwrap();
        frame.finish().unwrap();
    }
}

fn main() {
    State::<Application>::run_loop();
}
