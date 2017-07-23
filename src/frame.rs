
use glium;
use glium::Surface;
use support;
use image::Image;
use shapes::{Shape, Rectangle};

/**
Represents a single frame in the render loop. 

This structure contains method for rendering along with frame information
includeing delta time from the last frame. All colors and imaged drawn to the
frame enherit `Alpha Blend` transparencies.

# Example
See the example `examples/demo.rs` for a complete example.
*/
pub struct Frame<'a, 'b> {
	color: [f32; 4],
	finished: bool,
	delta: f64,
	display: &'a glium::Display,
	target: glium::Frame,
	programs: &'b (glium::Program, glium::Program),
}

impl<'a, 'b> Frame<'a, 'b> {
	/// Create a new Frame for a glium `Display` with shader programs for color
	/// and texture.
	pub fn new(
		display: &'a glium::Display,
		programs: &'b (glium::Program, glium::Program),
		delta: f64,
	) -> Self {
		Frame {
			color: [0.0; 4],
			finished: false,
			delta: delta,
			display: display,
			target: display.draw(),
			programs: programs,
		}
	}

	/// Get the delta time since the last frame in decimal seconds.
	pub fn delta(&self) -> f64 {
		self.delta
	}

	/// Clear the frame to black `(0.0, 0.0, 0.0, 1.0)`
	pub fn clear(&mut self) {
		self.target.clear_color(0.0, 0.0, 0.0, 1.0);
	}

	/// Clear the frame to the specified color
	pub fn clear_to_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
		self.target.clear_color(red, green, blue, alpha);
	}

	/// Set the foreground color for future draw calls. This does not effect
	/// `draw_image`.
	pub fn set_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
		self.color = [red, green, blue, alpha];
	}

	/// Set the color using a 3 or 6 character HTML color string
	///
	/// # Examples
	/// `"#333"`
	/// `"F25"`
	/// `"3E59F4"`
	/// `"#45F0D5"`
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

	/// Draw a set of points as a `Triangle Fan`. 
	///
	/// Each point is tupil pair of `(x, y)` cooridnates for said point. Each
	/// point enherits the color set by the previous call to `set_color` or
	/// `set_color_html`.
	///
	/// # Example
	/// ```rust,no_run
	/// # extern crate simple;
	/// # fn main() {
	/// # use simple::Window;
	/// # let mut app = Window::new("draw_rect Example", 300, 200);
	/// # let mut frame = app.next_frame();
	/// frame.draw(&[(0.0, 0.0), (0.2, 0.3), (0.3, 0.2)]);
	/// # frame.finish();
	/// # }
	/// ```
	pub fn draw(&mut self, points: &[(f64, f64)]) {
		let vert_buff =
			support::buffer::poly_vert_buffer(self.display, &points, self.color).unwrap();
		let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);
		self.target
			.draw(
				&vert_buff,
				&indices,
				&self.programs.0,
				&glium::uniforms::EmptyUniforms,
				&glium::DrawParameters {
					blend: glium::draw_parameters::Blend::alpha_blending(),
					..Default::default()
				},
			)
			.unwrap();
	}

	/// Draw a struct that implements the `Shape` trait.
	///
	/// The object will be draw as a triangle fan with the current foreground color.
	///
	/// # Example
	/// ```rust,no_run
	/// # extern crate simple;
	/// # fn main() {
	/// # use simple::{Window, Circle};
	/// # let mut app = Window::new("draw_rect Example", 300, 200);
	/// # let mut frame = app.next_frame();
	/// frame.draw_shape(Circle {
	/// 	x: 0.25,
	/// 	y: -0.25,
	/// 	rx: 0.75,
	/// 	ry: 0.25,
	/// 	step_size: 10,
	/// });
	/// # frame.finish();
	/// # }
	/// ```
	pub fn draw_shape<S: Shape>(&mut self, shape: S) {
		self.draw(&shape.to_points());
	}

	/// Draw a rectangle from `x, y, width, height` parameters.
	///
	/// # Example
	/// ```rust,no_run
	/// # extern crate simple;
	/// # fn main() {
	/// # use simple::Window;
	/// # let mut app = Window::new("draw_rect Example", 300, 200);
	/// # let mut frame = app.next_frame();
	/// frame.draw_rect(-0.2, -0.3, 1.0, 1.0);
	/// # frame.finish();
	/// # }
	/// ```
	pub fn draw_rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
		self.draw(&vec![(x, y), (x, y + h), (x + w, y + h), (x + w, y)]);
	}

	/// Draw an image with location `x, y, width, height` and croppinc specified by `parameters`.
	///
	/// # Example
	/// ```rust,no_run
	/// # extern crate simple;
	/// # fn main() {
	/// # use simple::{Window, Rectangle};
	/// # let mut app = Window::new("draw_circle Example", 300, 200);
	/// let pic = app.load_image("image.jpg").unwrap();
	///
	/// # let mut frame = app.next_frame();
	/// frame.draw_image(
	/// 	&pic,
	/// 	Rectangle {
	/// 		x: -0.5,
	/// 		y: 0.0,
	/// 		w: 0.5,
	/// 		h: 0.5,
	/// 	},
	/// 	Default::default(),
	/// );
	/// # frame.finish();
	/// # }
	/// ```
	pub fn draw_image(
		&mut self,
		image: &Image,
		rect: Rectangle,
		parameters: super::ImageParameters,
	) {
		let vert_buff = support::buffer::image_vert_buffer(
			self.display,
			rect.x,
			rect.y,
			rect.x + rect.w,
			rect.y + rect.h,
			parameters,
		).unwrap();
		let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
		self.target
			.draw(
				&vert_buff,
				&indices,
				&self.programs.1,
				&uniform! {
					tex: image.get_texture(),
				},
				&glium::DrawParameters {
					blend: glium::draw_parameters::Blend::alpha_blending(),
					..Default::default()
				},
			)
			.unwrap();
	}

	/// Finish drawing the frame and push it to the screen.
	///
	/// **Note**: this method must be called before the next call to
	/// `Window::next_frame()`.
	///
	/// **Note**: no draw draw methods may be called on the current frame after
	/// this method is called.
	pub fn finish(mut self) {
		let ok = self.target.finish();
		self.finished = ok.is_ok();
	}
}
