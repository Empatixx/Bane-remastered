#version 330 core

uniform sampler2D texSampler;
uniform float u_shadowIntensity;

in vec2 tex_coords;

out vec4 FragColor;

void main() {
    vec4 texColor = texture(texSampler, tex_coords);

    // Discard fully transparent pixels
    if (texColor.a < 0.1) {
        discard;
    }

    // Output black with the texture's alpha multiplied by shadow intensity
    FragColor = vec4(0.0, 0.0, 0.0, texColor.a * u_shadowIntensity);
}
