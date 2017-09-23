use glium;
use glium::Surface;
use glium::glutin;
use std::path;
use Event;
use event;
use img;
use glium::texture::Texture2d;
use std::io::prelude::*;
use std::fs::File;
use std::rc::Rc;
use Color;
use Shape;
use std::vec;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Image(img::ImageError);
        Texture(glium::texture::TextureCreationError);
        Program(glium::program::ProgramChooserCreationError);
        DisplayCreation(glium::backend::glutin::DisplayCreationError);
    }
}

/// Represets a window with OpenGL context.
///
/// This window provides image loading, rendering via `Window::next_frame(...)`,
/// and events via `Window::poll_events(...)`.
///
/// # Example
/// ```rust,no_run
/// extern crate nest;
/// use nest::{Window, Event};
///
/// fn main() {
///     let mut app = Window::new("Hello World", 640, 480);
///
///     loop {
///         // Note rust requires this code to be in a closure to please the borrow checker
///         {
///             let mut frame = app.next_frame();
///
///             // Render Code Goes Here
///
///             frame.finish();
///         }
///
///         for ev in app.poll_events() {
///             match ev {
///                 Event::Closed => break,
///                 _ => (),
///             }
///         }
///     }
/// }
/// ```
pub struct Window {
    pub(crate) display: glium::Display,
    events_loop: glium::glutin::EventsLoop,
    pub(crate) texture_program: glium::Program,
    pub(crate) plain_program: glium::Program,
    pub clear_color: Color,
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
    /// # extern crate nest;
    /// # fn main() {
    /// # use nest::Window;
    /// let mut app = Window::new("Hello World", 640, 480);
    /// let pic = app.load_image("res/city.jpg");
    /// # }
    /// ```
    pub fn load_image<'a, P: AsRef<path::Path>>(&self, path: P) -> Result<Rc<Texture2d>> {
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

    /// Poll the window for events.
    ///
    /// # Example
    /// ```rust,no_run
    /// # extern crate nest;
    /// # use nest::Window;
    /// # fn main() {
    /// use nest::Event;
    ///
    /// let mut app = Window::new("Hello World", 640, 480);
    ///
    /// for ev in app.poll_events() {
    ///     match ev {
    ///         _ => ()
    ///     }
    /// }
    /// # }
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

    pub fn draw<S>(&mut self, shape: S) where S: Shape {
        let mut frame = self.frame();
        frame.draw(shape);
    }
}

pub struct Frame<'a> {
    target: Option<glium::Frame>,
    window: &'a Window,
}

impl<'a> Frame<'a> {
    pub fn finish(mut self) {
        self.target.take().unwrap().finish().expect("error: failed to finish drawing");
    }

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
