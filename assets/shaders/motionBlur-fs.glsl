
#version 330 core

uniform sampler2D texSampler;
uniform vec2 motionVector; // Adjusted to reflect the distance and direction of movement

in vec2 tex_coords;
out vec4 fragColor;

void main() {
    vec4 baseColor = texture(texSampler, tex_coords);
    vec4 accumulatedColor = baseColor * 0.22; // Base weight
    float totalWeight = 0.22;
    
    for (int i = 1; i <= 4; i++) {
        float weight = 0.78 * pow(0.5, float(i)); // Example decay factor for weight
        vec2 offset = motionVector * float(i) * 0.005; // Adjust the scale factor
        accumulatedColor += texture(texSampler, tex_coords + offset) * weight;
        accumulatedColor += texture(texSampler, tex_coords - offset) * weight;
        totalWeight += 2.0 * weight;
    }
    
    gl_FragColor = accumulatedColor / totalWeight; // Normalize the accumulated color
}