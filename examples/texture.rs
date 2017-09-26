extern crate nest;

use nest::*;
use std::time::Instant;

fn main() {
    let mut app = Window::new("Demo", 640, 480).expect("error: failed to open window");
    let city = app.load_image("examples/city.jpg").unwrap();

    let start = Instant::now();

    while !app.poll_events().any(|e| e == Event::Closed) {
        app.draw(image([0.0, 0.0], [0.5, 0.5], city.clone()).rotate(start.elapsed().to_secs()));
    }
}
