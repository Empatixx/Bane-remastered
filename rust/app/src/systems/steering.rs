use hecs::World;

use crate::components::{Health, Player, Velocity};
use crate::input::Input;

const PLAYER_SPEED: f32 = 411.0;

pub(crate) fn steer_player(world: &mut World, input: &Input) {
    let direction = input.direction();
    for (velocity, health) in world
        .query_mut::<(&mut Velocity, &Health)>()
        .with::<&Player>()
    {
        velocity.0 = if health.current > 0 {
            direction * PLAYER_SPEED
        } else {
            glam::Vec2::ZERO
        };
    }
}
