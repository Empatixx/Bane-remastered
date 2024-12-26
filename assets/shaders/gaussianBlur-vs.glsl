#version 330

uniform mat4 projection;

layout (location = 0) in vec3 vertices;
layout (location = 1) in vec2 textures;

out vec2 tex_coords;

void main() {
    tex_coords = textures;
    gl_Position = projection * vec4(vertices, 1.0f);
}