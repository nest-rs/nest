#version 150

in mat3x2 positions;
in mat3x2 texcoords;

out mat3x2 v_positions;
out mat3x2 v_texcoords;

void main() {
    v_positions = positions;
    v_texcoords = texcoords;
}