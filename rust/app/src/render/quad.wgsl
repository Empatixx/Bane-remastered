struct Screen {
    size: vec2<f32>,
};

@group(0) @binding(0) var<uniform> screen: Screen;
@group(1) @binding(0) var image: texture_2d<f32>;
@group(1) @binding(1) var image_sampler: sampler;

struct Instance {
    @location(0) center: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

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
        pixel.x / screen.size.x * 2.0 - 1.0,
        1.0 - pixel.y / screen.size.y * 2.0,
    );
    var out: VertexOut;
    out.position = vec4<f32>(ndc, 0.0, 1.0);
    out.color = instance.color;
    out.uv = corner + vec2<f32>(0.5, 0.5);
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return textureSample(image, image_sampler, in.uv) * in.color;
}
