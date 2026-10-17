const NOISE_INTENSITY: f32 = 0.10;

struct Globals {
    size: vec2<f32>,
    camera: vec2<f32>,
};

@group(0) @binding(0) var<uniform> globals: Globals;
@group(0) @binding(1) var lightmap: texture_2d<f32>;
@group(0) @binding(2) var lightmap_sampler: sampler;
@group(1) @binding(0) var image: texture_2d<f32>;
@group(1) @binding(1) var image_sampler: sampler;

struct Instance {
    @location(0) center: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) uv: vec4<f32>,
    @location(4) lit_angle: vec2<f32>,
    @location(5) fx: vec4<f32>,
    @location(6) local: vec4<f32>,
    @location(7) outline: vec4<f32>,
};

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) lit: f32,
    @location(3) world: vec2<f32>,
    @location(4) @interpolate(flat) fx: vec4<f32>,
    @location(5) local: vec2<f32>,
    @location(6) @interpolate(flat) outline: vec4<f32>,
    @location(7) @interpolate(flat) bounds: vec4<f32>,
};

fn srgb_to_linear(color: vec3<f32>) -> vec3<f32> {
    let low = color / 12.92;
    let high = pow((color + vec3<f32>(0.055)) / 1.055, vec3<f32>(2.4));
    return select(high, low, color <= vec3<f32>(0.04045));
}

fn linear_to_srgb(color: vec3<f32>) -> vec3<f32> {
    let low = color * 12.92;
    let high = 1.055 * pow(color, vec3<f32>(1.0 / 2.4)) - vec3<f32>(0.055);
    return select(high, low, color <= vec3<f32>(0.0031308));
}

fn noise(position: vec2<f32>) -> f32 {
    return fract(sin(dot(position, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

const MAP_FOG: vec3<f32> = vec3<f32>(0.1, 0.1, 0.1);
const MAP_GATE: vec3<f32> = vec3<f32>(0.8196, 0.5333, 0.4117);
const MAP_OUTLINE: f32 = 0.0625;
const MAP_GATE_START: f32 = 0.28375;
const MAP_GATE_END: f32 = 0.71625;
const MAP_OCTAVES: i32 = 5;
const MAP_AMPLITUDE: f32 = 0.5;
const MAP_FREQUENCY: f32 = 2.0;
const MAP_SPEED: f32 = 0.05;
const MAP_OSCILLATION: f32 = 0.25;

fn map_random(uv: vec2<f32>) -> vec2<f32> {
    let p = vec2<f32>(dot(uv, vec2<f32>(127.1, 311.7)), dot(uv, vec2<f32>(269.5, 183.3)));
    return vec2<f32>(-1.0) + 2.0 * fract(sin(p) * 43758.5453123);
}

fn map_noise(uv_in: vec2<f32>, seed_h: f32, seed_v: f32) -> f32 {
    let uv = vec2<f32>(uv_in.x + seed_h, uv_in.y + seed_v);
    let index = floor(uv);
    let part = fract(uv);
    let blur = smoothstep(vec2<f32>(0.0), vec2<f32>(1.0), part);
    let a = dot(map_random(index + vec2<f32>(0.0, 0.0)), part - vec2<f32>(0.0, 0.0));
    let b = dot(map_random(index + vec2<f32>(1.0, 0.0)), part - vec2<f32>(1.0, 0.0));
    let c = dot(map_random(index + vec2<f32>(0.0, 1.0)), part - vec2<f32>(0.0, 1.0));
    let d = dot(map_random(index + vec2<f32>(1.0, 1.0)), part - vec2<f32>(1.0, 1.0));
    return mix(mix(a, b, blur.x), mix(c, d, blur.x), blur.y) + 0.5;
}

fn map_fbm(uv: vec2<f32>, seed_h: f32, seed_v: f32) -> f32 {
    var value = 0.0;
    var frequency = MAP_FREQUENCY;
    var amplitude = MAP_AMPLITUDE;
    for (var i = 0; i < MAP_OCTAVES; i = i + 1) {
        value = value + amplitude * map_noise(uv * frequency, seed_h, seed_v);
        amplitude = amplitude * 0.5;
        frequency = frequency * 2.0;
    }
    return value;
}

fn map_room(in: VertexOut, texel: vec4<f32>) -> vec4<f32> {
    let uv = in.local;
    let links = u32(in.fx.z + 0.5);
    let along_x = uv.y > MAP_GATE_START && uv.y < MAP_GATE_END;
    let along_y = uv.x > MAP_GATE_START && uv.x < MAP_GATE_END;
    var gate = false;
    if ((links & 1u) != 0u && uv.x < MAP_OUTLINE && along_x) { gate = true; }
    if ((links & 2u) != 0u && uv.x > 1.0 - MAP_OUTLINE && along_x) { gate = true; }
    if ((links & 4u) != 0u && uv.y < MAP_OUTLINE && along_y) { gate = true; }
    if ((links & 8u) != 0u && uv.y > 1.0 - MAP_OUTLINE && along_y) { gate = true; }
    var color = vec4<f32>(texel.rgb * in.color.rgb, texel.a);
    if (gate) {
        color = vec4<f32>(srgb_to_linear(MAP_GATE), 1.0);
    }
    if (in.fx.w > 1.5) {
        let time = in.fx.x;
        let seed = in.fx.y;
        let oscillation = (time + seed) * MAP_OSCILLATION;
        let moved = vec2<f32>(uv.x - time * MAP_SPEED, uv.y + time * MAP_SPEED);
        let value = map_fbm(moved, oscillation, oscillation);
        let fog = vec4<f32>(srgb_to_linear(MAP_FOG * value), 1.0);
        color = mix(color, fog, value);
    }
    return vec4<f32>(color.rgb, color.a * in.color.a);
}

@vertex
fn vs_main(@builtin(vertex_index) index: u32, instance: Instance) -> VertexOut {
    var corners = array<vec2<f32>, 6>(
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5),
        vec2<f32>(-0.5, 0.5),
        vec2<f32>(-0.5, 0.5),
        vec2<f32>(0.5, -0.5),
        vec2<f32>(0.5, 0.5),
    );
    let corner = corners[index];
    let angle = instance.lit_angle.y;
    var pixel: vec2<f32>;
    if (angle == 0.0) {
        let low = round(instance.center - instance.size * 0.5);
        let high = max(round(instance.center + instance.size * 0.5), low + min(vec2<f32>(1.0), instance.size));
        pixel = mix(low, high, corner + vec2<f32>(0.5, 0.5));
    } else {
        let offset = corner * instance.size;
        let rotated = vec2<f32>(
            offset.x * cos(angle) - offset.y * sin(angle),
            offset.x * sin(angle) + offset.y * cos(angle),
        );
        pixel = instance.center + rotated;
    }
    let ndc = vec2<f32>(
        pixel.x / globals.size.x * 2.0 - 1.0,
        1.0 - pixel.y / globals.size.y * 2.0,
    );
    var out: VertexOut;
    out.position = vec4<f32>(ndc, 0.0, 1.0);
    out.color = vec4<f32>(srgb_to_linear(instance.color.rgb), instance.color.a);
    out.uv = mix(instance.uv.xy, instance.uv.zw, corner + vec2<f32>(0.5, 0.5));
    out.lit = instance.lit_angle.x;
    out.world = pixel + globals.camera;
    out.fx = instance.fx;
    out.local = mix(instance.local.xy, instance.local.zw, corner + vec2<f32>(0.5, 0.5));
    out.outline = instance.outline;
    out.bounds = vec4<f32>(min(instance.uv.xy, instance.uv.zw), max(instance.uv.xy, instance.uv.zw));
    return out;
}

const OUTLINE_PIXELS: f32 = 1.5;

const OUTLINE_OFFSETS: array<vec2<f32>, 8> = array<vec2<f32>, 8>(
    vec2<f32>(-1.0, -1.0), vec2<f32>(0.0, -1.0), vec2<f32>(1.0, -1.0),
    vec2<f32>(-1.0, 0.0), vec2<f32>(1.0, 0.0),
    vec2<f32>(-1.0, 1.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 1.0),
);

fn outlined(color: vec4<f32>, in: VertexOut, uv_per_pixel: vec2<f32>) -> vec4<f32> {
    if (in.outline.a <= 0.0 || color.a <= 0.1) {
        return color;
    }
    let center = textureSampleLevel(image, image_sampler, in.uv, 0.0).a;
    for (var i = 0; i < 8; i = i + 1) {
        let point = in.uv + OUTLINE_OFFSETS[i] * uv_per_pixel * OUTLINE_PIXELS;
        let within = all(point >= in.bounds.xy) && all(point <= in.bounds.zw);
        let neighbor = select(0.0, textureSampleLevel(image, image_sampler, clamp(point, in.bounds.xy, in.bounds.zw), 0.0).a, within);
        if (abs(center - neighbor) > 0.1) {
            let factor = select(1.0, 1.0 - center, center < 1.0);
            let rim = vec3<f32>(srgb_to_linear(in.outline.rgb));
            return vec4<f32>(mix(color.rgb, rim, factor * in.outline.a), color.a);
        }
    }
    return color;
}

fn frosted(color: vec4<f32>, in: VertexOut) -> vec4<f32> {
    let strength = in.fx.x;
    if (in.fx.w > 0.5 || strength <= 0.0 || color.a <= 0.01) {
        return color;
    }
    let luma = dot(color.rgb, vec3<f32>(0.299, 0.587, 0.114));
    let ice = srgb_to_linear(vec3<f32>(0.42, 0.74, 1.0)) * (0.55 + luma * 1.4);
    let facet = abs(fract((in.world.x * 0.9 + in.world.y * 0.5) / 7.0) - 0.5);
    let shine = smoothstep(0.4, 0.5, facet) * 0.3;
    let glint = select(0.0, 0.7, noise(floor(in.world / 3.0)) > 0.985);
    let tinted = mix(color.rgb, ice, 0.7 * strength) + vec3<f32>(shine + glint) * strength;
    return vec4<f32>(tinted, color.a);
}

fn shined(color: vec4<f32>, in: VertexOut) -> vec4<f32> {
    let progress = in.fx.y;
    if (in.fx.w > 0.5 || progress <= 0.0 || color.a <= 0.01) {
        return color;
    }
    let band = in.local.x + in.local.y * 0.5;
    let center = mix(-0.25, 1.75, progress);
    let strength = smoothstep(0.2, 0.0, abs(band - center));
    return vec4<f32>(color.rgb + vec3<f32>(strength * 0.55), color.a);
}

fn underworld(in: VertexOut) -> vec4<f32> {
    let time = in.fx.x;
    let uv = in.local;
    let p = vec2<f32>(uv.x * 1.78, uv.y);
    let slow = map_fbm(p * 0.9 + vec2<f32>(time * 0.02, -time * 0.015), 1.3, 4.1);
    let wisps = map_fbm(p * 1.8 + vec2<f32>(-time * 0.04, time * 0.03), 8.2, 2.7);
    let fog = smoothstep(0.55, 1.2, slow * wisps * 1.8);
    let heat = pow(clamp(uv.y, 0.0, 1.0), 2.2);
    var rgb = vec3<f32>(0.0, 0.0, 0.0);
    rgb = rgb + fog * vec3<f32>(0.045, 0.018, 0.03) * (0.3 + heat);
    rgb = rgb + heat * vec3<f32>(0.10, 0.03, 0.01) * (0.4 + 0.6 * slow);
    let edge = length((uv - vec2<f32>(0.5)) * vec2<f32>(1.15, 1.0));
    rgb = rgb * mix(0.35, 1.0, smoothstep(0.95, 0.35, edge));
    return vec4<f32>(srgb_to_linear(rgb), 1.0);
}

fn value_noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    let a = noise(i);
    let b = noise(i + vec2<f32>(1.0, 0.0));
    let c = noise(i + vec2<f32>(0.0, 1.0));
    let d = noise(i + vec2<f32>(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

fn flow_fbm(p: vec2<f32>) -> f32 {
    return value_noise(p) * 0.55 + value_noise(p * 2.03 + vec2<f32>(7.1, 3.7)) * 0.3
        + value_noise(p * 4.1 + vec2<f32>(1.9, 8.3)) * 0.15;
}

const LIQUID_PIXEL: f32 = 4.0;

const WATER_SPEED: f32 = 0.8;

fn water_color(px: vec2<f32>, clock: f32) -> vec3<f32> {
    let t = clock * WATER_SPEED;
    let a = flow_fbm(px * 0.05 + vec2<f32>(t * 0.035, t * 0.012));
    let b = flow_fbm(px * 0.085 - vec2<f32>(t * 0.02, -t * 0.04) + vec2<f32>(11.0, 5.0));
    let f = a * 0.6 + b * 0.4;
    var rgb = vec3<f32>(46.0, 68.0, 32.0) / 255.0;
    if (f > 0.42) { rgb = vec3<f32>(62.0, 88.0, 38.0) / 255.0; }
    if (f > 0.54) { rgb = vec3<f32>(88.0, 112.0, 48.0) / 255.0; }
    if (f > 0.66) { rgb = vec3<f32>(116.0, 136.0, 62.0) / 255.0; }
    let scum = flow_fbm(px * 0.03 + vec2<f32>(40.0, 2.0));
    if (scum > 0.64 && f < 0.6) { rgb = vec3<f32>(92.0, 78.0, 44.0) / 255.0; }
    return rgb;
}

fn ice_color(px: vec2<f32>, t: f32) -> vec3<f32> {
    let a = flow_fbm(px * 0.04 + vec2<f32>(3.0, 9.0));
    var rgb = vec3<f32>(118.0, 172.0, 214.0) / 255.0;
    if (a > 0.46) { rgb = vec3<f32>(132.0, 184.0, 224.0) / 255.0; }
    if (a > 0.62) { rgb = vec3<f32>(152.0, 200.0, 234.0) / 255.0; }
    let crack = abs(flow_fbm(px * 0.11 + vec2<f32>(17.0, 4.0)) - 0.5);
    if (crack < 0.012) { rgb = vec3<f32>(206.0, 232.0, 250.0) / 255.0; }
    if (noise(px + floor(t * 3.0)) > 0.9985) { rgb = vec3<f32>(255.0); }
    return rgb;
}

fn lava_color(px: vec2<f32>, t: f32) -> vec3<f32> {
    let a = flow_fbm(px * 0.06 + vec2<f32>(t * 0.05, -t * 0.03));
    let b = flow_fbm(px * 0.11 + vec2<f32>(-t * 0.04, t * 0.06) + vec2<f32>(5.0, 13.0));
    let f = a * 0.55 + b * 0.45;
    var rgb = vec3<f32>(150.0, 40.0, 22.0) / 255.0;
    if (f > 0.40) { rgb = vec3<f32>(200.0, 66.0, 26.0) / 255.0; }
    if (f > 0.52) { rgb = vec3<f32>(236.0, 112.0, 34.0) / 255.0; }
    if (f > 0.62) { rgb = vec3<f32>(255.0, 168.0, 56.0) / 255.0; }
    let ridge = abs(a - 0.5);
    if (ridge < 0.025) { rgb = vec3<f32>(255.0, 226.0, 110.0) / 255.0; }
    if (f < 0.3) { rgb = vec3<f32>(74.0, 24.0, 20.0) / 255.0; }
    return rgb;
}

fn bog_color(px: vec2<f32>, t: f32) -> vec3<f32> {
    let a = flow_fbm(px * 0.05 + vec2<f32>(t * 0.016, t * 0.008) + vec2<f32>(21.0, 6.0));
    let b = flow_fbm(px * 0.11 + vec2<f32>(-t * 0.008, t * 0.012) + vec2<f32>(3.0, 18.0));
    let f = a * 0.65 + b * 0.35;
    var rgb = vec3<f32>(52.0, 42.0, 28.0) / 255.0;
    if (f > 0.40) { rgb = vec3<f32>(66.0, 54.0, 34.0) / 255.0; }
    if (f > 0.52) { rgb = vec3<f32>(86.0, 70.0, 42.0) / 255.0; }
    if (f > 0.64) { rgb = vec3<f32>(84.0, 112.0, 52.0) / 255.0; }
    let bubble_cycle = floor(t * 0.4 + noise(floor(px / 14.0)) * 7.0);
    let bubble = noise(floor(px / 14.0) + vec2<f32>(bubble_cycle * 5.3, 1.7));
    let local = fract(px / 14.0) - vec2<f32>(0.5);
    if (bubble > 0.9 && length(local) < 0.16) { rgb = vec3<f32>(104.0, 92.0, 56.0) / 255.0; }
    return rgb;
}

// Sand: wind ripples running across it, bent by the dunes and creeping slowly along.
fn sand_color(px: vec2<f32>, t: f32) -> vec3<f32> {
    let dune = flow_fbm(px * 0.035 + vec2<f32>(9.0, 2.0));
    let bend = flow_fbm(px * 0.08 + vec2<f32>(3.0, 14.0));
    let ripple = fract(px.y * 0.13 + px.x * 0.03 + dune * 2.2 + bend * 0.8 - t * 0.12);
    var rgb = vec3<f32>(164.0, 132.0, 82.0) / 255.0;
    if (dune > 0.56) { rgb = vec3<f32>(176.0, 144.0, 90.0) / 255.0; }
    if (dune < 0.38) { rgb = vec3<f32>(150.0, 120.0, 74.0) / 255.0; }
    let patchy = noise(floor(px / 3.0)) > 0.35;
    if (ripple < 0.08 && patchy) { rgb = rgb + vec3<f32>(18.0, 16.0, 10.0) / 255.0; }
    else if (ripple < 0.16 && patchy) { rgb = rgb - vec3<f32>(16.0, 14.0, 10.0) / 255.0; }
    if (noise(px + floor(t * 2.0) * 0.37) > 0.998) { rgb = vec3<f32>(226.0, 200.0, 140.0) / 255.0; }
    return rgb;
}

// Blood: thick and dark, slowly stirring, a sheen of light sliding over it and clotted patches.
fn blood_color(px: vec2<f32>, t: f32) -> vec3<f32> {
    let a = flow_fbm(px * 0.05 + vec2<f32>(t * 0.012, -t * 0.008) + vec2<f32>(31.0, 4.0));
    let b = flow_fbm(px * 0.1 + vec2<f32>(-t * 0.01, t * 0.006) + vec2<f32>(2.0, 23.0));
    let f = a * 0.6 + b * 0.4;
    var rgb = vec3<f32>(48.0, 8.0, 12.0) / 255.0;
    if (f > 0.42) { rgb = vec3<f32>(62.0, 10.0, 14.0) / 255.0; }
    if (f > 0.56) { rgb = vec3<f32>(78.0, 14.0, 18.0) / 255.0; }
    let clot = flow_fbm(px * 0.04 + vec2<f32>(50.0, 9.0));
    if (clot > 0.66) { rgb = vec3<f32>(34.0, 6.0, 10.0) / 255.0; }
    let sheen = abs(fract(px.x * 0.02 + px.y * 0.035 + a * 1.5 - t * 0.04) - 0.5);
    if (sheen < 0.02 && f > 0.45) { rgb = vec3<f32>(120.0, 40.0, 44.0) / 255.0; }
    return rgb;
}

fn liquid_color(kind: f32, world: vec2<f32>, t: f32) -> vec3<f32> {
    let px = floor(world / LIQUID_PIXEL);
    if (kind > 5.5) { return blood_color(px, t); }
    if (kind > 4.5) { return sand_color(px, t); }
    if (kind > 3.5) { return bog_color(px, t); }
    if (kind > 2.5) { return lava_color(px, t); }
    if (kind > 1.5) { return ice_color(px, t); }
    return water_color(px, t);
}

fn liquid(in: VertexOut) -> vec4<f32> {
    let texel = textureSample(image, image_sampler, in.uv);
    if (texel.a < 0.05) {
        return vec4<f32>(0.0);
    }
    var rgb = texel.rgb * in.color.rgb;
    if (texel.a < 0.995) {
        rgb = srgb_to_linear(liquid_color(in.fx.y, in.world, in.fx.x)) * in.color.rgb;
    }
    if (in.lit < 0.5) {
        return vec4<f32>(rgb, 1.0);
    }
    let screen = (in.world - globals.camera) / globals.size;
    var light = textureSampleLevel(lightmap, lightmap_sampler, screen, 0.0).rgb;
    light = light * (1.0 + noise(in.position.xy) * NOISE_INTENSITY);
    let shaded = linear_to_srgb(rgb) * light;
    return vec4<f32>(srgb_to_linear(min(shaded, vec3<f32>(1.0))), 1.0);
}

const SHADOW_TINT: vec3<f32> = vec3<f32>(0.12, 0.10, 0.20);

fn shadow_solid(uv: vec2<f32>, in: VertexOut) -> bool {
    let within = all(uv >= in.bounds.xy) && all(uv <= in.bounds.zw);
    return within && textureSampleLevel(image, image_sampler, uv, 0.0).a > 0.5;
}

fn shadow(in: VertexOut, uv_per_pixel: vec2<f32>) -> vec4<f32> {
    let texel = textureSampleLevel(image, image_sampler, in.uv, 0.0);
    if (texel.a < 0.5) {
        return vec4<f32>(0.0);
    }
    let time = in.fx.x;
    let step = 1.0 / vec2<f32>(textureDimensions(image));
    let cell = floor(in.uv / step);
    let smoke = value_noise(cell * 0.22 + vec2<f32>(time * 0.9, -time * 1.4));
    let swirl = value_noise(cell * 0.5 - vec2<f32>(time * 1.7, time * 0.6) + vec2<f32>(13.0, 7.0));
    var alpha = mix(0.3, 0.6, smoke * 0.7 + swirl * 0.3);
    var edge = 0.0;
    if (!shadow_solid(in.uv + vec2<f32>(step.x, 0.0), in)) { edge = edge + 1.0; }
    if (!shadow_solid(in.uv - vec2<f32>(step.x, 0.0), in)) { edge = edge + 1.0; }
    if (!shadow_solid(in.uv + vec2<f32>(0.0, step.y), in)) { edge = edge + 1.0; }
    if (!shadow_solid(in.uv - vec2<f32>(0.0, step.y), in)) { edge = edge + 1.0; }
    let flicker = noise(cell + vec2<f32>(floor(time * 10.0) * 3.1, 0.7));
    if (edge > 0.5) {
        if (flicker < 0.25 + edge * 0.12) {
            return vec4<f32>(0.0);
        }
        alpha = alpha * 0.6;
    } else if (flicker > 0.97) {
        alpha = alpha * 0.5;
    }
    let depth = 0.75 + 0.5 * swirl;
    let rgb = srgb_to_linear(SHADOW_TINT * depth);
    let rimmed = outlined(vec4<f32>(rgb, alpha), in, uv_per_pixel);
    return vec4<f32>(rimmed.rgb, rimmed.a * in.color.a);
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    if (in.fx.w > 3.5 && in.fx.w < 4.5) {
        return liquid(in);
    }
    let uv_per_pixel = max(abs(dpdx(in.uv)), abs(dpdy(in.uv)));
    let albedo = textureSample(image, image_sampler, in.uv) * in.color;
    if (in.fx.w > 4.5) {
        return shadow(in, uv_per_pixel);
    }
    if (in.fx.w > 2.5) {
        return underworld(in);
    }
    if (in.fx.w > 0.5) {
        return map_room(in, textureSample(image, image_sampler, in.uv));
    }
    if (in.lit < 0.5) {
        return shined(frosted(outlined(albedo, in, uv_per_pixel), in), in);
    }
    let screen = (in.world - globals.camera) / globals.size;
    var light = textureSampleLevel(lightmap, lightmap_sampler, screen, 0.0).rgb;
    light = light * (1.0 + noise(in.position.xy) * NOISE_INTENSITY);
    let shaded = linear_to_srgb(albedo.rgb) * light;
    return shined(
        frosted(
            outlined(vec4<f32>(srgb_to_linear(min(shaded, vec3<f32>(1.0))), albedo.a), in, uv_per_pixel),
            in,
        ),
        in,
    );
}
