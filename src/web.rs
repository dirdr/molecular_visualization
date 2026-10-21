use js_sys::Float32Array;
use nalgebra::{Matrix4, Point3, Vector3};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as Gl, WebGlBuffer, WebGlProgram, WebGlShader,
    WebGlVertexArrayObject,
};

use crate::{
    arcball::ArcballControl,
    camera::{Camera, PerspectiveCamera, Ready, Virtual},
    geometry::quad::Quad,
    molecule_data::MoleculeData,
    shaders,
};

fn error(message: &str) -> JsValue {
    js_sys::Error::new(message).into()
}

struct Shader {
    gl: Gl,
    handle: WebGlShader,
}

impl Shader {
    fn new(gl: &Gl, kind: u32, source: &str) -> Result<Self, JsValue> {
        let shader = Self {
            gl: gl.clone(),
            handle: gl
                .create_shader(kind)
                .ok_or_else(|| error("Could not create shader"))?,
        };
        gl.shader_source(&shader.handle, &shaders::webgl_source(source));
        gl.compile_shader(&shader.handle);
        if gl
            .get_shader_parameter(&shader.handle, Gl::COMPILE_STATUS)
            .as_bool()
            != Some(true)
        {
            return Err(error(
                &gl.get_shader_info_log(&shader.handle).unwrap_or_default(),
            ));
        }
        Ok(shader)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.gl.delete_shader(Some(&self.handle));
    }
}

struct Batch {
    gl: Gl,
    program: Option<WebGlProgram>,
    vao: Option<WebGlVertexArrayObject>,
    geometry: Option<WebGlBuffer>,
    instances: Option<WebGlBuffer>,
    count: i32,
}

impl Batch {
    fn new(
        gl: &Gl,
        vertex: &str,
        fragment: &str,
        attributes: &[i32],
        data: &[f32],
    ) -> Result<Self, JsValue> {
        let mut batch = Self {
            gl: gl.clone(),
            program: None,
            vao: None,
            geometry: None,
            instances: None,
            count: 0,
        };
        let vs = Shader::new(gl, Gl::VERTEX_SHADER, vertex)?;
        let fs = Shader::new(gl, Gl::FRAGMENT_SHADER, fragment)?;
        batch.program = Some(
            gl.create_program()
                .ok_or_else(|| error("Could not create program"))?,
        );
        let program = batch.program.as_ref().unwrap();
        gl.attach_shader(program, &vs.handle);
        gl.attach_shader(program, &fs.handle);
        gl.link_program(program);
        if gl.get_program_parameter(program, Gl::LINK_STATUS).as_bool() != Some(true) {
            return Err(error(&gl.get_program_info_log(program).unwrap_or_default()));
        }
        gl.detach_shader(program, &vs.handle);
        gl.detach_shader(program, &fs.handle);

        batch.vao = Some(
            gl.create_vertex_array()
                .ok_or_else(|| error("Could not create vertex array"))?,
        );
        batch.geometry = Some(
            gl.create_buffer()
                .ok_or_else(|| error("Could not create geometry buffer"))?,
        );
        batch.instances = Some(
            gl.create_buffer()
                .ok_or_else(|| error("Could not create instance buffer"))?,
        );
        gl.bind_vertex_array(batch.vao.as_ref());
        gl.bind_buffer(Gl::ARRAY_BUFFER, batch.geometry.as_ref());
        let vertices: Vec<f32> = Quad::get_vertices_vertices()
            .iter()
            .flat_map(|vertex| vertex.pos.into_iter().chain(vertex.uv_coordinates))
            .collect();
        gl.buffer_data_with_array_buffer_view(
            Gl::ARRAY_BUFFER,
            &Float32Array::from(vertices.as_slice()),
            Gl::STATIC_DRAW,
        );
        for (location, size, offset) in [(0, 3, 0), (1, 2, 12)] {
            gl.enable_vertex_attrib_array(location);
            gl.vertex_attrib_pointer_with_i32(location, size, Gl::FLOAT, false, 20, offset);
        }
        gl.bind_buffer(Gl::ARRAY_BUFFER, batch.instances.as_ref());
        gl.buffer_data_with_array_buffer_view(
            Gl::ARRAY_BUFFER,
            &Float32Array::from(data),
            Gl::STATIC_DRAW,
        );
        let stride = attributes.iter().sum::<i32>() * 4;
        batch.count = i32::try_from(data.len() / (stride as usize / 4))
            .map_err(|_| error("Too many molecule instances"))?;
        let mut offset = 0;
        for (index, &size) in attributes.iter().enumerate() {
            let location = index as u32 + 2;
            gl.enable_vertex_attrib_array(location);
            gl.vertex_attrib_pointer_with_i32(location, size, Gl::FLOAT, false, stride, offset);
            gl.vertex_attrib_divisor(location, 1);
            offset += size * 4;
        }
        gl.bind_vertex_array(None);
        Ok(batch)
    }

    fn draw(&self, model: &Matrix4<f32>, camera: &PerspectiveCamera<Ready>, aspect: f32) {
        if self.count == 0 {
            return;
        }
        let gl = &self.gl;
        let program = self.program.as_ref().unwrap();
        gl.use_program(Some(program));
        for (name, matrix) in [
            ("model", *model),
            ("view", camera.get_view_matrix()),
            ("projection", camera.get_projection_matrix(aspect)),
        ] {
            gl.uniform_matrix4fv_with_f32_array(
                gl.get_uniform_location(program, name).as_ref(),
                false,
                matrix.as_slice(),
            );
        }
        let position = camera.get_position();
        gl.uniform3f(
            gl.get_uniform_location(program, "camera_position").as_ref(),
            position.x,
            position.y,
            position.z,
        );
        gl.uniform3f(
            gl.get_uniform_location(program, "light_position").as_ref(),
            0.0,
            2.0,
            1.0,
        );
        gl.uniform1i(
            gl.get_uniform_location(program, "debug_billboard").as_ref(),
            0,
        );
        gl.uniform1i(
            gl.get_uniform_location(program, "u_show_silhouette")
                .as_ref(),
            0,
        );
        gl.bind_vertex_array(self.vao.as_ref());
        gl.draw_arrays_instanced(Gl::TRIANGLE_STRIP, 0, 4, self.count);
        gl.bind_vertex_array(None);
    }
}

impl Drop for Batch {
    fn drop(&mut self) {
        self.gl.delete_buffer(self.geometry.as_ref());
        self.gl.delete_buffer(self.instances.as_ref());
        self.gl.delete_vertex_array(self.vao.as_ref());
        self.gl.delete_program(self.program.as_ref());
    }
}

#[wasm_bindgen]
pub struct MoleculeViewer {
    canvas: HtmlCanvasElement,
    gl: Gl,
    spheres: Batch,
    bonds: Batch,
    camera: PerspectiveCamera<Ready>,
    arcball: ArcballControl,
    scale: f32,
    radius: f32,
}

#[wasm_bindgen]
impl MoleculeViewer {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, pdb: &str) -> Result<MoleculeViewer, JsValue> {
        let data = MoleculeData::from_pdb(pdb).map_err(|e| error(&e.to_string()))?;
        let gl: Gl = canvas
            .get_context("webgl2")?
            .ok_or_else(|| error("WebGL 2 is unavailable in this browser"))?
            .dyn_into()?;
        let sphere_data: Vec<f32> = data
            .atoms
            .iter()
            .flat_map(|atom| {
                atom.instance_pos
                    .into_iter()
                    .chain(atom.instance_color)
                    .chain([atom.instance_radius])
            })
            .collect();
        let bond_data: Vec<f32> = data
            .bonds
            .iter()
            .flat_map(|bond| {
                bond.instance_start_pos
                    .into_iter()
                    .chain(bond.instance_end_pos)
                    .chain(bond.instance_color_first_half)
                    .chain(bond.instance_color_second_half)
                    .chain([bond.instance_radius])
            })
            .collect();
        let spheres = Batch::new(
            &gl,
            shaders::SPHERE_VERTEX,
            shaders::SPHERE_FRAGMENT,
            &[3, 4, 1],
            &sphere_data,
        )?;
        let bonds = Batch::new(
            &gl,
            shaders::CYLINDER_VERTEX,
            shaders::CYLINDER_FRAGMENT,
            &[3, 3, 4, 4, 1],
            &bond_data,
        )?;
        let camera = PerspectiveCamera::<Virtual>::default()
            .place(Point3::new(0.0, 0.0, 4.0))
            .point(Point3::origin(), Vector3::y());
        let arcball = ArcballControl::new(
            canvas.client_width().max(1) as f32,
            canvas.client_height().max(1) as f32,
        );
        gl.enable(Gl::DEPTH_TEST);
        gl.depth_func(Gl::LESS);
        gl.clear_color(0.067, 0.067, 0.067, 1.0);
        let mut viewer = Self {
            canvas,
            gl,
            spheres,
            bonds,
            camera,
            arcball,
            scale: data.scale_factor,
            radius: data.radius,
        };
        viewer.reset_view();
        Ok(viewer)
    }

    #[wasm_bindgen(getter)]
    pub fn atom_count(&self) -> u32 {
        self.spheres.count as u32
    }

    #[wasm_bindgen(getter)]
    pub fn bond_count(&self) -> u32 {
        self.bonds.count as u32
    }

    pub fn draw(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        if width == 0 || height == 0 || width > i32::MAX as u32 || height > i32::MAX as u32 {
            return Err(error(
                "Canvas dimensions must be positive WebGL viewport dimensions",
            ));
        }
        if self.gl.is_context_lost() {
            return Err(error("Graphics context lost"));
        }
        if self.canvas.width() != width {
            self.canvas.set_width(width);
        }
        if self.canvas.height() != height {
            self.canvas.set_height(height);
        }
        self.arcball.resize(
            self.canvas.client_width().max(1) as f32,
            self.canvas.client_height().max(1) as f32,
        );
        self.gl.viewport(0, 0, width as i32, height as i32);
        self.gl.clear(Gl::COLOR_BUFFER_BIT | Gl::DEPTH_BUFFER_BIT);
        let model = self.arcball.get_rotation_matrix() * Matrix4::new_scaling(self.scale);
        let aspect = width as f32 / height as f32;
        self.spheres.draw(&model, &self.camera, aspect);
        self.bonds.draw(&model, &self.camera, aspect);
        Ok(())
    }

    pub fn pointer_down(&mut self, x: f32, y: f32) {
        if x.is_finite() && y.is_finite() {
            self.arcball.mouse_down(x, y);
        }
    }

    pub fn pointer_move(&mut self, x: f32, y: f32) {
        if x.is_finite() && y.is_finite() {
            self.arcball.mouse_move(x, y);
        }
    }

    pub fn pointer_up(&mut self) {
        self.arcball.mouse_up();
    }

    pub fn zoom(&mut self, delta: f32) {
        if delta.is_finite() {
            self.camera.zoom(-delta.clamp(-200.0, 200.0) * 0.2);
        }
    }

    pub fn reset_view(&mut self) {
        self.arcball.reset();
        self.camera.fov = std::f32::consts::FRAC_PI_2;
        let aspect =
            self.canvas.client_width().max(1) as f32 / self.canvas.client_height().max(1) as f32;
        let half_angle = ((self.camera.fov / 2.0).tan() * aspect.min(1.0)).atan();
        self.camera.state.pos.z = self.radius / half_angle.sin() * 1.12;
    }
}
