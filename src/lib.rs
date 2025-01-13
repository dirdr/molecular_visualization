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

/// These are the only version for which the program has been tested, on a macbook with apple
/// sillicon, the program should work with more recent version, but i have no guarentee.
pub static GLSL_TARGET: u16 = 410;
pub static OPEN_GL_TARGET: glium::Version = glium::Version(glium::Api::Gl, 4, 1);

/// Global static accessor for the command line arguments
pub static ARGS: Lazy<Args> = Lazy::new(Args::parse);
