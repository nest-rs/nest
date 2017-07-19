#![crate_name = "love2d"]

#[macro_use]
extern crate glium;

#[macro_use]
mod support;
mod draw;
mod events;

pub use support::Action;
pub use events::Event;
pub use glium::glutin::{ElementState, VirtualKeyCode};
use draw::Shape;
use glium::glutin;

pub struct Window {
    display: glium::Display,
    events_loop: glium::glutin::EventsLoop,
    color: [f32; 4],
    shapes: Vec<Shape>,
    events: Vec<Event>,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);
        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        Window {
            display: display,
            events_loop: events_loop,
            color: [1.0, 1.0, 1.0, 1.0],
            shapes: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn poll_events(&mut self) -> Vec<Event> {
        self.events.drain(..).collect()
    }

    pub fn start_loop<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut Window, f32) -> support::Action,
    {
        let program = support::shaders::load_program(&self.display).unwrap();
        let params = glium::DrawParameters {
            blend: glium::draw_parameters::Blend::alpha_blending(),
            ..Default::default()
        };

        use glium::Surface;
        support::start_loop(|delta| {

            let mut target = self.display.draw();
            let rtn = callback(self, delta);

            for shape in self.shapes.iter() {
                match shape {
                    &Shape::Clear(color) => {
                        target.clear_color(color[0], color[1], color[2], color[3])
                    }
                    &Shape::Line(x1, y1, x2, y2, color) => {
                        let vert_buff =
                            support::buffer::line_vert_buff(&self.display, x1, y1, x2, y2, color)
                                .unwrap();
                        let indices =
                            glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);
                        target
                            .draw(
                                &vert_buff,
                                &indices,
                                &program,
                                &glium::uniforms::EmptyUniforms,
                                &params,
                            )
                            .unwrap();
                    }
                    &Shape::Polygon(ref points, color) => {
                        let vert_buff =
                            support::buffer::poly_vert_buffer(&self.display, &points, color)
                                .unwrap();
                        let indices =
                            glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);
                        target
                            .draw(
                                &vert_buff,
                                &indices,
                                &program,
                                &glium::uniforms::EmptyUniforms,
                                &params,
                            )
                            .unwrap();
                    }
                }
            }

            self.shapes.clear();

            target.finish().unwrap();

            let mut events: Vec<Event> = Vec::new();
            self.events_loop.poll_events(|ev| {
                let event = events::translate(ev);
                match event {
                    Event::Unsupported => (),
                    event => events.push(event),
                }
            });

            for event in events {
                self.events.push(event);
            }

            rtn
        });
    }
}
