use glium;
use cgm;

use color::{self, Color};
use std::rc::Rc;
use glium::texture::Texture2d;
use std::iter::{Chain, Once, once};

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
pub struct Positions(pub [cgm::Point2<f32>; 3]);

/// A triangle primitive which enters the shader pipeline as a single vertex and is the only primitive in nest
#[derive(Copy, Clone, Debug)]
pub struct Tri {
    /// The three space vertices of the triangles
    pub positions: Positions,
    /// The three texture coordinates of the above vertices
    pub texcoords: Positions,
}

impl Tri {
    /// Create a new triangle with points and tex coordinates specified
    #[inline]
    pub fn new<P: Into<cgm::Point2<f32>> + Copy, T: Into<cgm::Point2<f32>> + Copy>(
        positions: [P; 3],
        texcoords: [T; 3],
    ) -> Tri {
        Tri {
            positions: Positions(
                [
                    positions[0].into(),
                    positions[1].into(),
                    positions[2].into(),
                ],
            ),
            texcoords: Positions(
                [
                    texcoords[0].into(),
                    texcoords[1].into(),
                    texcoords[2].into(),
                ],
            ),
        }
    }

    /// Create a new triangle with points with coordinates to create a single-color triangle
    #[inline]
    pub fn new_pos<P: Into<cgm::Point2<f32>> + Copy>(positions: [P; 3]) -> Tri {
        Tri::new(
            [
                positions[0].into(),
                positions[1].into(),
                positions[2].into(),
            ],
            [cgm::Point2::new(0.0, 0.0); 3],
        )
    }
}

implement_vertex!(Tri, positions, texcoords);

unsafe impl glium::vertex::Attribute for Positions {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32x3x2
    }
}

/// Two points make a rectangle.
pub struct Rect(pub [f32; 2], pub [f32; 2]);

impl Shape for Rect {
    type Iter = Chain<Once<Tri>, Once<Tri>>;
    fn tris(&self) -> Self::Iter {
        once(Tri::new_pos(
            [
                [self.0[0], self.0[1]],
                [self.1[0], self.0[1]],
                [self.0[0], self.1[1]],
            ],
        )).chain(once(Tri::new_pos(
            [
                [self.1[0], self.1[1]],
                [self.0[0], self.1[1]],
                [self.1[0], self.0[1]],
            ],
        )))
    }
}
