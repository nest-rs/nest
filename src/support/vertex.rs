
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}
implement_vertex!(Vertex, position, color);

impl Vertex {
    pub fn new() -> Self {
        Vertex {
            position: [0.0, 0.0],
            color: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn from(x: f32, y: f32, color: [f32; 4]) -> Self {
        Vertex {
            position: [x, y],
            color: color,
        }
    }

    pub fn from_points(x: f32, y: f32) -> Self {
        Vertex {
            position: [x, y],
            color: [1., 1., 1., 1.],
        }
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position = [x, y];
        self
    }

    pub fn with_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.color = [r, g, b, a];
        self
    }
}
