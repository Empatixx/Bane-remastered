use hecs::World;

use crate::components::{Animation, Player};
use crate::input::Input;
use crate::sheets::player::{BOTTOM, IDLE, LEFT, RIGHT, TOP};

pub(crate) fn animate_player(world: &mut World, input: &Input) {
    let direction = input.direction();
    let clip = if direction.x < 0.0 {
        LEFT
    } else if direction.x > 0.0 {
        RIGHT
    } else if direction.y < 0.0 {
        TOP
    } else if direction.y > 0.0 {
        BOTTOM
    } else {
        IDLE
    };
    for animation in world.query_mut::<&mut Animation>().with::<&Player>() {
        animation.switch_to(clip);
    }
}
