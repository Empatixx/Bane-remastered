#version 330

uniform sampler2D u_TexSampler;

in vec2 tex_coords;

void main() {
    gl_FragColor = texture2D(u_TexSampler, tex_coords);
}
