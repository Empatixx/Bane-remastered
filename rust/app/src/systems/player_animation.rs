use glam::Vec2;
use hecs::World;

use crate::components::{Animation, Player, Position};
use crate::input::Input;
use crate::sheets::player::{
    IDLE, RUN_DOWN, RUN_LEFT, RUN_RIGHT, RUN_UP, STAND_LEFT, STAND_RIGHT, STAND_UP,
};

/// Which way the hero looks: where he aims, in four quarters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Facing {
    Down,
    Up,
    Right,
    Left,
}

pub(crate) fn facing(aim: Vec2) -> Facing {
    if aim.y.abs() > aim.x.abs() {
        if aim.y < 0.0 { Facing::Up } else { Facing::Down }
    } else if aim.x < 0.0 {
        Facing::Left
    } else {
        Facing::Right
    }
}

/// The hero faces the cursor and runs (or stands breathing) that way, whichever way he moves.
pub(crate) fn animate_player(world: &mut World, input: &Input, camera: Vec2) {
    let target = input.mouse() + camera;
    let moving = input.direction() != Vec2::ZERO;
    for (position, animation) in world.query_mut::<(&Position, &mut Animation)>().with::<&Player>() {
        let clip = match (facing(target - position.0), moving) {
            (Facing::Right, true) => RUN_RIGHT,
            (Facing::Left, true) => RUN_LEFT,
            (Facing::Down, true) => RUN_DOWN,
            (Facing::Up, true) => RUN_UP,
            (Facing::Right, false) => STAND_RIGHT,
            (Facing::Left, false) => STAND_LEFT,
            (Facing::Down, false) => IDLE,
            (Facing::Up, false) => STAND_UP,
        };
        animation.switch_to(clip);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_hero_faces_the_quarter_the_aim_points_into() {
        assert_eq!(facing(Vec2::new(5.0, 1.0)), Facing::Right);
        assert_eq!(facing(Vec2::new(-5.0, 1.0)), Facing::Left);
        assert_eq!(facing(Vec2::new(1.0, 5.0)), Facing::Down);
        assert_eq!(facing(Vec2::new(1.0, -5.0)), Facing::Up);
    }
}
