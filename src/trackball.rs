use nalgebra::{Unit, UnitQuaternion, Vector3};

/// Ken Shoemake Arcball implementation
pub struct ArcballControl {
    last_quaternion: UnitQuaternion<f32>,
    current_quaternion: UnitQuaternion<f32>,
    start_point: Option<Vector3<f32>>,
    radius: f32,
    width: f32,
    height: f32,
}

impl ArcballControl {
    pub fn new(width: f32, height: f32, radius: f32) -> Self {
        Self {
            last_quaternion: UnitQuaternion::identity(),
            current_quaternion: UnitQuaternion::identity(),
            start_point: None,
            radius,
            width,
            height,
        }
    }

    fn project_to_sphere(&self, x: f32, y: f32) -> Vector3<f32> {
        let x = (2.0 * x - self.width) / self.width.min(self.height);
        let y = (self.height - 2.0 * y) / self.width.min(self.height);
        let d = x * x + y * y;
        let z = if d <= self.radius * self.radius {
            (self.radius * self.radius - d).sqrt() // On the hemisphere
        } else {
            self.radius * self.radius / (2.0 * d.sqrt()) // On the hyperbolic region
        };
        Vector3::new(x, y, z)
    }

    pub fn mouse_down(&mut self, x: f32, y: f32) {
        self.start_point = Some(self.project_to_sphere(x, y));
    }

    pub fn mouse_move(&mut self, x: f32, y: f32) {
        if let Some(start) = self.start_point {
            let end = self.project_to_sphere(x, y);
            let axis = start.cross(&end).normalize();
            let unit_axis = Unit::new_normalize(axis);
            let angle = start.dot(&end).acos();
            let rotation = UnitQuaternion::from_axis_angle(&unit_axis, angle);
            self.current_quaternion = rotation * self.last_quaternion;
        }
    }

    pub fn mouse_up(&mut self) {
        self.last_quaternion = self.current_quaternion;
        self.start_point = None;
    }

    pub fn get_rotation_matrix(&self) -> [[f32; 4]; 4] {
        let rotation_matrix = self.current_quaternion.to_homogeneous();
        rotation_matrix.into()
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}
