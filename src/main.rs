
extern crate love2d;
use love2d::{Window, Action};

fn main() {
    let mut window = Window::new("Hello World", 640, 480);

    window.start_loop(|app, delta| {
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
        app.draw_circle(0.0, 0.0, 0.75, 0.25, 10);

        Action::Continue
    });
}
