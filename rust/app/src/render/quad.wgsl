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
    @location(4) lit: f32,
};

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) lit: f32,
    @location(3) world: vec2<f32>,
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
    let pixel = instance.center + corner * instance.size;
    let ndc = vec2<f32>(
        pixel.x / globals.size.x * 2.0 - 1.0,
        1.0 - pixel.y / globals.size.y * 2.0,
    );
    var out: VertexOut;
    out.position = vec4<f32>(ndc, 0.0, 1.0);
    out.color = vec4<f32>(srgb_to_linear(instance.color.rgb), instance.color.a);
    out.uv = mix(instance.uv.xy, instance.uv.zw, corner + vec2<f32>(0.5, 0.5));
    out.lit = instance.lit;
    out.world = pixel + globals.camera;
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let albedo = textureSample(image, image_sampler, in.uv) * in.color;
    if (in.lit < 0.5) {
        return albedo;
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
    return vec4<f32>(srgb_to_linear(min(shaded, vec3<f32>(1.0))), albedo.a);
}
