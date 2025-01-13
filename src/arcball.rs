use nalgebra::{UnitQuaternion, Vector2, Vector3};

/// Shoeman arcball control for intuitive rotation,
/// This implementation is using quaternion to avoid gimball locks.
/// The code is ported from [this resource](https://raw.org/code/trackball-rotation-using-quaternions/).
/// `radius` is controlling the virtual sphere radius, lowering it make the arcball rotation
/// mecanically faster and conversaly.
pub struct ArcballControl {
    last_quaternion: UnitQuaternion<f32>,
    current_quaternion: UnitQuaternion<f32>,
    start: Option<Vector2<f32>>,
    width: f32,
    height: f32,
    radius: f32,
}

impl ArcballControl {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            last_quaternion: UnitQuaternion::identity(),
            current_quaternion: UnitQuaternion::identity(),
            start: None,
            width,
            height,
            radius: 0.5,
        }
    }

    pub fn reset(&mut self) {
        self.last_quaternion = UnitQuaternion::identity();
        self.current_quaternion = UnitQuaternion::identity();
        self.start = None;
    }

    pub fn mouse_down(&mut self, x: f32, y: f32) {
        self.start = Some(Vector2::new(x, y));
    }

    pub fn mouse_move(&mut self, x: f32, y: f32) {
        if let Some(start_pt) = self.start {
            let a = self.project(start_pt.x, start_pt.y);
            let b = self.project(x, y);

            if let Some(rot) = UnitQuaternion::rotation_between(&a, &b) {
                self.current_quaternion = rot;
            } else {
                self.current_quaternion = UnitQuaternion::identity();
            }
        }
    }

    pub fn mouse_up(&mut self) {
        if self.start.is_some() {
            self.last_quaternion = self.current_quaternion * self.last_quaternion;
            self.current_quaternion = UnitQuaternion::identity();
            self.start = None;
        }
    }

    pub fn get_rotation(&self) -> UnitQuaternion<f32> {
        self.current_quaternion * self.last_quaternion
    }

    pub fn get_rotation_matrix(&self) -> nalgebra::Matrix4<f32> {
        self.get_rotation().to_homogeneous()
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    /// Maps window coordinates (x,y) into a [-1..1]^2 region,
    /// then projects onto a virtual sphere of radius `self.radius`.
    fn project(&self, x: f32, y: f32) -> Vector3<f32> {
        // Similar to:
        // x' = (2*x - width - 1) / res
        // y' = (2*y - height - 1) / res

        let res = (self.width.min(self.height) - 1.0).max(1.0);
        let nx = (2.0 * x - self.width - 1.0) / res;
        let ny = -(2.0 * y - self.height - 1.0) / res;

        let d = nx * nx + ny * ny;
        let r2 = self.radius * self.radius;

        // If 2*d <= r^2 => on the hemisphere
        // else => on the hyperbolic region
        let z = if 2.0 * d <= r2 {
            (r2 - d).sqrt()
        } else {
            r2 / (2.0 * d.sqrt())
        };

        Vector3::new(nx, ny, z)
    }
}
