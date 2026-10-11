use glam::Vec2;
use hecs::World;

use crate::components::{Color, InRange, Layer, Position, Prompt, Size, Sprite, Text};
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
    for (position, size, prompt) in world
        .query::<(&Position, &Size, &Prompt)>()
        .with::<&InRange>()
        .iter()
    {
        let line = fonts.get(prompt.size);
        let center = Vec2::new(
            position.0.x,
            position.0.y + size.0.y / 2.0 + prompt.gap + line.line_height() / 2.0,
        ) - camera;
        for draw in line.draw_centered(prompt.text, center, prompt.color) {
            layered.push((Layer(3), draw));
        }
    }
    layered.sort_by_key(|(layer, _)| *layer);
    layered.into_iter().map(|(_, draw)| draw).collect()
}
