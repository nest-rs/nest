
use glium;
use support::vertex::{color, texture};
use image::ImageParameters;

pub fn line_vert_buff<'a, D>(
    display: &'a D,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    color: [f32; 4],
) -> Result<glium::VertexBuffer<color::Vertex>, glium::vertex::BufferCreationError>
where
    D: glium::backend::Facade,
{
    let mut shape: Vec<color::Vertex> = Vec::new();

    shape.push(color::Vertex::new(x1, -y1, color));
    shape.push(color::Vertex::new(x2, -y2, color));

    glium::VertexBuffer::new(display, &shape)
}

pub fn poly_vert_buffer<'a, D>(
    display: &'a D,
    points: &[(f32, f32)],
    color: [f32; 4],
) -> Result<glium::VertexBuffer<color::Vertex>, glium::vertex::BufferCreationError>
where
    D: glium::backend::Facade,
{
    let shape: Vec<color::Vertex> = points
        .iter()
        .map(|point| color::Vertex::new(point.0, -point.1, color))
        .collect();
    glium::VertexBuffer::new(display, &shape)
}

pub fn image_vert_buffer<'a, D>(
    display: &'a D,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    params: ImageParameters,
) -> Result<glium::VertexBuffer<texture::Vertex>, glium::vertex::BufferCreationError>
where
    D: glium::backend::Facade,
{
    let mut shape: Vec<texture::Vertex> = Vec::new();
    let dx1 = params.dx;
    let dy1 = params.dy;
    let dx2 = params.dx + params.dw;
    let dy2 = params.dy + params.dh;

    shape.push(texture::Vertex::new(x1, -y1, dx1, dy1));
    shape.push(texture::Vertex::new(x2, -y1, dx2, dy1));
    shape.push(texture::Vertex::new(x1, -y2, dx1, dy2));
    shape.push(texture::Vertex::new(x2, -y2, dx2, dy2));

    glium::VertexBuffer::new(display, &shape)
}
