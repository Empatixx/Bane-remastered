use hecs::World;

use crate::components::{Color, Layer, Position, Size, Sprite, Text};
use crate::render::{Draw, Font, Quad, TextureId};

pub(crate) fn collect_draws(world: &World, font: &Font) -> Vec<Draw> {
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
    for (position, text, layer) in world.query::<(&Position, &Text, Option<&Layer>)>().iter() {
        let layer = layer.copied().unwrap_or(Layer(0));
        for draw in font.draw_centered(&text.content, position.0, text.color) {
            layered.push((layer, draw));
        }
    }
    layered.sort_by_key(|(layer, _)| *layer);
    layered.into_iter().map(|(_, draw)| draw).collect()
}
