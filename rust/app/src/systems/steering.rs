use hecs::World;

use crate::components::{Cobweb, Health, Player, Position, Size, Velocity};
use crate::geometry;
use crate::input::Input;

const PLAYER_SPEED: f32 = 411.0;
const COBWEB_SLOWDOWN: f32 = 0.5;

pub(crate) fn steer_player(world: &mut World, input: &Input) {
    let direction = input.direction();
    let webs: Vec<(glam::Vec2, glam::Vec2)> = world
        .query::<(&Position, &Size)>()
        .with::<&Cobweb>()
        .iter()
        .map(|(position, size)| (position.0, size.0))
        .collect();
    for (velocity, health, position, size) in world
        .query_mut::<(&mut Velocity, &Health, &Position, &Size)>()
        .with::<&Player>()
    {
        let stuck = webs
            .iter()
            .any(|(web, web_size)| geometry::overlaps(position.0, size.0, *web, *web_size));
        let speed = if stuck {
            PLAYER_SPEED * COBWEB_SLOWDOWN
        } else {
            PLAYER_SPEED
        };
        velocity.0 = if health.current > 0 {
            direction * speed
        } else {
            glam::Vec2::ZERO
        };
    }
}
