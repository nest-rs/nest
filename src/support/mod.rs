
extern crate glium;

// MARK: Exports
pub mod shaders;
pub mod vertex;
pub mod buffer;

pub use self::vertex::Vertex;

use std::time::{Instant, Duration};

// MARK: Loop

pub enum Action {
    Continue,
    Stop,
}

pub fn as_sec(elapsed: Duration) -> f64 {
    elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1000000000.0
}

pub fn start_loop<F>(mut callback: F)
where
    F: FnMut(f64) -> Action,
{
    let mut last = Instant::now();

    loop {
        let curr = Instant::now();
        let delta: f64 = as_sec(curr - last);
        last = curr;

        match callback(delta) {
            Action::Stop => break,
            Action::Continue => (),
        }
    }
}
