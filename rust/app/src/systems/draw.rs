use glam::Vec2;
use hecs::World;

use crate::components::{
    Color, Hidden, InRange, Layer, Position, Prompt, ScreenSpace, Size, Sprite, StatusBar, Text,
};
use crate::render::{Draw, Fonts, Quad, TextureId};

use super::hud::bar_draws;

pub(crate) fn collect_draws(world: &World, fonts: &Fonts, camera: Vec2) -> Vec<Draw> {
    let mut layered: Vec<(Layer, Draw)> = Vec::new();
    for (position, size, color, sprite, layer, screen) in world
        .query::<(
            &Position,
            &Size,
            &Color,
            Option<&Sprite>,
            Option<&Layer>,
            Option<&ScreenSpace>,
        )>()
        .without::<&Hidden>()
        .iter()
    {
        let center = position.0 - offset(screen, camera);
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
        layered.push((layer.copied().unwrap_or(Layer(0)), draw));
    }
    for (position, text, layer, screen) in world
        .query::<(&Position, &Text, Option<&Layer>, Option<&ScreenSpace>)>()
        .without::<&Hidden>()
        .iter()
    {
        let layer = layer.copied().unwrap_or(Layer(0));
        let point = position.0 - offset(screen, camera);
        for draw in
            fonts
                .get(text.size)
                .draw(&text.content, point, text.align, text.anchor, text.color)
        {
            layered.push((layer, draw));
        }
    }
    for (position, size, bar) in world.query::<(&Position, &Size, &StatusBar)>().iter() {
        for draw in bar_draws(position.0, size.0, bar) {
            layered.push((Layer(5), draw));
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

fn offset(screen: Option<&ScreenSpace>, camera: Vec2) -> Vec2 {
    if screen.is_some() {
        Vec2::ZERO
    } else {
        camera
    }
}
