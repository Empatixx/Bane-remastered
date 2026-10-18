#version 330 core

uniform mat4 u_viewMatrix;
uniform mat4 u_projectionMatrix;

layout (location = 0) in vec3 vertices;
layout (location = 1) in vec2 textures;

void main() {
    gl_Position = u_projectionMatrix * u_viewMatrix * vec4(vertices, 1.0);
}
