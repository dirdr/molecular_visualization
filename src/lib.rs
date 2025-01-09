pub mod arcball;
pub mod backend;
pub mod camera;
pub mod shader;
pub mod teapot;

pub static GLSL_TARGET: u16 = 410;
pub static OPEN_GL_TARGET: glium::Version = glium::Version(glium::Api::Gl, 4, 1);
