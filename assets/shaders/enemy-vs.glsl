#version 330

uniform mat4 u_Projection;
uniform int u_FacingRight;

layout (location = 0) in vec2 vertices;
layout (location = 1) in vec2 textures;

out vec2 tex_coords;

void main() {
    tex_coords = textures;
    if (u_FacingRight == 1) {
        tex_coords.x = 1.0f - tex_coords.x;
    }
    gl_Position = u_Projection * vec4(vertices, 0.0f, 1.0f);
}