Nest
====
> A 2d graphics crate with propagated transformation

## Usage

Cargo.toml
```
[dependencies]
nest = { git = "https://github.com/twh2898/nest.git" }
```

## Example

```rust
extern crate nest;
use nest::{Window, Event, Rectangle, Circle};

fn main() {
	let mut app = Window::new("Nest Example", 640, 480);

	'main: loop {
		{
			let mut frame = app.next_frame();
			frame.clear();

			frame.set_color(0.0, 0.2, 0.5, 1.0);
			frame.draw_shape(Rectangle {
				x: -0.5,
				y: -0.5,
				w: 1.0,
				h: 1.0,
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
```

## Licence

nest uses the [MIT](LICENCE) licence
