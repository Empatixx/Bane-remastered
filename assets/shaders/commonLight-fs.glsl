#version 330 core

#define MAX_LIGHTS 255
#define NOISE_INTENSITY 0.10
#define AMBIENT_LIGHT vec3(0.6)

struct Light {
    vec2 position;
    vec3 color;
    float intensity;
    float radius;
};

uniform vec2 u_resolution;

uniform sampler2D texSampler;
uniform sampler2D noiseSampler;

uniform Light lights[MAX_LIGHTS];
uniform int numLights;

in vec2 tex_coords;
in vec2 world_position;

out vec4 FragColor;

float calcDiffuse(vec2 lightDir, float lightRadius, float lightIntensity) {
    float dist = length(lightDir);
    float normalizedDist = dist / lightRadius;
    float attenuation = 1.0 / (1.0 + normalizedDist * normalizedDist);
    return clamp(attenuation, 0.0, 1.0) * lightIntensity;
}

vec3 computeTotalDiffuse(vec2 worldPos) {
    vec3 result = vec3(0.0);
    result += AMBIENT_LIGHT;

    for (int i = 0; i < numLights; ++i) {
        vec2 lightDir = lights[i].position - worldPos;
        result += calcDiffuse(lightDir, lights[i].radius, lights[i].intensity) * lights[i].color;
    }
    return result;
}

void main() {
    vec4 albedo = texture(texSampler, tex_coords);
    if (albedo.a <= 0.1) discard;

    vec3 totalDiffuse = computeTotalDiffuse(world_position);

    vec2 noiseUV = gl_FragCoord.xy / u_resolution;
    float noiseValue = texture(noiseSampler, noiseUV).r;
    totalDiffuse *= (1.0 + noiseValue * NOISE_INTENSITY);

    FragColor = vec4(albedo.rgb * totalDiffuse, albedo.a);
}
