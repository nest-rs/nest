//! Provides an easy-to-use Color struct for doing color manipulation
//! and providing colors to nest.

use glium::uniforms::{AsUniformValue, UniformValue};

/// Color in `(red, green, blue, alpha)` tuple form.
#[derive(Copy, Clone, Debug)]
pub struct Color(pub [f32; 4]);

impl Color {
    /// Scale the brightness of the color.
    pub fn scale(self, factor: f32) -> Color {
        Color([
            self.0[0] * factor,
            self.0[1] * factor,
            self.0[2] * factor,
            self.0[3],
        ])
    }

    /// Scale the transparency of the color.
    pub fn alpha(self, factor: f32) -> Color {
        Color([self.0[0], self.0[1], self.0[2], self.0[3] * factor])
    }

    /// Multiply the colors together (scale every component by each other).
    pub fn multiply<C: Into<Color>>(self, other: C) -> Color {
        let other = other.into();
        Color([
            self.0[0] * other.0[0],
            self.0[1] * other.0[1],
            self.0[2] * other.0[2],
            self.0[3] * other.0[3],
        ])
    }

    #[allow(missing_docs)]
    pub const WHITE: Color = Color([1.0, 1.0, 1.0, 1.0]);
    #[allow(missing_docs)]
    pub const YELLOW: Color = Color([1.0, 1.0, 0.0, 1.0]);
    #[allow(missing_docs)]
    pub const MAGENTA: Color = Color([1.0, 0.0, 1.0, 1.0]);
    #[allow(missing_docs)]
    pub const CYAN: Color = Color([0.0, 1.0, 1.0, 1.0]);
    #[allow(missing_docs)]
    pub const RED: Color = Color([1.0, 0.0, 0.0, 1.0]);
    #[allow(missing_docs)]
    pub const GREEN: Color = Color([0.0, 1.0, 0.0, 1.0]);
    #[allow(missing_docs)]
    pub const BLUE: Color = Color([0.0, 0.0, 1.0, 1.0]);
    #[allow(missing_docs)]
    pub const BLACK: Color = Color([0.0, 0.0, 0.0, 1.0]);
}

impl AsUniformValue for Color {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Vec4(self.0)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(tup: (u8, u8, u8, u8)) -> Color {
        Color([
            tup.0 as f32 / 255.0,
            tup.1 as f32 / 255.0,
            tup.2 as f32 / 255.0,
            tup.3 as f32 / 255.0,
        ])
    }
}

impl From<[u8; 4]> for Color {
    fn from(arr: [u8; 4]) -> Color {
        Color([
            arr[0] as f32 / 255.0,
            arr[1] as f32 / 255.0,
            arr[2] as f32 / 255.0,
            arr[3] as f32 / 255.0,
        ])
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(tup: (f32, f32, f32, f32)) -> Color {
        Color([tup.0, tup.1, tup.2, tup.3])
    }
}

impl From<[f32; 4]> for Color {
    fn from(arr: [f32; 4]) -> Color {
        Color(arr)
    }
}
