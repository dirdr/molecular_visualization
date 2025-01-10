#[macro_use]
extern crate glium;

use std::fs;

use glium::{
    glutin::surface::WindowSurface,
    texture::RawImage2d,
    winit::{
        dpi::PhysicalPosition,
        event::{ElementState, MouseScrollDelta, TouchPhase, WindowEvent},
    },
    Surface, Texture2d,
};
use molecular_visualization::{
    arcball::ArcballControl,
    backend::{ApplicationContext, State},
    camera::{Camera, PerspectiveCamera, Ready, Virtual},
};
use nalgebra::{Matrix4, Point3, Vector3};

struct Application {
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub program: glium::Program,
    pub camera: PerspectiveCamera<Ready>,
    pub arcball: ArcballControl,
    pub last_cursor_position: Option<PhysicalPosition<f64>>,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

impl ApplicationContext for Application {
    fn new(display: &glium::Display<WindowSurface>) -> Self {
        let vertices = [
            Vertex {
                position: [-0.5, -0.5, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.0],
                tex_coords: [1.0, 1.0],
            },
        ];

        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();

        let vertex_shader = fs::read_to_string("./resources/shaders/billboard.vert")
            .expect("Failed to read Vertex shader");

        let fragment_shader = fs::read_to_string("./resources/shaders/billboard.frag")
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
            vertex_buffer,
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
            WindowEvent::MouseWheel {
                delta,
                phase: TouchPhase::Moved,
                ..
            } => {
                let scroll_amount = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y,
                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32 * 0.1,
                };
                self.camera.zoom(scroll_amount);
            }
            WindowEvent::Resized(size) => {
                self.arcball.resize(size.width as f32, size.height as f32);
            }
            _ => {}
        }
    }

    fn draw_frame(&mut self, display: &glium::Display<WindowSurface>) {
        let mut frame = display.draw();

        #[rustfmt::skip]
        let model: Matrix4<f32> = Matrix4::<f32>::from_row_slice(&[
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0f32,
        ]);

        let rotation = self.arcball.get_rotation_matrix();

        let view = self.camera.get_view_matrix();
        let view_array: [[f32; 4]; 4] = view.into();

        // HACK - Applying the arcball rotation relative to the camera.
        // 1. Move to camera space (view matrix)
        // 2. Apply rotation
        // 3. Move back to world space (inverse view matrix)
        // 4. Apply model transformations
        let model: [[f32; 4]; 4] = (model * view.try_inverse().unwrap() * rotation * view).into();

        let (width, height) = frame.get_dimensions();
        let aspect_ratio = width as f32 / height as f32;
        // HACK - the aspect ratio is passed dynamically at each frame mainly to avoid scaling with
        // a fixed base aspect ratio.
        let projection = self.camera.get_projection_matrix(aspect_ratio);
        let projection_array: [[f32; 4]; 4] = *projection.as_ref();

        let white_pixel = vec![255u8, 255, 255, 255]; // RGBA for white (255, 255, 255, 255)

        // Create a 1x1 white texture
        let white_texture =
            Texture2d::new(display, RawImage2d::from_raw_rgba(white_pixel, (1, 1))).unwrap();

        let uniforms = uniform! {
            model: model,
            view: view_array,
            projection: projection_array,
            u_texture: white_texture,
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

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

        frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        frame
            .draw(
                &self.vertex_buffer,
                indices,
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
