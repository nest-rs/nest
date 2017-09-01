//! Method, types, and structs that are not appart of any library component, but
//! are useful in tandom with library components.

#[doc(hidden)]
pub mod vertex;
#[doc(hidden)]
pub mod events;

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

/// Set the color using a 3 or 6 character HTML color string
///
/// # Examples
/// `"#333"`
/// `"F25"`
/// `"3E59F4"`
/// `"#45F0D5"`
pub fn from_html(color: &str) -> [f32; 4] {
    let mut bytes = color.as_bytes();
    if bytes[0] == b'#' {
        bytes = &bytes[1..];
    }

    use std::str::from_utf8;
    use std::u8;

    if bytes.len() == 3 {
        let r = u8::from_str_radix(from_utf8(&vec![bytes[0], bytes[0]]).unwrap(), 16).unwrap();
        let g = u8::from_str_radix(from_utf8(&vec![bytes[1], bytes[1]]).unwrap(), 16).unwrap();
        let b = u8::from_str_radix(from_utf8(&vec![bytes[2], bytes[2]]).unwrap(), 16).unwrap();
        return [
            (r as f32) / 255.0,
            (g as f32) / 255.0,
            (b as f32) / 255.0,
            1.0,
        ];
    } else if bytes.len() == 6 {
        let r = u8::from_str_radix(from_utf8(&bytes[0..2]).unwrap(), 16).unwrap();
        let g = u8::from_str_radix(from_utf8(&bytes[2..4]).unwrap(), 16).unwrap();
        let b = u8::from_str_radix(from_utf8(&bytes[4..6]).unwrap(), 16).unwrap();
        return [
            (r as f32) / 255.0,
            (g as f32) / 255.0,
            (b as f32) / 255.0,
            1.0,
        ];
    }

    return [0.0, 0.0, 0.0, 0.0];
}
