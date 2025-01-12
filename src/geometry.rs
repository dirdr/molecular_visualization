pub mod quad {
    use glium::implement_vertex;
    /// Simple Quad static geometry definition
    pub struct Quad {}

    #[derive(Copy, Clone)]
    pub struct QuadVertex {
        pos: [f32; 3],
        uv_coordinates: [f32; 2],
    }

    implement_vertex!(QuadVertex, pos, uv_coordinates);

    impl Quad {
        /// Static accessor to the array of `Vertex` defining a billboard.
        /// The billboard is defined by two triangles using those four vertices, see `get_billboard_indices`.
        pub fn get_vertices_vertices() -> [QuadVertex; 4] {
            [
                QuadVertex {
                    pos: [-0.5, -0.5, 0.0],
                    uv_coordinates: [0.0, 0.0],
                },
                QuadVertex {
                    pos: [0.5, -0.5, 0.0],
                    uv_coordinates: [1.0, 0.0],
                },
                QuadVertex {
                    pos: [-0.5, 0.5, 0.0],
                    uv_coordinates: [0.0, 1.0],
                },
                QuadVertex {
                    pos: [0.5, 0.5, 0.0],
                    uv_coordinates: [1.0, 1.0],
                },
            ]
        }

        /// Static accessor to the array of indices defining the two triangles of a billboard.
        /// The indices in the array are Vertex indices, see `get_vertices_vertices`.
        pub fn get_billboard_indices() -> [u16; 6] {
            [0, 1, 2, 1, 3, 2]
        }
    }
}
