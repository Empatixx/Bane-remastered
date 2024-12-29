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

in vec2 tex_coords;
in vec2 world_position;

out vec4 FragColor;

float calcDiffuse(vec2 lightDir, float lightRadius, float lightIntensity) {
    float dist = length(lightDir);
    float normalizedDist = dist / lightRadius;
    float attenuation = 1.0 / (1.0 + normalizedDist * normalizedDist);
    float gradient = clamp(attenuation, 0.0, 1.0);
    return gradient * lightIntensity;
}

float random(vec2 st) {
    return fract(sin(dot(st.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

void main() {
    vec4 albedo = texture(texSampler, tex_coords);

    vec3 finalLightColor = vec3(0.0);

    for (int i = 0; i < numLights; ++i) {
        vec2 lightDir = lights[i].position - world_position;
        float diffuse = calcDiffuse(lightDir, lights[i].radius, lights[i].intensity);
        finalLightColor += diffuse * lights[i].color;
    }

    float noiseValue = random(tex_coords);
    float adjustedNoise = mix(1.0, noiseValue, NOISE_INTENSITY);

    FragColor = vec4(albedo.rgb * (AMBIENT_LIGHT + finalLightColor * adjustedNoise), albedo.a);
}
