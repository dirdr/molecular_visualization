#[macro_use]
extern crate glium;

use glium::{
    glutin::surface::WindowSurface,
    index::PrimitiveType,
    winit::{
        application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
        window::WindowId,
    },
    Surface,
};

pub trait ApplicationContext {
    fn draw_frame(&mut self, _display: &glium::Display<WindowSurface>) {}
    fn new(display: &glium::Display<WindowSurface>) -> Self;
    fn update(&mut self) {}
    fn handle_window_event(
        &mut self,
        _event: &glium::winit::event::WindowEvent,
        _window: &glium::winit::window::Window,
    ) {
    }
    const WINDOW_TITLE: &'static str;
}

struct State<T> {
    pub display: glium::Display<WindowSurface>,
    pub window: glium::winit::window::Window,
    pub context: T,
}

struct AppLifecycle<T> {
    state: Option<State<T>>,
    close_promptly: bool,
}

impl<T: ApplicationContext + 'static> ApplicationHandler<()> for AppLifecycle<T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.state = Some(State::new(event_loop));
        if self.close_promptly {
            event_loop.exit();
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.state = None;
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            glium::winit::event::WindowEvent::Resized(new_size) => {
                if let Some(state) = &self.state {
                    state.display.resize(new_size.into());
                }
            }
            glium::winit::event::WindowEvent::RedrawRequested => {
                if let Some(state) = &mut self.state {
                    state.context.update();
                    state.context.draw_frame(&state.display);
                    if self.close_promptly {
                        event_loop.exit();
                    }
                }
            }
            // Exit the event loop when requested (by closing the window for example) or when
            // pressing the Esc key.
            glium::winit::event::WindowEvent::CloseRequested
            | glium::winit::event::WindowEvent::KeyboardInput {
                event:
                    glium::winit::event::KeyEvent {
                        state: glium::winit::event::ElementState::Pressed,
                        logical_key:
                            glium::winit::keyboard::Key::Named(glium::winit::keyboard::NamedKey::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            // Every other event
            ev => {
                if let Some(state) = &mut self.state {
                    state.context.handle_window_event(&ev, &state.window);
                }
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &self.state {
            state.window.request_redraw();
        }
    }
}

impl<T: ApplicationContext + 'static> State<T> {
    pub fn new(event_loop: &glium::winit::event_loop::ActiveEventLoop) -> Self {
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title(T::WINDOW_TITLE)
            .build(event_loop);
        Self::from_display_window(display, window)
    }

    pub fn from_display_window(
        display: glium::Display<WindowSurface>,
        window: glium::winit::window::Window,
    ) -> Self {
        let context = T::new(&display);
        Self {
            display,
            window,
            context,
        }
    }

    pub fn run_loop() {
        let event_loop = glium::winit::event_loop::EventLoop::builder()
            .build()
            .expect("event loop building");
        let mut app = AppLifecycle::<T> {
            state: None,
            close_promptly: false,
        };
        let result = event_loop.run_app(&mut app);
        result.unwrap();
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

struct Application {
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub index_buffer: glium::IndexBuffer<u16>,
    pub program: glium::Program,
}

impl ApplicationContext for Application {
    const WINDOW_TITLE: &'static str = "Glium triangle example";

    fn new(display: &glium::Display<WindowSurface>) -> Self {
        let vertex_buffer = {
            glium::VertexBuffer::new(
                display,
                &[
                    Vertex {
                        position: [-0.5, -0.5],
                        color: [0.0, 1.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, 0.5],
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: [0.5, -0.5],
                        color: [1.0, 0.0, 0.0],
                    },
                ],
            )
            .unwrap()
        };

        // building the index buffer
        let index_buffer =
            glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();

        // compiling shaders and linking them together
        let program = program!(display,
            100 => {
                vertex: "
                    #version 100

                    uniform lowp mat4 matrix;

                    attribute lowp vec2 position;
                    attribute lowp vec3 color;

                    varying lowp vec3 vColor;

                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0) * matrix;
                        vColor = color;
                    }
                ",

                fragment: "
                    #version 100
                    varying lowp vec3 vColor;

                    void main() {
                        gl_FragColor = vec4(vColor, 1.0);
                    }
                ",
            },
        )
        .unwrap();

        Self {
            vertex_buffer,
            index_buffer,
            program,
        }
    }

    fn draw_frame(&mut self, display: &glium::Display<WindowSurface>) {
        let mut frame = display.draw();
        // For this example a simple identity matrix suffices
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        // Now we can draw the triangle
        frame.clear_color(0.0, 0.0, 0.0, 0.0);
        frame
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        frame.finish().unwrap();
    }
}

fn main() {
    State::<Application>::run_loop();
}

mod shader {
    use glium::{glutin::surface::WindowSurface, Display};

    pub fn get_program_from_file(
        display: &Display<WindowSurface>,
        frag_path: &str,
        vert_path: &str,
    ) -> Result<glium::Program, anyhow::Error> {
        let frag_shader = std::fs::read_to_string(frag_path)?;
        let vert_shader = std::fs::read_to_string(vert_path)?;
        let program = glium::Program::from_source(display, &frag_shader, &vert_shader, None)?;
        Ok(program)
    }
}
