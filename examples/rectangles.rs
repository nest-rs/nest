extern crate nest;
use nest::*;

fn main() {
    let mut app = Window::new("Rectangle Example", 640, 480).expect("error: failed to open window");

    while !app.poll_events().any(|e| e == Event::Closed) {
        app.draw(
            rect([-0.5, -0.5], [0.5, 0.5])
                .combine(rect([-0.8, -0.8], [0.3, 0.3]).recolor(Color::CYAN)),
        );
    }
}
