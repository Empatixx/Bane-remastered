#version 330

uniform sampler2D uTexSample;
uniform float uTime; // Represents the progression of the spawn animation, from 0.0 (start) to 1.0 (end)

in vec2 tex_coords;

void main() {
    float startBlinkingAt = 0.7; // Start blinking when time is greater than 0.75
    float blinkSpeed = 20.0; // Controls how fast the blinking occurs

    gl_FragColor = texture2D(uTexSample, tex_coords);

    if (gl_FragColor.a > 0.0) {
        gl_FragColor = vec4(0,0,0,1); // Set the color to black
        if (uTime > startBlinkingAt){
            float blink = abs(sin((uTime - startBlinkingAt) * blinkSpeed));
            if(blink > 0.5) {
                gl_FragColor = vec4(1, 1, 1, 1); // Blink to white
            } else {
                gl_FragColor = vec4(0, 0, 0, 1);
            }
        }
    }
}