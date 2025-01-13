#[macro_use]
extern crate glium;

use core::f32;

use glium::{
    glutin::surface::WindowSurface,
    uniforms::Uniforms,
    winit::{
        dpi::PhysicalPosition,
        event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent},
    },
    Frame, Program, Surface,
};
use molecular_visualization::{
    arcball::ArcballControl,
    backend::{ApplicationContext, State},
    camera::{Camera, PerspectiveCamera, Ready, Virtual},
    cylinder_batch::CylinderBatch,
    geometry::{Model, Rotate, Scale},
    molecule::Molecule,
    sphere_batch::SphereBatch,
};
use nalgebra::{Matrix4, Point3, Vector3};

/// OpenGL Application wrapper,
/// contains all the necessary informations to make the program run,
/// for more informations on how the glium/winit backend is running, see `backend.rs`.
struct Application {
    pub camera: PerspectiveCamera<Ready>,
    pub arcball: ArcballControl,
    pub last_cursor_position: Option<PhysicalPosition<f64>>,
    pub molecule: Molecule,
    pub sphere_instances_program: Program,
    pub cylinder_instance_program: Program,
    light: Point3<f32>,
}

impl Application {
    fn get_uniforms(&mut self, frame: &Frame) -> impl Uniforms {
        self.molecule.reset_model_matrix();
        self.molecule
            .scale(Matrix4::new_scaling(self.molecule.scale_factor));
        self.molecule.rotate(self.arcball.get_rotation_matrix());
        let molecule_model: [[f32; 4]; 4] = self.molecule.model_matrix().into();

        let view: [[f32; 4]; 4] = self.camera.get_view_matrix().into();

        // HACK - the aspect ratio is passed dynamically at each frame mainly to avoid scaling with
        // a fixed base aspect ratio.
        let (width, height) = frame.get_dimensions();
        let aspect_ratio = width as f32 / height as f32;
        let projection: [[f32; 4]; 4] = self.camera.get_projection_matrix(aspect_ratio).into();

        let light: [f32; 3] = self.light.into();
        let camera_position: [f32; 3] = self.camera.get_position().into();

        uniform! {
            view: view,
            projection: projection,
            light_position: light,
            camera_position: camera_position,
            debug_billboard: false,
            model: molecule_model,
            u_show_silhouette: self.molecule.show_silhouette,
        }
    }
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
            .init_molecule()
            .expect("Failed to populate molecule instances");

        molecule
            .sync_buffers(display)
            .expect("Failed to synchronize the molecule vertex buffer");

        Self {
            camera,
            arcball,
            last_cursor_position: None,
            molecule,
            sphere_instances_program: SphereBatch::build_program(display)
                .expect("Sphere shader program has failed to build"),
            cylinder_instance_program: CylinderBatch::build_program(display)
                .expect("Cylinder shader program has failed to build"),
            light: Point3::new(0.0, 2.0, 1.0),
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
                if *button == MouseButton::Left {
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
                if *button == MouseButton::Right && state == &ElementState::Pressed {
                    self.molecule.toggle_silhouette();
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
        let uniforms = self.get_uniforms(&frame);

        self.arcball.resize(
            frame.get_dimensions().0 as f32,
            frame.get_dimensions().1 as f32,
        );

        assert!(self.molecule.atoms.index_buffer.get_size() != 0);
        assert!(self.molecule.atoms.vertex_buffer.get_size() != 0);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        frame.clear_color_and_depth((0.1294, 0.1294, 0.1294, 1.0), 1.0);
        frame
            .draw(
                (
                    &self.molecule.atoms.vertex_buffer,
                    self.molecule.atoms.instance_buffer.per_instance().unwrap(),
                ),
                &self.molecule.atoms.index_buffer,
                &self.sphere_instances_program,
                &uniforms,
                &params,
            )
            .expect("Frame draw call have failed");

        frame
            .draw(
                (
                    &self.molecule.bonds.vertex_buffer,
                    self.molecule.bonds.instance_buffer.per_instance().unwrap(),
                ),
                &self.molecule.bonds.index_buffer,
                &self.cylinder_instance_program,
                &uniforms,
                &params,
            )
            .expect("Frame draw call have failed");

        frame.finish().unwrap();
    }

    const WINDOW_TITLE: &'static str = "Adrien Pelfresne's MolViz";
}

fn main() {
    State::<Application>::run_loop();
}
