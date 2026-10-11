use glam::Vec2;
use hecs::World;

use crate::components::{Color, Layer, Position, Size, Sprite, Text};
use crate::render::{Draw, Fonts, Quad, TextureId};

pub(crate) fn collect_draws(world: &World, fonts: &Fonts, camera: Vec2) -> Vec<Draw> {
    let mut layered: Vec<(Layer, Draw)> = world
        .query::<(&Position, &Size, &Color, Option<&Sprite>, Option<&Layer>)>()
        .iter()
        .map(|(position, size, color, sprite, layer)| {
            let center = position.0 - camera;
            let draw = match sprite {
                Some(sprite) => Draw {
                    texture: sprite.texture,
                    quad: Quad::with_uv(center, size.0, color.0, sprite.uv),
                },
                None => Draw {
                    texture: TextureId::WHITE,
                    quad: Quad::new(center, size.0, color.0),
                },
            };
            (layer.copied().unwrap_or(Layer(0)), draw)
        })
        .collect();
    for (position, text, layer) in world.query::<(&Position, &Text, Option<&Layer>)>().iter() {
        let layer = layer.copied().unwrap_or(Layer(0));
        for draw in
            fonts
                .get(text.size)
                .draw_centered(&text.content, position.0 - camera, text.color)
        {
            layered.push((layer, draw));
        }
    }
    layered.sort_by_key(|(layer, _)| *layer);
    layered.into_iter().map(|(_, draw)| draw).collect()
}
