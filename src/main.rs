#[macro_use]
extern crate glium;

use std::{f32::consts::PI, fs};

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
                    uniform mat4 projection;

                    void main() {
                        v_normal = transpose(inverse(mat3(model))) * normal;
                        gl_Position = projection * model * vec4(position, 1.0);
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

        let perspective = {
            let (width, height) = frame.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = PI / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
            ]
        };

        let uniforms = uniform! {
            model: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 2.5, 1.0f32]
            ],
            u_light: [-1.0, 0.4, 0.9f32],
            projection: perspective,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
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
