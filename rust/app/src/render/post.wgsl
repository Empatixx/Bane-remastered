struct Params {
    texel: vec2<f32>,
    blur: f32,
    darkness: f32,
    vignette: f32,
    time: f32,
    time_based: f32,
    padding: f32,
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
