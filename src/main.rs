#[macro_use]
extern crate glium;

use glium::{
    glutin::surface::WindowSurface,
    winit::{
        dpi::PhysicalPosition,
        event::{ElementState, MouseScrollDelta, TouchPhase, WindowEvent},
    },
    DrawParameters, Program, Surface,
};
use molecular_visualization::{
    arcball::ArcballControl,
    backend::{ApplicationContext, State},
    camera::{Camera, PerspectiveCamera, Ready, Virtual},
    molecule::Molecule,
    sphere_batch::SphereBatch,
};
use nalgebra::{Point3, Vector3};

struct Application {
    pub camera: PerspectiveCamera<Ready>,
    pub arcball: ArcballControl,
    pub last_cursor_position: Option<PhysicalPosition<f64>>,
    pub molecule: Molecule,
    pub sphere_instances_program: Program,
    pub draw_params: DrawParameters<'static>,
}

impl ApplicationContext for Application {
    /// Create a new Application, if one of the operation fails (Related to OpenGL errors), make
    /// the program panic rather than propagating the Error to the backend.
    fn new(display: &glium::Display<WindowSurface>) -> Self {
        let (width, height) = display.get_framebuffer_dimensions();
        let arcball = ArcballControl::new(width as f32, height as f32);

        let camera_pos = Point3::new(0.0, 0.0, 4.0);
        let camera_target = Point3::new(0.0, 0.0, 0.0);
        let camera_up = Vector3::y();
        let camera = PerspectiveCamera::<Virtual> {
            ..Default::default()
        }
        .place(camera_pos)
        .point(camera_target, camera_up);

        let mut molecule = Molecule::initialize_instances(display)
            .expect("Molecule have failed to initialize instances");

        molecule
            .init_molecule(display)
            .expect("Failed to populate molecule instances");

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            // TODO enable the backface cullink
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };

        Self {
            camera,
            arcball,
            last_cursor_position: None,
            molecule,
            sphere_instances_program: SphereBatch::build_program(display)
                .expect("Sphere instances program has failed to build"),
            draw_params: params,
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

        let rotation = self.arcball.get_rotation_matrix();
        let view = self.camera.get_view_matrix();
        let view_array: [[f32; 4]; 4] = view.into();

        let (width, height) = frame.get_dimensions();
        let aspect_ratio = width as f32 / height as f32;
        // HACK - the aspect ratio is passed dynamically at each frame mainly to avoid scaling with
        // a fixed base aspect ratio.
        let projection = self.camera.get_projection_matrix(aspect_ratio);
        let projection_array: [[f32; 4]; 4] = *projection.as_ref();

        let light: [f32; 3] = Point3::new(1.0, 0.0, 1.0).into();
        let camera_position: [f32; 3] = self.camera.get_position().into();

        let uniforms = uniform! {
            view: view_array,
            projection: projection_array,
            light_position: light,
            camera_position: camera_position,
        };

        self.arcball.resize(
            frame.get_dimensions().0 as f32,
            frame.get_dimensions().1 as f32,
        );

        assert!(self.molecule.sphere_instances.index_buffer.get_size() != 0);
        assert!(self.molecule.sphere_instances.vertex_buffer.get_size() != 0);

        frame.clear_color_and_depth((0.95, 0.95, 0.95, 1.0), 1.0);
        frame
            .draw(
                (
                    &self.molecule.sphere_instances.vertex_buffer,
                    self.molecule
                        .sphere_instances
                        .instance_buffer
                        .per_instance()
                        .unwrap(),
                ),
                &self.molecule.sphere_instances.index_buffer,
                &self.sphere_instances_program,
                &uniforms,
                &self.draw_params,
            )
            .expect("Frame draw call have failed");

        frame.finish().unwrap();
    }

    const WINDOW_TITLE: &'static str = "PDB Viewer - Adrien Pelfresne - FIB 2025";
}

fn main() {
    State::<Application>::run_loop();
}
