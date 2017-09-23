#![warn(missing_docs)]

//! Nest is a crate to start protyping and making 2d graphics with as little effort as possible.
//! It's powerful combinators assist in making ordered transformations on different levels to
//! build larger and larger shapes.
//! 
//! This library does perform well, but not as well as rendering manually with custom shaders.
//! For almost any 2d application this performance will be sufficient, but this library is not designed
//! for high-performance drawing. If you are drawing more than a million polygons per frame, you may not
//! want to use this library, but profile as desired. We are still committed to improving the performance given our
//! API as much as possible.
//!
//! # Example
//!
//! ```rust,no_run
//! extern crate nest;
//! use nest::*;
//! 
//! fn main() {
//!     let mut app = Window::new("Example", 640, 480).expect("error: failed to open window");
//! 
//!     while !app.poll_events().any(|e| e == Event::Closed) {
//!         app.draw(rect([-0.5, -0.5], [0.5, 0.5]).translate((-0.1, -0.1)).combine(
//!             rect([-0.8, -0.8], [0.3, 0.3])).translate([0.1, 0.1]).rotate(0.5));
//!     }
//! }
//! ```

#[macro_use]
extern crate glium;
extern crate image as img;
extern crate cgmath as cgm;
#[macro_use]
extern crate error_chain;

mod window;
mod shape;
mod color;
mod event;
mod time;

pub use window::*;
pub use shape::*;
pub use color::*;
pub use event::*;
pub use time::*;
/// Re-export of `glium::glutin::ElementState`
pub use glium::glutin::ElementState as KeyState;
/// Re-export of `glium::glutin::VirtualKeyCode`
pub use glium::glutin::VirtualKeyCode as Key;
