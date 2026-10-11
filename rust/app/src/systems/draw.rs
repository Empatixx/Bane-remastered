use hecs::World;

use crate::components::{Color, Layer, Position, Size, Sprite};
use crate::render::{Draw, Quad, TextureId};

pub(crate) fn collect_draws(world: &World) -> Vec<Draw> {
    let mut layered: Vec<(Layer, Draw)> = world
        .query::<(&Position, &Size, &Color, Option<&Sprite>, Option<&Layer>)>()
        .iter()
        .map(|(position, size, color, sprite, layer)| {
            let draw = Draw {
                texture: sprite.map_or(TextureId::WHITE, |sprite| sprite.0),
                quad: Quad::new(position.0, size.0, color.0),
            };
            (layer.copied().unwrap_or(Layer(0)), draw)
        })
        .collect();
    layered.sort_by_key(|(layer, _)| *layer);
    layered.into_iter().map(|(_, draw)| draw).collect()
}
