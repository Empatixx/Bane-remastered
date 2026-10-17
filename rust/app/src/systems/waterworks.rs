//! Brings the waterworks to life (see [`crate::dungeon::Floater`]): water pours out of the
//! sewer's pipes and splashes into its pools, rubbish drifts and bobs on them, barrels float
//! about and get shoved along by shots, and footbridges cross them. The lava floor gets the
//! same in its own colours (lava running out of glowing cracks, cooled crusts afloat, stepping
//! stones), and so does the moss floor's bog (murky water seeping between roots, lily pads and a
//! frog that leaps now and then, fallen logs). The temple's sand pours out of cracks in its walls
//! the same way, into heaps in its sand.

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Tex, Textures, tex};
use crate::components::{
    Animation, Body, BodyOffset, BreakKind, Bullet, Clip, Color, Expiring, Health,
    Layer, Light, Lit, Position, Size, Sprite,
};
use crate::dungeon::{Bridge, Debris, Dungeon, Floater, FloatingBarrel, Mark, Surface, Surfaces};
use crate::physics::Physics;
use crate::prefabs;
use crate::tilemap::TILE_SIZE;

const PIPE_COLUMNS: usize = 2;
const STREAM_COLUMNS: usize = 4;
const STREAM_ROWS: usize = 2;
const DEBRIS_COLUMNS: usize = 7;
/// The ring round anything afloat; on the ice sheet, the drift of snow.
const RING_CELL: usize = 6;
/// The ice sheet's two cracks.
const CRACK_CELL: usize = 4;
/// What lies under the ice shows pale through it.
const UNDER_ICE: Vec4 = Vec4::new(0.8, 0.92, 1.0, 0.6);
const RAT_FRAMES: usize = 2;
/// A frog leaps about every three seconds, for a tenth of that.
const FROG_RATE: f32 = 0.33;
const FROG_LEAP: f32 = 0.1;
const BRIDGE_COLUMNS: usize = 3;
const BRIDGE_ROWS: usize = 2;

/// The pipe's mouth sits this far down its wall tile; the water falls from there to the pool.
const PIPE_AT: f32 = 0.5;
const STREAM_FROM: f32 = 0.56;
const STREAM_TO: f32 = 1.3;
const STREAM_WIDTH: f32 = TILE_SIZE;
const SPLASH_RIPPLE_EVERY: f32 = 0.45;
const RIPPLE_SIZE: Vec2 = Vec2::new(96.0, 96.0);
const RIPPLE_FRAMES: usize = 6;

const FLOOR_LAYER: i32 = -1;
const UNDER_LAYER: i32 = -2;
const BRIDGE_SHADOW: Vec2 = Vec2::new(4.0, 10.0);
const BRIDGE_SHADOW_ALPHA: f32 = 0.35;

const DRIFT_SPEED: f32 = 14.0;
const BARREL_DRIFT: f32 = 10.0;
const SHOVE_SPEED: f32 = 170.0;
/// How quickly a shoved barrel slows back down to drifting.
const SETTLE_RATE: f32 = 1.2;
const LOOK_AHEAD: f32 = 26.0;
const BOB: f32 = 1.5;
const BOB_RATE: f32 = 1.7;
/// The ring is drawn in the bottom of its cell; a barrel sits lower in the water than rubbish.
const RING_DROP: f32 = -6.0;
const BARREL_RING_DROP: f32 = 14.0;

/// The pictures for one liquid's waterworks, all laid out alike.
#[derive(Debug, Clone, Copy)]
struct Kit {
    liquid: Surface,
    pipe: Tex,
    stream: Tex,
    debris: Tex,
    bridge: Tex,
    /// What the splash's rings look like on the pool; ice takes none.
    ripple: Option<Vec4>,
}

impl Kit {
    fn of(liquid: Surface) -> Option<Self> {
        match liquid {
            Surface::Water => Some(Self {
                liquid,
                pipe: tex::SEWER_PIPE,
                stream: tex::SEWER_STREAM,
                debris: tex::SEWER_DEBRIS,
                bridge: tex::SEWER_BRIDGE,
                ripple: Some(Vec4::new(1.0, 1.0, 1.0, 0.8)),
            }),
            Surface::Lava => Some(Self {
                liquid,
                pipe: tex::LAVA_CRACK,
                stream: tex::LAVA_FALL,
                debris: tex::LAVA_DEBRIS,
                bridge: tex::LAVA_BRIDGE,
                ripple: Some(Vec4::new(1.0, 0.7, 0.3, 0.7)),
            }),
            Surface::Bog => Some(Self {
                liquid,
                pipe: tex::BOG_SEEP,
                stream: tex::BOG_TRICKLE,
                debris: tex::BOG_DEBRIS,
                bridge: tex::BOG_LOG,
                ripple: Some(Vec4::new(0.55, 0.45, 0.3, 0.75)),
            }),
            // Sand pours out of cracks into the temple's sand; nothing floats on it or bridges it.
            Surface::Sand => Some(Self {
                liquid,
                pipe: tex::SAND_CRACK,
                stream: tex::SAND_FALL,
                debris: tex::SAND_FALL,
                bridge: tex::SAND_FALL,
                ripple: None,
            }),
            // Nothing pours into the blood, floats on it or crosses it.
            Surface::Blood => None,
            Surface::Ice => Some(Self {
                liquid,
                pipe: tex::ICE_SPOUT,
                stream: tex::ICE_FALL,
                debris: tex::ICE_DEBRIS,
                // Never used: nothing bridges the ice.
                bridge: tex::ICE_FALL,
                ripple: None,
            }),
        }
    }

    fn glows(&self) -> bool {
        self.liquid == Surface::Lava
    }
}

const LAVA_GLOW: (f32, f32, f32) = (255.0, 120.0, 30.0);

/// Something adrift. Rubbish moves itself; a barrel's body is moved by the physics.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Adrift {
    liquid: Surface,
    at: Vec2,
    velocity: Vec2,
    phase: f32,
    ring: Entity,
    /// What kind of rubbish (none for a barrel), and which of its two pictures shows: the rat
    /// wags its tail, the frog now and then leaps.
    debris: Option<Debris>,
    shown: usize,
    /// A barrel: its body's usual offset, which the bobbing moves about.
    offset: Option<Vec2>,
}

/// The ruffled water round something afloat.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Ring {
    owner: Entity,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Splash {
    at: Vec2,
    ripple: Option<Vec4>,
    timer: f32,
}

fn tile_center(tile: (usize, usize)) -> Vec2 {
    (Vec2::new(tile.0 as f32, tile.1 as f32) + 0.5) * TILE_SIZE
}

/// Liquid something can float on: not under a bridge.
fn afloat(surfaces: &Surfaces, liquid: Surface, point: Vec2) -> bool {
    surfaces.at_point(point) == Some(liquid)
}

pub(crate) fn spawn_waterworks(
    world: &mut World,
    physics: &mut Physics,
    dungeon: &Dungeon,
    textures: &Textures,
) {
    let Some(kit) = dungeon.theme.surface().and_then(Kit::of) else {
        return;
    };
    for room in &dungeon.rooms {
        let works = &room.waterworks;
        for &(column, row) in &works.pipes {
            spawn_pipe(world, &kit, (column, row), textures);
        }
        for bridge in &works.bridges {
            spawn_bridge(world, &kit, bridge, textures);
        }
        for &(mark, tile) in &works.marks {
            spawn_mark(world, &kit, mark, tile, textures);
        }
        for (index, &(floater, tile)) in works.floaters.iter().enumerate() {
            let phase = (tile.0 * 7 + tile.1 * 13 + index * 5) as f32 * 0.61;
            let heading = Vec2::from_angle(phase);
            match floater {
                Floater::Debris(Debris::Frozen(variant)) => {
                    spawn_frozen(world, &kit, variant, tile_center(tile), textures);
                }
                Floater::Debris(debris) => {
                    spawn_debris(world, &kit, debris, tile_center(tile), heading, phase, textures);
                }
                Floater::Barrel(kind) => {
                    let at = tile_center(tile);
                    spawn_barrel(world, physics, &kit, kind, at, heading, phase, textures);
                }
            }
        }
    }
}

/// Something frozen under the ice, seen through it.
fn spawn_frozen(world: &mut World, kit: &Kit, variant: usize, at: Vec2, textures: &Textures) {
    world.spawn((
        Position(at),
        Size(Vec2::splat(TILE_SIZE)),
        Color(UNDER_ICE),
        Sprite::cell(textures.get(kit.debris), variant, 0, DEBRIS_COLUMNS, 1),
        Layer(UNDER_LAYER),
        Lit,
    ));
}

fn spawn_mark(world: &mut World, kit: &Kit, mark: Mark, tile: (usize, usize), textures: &Textures) {
    let (cell, colour, layer) = match mark {
        Mark::Crack(variant) => (CRACK_CELL + variant, Vec4::new(1.0, 1.0, 1.0, 0.8), UNDER_LAYER),
        Mark::Drift => (RING_CELL, Vec4::ONE, FLOOR_LAYER),
    };
    world.spawn((
        Position(tile_center(tile)),
        Size(Vec2::splat(TILE_SIZE)),
        Color(colour),
        Sprite::cell(textures.get(kit.debris), cell, 0, DEBRIS_COLUMNS, 1),
        Layer(layer),
        Lit,
    ));
}

fn spawn_pipe(world: &mut World, kit: &Kit, (column, row): (usize, usize), textures: &Textures) {
    let x = (column as f32 + 0.5) * TILE_SIZE;
    let wall = row as f32 * TILE_SIZE;
    let pipe = world.spawn((
        Position(Vec2::new(x, wall + PIPE_AT * TILE_SIZE)),
        Size(Vec2::splat(TILE_SIZE)),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(kit.pipe), (column + row) % PIPE_COLUMNS, 0, PIPE_COLUMNS, 1),
        Layer(FLOOR_LAYER),
        Lit,
    ));
    if kit.glows() {
        let (r, g, b) = LAVA_GLOW;
        let _ = world.insert_one(pipe, Light::rgb(Vec2::ZERO, r, g, b, 70.0, 1.0));
    }
    let stream = |row: usize, frame_time: f32| {
        Animation::new(
            STREAM_COLUMNS,
            STREAM_ROWS,
            Clip {
                row,
                frames: STREAM_COLUMNS,
                frame_time,
                looping: true,
                flipped: false,
            },
        )
    };
    let (from, to) = (wall + STREAM_FROM * TILE_SIZE, wall + STREAM_TO * TILE_SIZE);
    world.spawn((
        Position(Vec2::new(x, (from + to) / 2.0)),
        Size(Vec2::new(STREAM_WIDTH, to - from)),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(kit.stream), 0, 0, STREAM_COLUMNS, STREAM_ROWS),
        stream(0, 0.07),
        Layer(FLOOR_LAYER),
        Lit,
    ));
    let splash = Vec2::new(x, to);
    let pool = world.spawn((
        Position(splash),
        Size(Vec2::splat(TILE_SIZE)),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(kit.stream), 0, 1, STREAM_COLUMNS, STREAM_ROWS),
        stream(1, 0.11),
        Splash {
            at: splash,
            ripple: kit.ripple,
            timer: 0.0,
        },
        Layer(FLOOR_LAYER),
        Lit,
    ));
    if kit.glows() {
        let (r, g, b) = LAVA_GLOW;
        let _ = world.insert_one(pool, Light::rgb(Vec2::ZERO, r, g, b, 110.0, 1.2));
    }
}

fn spawn_bridge(world: &mut World, kit: &Kit, bridge: &Bridge, textures: &Textures) {
    let texture = textures.get(kit.bridge);
    let row = usize::from(bridge.upright);
    let step = if bridge.upright { (0, 1) } else { (1, 0) };
    let before = (bridge.first.0 - step.0, bridge.first.1 - step.1);
    let after = (bridge.last.0 + step.0, bridge.last.1 + step.1);
    let tiles = std::iter::once((before, 0))
        .chain(bridge.tiles().map(|tile| (tile, 1)))
        .chain(std::iter::once((after, 2)));
    for (tile, cell) in tiles {
        let sprite = Sprite::cell(texture, cell, row, BRIDGE_COLUMNS, BRIDGE_ROWS);
        let at = tile_center(tile);
        if cell == 1 {
            world.spawn((
                Position(at + BRIDGE_SHADOW),
                Size(Vec2::splat(TILE_SIZE)),
                Color(Vec4::new(0.0, 0.0, 0.0, BRIDGE_SHADOW_ALPHA)),
                sprite,
                Layer(UNDER_LAYER),
            ));
        }
        world.spawn((
            Position(at),
            Size(Vec2::splat(TILE_SIZE)),
            Color(Vec4::ONE),
            sprite,
            Layer(FLOOR_LAYER),
            Lit,
        ));
    }
}

fn spawn_ring(world: &mut World, kit: &Kit, owner: Entity, at: Vec2, textures: &Textures) -> Entity {
    world.spawn((
        Ring { owner },
        Position(at + Vec2::new(0.0, RING_DROP)),
        Size(Vec2::splat(TILE_SIZE)),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(kit.debris), RING_CELL, 0, DEBRIS_COLUMNS, 1),
        Layer(UNDER_LAYER),
        Lit,
    ))
}

fn spawn_debris(
    world: &mut World,
    kit: &Kit,
    debris: Debris,
    at: Vec2,
    heading: Vec2,
    phase: f32,
    textures: &Textures,
) {
    let entity = world.spawn((
        Position(at),
        Size(Vec2::splat(TILE_SIZE)),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(kit.debris), debris.cell(), 0, DEBRIS_COLUMNS, 1),
        Layer(FLOOR_LAYER),
        Lit,
    ));
    let ring = spawn_ring(world, kit, entity, at, textures);
    let _ = world.insert_one(
        entity,
        Adrift {
            liquid: kit.liquid,
            at,
            velocity: heading * DRIFT_SPEED,
            phase,
            ring,
            debris: Some(debris),
            shown: 0,
            offset: None,
        },
    );
}

#[allow(clippy::too_many_arguments)]
fn spawn_barrel(
    world: &mut World,
    physics: &mut Physics,
    kit: &Kit,
    kind: FloatingBarrel,
    at: Vec2,
    heading: Vec2,
    phase: f32,
    textures: &Textures,
) {
    let kind = match kind {
        FloatingBarrel::Plain => BreakKind::Barrel,
        FloatingBarrel::Explosive => BreakKind::Explosive,
        FloatingBarrel::Poison => BreakKind::Poison,
    };
    let size = crate::objects::OBJECT_SIZE;
    let barrel = prefabs::breakable(world, physics, kind, at - size / 2.0, textures);
    let offset = world.get::<&BodyOffset>(barrel).map_or(Vec2::ZERO, |offset| offset.0);
    let ring = spawn_ring(world, kit, barrel, at, textures);
    let _ = world.insert_one(
        barrel,
        Adrift {
            liquid: kit.liquid,
            at,
            velocity: heading * BARREL_DRIFT,
            phase,
            ring,
            debris: None,
            shown: 0,
            offset: Some(offset),
        },
    );
}

pub(crate) fn tick_waterworks(
    world: &mut World,
    physics: &mut Physics,
    surfaces: &Surfaces,
    textures: &Textures,
    clock: f32,
    dt: f32,
) {
    let mut ripples = Vec::new();
    for splash in world.query_mut::<&mut Splash>() {
        splash.timer -= dt;
        if splash.timer <= 0.0 {
            splash.timer = SPLASH_RIPPLE_EVERY;
            if let Some(tint) = splash.ripple {
                ripples.push((splash.at, tint));
            }
        }
    }
    for (at, tint) in ripples {
        spawn_ripple(world, at, tint, textures);
    }
    let shots: Vec<(Vec2, Vec2, Vec2)> = world
        .query::<(&Position, &Size, &crate::components::Velocity)>()
        .with::<&Bullet>()
        .iter()
        .map(|(position, size, velocity)| (position.0, size.0, velocity.0))
        .collect();
    let drifting: Vec<Entity> = world.query::<Entity>().with::<&Adrift>().iter().collect();
    let mut splashes = Vec::new();
    for entity in drifting {
        let Ok(mut adrift) = world.get::<&Adrift>(entity).map(|adrift| *adrift) else {
            continue;
        };
        let bob = Vec2::new(0.0, (clock * BOB_RATE + adrift.phase).sin() * BOB);
        let body = world.get::<&Body>(entity).ok().map(|body| *body);
        let alive = world.get::<&Health>(entity).map_or(true, |health| health.current > 0)
            && world.get::<&crate::components::Broken>(entity).is_err();
        if let (Some(body), Some(offset)) = (body, adrift.offset) {
            // A barrel: the physics moves it, the water only steers it.
            let at = physics.position(body).unwrap_or(adrift.at);
            adrift.at = at;
            if alive && afloat(surfaces, adrift.liquid, at) {
                let shove = shots.iter().find_map(|(position, size, velocity)| {
                    let reach = (*size + crate::objects::OBJECT_SIZE) / 2.0;
                    ((*position - at).abs().cmple(reach).all()).then(|| velocity.normalize_or_zero())
                });
                let mut velocity = physics.velocity(body).unwrap_or(adrift.velocity);
                if let Some(direction) = shove {
                    velocity = direction * SHOVE_SPEED;
                } else {
                    let drift = velocity.normalize_or(adrift.velocity.normalize_or_zero()) * BARREL_DRIFT;
                    velocity += (drift - velocity) * (SETTLE_RATE * dt).min(1.0);
                }
                if !afloat(surfaces, adrift.liquid, at + velocity.normalize_or_zero() * LOOK_AHEAD) {
                    velocity = -velocity;
                }
                adrift.velocity = velocity;
                physics.set_velocity(body, velocity);
                if let Ok(mut body_offset) = world.get::<&mut BodyOffset>(entity) {
                    body_offset.0 = offset + bob;
                }
            }
        } else {
            let next = adrift.at + adrift.velocity * dt;
            if afloat(surfaces, adrift.liquid, next + adrift.velocity.normalize_or_zero() * LOOK_AHEAD) {
                adrift.at = next;
            } else {
                // Turn back from the bank, a little differently each time.
                adrift.phase += 1.3;
                let away = -adrift.velocity.normalize_or(Vec2::X);
                adrift.velocity = Vec2::from_angle(adrift.phase.sin()).rotate(away) * DRIFT_SPEED;
            }
            if let Ok(mut position) = world.get::<&mut Position>(entity) {
                position.0 = adrift.at + bob;
            }
            let frame = match adrift.debris {
                Some(Debris::RatRaft) => ((clock * 3.0 + adrift.phase) as usize) % RAT_FRAMES,
                Some(Debris::Frog) => usize::from((clock * FROG_RATE + adrift.phase).fract() < FROG_LEAP),
                _ => 0,
            };
            if let Some(debris) = adrift.debris.filter(|_| frame != adrift.shown) {
                if debris == Debris::Frog && frame == 1 {
                    if let Some(tint) = Kit::of(adrift.liquid).and_then(|kit| kit.ripple) {
                        splashes.push((adrift.at, tint));
                    }
                }
                if let Ok(mut sprite) = world.get::<&mut Sprite>(entity) {
                    sprite.uv = Sprite::cell_uv(debris.cell() + frame, 0, DEBRIS_COLUMNS, 1);
                }
                adrift.shown = frame;
            }
        }
        let drop = if adrift.offset.is_some() { BARREL_RING_DROP } else { RING_DROP };
        if let Ok(mut ring) = world.get::<&mut Position>(adrift.ring) {
            ring.0 = adrift.at + Vec2::new(0.0, drop);
        }
        let afloat = alive && afloat(surfaces, adrift.liquid, adrift.at);
        if let Ok(mut color) = world.get::<&mut Color>(adrift.ring) {
            color.0.w = if afloat { 1.0 } else { 0.0 };
        }
        if let Ok(mut state) = world.get::<&mut Adrift>(entity) {
            *state = adrift;
        }
    }
    for (at, tint) in splashes {
        spawn_ripple(world, at, tint, textures);
    }
    // Rings whose barrel has been blown up and gone.
    let orphans: Vec<Entity> = world
        .query::<(Entity, &Ring)>()
        .iter()
        .filter(|(_, ring)| !world.contains(ring.owner))
        .map(|(entity, _)| entity)
        .collect();
    for ring in orphans {
        let _ = world.despawn(ring);
    }
}

fn spawn_ripple(world: &mut World, at: Vec2, tint: Vec4, textures: &Textures) {
    let clip = Clip {
        row: 0,
        frames: RIPPLE_FRAMES,
        frame_time: 0.12,
        looping: false,
        flipped: false,
    };
    world.spawn((
        Expiring,
        Position(at),
        Size(RIPPLE_SIZE),
        Color(tint),
        Sprite::cell(textures.get(tex::RIPPLE), 0, 0, RIPPLE_FRAMES, 1),
        Animation::new(RIPPLE_FRAMES, 1, clip),
        Layer(FLOOR_LAYER),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::Theme;
    use crate::rng::Rng;

    fn sewer() -> Dungeon {
        (0..30)
            .map(|seed| Dungeon::generate_themed(&mut Rng::new(seed), Theme::Sewer).expect("a dungeon"))
            .find(|dungeon| {
                dungeon.rooms.iter().any(|room| {
                    room.waterworks
                        .floaters
                        .iter()
                        .any(|(floater, _)| matches!(floater, Floater::Debris(_)))
                })
            })
            .expect("a sewer with rubbish afloat")
    }

    #[test]
    fn rubbish_drifts_but_never_leaves_the_water() {
        let dungeon = sewer();
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        spawn_waterworks(&mut world, &mut physics, &dungeon, &textures);
        let rubbish = |world: &World| -> Vec<Vec2> {
            world
                .query::<&Adrift>()
                .iter()
                .filter(|adrift| adrift.offset.is_none())
                .map(|adrift| adrift.at)
                .collect()
        };
        let before = rubbish(&world);
        for step in 0..600 {
            tick_waterworks(&mut world, &mut physics, &dungeon.surfaces, &textures, step as f32 / 60.0, 1.0 / 60.0);
        }
        let after = rubbish(&world);
        assert!(before.iter().zip(&after).any(|(a, b)| a.distance(*b) > 5.0));
        assert!(after.iter().all(|at| dungeon.surfaces.at_point(*at) == Some(Surface::Water)));
    }

    #[test]
    fn pipes_pour_and_splash_and_bridges_get_their_ends_on_the_banks() {
        let dungeon = (0..30)
            .map(|seed| Dungeon::generate_themed(&mut Rng::new(seed), Theme::Sewer).expect("a dungeon"))
            .find(|dungeon| {
                dungeon.rooms.iter().any(|room| !room.waterworks.pipes.is_empty())
                    && dungeon.rooms.iter().any(|room| !room.waterworks.bridges.is_empty())
            })
            .expect("a sewer with a pipe and a bridge");
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        spawn_waterworks(&mut world, &mut physics, &dungeon, &textures);
        let pipes: usize = dungeon.rooms.iter().map(|room| room.waterworks.pipes.len()).sum();
        assert_eq!(world.query::<&Splash>().iter().count(), pipes);
        tick_waterworks(&mut world, &mut physics, &dungeon.surfaces, &textures, 0.0, 0.01);
        assert!(world.query::<&Expiring>().iter().count() >= pipes);
    }
}
