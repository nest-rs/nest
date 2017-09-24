#version 150

in vec2 g_texcoord;
in vec4 g_color;

void main() {
    float intensity = 1.0 - pow(length(g_texcoord), 2);
    gl_FragColor = intensity * g_color;
}