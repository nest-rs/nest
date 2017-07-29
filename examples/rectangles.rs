
extern crate nest;
use nest::{Window, Event, ColorRectangle};

fn main() {
	let mut app = Window::new("Window Example", 640, 480);

	'main: loop {
		{
			let mut frame = app.next_frame();
			frame.clear();

			frame.draw_rect(0.0, 0.0, 1.0, 1.0, [1.0, 0.0, 0.0, 1.0]);
			frame.draw_rect(0.0, 0.0, -1.0, 1.0, [0.0, 1.0, 0.0, 1.0]);
			frame.draw_rect(0.0, 0.0, 1.0, -1.0, [0.0, 0.0, 1.0, 1.0]);
			frame.draw_rect(0.0, 0.0, -1.0, -1.0, [1.0, 1.0, 1.0, 1.0]);

			frame.draw_shape(&ColorRectangle {
				x: -0.5,
				y: -0.5,
				w: 1.0,
				h: 1.0,
				color: [0.0, 0.2, 0.5, 0.7]
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
