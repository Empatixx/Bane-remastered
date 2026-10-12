use std::f32::consts::{PI, TAU};

use glam::Vec2;
use hecs::{Entity, World};

use crate::components::{
    Animation, Body, Expiring, Floating, GazeWind, Light, LightFade, Particle, Position, Size,
};
use crate::physics::Physics;
use crate::prefabs;
use crate::render::TextureId;

const COOLDOWN: f32 = 0.2;
const MAX_PARTICLES: usize = 4;
const WOBBLE: f32 = 16.0;

pub(crate) fn spawn_gaze_wind(
    world: &mut World,
    physics: &Physics,
    texture: TextureId,
    dt: f32,
    clock: f32,
) {
    let mut particles = world.query::<&Particle>().iter().count();
    let mut spawn_at = Vec::new();
    for (position, size, body, gaze) in
        world.query_mut::<(&Position, &Size, &Body, &mut GazeWind)>()
    {
        gaze.cooldown = (gaze.cooldown - dt).max(0.0);
        let Some(velocity) = physics.velocity(*body) else {
            continue;
        };
        let running = velocity.x.abs() >= gaze.speed_limit || velocity.y.abs() >= gaze.speed_limit;
        if !running || gaze.cooldown > 0.0 || particles >= MAX_PARTICLES {
            continue;
        }
        gaze.cooldown = COOLDOWN;
        particles += 1;
        spawn_at.push(gaze_position(position.0, size.0, velocity, clock));
    }
    for center in spawn_at {
        prefabs::gaze_particle(world, center, texture);
    }
}

fn gaze_position(center: Vec2, size: Vec2, velocity: Vec2, clock: f32) -> Vec2 {
    let feet = Vec2::new(center.x, center.y + size.y / 2.0);
    let phase = clock.rem_euclid(1.0);
    let vertical_only = velocity.x.abs() < 1.0 && velocity.y.abs() >= 1.0;
    let horizontal_only = velocity.y.abs() < 1.0 && velocity.x.abs() >= 1.0;
    if vertical_only {
        Vec2::new(feet.x + WOBBLE * (TAU * phase).sin(), feet.y)
    } else if horizontal_only {
        Vec2::new(feet.x, feet.y + WOBBLE * (PI * (1.0 + phase)).sin())
    } else {
        feet
    }
}

pub(crate) fn show_change(world: &mut World, entity: Entity, change: i32, roll: f32) {
    let placement = world
        .query_one_mut::<(&Position, &Size)>(entity)
        .map(|(position, size)| (position.0, size.0));
    if let Ok((center, size)) = placement {
        let top_left = center - size / 2.0;
        let point = Vec2::new(top_left.x + roll * size.x, top_left.y + size.y / 5.0);
        prefabs::floating_number(world, change, point);
    }
}

pub(crate) fn float_texts(world: &mut World, dt: f32) {
    let mut gone = Vec::new();
    for (entity, floating, position) in world.query_mut::<(Entity, &mut Floating, &mut Position)>()
    {
        floating.remaining -= dt;
        position.0 += floating.velocity * dt;
        if floating.remaining <= 0.0 {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

pub(crate) fn fade_lights(world: &mut World) {
    for (light, fade, animation) in world
        .query_mut::<(&mut Light, &LightFade, &Animation)>()
        .with::<&Expiring>()
    {
        let progress = animation.frame() as f32 / animation.clip_frames().max(1) as f32;
        light.intensity = fade.base * (1.0 - progress);
    }
}

#[cfg(test)]
mod tests {
    use crate::assets::Textures;

    use super::*;

    fn running_player() -> (World, Physics) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let player = prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(200.0, 200.0),
            &Textures::default(),
        );
        if let Ok(body) = world.get::<&Body>(player).map(|body| *body) {
            physics.set_velocity(body, Vec2::new(411.0, 0.0));
            physics.step(1.0 / 60.0);
        }
        (world, physics)
    }

    fn count(world: &World) -> usize {
        world.query::<&Particle>().iter().count()
    }

    #[test]
    fn running_fast_spawns_a_puff_and_the_cooldown_spaces_them_out() {
        let (mut world, physics) = running_player();

        spawn_gaze_wind(&mut world, &physics, TextureId::WHITE, 0.0, 0.0);
        spawn_gaze_wind(&mut world, &physics, TextureId::WHITE, 0.1, 0.0);
        assert_eq!(count(&world), 1);

        spawn_gaze_wind(&mut world, &physics, TextureId::WHITE, 0.15, 0.0);
        assert_eq!(count(&world), 2);
    }

    #[test]
    fn standing_still_spawns_nothing() {
        let mut world = World::new();
        let mut physics = Physics::new();
        prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(200.0, 200.0),
            &Textures::default(),
        );
        physics.step(1.0 / 60.0);

        spawn_gaze_wind(&mut world, &physics, TextureId::WHITE, 1.0, 0.0);

        assert_eq!(count(&world), 0);
    }

    #[test]
    fn horizontal_running_puffs_appear_at_the_feet() {
        let position = gaze_position(
            Vec2::new(100.0, 100.0),
            Vec2::new(32.0, 76.0),
            Vec2::new(400.0, 0.0),
            0.0,
        );
        assert_eq!(position.x, 100.0);
        assert!((position.y - 138.0).abs() < 0.01);
    }

    #[test]
    fn a_hit_number_rises_for_half_a_second_and_disappears() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Rat,
            Vec2::new(300.0, 300.0),
        );

        show_change(&mut world, rat, -4, 0.5);
        let number = world
            .query::<(&Floating, &crate::components::Text, &Position)>()
            .iter()
            .map(|(_, text, position)| (text.content.clone(), position.0))
            .next();
        let (content, start) = number.expect("a number is shown");
        assert_eq!(content, "4");
        assert!((start.y - (300.0 - 27.0 + 11.0)).abs() < 1.0, "{start:?}");

        float_texts(&mut world, 0.25);
        let risen = world
            .query::<(&Floating, &Position)>()
            .iter()
            .map(|(_, position)| position.0.y)
            .next();
        assert!(risen.is_some_and(|y| y < start.y - 10.0));

        float_texts(&mut world, 0.3);
        assert_eq!(world.query::<&Floating>().iter().count(), 0);
    }

    #[test]
    fn a_heal_number_is_green_and_signed() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Rat,
            Vec2::new(300.0, 300.0),
        );

        show_change(&mut world, rat, 1, 0.0);

        let shown = world
            .query::<&crate::components::Text>()
            .with::<&Floating>()
            .iter()
            .map(|text| (text.content.clone(), text.color.y))
            .next();
        assert_eq!(shown, Some(("+1".to_owned(), 1.0)));
    }
}
