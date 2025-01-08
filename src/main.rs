#[macro_use]
extern crate glium;

use std::{f32::consts::PI, fs};

use glium::{glutin::surface::WindowSurface, Surface};
use molecular_visualization::{
    backend::{ApplicationContext, State},
    teapot::{self},
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
                    uniform mat4 view;
                    uniform mat4 projection;

                    void main() {
                        v_normal = transpose(inverse(mat3(model))) * normal;
                        gl_Position = projection * view * model * vec4(position, 1.0);
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

        let view = view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);

        let uniforms = uniform! {
            model: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 2.5, 1.0f32]
            ],
            u_light: [-1.0, 0.4, 0.9f32],
            projection: perspective,
            view: view,
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

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0],
    ];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0],
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
    ];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

fn main() {
    State::<Application>::run_loop();
}
