
use glium;
use super::{Frame, Event, Image, LoadImageError};
use glium::glutin;
use std::path;
use std::time::Instant;
use support::{self, as_sec};
use events;
use img;

pub struct Window {
    display: glium::Display,
    events_loop: glium::glutin::EventsLoop,
    programs: (glium::Program, glium::Program),
    last: Instant,
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

    pub fn load_image<'a, P: AsRef<path::Path>>(&self, path: P) -> Result<Image, LoadImageError> {
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

        match img::load_from_memory(&buf[..]) {
            Ok(image) => {
                let image = image.to_rgba();
                let image_dimensions = image.dimensions();
                let glimage =
                    glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
                let texture = glium::texture::Texture2d::new(&self.display, glimage).unwrap();

                return Ok(Image::new(image_dimensions, texture));
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
