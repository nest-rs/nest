
extern crate nest;
use nest::{Window, Event, ImageParameters};
use nest::{Rectangle, Circle};
use nest::{ElementState, VirtualKeyCode};

use std::collections::HashMap;

fn main() {
    let mut app = Window::new("Hello World", 640, 480);
    let mut keymap: HashMap<VirtualKeyCode, bool> = HashMap::new();
    let pic = app.load_image("examples/city.jpg").unwrap();

    let mut time = 0.0;

    'main: loop {
        {
            let mut frame = app.next_frame();
            frame.clear();

            let delta = frame.delta();
            time += delta;

            frame.set_color_html("312");
            frame.draw_rect(-0.5, -0.5, 0.5, 0.5);

            frame.set_color_html("#033112");
            frame.draw_rect(0.0, 0.0, -1.0, -1.0);

            frame.set_color(0.0, 0.0, 1.0, 0.3);
            frame.draw(&[(0.0, 0.0), (1.0, 1.0), (1.0, 0.0)]);

            frame.set_color(0.0, 1.0, 0.0, 0.3);
            frame.draw(&[(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)]);

            frame.set_color(1.0, 0.0, 0.0, 0.3);
            frame.draw_shape(Circle {
                x: 0.25 * time.sin(),
                y: -0.25 * time.cos(),
                rx: 0.75,
                ry: 0.25,
                step_size: 10,
            });

            frame.draw_image(
                &pic,
                Rectangle {
                    x: -1.0,
                    y: 0.0,
                    w: 1.0,
                    h: 1.0,
                },
                ImageParameters {
                    dx: 0.3,
                    dy: 0.0,
                    dw: 0.3,
                    dh: 1.0,
                },
            );
            frame.draw_image(
                &pic,
                Rectangle {
                    x: -0.5,
                    y: 0.0,
                    w: 0.5,
                    h: 0.5,
                },
                Default::default(),
            );

            frame.set_color(0.0, 1.0, 1.0, 0.1);
            frame.draw_rect(0.0, 0.0, 1.0, 1.0);

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
