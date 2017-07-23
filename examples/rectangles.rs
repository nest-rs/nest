
extern crate simple;
use simple::{Window, Event, Rectangle, Circle};

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

			frame.set_color(0.0, 0.2, 0.5, 1.0);
			frame.draw_shape(Rectangle {
				x: -0.5,
				y: -0.5,
				w: 1.0,
				h: 1.0,
			});

			frame.set_color(0.5, 0.2, 0.0, 1.0);
			frame.draw_shape(Circle {
				x: 0.0,
				y: 0.0,
				rx: 0.2,
				ry: 0.2,
				step_size: 10,
			});

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
