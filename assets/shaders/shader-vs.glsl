#version 330

uniform mat4 projection;

layout (location = 0) in vec2 vertices;
layout (location = 1) in vec2 textures;
layout (location = 2) in float textureIndex;

out vec2 tex_coords;
out float tex_index;

void main() {
    tex_coords = textures;
    tex_index = textureIndex;
    gl_Position = projection * vec4(vertices, 0.0f, 1.0f);
}