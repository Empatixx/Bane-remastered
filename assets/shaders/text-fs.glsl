#version 130

uniform sampler2D sampler;
uniform vec3 color;

in vec2 tex_coords;

void main(){
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(sampler, tex_coords).r);
    gl_FragColor = vec4(color, 1.0) * sampled;
}