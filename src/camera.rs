use std::f32::consts::PI;

use nalgebra::{Matrix4, Point3};

/// Define a generic camera contract
pub trait Camera {
    fn get_view_matrix(&self) -> Matrix4<f32>;
    fn get_projection_matrix(&self) -> Matrix4<f32>;
}

pub trait Place {}

pub trait Point {}

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
    pub fn place(pos: Point3<f32>) -> PerspectiveCamera<Placed> {
        todo!()
    }

    pub fn point(target: Point3<f32>, up: Point3<f32>) -> PerspectiveCamera<Pointed> {
        todo!()
    }
}

impl<S> Camera for PerspectiveCamera<S>
where
    S: CameraState + Place + Point,
{
    fn get_view_matrix(&self) -> Matrix4<f32> {
        todo!()
    }

    fn get_projection_matrix(&self) -> Matrix4<f32> {
        todo!()
    }
}

pub struct Placed {
    pos: Point3<f32>,
}

pub struct Pointed {
    target: Point3<f32>,
    up: Point3<f32>,
}

/// Define the initial state of the camera,
/// which is not placed nor pointed, thus not existing in the scene
pub struct Virtual {}

pub trait CameraState {}

impl CameraState for Placed {}
impl CameraState for Pointed {}
impl CameraState for Virtual {}

impl Place for Placed {}

impl Point for Pointed {}
