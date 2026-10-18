use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::components::{Color, Layer, Light, Player, Position, Size, Sprite};
use crate::render::TextureId;
use crate::rng::Rng;
use crate::systems::dust::{DustSpec, FRAMES, spawn_dust};
use crate::viewport::TARGET;

const DUST_COUNT: usize = 14;
const DUST_PER_TICK: usize = 3;
const DUST_MARGIN: f32 = 160.0;
const DUST_LAYER: i32 = 2;
const DUST_SIZE: (f32, f32) = (12.0, 22.0);
const DUST_PEAK: (f32, f32) = (0.12, 0.3);
const DUST_LIFE: (f32, f32) = (6.0, 12.0);
const FIREFLIES: usize = 4;
const FIREFLY_LAYER: i32 = 3;
const FIREFLY_SIZE: f32 = 16.0;
const FIREFLY_FLEE_RANGE: f32 = 150.0;
const FIREFLY_FLEE_PUSH: f32 = 0.6;
const FIREFLY_LIGHT: f32 = 1.2;
const FIREFLY_RADIUS: f32 = 90.0;
const FIREFLY_TINT: Vec4 = Vec4::new(0.85, 1.0, 0.3, 0.0);

/// The dice and clock a quiet room's dust and fireflies are drawn with. Not an entity: the scene
/// that spawned the fireflies keeps it, for as long as their room stands.
pub(crate) struct Ambience {
    rng: Rng,
    clock: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Ambient;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Firefly {
    home: Vec2,
    reach: Vec2,
    rate: Vec2,
    phase: f32,
    blink: f32,
}

fn between(rng: &mut Rng, low: f32, high: f32) -> f32 {
    low + (high - low) * rng.float()
}

pub(crate) fn spawn_ambience(
    world: &mut World,
    texture: TextureId,
    bounds: Vec2,
    portal_center: Vec2,
    seed: u64,
) -> Ambience {
    let mut rng = Rng::new(seed);
    for index in 0..FIREFLIES {
        let home = if index % 2 == 0 {
            let angle = between(&mut rng, 0.0, std::f32::consts::TAU);
            let distance = between(&mut rng, 120.0, 280.0);
            portal_center + Vec2::new(angle.cos(), angle.sin()) * distance
        } else {
            let angle = between(&mut rng, 0.0, std::f32::consts::TAU);
            let distance = between(&mut rng, 80.0, 260.0);
            bounds / 2.0 + Vec2::new(angle.cos(), angle.sin()) * distance
        };
        let firefly = Firefly {
            home,
            reach: Vec2::new(
                between(&mut rng, 70.0, 140.0),
                between(&mut rng, 50.0, 100.0),
            ),
            rate: Vec2::new(between(&mut rng, 0.5, 0.9), between(&mut rng, 0.6, 1.1)),
            phase: between(&mut rng, 0.0, std::f32::consts::TAU),
            blink: between(&mut rng, 0.8, 1.4),
        };
        world.spawn((
            firefly,
            Position(home),
            Size(Vec2::splat(FIREFLY_SIZE)),
            Color(FIREFLY_TINT),
            Sprite::cell(texture, 1, 0, FRAMES, 1),
            Layer(FIREFLY_LAYER),
            Light::rgb(Vec2::ZERO, 200.0, 255.0, 90.0, FIREFLY_RADIUS, 0.0),
        ));
    }
    Ambience { rng, clock: 0.0 }
}

pub(crate) fn tick_ambience(
    world: &mut World,
    ambience: &mut Ambience,
    texture: TextureId,
    camera: Vec2,
    dt: f32,
) {
    ambience.clock += dt;
    let player = world
        .query::<&Position>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|position| position.0);

    cull_dust(world, camera);
    top_up_dust(world, ambience, texture, camera);
    glow_fireflies(world, ambience.clock, player);
}

fn top_up_dust(world: &mut World, ambience: &mut Ambience, texture: TextureId, camera: Vec2) {
    let present = world.query::<&Ambient>().iter().count();
    for _ in 0..DUST_COUNT.saturating_sub(present).min(DUST_PER_TICK) {
        let rng = &mut ambience.rng;
        let dust = spawn_dust(
            world,
            texture,
            DustSpec {
                at: camera + Vec2::new(between(rng, 0.0, TARGET.x), between(rng, 0.0, TARGET.y)),
                velocity: Vec2::new(between(rng, -6.0, 6.0), -between(rng, 3.0, 12.0)),
                size: between(rng, DUST_SIZE.0, DUST_SIZE.1),
                life: between(rng, DUST_LIFE.0, DUST_LIFE.1),
                peak: between(rng, DUST_PEAK.0, DUST_PEAK.1),
                layer: DUST_LAYER,
                seed: between(rng, 0.0, 1000.0),
            },
        );
        let _ = world.insert_one(dust, Ambient);
    }
}

fn cull_dust(world: &mut World, camera: Vec2) {
    let low = camera - Vec2::splat(DUST_MARGIN);
    let high = camera + TARGET + Vec2::splat(DUST_MARGIN);
    let gone: Vec<Entity> = world
        .query::<(Entity, &Position)>()
        .with::<&Ambient>()
        .iter()
        .filter(|(_, position)| position.0.cmplt(low).any() || position.0.cmpgt(high).any())
        .map(|(entity, _)| entity)
        .collect();
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

fn glow_fireflies(world: &mut World, clock: f32, player: Option<Vec2>) {
    for (firefly, position, color, light) in
        world.query_mut::<(&Firefly, &mut Position, &mut Color, &mut Light)>()
    {
        let wander = Vec2::new(
            (clock * firefly.rate.x + firefly.phase).sin() * firefly.reach.x,
            (clock * firefly.rate.y + firefly.phase * 1.3).cos() * firefly.reach.y,
        );
        let mut at = firefly.home + wander;
        if let Some(player) = player {
            let away = at - player;
            let distance = away.length();
            if distance < FIREFLY_FLEE_RANGE {
                at +=
                    away.normalize_or_zero() * (FIREFLY_FLEE_RANGE - distance) * FIREFLY_FLEE_PUSH;
            }
        }
        position.0 = at;
        let wave = ((clock * firefly.blink + firefly.phase).sin() + 0.35) / 1.35;
        let glow = wave.clamp(0.0, 1.0);
        let glow = glow * glow * (3.0 - 2.0 * glow);
        color.0.w = glow * 0.9;
        light.intensity = glow * FIREFLY_LIGHT;
    }
}

#[cfg(test)]
mod tests {
    use crate::systems::dust::Dust;
    use crate::systems::tick_dust;

    use super::*;

    fn lobby() -> (World, Ambience) {
        let mut world = World::new();
        let ambience = spawn_ambience(
            &mut world,
            TextureId::WHITE,
            Vec2::new(1600.0, 900.0),
            Vec2::new(800.0, 300.0),
            7,
        );
        (world, ambience)
    }

    fn run(world: &mut World, ambience: &mut Ambience, seconds: f32) {
        let steps = (seconds * 60.0) as usize;
        for step in 0..steps {
            tick_ambience(world, ambience, TextureId::WHITE, Vec2::ZERO, 1.0 / 60.0);
            tick_dust(world, step as f32 / 60.0, 1.0 / 60.0);
        }
    }

    fn ambient(world: &World) -> usize {
        world.query::<&Ambient>().iter().count()
    }

    #[test]
    fn dust_fills_the_view_up_to_a_limit_and_never_beyond() {
        let (mut world, mut ambience) = lobby();
        run(&mut world, &mut ambience, 0.05);
        assert!(ambient(&world) <= DUST_PER_TICK * 3);
        run(&mut world, &mut ambience, 3.0);
        assert_eq!(ambient(&world), DUST_COUNT);
        run(&mut world, &mut ambience, 30.0);
        assert!(ambient(&world) <= DUST_COUNT);
    }

    #[test]
    fn dust_stays_faint() {
        let (mut world, mut ambience) = lobby();
        run(&mut world, &mut ambience, 5.0);
        let alphas: Vec<f32> = world
            .query::<(&Dust, &Color)>()
            .iter()
            .map(|(_, color)| color.0.w)
            .collect();
        assert!(!alphas.is_empty());
        assert!(
            alphas
                .iter()
                .all(|alpha| (0.0..=DUST_PEAK.1 + 0.001).contains(alpha))
        );
        assert!(alphas.iter().any(|alpha| *alpha > 0.03));
    }

    #[test]
    fn dust_that_drifts_out_of_view_is_removed() {
        let (mut world, mut ambience) = lobby();
        run(&mut world, &mut ambience, 1.0);
        tick_ambience(
            &mut world,
            &mut ambience,
            TextureId::WHITE,
            Vec2::new(10_000.0, 10_000.0),
            1.0 / 60.0,
        );
        let near_origin = world
            .query::<(&Ambient, &Position)>()
            .iter()
            .filter(|(_, position)| position.0.x < 5_000.0)
            .count();
        assert_eq!(near_origin, 0);
    }

    #[test]
    fn fireflies_travel_visibly_and_blink_on_and_off() {
        let (mut world, mut ambience) = lobby();
        let start: Vec<Vec2> = world
            .query::<(&Firefly, &Position)>()
            .iter()
            .map(|(_, position)| position.0)
            .collect();
        let mut brightest: f32 = 0.0;
        let mut dimmest: f32 = f32::MAX;
        let mut farthest: f32 = 0.0;
        for step in 0..1200 {
            tick_ambience(&mut world, &mut ambience, TextureId::WHITE, Vec2::ZERO, 1.0 / 60.0);
            for (firefly, position, light) in world.query::<(&Firefly, &Position, &Light)>().iter()
            {
                assert!(
                    (position.0 - firefly.home)
                        .abs()
                        .cmple(firefly.reach + Vec2::splat(1.0))
                        .all()
                );
                brightest = brightest.max(light.intensity);
                dimmest = dimmest.min(light.intensity);
            }
            if step == 120 {
                farthest = world
                    .query::<(&Firefly, &Position)>()
                    .iter()
                    .zip(&start)
                    .map(|((_, position), from)| position.0.distance(*from))
                    .fold(0.0, f32::max);
            }
        }
        assert!(farthest > 40.0, "{farthest}");
        assert!(brightest > FIREFLY_LIGHT * 0.9, "{brightest}");
        assert!(dimmest < 0.05, "{dimmest}");
    }

    #[test]
    fn a_firefly_shies_away_from_the_player() {
        let (mut world, mut ambience) = lobby();
        tick_ambience(&mut world, &mut ambience, TextureId::WHITE, Vec2::ZERO, 0.0);
        let target = world
            .query::<(&Firefly, &Position)>()
            .iter()
            .next()
            .map(|(_, position)| position.0)
            .unwrap_or_default();
        world.spawn((Player, Position(target + Vec2::new(-30.0, 0.0))));

        tick_ambience(&mut world, &mut ambience, TextureId::WHITE, Vec2::ZERO, 0.0);

        let after = world
            .query::<(&Firefly, &Position)>()
            .iter()
            .next()
            .map(|(_, position)| position.0)
            .unwrap_or_default();
        assert!(after.x > target.x, "{after:?} {target:?}");
    }
}
