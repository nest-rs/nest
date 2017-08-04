use glium;
use frame::Frame;
use glium::glutin;
use std::path;
use support;
use support::events::{self, Event};
use img;
use glium::texture::Texture2d;
use std::io::prelude::*;
use std::fs::File;
use std::rc::Rc;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Image(img::ImageError);
        Texture(glium::texture::TextureCreationError);
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
    pub fn new<S: Into<String>>(title: S, width: u32, height: u32) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);
        let context = glutin::ContextBuilder::new();
        let display =
            glium::Display::new(window, context, &events_loop).expect("Could not create Display");
        let color_program =
            support::shaders::color::load_program(&display).expect("Could not create color shader");
        let texture_program = support::shaders::texture::load_program(&display).expect(
            "Could not create texture shader",
        );

        Window {
            display: display,
            events_loop: events_loop,
            texture_program: program!(&display,
                150 => {
                    vertex: include_str!("shader/texture.vert"),
                    geometry: include_str!("shader/texture.geom"),
                    fragment: include_str!("shader/texture.frag"),
                },
            ),
            plain_program: program!(&display,
                150 => {
                    vertex: include_str!("shader/plain.vert"),
                    geometry: include_str!("shader/plain.geom"),
                    fragment: include_str!("shader/plain.frag"),
                },
            ),
        }
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
    pub fn poll_events(&mut self) -> Vec<Event> {
        let mut events: Vec<Event> = Vec::new();

        self.events_loop.poll_events(|ev| {
            let event = events::translate(ev);
            match event {
                Event::Unsupported => (),
                event => events.push(event),
            }
        });

        events
    }

    /// Get the next frame for rendering.
    ///
    /// This method sets up the window and renderer for the next frame. The
    /// `Frame` object returned includes the methods for rendering to the frame
    /// along with delta time and window size.
    ///
    /// # Example
    /// ```rust,no_run
    /// # extern crate nest;
    /// # use nest::Window;
    /// # fn main() {
    /// let mut app = Window::new("Hello World", 640, 480);
    ///
    /// loop {
    ///     let mut frame = app.next_frame();
    ///
    ///     // Render code goes here
    ///
    ///     frame.finish();
    /// }
    /// # }
    /// ```
    pub fn next_frame(&self) -> Frame {
        Frame::new(self)
    }
}
