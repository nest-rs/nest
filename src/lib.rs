#![crate_name = "love2d"]

#[macro_use]
extern crate glium;

#[macro_use]
mod support;
mod events;
mod frame;

pub use frame::Frame;
pub use events::Event;
pub use support::{Action, start_loop};
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
