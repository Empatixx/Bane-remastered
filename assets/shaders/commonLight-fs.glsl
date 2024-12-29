#version 330 core
#define AMBIENT_LIGHT 0.6
#define MAX_LIGHTS 254
#define NOISE_INTENSITY 0.25

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
    float gradient = clamp(attenuation, 0.0, 1.0);
    return gradient * lightIntensity;
}


void main() {
    vec4 albedo = texture(texSampler, tex_coords);
    if (albedo.a <= 0.1){
        discard;
    }
    vec3 finalLightColor = vec3(0.0);
    for (int i = 0; i < numLights; ++i) {
        vec2 lightDir = lights[i].position - world_position;
        float diffuse = calcDiffuse(lightDir, lights[i].radius, lights[i].intensity);
        finalLightColor += diffuse * lights[i].color;
    }

    vec2 noiseUV = gl_FragCoord.xy / u_resolution;
    float noiseValue = texture(noiseSampler, noiseUV).r;

    finalLightColor *= (1.0 + noiseValue * NOISE_INTENSITY);

    FragColor = vec4(albedo.rgb * (AMBIENT_LIGHT + finalLightColor), albedo.a);
    //FragColor = vec4(vec3(noiseValue), 1.0);
}




