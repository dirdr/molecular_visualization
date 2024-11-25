use glium::{glutin::surface::WindowSurface, implement_vertex};

/// Returns a vertex buffer that should be rendered as `TrianglesList`.
/// This code has been taken 'as it' from [glium_examples](https://github.com/glium/glium/blob/master/examples/support/mod.rs)
/// with minimum changes.
pub fn load_wavefront(
    display: &glium::Display<WindowSurface>,
    data: &[u8],
) -> glium::vertex::VertexBufferAny {
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3],
        texture: [f32; 2],
    }

    implement_vertex!(Vertex, position, normal, texture);

    let mut data = ::std::io::BufReader::new(data);
    let data = obj::ObjData::load_buf(&mut data).unwrap();

    let mut vertex_data = Vec::new();

    for object in data.objects.iter() {
        for polygon in object.groups.iter().flat_map(|g| g.polys.iter()) {
            match polygon {
                obj::SimplePolygon(indices) => {
                    for v in indices.iter() {
                        let position = data.position[v.0];
                        let texture = v.1.map(|index| data.texture[index]);
                        let normal = v.2.map(|index| data.normal[index]);
                        let texture = texture.unwrap_or([0.0, 0.0]);
                        let normal = normal.unwrap_or([0.0, 0.0, 0.0]);
                        vertex_data.push(Vertex {
                            position,
                            normal,
                            texture,
                        })
                    }
                }
            }
        }
    }

    glium::vertex::VertexBuffer::new(display, &vertex_data)
        .unwrap()
        .into()
}
