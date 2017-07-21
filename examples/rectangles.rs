
extern crate love2d;
use love2d::{Window, Event};

fn main() {
	let mut app = Window::new("Window Example", 640, 480);

	'main: loop {
		{
			let mut frame = app.next_frame();
			frame.clear();

			frame.set_color(1.0, 0.0, 0.0, 1.0);
			frame.draw_rect(0.0, 0.0, 1.0, 1.0);

			frame.set_color(0.0, 1.0, 0.0, 1.0);
			frame.draw_rect(0.0, 0.0, -1.0, 1.0);

			frame.set_color(0.0, 0.0, 1.0, 1.0);
			frame.draw_rect(0.0, 0.0, 1.0, -1.0);

			frame.set_color(1.0, 1.0, 1.0, 1.0);
			frame.draw_rect(0.0, 0.0, -1.0, -1.0);

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
