extern crate nest;
use nest::{Window, Event, Rect};
use std::iter::empty;

fn main() {
    let mut app = Window::new("Window Example", 640, 480).expect("error: failed to open window");

    while !app.poll_events().any(|e| e == Event::Closed) {
        app.draw(empty());
    }
}