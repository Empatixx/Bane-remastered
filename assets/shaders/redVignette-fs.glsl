#version 330
// Inputs
in vec2 tex_coords;

// Uniforms
uniform sampler2D u_texture;
uniform float u_vignetteAmount; // Value from 0.0 (no health) to 1.0 (full health)
uniform float u_time; // Time in seconds
uniform int u_timeBased; // 1 for time-based, 0 for hit-based

void main() {
    // Calculate distance from center
    float dist = distance(tex_coords, vec2(0.5));

    // Vignette effect
    float vignette = smoothstep(0.4, 0.6, dist);

    // Get original color
    vec4 color = texture2D(u_texture, tex_coords);

    // Calculate intensity based on mode
    float intensity;
    if (u_timeBased == 1) {
        // Time-based: intensity has a heartbeat-like pulse
        float pulse = fract(u_time * 0.5); // Decrease the speed of the pulse by multiplying time by 0.5
        intensity = exp(-6.0 * (pulse - 0.1) * (pulse - 0.1)) * 0.5; // Keep the intensity the same
    } else {
        // Hit-based: intensity based on health
        intensity = u_vignetteAmount  * 0.5;
    }

    // Interpolate between original color and red based on intensity and vignette
    color = mix(color, vec4(1.0, 0.0, 0.0, 1.0), intensity * vignette);

    gl_FragColor = color;
}
