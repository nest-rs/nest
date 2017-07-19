
extern crate love2d;
use love2d::{Window, Action, Event};
use love2d::{ElementState, VirtualKeyCode};

use std::collections::HashMap;

fn main() {
    let mut window = Window::new("Hello World", 640, 480);
    let mut keymap: HashMap<VirtualKeyCode, bool> = HashMap::new();

    window.start_loop(|app, _| {
        app.clear();

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
        app.draw_circle(0.25, -0.25, 0.75, 0.25, 10);

        for ev in app.poll_events() {
            match ev {
                Event::Closed => return Action::Stop,
                Event::KeyboardInput(ElementState::Pressed, Some(key)) => {
                    match key {
                        VirtualKeyCode::Space => println!("Space!"),
                        VirtualKeyCode::Escape => return Action::Stop,
                        _ => (),
                    };
                    keymap.insert(key, true);
                }
                Event::KeyboardInput(ElementState::Released, Some(key)) => {
                    keymap.insert(key, false);
                }
                _ => (),
            }
        }

        Action::Continue
    });
}
