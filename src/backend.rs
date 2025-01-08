use glium::{
    glutin::surface::WindowSurface,
    winit::{
        application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
        window::WindowId,
    },
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

pub struct State<T> {
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

        let version = display.get_opengl_version();
        let version_string = display.get_opengl_version_string();
        println!("OpenGL version: {:?}", version);
        println!("OpenGL version string: {}", version_string);

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
