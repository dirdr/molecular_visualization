use std::f32::consts::PI;

use nalgebra::{Matrix4, Point3, Vector3};

pub trait Camera {
    fn zoom(&mut self, zoom_amount: f32);
    fn get_view_matrix(&self) -> Matrix4<f32>;
    fn get_projection_matrix(&self, aspect_ratio: f32) -> Matrix4<f32>;
}

/// Camera marker for type-state pattern
pub trait CameraState {}

/// Define the initial type-state of the camera,
/// which is not placed nor pointed, thus not existing in the scene
pub struct Virtual {}

/// type-state marker for a camera that have a position in the scene
pub struct Placed {
    pub pos: Point3<f32>,
}

/// type-state marker for a camera that look at a point in the scene
pub struct Pointed {
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
}

/// type-state marker for a camera that is placed, and look at something,
/// which make it ready to use.
pub struct Ready {
    pub pos: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
}

/// Camera that use a perspective projection.
/// `fov` is in radian.
/// `fov_min` and `fov_max` hold the minimum and maximum acceptable value for the field of view of
/// the camera, this is used to clamp the camera zoom.
/// `state` is the type-state marker for the camera state, it also hold necessary informations.
pub struct PerspectiveCamera<S: CameraState> {
    pub fov: f32,
    pub fov_min: f32,
    pub fov_max: f32,
    pub zoom_sensitivity: f32,
    pub znear: f32,
    pub zfar: f32,
    pub state: S,
}

impl Default for PerspectiveCamera<Virtual> {
    fn default() -> Self {
        Self {
            fov: 90.0 * (PI / 180.0),
            fov_min: 20.0 * (PI / 180.0),
            fov_max: 120.0 * (PI / 180.0),
            zoom_sensitivity: 0.01,
            znear: 0.1,
            zfar: 1024.0,
            state: Virtual {},
        }
    }
}

impl PerspectiveCamera<Virtual> {
    pub fn place(self, pos: Point3<f32>) -> PerspectiveCamera<Placed> {
        PerspectiveCamera::<Placed> {
            state: Placed { pos },
            fov: self.fov,
            fov_min: self.fov_min,
            fov_max: self.fov_max,
            zoom_sensitivity: self.zoom_sensitivity,
            znear: self.znear,
            zfar: self.zfar,
        }
    }

    pub fn point(self, target: Point3<f32>, up: Vector3<f32>) -> PerspectiveCamera<Pointed> {
        PerspectiveCamera::<Pointed> {
            state: Pointed { target, up },
            fov: self.fov,
            fov_min: self.fov_min,
            fov_max: self.fov_max,
            zoom_sensitivity: self.zoom_sensitivity,
            znear: self.znear,
            zfar: self.zfar,
        }
    }
}

impl PerspectiveCamera<Placed> {
    pub fn point(self, target: Point3<f32>, up: Vector3<f32>) -> PerspectiveCamera<Ready> {
        PerspectiveCamera::<Ready> {
            state: Ready {
                target,
                up,
                pos: self.state.pos,
            },
            fov: self.fov,
            fov_min: self.fov_min,
            fov_max: self.fov_max,
            zoom_sensitivity: self.zoom_sensitivity,
            znear: self.znear,
            zfar: self.zfar,
        }
    }
}

impl PerspectiveCamera<Pointed> {
    pub fn place(self, pos: Point3<f32>) -> PerspectiveCamera<Ready> {
        PerspectiveCamera::<Ready> {
            state: Ready {
                pos,
                target: self.state.target,
                up: self.state.up,
            },
            fov: self.fov,
            fov_min: self.fov_min,
            fov_max: self.fov_max,
            zoom_sensitivity: self.zoom_sensitivity,
            znear: self.znear,
            zfar: self.zfar,
        }
    }
}

impl Camera for PerspectiveCamera<Ready> {
    fn zoom(&mut self, zoom_amount: f32) {
        self.fov =
            (self.fov - zoom_amount * self.zoom_sensitivity).clamp(self.fov_min, self.fov_max);
    }

    fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.state.pos, &self.state.target, &self.state.up)
    }

    fn get_projection_matrix(&self, aspect_ratio: f32) -> Matrix4<f32> {
        Matrix4::new_perspective(aspect_ratio, self.fov, self.znear, self.zfar)
    }
}

impl PerspectiveCamera<Ready> {
    pub fn get_position(&self) -> Point3<f32> {
        self.state.pos
    }
}

impl CameraState for Virtual {}
impl CameraState for Placed {}
impl CameraState for Pointed {}
impl CameraState for Ready {}
