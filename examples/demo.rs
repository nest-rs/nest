
extern crate nest;
use nest::{Window, Event};
use nest::{Rectangle, Circle, ShaderMode};
use nest::{ElementState, VirtualKeyCode};
use nest::support::{self, from_html};
use nest::vertex::color::Vertex;

use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let mut app = Window::new("Hello World", 640, 480);
    let mut keymap: HashMap<VirtualKeyCode, bool> = HashMap::new();
    let pic = app.load_image("examples/city.jpg").unwrap();

    let mut time = 0.0;
    let mut last = Instant::now();

    'main: loop {
        {
            let mut frame = app.next_frame();
            frame.clear();

            let curr = Instant::now();
            let delta = support::as_sec(curr - last);
            last = curr;

            time += delta;

            frame.draw_rect(-0.5, -0.5, 0.5, 0.5, from_html("312"));

            frame.draw_rect(0.0, 0.0, -1.0, -1.0, from_html("#033112"));

            let color = [0.0, 0.0, 1.0, 0.3];
            frame.draw(&[Vertex::new(0.0, 0.0, color), Vertex::new(1.0, 1.0, color), Vertex::new(1.0, 0.0, color)], ShaderMode::Color);

            let color = [0.0, 1.0, 0.0, 0.3];
            frame.draw(&[Vertex::new(0.0, 0.0, color), Vertex::new(0.0, 1.0, color), Vertex::new(1.0, 1.0, color)], ShaderMode::Color);

            frame.draw_shape(&Circle {
                x: 0.25 * time.sin(),
                y: -0.25 * time.cos(),
                rx: 0.75,
                ry: 0.25,
                step_size: 10,
                color: [1.0, 0.0, 0.0, 0.3],
            });

            frame.draw_image(
                &pic,
                Rectangle {
                    x: -1.0,
                    y: 0.0,
                    w: 1.0,
                    h: 1.0,
                },
                Some(Rectangle {
                    x: 0.3,
                    y: 0.0,
                    w: 0.3,
                    h: 1.0,
                }),
            );
            frame.draw_image(
                &pic,
                Rectangle {
                    x: -0.5,
                    y: 0.0,
                    w: 0.5,
                    h: 0.5,
                },
                None,
            );

            frame.draw_rect(0.0, 0.0, 1.0, 1.0, [0.0, 1.0, 1.0, 0.1]);

            frame.finish();
        }

        for ev in app.poll_events() {
            use Event::*;
            use VirtualKeyCode::*;

            match ev {
                Closed => break 'main,
                KeyboardInput(ElementState::Pressed, Some(key)) => {
                    match key {
                        Space => println!("Space!"),
                        Escape => break 'main,
                        _ => (),
                    };
                    keymap.insert(key, true);
                }
                KeyboardInput(ElementState::Released, Some(key)) => {
                    keymap.insert(key, false);
                }
                _ => (),
            }
        }
    }
}
