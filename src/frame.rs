use glium;
use glium::Surface;
use shape::Shape;


/// Represents a single frame in the render loop.
///
/// This structure contains method for rendering along with frame information
/// includeing delta time from the last frame. All colors and imaged drawn to the
/// frame enherit `Alpha Blend` transparencies.
///
/// # Example
/// See the example `examples/demo.rs` for a complete example.
pub struct Frame<'a, 'b> {
    display: &'a glium::Display,
    target: Option<glium::Frame>,
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
            target: Some(display.draw()),
            programs: programs,
        }
    }

    /// Clear the frame to black `(0.0, 0.0, 0.0, 1.0)`
    pub fn clear(&mut self) {
        self.target.as_mut().unwrap().clear_color(
            0.0,
            0.0,
            0.0,
            1.0,
        );
    }

    /// Clear the frame to the specified color
    pub fn clear_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.target.as_mut().unwrap().clear_color(
            red,
            green,
            blue,
            alpha,
        );
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
    /// # use nest::{Window, ShaderMode};
    /// # let mut app = Window::new("draw_rect Example", 300, 200);
    /// # let mut frame = app.next_frame();
    /// frame.draw(&[(0.0, 0.0), (0.2, 0.3), (0.3, 0.2)], ShaderMode::Color);
    /// # frame.finish();
    /// # }
    /// ```
    pub fn draw<S: Shape>(&mut self, shape: S) {
        let vert_buff =
            glium::VertexBuffer::new(self.display, &shape.tris().collect::<Vec<_>>()[..])
                .expect("error: failed to form vertex buffer");
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);
        match shape.texture() {
            Some(tex) => {
                self.target
                    .as_mut()
                    .unwrap()
                    .draw(
                        &vert_buff,
                        &indices,
                        &self.programs.1,
                        &uniform! {
                            tex: &*tex,
                            color: shape.color(),
                        },
                        &glium::DrawParameters {
                            blend: glium::draw_parameters::Blend::alpha_blending(),
                            ..Default::default()
                        },
                    )
                    .expect("error: failed to draw");
            }
            None => {
                self.target
                    .as_mut()
                    .unwrap()
                    .draw(
                        &vert_buff,
                        &indices,
                        &self.programs.1,
                        &uniform! {
                            color: shape.color(),
                        },
                        &glium::DrawParameters {
                            blend: glium::draw_parameters::Blend::alpha_blending(),
                            ..Default::default()
                        },
                    )
                    .expect("error: failed to draw");
            }
        }
    }

    /// Finish drawing the frame and push it to the screen.
    pub fn finish(mut self) {
        self.target.take().unwrap().finish().expect(
            "error: failed to finish frame render",
        );
    }
}

impl<'a, 'b> Drop for Frame<'a, 'b> {
    fn drop(&mut self) {
        self.target.take().unwrap().finish().expect(
            "error: failed to finish frame render",
        );
    }
}
