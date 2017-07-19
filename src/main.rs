
extern crate love2d;
use love2d::{Window, Action, Event};
use love2d::{ElementState, VirtualKeyCode};

fn main() {
    let mut window = Window::new("Hello World", 640, 480);

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
                Event::KeyboardInput(ElementState::Pressed, Some(VirtualKeyCode::Space)) => {
                    println!("Space!");
                }
                _ => (),
            }
        }

        Action::Continue
    });
}
