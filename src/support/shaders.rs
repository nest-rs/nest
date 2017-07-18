
extern crate glium;

pub fn load_program<'a, D>(display: &'a D) -> Result<glium::Program, glium::ProgramCreationError>
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
        gl_Position = vec4(position, 0.0, 1.0);
        o_color = color;
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
