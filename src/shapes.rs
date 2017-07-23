
/// Trait for structs to be drawn with `draw_shape`
pub trait Shape {
	/// Get the points of the shape in triangle fan format
	fn to_points(&self) -> Vec<(f64, f64)>;
}

/// Rectangle shape depicted by location `x, y` and size `w, h`
pub struct Rectangle {
	/// The x coordinate of the rectange
	pub x: f64,
	/// The y coordinate of the rectange
	pub y: f64,
	/// The rectangles width
	pub w: f64,
	/// The rectangles height
	pub h: f64,
}

impl Shape for Rectangle {
	fn to_points(&self) -> Vec<(f64, f64)> {
		vec![
			(self.x, self.y),
			(self.x + self.w, self.y),
			(self.x + self.w, self.y + self.h),
			(self.x, self.y + self.h),
		]
	}
}

/// Circle / Oval shape with width, height, and center point
pub struct Circle {
	/// The x location of the circles center point
	pub x: f64,
	/// The y location of the circles center point
	pub y: f64,
	/// The width of the circle
	pub rx: f64,
	/// The height of the circle
	pub ry: f64,
	/// The number of degrees for each step in the circle
	pub step_size: u32,
}


impl Shape for Circle {
	fn to_points(&self) -> Vec<(f64, f64)> {
		(0u32..360)
			.filter(|d| d % self.step_size == 0)
			.map(|d| {
				let r = (d as f64).to_radians();
				(self.x + r.cos() * self.rx, self.y + r.sin() * self.ry)
			})
			.collect()
	}
}
