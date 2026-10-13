const MAX_LIGHTS: u32 = 64u;
const AMBIENT_LIGHT: f32 = 0.6;
const MIN_CONTRIBUTION: f32 = 0.01;
const MAX_STEPS: i32 = 48;
const SOFTNESS: f32 = 8.0;

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
    grid_columns: u32,
    grid_rows: u32,
    tile_size: f32,
    lights: array<Light, 64>,
};

@group(0) @binding(0) var<uniform> globals: Globals;
@group(0) @binding(1) var occluders: texture_2d<f32>;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

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

fn is_wall(cell: vec2<i32>) -> bool {
    let inside = cell.x >= 0 && cell.y >= 0
        && cell.x < i32(globals.grid_columns) && cell.y < i32(globals.grid_rows);
    if (!inside) {
        return true;
    }
    return textureLoad(occluders, cell, 0).r > 0.5;
}

// Walks the tiles between a point and a light. The light is blocked only when the ray leaves
// open floor, crosses a wall and comes out on open floor again: a wall tile lights its own face,
// and a torch set into a wall still shines out of it.
fn visibility(origin: vec2<f32>, light: vec2<f32>) -> f32 {
    let tile = globals.tile_size;
    var cell = vec2<i32>(floor(origin / tile));
    let end = vec2<i32>(floor(light / tile));
    let delta = light - origin;
    let direction = sign(delta);
    let stride = vec2<i32>(direction);
    let moving = abs(delta) > vec2<f32>(1e-4);
    let boundary = (vec2<f32>(cell) + max(direction, vec2<f32>(0.0))) * tile;
    var next = select(vec2<f32>(1e9), (boundary - origin) / delta, moving);
    let span = select(vec2<f32>(1e9), vec2<f32>(tile) / abs(delta), moving);
    var seen_open = !is_wall(cell);
    var behind_wall = false;
    for (var i = 0; i < MAX_STEPS; i = i + 1) {
        if (all(cell == end)) {
            break;
        }
        if (next.x < next.y) {
            cell.x = cell.x + stride.x;
            next.x = next.x + span.x;
        } else {
            cell.y = cell.y + stride.y;
            next.y = next.y + span.y;
        }
        if (is_wall(cell)) {
            behind_wall = behind_wall || seen_open;
        } else {
            if (behind_wall) {
                return 0.0;
            }
            seen_open = true;
        }
    }
    return 1.0;
}

fn soft_visibility(world: vec2<f32>, light: vec2<f32>) -> f32 {
    let offset = light - world;
    let length_squared = dot(offset, offset);
    if (length_squared < 1.0) {
        return 1.0;
    }
    let side = vec2<f32>(-offset.y, offset.x) * inverseSqrt(length_squared) * SOFTNESS;
    return (visibility(world, light)
        + visibility(world, light + side)
        + visibility(world, light - side)) / 3.0;
}

@fragment
fn fs_light(in: VertexOut) -> @location(0) vec4<f32> {
    let world = globals.camera + in.uv * globals.size;
    let shadows = globals.grid_columns > 0u;
    var light = vec3<f32>(AMBIENT_LIGHT);
    for (var i = 0u; i < min(globals.light_count, MAX_LIGHTS); i = i + 1u) {
        let source = globals.lights[i];
        let distance = length(source.position - world) / source.radius;
        let attenuation = clamp(1.0 / (1.0 + distance * distance), 0.0, 1.0);
        let amount = attenuation * source.intensity;
        if (amount < MIN_CONTRIBUTION) {
            continue;
        }
        var visible = 1.0;
        if (shadows) {
            visible = soft_visibility(world, source.position);
        }
        light = light + amount * visible * source.color.rgb;
    }
    return vec4<f32>(light, 1.0);
}
