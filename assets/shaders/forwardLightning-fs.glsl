#version 330 core
#define AMBIENT_LIGHT 0.6
#define MAX_LIGHTS 254
#define NOISE_INTENSITY 0.15

uniform sampler2D texSampler;
uniform vec2 lightPosition[MAX_LIGHTS];
uniform vec3 lightColor[MAX_LIGHTS];
uniform float lightIntensity[MAX_LIGHTS];
uniform float lightRadius[MAX_LIGHTS];
uniform int numLights;
uniform vec2 resolution;
uniform vec2 cameraPosition;

in vec2 tex_coords;

float calcDiffuse(vec2 lightDir, float lightRadius, float lightIntensity) {
    float dist = length(lightDir);
    float normalizedDist = dist / lightRadius;
    float attenuation = 1.0 / (1.0 + normalizedDist * normalizedDist); // Inverse square law for light attenuation
    float gradient = clamp(attenuation, 0.0, 1.0); // Clamp the attenuation to avoid negative values
    return gradient * lightIntensity;
}
float random(vec2 st) {
    return fract(sin(dot(st.xy, vec2(12.9898, 78.233))) * 43758.5453);
}


void main() {
    vec4 albedo = texture2D(texSampler, tex_coords);
    vec2 screenSpaceTexCoord = tex_coords * resolution;

    vec3 finalLightColor = vec3(0.0, 0.0, 0.0);

    for (int i = 0; i < numLights; ++i) {
        vec2 lightPositionInScreenSpace = lightPosition[i] / resolution;
        vec2 cameraPositionInScreenSpace = cameraPosition / resolution;

        vec2 calcLightPosition = vec2(lightPositionInScreenSpace.x - cameraPositionInScreenSpace.x, 1.0 - lightPositionInScreenSpace.y + cameraPositionInScreenSpace.y); // Updated calculation
        vec2 worldSpaceLightPosition = calcLightPosition * resolution;

        vec2 screenSpacePixelatedLightPosition = worldSpaceLightPosition / resolution;

        vec2 lightDirWorldSpace = screenSpacePixelatedLightPosition * resolution - screenSpaceTexCoord;
        float diffuse = calcDiffuse(lightDirWorldSpace, lightRadius[i], lightIntensity[i]); // calculate diffuse lighting
        finalLightColor += diffuse * lightColor[i];
    }

    float noiseValue = random(tex_coords); // calculate noise value
    float adjustedNoise = mix(1.0, noiseValue, NOISE_INTENSITY);
    gl_FragColor = vec4(albedo.rgb * (AMBIENT_LIGHT + finalLightColor * adjustedNoise), albedo.a);
}

