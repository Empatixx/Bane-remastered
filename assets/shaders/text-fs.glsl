#version 330

uniform sampler2D texSampler;
uniform vec3 color;

in vec2 tex_coords;

void main(){
    gl_FragColor = vec4(vec3(color), texture2D(texSampler, tex_coords).r);
    if  (gl_FragColor.a < 0.1){
        discard;
    }
}