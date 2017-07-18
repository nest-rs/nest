#[macro_use]
extern crate glium;
use glium::glutin;

use std::collections::HashMap;

#[macro_use]
mod support;

pub type Color = [f32; 4];

enum Shape {
    Clear(Color),
    Polygon(Vec<(f32, f32)>, Color),
    Line(f32, f32, f32, f32, Color),
}

pub struct Window {
    display: glium::Display,
    events_loop: glium::glutin::EventsLoop,
    color: [f32; 4],
    shapes: Vec<Shape>,
    keys: HashMap<glutin::VirtualKeyCode, bool>,
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
            keys: HashMap::new(),
        }
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

            let mut rtn = callback(self, delta);
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

            let mut keys: HashMap<glutin::VirtualKeyCode, bool> = HashMap::new();
            self.events_loop.poll_events(|ev| match ev {
                glium::glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::Closed => rtn = support::Action::Stop,
                        glutin::WindowEvent::KeyboardInput { input, .. } => {
                            match input.virtual_keycode {
                                Some(key) => {
                                    keys.insert(
                                        key,
                                        match input.state {
                                            glutin::ElementState::Pressed => true,
                                            glutin::ElementState::Released => false,
                                        },
                                    );
                                }
                                None => (),
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            });

            for (key, value) in keys {
                self.keys.insert(key, value);
            }

            rtn
        });
    }

    pub fn clear_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        let shape = Shape::Clear([red, green, blue, alpha]);
        self.shapes.push(shape);
    }

    pub fn set_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.color = [red, green, blue, alpha];
    }

    pub fn set_color_html(&mut self, color: &str) {
        let mut bytes = color.as_bytes();
        if bytes[0] == b'#' {
            bytes = &bytes[1..];
        }

        use std::str::from_utf8;
        use std::u8;

        if bytes.len() == 3 {
            let r = u8::from_str_radix(from_utf8(&vec![bytes[0], bytes[0]]).unwrap(), 16).unwrap();
            let g = u8::from_str_radix(from_utf8(&vec![bytes[1], bytes[1]]).unwrap(), 16).unwrap();
            let b = u8::from_str_radix(from_utf8(&vec![bytes[2], bytes[2]]).unwrap(), 16).unwrap();
            self.set_color(
                (r as f32) / 255.0,
                (g as f32) / 255.0,
                (b as f32) / 255.0,
                1.0,
            );
        } else if bytes.len() == 6 {
            let r = u8::from_str_radix(from_utf8(&bytes[0..2]).unwrap(), 16).unwrap();
            let g = u8::from_str_radix(from_utf8(&bytes[2..4]).unwrap(), 16).unwrap();
            let b = u8::from_str_radix(from_utf8(&bytes[4..6]).unwrap(), 16).unwrap();
            self.set_color(
                (r as f32) / 255.0,
                (g as f32) / 255.0,
                (b as f32) / 255.0,
                1.0,
            );
        }
    }

    pub fn draw(&mut self, points: &[(f32, f32)]) {
        let shape = Shape::Polygon(Vec::from(points), self.color);
        self.shapes.push(shape);
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let shape = Shape::Line(x1, y1, x2, y2, self.color);
        self.shapes.push(shape);
    }

    pub fn draw_rect(&mut self, p1: (f32, f32), p2: (f32, f32)) {
        let shape = Shape::Polygon(
            vec![(p1.0, p1.1), (p1.0, p2.1), (p2.0, p2.1), (p2.0, p1.1)],
            self.color,
        );
        self.shapes.push(shape);
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, rx: f32, ry: f32, step_size: u32) {
        let circle: Vec<(f32, f32)> = (0u32..360)
            .filter(|d| d % step_size == 0)
            .map(|d| {
                let r = (d as f32).to_radians();
                (r.cos() * rx, r.sin() * ry)
            })
            .collect();
        let shape = Shape::Polygon(circle, self.color);
        self.shapes.push(shape);
    }
}

fn main() {
    let mut window = Window::new("Hello World", 640, 480);

    window.start_loop(|app, delta| {
        app.clear_color(0.1, 0.3, 0.2, 1.0);

        app.set_color_html("312");
        app.draw_rect((-0.5, -0.5), (0.5, 0.5));

        app.set_color_html("#033112");
        app.draw_rect((0.0, 0.0), (-1.0, -1.0));

        app.set_color(1.0, 0.0, 0.0, 1.0);
        app.draw_line(0.0, 0.0, 1.0, 1.0);

        app.set_color(0.0, 0.0, 1.0, 0.3);
        app.draw(&[(0.0, 0.0), (1.0, 1.0), (1.0, 0.0)]);

        app.set_color(0.0, 1.0, 0.0, 0.3);
        app.draw(&[(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)]);

        app.set_color(1.0, 0.0, 0.0, 0.3);
        app.draw_circle(0.0, 0.0, 0.75, 0.25, 10);

        support::Action::Continue
    });
}
