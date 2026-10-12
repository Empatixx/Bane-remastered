use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::components::{Color, Interactable, Interaction, Layer, Light, Player, Position, Size};
use crate::dungeon::Rng;
use crate::viewport::TARGET;

const MOTES: usize = 40;
const MOTES_PER_TICK: usize = 3;
const MOTE_MARGIN: f32 = 160.0;
const MOTE_LAYER: i32 = 2;
const FIREFLIES: usize = 10;
const FIREFLY_LAYER: i32 = 3;
const FIREFLY_FLEE_RANGE: f32 = 150.0;
const FIREFLY_FLEE_PUSH: f32 = 0.6;
const FIREFLY_LIGHT: f32 = 1.2;
const FIREFLY_RADIUS: f32 = 70.0;
const SPARK_LAYER: i32 = 3;
const SPARK_PERIOD: f32 = 0.11;
const SPARK_SPREAD: f32 = 90.0;
const PORTAL_PULSE: f32 = 0.15;

pub(crate) struct Ambience {
    rng: Rng,
    clock: f32,
    spark_timer: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Mote {
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct Spark {
    velocity: Vec2,
    phase: f32,
    age: f32,
    life: f32,
    size: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PortalGlow {
    base: f32,
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
    let portals: Vec<(Entity, f32)> = world
        .query::<(Entity, &Interactable, &Light)>()
        .iter()
        .filter(|(_, interactable, _)| interactable.action == Interaction::EnterPortal)
        .map(|(entity, _, light)| (entity, light.intensity))
        .collect();
    for (entity, base) in portals {
        let _ = world.insert_one(entity, PortalGlow { base });
    }
    world.spawn((Ambience {
        rng,
        clock: 0.0,
        spark_timer: 0.0,
    },));
}

pub(crate) fn tick_ambience(world: &mut World, camera: Vec2, dt: f32) {
    let Some(controller) = world.query::<Entity>().with::<&Ambience>().iter().next() else {
        return;
    };
    let Ok(mut ambience) = world.remove_one::<Ambience>(controller) else {
        return;
    };
    ambience.clock += dt;
    ambience.spark_timer -= dt;
    let player = world
        .query::<&Position>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|position| position.0);
    let portal = world
        .query::<&Position>()
        .with::<&PortalGlow>()
        .iter()
        .next()
        .map(|position| position.0);

    top_up_motes(world, &mut ambience, camera);
    drift_motes(world, ambience.clock, camera, dt);
    glow_fireflies(world, ambience.clock, player);
    if let Some(portal) = portal {
        if ambience.spark_timer <= 0.0 {
            ambience.spark_timer = SPARK_PERIOD;
            spawn_spark(world, &mut ambience.rng, portal);
        }
    }
    rise_sparks(world, ambience.clock, dt);
    pulse_portal(world, ambience.clock);

    let _ = world.insert_one(controller, ambience);
}

fn top_up_motes(world: &mut World, ambience: &mut Ambience, camera: Vec2) {
    let present = world.query::<&Mote>().iter().count();
    for _ in 0..MOTES.saturating_sub(present).min(MOTES_PER_TICK) {
        let rng = &mut ambience.rng;
        let at = camera + Vec2::new(between(rng, 0.0, TARGET.x), between(rng, 0.0, TARGET.y));
        let size = between(rng, 2.0, 4.0);
        let mote = Mote {
            velocity: Vec2::new(between(rng, -6.0, 6.0), -between(rng, 3.0, 12.0)),
            phase: between(rng, 0.0, std::f32::consts::TAU),
            age: 0.0,
            life: between(rng, 6.0, 12.0),
            peak: between(rng, 0.15, 0.4),
        };
        world.spawn((
            mote,
            Position(at),
            Size(Vec2::splat(size)),
            Color(Vec4::new(1.0, 0.95, 0.8, 0.0)),
            Layer(MOTE_LAYER),
        ));
    }
}

fn drift_motes(world: &mut World, clock: f32, camera: Vec2, dt: f32) {
    let mut gone = Vec::new();
    let low = camera - Vec2::splat(MOTE_MARGIN);
    let high = camera + TARGET + Vec2::splat(MOTE_MARGIN);
    for (entity, mote, position, color) in
        world.query_mut::<(Entity, &mut Mote, &mut Position, &mut Color)>()
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

fn spawn_spark(world: &mut World, rng: &mut Rng, portal: Vec2) {
    let size = between(rng, 3.0, 5.0);
    let spark = Spark {
        velocity: Vec2::new(0.0, -between(rng, 35.0, 65.0)),
        phase: between(rng, 0.0, std::f32::consts::TAU),
        age: 0.0,
        life: between(rng, 1.0, 2.0),
        size,
    };
    let at = portal + Vec2::new(between(rng, -SPARK_SPREAD / 2.0, SPARK_SPREAD / 2.0), 10.0);
    world.spawn((
        spark,
        Position(at),
        Size(Vec2::splat(size)),
        Color(Vec4::new(0.85, 0.35, 1.0, 0.9)),
        Layer(SPARK_LAYER),
    ));
}

fn rise_sparks(world: &mut World, clock: f32, dt: f32) {
    let mut gone = Vec::new();
    for (entity, spark, position, size, color) in
        world.query_mut::<(Entity, &mut Spark, &mut Position, &mut Size, &mut Color)>()
    {
        spark.age += dt;
        let sway = (clock * 3.0 + spark.phase).sin() * 14.0;
        position.0 += (spark.velocity + Vec2::new(sway, 0.0)) * dt;
        let remaining = (1.0 - spark.age / spark.life).clamp(0.0, 1.0);
        color.0.w = 0.9 * remaining;
        size.0 = Vec2::splat(spark.size * (0.4 + 0.6 * remaining));
        if spark.age >= spark.life {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

fn pulse_portal(world: &mut World, clock: f32) {
    for (glow, light) in world.query_mut::<(&PortalGlow, &mut Light)>() {
        light.intensity = glow.base * (1.0 + PORTAL_PULSE * (clock * 2.2).sin());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lobby() -> World {
        let mut world = World::new();
        world.spawn((
            Position(Vec2::new(800.0, 300.0)),
            Interactable {
                action: Interaction::EnterPortal,
            },
            Light::rgb(Vec2::ZERO, 128.0, 0.0, 128.0, 50.0, 2.5),
        ));
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
            tick_ambience(world, Vec2::ZERO, 1.0 / 60.0);
        }
    }

    fn count<T: hecs::Component>(world: &World) -> usize {
        world.query::<&T>().iter().count()
    }

    #[test]
    fn dust_fills_the_view_up_to_a_limit_and_never_beyond() {
        let mut world = lobby();
        run(&mut world, 0.05);
        assert!(count::<Mote>(&world) <= MOTES_PER_TICK * 3);
        run(&mut world, 3.0);
        assert_eq!(count::<Mote>(&world), MOTES);
        run(&mut world, 30.0);
        assert!(count::<Mote>(&world) <= MOTES);
    }

    #[test]
    fn dust_fades_in_from_nothing_and_stays_faint() {
        let mut world = lobby();
        run(&mut world, 5.0);
        let alphas: Vec<f32> = world
            .query::<(&Mote, &Color)>()
            .iter()
            .map(|(_, color)| color.0.w)
            .collect();
        assert!(alphas.iter().all(|alpha| (0.0..=0.4001).contains(alpha)));
        assert!(alphas.iter().any(|alpha| *alpha > 0.05));
    }

    #[test]
    fn dust_that_drifts_out_of_view_is_removed() {
        let mut world = lobby();
        run(&mut world, 1.0);
        tick_ambience(&mut world, Vec2::new(10_000.0, 10_000.0), 1.0 / 60.0);
        let far = world
            .query::<(&Mote, &Position)>()
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
            tick_ambience(&mut world, Vec2::ZERO, 1.0 / 60.0);
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
        tick_ambience(&mut world, Vec2::ZERO, 0.0);
        let target = world
            .query::<(&Firefly, &Position)>()
            .iter()
            .next()
            .map(|(_, position)| position.0)
            .unwrap_or_default();
        world.spawn((Player, Position(target + Vec2::new(-30.0, 0.0))));

        tick_ambience(&mut world, Vec2::ZERO, 0.0);

        let after = world
            .query::<(&Firefly, &Position)>()
            .iter()
            .next()
            .map(|(_, position)| position.0)
            .unwrap_or_default();
        assert!(after.x > target.x, "{after:?} {target:?}");
    }

    #[test]
    fn sparks_rise_from_the_portal_and_burn_out() {
        let mut world = lobby();
        run(&mut world, 0.5);
        let portal_y = 300.0;
        assert!(count::<Spark>(&world) > 2);
        run(&mut world, 1.0);
        let highest = world
            .query::<(&Spark, &Position)>()
            .iter()
            .map(|(_, position)| position.0.y)
            .fold(f32::MAX, f32::min);
        assert!(highest < portal_y - 10.0, "{highest}");
        let alive_after = {
            for spark in world.query_mut::<&mut Spark>() {
                spark.age = spark.life;
            }
            tick_ambience(&mut world, Vec2::ZERO, 0.0);
            count::<Spark>(&world)
        };
        assert!(alive_after <= 1);
    }

    #[test]
    fn the_portal_light_pulses_around_its_base() {
        let mut world = lobby();
        let mut low: f32 = f32::MAX;
        let mut high: f32 = 0.0;
        for _ in 0..400 {
            tick_ambience(&mut world, Vec2::ZERO, 1.0 / 60.0);
            for (_, light) in world.query::<(&PortalGlow, &Light)>().iter() {
                low = low.min(light.intensity);
                high = high.max(light.intensity);
            }
        }
        assert!(low < 2.5 && high > 2.5);
        assert!(low >= 2.5 * (1.0 - PORTAL_PULSE) - 0.001);
        assert!(high <= 2.5 * (1.0 + PORTAL_PULSE) + 0.001);
    }
}
