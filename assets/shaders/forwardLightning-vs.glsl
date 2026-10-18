#version 330

uniform mat4 u_projectionMatrix;

layout (location = 0) in vec3 vertices;
layout (location = 1) in vec2 textures;

out vec2 tex_coords;

void main() {
    tex_coords = textures;
    gl_Position = u_projectionMatrix * vec4(vertices, 1.0f);
}