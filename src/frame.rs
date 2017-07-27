
use glium;
use glium::Surface;
use image::Image;
use shapes::{Shape, Rectangle, ColorRectangle, ImageRectangle};
use support::shaders::ShaderMode;

/**
Represents a single frame in the render loop. 

This structure contains method for rendering along with frame information
includeing delta time from the last frame. All colors and imaged drawn to the
frame enherit `Alpha Blend` transparencies.

# Example
See the example `examples/demo.rs` for a complete example.
*/
pub struct Frame<'a, 'b> {
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
	) -> Self {
		Frame {
			display: display,
			target: display.draw(),
			programs: programs,
		}
	}

	/// Clear the frame to black `(0.0, 0.0, 0.0, 1.0)`
	pub fn clear(&mut self) {
		self.target.clear_color(0.0, 0.0, 0.0, 1.0);
	}

	/// Clear the frame to the specified color
	pub fn clear_to_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
		self.target.clear_color(red, green, blue, alpha);
	}

	/// Draw a set of points as a `Triangle Fan`. 
	///
	/// Each point is tupil pair of `(x, y)` cooridnates for said point. Each
	/// point enherits the color set by the previous call to `set_color` or
	/// `set_color_html`.
	///
	/// # Example
	/// ```rust,no_run
	/// # extern crate nest;
	/// # fn main() {
	/// # use nest::Window;
	/// # let mut app = Window::new("draw_rect Example", 300, 200);
	/// # let mut frame = app.next_frame();
	/// frame.draw(&[(0.0, 0.0), (0.2, 0.3), (0.3, 0.2)]);
	/// # frame.finish();
	/// # }
	/// ```
	pub fn draw<V: glium::Vertex>(&mut self, points: &[V], mode: ShaderMode) {
		let vert_buff = glium::VertexBuffer::new(self.display, &points).unwrap();
		let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);
		match mode {
			ShaderMode::Color => self.target
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
			.unwrap(),
			ShaderMode::Texture(image) => self.target
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
			.unwrap(),
		}
	}

	/// Draw a struct that implements the `Shape` trait.
	///
	/// The object will be draw as a triangle fan with the current foreground color.
	///
	/// # Example
	/// ```rust,no_run
	/// # extern crate nest;
	/// # fn main() {
	/// # use nest::{Window, Circle};
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
	pub fn draw_shape<V: glium::Vertex, S: Shape<V>>(&mut self, shape: &S) {
		self.draw(&shape.points(), shape.shader_mode());
	}

	/// Draw a rectangle from `x, y, width, height` parameters.
	///
	/// # Example
	/// ```rust,no_run
	/// # extern crate nest;
	/// # fn main() {
	/// # use nest::Window;
	/// # let mut app = Window::new("draw_rect Example", 300, 200);
	/// # let mut frame = app.next_frame();
	/// frame.draw_rect(-0.2, -0.3, 1.0, 1.0);
	/// # frame.finish();
	/// # }
	/// ```
	pub fn draw_rect(&mut self, x: f64, y: f64, w: f64, h: f64, color: [f32; 4]) {
		self.draw_shape(&ColorRectangle::new(Rectangle {
			x: x,
			y: y,
			w: w,
			h: h,
		}, color));
	}

	/// Draw an image with location `x, y, width, height` and croppinc specified by `parameters`.
	///
	/// # Example
	/// ```rust,no_run
	/// # extern crate nest;
	/// # fn main() {
	/// # use nest::{Window, Rectangle};
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
		crop: Option<Rectangle>
	) {
		self.draw_shape(&ImageRectangle::from((rect, crop, image)));
	}

	/// Finish drawing the frame and push it to the screen.
	///
	/// **Note**: this method must be called before the next call to
	/// `Window::next_frame()`.
	///
	/// **Note**: no draw draw methods may be called on the current frame after
	/// this method is called.
	pub fn finish(self) {
		self.target.finish().unwrap();
	}
}
