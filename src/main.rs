#[macro_use]
extern crate glium;

use std::fs;

use glium::{glutin::surface::WindowSurface, Surface};
use molecular_visualization::{
    backend::{ApplicationContext, State},
    camera::{Camera, PerspectiveCamera, Ready, Virtual},
    teapot::{self},
};
use nalgebra::{Point3, Vector3};

struct Application {
    pub vertex_buffer: glium::VertexBuffer<teapot::Vertex>,
    pub normals_buffer: glium::VertexBuffer<teapot::Normal>,
    pub index_buffer: glium::IndexBuffer<u16>,
    pub program: glium::Program,
    pub camera: PerspectiveCamera<Ready>,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texture: [f32; 2],
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

        let vertex_shader = fs::read_to_string("./resources/shaders/shader.vert")
            .expect("Failed to read Vertex shader");

        let fragment_shader = fs::read_to_string("./resources/shaders/bill_phong.frag")
            .expect("Failed to read fragment shader");

        let program = program!(display,
            410 => {
                vertex: &vertex_shader,
                fragment: &fragment_shader,
            },
        )
        .unwrap();

        let pos = Point3::new(2.0, 2.0, 0.5);
        let target = Point3::new(0.0, 0.0, 0.0);
        let up = Vector3::y();

        let camera = PerspectiveCamera::<Virtual> {
            ..Default::default()
        }
        .place(pos)
        .point(target, up);

        Self {
            vertex_buffer: positions,
            normals_buffer: normals,
            index_buffer: indices,
            program,
            camera,
        }
    }

    fn draw_frame(&mut self, display: &glium::Display<WindowSurface>) {
        let mut frame = display.draw();
        let (width, height) = frame.get_dimensions();
        let aspect_ratio = width as f32 / height as f32;

        // HACK - the aspect ratio is passed dynamically at each frame mainly to avoid scaling with
        // a fixed base aspect ratio.
        let projection = self.camera.get_projection_matrix(aspect_ratio);
        let projection_array: [[f32; 4]; 4] = *projection.as_ref();

        let view = self.camera.get_view_matrix();
        let view_array: [[f32; 4]; 4] = *view.as_ref();

        let light: [f32; 3] = [1.0, 1.0, 1.0];

        let uniforms = uniform! {
            model: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            view: view_array,
            projection: projection_array,
            u_light: light,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
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
