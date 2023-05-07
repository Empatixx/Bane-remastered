#version 330

uniform sampler2D texSampler;

in vec2 tex_coords;

void main() {
    gl_FragColor = texture2D(texSampler, tex_coords);
}
```