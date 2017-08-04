use glium;
use cgm;

use color::{self, Color};
use std::rc::Rc;
use glium::texture::Texture2d;

/// Trait for structs to be drawn with `draw_shape`
pub trait Shape {
    /// The iterator type for the shape which has triangle items
    type Iter: Iterator<Item = Tri>;

    /// The triangles of the shape
    fn tris(&self) -> Self::Iter;

    /// The color of the shape
    #[inline]
    fn color(&self) -> Color {
        color::WHITE
    }

    /// The texture of the shader
    #[inline]
    fn texture(&self) -> Option<Rc<Texture2d>> {
        None
    }
}

/// Three positions which form a matrix for shader purposes
#[derive(Copy, Clone, Debug)]
pub struct Positions([cgm::Point2<f32>; 3]);

/// A triangle primitive which enters the shader pipeline as a single vertex and is the only primitive in nest
#[derive(Copy, Clone, Debug)]
pub struct Tri {
    /// The three space vertices of the triangles
    pub positions: Positions,
    /// The three texture coordinates of the above vertices
    pub texcoords: Positions,
}

implement_vertex!(Tri, positions, texcoords);

unsafe impl glium::vertex::Attribute for Positions {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32x3x2
    }
}