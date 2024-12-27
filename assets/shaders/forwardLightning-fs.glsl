#version 330 core
#define AMBIENT_LIGHT 0.6
#define MAX_LIGHTS 254
#define NOISE_INTENSITY 0.15

struct Light {
    vec2 position;
    vec3 color;
    float intensity;
    float radius;
};

uniform sampler2D texSampler;

uniform Light lights[MAX_LIGHTS];
uniform int numLights;

uniform vec2 u_targetResolution;
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
    vec2 screenSpaceTexCoord = tex_coords * u_targetResolution;

    vec3 finalLightColor = vec3(0.0, 0.0, 0.0);

    for (int i = 0; i < numLights; ++i) {
        vec2 lightPos = lights[i].position;
        vec3 lightColor = lights[i].color;
        float intensity = lights[i].intensity;
        float radius = lights[i].radius;

        vec2 lightPositionInScreenSpace = lightPos / u_targetResolution;
        vec2 cameraPositionInScreenSpace = cameraPosition / u_targetResolution;

        vec2 calcLightPosition = vec2(lightPositionInScreenSpace.x - cameraPositionInScreenSpace.x, 1.0 - lightPositionInScreenSpace.y + cameraPositionInScreenSpace.y); // Updated calculation
        vec2 worldSpaceLightPosition = calcLightPosition * u_targetResolution;

        vec2 screenSpacePixelatedLightPosition = worldSpaceLightPosition / u_targetResolution;

        vec2 lightDirWorldSpace = screenSpacePixelatedLightPosition * u_targetResolution - screenSpaceTexCoord;
        float diffuse = calcDiffuse(lightDirWorldSpace, radius, intensity); // calculate diffuse lighting
        finalLightColor += diffuse * lightColor;
    }

    float noiseValue = random(tex_coords); // calculate noise value
    float adjustedNoise = mix(1.0, noiseValue, NOISE_INTENSITY);
    gl_FragColor = vec4(albedo.rgb * (AMBIENT_LIGHT + finalLightColor * adjustedNoise), albedo.a);
}

