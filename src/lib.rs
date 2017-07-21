#![crate_name = "love2d"]

#[macro_use]
extern crate glium;
extern crate image as img;

#[macro_use]
mod support;
mod events;
mod frame;
mod window;
mod image;

pub use window::Window;
pub use frame::Frame;
pub use events::Event;
pub use image::{Image, LoadImageError, ImageParameters};
pub use support::{Action, start_loop};
pub use glium::glutin::{ElementState, VirtualKeyCode};
