pub const SPHERE_VERTEX: &str = include_str!("../resources/shaders/sphere_imposter.vert");
pub const SPHERE_FRAGMENT: &str = include_str!("../resources/shaders/sphere_imposter.frag");
pub const CYLINDER_VERTEX: &str = include_str!("../resources/shaders/cylinder_imposter.vert");
pub const CYLINDER_FRAGMENT: &str = include_str!("../resources/shaders/cylinder_imposter.frag");

pub fn webgl_source(source: &str) -> String {
    source.replacen(
        "#version 410 core",
        "#version 300 es\nprecision highp float;",
        1,
    )
}
