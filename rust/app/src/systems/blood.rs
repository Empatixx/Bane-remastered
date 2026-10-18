use std::f32::consts::{PI, TAU};

use glam::{Vec2, Vec3, Vec4};
use hecs::{Entity, World};

use crate::artefacts::BLOODLUST_SECONDS;
use crate::assets::{Textures, tex};
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Bloodlust, Color, Enemy, Gone, HealPercent, Health, Layer, Lit, Minion, Player, Position, Size,
    Sprite,
};
use crate::render::TextureId;

const DROP_FRAMES: usize = 6;
const DROP_RATE: f32 = 14.0;
const DROP_LIFE: f32 = 0.9;
const DROP_SIZE: f32 = 14.0;
const DROP_RISE: f32 = 34.0;
const DROP_RADIUS: f32 = 22.0;
const BURST_DROPS: usize = 14;
const BURST_SPEED: f32 = 150.0;
const BURST_LIFE: f32 = 0.55;
const BURST_DRAG: f32 = 4.0;
const PULSE_HZ: f32 = 2.0;
const PULSE_PEAK: Vec3 = Vec3::new(1.0, 0.55, 0.55);
const KILL_HEAL: i32 = 1;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BloodDrop {
    velocity: Vec2,
    drag: f32,
    age: f32,
    life: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BloodTint {
    original: Vec3,
}

fn fraction(seed: f32) -> f32 {
    ((seed * 12.9898).sin() * 43_758.547).fract().abs()
}

fn spawn_drop(world: &mut World, texture: TextureId, at: Vec2, drop: BloodDrop) {
    world.spawn((
        drop,
        Position(at),
        Size(Vec2::splat(DROP_SIZE)),
        Color(Vec4::ONE),
        Sprite::cell(texture, 0, 0, DROP_FRAMES, 1),
        Layer(1),
        Lit,
    ));
}

pub(crate) fn apply_heal_percent(
    world: &mut World,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
) {
    let Some((player, origin, fraction)) = world
        .query::<(Entity, &Position, &HealPercent)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(entity, position, heal)| (entity, position.0, heal.0))
    else {
        return;
    };
    let _ = world.remove_one::<HealPercent>(player);
    if let Ok(mut health) = world.get::<&mut Health>(player) {
        let healed = (health.max as f32 * fraction).ceil() as i32;
        health.current = (health.current + healed).min(health.max);
    }
    sounds.push(AudioCommand::Effect(SoundEffect::Heal));
    let texture = textures.get(tex::ARTEFACTS_BLOOD_DROPS);
    for index in 0..BURST_DROPS {
        let angle = TAU * index as f32 / BURST_DROPS as f32;
        spawn_drop(
            world,
            texture,
            origin,
            BloodDrop {
                velocity: Vec2::from_angle(angle) * BURST_SPEED,
                drag: BURST_DRAG,
                age: 0.0,
                life: BURST_LIFE,
            },
        );
    }
}

/// Bloodlust heals for every enemy corpse that finished this frame (`Gone`), minions aside.
pub(crate) fn bloodlust_heals_on_kill(world: &mut World) {
    let kills = world
        .query::<()>()
        .with::<(&Enemy, &Gone)>()
        .without::<&Minion>()
        .iter()
        .count() as i32;
    if kills == 0 {
        return;
    }
    for health in world
        .query_mut::<&mut Health>()
        .with::<(&Player, &Bloodlust)>()
    {
        health.current = (health.current + KILL_HEAL * kills).min(health.max);
    }
}

pub(crate) fn spawn_bloodlust_drops(world: &mut World, textures: &Textures, dt: f32) {
    let Some((origin, remaining)) = world
        .query::<(&Position, &Bloodlust)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(position, bloodlust)| (position.0, bloodlust.remaining))
    else {
        return;
    };
    let now = BLOODLUST_SECONDS - remaining;
    let count = (now * DROP_RATE).floor() - ((now - dt) * DROP_RATE).floor();
    let texture = textures.get(tex::ARTEFACTS_BLOOD_DROPS);
    for index in 0..count.max(0.0) as usize {
        let seed = now * 7.31 + index as f32 * 3.7;
        let angle = fraction(seed) * TAU;
        let at = origin + Vec2::from_angle(angle) * DROP_RADIUS * (0.4 + fraction(seed + 1.0));
        spawn_drop(
            world,
            texture,
            at,
            BloodDrop {
                velocity: Vec2::new((fraction(seed + 2.0) - 0.5) * 12.0, -DROP_RISE),
                drag: 0.0,
                age: 0.0,
                life: DROP_LIFE * (0.8 + 0.4 * fraction(seed + 3.0)),
            },
        );
    }
}

pub(crate) fn tick_blood_drops(world: &mut World, dt: f32) {
    let mut gone = Vec::new();
    for (entity, drop, position, color, sprite) in world.query_mut::<(
        Entity,
        &mut BloodDrop,
        &mut Position,
        &mut Color,
        &mut Sprite,
    )>() {
        drop.age += dt;
        position.0 += drop.velocity * dt;
        drop.velocity *= (1.0 - drop.drag * dt).max(0.0);
        let progress = (drop.age / drop.life).clamp(0.0, 1.0);
        let frame = ((progress * DROP_FRAMES as f32) as usize).min(DROP_FRAMES - 1);
        sprite.uv = Sprite::cell_uv(frame, 0, DROP_FRAMES, 1);
        color.0.w = (progress * PI).sin().sqrt().min(1.0);
        if drop.age >= drop.life {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

pub(crate) fn pulse_bloodlust(world: &mut World) {
    let mut fresh = Vec::new();
    for (entity, bloodlust, color, tint) in world
        .query_mut::<(Entity, &Bloodlust, &mut Color, Option<&BloodTint>)>()
        .with::<&Player>()
    {
        let original = tint.map_or(color.0.truncate(), |tint| tint.original);
        if tint.is_none() {
            fresh.push((entity, original));
        }
        let elapsed = BLOODLUST_SECONDS - bloodlust.remaining;
        let pulse = 0.5 - 0.5 * (TAU * PULSE_HZ * elapsed).cos();
        let tinted = original * Vec3::ONE.lerp(PULSE_PEAK, pulse);
        color.0 = tinted.extend(color.0.w);
    }
    for (entity, original) in fresh {
        let _ = world.insert_one(entity, BloodTint { original });
    }
    let mut over = Vec::new();
    for (entity, tint, color) in world
        .query_mut::<(Entity, &BloodTint, &mut Color)>()
        .without::<&Bloodlust>()
    {
        color.0 = tint.original.extend(color.0.w);
        over.push(entity);
    }
    for entity in over {
        let _ = world.remove_one::<BloodTint>(entity);
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{Animation, Expiring};
    use crate::enemies::EnemyKind;
    use crate::physics::Physics;
    use crate::prefabs;
    use crate::systems::tick_artefact_effects;

    use super::*;

    fn setup() -> (World, Physics, Entity) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let player = prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(300.0, 200.0),
            &Textures::default(),
        );
        (world, physics, player)
    }

    fn hurt(world: &mut World, player: Entity, current: i32) {
        for health in world.query_mut::<&mut Health>().with::<&Player>() {
            health.current = current;
        }
        let _ = player;
    }

    fn health(world: &World) -> (i32, i32) {
        world
            .query::<&Health>()
            .with::<&Player>()
            .iter()
            .next()
            .map_or((-1, -1), |health| (health.current, health.max))
    }

    fn kill_a_rat(world: &mut World, physics: &mut Physics) {
        let rat = prefabs::ready_enemy(world, physics, EnemyKind::Rat, Vec2::new(900.0, 900.0));
        let _ = world.insert_one(rat, Expiring);
        if let Ok(mut animation) = world.get::<&mut Animation>(rat) {
            animation.play(EnemyKind::Rat.spec().death_clip());
            animation.advance(5.0);
        }
        crate::systems::mark_gone(world);
    }

    fn start_bloodlust(world: &mut World, player: Entity, remaining: f32) {
        let _ = world.insert_one(player, Bloodlust { remaining });
    }

    #[test]
    fn healing_a_percentage_rounds_up_and_is_capped_at_the_maximum() {
        let (mut world, _, player) = setup();
        let max = health(&world).1;
        hurt(&mut world, player, 1);
        let _ = world.insert_one(player, HealPercent(0.5));
        let mut sounds = Vec::new();

        apply_heal_percent(&mut world, &Textures::default(), &mut sounds);

        let expected = (1 + (max as f32 * 0.5).ceil() as i32).min(max);
        assert_eq!(health(&world).0, expected);
        assert_eq!(sounds, vec![AudioCommand::Effect(SoundEffect::Heal)]);
        assert!(world.get::<&HealPercent>(player).is_err());

        hurt(&mut world, player, max - 1);
        let _ = world.insert_one(player, HealPercent(0.5));
        apply_heal_percent(&mut world, &Textures::default(), &mut sounds);
        assert_eq!(health(&world).0, max);
    }

    #[test]
    fn activating_the_chalice_bursts_a_ring_of_blood() {
        let (mut world, _, player) = setup();
        let _ = world.insert_one(player, HealPercent(0.5));

        apply_heal_percent(&mut world, &Textures::default(), &mut Vec::new());

        assert_eq!(world.query::<&BloodDrop>().iter().count(), BURST_DROPS);
    }

    #[test]
    fn bloodlust_heals_one_hit_point_per_kill_and_stops_after_it_expires() {
        let (mut world, mut physics, player) = setup();
        hurt(&mut world, player, 2);
        start_bloodlust(&mut world, player, 0.5);
        kill_a_rat(&mut world, &mut physics);

        bloodlust_heals_on_kill(&mut world);
        assert_eq!(health(&world).0, 3);

        tick_artefact_effects(&mut world, 1.0);
        bloodlust_heals_on_kill(&mut world);
        assert_eq!(health(&world).0, 3);
    }

    #[test]
    fn kills_do_not_heal_without_bloodlust() {
        let (mut world, mut physics, player) = setup();
        hurt(&mut world, player, 2);
        kill_a_rat(&mut world, &mut physics);

        bloodlust_heals_on_kill(&mut world);

        assert_eq!(health(&world).0, 2);
    }

    #[test]
    fn the_player_colour_pulses_red_and_is_restored_exactly_afterwards() {
        let (mut world, _, player) = setup();
        let before = world.get::<&Color>(player).map(|color| color.0).ok();
        start_bloodlust(&mut world, player, BLOODLUST_SECONDS);
        pulse_bloodlust(&mut world);
        assert_eq!(
            world.get::<&Color>(player).map(|color| color.0).ok(),
            before
        );

        let peak_time = 1.0 / (2.0 * PULSE_HZ);
        start_bloodlust(&mut world, player, BLOODLUST_SECONDS - peak_time);
        pulse_bloodlust(&mut world);
        let peak = world
            .get::<&Color>(player)
            .map_or(Vec4::ZERO, |color| color.0);
        assert!((peak.y - PULSE_PEAK.y).abs() < 1e-4);
        assert_eq!(peak.w, before.map_or(0.0, |color| color.w));

        let _ = world.remove_one::<Bloodlust>(player);
        pulse_bloodlust(&mut world);
        assert_eq!(
            world.get::<&Color>(player).map(|color| color.0).ok(),
            before
        );
        assert!(world.get::<&BloodTint>(player).is_err());
    }

    #[test]
    fn the_pulse_leaves_the_dodge_alpha_alone() {
        let (mut world, _, player) = setup();
        start_bloodlust(&mut world, player, BLOODLUST_SECONDS - 0.1);
        for color in world.query_mut::<&mut Color>().with::<&Player>() {
            color.0.w = 0.0;
        }

        pulse_bloodlust(&mut world);

        assert_eq!(
            world.get::<&Color>(player).map_or(1.0, |color| color.0.w),
            0.0
        );
    }

    #[test]
    fn droplets_rise_around_the_player_while_bloodlust_lasts_and_then_fade_away() {
        let (mut world, _, player) = setup();
        start_bloodlust(&mut world, player, BLOODLUST_SECONDS - 0.5);

        spawn_bloodlust_drops(&mut world, &Textures::default(), 0.5);
        let spawned = world.query::<&BloodDrop>().iter().count();
        assert!(spawned > 0);

        tick_blood_drops(&mut world, 2.0);
        assert_eq!(world.query::<&BloodDrop>().iter().count(), 0);

        let _ = world.remove_one::<Bloodlust>(player);
        spawn_bloodlust_drops(&mut world, &Textures::default(), 0.5);
        assert_eq!(world.query::<&BloodDrop>().iter().count(), 0);
    }
}
