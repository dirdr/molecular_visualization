use std::{f32::consts::PI, io::Read};

use nalgebra::{Matrix4, Point3};

/// Define a generic camera contract
pub trait Camera {
    fn get_view_matrix(&self) -> Matrix4<f32>;
    fn get_projection_matrix(&self) -> Matrix4<f32>;
}

pub trait CameraState {}

/// Define the initial state of the camera,
/// which is not placed nor pointed, thus not existing in the scene
pub struct Virtual {}

pub struct Placed {
    pub pos: Point3<f32>,
}

pub struct Pointed {
    pub target: Point3<f32>,
    pub up: Point3<f32>,
}

pub struct Ready {
    pub pos: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Point3<f32>,
}

/// Camera that use a perspective projection.
pub struct PerspectiveCamera<S: CameraState> {
    fov: f32,
    aspect_ratio: f32,
    znear: f32,
    zfar: f32,
    state: S,
}

impl Default for PerspectiveCamera<Virtual> {
    fn default() -> Self {
        Self {
            fov: PI / 3.0,
            aspect_ratio: 16.0 / 9.0,
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
            aspect_ratio: self.aspect_ratio,
            znear: self.znear,
            zfar: self.zfar,
        }
    }

    pub fn point(self, target: Point3<f32>, up: Point3<f32>) -> PerspectiveCamera<Pointed> {
        PerspectiveCamera::<Pointed> {
            state: Pointed { target, up },
            fov: self.fov,
            aspect_ratio: self.aspect_ratio,
            znear: self.znear,
            zfar: self.zfar,
        }
    }
}

impl PerspectiveCamera<Placed> {
    pub fn point(self, target: Point3<f32>, up: Point3<f32>) -> PerspectiveCamera<Ready> {
        PerspectiveCamera::<Ready> {
            state: Ready {
                target,
                up,
                pos: self.state.pos,
            },
            fov: self.fov,
            aspect_ratio: self.aspect_ratio,
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
            aspect_ratio: self.aspect_ratio,
            znear: self.znear,
            zfar: self.zfar,
        }
    }
}

impl Camera for PerspectiveCamera<Ready> {
    fn get_view_matrix(&self) -> Matrix4<f32> {
        todo!()
    }

    fn get_projection_matrix(&self) -> Matrix4<f32> {
        todo!()
    }
}

impl CameraState for Virtual {}
impl CameraState for Placed {}
impl CameraState for Pointed {}
impl CameraState for Ready {}
