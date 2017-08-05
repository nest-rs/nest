#version 140

layout(points) in;
layout(triangle_strip, max_vertices = 3) out;

in vec4 f_color;

void main() {
    gl_Position = projection * (center + vec4(delta, 0, 0));
    EmitVertex();
    delta = full_radius * vec2(-1.7320508075689, -1);
    gl_Position = projection * (center + vec4(delta, 0, 0));
    EmitVertex();
    delta = full_radius * vec2(1.7320508075689, -1);
    gl_Position = projection * (center + vec4(delta, 0, 0));
    EmitVertex();
}
