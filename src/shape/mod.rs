use glium;
use cgm;

use Color;
use std::rc::Rc;
use glium::texture::Texture2d;

mod translate;
mod rotate;
mod scale;
mod combine;
mod image;
mod rect;
mod recolor;
mod mulcolor;

// Combinator helper structs
use self::translate::*;
use self::rotate::*;
use self::scale::*;
use self::combine::*;
use self::recolor::*;
use self::mulcolor::*;

// User types
pub use self::image::*;
pub use self::rect::*;

/// Trait for structs to be drawn with `Frame::draw`
pub trait Shape: IntoIterator<Item = RendTri> {
    /// Combine shapes together so they become one shape.
    #[inline]
    fn combine<S: Shape>(self, rhs: S) -> Combine<Self, S> where Self: Sized {
        Combine(self, rhs)
    }

    /// Translate a shape using a `vector` which represents the direction and magnitude to translate it by.
    ///
    /// ## Example
    /// ```rust,no_run
    /// use nest::*;
    /// let mut app = Window::new("Example", 640, 480).unwrap();
    /// app.draw(Rect([-0.5, -0.5], [0.5, 0.5]).translate([0.1, 0.1]));
    /// ```
    #[inline]
    fn translate<V: Into<cgm::Vector2<f32>>>(&self, vector: V) -> Translate<Self> where Self: Clone {
        Translate::new(self.clone(), vector.into())
    }

    /// Rotate a shape using an angle in radians.
    ///
    /// ## Example
    /// ```rust,no_run
    /// use nest::*;
    /// use std::f32::consts::PI;
    /// let mut app = Window::new("Example", 640, 480).unwrap();
    /// app.draw(Rect([-0.5, -0.5], [0.5, 0.5]).rotate(PI));
    /// ```
    #[inline]
    fn rotate(&self, angle: f32) -> Rotate<Self> where Self: Clone {
        Rotate::new(self.clone(), angle)
    }

    /// Scale a shape using a `vector` which represents the magnitude to scale
    /// it by.
    ///
    /// ## Example
    /// ```rust,no_run
    /// use nest::*;
    /// let mut app = Window::new("Example", 640, 480).unwrap();
    /// app.draw(Rect([-0.5, -0.5], [0.5, 0.5]).scale_both([0.1, 0.1]));
    /// ```
    #[inline]
    fn scale_both<V: Into<cgm::Vector2<f32>>>(&self, scale: V) -> Scale<Self> where Self: Clone {
        Scale::new(self.clone(), scale.into())
    }

    /// Scale a shape in both dimensions using a `f32` which represents the
    /// magnitude to scale it by.
    ///
    /// ## Example
    /// ```rust,no_run
    /// use nest::*;
    /// let mut app = Window::new("Example", 640, 480).unwrap();
    /// app.draw(Rect([-0.5, -0.5], [0.5, 0.5]).scale(0.1));
    /// ```
    #[inline]
    fn scale(&self, scale: f32) -> Scale<Self> where Self: Clone {
        Scale::new(self.clone(), cgm::Vector2::new(scale, scale))
    }

    /// Scale a shape in the x dimension using a `f32` which represents the
    /// magnitude to scale it by.
    ///
    /// ## Example
    /// ```rust,no_run
    /// use nest::*;
    /// let mut app = Window::new("Example", 640, 480).unwrap();
    /// app.draw(Rect([-0.5, -0.5], [0.5, 0.5]).scale_x(0.1));
    /// ```
    #[inline]
    fn scale_x(&self, scale_x: f32) -> Scale<Self> where Self: Clone {
        Scale::new(self.clone(), cgm::Vector2::new(scale_x, 1.0))
    }

    /// Scale a shape in the y dimension using a `f32` which represents the
    /// magnitude to scale it by.
    ///
    /// ## Example
    /// ```rust,no_run
    /// use nest::*;
    /// let mut app = Window::new("Example", 640, 480).unwrap();
    /// app.draw(Rect([-0.5, -0.5], [0.5, 0.5]).scale_y(0.1));
    /// ```
    #[inline]
    fn scale_y(&self, scale_y: f32) -> Scale<Self> where Self: Clone {
        Scale::new(self.clone(), cgm::Vector2::new(1.0, scale_y))
    }

    /// Completely recolor a shape.
    ///
    /// This does not erase textures, just colors them. Calling recolor() will
    /// cause every component of the shape to change its color to the passed
    /// color and should not be used in most situations as it erases
    /// sub-component color information. This can be used to make
    /// silhouettes and some other effects, but is mostly not useful.
    ///
    /// ## Example
    /// ```rust,no_run
    /// use nest::*;
    /// let mut app = Window::new("Example", 640, 480).unwrap();
    /// app.draw(Rect([-0.5, -0.5], [0.5, 0.5]).recolor(Color::BLUE));
    /// ```
    #[inline]
    fn recolor<C: Into<Color>>(&self, color: C) -> Recolor<Self> where Self: Clone {
        Recolor::new(self.clone(), color.into())
    }

    /// Multiply all of the colors in the shape component-wise with
    /// the passed color. See `Color::multiply()`.
    ///
    /// ## Example
    /// ```rust,no_run
    /// use nest::*;
    /// let mut app = Window::new("Example", 640, 480).unwrap();
    /// // This will be drawn with Color([0.0, 0.0, 0.5, 1.0]).
    /// // It extracts only the blue and alpha components.
    /// app.draw(Rect([-0.5, -0.5], [0.5, 0.5])
    ///    .recolor([0.5, 0.5, 0.5, 1.0])
    ///    .mul_color(Color::BLUE));
    /// ```
    #[inline]
    fn mul_color<C: Into<Color>>(&self, color: C) -> Mulcolor<Self> where Self: Clone {
        Mulcolor::new(self.clone(), color.into())
    }

    /// Multiply all of the colors in the shape component-wise with
    /// the passed color. See `Color::multiply()`.
    ///
    /// ## Example
    /// ```rust,no_run
    /// use nest::*;
    /// let mut app = Window::new("Example", 640, 480).unwrap();
    /// // This will be drawn with Color([0.25, 0.5, 0.5, 1.0]).
    /// // It extracts half of the color components.
    /// app.draw(Rect([-0.5, -0.5], [0.5, 0.5])
    ///    .recolor([0.5, 1.0, 1.0, 1.0])
    ///    .scale_color(0.5));
    /// ```
    #[inline]
    fn scale_color(&self, scale: f32) -> Mulcolor<Self> where Self: Clone {
        Mulcolor::new(self.clone(), Color([scale, scale, scale, 1.0]))
    }

    /// Scales the transparrency/alpha value of the shape's color.
    ///
    /// ## Example
    /// ```rust,no_run
    /// use nest::*;
    /// let mut app = Window::new("Example", 640, 480).unwrap();
    /// // This will be drawn with Color([0.5, 1.0, 1.0, 0.5]).
    /// // It extracts half of the alpha component.
    /// app.draw(Rect([-0.5, -0.5], [0.5, 0.5])
    ///    .recolor([0.5, 1.0, 1.0, 1.0])
    ///    .scale_alpha(0.5));
    /// ```
    #[inline]
    fn scale_alpha(&self, scale: f32) -> Mulcolor<Self> where Self: Clone {
        Mulcolor::new(self.clone(), Color([1.0, 1.0, 1.0, scale]))
    }
}

impl<S> Shape for S where S: IntoIterator<Item = RendTri> {}

/// Renderable triangle which includes color and texture information.
#[derive(Clone, Debug)]
pub struct RendTri {
    pub(crate) tri: Tri,
    pub(crate) texture: Option<Rc<Texture2d>>,
}

impl RendTri {
    #[inline]
    fn map_pos<F: FnMut(cgm::Point2<f32>) -> cgm::Point2<f32>>(mut self, f: F) -> RendTri {
        self.tri.positions = self.tri.positions.map(f);
        self
    }

    #[inline]
    fn map_tex<F: FnMut(cgm::Point2<f32>) -> cgm::Point2<f32>>(mut self, f: F) -> RendTri {
        self.tri.texcoords = self.tri.texcoords.map(f);
        self
    }

    #[inline]
    fn map_color<F: FnMut(Color) -> Color>(mut self, mut f: F) -> RendTri {
        self.tri.color = f(Color(self.tri.color)).0;
        self
    }

    #[inline]
    fn map_texture<T: Into<Option<Rc<Texture2d>>>>(mut self, t: T) -> RendTri {
        self.texture = t.into();
        self
    }
}

impl From<Tri> for RendTri {
    #[inline]
    fn from(tri: Tri) -> Self {
        RendTri {
            tri: tri,
            texture: None,
        }
    }
}

/// Three positions which form a matrix for shader purposes
#[derive(Copy, Clone, Debug)]
pub struct Positions(pub [[f32; 2]; 3]);

impl Positions {
    #[inline]
    fn map<F: FnMut(cgm::Point2<f32>) -> cgm::Point2<f32>>(self, mut f: F) -> Positions {
        Positions([f(self.0[0].into()).into(), f(self.0[1].into()).into(), f(self.0[2].into()).into()])
    }
}

/// A triangle primitive which enters the shader pipeline as a single vertex and is the only primitive in nest
#[derive(Copy, Clone, Debug)]
pub struct Tri {
    /// The three space vertices of the triangles
    pub positions: Positions,
    /// The three texture coordinates of the above vertices
    pub texcoords: Positions,
    /// The color of this triangle.
    pub color: [f32; 4],
}

impl Tri {
    /// Create a new triangle with points and tex coordinates specified
    #[inline]
    pub fn new<P: Into<cgm::Point2<f32>> + Copy, T: Into<cgm::Point2<f32>> + Copy, C: Into<Color>>(
        positions: [P; 3],
        texcoords: [T; 3],
        color: C,
    ) -> Tri {
        Tri {
            positions: Positions(
                [
                    positions[0].into().into(),
                    positions[1].into().into(),
                    positions[2].into().into(),
                ],
            ),
            texcoords: Positions(
                [
                    texcoords[0].into().into(),
                    texcoords[1].into().into(),
                    texcoords[2].into().into(),
                ],
            ),
            color: color.into().0,
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
            Color::WHITE,
        )
    }
}

implement_vertex!(Tri, positions, texcoords, color);

unsafe impl glium::vertex::Attribute for Positions {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32x2x3
    }
}
