use hecs::World;

use crate::components::{Animation, Clip, Player};
use crate::input::Input;

const FRAME_TIME: f32 = 0.1;
const FRAMES: usize = 6;

pub(crate) const PLAYER_COLUMNS: usize = 6;
pub(crate) const PLAYER_ROWS: usize = 4;

pub(crate) const IDLE: Clip = clip(0, false);
const RIGHT: Clip = clip(1, false);
const LEFT: Clip = clip(1, true);
const BOTTOM: Clip = clip(2, false);
const TOP: Clip = clip(3, false);

const fn clip(row: usize, flipped: bool) -> Clip {
    Clip {
        row,
        frames: FRAMES,
        frame_time: FRAME_TIME,
        looping: true,
        flipped,
    }
}

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
