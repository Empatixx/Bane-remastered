use hecs::World;

use crate::components::{Player, Velocity};
use crate::input::Input;

const PLAYER_SPEED: f32 = 240.0;

pub(crate) fn steer_player(world: &mut World, input: &Input) {
    let direction = input.direction();
    for velocity in world.query_mut::<&mut Velocity>().with::<&Player>() {
        velocity.0 = direction * PLAYER_SPEED;
    }
}
