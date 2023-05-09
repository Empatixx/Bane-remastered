#version 330

uniform sampler2D u_texture;
uniform float u_fillAmount;
uniform float u_delayedFillAmount;
uniform vec4 u_barRect;
uniform vec3 u_barColor;

in vec2 tex_coords;

void main() {
    vec4 texColor = texture(u_texture, tex_coords);

    // Calculate the relative position within the status bar
    vec2 relPos = (tex_coords - u_barRect.xy) / u_barRect.zw;

    float change = u_fillAmount - u_delayedFillAmount;

    // Check if the current fragment is within the health bar rectangle
    if (relPos.x >= 0.0 && relPos.x <= 1.0 &&
    relPos.y >= 0.0 && relPos.y <= 1.0) {

        bool isIncreasing = change >= 0.0;
        float primaryFill = isIncreasing ? u_delayedFillAmount : u_fillAmount;
        float secondaryFill = isIncreasing ? u_fillAmount : u_delayedFillAmount;

        if (relPos.x <= primaryFill) {
            gl_FragColor = vec4(u_barColor, 1.0);
        } else if (relPos.x <= secondaryFill) {
            gl_FragColor = vec4(isIncreasing ? vec3(u_barColor.r, 0.9, u_barColor.b) : vec3(.9, u_barColor.gb), 1);
        } else {
            gl_FragColor = vec4(0.0);
        }

        if(relPos.y >= 0.75){
            gl_FragColor.rgb += vec3(0.318, 0.168, 0.161);
        } else if(relPos.y >= 0.5){
            gl_FragColor.rgb += vec3(0.149, 0.068, 0.104);
        }
    } else {
        gl_FragColor = texColor;
    }
}