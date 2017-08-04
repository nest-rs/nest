#![crate_name = "nest"]
#![warn(missing_docs)]
#![deny(warnings)]

//! nest is a glium port of the [nest](https://crates.io/crates/nest)
//! crate which is a port of the [nest](https://nest.org/) lua graphics
//! library. This library was developed as a 2D graphics prototyping library and
//! is not ment for high-performance.
//!
//! # Example
//!
//! ```rust,no_run
//! extern crate nest;
//! use nest::{Window, Event};
//!
//! fn main() {
//! let mut app = Window::new("Window Example", 640, 480);
//!
//! 'main: loop {
//! Note rust requires this code to be in a closure to please the borrow checker
//! {
//! let mut frame = app.next_frame();
//! frame.clear();
//!
//! frame.set_color(1.0, 1.0, 1.0, 1.0);
//! frame.draw_rect(0.0, 0.0, 1.0, 1.0);
//!
//! frame.finish();
//! }
//!
//! for ev in app.poll_events() {
//! match ev {
//! Event::Closed => break 'main,
//! _ => (),
//! }
//! }
//! }
//! }
//! ```
//!
//! Current renderable objects are:
//!
//! Line (soon to be depricated)
//! Polygon (TrianglFan)
//! Rectangle
//! Circle (really an oval)
//! Image (with cropping)
//! Color (HTML / RGBA)
//! Alpha Blend Transparency
//!
//! Planned features include:
//!
//! - Rotation
//! - Text (ttf fonts)
//! - Text (char-map)
//! - Audio
//!

#[macro_use]
extern crate glium;
extern crate image as img;

pub mod support;
mod frame;
mod window;
mod image;
mod shape;

pub use window::Window;
pub use frame::Frame;
pub use shape::*;
pub use support::events::Event;
pub use support::vertex;
pub use support::shaders::ShaderMode;
pub use image::Image;
pub use image::LoadImageError;
pub use image::ImageParameters;
/// Re-export of `glium::glutin::ElementState`
pub use glium::glutin::ElementState;
/// Re-export of `glium::glutin::VirtualKeyCode`
pub use glium::glutin::VirtualKeyCode;
