use nalgebra::{Matrix4, Point3, Vector3};

trait PerspectiveCamera {
    fn get_view_matrix(&self) -> Matrix4<f32>;
    fn get_projection_matrix(&self) -> Matrix4<f32>;
}

trait SceneMove {
    fn move_forward(&mut self);
    fn move_backward(&mut self);
    fn strafe(&mut self, direction: StrafeDirection);
    // TODO - add rotate (yaw pitch etc...)
}

enum StrafeDirection {
    Left,
    Right,
}

pub struct Camera {
    pos: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    fov: f32,
    aspect_ratio: f32,
    znear: f32,
    zfar: f32,
}

pub struct CameraBuilder {
    pos: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    fov: f32,
    aspect_ratio: f32,
    znear: f32,
    zfar: f32,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            pos: Point3::new(0.0, 0.0, 0.0),
            target: Point3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            fov: 60.0,
            aspect_ratio: 16.0 / 9.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn with_position(mut self, pos: Point3<f32>) -> Self {
        self.pos = pos;
        self
    }

    pub fn with_target(mut self, target: Point3<f32>) -> Self {
        self.target = target;
        self
    }

    pub fn with_up(mut self, up: Vector3<f32>) -> Self {
        self.up = up;
        self
    }

    pub fn with_fov(mut self, fov: f32) -> Self {
        self.fov = fov;
        self
    }

    pub fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn with_znear(mut self, znear: f32) -> Self {
        self.znear = znear;
        self
    }

    pub fn with_zfar(mut self, zfar: f32) -> Self {
        self.zfar = zfar;
        self
    }

    pub fn build(self) -> Camera {
        Camera {
            pos: self.pos,
            target: self.target,
            up: self.up,
            fov: self.fov,
            aspect_ratio: self.aspect_ratio,
            znear: self.znear,
            zfar: self.zfar,
        }
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        CameraBuilder::new()
    }
}

impl PerspectiveCamera for Camera {
    fn get_view_matrix(&self) -> Matrix4<f32> {
        todo!()
    }

    fn get_projection_matrix(&self) -> Matrix4<f32> {
        todo!()
    }
}

impl SceneMove for Camera {
    fn move_forward(&mut self) {
        todo!()
    }

    fn move_backward(&mut self) {
        todo!()
    }

    fn strafe(&mut self, direction: StrafeDirection) {
        todo!()
    }
}
