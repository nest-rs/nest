
extern crate nest;
use nest::*;
use std::time::Instant;

fn main() {
	let mut app = Window::new("Window Example", 640, 480);

	let factor = 480.0 / 640.0;

	let mut shape = Transform::new(ColorRectangle {
		x: 0.0,
		y: 0.0,
		w: 0.7,
		h: 0.7,
		color: [0.2, 0.3, 0.4, 1.0],
	}).with_position(-0.2, -0.2)
		.with_scale(factor, 1.0);

	let mut last = Instant::now();

	'main: loop {
		{
			let curr = Instant::now();
			let delta = support::as_sec(curr - last);
			last = curr;

			let mut frame = app.next_frame();
			frame.clear();

			shape.rotate(delta);

			frame.draw_shape(&shape);

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
