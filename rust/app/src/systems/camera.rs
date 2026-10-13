use glam::Vec2;
use hecs::World;

use crate::components::{Backdrop, Player, Position, Previous, Size, blended};
use crate::viewport::TARGET;

const MOUSE_PULL: f32 = 10.0;

pub(crate) fn camera_target(world: &World, mouse: Vec2, blend: f32) -> Option<Vec2> {
    let (position, size) = world
        .query::<(&Position, Option<&Previous>, &Size)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(position, previous, size)| (blended(position.0, previous, blend), size.0))?;
    let top_left = position - size / 2.0;
    Some(top_left + size - TARGET / 2.0 + (mouse - TARGET / 2.0) / MOUSE_PULL)
}

pub(crate) fn follow_backdrop(world: &mut World, camera: Vec2) {
    for position in world.query_mut::<&mut Position>().with::<&Backdrop>() {
        position.0 = camera + TARGET / 2.0;
    }
}
