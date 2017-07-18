#[macro_use]
extern crate glium;
use glium::glutin;

use std::collections::HashMap;

#[macro_use]
mod support;

enum Shape {
    Clear([f32; 4]),
    Rectangle(f32, f32, f32, f32, [f32; 4]),
    Triangle(f32, f32, f32, f32, f32, f32, [f32; 4]),
    Line(f32, f32, f32, f32, [f32; 4]),
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
                    &Shape::Rectangle(x1, y1, x2, y2, color) => {
                        let vert_buff = support::buffer::rectangle_vert_buff(
                            &self.display,
                            x1,
                            y1,
                            x2,
                            y2,
                            color,
                        ).unwrap();
                        let indices =
                            glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
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
                    &Shape::Triangle(x1, y1, x2, y2, x3, y3, color) => {
                        let vert_buff = support::buffer::triangle_vert_buff(
                            &self.display,
                            x1,
                            y1,
                            x2,
                            y2,
                            x3,
                            y3,
                            color,
                        ).unwrap();
                        let indices =
                            glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
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

    pub fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let shape = Shape::Rectangle(x, y, x + width, y + height, self.color);
        self.shapes.push(shape);
    }

    pub fn draw_triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        let shape = Shape::Triangle(x1, y1, x2, y2, x3, y3, self.color);
        self.shapes.push(shape);
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let shape = Shape::Line(x1, y1, x2, y2, self.color);
        self.shapes.push(shape);
    }
}

fn main() {
    let mut window = Window::new("Hello World", 640, 480);

    window.start_loop(|app, delta| {
        app.clear_color(0.1, 0.3, 0.2, 1.0);

        app.set_color(0.3, 0.1, 0.2, 1.0);
        app.draw_rect(0.0, 0.0, 10.0, 10.0);

        app.set_color(1.0, 0.0, 0.0, 1.0);
        app.draw_line(0.0, 0.0, 1.0, 1.0);

        app.set_color(0.0, 0.0, 1.0, 0.1);
        app.draw_triangle(0.0, 0.0, 1.0, 1.0, 1.0, 0.0);

        support::Action::Continue
    });
}
