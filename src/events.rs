
extern crate glium;
use glium::glutin;
use glutin::{ElementState, MouseButton};

use glutin::VirtualKeyCode;

pub enum Event {
	Resized(u32, u32),
	Closed,
	RecievedCharacter(char),
	KeyboardInput(ElementState, Option<VirtualKeyCode>),
	MouseMoved(f32, f32),
	MouseInput(ElementState, MouseButton),
	MouseWheel(f32),
	Unsupported,
}

pub fn translate(ev: glutin::Event) -> Event {
	use glutin::WindowEvent;

	return match ev {
		glutin::Event::WindowEvent { event, .. } => {
			match event {
				WindowEvent::Resized(w, h) => Event::Resized(w, h),
				WindowEvent::Closed => Event::Closed,
				WindowEvent::ReceivedCharacter(c) => Event::RecievedCharacter(c),
				WindowEvent::KeyboardInput { input, .. } => {
					Event::KeyboardInput(input.state, input.virtual_keycode)
				}
				WindowEvent::MouseMoved { position, .. } => {
					Event::MouseMoved(position.0 as f32, position.1 as f32)
				}
				WindowEvent::MouseInput { state, button, .. } => Event::MouseInput(state, button),
				WindowEvent::MouseWheel { delta, .. } => {
					match delta {
						glutin::MouseScrollDelta::PixelDelta(_, y) => Event::MouseWheel(y),
						_ => Event::Unsupported,
					}
				}
				_ => Event::Unsupported,
			}
		}
		_ => Event::Unsupported,
	};
}
