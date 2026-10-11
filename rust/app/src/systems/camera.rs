use glam::Vec2;
use hecs::World;

use crate::components::{Player, Position};
use crate::viewport::TARGET;

pub(crate) fn follow_player(world: &World, bounds: Vec2) -> Vec2 {
    let max = (bounds - TARGET).max(Vec2::ZERO);
    world
        .query::<&Position>()
        .with::<&Player>()
        .iter()
        .next()
        .map_or(Vec2::ZERO, |position| {
            (position.0 - TARGET / 2.0).clamp(Vec2::ZERO, max).round()
        })
}
