
extern crate simple;
use simple::{Window, Event};

fn main() {
	let mut app = Window::new("Window Example", 640, 480);

	'main: loop {
		{
			let mut frame = app.next_frame();
			frame.clear();

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
