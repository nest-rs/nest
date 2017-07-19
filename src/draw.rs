
use super::*;

pub type Color = [f32; 4];

pub enum Shape {
	Clear(Color),
	Polygon(Vec<(f32, f32)>, Color),
	Line(f32, f32, f32, f32, Color),
}

impl Window {
	pub fn clear_to_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
		let shape = Shape::Clear([red, green, blue, alpha]);
		self.shapes.push(shape);
	}

	pub fn clear(&mut self) {
		self.clear_to_color(0.0, 0.0, 0.0, 1.0);
	}

	pub fn set_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
		self.color = [red, green, blue, alpha];
	}

	pub fn set_color_html(&mut self, color: &str) {
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
			self.set_color(
				(r as f32) / 255.0,
				(g as f32) / 255.0,
				(b as f32) / 255.0,
				1.0,
			);
		} else if bytes.len() == 6 {
			let r = u8::from_str_radix(from_utf8(&bytes[0..2]).unwrap(), 16).unwrap();
			let g = u8::from_str_radix(from_utf8(&bytes[2..4]).unwrap(), 16).unwrap();
			let b = u8::from_str_radix(from_utf8(&bytes[4..6]).unwrap(), 16).unwrap();
			self.set_color(
				(r as f32) / 255.0,
				(g as f32) / 255.0,
				(b as f32) / 255.0,
				1.0,
			);
		}
	}

	pub fn draw(&mut self, points: &[(f32, f32)]) {
		let shape = Shape::Polygon(Vec::from(points), self.color);
		self.shapes.push(shape);
	}

	pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
		let shape = Shape::Line(x1, y1, x2, y2, self.color);
		self.shapes.push(shape);
	}

	pub fn draw_rect(&mut self, p1: (f32, f32), p2: (f32, f32)) {
		let shape = Shape::Polygon(
			vec![(p1.0, p1.1), (p1.0, p2.1), (p2.0, p2.1), (p2.0, p1.1)],
			self.color,
		);
		self.shapes.push(shape);
	}

	pub fn draw_circle(&mut self, x: f32, y: f32, rx: f32, ry: f32, step_size: u32) {
		let circle: Vec<(f32, f32)> = (0u32..360)
			.filter(|d| d % step_size == 0)
			.map(|d| {
				let r = (d as f32).to_radians();
				(x + r.cos() * rx, y + r.sin() * ry)
			})
			.collect();
		let shape = Shape::Polygon(circle, self.color);
		self.shapes.push(shape);
	}
}
