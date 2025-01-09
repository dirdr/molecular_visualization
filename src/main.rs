#[macro_use]
extern crate glium;

use std::fs;

use glium::{
    glutin::surface::WindowSurface,
    winit::{
        dpi::PhysicalPosition,
        event::{ElementState, WindowEvent},
    },
    Surface,
};
use molecular_visualization::{
    arcball::ArcballControl,
    backend::{ApplicationContext, State},
    camera::{Camera, PerspectiveCamera, Ready, Virtual},
    teapot::{self},
};
use nalgebra::{Matrix4, Point3, Vector3};

struct Application {
    pub vertex_buffer: glium::VertexBuffer<teapot::Vertex>,
    pub normals_buffer: glium::VertexBuffer<teapot::Normal>,
    pub index_buffer: glium::IndexBuffer<u16>,
    pub program: glium::Program,
    pub camera: PerspectiveCamera<Ready>,
    pub arcball: ArcballControl,
    pub last_cursor_position: Option<PhysicalPosition<f64>>,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texture: [f32; 2],
}

impl ApplicationContext for Application {
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

        let pos = Point3::new(0.0, 0.0, 4.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let up = Vector3::y();

        let camera = PerspectiveCamera::<Virtual> {
            ..Default::default()
        }
        .place(pos)
        .point(target, up);

        let (width, height) = display.get_framebuffer_dimensions();
        let arcball = ArcballControl::new(width as f32, height as f32);

        Self {
            vertex_buffer: positions,
            normals_buffer: normals,
            index_buffer: indices,
            program,
            camera,
            arcball,
            last_cursor_position: None,
        }
    }

    fn handle_window_event(
        &mut self,
        event: &glium::winit::event::WindowEvent,
        _window: &glium::winit::window::Window,
    ) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.last_cursor_position = Some(*position);
                self.arcball
                    .mouse_move(position.x as f32, position.y as f32);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if *button == glium::winit::event::MouseButton::Left {
                    match state {
                        ElementState::Pressed => {
                            if let Some(pos) = self.last_cursor_position {
                                self.arcball.mouse_down(pos.x as f32, pos.y as f32);
                            }
                        }
                        ElementState::Released => {
                            self.arcball.mouse_up();
                        }
                    }
                }
            }
            WindowEvent::Resized(size) => {
                self.arcball.resize(size.width as f32, size.height as f32);
            }
            _ => {}
        }
    }

    fn draw_frame(&mut self, display: &glium::Display<WindowSurface>) {
        let mut frame = display.draw();

        let light: [f32; 3] = [1.0, 1.0, 1.0];

        #[rustfmt::skip]
        let model: Matrix4<f32> = Matrix4::<f32>::from_row_slice(&[
            0.01, 0.0, 0.0, 0.0,
            0.0, 0.01, 0.0, 0.0,
            0.0, 0.0, 0.01, 0.0,
            0.0, 0.0, 0.0, 1.0f32,
        ]);

        let rotation = self.arcball.get_rotation_matrix();
        let model: [[f32; 4]; 4] = (rotation * model).into();

        let view = self.camera.get_view_matrix();
        let view_array: [[f32; 4]; 4] = view.into();

        let (width, height) = frame.get_dimensions();
        let aspect_ratio = width as f32 / height as f32;
        // HACK - the aspect ratio is passed dynamically at each frame mainly to avoid scaling with
        // a fixed base aspect ratio.
        let projection = self.camera.get_projection_matrix(aspect_ratio);
        let projection_array: [[f32; 4]; 4] = *projection.as_ref();

        let uniforms = uniform! {
            model: model,
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

        self.arcball.resize(
            frame.get_dimensions().0 as f32,
            frame.get_dimensions().1 as f32,
        );

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

    const WINDOW_TITLE: &'static str = "PDB Viewer - Adrien Pelfresne - FIB 2025";
}

fn main() {
    State::<Application>::run_loop();
}
