
use img;
use glium;
use std::{io, error, fmt};

pub struct ImageParameters {
    pub dx: f32,
    pub dy: f32,
    pub dw: f32,
    pub dh: f32,
}

impl Default for ImageParameters {
    fn default() -> Self {
        ImageParameters {
            dx: 0.0,
            dy: 0.0,
            dw: 1.0,
            dh: 1.0,
        }
    }
}

#[derive(Debug)]
pub enum LoadImageError {
    FileError(io::Error),
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

pub struct Image {
    size: (u32, u32),
    texture: glium::texture::Texture2d,
}

impl Image {
    pub fn new(size: (u32, u32), texture: glium::texture::Texture2d) -> Self {
        Image {
            size: size,
            texture: texture,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn get_texture(&self) -> &glium::texture::Texture2d {
        &self.texture
    }
}
