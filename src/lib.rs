#![crate_name = "love2d"]

#[macro_use]
extern crate glium;
extern crate image;

#[macro_use]
mod support;
mod events;
mod frame;

pub use frame::Frame;
pub use events::Event;
pub use support::{Action, start_loop, ImageParameters};
pub use glium::glutin::{ElementState, VirtualKeyCode};

use glium::glutin;
use std::time::Instant;
use support::as_sec;

pub struct Window {
    display: glium::Display,
    events_loop: glium::glutin::EventsLoop,
    programs: (glium::Program, glium::Program),
    last: Instant,
}

#[derive(Debug)]
pub enum LoadImageError {
    FileError(std::io::Error),
    ImageError(image::ImageError),
}

impl std::error::Error for LoadImageError {
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

impl std::fmt::Display for LoadImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn get_texture(&self) -> &glium::texture::Texture2d {
        &self.texture
    }
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);
        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop).unwrap();
        let color_program = support::shaders::color::load_program(&display).unwrap();
        let texture_program = support::shaders::texture::load_program(&display).unwrap();

        Window {
            display: display,
            events_loop: events_loop,
            programs: (color_program, texture_program),
            last: Instant::now(),
        }
    }

    pub fn load_image<'a, P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<Image, LoadImageError> {
        use std::io::prelude::*;
        use std::fs::File;

        let mut buf = Vec::new();
        match File::open(path) {
            Ok(mut file) => {
                match file.read_to_end(&mut buf) {
                    Ok(_) => (),
                    Err(err) => return Err(LoadImageError::FileError(err)),
                }
            }
            Err(err) => return Err(LoadImageError::FileError(err)),
        }

        match image::load_from_memory(&buf[..]) {
            Ok(image) => {
                let image = image.to_rgba();
                let image_dimensions = image.dimensions();
                let glimage = glium::texture::RawImage2d::from_raw_rgba_reversed(
                    &image.into_raw(),
                    image_dimensions,
                );
                let texture = glium::texture::Texture2d::new(&self.display, glimage).unwrap();

                return Ok(Image {
                    size: image_dimensions,
                    texture: texture,
                });
            }
            Err(err) => return Err(LoadImageError::ImageError(err)),
        }
    }

    pub fn poll_events(&mut self) -> Vec<Event> {
        let mut events: Vec<Event> = Vec::new();

        self.events_loop.poll_events(|ev| {
            let event = events::translate(ev);
            match event {
                Event::Unsupported => (),
                event => events.push(event),
            }
        });

        events
    }

    pub fn next_frame(&mut self) -> Frame {
        let curr = Instant::now();
        let delta = as_sec(curr - self.last);
        self.last = curr;

        Frame::new(&self.display, &self.programs, delta)
    }
}
