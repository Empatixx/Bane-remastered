uniform sampler2D uTexSampler;
uniform float uOutlineThickness;
uniform vec4 uOutlineColor;
uniform vec2 uTexSize;

uniform vec2 uTexStartPos;
uniform vec2 uTexEndPos;

in vec2 tex_coords;

void main() {

    float centerAlpha = texture2D(uTexSampler, tex_coords).a;

    const vec2 offsets[8] = vec2[](
        vec2(-1, -1), vec2(0, -1), vec2(1, -1),
        vec2(-1, 0), vec2(1, 0),
        vec2(-1, 1), vec2(0, 1), vec2(1, 1)
    );

    bool isEdge = false;
    gl_FragColor = texture2D(uTexSampler, tex_coords);

    for (int i = 0; i < 8; i++) {
        vec2 samplePos = clamp(tex_coords + uOutlineThickness * offsets[i] / uTexSize, uTexStartPos, uTexEndPos);

        float neighborAlpha = texture2D(uTexSampler, samplePos).a;

        if (abs(centerAlpha - neighborAlpha) > 0.1) {
            gl_FragColor = uOutlineColor;
            break;
        }
    }
}