use args::Args;
use clap::Parser;
use once_cell::sync::Lazy;

pub mod arcball;
pub mod args;
pub mod backend;
pub mod camera;
pub mod cylinder_batch;
pub mod geometry;
pub mod molecule;
pub mod sphere_batch;

pub static GLSL_TARGET: u16 = 410;
pub static OPEN_GL_TARGET: glium::Version = glium::Version(glium::Api::Gl, 4, 1);

pub static ARGS: Lazy<Args> = Lazy::new(Args::parse);
