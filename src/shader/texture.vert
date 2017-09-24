#version 150

in mat3x2 positions;
in mat3x2 texcoords;
in vec4 color;

out mat3x2 v_positions;
out mat3x2 v_texcoords;
out vec4 v_color;

void main() {
    v_positions = positions;
    v_texcoords = texcoords;
    v_color = color;
}