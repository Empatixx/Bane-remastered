use std::f32::consts::TAU;

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::components::{
    Animation, CastLightning, Clip, Color, Enemy, Expiring, Health, Layer, Light, LightFade,
    Player, Position, Size, Spawning, Sprite,
};

use super::damage::{self, Damage, DamageSource};

pub(crate) const STRIKE_RANGE: f32 = 450.0;
pub(crate) const JUMP_RANGE: f32 = 300.0;
pub(crate) const MAX_TARGETS: usize = 6;
pub(crate) const LIGHTNING_DAMAGE: i32 = 6;
pub(crate) const LIGHTNING_SHAKE: f32 = 0.45;

const LAYER: i32 = 3;
const PIXEL: f32 = 3.0;

const BOLT_SECONDS: f32 = 0.35;
const FLICKER_SECONDS: f32 = 0.05;
const BOLT_FRAMES: usize = 4;
const BOLT_CELL: Vec2 = Vec2::new(32.0, 16.0);

const BURST_FRAMES: usize = 6;
const BURST_FRAME_TIME: f32 = 0.06;
const BURST_SIZE: f32 = 32.0 * PIXEL;
const FLASH_RADIUS: f32 = 220.0;
const FLASH_INTENSITY: f32 = 1.6;

const SPARK_FRAMES: usize = 3;
const SPARK_SIZE: f32 = 8.0 * PIXEL;
const SPARKS_MIN: usize = 6;
const SPARKS_EXTRA: usize = 5;
const SPARK_SPEED: (f32, f32) = (140.0, 360.0);
const SPARK_LIFE: (f32, f32) = (0.28, 0.5);
const SPARK_DRAG: f32 = 6.0;

/// One straight piece of a bolt, drawn rotated along the jump.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Bolt {
    pub(crate) angle: f32,
    age: f32,
    since_flicker: f32,
    seed: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Spark {
    velocity: Vec2,
    age: f32,
    life: f32,
}

fn fraction(seed: f32) -> f32 {
    ((seed * 12.9898).sin() * 43_758.547).fract().abs()
}

fn between(range: (f32, f32), roll: f32) -> f32 {
    range.0 + (range.1 - range.0) * roll
}

/// Who the chain hits, in order: the nearest enemy in strike range, then each next nearest
/// unhit enemy within jump range of the previous target.
pub(crate) fn chain_targets(origin: Vec2, enemies: &[(Entity, Vec2)]) -> Vec<(Entity, Vec2)> {
    let mut hit: Vec<(Entity, Vec2)> = Vec::new();
    let (mut from, mut range) = (origin, STRIKE_RANGE);
    while hit.len() < MAX_TARGETS {
        let next = enemies
            .iter()
            .filter(|(entity, at)| {
                at.distance(from) <= range && !hit.iter().any(|(done, _)| done == entity)
            })
            .min_by(|a, b| {
                a.1.distance_squared(from)
                    .total_cmp(&b.1.distance_squared(from))
            });
        let Some(&(entity, at)) = next else {
            break;
        };
        hit.push((entity, at));
        from = at;
        range = JUMP_RANGE;
    }
    hit
}

fn living_enemies(world: &World) -> Vec<(Entity, Vec2)> {
    world
        .query::<(Entity, &Position, &Health)>()
        .with::<&Enemy>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .filter(|(_, _, health)| health.current > 0)
        .map(|(entity, position, _)| (entity, position.0))
        .collect()
}

/// Handles a `CastLightning` request; returns how many enemies were struck.
pub(crate) fn cast_lightning(world: &mut World, textures: &Textures, seed: f32) -> usize {
    let Some((player, origin)) = world
        .query::<(Entity, &Position)>()
        .with::<(&Player, &CastLightning)>()
        .iter()
        .next()
        .map(|(entity, position)| (entity, position.0))
    else {
        return 0;
    };
    let _ = world.remove_one::<CastLightning>(player);
    let targets = chain_targets(origin, &living_enemies(world));
    let mut from = origin;
    for (index, &(target, at)) in targets.iter().enumerate() {
        damage::deal(
            world,
            Damage {
                target,
                amount: LIGHTNING_DAMAGE,
                source: DamageSource::Explosion,
            },
        );
        let seed = seed + index as f32 * 17.0;
        spawn_bolt(world, textures, from, at, seed);
        spawn_impact(world, textures, at, seed);
        from = at;
    }
    targets.len()
}

fn bolt_uv(seed: f32) -> [f32; 4] {
    let frame = (fraction(seed) * BOLT_FRAMES as f32) as usize % BOLT_FRAMES;
    let uv = Sprite::cell_uv(frame, 0, BOLT_FRAMES, 1);
    if fraction(seed + 0.5) < 0.5 {
        [uv[0], uv[3], uv[2], uv[1]]
    } else {
        uv
    }
}

fn spawn_bolt(world: &mut World, textures: &Textures, from: Vec2, to: Vec2, seed: f32) {
    let span = to - from;
    let length = span.length();
    if length < 1.0 {
        return;
    }
    let piece = BOLT_CELL.x * PIXEL;
    let pieces = (length / piece).round().max(1.0) as usize;
    let step = span / pieces as f32;
    let texture = textures.get(tex::ARTEFACTS_LIGHTNING_BOLT);
    for index in 0..pieces {
        let seed = seed + index as f32 * 3.1;
        world.spawn((
            Bolt {
                angle: span.to_angle(),
                age: 0.0,
                since_flicker: 0.0,
                seed,
            },
            Position(from + step * (index as f32 + 0.5)),
            Size(Vec2::new(step.length(), BOLT_CELL.y * PIXEL)),
            Color(Vec4::ONE),
            Sprite {
                texture,
                uv: bolt_uv(seed),
            },
            Layer(LAYER),
        ));
    }
}

fn spawn_impact(world: &mut World, textures: &Textures, at: Vec2, seed: f32) {
    let burst = Clip {
        row: 0,
        frames: BURST_FRAMES,
        frame_time: BURST_FRAME_TIME,
        looping: false,
        flipped: false,
    };
    world.spawn((
        Expiring,
        Position(at),
        Size(Vec2::splat(BURST_SIZE)),
        Color(Vec4::ONE),
        Sprite::cell(
            textures.get(tex::ARTEFACTS_LIGHTNING_BURST),
            0,
            0,
            BURST_FRAMES,
            1,
        ),
        Animation::new(BURST_FRAMES, 1, burst),
        Layer(LAYER),
        Light::rgb(
            Vec2::ZERO,
            255.0,
            244.0,
            190.0,
            FLASH_RADIUS,
            FLASH_INTENSITY,
        ),
        LightFade {
            base: FLASH_INTENSITY,
        },
    ));
    let texture = textures.get(tex::ARTEFACTS_LIGHTNING_SPARK);
    let count = SPARKS_MIN + (fraction(seed + 9.0) * SPARKS_EXTRA as f32) as usize;
    for index in 0..count {
        let seed = seed + index as f32 * 1.7;
        let angle = (index as f32 + fraction(seed + 1.0)) / count as f32 * TAU;
        let speed = between(SPARK_SPEED, fraction(seed + 2.0));
        world.spawn((
            Spark {
                velocity: Vec2::from_angle(angle) * speed,
                age: 0.0,
                life: between(SPARK_LIFE, fraction(seed + 3.0)),
            },
            Position(at),
            Size(Vec2::splat(SPARK_SIZE)),
            Color(Vec4::ONE),
            Sprite::cell(texture, 0, 0, SPARK_FRAMES, 1),
            Layer(LAYER),
        ));
    }
}

/// Flickers and fades the bolts and flies the sparks; both vanish when spent.
pub(crate) fn tick_lightning(world: &mut World, dt: f32) {
    let mut gone = Vec::new();
    for (entity, bolt, sprite, color) in
        world.query_mut::<(Entity, &mut Bolt, &mut Sprite, &mut Color)>()
    {
        bolt.age += dt;
        bolt.since_flicker += dt;
        if bolt.since_flicker >= FLICKER_SECONDS {
            bolt.since_flicker -= FLICKER_SECONDS;
            bolt.seed += 1.0;
            sprite.uv = bolt_uv(bolt.seed);
        }
        let left = (1.0 - bolt.age / BOLT_SECONDS).clamp(0.0, 1.0);
        color.0.w = left.sqrt() * (0.75 + 0.25 * fraction(bolt.seed + 0.25));
        if bolt.age >= BOLT_SECONDS {
            gone.push(entity);
        }
    }
    for (entity, spark, position, sprite, color) in
        world.query_mut::<(Entity, &mut Spark, &mut Position, &mut Sprite, &mut Color)>()
    {
        spark.age += dt;
        spark.velocity *= (-SPARK_DRAG * dt).exp();
        position.0 += spark.velocity * dt;
        let progress = (spark.age / spark.life).clamp(0.0, 1.0);
        let frame = ((progress * SPARK_FRAMES as f32) as usize).min(SPARK_FRAMES - 1);
        sprite.uv = Sprite::cell_uv(frame, 0, SPARK_FRAMES, 1);
        color.0.w = 1.0 - progress * progress;
        if spark.age >= spark.life {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

#[cfg(test)]
mod tests {
    use crate::enemies::EnemyKind;
    use crate::physics::Physics;
    use crate::prefabs;

    use super::*;

    fn entities(count: usize) -> Vec<Entity> {
        let mut world = World::new();
        (0..count).map(|_| world.spawn(())).collect()
    }

    fn order(hits: &[(Entity, Vec2)]) -> Vec<Entity> {
        hits.iter().map(|(entity, _)| *entity).collect()
    }

    #[test]
    fn the_chain_strikes_the_nearest_enemy_first_and_then_the_nearest_to_each_target() {
        let e = entities(3);
        let enemies = [
            (e[0], Vec2::new(-140.0, 0.0)),
            (e[1], Vec2::new(100.0, 0.0)),
            (e[2], Vec2::new(100.0, 120.0)),
        ];

        let hits = chain_targets(Vec2::ZERO, &enemies);

        assert_eq!(order(&hits), vec![e[1], e[2], e[0]]);
    }

    #[test]
    fn the_chain_never_strikes_the_same_enemy_twice() {
        let e = entities(2);
        let enemies = [(e[0], Vec2::new(50.0, 0.0)), (e[1], Vec2::new(60.0, 0.0))];

        let hits = chain_targets(Vec2::ZERO, &enemies);

        assert_eq!(order(&hits), vec![e[0], e[1]]);
    }

    #[test]
    fn nothing_beyond_the_strike_range_is_hit() {
        let e = entities(1);
        let enemies = [(e[0], Vec2::new(STRIKE_RANGE + 1.0, 0.0))];

        assert!(chain_targets(Vec2::ZERO, &enemies).is_empty());
    }

    #[test]
    fn the_chain_stops_when_the_next_enemy_is_beyond_the_jump_range_of_the_last_target() {
        let e = entities(3);
        let enemies = [
            (e[0], Vec2::new(400.0, 0.0)),
            (e[1], Vec2::new(400.0 + JUMP_RANGE - 1.0, 0.0)),
            (e[2], Vec2::new(400.0 + JUMP_RANGE * 2.0 + 1.0, 0.0)),
        ];

        let hits = chain_targets(Vec2::ZERO, &enemies);

        assert_eq!(order(&hits), vec![e[0], e[1]]);
    }

    #[test]
    fn the_chain_strikes_at_most_six_enemies() {
        let e = entities(10);
        let enemies: Vec<(Entity, Vec2)> = e
            .iter()
            .enumerate()
            .map(|(index, entity)| (*entity, Vec2::new(50.0 * (index + 1) as f32, 0.0)))
            .collect();

        let hits = chain_targets(Vec2::ZERO, &enemies);

        assert_eq!(hits.len(), MAX_TARGETS);
        assert_eq!(order(&hits), e[..MAX_TARGETS].to_vec());
    }

    fn arena(enemies: &[Vec2]) -> (World, Entity, Vec<Entity>) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let player = prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(500.0, 500.0),
            &Textures::default(),
        );
        let enemies = enemies
            .iter()
            .map(|at| prefabs::ready_enemy(&mut world, &mut physics, EnemyKind::Slime, *at))
            .collect();
        (world, player, enemies)
    }

    fn health(world: &World, entity: Entity) -> i32 {
        world.get::<&Health>(entity).map_or(-1, |h| h.current)
    }

    fn count<T: hecs::Component>(world: &World) -> usize {
        world.query::<&T>().iter().count()
    }

    #[test]
    fn casting_damages_each_struck_enemy_and_consumes_the_request() {
        let (mut world, player, enemies) =
            arena(&[Vec2::new(600.0, 500.0), Vec2::new(5000.0, 500.0)]);
        let before: Vec<i32> = enemies.iter().map(|e| health(&world, *e)).collect();
        let _ = world.insert_one(player, CastLightning);

        let hits = cast_lightning(&mut world, &Textures::default(), 1.0);

        assert_eq!(hits, 1);
        assert_eq!(
            health(&world, enemies[0]),
            (before[0] - LIGHTNING_DAMAGE).max(0)
        );
        assert_eq!(health(&world, enemies[1]), before[1]);
        assert!(world.get::<&CastLightning>(player).is_err());
    }

    #[test]
    fn without_a_request_nothing_happens() {
        let (mut world, _, enemies) = arena(&[Vec2::new(600.0, 500.0)]);
        let before = health(&world, enemies[0]);

        assert_eq!(cast_lightning(&mut world, &Textures::default(), 1.0), 0);
        assert_eq!(health(&world, enemies[0]), before);
        assert_eq!(count::<Bolt>(&world), 0);
    }

    #[test]
    fn dying_and_spawning_enemies_are_not_struck() {
        let (mut world, player, enemies) =
            arena(&[Vec2::new(600.0, 500.0), Vec2::new(650.0, 500.0)]);
        let _ = world.insert_one(enemies[0], Expiring);
        let _ = world.insert_one(
            enemies[1],
            Spawning {
                remaining: 1.0,
                total: 1.0,
            },
        );
        let _ = world.insert_one(player, CastLightning);

        assert_eq!(cast_lightning(&mut world, &Textures::default(), 1.0), 0);
    }

    #[test]
    fn every_strike_leaves_bolts_a_burst_and_sparks_which_all_fade_away() {
        let (mut world, player, _) = arena(&[Vec2::new(800.0, 500.0), Vec2::new(900.0, 600.0)]);
        let _ = world.insert_one(player, CastLightning);

        cast_lightning(&mut world, &Textures::default(), 1.0);

        assert!(count::<Bolt>(&world) >= 2);
        assert!(count::<Spark>(&world) >= 2 * SPARKS_MIN);
        assert_eq!(
            world.query::<&Light>().with::<&Expiring>().iter().count(),
            2
        );

        tick_lightning(&mut world, BOLT_SECONDS / 2.0);
        let mid = world
            .query::<(&Bolt, &Color)>()
            .iter()
            .map(|(_, color)| color.0.w)
            .fold(0.0_f32, f32::max);
        assert!(mid > 0.0 && mid < 1.0, "{mid}");

        for _ in 0..40 {
            tick_lightning(&mut world, 1.0 / 60.0);
        }
        assert_eq!(count::<Bolt>(&world), 0);
        assert_eq!(count::<Spark>(&world), 0);
    }

    #[test]
    fn a_long_jump_is_drawn_as_several_pieces_that_span_it_end_to_end() {
        let mut world = World::new();
        let from = Vec2::new(0.0, 0.0);
        let to = Vec2::new(0.0, 300.0);

        spawn_bolt(&mut world, &Textures::default(), from, to, 1.0);

        let pieces: Vec<(Vec2, Vec2, f32)> = world
            .query::<(&Position, &Size, &Bolt)>()
            .iter()
            .map(|(position, size, bolt)| (position.0, size.0, bolt.angle))
            .collect();
        assert!(pieces.len() >= 2);
        let total: f32 = pieces.iter().map(|(_, size, _)| size.x).sum();
        assert!((total - 300.0).abs() < 0.01, "{total}");
        for (at, _, angle) in pieces {
            assert!(at.x.abs() < 0.01 && at.y > 0.0 && at.y < 300.0);
            assert!((angle - std::f32::consts::FRAC_PI_2).abs() < 0.001);
        }
    }

    #[test]
    fn sparks_slow_down_as_they_fly() {
        let mut world = World::new();
        spawn_impact(&mut world, &Textures::default(), Vec2::ZERO, 2.0);
        let speeds = |world: &World| -> Vec<f32> {
            world
                .query::<&Spark>()
                .iter()
                .map(|spark| spark.velocity.length())
                .collect()
        };
        let before = speeds(&world);

        tick_lightning(&mut world, 0.1);

        for (now, then) in speeds(&world).iter().zip(before) {
            assert!(*now < then * 0.7, "{now} vs {then}");
        }
    }
}
