
extern crate glium;

pub mod shaders;
pub mod vertex;
pub mod buffer;

use std::time::Duration;

pub struct ImageParameters {
    pub dx: f32,
    pub dy: f32,
    pub dw: f32,
    pub dh: f32,
}

impl Default for ImageParameters {
    fn default() -> Self {
        ImageParameters {
            dx: 0.0,
            dy: 0.0,
            dw: 1.0,
            dh: 1.0,
        }
    }
}

pub enum Action {
    Continue,
    Stop,
}

pub fn as_sec(elapsed: Duration) -> f64 {
    elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1000000000.0
}

pub fn start_loop<F>(mut callback: F)
where
    F: FnMut() -> Action,
{
    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => (),
        }
    }
}
