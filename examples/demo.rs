extern crate nest;

use nest::*;

fn main() {
    let mut app = Window::new("Demo", 640, 480).expect("error: failed to open window");

    loop {
        // Handle events.
        for event in app.poll_events() {
            match event {
                // Close if they close the window or hit escape.
                Event::Closed | Event::KeyboardInput(KeyState::Pressed, Some(Key::Escape)) => return,
                // Print "Space!" if they hit space.
                Event::KeyboardInput(KeyState::Pressed, Some(Key::Space)) => println!("Space!"),
                _ => {}
            }
        }
    }
}
