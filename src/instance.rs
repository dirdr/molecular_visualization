use nalgebra::{Point3, Point4};

#[derive(Copy, Clone, Debug)]
pub struct SphereInstanceData {
    pub instance_pos: [f32; 3],
    pub instance_color: [f32; 4],
    pub instance_radius: f32,
}

impl SphereInstanceData {
    pub fn new(pos: Point3<f32>, color: Point4<f32>, radius: f32) -> Self {
        Self {
            instance_pos: pos.into(),
            instance_color: color.into(),
            instance_radius: radius,
        }
    }
}

#[cfg(feature = "desktop")]
glium::implement_vertex!(
    SphereInstanceData,
    instance_pos,
    instance_color,
    instance_radius
);

#[derive(Copy, Clone, Debug)]
pub struct CylinderInstanceData {
    pub instance_start_pos: [f32; 3],
    pub instance_end_pos: [f32; 3],
    pub instance_color_first_half: [f32; 4],
    pub instance_color_second_half: [f32; 4],
    pub instance_radius: f32,
}

impl CylinderInstanceData {
    pub fn new(
        start_pos: Point3<f32>,
        end_pos: Point3<f32>,
        color_first_half: Point4<f32>,
        color_second_half: Point4<f32>,
        radius: f32,
    ) -> Self {
        Self {
            instance_start_pos: start_pos.into(),
            instance_end_pos: end_pos.into(),
            instance_color_first_half: color_first_half.into(),
            instance_color_second_half: color_second_half.into(),
            instance_radius: radius,
        }
    }
}

#[cfg(feature = "desktop")]
glium::implement_vertex!(
    CylinderInstanceData,
    instance_start_pos,
    instance_end_pos,
    instance_color_first_half,
    instance_color_second_half,
    instance_radius
);
