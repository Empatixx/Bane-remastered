#version 330 core
#define AMBIENT_LIGHT 0.6
#define MAX_LIGHTS 10
#define BLOCK_SIZE 12.0

uniform sampler2D texSampler; // framebuffer texture
uniform vec2 lightPosition[MAX_LIGHTS]; // array of light positions in screen space
uniform vec3 lightColor[MAX_LIGHTS]; // array of light colors
uniform float lightIntensity[MAX_LIGHTS]; // array of light intensities
uniform float lightRadius[MAX_LIGHTS]; // array of light radius (in pixels)
uniform int numLights; // number of active lights
uniform vec2 resolution; // resolution of the framebuffer
uniform vec2 cameraPosition; // camera position in world space

in vec2 tex_coords;

// function to calculate the diffuse lighting
float calcDiffuse(vec2 lightDir, float lightRadius, float lightIntensity) {
    float dist = length(lightDir);
    float gradient = 1.0 - clamp(floor(dist / BLOCK_SIZE) * BLOCK_SIZE / lightRadius, 0.0, 1.0); // radial gradient with circular transition
    return gradient * lightIntensity;
}

void main() {
    vec4 albedo = texture2D(texSampler, tex_coords);
    vec2 screenSpaceTexCoord = tex_coords * resolution;

    vec2 pixelatedScreenSpaceTexCoord = floor(screenSpaceTexCoord / BLOCK_SIZE) * BLOCK_SIZE;

    vec3 finalLightColor = vec3(0.0, 0.0, 0.0);

    for (int i = 0; i < numLights; ++i) {
        vec2 calcLightPosition = vec2(lightPosition[i].x - cameraPosition.x, resolution.y - lightPosition[i].y + cameraPosition.y); // flip Y-axis of light position
        vec2 pixelatedLightPosition = floor(calcLightPosition / BLOCK_SIZE) * BLOCK_SIZE;

        vec2 lightDir = pixelatedLightPosition - pixelatedScreenSpaceTexCoord;
        float diffuse = calcDiffuse(lightDir, lightRadius[i], lightIntensity[i]); // calculate diffuse lighting
        finalLightColor += diffuse * lightColor[i];
    }

    gl_FragColor = vec4(albedo.rgb * (AMBIENT_LIGHT + finalLightColor), albedo.a);
}
