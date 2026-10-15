//! The air of a dungeon floor: fog banks drifting over the floor and wisps passing above the
//! camera on the damp floors, and the motes of each biome
//! (dust, embers, snow, spores, water drips).

use std::f32::consts::{PI, TAU};

use glam::{Vec2, Vec3, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::components::{Color, Layer, Lit, Position, Size, Sprite};
use crate::dungeon::{Dungeon, Theme};
use crate::render::TextureId;
use crate::rng::Rng;
use crate::tilemap::TILE_SIZE;
use crate::viewport::TARGET;

/// One pixel of the atmosphere sheets is this many world units, like the tiles.
const PIXEL: f32 = TILE_SIZE / 16.0;
const MARGIN: f32 = 240.0;
const PER_TICK: usize = 2;

const FOG_FRAMES: usize = 2;
const FOG_SIZE: Vec2 = Vec2::new(48.0 * PIXEL, 24.0 * PIXEL);
const FLOOR_FOG_LAYER: i32 = 0;
const WISP_LAYER: i32 = 6;
/// Wisps hang between the camera and the floor, so they slide past faster than the floor.
const WISP_PARALLAX: f32 = 0.35;

const MOTE_FRAMES: usize = 8;
const MOTE_SIZE: f32 = 8.0 * PIXEL;
const MOTE_LAYER: i32 = 3;
const DRIP_FALL: f32 = 0.45;
const DRIP_HEIGHT: f32 = 70.0;
const SPLASH_TIME: f32 = 0.22;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MoteKind {
    Dust,
    Ember,
    Snow,
    Spore,
}

impl MoteKind {
    fn frames(self) -> &'static [usize] {
        match self {
            Self::Dust => &[0],
            Self::Ember => &[1, 2, 2, 2],
            Self::Snow => &[3, 4, 4, 4],
            Self::Spore => &[5],
        }
    }

    /// Dust and snow only show where a light catches them; embers and spores glow by themselves.
    fn lit(self) -> bool {
        matches!(self, Self::Dust | Self::Snow)
    }
}

const DRIP_FRAME: usize = 6;
const SPLASH_FRAME: usize = 7;

#[derive(Debug, Clone, Copy)]
struct FogSpec {
    count: usize,
    tint: Vec3,
    peak: (f32, f32),
    drift: Vec2,
    scale: (f32, f32),
}

#[derive(Debug, Clone, Copy)]
struct MoteSpec {
    kind: MoteKind,
    count: usize,
    tint: Vec3,
    velocity_low: Vec2,
    velocity_high: Vec2,
    peak: (f32, f32),
    life: (f32, f32),
    sway: f32,
    flicker: bool,
}

/// What hangs in the air of one biome.
#[derive(Debug, Clone, Copy)]
struct Mood {
    floor_fog: Option<FogSpec>,
    wisps: Option<FogSpec>,
    motes: Option<MoteSpec>,
    /// Drips per second falling from the ceiling.
    drips: f32,
}

const fn rgb(red: f32, green: f32, blue: f32) -> Vec3 {
    Vec3::new(red / 255.0, green / 255.0, blue / 255.0)
}

const DUST: MoteSpec = MoteSpec {
    kind: MoteKind::Dust,
    count: 10,
    tint: rgb(255.0, 255.0, 255.0),
    velocity_low: Vec2::new(-5.0, 3.0),
    velocity_high: Vec2::new(5.0, 10.0),
    peak: (0.15, 0.35),
    life: (5.0, 10.0),
    sway: 6.0,
    flicker: false,
};

fn mood(theme: Theme) -> Mood {
    let none = Mood {
        floor_fog: None,
        wisps: None,
        motes: None,
        drips: 0.0,
    };
    match theme {
        Theme::Stone => Mood {
            motes: Some(DUST),
            ..none
        },
        Theme::Temple => Mood {
            motes: Some(MoteSpec {
                tint: rgb(255.0, 226.0, 150.0),
                velocity_low: Vec2::new(-4.0, -6.0),
                velocity_high: Vec2::new(4.0, 2.0),
                ..DUST
            }),
            ..none
        },
        Theme::Moss => Mood {
            floor_fog: Some(FogSpec {
                count: 6,
                tint: rgb(150.0, 196.0, 120.0),
                peak: (0.1, 0.17),
                drift: Vec2::new(7.0, 1.0),
                scale: (1.6, 2.6),
            }),
            wisps: Some(FogSpec {
                count: 2,
                tint: rgb(190.0, 220.0, 170.0),
                peak: (0.06, 0.1),
                drift: Vec2::new(10.0, 0.0),
                scale: (2.4, 3.2),
            }),
            motes: Some(MoteSpec {
                kind: MoteKind::Spore,
                count: 14,
                tint: rgb(255.0, 255.0, 255.0),
                velocity_low: Vec2::new(-4.0, -9.0),
                velocity_high: Vec2::new(4.0, -3.0),
                peak: (0.3, 0.6),
                life: (5.0, 9.0),
                sway: 9.0,
                flicker: true,
            }),
            ..none
        },
        Theme::Sewer => Mood {
            floor_fog: Some(FogSpec {
                count: 7,
                tint: rgb(120.0, 172.0, 166.0),
                peak: (0.1, 0.16),
                drift: Vec2::new(-6.0, 0.0),
                scale: (1.6, 2.4),
            }),
            drips: 2.2,
            ..none
        },
        Theme::Spider => Mood {
            motes: Some(MoteSpec { count: 12, ..DUST }),
            ..none
        },
        Theme::Ice => Mood {
            motes: Some(MoteSpec {
                kind: MoteKind::Snow,
                count: 46,
                tint: rgb(255.0, 255.0, 255.0),
                velocity_low: Vec2::new(10.0, 18.0),
                velocity_high: Vec2::new(22.0, 32.0),
                peak: (0.6, 1.0),
                life: (6.0, 11.0),
                sway: 10.0,
                flicker: false,
            }),
            ..none
        },
        Theme::Lava => Mood {
            motes: Some(MoteSpec {
                kind: MoteKind::Ember,
                count: 14,
                tint: rgb(255.0, 255.0, 255.0),
                velocity_low: Vec2::new(-7.0, -42.0),
                velocity_high: Vec2::new(7.0, -16.0),
                peak: (0.35, 0.6),
                life: (2.5, 5.0),
                sway: 8.0,
                flicker: true,
            }),
            ..none
        },
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct AtmosphereTextures {
    fog: TextureId,
    motes: TextureId,
}

impl AtmosphereTextures {
    pub(crate) fn load(textures: &Textures) -> Self {
        Self {
            fog: textures.get(tex::FOG),
            motes: textures.get(tex::MOTES),
        }
    }
}

struct Atmosphere {
    mood: Mood,
    rng: Rng,
    clock: f32,
    drip_debt: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HazeKind {
    Floor,
    Wisp,
}

#[derive(Debug, Clone, Copy)]
struct Haze {
    kind: HazeKind,
    anchor: Vec2,
    camera: Vec2,
    velocity: Vec2,
    age: f32,
    life: f32,
    peak: f32,
}

#[derive(Debug, Clone, Copy)]
struct Mote {
    velocity: Vec2,
    age: f32,
    life: f32,
    peak: f32,
    phase: f32,
    sway: f32,
    flicker: bool,
}

#[derive(Debug, Clone, Copy)]
struct Drip {
    landing: Vec2,
    age: f32,
}

fn between(rng: &mut Rng, low: f32, high: f32) -> f32 {
    low + (high - low) * rng.float()
}

/// Fills the floor's air for its biome; [`tick_atmosphere`] keeps it topped up around the camera.
pub(crate) fn spawn_atmosphere(world: &mut World, dungeon: &Dungeon, seed: u64) {
    let mood = mood(dungeon.theme);
    let rng = Rng::new(seed);
    world.spawn((Atmosphere {
        mood,
        rng,
        clock: 0.0,
        drip_debt: 0.0,
    },));
}

pub(crate) fn tick_atmosphere(
    world: &mut World,
    textures: AtmosphereTextures,
    camera: Vec2,
    dt: f32,
) {
    let Some(controller) = world.query::<Entity>().with::<&Atmosphere>().iter().next() else {
        return;
    };
    let Ok(mut air) = world.remove_one::<Atmosphere>(controller) else {
        return;
    };
    air.clock += dt;
    cull(world, camera);
    if let Some(spec) = air.mood.floor_fog {
        top_up_haze(
            world,
            &mut air.rng,
            textures.fog,
            spec,
            HazeKind::Floor,
            camera,
        );
    }
    if let Some(spec) = air.mood.wisps {
        top_up_haze(
            world,
            &mut air.rng,
            textures.fog,
            spec,
            HazeKind::Wisp,
            camera,
        );
    }
    if let Some(spec) = air.mood.motes {
        top_up_motes(world, &mut air.rng, textures.motes, spec, camera);
    }
    air.drip_debt += air.mood.drips * dt;
    while air.drip_debt >= 1.0 {
        air.drip_debt -= 1.0;
        spawn_drip(world, &mut air.rng, textures.motes, camera);
    }
    drift_haze(world, camera, dt);
    drift_motes(world, air.clock, dt);
    fall_drips(world, dt);
    let _ = world.insert_one(controller, air);
}

fn in_view(rng: &mut Rng, camera: Vec2, pad: f32) -> Vec2 {
    camera
        + Vec2::new(
            between(rng, -pad, TARGET.x + pad),
            between(rng, -pad, TARGET.y + pad),
        )
}

fn top_up_haze(
    world: &mut World,
    rng: &mut Rng,
    texture: TextureId,
    spec: FogSpec,
    kind: HazeKind,
    camera: Vec2,
) {
    let present = world
        .query::<&Haze>()
        .iter()
        .filter(|haze| haze.kind == kind)
        .count();
    for _ in 0..spec.count.saturating_sub(present).min(PER_TICK) {
        let at = in_view(rng, camera, FOG_SIZE.x);
        let scale = between(rng, spec.scale.0, spec.scale.1);
        let flip = rng.below(2) == 1;
        let frame = rng.below(FOG_FRAMES);
        let mut sprite = Sprite::cell(texture, frame, 0, FOG_FRAMES, 1);
        if flip {
            sprite.uv = [sprite.uv[2], sprite.uv[1], sprite.uv[0], sprite.uv[3]];
        }
        let speed = between(rng, 0.6, 1.4);
        let haze = Haze {
            kind,
            anchor: at,
            camera,
            velocity: spec.drift * speed,
            age: 0.0,
            life: between(rng, 14.0, 24.0),
            peak: between(rng, spec.peak.0, spec.peak.1),
        };
        let layer = match kind {
            HazeKind::Floor => FLOOR_FOG_LAYER,
            HazeKind::Wisp => WISP_LAYER,
        };
        let entity = world.spawn((
            haze,
            Position(at),
            Size(FOG_SIZE * scale),
            Color(spec.tint.extend(0.0)),
            sprite,
            Layer(layer),
        ));
        if kind == HazeKind::Floor {
            let _ = world.insert_one(entity, Lit);
        }
    }
}

fn top_up_motes(
    world: &mut World,
    rng: &mut Rng,
    texture: TextureId,
    spec: MoteSpec,
    camera: Vec2,
) {
    let present = world.query::<&Mote>().iter().count();
    for _ in 0..spec.count.saturating_sub(present).min(PER_TICK) {
        let frames = spec.kind.frames();
        let frame = frames[rng.below(frames.len())];
        let entity = world.spawn((
            Mote {
                velocity: Vec2::new(
                    between(rng, spec.velocity_low.x, spec.velocity_high.x),
                    between(rng, spec.velocity_low.y, spec.velocity_high.y),
                ),
                age: 0.0,
                life: between(rng, spec.life.0, spec.life.1),
                peak: between(rng, spec.peak.0, spec.peak.1),
                phase: between(rng, 0.0, TAU),
                sway: spec.sway,
                flicker: spec.flicker,
            },
            Position(in_view(rng, camera, 0.0)),
            Size(Vec2::splat(MOTE_SIZE)),
            Color(spec.tint.extend(0.0)),
            Sprite::cell(texture, frame, 0, MOTE_FRAMES, 1),
            Layer(MOTE_LAYER),
        ));
        if spec.kind.lit() {
            let _ = world.insert_one(entity, Lit);
        }
    }
}

fn spawn_drip(world: &mut World, rng: &mut Rng, texture: TextureId, camera: Vec2) {
    let landing = in_view(rng, camera, 0.0);
    world.spawn((
        Drip { landing, age: 0.0 },
        Position(landing - Vec2::new(0.0, DRIP_HEIGHT)),
        Size(Vec2::splat(MOTE_SIZE)),
        Color(Vec4::new(1.0, 1.0, 1.0, 0.85)),
        Sprite::cell(texture, DRIP_FRAME, 0, MOTE_FRAMES, 1),
        Layer(MOTE_LAYER),
        Lit,
    ));
}

/// Fades in, holds, fades out over a life.
fn swell(age: f32, life: f32) -> f32 {
    (age / life * PI).sin().clamp(0.0, 1.0).powf(0.6)
}

fn drift_haze(world: &mut World, camera: Vec2, dt: f32) {
    let mut gone = Vec::new();
    for (entity, haze, position, color) in
        world.query_mut::<(Entity, &mut Haze, &mut Position, &mut Color)>()
    {
        haze.age += dt;
        haze.anchor += haze.velocity * dt;
        let parallax = match haze.kind {
            HazeKind::Floor => 0.0,
            HazeKind::Wisp => WISP_PARALLAX,
        };
        position.0 = haze.anchor - (camera - haze.camera) * parallax;
        color.0.w = haze.peak * swell(haze.age, haze.life);
        if haze.age >= haze.life {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

fn drift_motes(world: &mut World, clock: f32, dt: f32) {
    let mut gone = Vec::new();
    for (entity, mote, position, color) in
        world.query_mut::<(Entity, &mut Mote, &mut Position, &mut Color)>()
    {
        mote.age += dt;
        let sway = (clock * 0.8 + mote.phase).sin() * mote.sway;
        position.0 += (mote.velocity + Vec2::new(sway, 0.0)) * dt;
        let mut alpha = mote.peak * swell(mote.age, mote.life);
        if mote.flicker {
            alpha *= 0.65 + 0.35 * (clock * 9.0 + mote.phase * 3.0).sin().abs();
        }
        color.0.w = alpha;
        if mote.age >= mote.life {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

fn fall_drips(world: &mut World, dt: f32) {
    let mut gone = Vec::new();
    for (entity, drip, position, sprite) in
        world.query_mut::<(Entity, &mut Drip, &mut Position, &mut Sprite)>()
    {
        drip.age += dt;
        if drip.age < DRIP_FALL {
            let fallen = (drip.age / DRIP_FALL).powi(2);
            position.0 = drip.landing - Vec2::new(0.0, DRIP_HEIGHT * (1.0 - fallen));
        } else {
            position.0 = drip.landing;
            sprite.uv = Sprite::cell_uv(SPLASH_FRAME, 0, MOTE_FRAMES, 1);
            if drip.age >= DRIP_FALL + SPLASH_TIME {
                gone.push(entity);
            }
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

fn cull(world: &mut World, camera: Vec2) {
    let low = camera - Vec2::splat(MARGIN + FOG_SIZE.x * 2.0);
    let high = camera + TARGET + Vec2::splat(MARGIN + FOG_SIZE.x * 2.0);
    let outside = |at: Vec2| at.cmplt(low).any() || at.cmpgt(high).any();
    let mut gone: Vec<Entity> = world
        .query::<(Entity, &Position)>()
        .with::<&Haze>()
        .iter()
        .filter(|(_, position)| outside(position.0))
        .map(|(entity, _)| entity)
        .collect();
    gone.extend(
        world
            .query::<(Entity, &Position)>()
            .with::<&Mote>()
            .iter()
            .filter(|(_, position)| outside(position.0))
            .map(|(entity, _)| entity),
    );
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [Theme; 7] = [
        Theme::Stone,
        Theme::Sewer,
        Theme::Moss,
        Theme::Spider,
        Theme::Ice,
        Theme::Lava,
        Theme::Temple,
    ];

    fn textures() -> AtmosphereTextures {
        AtmosphereTextures {
            fog: TextureId::WHITE,
            motes: TextureId::WHITE,
        }
    }

    fn floor(theme: Theme, seed: u64) -> (World, Dungeon) {
        let dungeon = Dungeon::generate_themed(&mut Rng::new(seed), theme).expect("a dungeon");
        let mut world = World::new();
        spawn_atmosphere(&mut world, &dungeon, seed);
        (world, dungeon)
    }

    fn run(world: &mut World, camera: Vec2, seconds: f32) {
        for _ in 0..(seconds * 60.0) as usize {
            tick_atmosphere(world, textures(), camera, 1.0 / 60.0);
        }
    }

    fn count<T: hecs::Component>(world: &World) -> usize {
        world.query::<&T>().iter().count()
    }

    #[test]
    fn every_biome_has_something_in_the_air_and_never_more_than_it_asks_for() {
        for theme in ALL {
            let (mut world, _) = floor(theme, 5);
            run(&mut world, Vec2::new(400.0, 400.0), 20.0);
            let air = mood(theme);
            let fog = count::<Haze>(&world);
            let motes = count::<Mote>(&world);
            assert!(fog + motes + count::<Drip>(&world) > 0, "{theme:?}");
            let fog_cap = air.floor_fog.map_or(0, |f| f.count) + air.wisps.map_or(0, |f| f.count);
            assert!(fog <= fog_cap, "{theme:?}");
            assert!(motes <= air.motes.map_or(0, |m| m.count), "{theme:?}");
        }
    }

    #[test]
    fn embers_rise_and_snow_falls() {
        for (theme, rising) in [(Theme::Lava, true), (Theme::Ice, false)] {
            let (mut world, _) = floor(theme, 9);
            run(&mut world, Vec2::ZERO, 0.1);
            let before: Vec<(Entity, f32)> = world
                .query::<(Entity, &Position)>()
                .with::<&Mote>()
                .iter()
                .map(|(entity, position)| (entity, position.0.y))
                .collect();
            run(&mut world, Vec2::ZERO, 1.0);
            for (entity, y) in before {
                let Ok(now) = world.get::<&Position>(entity) else {
                    continue;
                };
                assert_eq!(now.0.y < y, rising, "{theme:?}");
            }
        }
    }

    #[test]
    fn sewer_drips_fall_splash_and_go() {
        let (mut world, _) = floor(Theme::Sewer, 3);
        run(&mut world, Vec2::ZERO, 3.0);
        assert!(count::<Drip>(&world) > 0);
        assert!(count::<Drip>(&world) < 10);
    }

    #[test]
    fn air_left_behind_by_the_camera_is_removed() {
        let (mut world, _) = floor(Theme::Moss, 4);
        run(&mut world, Vec2::ZERO, 5.0);
        run(&mut world, Vec2::new(20_000.0, 20_000.0), 1.0 / 60.0);
        let near_start = world
            .query::<&Position>()
            .with::<&Mote>()
            .iter()
            .filter(|position| position.0.x < 5_000.0)
            .count();
        assert_eq!(near_start, 0);
    }

    #[test]
    fn wisps_slide_faster_than_the_floor_when_the_camera_moves() {
        let (mut world, _) = floor(Theme::Moss, 2);
        run(&mut world, Vec2::ZERO, 2.0);
        let wisp = world
            .query::<(Entity, &Haze, &Position)>()
            .iter()
            .find(|(_, haze, _)| haze.kind == HazeKind::Wisp)
            .map(|(entity, _, position)| (entity, position.0))
            .expect("a wisp");
        tick_atmosphere(&mut world, textures(), Vec2::new(100.0, 0.0), 0.0);
        let moved = world
            .get::<&Position>(wisp.0)
            .map(|p| p.0)
            .expect("still there");
        assert!((moved.x - (wisp.1.x - 100.0 * WISP_PARALLAX)).abs() < 0.01);
    }
}
