#version 330 core

uniform vec2 uTexSize;
uniform float uBlurAmount;
uniform float uDarknessAmount;
uniform sampler2D uTexture;

in vec2 tex_coords;

const int KERNEL_SIZE = 9;
const float KERNEL[KERNEL_SIZE] = float[](0.0093, 0.028002, 0.065984, 0.121703, 0.175713, 0.121703, 0.065984, 0.028002, 0.0093);

vec4 applyBlur(vec2 texCoord) {
    vec4 color = vec4(0.0);
    vec2 texOffset = 1.0 / uTexSize;
    float weightSum = 0.0;

    for (int i = 0; i < KERNEL_SIZE; ++i) {
        float weight = KERNEL[i];
        vec2 offset = vec2(float(i) - float(KERNEL_SIZE / 2), 0.0) * texOffset * uBlurAmount;
        color += texture2D(uTexture, texCoord + offset) * weight;

        if (i != 0) {
            color += texture2D(uTexture, texCoord - offset) * weight;
        }

        weightSum += (i == 0) ? weight : weight * 2.0;
    }

    color /= weightSum;
    return color;
}

void main() {
    gl_FragColor = applyBlur(tex_coords);
    gl_FragColor.rgb *= uDarknessAmount;
}
