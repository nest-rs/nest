#version 140

in vec2 positions;
in vec2 texcoords;
in vec4 color;

out vec4 f_color;

void main() {
    f_color = color;
    gl_Position = vec4(positions, 0.0, 1.0);
}