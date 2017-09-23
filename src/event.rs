use glium::glutin::{self, ElementState, MouseButton, VirtualKeyCode, WindowEvent};

/// Represents events passed form `Window`.
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    /// Window resize event `(x, y)`
    Resized(u32, u32),
    /// Window close request, usualy the close button
    Closed,
    /// Keyboard event as a `char`
    RecievedCharacter(char),
    /// Keyboard event with key state
    KeyboardInput(ElementState, Option<VirtualKeyCode>),
    /// Mouse move event `(x, y)`
    MouseMoved(f64, f64),
    /// Mouse button event with button state
    MouseInput(ElementState, MouseButton),
    /// Mouse wheel event with delta value
    MouseWheel(f32),
    /// And unsupported event
    Unsupported,
}

pub(crate) fn translate(ev: glutin::Event) -> Event {
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
                    Event::MouseMoved(position.0, position.1)
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
