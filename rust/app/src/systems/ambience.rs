use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::components::{Color, Layer, Light, Lit, Player, Position, Size, Sprite};
use crate::dungeon::Rng;
use crate::render::TextureId;
use crate::viewport::TARGET;

const DUST_COUNT: usize = 14;
const DUST_PER_TICK: usize = 3;
const DUST_MARGIN: f32 = 160.0;
const DUST_LAYER: i32 = 2;
const DUST_FRAMES: usize = 3;
const DUST_SIZE: (f32, f32) = (12.0, 22.0);
const DUST_PEAK: (f32, f32) = (0.12, 0.3);
const FIREFLIES: usize = 4;
const FIREFLY_LAYER: i32 = 3;
const FIREFLY_FLEE_RANGE: f32 = 150.0;
const FIREFLY_FLEE_PUSH: f32 = 0.6;
const FIREFLY_LIGHT: f32 = 1.2;
const FIREFLY_RADIUS: f32 = 70.0;

pub(crate) struct Ambience {
    rng: Rng,
    clock: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Dust {
    velocity: Vec2,
    phase: f32,
    age: f32,
    life: f32,
    peak: f32,
}

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

pub(crate) fn spawn_ambience(world: &mut World, bounds: Vec2, portal_center: Vec2, seed: u64) {
    let mut rng = Rng::new(seed);
    for index in 0..FIREFLIES {
        let home = if index < FIREFLIES / 2 {
            let angle = between(&mut rng, 0.0, std::f32::consts::TAU);
            let distance = between(&mut rng, 140.0, 320.0);
            portal_center + Vec2::new(angle.cos(), angle.sin()) * distance
        } else {
            Vec2::new(
                between(&mut rng, 0.2, 0.8) * bounds.x,
                between(&mut rng, 0.25, 0.8) * bounds.y,
            )
        };
        let firefly = Firefly {
            home,
            reach: Vec2::new(
                between(&mut rng, 40.0, 110.0),
                between(&mut rng, 30.0, 80.0),
            ),
            rate: Vec2::new(between(&mut rng, 0.15, 0.4), between(&mut rng, 0.2, 0.5)),
            phase: between(&mut rng, 0.0, std::f32::consts::TAU),
            blink: between(&mut rng, 0.5, 1.1),
        };
        world.spawn((
            firefly,
            Position(home),
            Size(Vec2::splat(5.0)),
            Color(Vec4::new(0.8, 1.0, 0.35, 0.0)),
            Layer(FIREFLY_LAYER),
            Light::rgb(Vec2::ZERO, 200.0, 255.0, 90.0, FIREFLY_RADIUS, 0.0),
        ));
    }
    world.spawn((Ambience { rng, clock: 0.0 },));
}

pub(crate) fn tick_ambience(world: &mut World, texture: TextureId, camera: Vec2, dt: f32) {
    let Some(controller) = world.query::<Entity>().with::<&Ambience>().iter().next() else {
        return;
    };
    let Ok(mut ambience) = world.remove_one::<Ambience>(controller) else {
        return;
    };
    ambience.clock += dt;
    let player = world
        .query::<&Position>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|position| position.0);
    top_up_dust(world, &mut ambience, texture, camera);
    drift_dust(world, ambience.clock, camera, dt);
    glow_fireflies(world, ambience.clock, player);
    let _ = world.insert_one(controller, ambience);
}

fn top_up_dust(world: &mut World, ambience: &mut Ambience, texture: TextureId, camera: Vec2) {
    let present = world.query::<&Dust>().iter().count();
    for _ in 0..DUST_COUNT.saturating_sub(present).min(DUST_PER_TICK) {
        let rng = &mut ambience.rng;
        let at = camera + Vec2::new(between(rng, 0.0, TARGET.x), between(rng, 0.0, TARGET.y));
        let size = between(rng, DUST_SIZE.0, DUST_SIZE.1);
        let frame = rng.below(DUST_FRAMES);
        let mote = Dust {
            velocity: Vec2::new(between(rng, -6.0, 6.0), -between(rng, 3.0, 12.0)),
            phase: between(rng, 0.0, std::f32::consts::TAU),
            age: 0.0,
            life: between(rng, 6.0, 12.0),
            peak: between(rng, DUST_PEAK.0, DUST_PEAK.1),
        };
        world.spawn((
            mote,
            Position(at),
            Size(Vec2::splat(size)),
            Color(Vec4::new(1.0, 0.95, 0.85, 0.0)),
            Sprite::cell(texture, frame, 0, DUST_FRAMES, 1),
            Layer(DUST_LAYER),
            Lit,
        ));
    }
}

fn drift_dust(world: &mut World, clock: f32, camera: Vec2, dt: f32) {
    let mut gone = Vec::new();
    let low = camera - Vec2::splat(DUST_MARGIN);
    let high = camera + TARGET + Vec2::splat(DUST_MARGIN);
    for (entity, mote, position, color) in
        world.query_mut::<(Entity, &mut Dust, &mut Position, &mut Color)>()
    {
        mote.age += dt;
        let sway = (clock * 0.7 + mote.phase).sin() * 5.0;
        position.0 += (mote.velocity + Vec2::new(sway, 0.0)) * dt;
        let progress = (mote.age / mote.life).clamp(0.0, 1.0);
        color.0.w = mote.peak * (progress * std::f32::consts::PI).sin();
        if mote.age >= mote.life || position.0.cmplt(low).any() || position.0.cmpgt(high).any() {
            gone.push(entity);
        }
    }
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
        color.0.w = glow;
        light.intensity = glow * FIREFLY_LIGHT;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lobby() -> World {
        let mut world = World::new();
        spawn_ambience(
            &mut world,
            Vec2::new(1600.0, 900.0),
            Vec2::new(800.0, 300.0),
            7,
        );
        world
    }

    fn run(world: &mut World, seconds: f32) {
        let steps = (seconds * 60.0) as usize;
        for _ in 0..steps {
            tick_ambience(world, TextureId::WHITE, Vec2::ZERO, 1.0 / 60.0);
        }
    }

    fn count<T: hecs::Component>(world: &World) -> usize {
        world.query::<&T>().iter().count()
    }

    #[test]
    fn dust_fills_the_view_up_to_a_limit_and_never_beyond() {
        let mut world = lobby();
        run(&mut world, 0.05);
        assert!(count::<Dust>(&world) <= DUST_PER_TICK * 3);
        run(&mut world, 3.0);
        assert_eq!(count::<Dust>(&world), DUST_COUNT);
        run(&mut world, 30.0);
        assert!(count::<Dust>(&world) <= DUST_COUNT);
    }

    #[test]
    fn dust_fades_in_from_nothing_and_stays_faint() {
        let mut world = lobby();
        run(&mut world, 5.0);
        let alphas: Vec<f32> = world
            .query::<(&Dust, &Color)>()
            .iter()
            .map(|(_, color)| color.0.w)
            .collect();
        assert!(alphas
            .iter()
            .all(|alpha| (0.0..=DUST_PEAK.1 + 0.001).contains(alpha)));
        assert!(alphas.iter().any(|alpha| *alpha > 0.03));
    }

    #[test]
    fn dust_that_drifts_out_of_view_is_removed() {
        let mut world = lobby();
        run(&mut world, 1.0);
        tick_ambience(
            &mut world,
            TextureId::WHITE,
            Vec2::new(10_000.0, 10_000.0),
            1.0 / 60.0,
        );
        let far = world
            .query::<(&Dust, &Position)>()
            .iter()
            .filter(|(_, position)| position.0.x < 5_000.0)
            .count();
        assert_eq!(far, 0);
    }

    #[test]
    fn fireflies_wander_near_home_and_blink_on_and_off() {
        let mut world = lobby();
        let mut brightest: f32 = 0.0;
        let mut dimmest: f32 = f32::MAX;
        for _ in 0..1200 {
            tick_ambience(&mut world, TextureId::WHITE, Vec2::ZERO, 1.0 / 60.0);
            for (firefly, position, light) in world.query::<(&Firefly, &Position, &Light)>().iter()
            {
                assert!((position.0 - firefly.home)
                    .abs()
                    .cmple(firefly.reach + Vec2::splat(1.0))
                    .all());
                brightest = brightest.max(light.intensity);
                dimmest = dimmest.min(light.intensity);
            }
        }
        assert!(brightest > FIREFLY_LIGHT * 0.9, "{brightest}");
        assert!(dimmest < 0.05, "{dimmest}");
    }

    #[test]
    fn a_firefly_shies_away_from_the_player() {
        let mut world = lobby();
        tick_ambience(&mut world, TextureId::WHITE, Vec2::ZERO, 0.0);
        let target = world
            .query::<(&Firefly, &Position)>()
            .iter()
            .next()
            .map(|(_, position)| position.0)
            .unwrap_or_default();
        world.spawn((Player, Position(target + Vec2::new(-30.0, 0.0))));

        tick_ambience(&mut world, TextureId::WHITE, Vec2::ZERO, 0.0);

        let after = world
            .query::<(&Firefly, &Position)>()
            .iter()
            .next()
            .map(|(_, position)| position.0)
            .unwrap_or_default();
        assert!(after.x > target.x, "{after:?} {target:?}");
    }
}
