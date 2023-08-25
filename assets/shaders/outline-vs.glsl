#version 330

uniform mat4 uProjection;

layout (location = 0) in vec2 vertices;
layout (location = 1) in vec2 textures;

out vec2 tex_coords;

void main() {
    tex_coords = textures;
    gl_Position = uProjection * vec4(vertices, 0.0f, 1.0f);
}