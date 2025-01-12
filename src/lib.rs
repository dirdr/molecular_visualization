pub mod arcball;
pub mod backend;
pub mod camera;
pub mod cylinder_batch;
pub mod geometry;
pub mod molecule;
pub mod shader;
pub mod sphere_batch;

pub static GLSL_TARGET: u16 = 410;
pub static OPEN_GL_TARGET: glium::Version = glium::Version(glium::Api::Gl, 4, 1);
