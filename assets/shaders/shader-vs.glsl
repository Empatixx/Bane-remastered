#version 130

uniform mat4 projection;

in vec2 vertices;
in vec2 textures;
out vec2 tex_coords;

void main() {
    tex_coords = textures;
    gl_Position = projection * vec4(vertices, 0, 1);
}