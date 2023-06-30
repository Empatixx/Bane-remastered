#version 330

uniform sampler2D uTexSampler;
uniform float uValue;

in vec2 tex_coords;

void main() {
    vec2 center = vec2(0.5, 0.5);

    // Calculate distance from the center
    float distanceFromCenter = distance(tex_coords, center);

    // Normalize distance (diagonal of texture is sqrt(0.5^2 + 0.5^2) = sqrt(0.5))
    float normalizedDistance = distanceFromCenter / sqrt(0.5*0.5 + 0.5*0.5);

    // Clamp the distance to avoid issues at the center
    normalizedDistance = clamp(normalizedDistance, 0.0, 1.0);

    // Get the color from the texture
    vec4 color = texture(uTexSampler, tex_coords);
    gl_FragColor = color;

    // Determine whether to use the texture color or black based on transparency and distance
    if (normalizedDistance > (1.0 - uValue)) {
        float alpha = min(1.0, (normalizedDistance - (1.0 - uValue)) * 1.5);  // Adjust the 10.0 factor as needed
        vec4 black = vec4(0.0, 0.0, 0.0, alpha);
        gl_FragColor = mix(color, black, alpha);
    }
}