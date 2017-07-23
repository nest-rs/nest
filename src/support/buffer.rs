
use glium;
use support::vertex::{color, texture};
use image::ImageParameters;

pub fn poly_vert_buffer<'a, D>(
    display: &'a D,
    points: &[(f64, f64)],
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
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
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
