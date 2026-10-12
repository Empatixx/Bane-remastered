use hecs::World;

use crate::assets::{Textures, PLAYER, PLAYER_DODGE};
use crate::components::{Animation, Clip, Dodge, Player, Sprite};
use crate::input::Input;

const FRAME_TIME: f32 = 0.1;
const FRAMES: usize = 6;

pub(crate) const PLAYER_COLUMNS: usize = 6;
pub(crate) const PLAYER_ROWS: usize = 4;
const DODGE_ROWS: usize = 1;

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

fn dodge_clip(direction_x: f32) -> Clip {
    Clip {
        row: 0,
        frames: FRAMES,
        frame_time: super::DODGE_SECONDS / FRAMES as f32,
        looping: false,
        flipped: direction_x < 0.0,
    }
}

pub(crate) fn animate_player(world: &mut World, input: &Input, textures: &Textures) {
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
    let walking_texture = textures.get(PLAYER);
    let dodging_texture = textures.get(PLAYER_DODGE);
    for (animation, sprite, dodge) in world
        .query_mut::<(&mut Animation, &mut Sprite, Option<&Dodge>)>()
        .with::<&Player>()
    {
        animation.columns = PLAYER_COLUMNS;
        if let Some(dodge) = dodge {
            sprite.texture = dodging_texture;
            animation.rows = DODGE_ROWS;
            animation.switch_to(dodge_clip(dodge.direction.x));
        } else {
            sprite.texture = walking_texture;
            animation.rows = PLAYER_ROWS;
            animation.switch_to(clip);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_dodge_clip_plays_once_and_flips_when_going_left() {
        assert!(!dodge_clip(1.0).flipped);
        assert!(dodge_clip(-1.0).flipped);
        assert!(!dodge_clip(1.0).looping);
        assert_eq!(dodge_clip(1.0).frames, 6);
    }
}
