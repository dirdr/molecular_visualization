#[macro_use]
extern crate glium;

use std::error::Error;

use glium::{
    winit::{
        application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
        window::WindowId,
    },
    Surface,
};

mod shader;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

struct MyApp {
    display: Option<glium::Display<glium::glutin::surface::WindowSurface>>,
    window: Option<glium::winit::window::Window>,
}

impl ApplicationHandler for MyApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (window, display) =
            glium::backend::glutin::SimpleWindowBuilder::new().build(event_loop);

        self.display = Some(display);
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(display) = &self.display {
                    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

                    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
                    let program = glium::Program::from_source(
                        display,
                        vertex_shader_src,
                        fragment_shader_src,
                        None,
                    )
                    .unwrap();

                    let vertex1 = Vertex {
                        position: [-0.5, -0.5],
                    };
                    let vertex2 = Vertex {
                        position: [0.0, 0.5],
                    };
                    let vertex3 = Vertex {
                        position: [0.5, -0.25],
                    };
                    let shape = vec![vertex1, vertex2, vertex3];
                    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
                    let indices =
                        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);
                    target
                        .draw(
                            &vertex_buffer,
                            indices,
                            &program,
                            &glium::uniforms::EmptyUniforms,
                            &Default::default(),
                        )
                        .unwrap();
                    target.finish().unwrap();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = glium::winit::event_loop::EventLoop::builder().build()?;

    let mut my_app = MyApp {
        display: None,
        window: None,
    };

    implement_vertex!(Vertex, position);

    Ok(event_loop.run_app(&mut my_app)?)
}
