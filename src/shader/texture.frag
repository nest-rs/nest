#version 150

in vec2 g_texcoord;
in vec4 g_color;

uniform sampler2D tex;

void main() {
    gl_FragColor = g_color * texture(tex, g_texcoord);
}