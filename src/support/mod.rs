/*! 
Method, types, and structs that are not appart of any library component, but
are useful in tandom with library components.
 */

extern crate glium;

#[doc(hidden)]
pub mod shaders;
#[doc(hidden)]
pub mod vertex;
#[doc(hidden)]
pub mod buffer;

use std::time::Duration;

/// Represents the desired action for `start_loop(...)`, eigher `Continue` or `Stop`
pub enum Action {
    /// Continue the loop
    Continue,
    /// Stop the loop and return
    Stop,
}

/// Return a `Duration` represented as `f64` decimal seconds.
pub fn as_sec(elapsed: Duration) -> f64 {
    elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1000000000.0
}

/// Support method that calls a closure in loop untill recieving `Action::Stop`
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
