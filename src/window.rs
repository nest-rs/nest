use glium;
use frame::Frame;
use image::{Image, LoadImageError};
use glium::glutin;
use std::path;
use support;
use support::events::{self, Event};
use img;

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
    display: glium::Display,
    events_loop: glium::glutin::EventsLoop,
    programs: (glium::Program, glium::Program),
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
            programs: (color_program, texture_program),
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
    pub fn load_image<'a, P: AsRef<path::Path>>(&self, path: P) -> Result<Image, LoadImageError> {
        use std::io::prelude::*;
        use std::fs::File;

        let mut buf = Vec::new();
        match File::open(path) {
            Ok(mut file) => {
                match file.read_to_end(&mut buf) {
                    Ok(_) => (),
                    Err(err) => return Err(LoadImageError::FileError(err)),
                }
            }
            Err(err) => return Err(LoadImageError::FileError(err)),
        }

        match img::load_from_memory(&buf[..]) {
            Ok(image) => {
                let image = image.to_rgba();
                let image_dimensions = image.dimensions();
                let glimage =
                    glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
                let texture = glium::texture::Texture2d::new(&self.display, glimage).unwrap();

                return Ok(Image::new(image_dimensions, texture));
            }
            Err(err) => return Err(LoadImageError::ImageError(err)),
        }
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
    pub fn next_frame(&mut self) -> Frame {
        Frame::new(&self.display, &self.programs)
    }
}
