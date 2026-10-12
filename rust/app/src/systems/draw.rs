use glam::Vec2;
use hecs::World;

use crate::components::{
    Color, Hidden, InRange, Layer, Lit, MapSize, Minimap, Overlay, PlayerShadow, Position, Prompt,
    ScreenSpace, Size, Sprite, StatusBar, Text,
};
use crate::render::{Draw, Fonts, Layers, Quad, TextureId};

use super::hud::bar_draws;
use super::minimap::minimap_draws;
use super::shadow::shadow_draw;

#[derive(Default)]
struct Sorted {
    scene: Vec<(Layer, Draw)>,
    hud: Vec<(Layer, Draw)>,
    overlay: Vec<(Layer, Draw)>,
}

impl Sorted {
    fn bucket(
        &mut self,
        screen: Option<&ScreenSpace>,
        overlay: Option<&Overlay>,
    ) -> &mut Vec<(Layer, Draw)> {
        if overlay.is_some() {
            &mut self.overlay
        } else if screen.is_some() {
            &mut self.hud
        } else {
            &mut self.scene
        }
    }

    fn finish(self) -> Layers {
        let strip = |mut list: Vec<(Layer, Draw)>| {
            list.sort_by_key(|(layer, _)| *layer);
            list.into_iter().map(|(_, draw)| draw).collect()
        };
        Layers {
            scene: strip(self.scene),
            hud: strip(self.hud),
            overlay: strip(self.overlay),
        }
    }
}

pub(crate) fn collect_draws(
    world: &World,
    fonts: &Fonts,
    camera: Vec2,
    interact_key: &str,
) -> Layers {
    let mut sorted = Sorted::default();
    for (position, size, color, sprite, layer, screen, overlay, lit) in world
        .query::<(
            &Position,
            &Size,
            &Color,
            Option<&Sprite>,
            Option<&Layer>,
            Option<&ScreenSpace>,
            Option<&Overlay>,
            Option<&Lit>,
        )>()
        .without::<&Hidden>()
        .iter()
    {
        let center = position.0 - offset(screen, camera);
        let draw = match sprite {
            Some(sprite) => Draw {
                texture: sprite.texture,
                quad: Quad::with_uv(center, size.0, color.0, sprite.uv).lit(lit.is_some()),
            },
            None => Draw {
                texture: TextureId::WHITE,
                quad: Quad::new(center, size.0, color.0),
            },
        };
        sorted
            .bucket(screen, overlay)
            .push((layer.copied().unwrap_or(Layer(0)), draw));
    }
    for (position, text, layer, screen, overlay) in world
        .query::<(
            &Position,
            &Text,
            Option<&Layer>,
            Option<&ScreenSpace>,
            Option<&Overlay>,
        )>()
        .without::<&Hidden>()
        .iter()
    {
        let layer = layer.copied().unwrap_or(Layer(0));
        let point = position.0 - offset(screen, camera);
        let draws =
            fonts
                .get(text.size)
                .draw(&text.content, point, text.align, text.anchor, text.color);
        let bucket = sorted.bucket(screen, overlay);
        for draw in draws {
            bucket.push((layer, draw));
        }
    }
    for map in world.query::<&Minimap>().without::<&Hidden>().iter() {
        let layer = if map.size == MapSize::Big { 7 } else { 5 };
        for draw in minimap_draws(map) {
            sorted.hud.push((Layer(layer), draw));
        }
    }
    for (position, size, sprite, shadow) in world
        .query::<(&Position, &Size, &Sprite, &PlayerShadow)>()
        .without::<&Hidden>()
        .iter()
    {
        if let Some(mut draw) = shadow_draw(position.0 - camera, size.0, sprite, shadow) {
            draw.quad.set_lit(false);
            sorted.scene.push((Layer(0), draw));
        }
    }
    for (position, size, bar, screen) in world
        .query::<(&Position, &Size, &StatusBar, Option<&ScreenSpace>)>()
        .iter()
    {
        let bucket = sorted.bucket(screen, None);
        for draw in bar_draws(position.0, size.0, bar) {
            bucket.push((Layer(5), draw));
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
        let text = match prompt.text.strip_prefix("Press E ") {
            Some(rest) => format!("Press {interact_key} {rest}"),
            None => prompt.text.to_owned(),
        };
        for draw in line.draw_centered(&text, center, prompt.color) {
            sorted.scene.push((Layer(3), draw));
        }
    }
    sorted.finish()
}

fn offset(screen: Option<&ScreenSpace>, camera: Vec2) -> Vec2 {
    if screen.is_some() {
        Vec2::ZERO
    } else {
        camera
    }
}
