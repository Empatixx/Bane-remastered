struct Params {
    texel: vec2<f32>,
    blur: f32,
    darkness: f32,
    vignette: f32,
    time: f32,
    time_based: f32,
    time_slow: f32,
    clock: f32,
    padding_0: f32,
    padding_1: f32,
    padding_2: f32,
};

@group(0) @binding(0) var<uniform> params: Params;
@group(1) @binding(0) var source: texture_2d<f32>;
@group(1) @binding(1) var source_sampler: sampler;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
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

@vertex
fn vs_fullscreen(@builtin(vertex_index) index: u32) -> VertexOut {
    var corners = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    let corner = corners[index];
    var out: VertexOut;
    out.position = vec4<f32>(corner, 0.0, 1.0);
    out.uv = vec2<f32>((corner.x + 1.0) / 2.0, 1.0 - (corner.y + 1.0) / 2.0);
    return out;
}

@fragment
fn fs_blit(in: VertexOut) -> @location(0) vec4<f32> {
    return textureSample(source, source_sampler, in.uv);
}

@fragment
fn fs_vignette(in: VertexOut) -> @location(0) vec4<f32> {
    let color = textureSample(source, source_sampler, in.uv);
    let dist = distance(in.uv, vec2<f32>(0.5));
    let vignette = smoothstep(0.4, 0.6, dist);
    var intensity = params.vignette * 0.5;
    if (params.time_based > 0.5) {
        let pulse = fract(params.time * 0.5);
        intensity = exp(-6.0 * (pulse - 0.1) * (pulse - 0.1)) * 0.5;
    }
    let mixed = mix(linear_to_srgb(color.rgb), vec3<f32>(1.0, 0.0, 0.0), intensity * vignette);
    return vec4<f32>(srgb_to_linear(mixed), color.a);
}

fn hash_cell(cell: vec2<f32>) -> f32 {
    let x = bitcast<u32>(i32(cell.x)) * 1597334677u;
    let y = bitcast<u32>(i32(cell.y)) * 3812015801u;
    var n = x ^ y;
    n = (n ^ (n >> 16u)) * 2246822519u;
    n = n ^ (n >> 13u);
    return f32(n) / 4294967295.0;
}

fn ash_layer(uv: vec2<f32>, cells: vec2<f32>, drift: vec2<f32>, density: f32) -> vec2<f32> {
    let cell = floor(uv * cells + drift);
    let roll = hash_cell(cell);
    if (roll < 1.0 - density) {
        return vec2<f32>(0.0);
    }
    let shade = hash_cell(cell + vec2<f32>(17.0, 41.0));
    let flicker = 0.6 + 0.4 * sin(params.clock * (1.5 + shade * 3.0) + roll * 40.0);
    return vec2<f32>(flicker, shade);
}

@fragment
fn fs_slow(in: VertexOut) -> @location(0) vec4<f32> {
    let strength = smoothstep(0.0, 1.0, params.time_slow);
    let t = params.clock;
    let aspect = params.texel.y / params.texel.x;
    let offset = (in.uv - vec2<f32>(0.5)) * vec2<f32>(aspect, 1.0);
    let radius = length(offset);
    let pulse = 0.55 + 0.45 * sin(t * 1.8);
    let wave = sin(radius * 38.0 - t * 3.5) * pulse * smoothstep(0.04, 0.45, radius);
    let direction = offset / max(radius, 0.0001) / vec2<f32>(aspect, 1.0);
    let uv = in.uv + direction * wave * 0.0045 * strength;
    let color = textureSample(source, source_sampler, uv);
    var rgb = linear_to_srgb(color.rgb);

    let grey = dot(rgb, vec3<f32>(0.299, 0.587, 0.114));
    let ash = vec3<f32>(grey * 1.08, grey * 0.94, grey * 0.74);
    rgb = mix(rgb, ash, 0.7 * strength);
    rgb = rgb * (1.0 + 0.1 * wave * strength);

    let edge = smoothstep(0.3, 0.95, radius);
    rgb = rgb * mix(vec3<f32>(1.0), vec3<f32>(0.5, 0.3, 0.12), edge * strength);

    let fine = vec2<f32>(480.0, 480.0 / aspect);
    let sway = sin(t * 0.7) * 3.0;
    let far = ash_layer(in.uv, fine, vec2<f32>(sway - t * 4.0, -t * 9.0), 0.004);
    let near = ash_layer(in.uv, fine * 0.5, vec2<f32>(sway * 0.5 - t * 3.0, -t * 7.5), 0.003);
    let light_ash = vec3<f32>(0.86, 0.8, 0.7);
    let dark_ash = vec3<f32>(0.3, 0.26, 0.22);
    let far_color = mix(dark_ash, light_ash, step(0.5, far.y));
    let near_color = mix(dark_ash, light_ash, step(0.4, near.y));
    rgb = mix(rgb, far_color, far.x * 0.75 * strength);
    rgb = mix(rgb, near_color, near.x * 0.9 * strength);

    return vec4<f32>(srgb_to_linear(clamp(rgb, vec3<f32>(0.0), vec3<f32>(1.0))), color.a);
}

fn blur(uv: vec2<f32>, direction: vec2<f32>) -> vec4<f32> {
    var kernel = array<f32, 9>(0.0093, 0.028002, 0.065984, 0.121703, 0.175713, 0.121703, 0.065984, 0.028002, 0.0093);
    let step = direction * params.texel * params.blur;
    var color = vec4<f32>(0.0);
    var weight_sum = 0.0;
    for (var i = 0; i < 9; i = i + 1) {
        let weight = kernel[i];
        color = color + textureSample(source, source_sampler, uv + step * (f32(i) - 4.0)) * weight;
        weight_sum = weight_sum + weight;
    }
    return color / weight_sum;
}

@fragment
fn fs_blur_horizontal(in: VertexOut) -> @location(0) vec4<f32> {
    return blur(in.uv, vec2<f32>(1.0, 0.0));
}

@fragment
fn fs_blur_vertical(in: VertexOut) -> @location(0) vec4<f32> {
    let color = blur(in.uv, vec2<f32>(0.0, 1.0));
    let darkened = srgb_to_linear(linear_to_srgb(color.rgb) * params.darkness);
    return vec4<f32>(darkened, color.a);
}
