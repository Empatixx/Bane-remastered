use glam::{Vec2, Vec4};
use hecs::World;

use crate::components::{
    Afterimage, Casing, Chilled, Color, DodgeVisual, DropShadow, EnemyArrow, Frozen, Hidden, Hover,
    InRange, Layer, Liquid, Lit, MapSize, Minimap, Outline, Overlay, Phantom, Player, Position,
    Previous, Prompt, ScreenSpace, Shine, Size, Sky, Sprite, StatusBar, Text, blended,
};
use crate::render::{Draw, Fonts, Layers, Quad, TextureId};

use crate::viewport::TARGET;

const FROST_RIM: Vec4 = Vec4::new(0.7, 0.95, 1.0, 0.9);

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
    let cloak = world
        .query::<&Phantom>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|phantom| phantom.remaining);
    for (
        position,
        previous,
        size,
        color,
        sprite,
        layer,
        screen,
        overlay,
        lit,
        outline,
        hover,
        frozen,
        shine,
        sky,
        (liquid, cloakable),
    ) in world
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
            (Option<&Frozen>, Option<&Chilled>),
            Option<&Shine>,
            Option<&Sky>,
            (
                Option<&Liquid>,
                Option<hecs::Or<&Player, hecs::Or<&DodgeVisual, &Afterimage>>>,
            ),
        )>()
        .without::<&Hidden>()
        .without::<&EnemyArrow>()
        .without::<&Casing>()
        .iter()
    {
        let bob = Vec2::new(0.0, hover.map_or(0.0, Hover::offset));
        let center = blended(position.0, previous, blend) - offset(screen, camera) + bob;
        if screen.is_none() && off_screen(center, size.0) {
            continue;
        }
        let draw = match sprite {
            Some(sprite) => {
                let mut quad = Quad::with_uv(center, size.0, color.0, sprite.uv).lit(lit.is_some());
                if let Some(liquid) = liquid {
                    quad = quad.liquid(liquid.kind, liquid.time);
                }
                let frost = frozen
                    .0
                    .map_or(0.0, Frozen::strength)
                    .max(frozen.1.map_or(0.0, Chilled::strength));
                if frost > 0.0 {
                    quad = quad.frosted(frost).outlined(Vec4::new(
                        FROST_RIM.x,
                        FROST_RIM.y,
                        FROST_RIM.z,
                        FROST_RIM.w * frost,
                    ));
                }
                if let Some(shine) = shine {
                    quad = quad.shined(shine.0);
                }
                quad = outline.map_or(quad, |outline| quad.outlined(outline.0));
                if let (Some(time), Some(_)) = (cloak, cloakable) {
                    quad = quad.shadowed(time);
                }
                Draw {
                    texture: sprite.texture,
                    quad,
                }
            }
            None => {
                let quad = Quad::new(center, size.0, color.0);
                Draw {
                    texture: TextureId::WHITE,
                    quad: sky.map_or(quad, |sky| quad.underworld(sky.0)),
                }
            }
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
    for (position, size, color, sprite, casing) in world
        .query::<(&Position, &Size, &Color, &Sprite, &Casing)>()
        .without::<&Hidden>()
        .iter()
    {
        let center = position.0 - camera - Vec2::new(0.0, casing.height);
        if off_screen(center, size.0) {
            continue;
        }
        sorted.scene.push((
            Layer(-1),
            Draw {
                texture: sprite.texture,
                quad: Quad::with_uv(center, size.0, color.0, sprite.uv)
                    .lit(true)
                    .rotated(casing.angle),
            },
        ));
    }
    for (position, size, color, sprite, arrow) in world
        .query::<(&Position, &Size, &Color, &Sprite, &EnemyArrow)>()
        .without::<&Hidden>()
        .iter()
    {
        sorted.hud.push((
            Layer(4),
            Draw {
                texture: sprite.texture,
                quad: Quad::with_uv(position.0, size.0, color.0, sprite.uv).rotated(arrow.angle),
            },
        ));
    }
    for (position, previous, size, sprite, shadow, layer) in world
        .query::<(
            &Position,
            Option<&Previous>,
            &Size,
            &Sprite,
            &DropShadow,
            Option<&Layer>,
        )>()
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
            let under = layer.map_or(0, |layer| layer.0) - 1;
            sorted.scene.push((Layer(under), draw));
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
    if screen.is_some() { Vec2::ZERO } else { camera }
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

    fn sprite(world: &mut World, at: Vec2) -> hecs::Entity {
        world.spawn((
            Position(at),
            Size(Vec2::splat(32.0)),
            Color(glam::Vec4::ONE),
            Sprite::whole(TextureId::WHITE),
        ))
    }

    fn shadowed_count(world: &World) -> usize {
        collect_draws(world, &Fonts::default(), Vec2::ZERO, "E", 1.0)
            .scene
            .iter()
            .filter(|draw| draw.quad.is_shadowed())
            .count()
    }

    #[test]
    fn a_cloaked_player_and_its_dodge_pictures_are_drawn_as_shadows() {
        let mut world = World::new();
        let player = sprite(&mut world, Vec2::new(100.0, 100.0));
        let _ = world.insert_one(player, Player);
        let roll = sprite(&mut world, Vec2::new(140.0, 100.0));
        let _ = world.insert_one(roll, DodgeVisual);
        let echo = sprite(&mut world, Vec2::new(180.0, 100.0));
        let _ = world.insert_one(echo, Afterimage { remaining: 0.2 });
        sprite(&mut world, Vec2::new(220.0, 100.0));
        assert_eq!(shadowed_count(&world), 0);

        let _ = world.insert_one(player, Phantom { remaining: 3.0 });

        assert_eq!(shadowed_count(&world), 3);
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

    #[test]
    fn an_enemy_arrow_is_drawn_once_and_not_a_second_time_without_its_rotation() {
        let mut world = World::new();
        world.spawn((
            EnemyArrow { angle: 1.0 },
            Position(Vec2::new(40.0, 270.0)),
            Size(Vec2::splat(18.0)),
            Color(glam::Vec4::ONE),
            Sprite::whole(TextureId::WHITE),
            Layer(4),
            ScreenSpace,
        ));

        let layers = collect_draws(&world, &Fonts::default(), Vec2::ZERO, "E", 1.0);

        assert_eq!(layers.hud.len(), 1);
        assert_eq!(layers.scene.len(), 0);
    }

    #[test]
    fn a_casing_is_drawn_once_in_the_scene_and_lifted_by_its_height() {
        let mut world = World::new();
        world.spawn((
            Casing {
                velocity: Vec2::ZERO,
                height: 10.0,
                rise: 0.0,
                angle: 0.7,
                spin: 0.0,
                age: 0.0,
                settled: false,
            },
            Position(Vec2::new(300.0, 200.0)),
            Size(Vec2::new(12.0, 6.0)),
            Color(glam::Vec4::ONE),
            Sprite::whole(TextureId::WHITE),
            Layer(-1),
        ));

        let layers = collect_draws(&world, &Fonts::default(), Vec2::ZERO, "E", 1.0);

        assert_eq!(layers.scene.len(), 1);
        assert_eq!(layers.hud.len(), 0);
    }
}
