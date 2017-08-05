
extern crate nest;
use nest::{Window, Event, Rect};

fn main() {
    let mut app = Window::new("Window Example", 640, 480).expect("error: failed to open window");

    'main: loop {
        {
            let mut frame = app.next_frame();
            frame.clear();

            frame.draw(Rect([-0.5, -0.5], [0.5, 0.5]));

            frame.finish();
        }

        for ev in app.poll_events() {
            match ev {
                Event::Closed => break 'main,
                _ => (),
            }
        }
    }
}
