#version 140

in vec2 position;
in vec4 color;

out vec4 o_color;

void main() {
    o_color = color;
    gl_Position = vec4(position, 0.0, 1.0);
}