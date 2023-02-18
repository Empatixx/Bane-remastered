#version 330
#define MAX_TEXTURES 32

uniform sampler2D textures[32];

in vec2 tex_coords;
in float tex_index;

void main() {
    int index = int(tex_index);
    gl_FragColor = texture2D(textures[index], tex_coords);
}
