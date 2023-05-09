#version 330

uniform sampler2D u_texture;
uniform float u_fillAmount;
uniform vec4 u_barRect;
uniform vec3 u_barColor;

in vec2 tex_coords;

void main() {
    vec4 texColor = texture(u_texture, tex_coords);

    // Check if the current fragment is within the health bar rectangle
    if (tex_coords.x >= u_barRect.x && tex_coords.x <= (u_barRect.x + u_barRect.z) &&
    tex_coords.y >= u_barRect.y && tex_coords.y <= (u_barRect.y + u_barRect.w)) {

        // If the current fragment is within the filled portion of the health bar, render it.
        //if (tex_coords.x <= u_barRect.x + u_fillAmount * u_barRect.z || texColor.a > 0.0) {
            gl_FragColor = vec4(u_barColor,1.);
        //} else {
        //    gl_FragColor = texColor;s
        //}
    } else {
        gl_FragColor = texColor;
    }
}