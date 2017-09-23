use glium;
use glium::Surface;
use glium::glutin;
use glium::texture::Texture2d;
use std::io::prelude::*;
use std::fs::File;
use std::rc::Rc;
use std::vec;
use std::path;
use *;

#[allow(missing_docs, unused_doc_comment)]
pub mod error {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Image(super::img::ImageError);
            Texture(super::glium::texture::TextureCreationError);
            Program(super::glium::program::ProgramChooserCreationError);
            DisplayCreation(super::glium::backend::glutin::DisplayCreationError);
        }
    }
}

pub use error::*;

/// Represets a window with OpenGL context.
///
/// # Example
/// ```rust,no_run
/// use nest::*;
/// use std::iter::empty;
/// 
/// fn main() {
///     let mut app = Window::new("Window Example", 640, 480).unwrap();
/// 
///     while !app.poll_events().any(|e| e == Event::Closed) {
///         app.draw(empty());
///     }
/// }
/// ```
pub struct Window {
    pub(crate) display: glium::Display,
    events_loop: glium::glutin::EventsLoop,
    pub(crate) texture_program: glium::Program,
    pub(crate) plain_program: glium::Program,
    pub(crate) clear_color: Color,
}

impl Window {
    /// Create a new Window with a Title, and size (width / height) specified in
    /// pixels.
    ///
    /// # Pacnic
    /// This method will panic if there are any issues with creating the window
    /// or compiling the shaders. See the glium library for more specifics on
    /// window creation.
    ///
    /// # Example
    /// ```rust,no_run
    /// extern crate nest;
    /// # fn main() {
    /// use nest::Window;
    ///
    /// let mut app = Window::new("Hello World", 640, 480);
    /// # }
    /// ```
    pub fn new<S: Into<String>>(title: S, width: u32, height: u32) -> Result<Self> {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);
        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop)?;

        let texture_program = program!(&display,
            150 => {
                vertex: include_str!("shader/texture.vert"),
                geometry: include_str!("shader/texture.geom"),
                fragment: include_str!("shader/texture.frag"),
            },
        )?;

        let plain_program = program!(&display,
            150 => {
                vertex: include_str!("shader/plain.vert"),
                geometry: include_str!("shader/plain.geom"),
                fragment: include_str!("shader/plain.frag"),
            },
        )?;

        Ok(Window {
            display: display,
            events_loop: events_loop,
            texture_program: texture_program,
            plain_program: plain_program,
            clear_color: Color::BLACK,
        })
    }

    /// Load an image from a file to be drawn by `Frame::draw_image(...)`.
    ///
    /// # Parameters
    ///
    /// * `path` - a path pointing to an image file
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nest::Window;
    /// let mut app = Window::new("Window Example", 640, 480).unwrap();
    /// let pic = app.load_image("res/city.jpg").unwrap();
    /// ```
    pub fn load_image<P: AsRef<path::Path>>(&self, path: P) -> Result<Rc<Texture2d>> {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;
        let image = img::load_from_memory(&buf[..])?.to_rgba();
        let dims = image.dimensions();
        Ok(Rc::new(Texture2d::new(
            &self.display,
            glium::texture::RawImage2d::from_raw_rgba(
                image.into_raw(),
                dims,
            ),
        )?))
    }

    /// Change the color which the screen is cleared with between frames.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nest::*;
    /// let mut app = Window::new("Window Example", 640, 480).unwrap();
    /// app.clear_color(Color::RED);
    /// ```
    pub fn clear_color<C: Into<Color>>(&mut self, color: C) {
        self.clear_color = color.into();
    }

    /// Poll the window for events.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nest::*;
    /// let mut app = Window::new("Window Example", 640, 480).unwrap();
    ///
    /// for event in app.poll_events() {
    ///     match event {
    ///         // Close if they close the window or hit escape.
    ///         Event::Closed | Event::KeyboardInput(KeyState::Pressed, Some(Key::Escape)) => return,
    ///         // Print "Space!" if they hit space.
    ///         Event::KeyboardInput(KeyState::Pressed, Some(Key::Space)) => println!("Space!"),
    ///         _ => {}
    ///     }
    /// }
    /// ```
    pub fn poll_events(&mut self) -> vec::IntoIter<Event> {
        let mut events: Vec<Event> = Vec::new();

        self.events_loop.poll_events(|ev| {
            let event = event::translate(ev);
            match event {
                Event::Unsupported => (),
                event => events.push(event),
            }
        });

        events.into_iter()
    }

    /// Clears the screen and gets a `Frame` which can be used to make multiple draw calls to in one frame.
    /// When dropped, `Frame` automatically draws the frame.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nest::*;
    /// let mut app = Window::new("Window Example", 640, 480).unwrap();
    ///
    /// while !app.poll_events().any(|e| e == Event::Closed) {
    ///     let mut frame = app.frame();
    ///     frame.draw(rect([0.0, 0.0], [1.0, 1.0]));
    ///     frame.finish();
    /// }
    /// ```
    pub fn frame<'a>(&'a self) -> Frame<'a> {
        let mut f = self.display.draw();
        f.clear_color(
            self.clear_color.0[0],
            self.clear_color.0[1],
            self.clear_color.0[2],
            self.clear_color.0[3],
        );
        Frame{
            target: Some(f),
            window: self,
        }
    }

    /// Clears the frame, draws the `shape`, and updates the window.
    pub fn draw<S>(&mut self, shape: S) where S: Shape {
        let mut frame = self.frame();
        frame.draw(shape);
    }
}

/// `Frame` can be used to make multiple separate draw calls in one frame. When it drops/exits scope,
/// it automatically updates the window with the frame, but it can also be finished by calling the method `finish()`.
pub struct Frame<'a> {
    target: Option<glium::Frame>,
    window: &'a Window,
}

impl<'a> Frame<'a> {
    /// Forces the frame to be consumed and update the window immediately.
    pub fn finish(self) {}

    /// Draws the `shape`.
    pub fn draw<S>(&mut self, shape: S) where S: Shape {
        for rtri in shape {
            let texture = rtri.texture;
            let vert_buff = glium::VertexBuffer::new(
                &self.window.display,
                &[rtri.tri],
            ).expect("error: failed to form vertex buffer");
            let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);
            match texture {
                Some(tex) => {
                    self.target.as_mut().unwrap()
                        .draw(
                            &vert_buff,
                            &indices,
                            &self.window.texture_program,
                            &uniform! {
                                tex: &*tex,
                            },
                            &glium::DrawParameters {
                                blend: glium::draw_parameters::Blend::alpha_blending(),
                                ..Default::default()
                            },
                        )
                        .expect("error: failed to draw");
                }
                None => {
                    self.target.as_mut().unwrap()
                        .draw(
                            &vert_buff,
                            &indices,
                            &self.window.plain_program,
                            &glium::uniforms::EmptyUniforms,
                            &glium::DrawParameters {
                                blend: glium::draw_parameters::Blend::alpha_blending(),
                                ..Default::default()
                            },
                        )
                        .expect("error: failed to draw");
                }
            }
        }
    }
}

impl<'a> Drop for Frame<'a> {
    fn drop(&mut self) {
        if let Some(t) = self.target.take() {
            t.finish().expect("error: failed to finish drawing");
        }
    }
}
