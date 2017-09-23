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
use nest::*;

fn main() {
    let mut app = Window::new("Example", 640, 480).expect("error: failed to open window");

    while !app.poll_events().any(|e| e == Event::Closed) {
        app.draw(rect([-0.5, -0.5], [0.5, 0.5]).translate((-0.1, -0.1)).combine(
            rect([-0.8, -0.8], [0.3, 0.3])).translate([0.1, 0.1]).rotate(0.5));
    }
}
```

## Licence

nest uses the [MIT](LICENCE) licence
