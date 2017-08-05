#version 150

layout(points) in;
layout(triangle_strip, max_vertices = 3) out;

in mat3x2 v_positions[1];
in mat3x2 v_texcoords[1];

out vec2 g_texcoord;

void main() {
    gl_Position = vec4(v_positions[0][0], 0.0, 1.0);
    g_texcoord = v_texcoords[0][0];
    EmitVertex();
    gl_Position = vec4(v_positions[0][1], 0.0, 1.0);
    g_texcoord = v_texcoords[0][1];
    EmitVertex();
    gl_Position = vec4(v_positions[0][2], 0.0, 1.0);
    g_texcoord = v_texcoords[0][2];
    EmitVertex();
}
