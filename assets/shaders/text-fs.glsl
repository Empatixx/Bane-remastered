#version 330

uniform sampler2D texSampler;
uniform vec3 color;

in vec2 tex_coords;

void main(){
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture2D(texSampler, tex_coords).r);
    gl_FragColor = vec4(color, 1.0) * sampled;
}