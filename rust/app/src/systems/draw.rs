use glam::Vec2;
use hecs::World;

use crate::components::{
    blended, Color, Hidden, Hover, InRange, Layer, Lit, MapSize, Minimap, Outline, Overlay,
    PlayerShadow, Position, Previous, Prompt, ScreenSpace, Size, Sprite, StatusBar, Text,
};
use crate::render::{Draw, Fonts, Layers, Quad, TextureId};

use crate::viewport::TARGET;

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
    blend: f32,
) -> Layers {
    let mut sorted = Sorted::default();
    for (position, previous, size, color, sprite, layer, screen, overlay, lit, outline, hover) in
        world
            .query::<(
                &Position,
                Option<&Previous>,
                &Size,
                &Color,
                Option<&Sprite>,
                Option<&Layer>,
                Option<&ScreenSpace>,
                Option<&Overlay>,
                Option<&Lit>,
                Option<&Outline>,
                Option<&Hover>,
            )>()
            .without::<&Hidden>()
            .iter()
    {
        let bob = Vec2::new(0.0, hover.map_or(0.0, Hover::offset));
        let center = blended(position.0, previous, blend) - offset(screen, camera) + bob;
        if screen.is_none() && off_screen(center, size.0) {
            continue;
        }
        let draw = match sprite {
            Some(sprite) => {
                let quad = Quad::with_uv(center, size.0, color.0, sprite.uv).lit(lit.is_some());
                Draw {
                    texture: sprite.texture,
                    quad: outline.map_or(quad, |outline| quad.outlined(outline.0)),
                }
            }
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
    for (position, previous, size, sprite, shadow) in world
        .query::<(&Position, Option<&Previous>, &Size, &Sprite, &PlayerShadow)>()
        .without::<&Hidden>()
        .iter()
    {
        if let Some(mut draw) = shadow_draw(
            blended(position.0, previous, blend) - camera,
            size.0,
            sprite,
            shadow,
        ) {
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

fn off_screen(center: Vec2, size: Vec2) -> bool {
    let reach = size.length() / 2.0;
    center.x + reach < 0.0
        || center.x - reach > TARGET.x
        || center.y + reach < 0.0
        || center.y - reach > TARGET.y
}

fn offset(screen: Option<&ScreenSpace>, camera: Vec2) -> Vec2 {
    if screen.is_some() {
        Vec2::ZERO
    } else {
        camera
    }
}

#[cfg(test)]
mod tests {
    use crate::components::Color;

    use super::*;

    fn square(world: &mut World, at: Vec2, screen: bool) {
        let entity = world.spawn((
            Position(at),
            Size(Vec2::splat(64.0)),
            Color(glam::Vec4::ONE),
        ));
        if screen {
            let _ = world.insert_one(entity, ScreenSpace);
        }
    }

    #[test]
    fn world_sprites_outside_the_camera_are_not_drawn() {
        let mut world = World::new();
        square(&mut world, Vec2::new(500.0, 300.0), false);
        square(&mut world, Vec2::new(5000.0, 300.0), false);
        square(&mut world, Vec2::new(500.0, -4000.0), false);

        let layers = collect_draws(&world, &Fonts::default(), Vec2::ZERO, "E", 1.0);

        assert_eq!(layers.scene.len(), 1);
    }

    #[test]
    fn the_camera_decides_what_is_visible_and_screen_space_is_never_culled() {
        let mut world = World::new();
        square(&mut world, Vec2::new(5000.0, 300.0), false);
        square(&mut world, Vec2::new(5000.0, 300.0), true);

        let away = collect_draws(&world, &Fonts::default(), Vec2::ZERO, "E", 1.0);
        let following = collect_draws(&world, &Fonts::default(), Vec2::new(4800.0, 0.0), "E", 1.0);

        assert_eq!((away.scene.len(), away.hud.len()), (0, 1));
        assert_eq!((following.scene.len(), following.hud.len()), (1, 1));
    }

    #[test]
    fn a_sprite_that_only_touches_the_edge_is_still_drawn() {
        let mut world = World::new();
        square(&mut world, Vec2::new(-20.0, 100.0), false);

        let layers = collect_draws(&world, &Fonts::default(), Vec2::ZERO, "E", 1.0);

        assert_eq!(layers.scene.len(), 1);
    }
}
