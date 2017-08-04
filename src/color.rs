#[derive(Copy, Clone, Debug)]
pub struct Color(f32, f32, f32, f32);

impl Color {
    /// Scale the brightness of the color
    pub fn scale(self, factor: f32) -> Color {
        Color(self.0 * factor, self.1 * factor, self.2 * factor, self.3)
    }

    /// Scale the brightness of the color
    pub fn alpha(self, factor: f32) -> Color {
        Color(self.0, self.1, self.2, self.3 * factor)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(tup: (u8, u8, u8, u8)) -> Color {
        Color(
            tup.0 as f32 / 255.0,
            tup.1 as f32 / 255.0,
            tup.2 as f32 / 255.0,
            tup.3 as f32 / 255.0,
        )
    }
}

impl From<[u8; 4]> for Color {
    fn from(arr: [u8; 4]) -> Color {
        Color(
            arr[0] as f32 / 255.0,
            arr[1] as f32 / 255.0,
            arr[2] as f32 / 255.0,
            arr[3] as f32 / 255.0,
        )
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(tup: (f32, f32, f32, f32)) -> Color {
        Color(tup.0, tup.1, tup.2, tup.3)
    }
}

impl From<[f32; 4]> for Color {
    fn from(arr: [f32; 4]) -> Color {
        Color(arr[0], arr[1], arr[2], arr[3])
    }
}

pub const WHITE: Color = Color(0.0, 0.0, 1.0, 1.0);
pub const RED: Color = Color(1.0, 0.0, 0.0, 1.0);
pub const GREEN: Color = Color(0.0, 1.0, 0.0, 1.0);
pub const BLUE: Color = Color(0.0, 0.0, 1.0, 1.0);
pub const YELLOW: Color = Color(1.0, 1.0, 0.0, 1.0);
pub const CYAN: Color = Color(0.0, 1.0, 1.0, 1.0);
pub const MAGENTA: Color = Color(1.0, 0.0, 1.0, 1.0);
