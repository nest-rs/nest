
pub mod color {
    use glium;

    pub fn load_program<'a, D>(
        display: &'a D,
    ) -> Result<glium::Program, glium::ProgramCreationError>
    where
        D: glium::backend::Facade,
    {
        glium::Program::from_source(display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None)
    }

    pub const VERTEX_SHADER_SRC: &'static str = r#"
        #version 140

        in vec2 position;
        in vec4 color;

        out vec4 o_color;

        void main() {
            o_color = color;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    pub const FRAGMENT_SHADER_SRC: &'static str = r#"
        #version 140

        in vec4 o_color;
        out vec4 color;

        void main() {
            color = o_color;
        }
    "#;
}

pub mod texture {
    use glium;

    pub fn load_program<'a, D>(
        display: &'a D,
    ) -> Result<glium::Program, glium::ProgramCreationError>
    where
        D: glium::backend::Facade,
    {
        glium::Program::from_source(display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None)
    }

    pub const VERTEX_SHADER_SRC: &'static str = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    pub const FRAGMENT_SHADER_SRC: &'static str = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;
}
