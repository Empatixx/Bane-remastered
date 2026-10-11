use hecs::World;

use crate::components::{Color, Position, Size};
use crate::render::Quad;

pub(crate) fn collect_quads(world: &World) -> Vec<Quad> {
    world
        .query::<(&Position, &Size, &Color)>()
        .iter()
        .map(|(position, size, color)| Quad::new(position.0, size.0, color.0))
        .collect()
}
