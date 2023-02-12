#version 330
#define MAX_TEXTURES 32

uniform sampler2D textures[MAX_TEXTURES];
uniform vec3 color;

in vec2 tex_coords;
in float tex_index;

void main(){
    int index = int(tex_index);
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture2D(textures[index], tex_coords).r);
    gl_FragColor = vec4(color, 1.0) * sampled;
}