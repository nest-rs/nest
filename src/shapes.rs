
use glium;
use support::shaders::ShaderMode;
use support::vertex::*;
use image::Image;
use std::marker::PhantomData;

/// Trait for structs to be drawn with `draw_shape`
pub trait Shape<V>: Sized
where
	V: glium::Vertex,
{
	/// Get the points of the shape in triangle fan format
	fn points(&self) -> Vec<V>;
	/// Get the shader mode (texture | color)
	fn shader_mode(&self) -> ShaderMode {
		ShaderMode::Color
	}
}

/// Adds transforms to a Shape
pub struct Transform<V: glium::Vertex, S: Shape<V>> {
	phantom: PhantomData<V>,
	shape: S,
	translate: [f64; 2],
	rotate: f64,
}

impl<V, S: Shape<V>> Transform<V, S>
where
	V: glium::Vertex,
{
	/// Create a new Transform for a shape
	pub fn new(shape: S) -> Self {
		Transform {
			phantom: PhantomData,
			shape: shape,
			translate: [0.0; 2],
			rotate: 0.0,
		}
	}

	/// Set the position transform in X and Y
	pub fn with_position(self, x: f64, y: f64) -> Self {
		Transform {
			translate: [x, y],
			..self
		}
	}

	/// Set the rotation transform in radians
	pub fn with_rotation(self, angle: f64) -> Self {
		Transform {
			rotate: angle,
			..self
		}
	}

	/// Add x, y to the translation
	pub fn translate(&mut self, x: f64, y: f64) {
		self.translate[0] += x;
		self.translate[1] += y;
	}

	/// Add angle to the rotation
	pub fn rotate(&mut self, angle: f64) {
		self.rotate += angle;
	}
}


impl<S: Shape<color::Vertex>> Shape<color::Vertex> for Transform<color::Vertex, S> {
	fn points(&self) -> Vec<color::Vertex> {
		self.shape
			.points()
			.into_iter()
			.map(|v| {
				let x = v.position[0];
				let y = v.position[1];
				let dx = self.translate[0];
				let dy = self.translate[1];
				let s = self.rotate.sin();
				let c = self.rotate.cos();

				color::Vertex {
					position: [x * c - y * s + dx, x * s + y * c + dy],
					..v
				}
			})
			.collect()
	}

	fn shader_mode(&self) -> ShaderMode {
		self.shape.shader_mode()
	}
}

/// A rectangle that does not implement shape.
pub struct Rectangle {
	/// The x component
	pub x: f64,
	/// The y component
	pub y: f64,
	/// The width component
	pub w: f64,
	/// The height component
	pub h: f64,
}

impl Default for Rectangle {
	fn default() -> Self {
		Rectangle {
			x: 0.0,
			y: 0.0,
			w: 1.0,
			h: 1.0,
		}
	}
}
/// Represents a textured rectangle
pub struct ImageRectangle<'a> {
	/// X component
	pub x: f64,
	/// Y component
	pub y: f64,
	/// Width component
	pub w: f64,
	/// Height component
	pub h: f64,

	/// Crop x
	pub dx: f64,
	/// Crop y
	pub dy: f64,
	/// Crop width
	pub dw: f64,
	/// Crop height
	pub dh: f64,

	/// Texture image
	pub texture: &'a Image,
}

impl<'a> Shape<texture::Vertex> for ImageRectangle<'a> {
	fn points(&self) -> Vec<texture::Vertex> {
		vec![
			texture::Vertex::new(self.x, self.y, self.dx, self.dy),
			texture::Vertex::new(self.x + self.w, self.y, self.dx + self.dw, self.dy),
			texture::Vertex::new(
				self.x + self.w,
				self.y + self.h,
				self.dx + self.dw,
				self.dy + self.dh,
			),
			texture::Vertex::new(self.x, self.y + self.h, self.dx, self.dy + self.dh),
		]
	}

	fn shader_mode(&self) -> ShaderMode {
		ShaderMode::Texture(self.texture)
	}
}

impl<'a> From<(Rectangle, Option<Rectangle>, &'a Image)> for ImageRectangle<'a> {
	fn from(parts: (Rectangle, Option<Rectangle>, &'a Image)) -> ImageRectangle<'a> {
		let pos = parts.0;
		let crop = parts.1.unwrap_or(Default::default());
		let image = parts.2;

		ImageRectangle {
			x: pos.x,
			y: pos.y,
			w: pos.w,
			h: pos.h,

			dx: crop.x,
			dy: crop.y,
			dw: crop.w,
			dh: crop.h,

			texture: image,
		}
	}
}

/// Rectangle shape depicted by location `x, y` and size `w, h`
pub struct ColorRectangle {
	/// The x coordinate of the rectange
	pub x: f64,
	/// The y coordinate of the rectange
	pub y: f64,
	/// The rectangles width
	pub w: f64,
	/// The rectangles height
	pub h: f64,
	/// The color fo the rectangle
	pub color: [f32; 4],
}

impl Shape<color::Vertex> for ColorRectangle {
	fn points(&self) -> Vec<color::Vertex> {
		vec![
			color::Vertex::new(self.x, self.y, self.color),
			color::Vertex::new(self.x + self.w, self.y, self.color),
			color::Vertex::new(self.x + self.w, self.y + self.h, self.color),
			color::Vertex::new(self.x, self.y + self.h, self.color),
		]
	}
}

impl From<(Rectangle, [f32; 4])> for ColorRectangle {
	fn from(parts: (Rectangle, [f32; 4])) -> ColorRectangle {
		let pos = parts.0;
		let color = parts.1;

		ColorRectangle {
			x: pos.x,
			y: pos.y,
			w: pos.w,
			h: pos.h,
			color: color,
		}
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
	/// The color of the oval
	pub color: [f32; 4],
}

impl Shape<color::Vertex> for Circle {
	fn points(&self) -> Vec<color::Vertex> {
		(0u32..360)
			.filter(|d| d % self.step_size == 0)
			.map(|d| {
				let r = (d as f64).to_radians();
				color::Vertex::new(
					self.x + r.cos() * self.rx,
					self.y + r.sin() * self.ry,
					self.color,
				)
			})
			.collect()
	}
}
