use hecs::World;

use crate::artefacts::BERSERK_SPEED;
use crate::components::{
    Berserk, Cobweb, Dodge, Health, Player, PlayerPerks, Position, Size, Velocity,
};
use crate::geometry;
use crate::input::Input;

const PLAYER_SPEED: f32 = 411.0;
const COBWEB_SLOWDOWN: f32 = 0.5;
const PLAYER_DAMPING: f32 = 5.0;

fn accelerate(velocity: glam::Vec2, direction: glam::Vec2, top_speed: f32, dt: f32) -> glam::Vec2 {
    let keep = (1.0 - PLAYER_DAMPING * dt).max(0.0);
    let acceleration = top_speed * PLAYER_DAMPING / keep.max(f32::EPSILON);
    (velocity + direction * acceleration * dt) * keep
}

pub(crate) fn steer_player(world: &mut World, input: &Input, dt: f32) {
    let direction = input.direction();
    let webs: Vec<(glam::Vec2, glam::Vec2)> = world
        .query::<(&Position, &Size)>()
        .with::<&Cobweb>()
        .iter()
        .map(|(position, size)| (position.0, size.0))
        .collect();
    for (velocity, health, position, size, berserk, perks, dodge, frozen) in world
        .query_mut::<(
            &mut Velocity,
            &Health,
            &Position,
            &Size,
            Option<&Berserk>,
            Option<&PlayerPerks>,
            Option<&Dodge>,
            Option<&crate::components::Frozen>,
        )>()
        .with::<&Player>()
    {
        let stuck = webs
            .iter()
            .any(|(web, web_size)| geometry::overlaps(position.0, size.0, *web, *web_size));
        let boost = if berserk.is_some() {
            BERSERK_SPEED
        } else {
            1.0
        } * perks.map_or(1.0, |perks| perks.0.speed);
        let speed = if stuck {
            PLAYER_SPEED * COBWEB_SLOWDOWN
        } else {
            PLAYER_SPEED
        } * boost;
        velocity.0 = if let (Some(dodge), true) = (dodge, health.current > 0) {
            dodge.direction * super::DODGE_SPEED
        } else if health.current > 0 {
            let direction = if frozen.is_some() {
                glam::Vec2::ZERO
            } else {
                direction
            };
            accelerate(velocity.0, direction, speed, dt)
        } else {
            glam::Vec2::ZERO
        };
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use super::*;

    const STEP: f32 = 1.0 / 60.0;

    #[test]
    fn holding_a_direction_accelerates_towards_the_top_speed() {
        let mut velocity = Vec2::ZERO;
        let first = accelerate(velocity, Vec2::X, 411.0, STEP);
        assert!(first.x > 0.0 && first.x < 411.0 * 0.5, "{first}");
        for _ in 0..300 {
            velocity = accelerate(velocity, Vec2::X, 411.0, STEP);
        }
        assert!((velocity.x - 411.0).abs() < 1.0, "{velocity}");
    }

    #[test]
    fn letting_go_slides_to_a_stop_instead_of_halting_at_once() {
        let mut velocity = Vec2::new(411.0, 0.0);
        velocity = accelerate(velocity, Vec2::ZERO, 411.0, STEP);
        assert!(velocity.x > 300.0 && velocity.x < 411.0, "{velocity}");
        for _ in 0..120 {
            velocity = accelerate(velocity, Vec2::ZERO, 411.0, STEP);
        }
        assert!(velocity.x < 1.0, "{velocity}");
    }

    #[test]
    fn reversing_takes_longer_than_stopping() {
        let mut velocity = Vec2::new(411.0, 0.0);
        velocity = accelerate(velocity, -Vec2::X, 411.0, STEP);
        assert!(velocity.x > 0.0, "{velocity}");
    }
}
