const MAX_LIGHTS: u32 = 64u;
const AMBIENT_LIGHT: f32 = 0.6;
const NOISE_INTENSITY: f32 = 0.10;

struct Light {
    position: vec2<f32>,
    radius: f32,
    intensity: f32,
    color: vec4<f32>,
};

struct Globals {
    size: vec2<f32>,
    camera: vec2<f32>,
    light_count: u32,
    lights: array<Light, 64>,
};

@group(0) @binding(0) var<uniform> globals: Globals;
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
    let offset = corner * instance.size;
    let rotated = vec2<f32>(
        offset.x * cos(angle) - offset.y * sin(angle),
        offset.x * sin(angle) + offset.y * cos(angle),
    );
    let pixel = instance.center + rotated;
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
        let point = clamp(in.uv + OUTLINE_OFFSETS[i] * uv_per_pixel * OUTLINE_PIXELS, in.bounds.xy, in.bounds.zw);
        let neighbor = textureSampleLevel(image, image_sampler, point, 0.0).a;
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

fn sky_hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
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
    for (var layer = 0; layer < 3; layer = layer + 1) {
        let fl = f32(layer);
        let scale = 16.0 + fl * 12.0;
        let rise = time * (0.05 + fl * 0.035);
        let q = vec2<f32>(p.x * scale, (uv.y + rise) * scale * 0.9);
        let cell = floor(q);
        let h = sky_hash(cell + vec2<f32>(fl * 17.0, fl * 5.0));
        let inside = fract(q) - vec2<f32>(0.5);
        let sway = sin(time * 1.7 + h * 40.0) * 0.18;
        let d = length(inside - vec2<f32>(sway, 0.0));
        let twinkle = 0.6 + 0.4 * sin(time * 4.0 + h * 30.0);
        let spark = select(0.0, 1.0, h > 0.97) * smoothstep(0.16, 0.0, d) * twinkle;
        rgb = rgb + spark * vec3<f32>(1.0, 0.5, 0.14) * (0.6 + heat);
    }
    let edge = length((uv - vec2<f32>(0.5)) * vec2<f32>(1.15, 1.0));
    rgb = rgb * mix(0.35, 1.0, smoothstep(0.95, 0.35, edge));
    return vec4<f32>(srgb_to_linear(rgb), 1.0);
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let uv_per_pixel = max(abs(dpdx(in.uv)), abs(dpdy(in.uv)));
    let albedo = textureSample(image, image_sampler, in.uv) * in.color;
    if (in.fx.w > 2.5) {
        return underworld(in);
    }
    if (in.fx.w > 0.5) {
        return map_room(in, textureSample(image, image_sampler, in.uv));
    }
    if (in.lit < 0.5) {
        return shined(frosted(outlined(albedo, in, uv_per_pixel), in), in);
    }
    var light = vec3<f32>(AMBIENT_LIGHT);
    for (var i = 0u; i < globals.light_count; i = i + 1u) {
        let source = globals.lights[i];
        let distance = length(source.position - in.world) / source.radius;
        let attenuation = clamp(1.0 / (1.0 + distance * distance), 0.0, 1.0);
        light = light + attenuation * source.intensity * source.color.rgb;
    }
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
