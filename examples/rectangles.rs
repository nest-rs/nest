extern crate nest;
use nest::{Window, Event, Rect};

fn main() {
    let mut app = Window::new("Window Example", 640, 480).expect("error: failed to open window");

    while !app.poll_events().any(|e| e == Event::Closed) {
        app.draw(Rect([-0.5, -0.5], [0.5, 0.5]).into_iter().chain(
            Rect([-0.8, -0.8], [0.3, 0.3])));
    }
}
