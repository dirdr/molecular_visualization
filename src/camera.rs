use nalgebra::{Matrix4, Point3, Vector3};

trait Project {
    fn get_view_matrix(&self) -> Matrix4<f32>;
    fn get_projection_matrix(&self) -> Matrix4<f32>;
}

pub enum Placed {}
pub enum Pointed {}

pub trait CameraState {}

impl CameraState for Placed {}
impl CameraState for Pointed {}

pub struct Camera<S: CameraState> {
    pos: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    aspect_ratio: f32,
    znear: f32,
    zfar: f32,
    marker: std::marker::PhantomData<S>,
}

// impl<S> Camera<S>
// where
//     S: CameraState,
// {
//     pub fn new() -> Self {
//         Self {
//             pos: Point3::origin(),
//             target: Point3::origin(),
//         }
//     }
// }
