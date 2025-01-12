#[macro_use]
extern crate glium;

use core::f32;

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
    cylinder_batch::CylinderBatch,
    molecule::Molecule,
    sphere_batch::SphereBatch,
};
use nalgebra::{Matrix4, Point3, Vector3};

struct Application {
    pub camera: PerspectiveCamera<Ready>,
    pub arcball: ArcballControl,
    pub last_cursor_position: Option<PhysicalPosition<f64>>,
    pub molecule: Molecule,
    pub sphere_instances_program: Program,
    pub cylinder_instance_program: Program,
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

        Self {
            camera,
            arcball,
            last_cursor_position: None,
            molecule,
            sphere_instances_program: SphereBatch::build_program(display)
                .expect("Sphere shader program has failed to build"),
            cylinder_instance_program: CylinderBatch::build_program(display)
                .expect("Cylinder shader program has failed to build"),
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
        self.molecule.rotate(rotation);

        self.molecule
            .sync_buffers(display)
            .expect("Failed to synchronize the molecule instances buffer");

        let view = self.camera.get_view_matrix();
        let view_array: [[f32; 4]; 4] = view.into();

        let (width, height) = frame.get_dimensions();
        let aspect_ratio = width as f32 / height as f32;

        // HACK - the aspect ratio is passed dynamically at each frame mainly to avoid scaling with
        // a fixed base aspect ratio.
        let projection = self.camera.get_projection_matrix(aspect_ratio);
        let projection_array: [[f32; 4]; 4] = *projection.as_ref();

        let light: [f32; 3] = Point3::new(0.0, 3.0, 2.0).into();
        let camera_position: [f32; 3] = self.camera.get_position().into();

        let scaling_factor = 0.1; // Uniform scaling factor
        let scene_model: [[f32; 4]; 4] = Matrix4::new_scaling(scaling_factor).into();

        let uniforms = uniform! {
            view: view_array,
            projection: projection_array,
            light_position: light,
            camera_position: camera_position,
            debug_billboard: false,
            scene_model: scene_model
        };

        self.arcball.resize(
            frame.get_dimensions().0 as f32,
            frame.get_dimensions().1 as f32,
        );

        assert!(self.molecule.atoms.index_buffer.get_size() != 0);
        assert!(self.molecule.atoms.vertex_buffer.get_size() != 0);

        let sphere_draw_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            stencil: glium::draw_parameters::Stencil {
                reference_value_clockwise: 1,
                write_mask_clockwise: 0xFF,
                fail_operation_clockwise: glium::StencilOperation::Keep,
                pass_depth_fail_operation_clockwise: glium::StencilOperation::Keep,
                depth_pass_operation_clockwise: glium::StencilOperation::Replace,
                test_clockwise: glium::StencilTest::AlwaysPass,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        let cylinder_draw_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            stencil: glium::draw_parameters::Stencil {
                reference_value_clockwise: 0,
                test_clockwise: glium::StencilTest::IfEqual { mask: 0xFF },
                write_mask_clockwise: 0xFF,
                fail_operation_clockwise: glium::StencilOperation::Keep,
                pass_depth_fail_operation_clockwise: glium::StencilOperation::Keep,
                depth_pass_operation_clockwise: glium::StencilOperation::Keep,
                ..Default::default()
            },
            ..Default::default()
        };
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess, // Standard depth test
                write: true,                    // Write to the depth buffer
                ..Default::default()
            },
            ..Default::default()
        };

        frame.clear_color_and_depth((0.95, 0.95, 0.95, 1.0), 1.0);
        frame.clear_stencil(1);
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

    const WINDOW_TITLE: &'static str = "PDB Viewer - Adrien Pelfresne - FIB 2025";
}

fn main() {
    State::<Application>::run_loop();
}
