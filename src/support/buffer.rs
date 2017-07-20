
extern crate glium;

use super::vertex::color::Vertex;

pub fn line_vert_buff<'a, D>(
    display: &'a D,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    color: [f32; 4],
) -> Result<glium::VertexBuffer<Vertex>, glium::vertex::BufferCreationError>
where
    D: glium::backend::Facade,
{
    let mut shape: Vec<Vertex> = Vec::new();

    shape.push(Vertex::new(x1, y1, color));
    shape.push(Vertex::new(x2, y2, color));

    glium::VertexBuffer::new(display, &shape)
}

pub fn poly_vert_buffer<'a, D>(
    display: &'a D,
    points: &[(f32, f32)],
    color: [f32; 4],
) -> Result<glium::VertexBuffer<Vertex>, glium::vertex::BufferCreationError>
where
    D: glium::backend::Facade,
{
    let mut shape: Vec<Vertex> = points
        .iter()
        .map(|point| Vertex::new(point.0, point.1, color))
        .collect();
    glium::VertexBuffer::new(display, &shape)
}
