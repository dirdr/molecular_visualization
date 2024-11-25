use glium::{glutin::surface::WindowSurface, Display};

pub fn get_program_from_file(
    display: &Display<WindowSurface>,
    frag_path: &str,
    vert_path: &str,
) -> Result<glium::Program, anyhow::Error> {
    let frag_shader = std::fs::read_to_string(frag_path)?;
    let vert_shader = std::fs::read_to_string(vert_path)?;
    let program = glium::Program::from_source(display, &frag_shader, &vert_shader, None)?;
    Ok(program)
}
