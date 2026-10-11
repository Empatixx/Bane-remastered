use glam::Vec2;
use hecs::World;

use crate::components::{Player, Position, Size};
use crate::viewport::TARGET;

const MOUSE_PULL: f32 = 10.0;

pub(crate) fn camera_target(world: &World, mouse: Vec2) -> Option<Vec2> {
    let (position, size) = world
        .query::<(&Position, &Size)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(position, size)| (position.0, size.0))?;
    let top_left = position - size / 2.0;
    Some(top_left + size - TARGET / 2.0 + (mouse - TARGET / 2.0) / MOUSE_PULL)
}
