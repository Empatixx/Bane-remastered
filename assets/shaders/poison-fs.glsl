#version 330
// Inputs
in vec2 tex_coords;

// Uniforms
uniform sampler2D u_texture;
uniform float u_vignetteAmount; // Value from 0.0 (no effect) to 1.0 (full effect)
uniform float u_time;
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
        // Hit-based: intensity based on poison effect strength
    intensity = u_vignetteAmount * 0.5;

    // Poison color shift towards green with slight noise
    vec3 poisonColor = vec3(0.0, 1.0, 0.0); // Green color for poison
    float noise = fract(sin(dot(tex_coords, vec2(12.9898, 78.233))) * 43758.5453);
    vec3 finalColor = mix(color.rgb, poisonColor, intensity * vignette * (0.8 + 0.2 * noise));

    // Optional: Add distortion effect based on poison intensity (simulating disorientation or dizziness)
    vec2 distortion = tex_coords + (vec2(0.5) - tex_coords) * intensity * vignette * noise * 0.05;
    finalColor = mix(finalColor, texture2D(u_texture, distortion).rgb, intensity * vignette);

    gl_FragColor = vec4(finalColor, 1.0);
}
