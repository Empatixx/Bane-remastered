#version 330

#define OCTAVES 5
#define AMPLITUDE 0.5
#define FREQUENCY 2.0
#define HORIZONTAL_SPEED 0.05
#define VERTICAL_SPEED 0.0
#define HORIZONTAL_OSCILLATION_FACTOR 0.25
#define VERTICAL_OSCILLATION_FACTOR 0.25
#define FOG_COLOR vec3(0.25)
#define OUTLINE_COLOR vec4(1, 0.25, 0.25, 1.0)
#define OUTLINE_WIDTH 0.0625
#define GLOW_SPEED 2.0
#define DEFAULT_COLOR vec4(0.8196, 0.5333, 0.4117, 1.0)
#define GATES_WIDTH 0.21625
#define GATE_UV_START 0.5-GATES_WIDTH
#define GATE_UV_END 0.5+GATES_WIDTH

uniform sampler2D uTexSampler;
uniform int uVisited;
uniform int uCurrent;
uniform float uSpriteShift;
uniform float uAlpha;
uniform float uTime;
uniform vec2 uAtlasSize;
uniform vec2 uSubImagePos;
uniform vec2 uSpriteSize;
uniform ivec4 uConnections; // left, right, up, down

in vec2 tex_coords;

vec2 random(vec2 uv) {
    uv = vec2(dot(uv, vec2(127.1, 311.7)),
              dot(uv, vec2(269.5, 183.3)));
    return -1.0 + 2.0 * fract(sin(uv) * 43758.5453123);
}

float noise(vec2 uv, float seed_h, float seed_v) {

    uv.x = uv.x + seed_h;
    uv.y = uv.y + seed_v;

    vec2 uv_index = floor(uv);
    vec2 uv_fract = fract(uv);

    vec2 blur = smoothstep(0.0, 1.0, uv_fract);

    return mix(mix(dot(random(uv_index + vec2(0.0, 0.0)), uv_fract - vec2(0.0, 0.0)),
                   dot(random(uv_index + vec2(1.0, 0.0)), uv_fract - vec2(1.0, 0.0)), blur.x),
               mix(dot(random(uv_index + vec2(0.0, 1.0)), uv_fract - vec2(0.0, 1.0)),
                   dot(random(uv_index + vec2(1.0, 1.0)), uv_fract - vec2(1.0, 1.0)), blur.x), blur.y) + 0.5;
}


float fbm(vec2 uv, float seed_h, float seed_v) {
    float value = 0.0;
    float frequency = FREQUENCY;
    float amplitude = AMPLITUDE;

    for (int i = 0; i < OCTAVES; i++) {
        float noise_val = noise(uv.xy * frequency, seed_h, seed_v);
        value += amplitude * noise_val;
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    return value;
}

void main() {
    vec4 baseColor = texture2D(uTexSampler, tex_coords);
    vec2 shift = vec2(uSpriteShift, 0.0);
    // normalizing from subimage coords to [0, 1]
    vec2 uv = (tex_coords - uSubImagePos - shift) / uSpriteSize;

    bool isGate = false;
    if (uConnections.x == 1 && uv.x < OUTLINE_WIDTH && uv.y > GATE_UV_START && uv.y < GATE_UV_END) {
        baseColor = DEFAULT_COLOR;
        isGate = true;
    }
    if (uConnections.y == 1 && uv.x > 1.0 - OUTLINE_WIDTH && uv.y > GATE_UV_START && uv.y < GATE_UV_END) {
        baseColor = DEFAULT_COLOR;
        isGate = true;
    }
    if (uConnections.z == 1 && uv.y < OUTLINE_WIDTH && uv.x > GATE_UV_START && uv.x < GATE_UV_END) {
        baseColor = DEFAULT_COLOR;
        isGate = true;
    }
    if (uConnections.w == 1 && uv.y > 1.0 - OUTLINE_WIDTH && uv.x > GATE_UV_START && uv.x < GATE_UV_END) {
        baseColor = DEFAULT_COLOR;
        isGate = true;
    }


    if (uVisited == 0) {
        float horizontal_seed = uTime * HORIZONTAL_OSCILLATION_FACTOR;
        float vertical_seed = uTime * VERTICAL_OSCILLATION_FACTOR;
        // noise
        uv.x = uv.x - uTime * HORIZONTAL_SPEED;
        uv.y = uv.y + uTime * HORIZONTAL_SPEED;
        float noise = fbm(uv, horizontal_seed, vertical_seed);
        vec3 col = FOG_COLOR * noise;

        vec4 fogColor = vec4(col, 1.0);
        gl_FragColor = mix(baseColor, fogColor, noise);
    } else if (uCurrent == 1 && !isGate) {
        float pulse = (sin(uTime * GLOW_SPEED) + 1.0) * 0.5;
        vec4 outlineColor = OUTLINE_COLOR * pulse;

        float distToEdgeX = min(uv.x, 1.0 - uv.x);
        float distToEdgeY = min(uv.y, 1.0 - uv.y);
        float distToEdge = min(distToEdgeX, distToEdgeY);

        if (distToEdge < OUTLINE_WIDTH) {
            gl_FragColor = mix(vec4(outlineColor.rgb, 1.0), vec4(baseColor.rgb, 1.0), 1.0 - outlineColor.a);
        } else {
            gl_FragColor = baseColor;
        }
    } else {
        gl_FragColor = baseColor;
    }
    gl_FragColor.a *= uAlpha;
}
