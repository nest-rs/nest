
use img;
use glium;
use std::{io, error, fmt};

/// Represets the UVW coordinates for drawing an immage, used with
/// `Frame::draw_image(...)`
pub struct ImageParameters {
    /// U or X location on the texture on the range `0.0 - 1.0`
    pub dx: f32,
    /// V or Y location on the texture on the range `0.0 - 1.0`
    pub dy: f32,
    /// U width on the texture on the range `0.0 - 1.0`
    pub dw: f32,
    /// V height on the texture on the range `0.0 - 1.0`
    pub dh: f32,
}

impl Default for ImageParameters {
    /// The default crop which is the entire image
    ///
    /// ```rust,no_run
    /// # extern crate love2d;
    /// # fn main() {
    /// # use love2d::ImageParameters;
    /// # let params =
    /// ImageParameters {
    ///     dx: 0.0,
    ///     dy: 0.0,
    ///     dw: 1.0,
    ///     dh: 1.0,
    /// }
    /// # ;
    /// # }
    /// ```
    fn default() -> Self {
        ImageParameters {
            dx: 0.0,
            dy: 0.0,
            dw: 1.0,
            dh: 1.0,
        }
    }
}

/// Error type returned by `Window::load_image(...)`
#[derive(Debug)]
pub enum LoadImageError {
    /// Error reading the image file
    FileError(io::Error),
    /// Error with the image file format
    ImageError(img::ImageError),
}

impl error::Error for LoadImageError {
    fn description(&self) -> &str {
        match *self {
            LoadImageError::FileError(ref err) => err.description(),
            LoadImageError::ImageError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            LoadImageError::FileError(ref err) => Some(err),
            LoadImageError::ImageError(ref err) => Some(err),
        }
    }
}

impl fmt::Display for LoadImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoadImageError::FileError(ref err) => err.fmt(f),
            LoadImageError::ImageError(ref err) => err.fmt(f),
        }
    }
}

/// Represents an image that can be drawn with `Frame::draw_image(...)`.
pub struct Image {
    size: (u32, u32),
    texture: glium::texture::Texture2d,
}

impl Image {
    /// Creates a new image from a glium `Texture2d` and `(u32, u32)` size.
    pub fn new(size: (u32, u32), texture: glium::texture::Texture2d) -> Self {
        Image {
            size: size,
            texture: texture,
        }
    }

    /// Get the size in pixels of the image
    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    /// Get the texture for rendering.
    ///
    /// **Note**: This method is exposed only for the backend and is not used by
    /// the public API.
    pub fn get_texture(&self) -> &glium::texture::Texture2d {
        &self.texture
    }
}
