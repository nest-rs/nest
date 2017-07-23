
pub mod color {
    #[derive(Copy, Clone)]
    pub struct Vertex {
        pub position: [f64; 2],
        pub color: [f32; 4],
    }
    implement_vertex!(Vertex, position, color);

    impl Vertex {
        pub fn new(x: f64, y: f64, color: [f32; 4]) -> Self {
            Vertex {
                position: [x, y],
                color: color,
            }
        }
    }
}

pub mod texture {
    #[derive(Copy, Clone)]
    pub struct Vertex {
        pub position: [f64; 2],
        pub tex_coords: [f64; 2],
    }
    implement_vertex!(Vertex, position, tex_coords);

    impl Vertex {
        pub fn new(x: f64, y: f64, u: f64, v: f64) -> Self {
            Vertex {
                position: [x, y],
                tex_coords: [u, v],
            }
        }
    }
}
