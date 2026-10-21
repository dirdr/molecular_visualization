pub mod arcball;
pub mod camera;
pub mod geometry;
pub mod instance;
pub mod molecule_data;
pub mod shaders;

#[cfg(feature = "desktop")]
pub mod args;
#[cfg(feature = "desktop")]
pub mod backend;
#[cfg(feature = "desktop")]
pub mod cylinder_batch;
#[cfg(feature = "desktop")]
pub mod molecule;
#[cfg(feature = "desktop")]
pub mod sphere_batch;
#[cfg(all(feature = "web", target_arch = "wasm32"))]
pub mod web;

#[cfg(feature = "desktop")]
pub static GLSL_TARGET: u16 = 410;
#[cfg(feature = "desktop")]
pub static OPEN_GL_TARGET: glium::Version = glium::Version(glium::Api::Gl, 4, 1);
#[cfg(feature = "desktop")]
pub static ARGS: once_cell::sync::Lazy<args::Args> =
    once_cell::sync::Lazy::new(<args::Args as clap::Parser>::parse);
