
extern crate glium;

use super::Vertex;

pub fn triangle_vert_buff<'a, D>(
    display: &'a D,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    color: [f32; 4],
) -> Result<glium::VertexBuffer<Vertex>, glium::vertex::BufferCreationError>
where
    D: glium::backend::Facade,
{
    let mut shape: Vec<Vertex> = Vec::new();

    shape.push(Vertex::from(x1, y1, color));
    shape.push(Vertex::from(x2, y2, color));
    shape.push(Vertex::from(x3, y3, color));

    glium::VertexBuffer::new(display, &shape)
}

pub fn rectangle_vert_buff<'a, D>(
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

    shape.push(Vertex::from(x1, y1, color));
    shape.push(Vertex::from(x2, y1, color));
    shape.push(Vertex::from(x1, y2, color));

    shape.push(Vertex::from(x2, y1, color));
    shape.push(Vertex::from(x1, y2, color));
    shape.push(Vertex::from(x2, y2, color));

    glium::VertexBuffer::new(display, &shape)
}

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

    shape.push(Vertex::from(x1, y1, color));
    shape.push(Vertex::from(x2, y2, color));

    glium::VertexBuffer::new(display, &shape)
}
