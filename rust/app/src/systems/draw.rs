use glam::{Vec2, Vec4};
use hecs::World;

use crate::components::{
    Afterimage, Casing, Chilled, Color, DodgeVisual, DropShadow, EnemyArrow, Frozen, Heading, Hidden,
    HitFlash,
    Hover, InRange, Layer, Liquid, Lit, MapSize, Minimap, Outline, Overlay, Phantom, Player,
    Position, Previous, Prompt, ScreenSpace, Shine, Size, Sky, Sprite, StatusBar, Text, YSort,
    blended,
};
use crate::render::{Draw, Fonts, Layers, Quad, TextureId};

use crate::viewport::TARGET;

/// How red the player turns when hurt, fading over this long.
const HURT_TINT: Vec4 = Vec4::new(1.0, 0.35, 0.35, 1.0);
const HURT_TINT_SECONDS: f32 = 0.35;

const FROST_RIM: Vec4 = Vec4::new(0.7, 0.95, 1.0, 0.9);

use super::hud::bar_draws;
use super::lightning::Bolt;
use super::minimap::minimap_draws;
use super::shadow::shadow_draw;

#[derive(Default)]
struct Sorted {
    scene: Vec<(Layer, Draw)>,
    /// What stands in [`Layer::ACTORS`], with the height of its feet.
    actors: Vec<(f32, Draw)>,
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
            list.into_iter().map(|(_, draw)| draw).collect::<Vec<_>>()
        };
        let mut scene = self.scene;
        scene.sort_by_key(|(layer, _)| *layer);
        // The standing things go after everything else in their layer, back to front. A few
        // hundred entries sort in microseconds, and since the sprites share one atlas the
        // order costs no extra draw calls.
        let mut actors = self.actors;
        actors.sort_by(|a, b| a.0.total_cmp(&b.0));
        let at = scene.partition_point(|(layer, _)| *layer <= Layer::ACTORS);
        let mut draws: Vec<Draw> = Vec::with_capacity(scene.len() + actors.len());
        let mut rest = scene.into_iter().map(|(_, draw)| draw);
        draws.extend(rest.by_ref().take(at));
        draws.extend(actors.into_iter().map(|(_, draw)| draw));
        draws.extend(rest);
        Layers {
            scene: draws,
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
        (shine, heading),
        (sky, standing, hit),
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
            (Option<&Shine>, Option<&Heading>),
            (Option<&Sky>, Option<&YSort>, Option<&HitFlash>),
            (
                Option<&Liquid>,
                Option<hecs::Or<&Player, hecs::Or<&DodgeVisual, &Afterimage>>>,
            ),
        )>()
        .without::<&Hidden>()
        .without::<&EnemyArrow>()
        .without::<&Casing>()
        .without::<&Bolt>()
        .iter()
    {
        let bob = Vec2::new(0.0, hover.map_or(0.0, Hover::offset));
        let center = blended(position.0, previous, blend) - offset(screen, camera) + bob;
        if screen.is_none() && off_screen(center, size.0) {
            continue;
        }
        let draw = match sprite {
            Some(sprite) => {
                // The player flushes red for a moment when hurt, over the red outline every hit
                // gives, so a hit can't be missed.
                let tint = match (hit, cloakable) {
                    (Some(hit), Some(_)) => {
                        let strength = (hit.remaining / HURT_TINT_SECONDS).clamp(0.0, 1.0);
                        Vec4::ONE.lerp(HURT_TINT, strength)
                    }
                    _ => Vec4::ONE,
                };
                let mut quad = Quad::with_uv(center, size.0, color.0 * tint, sprite.uv).lit(lit.is_some());
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
                if let Some(heading) = heading {
                    quad = quad.rotated(heading.0);
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
        match standing {
            Some(standing) if screen.is_none() && overlay.is_none() => {
                sorted.actors.push((center.y - bob.y + standing.foot, draw));
            }
            _ => sorted
                .bucket(screen, overlay)
                .push((layer.copied().unwrap_or(Layer(0)), draw)),
        }
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
    for (position, size, color, sprite, layer, bolt) in world
        .query::<(&Position, &Size, &Color, &Sprite, &Layer, &Bolt)>()
        .iter()
    {
        let center = position.0 - camera;
        if off_screen(center, size.0) {
            continue;
        }
        sorted.scene.push((
            *layer,
            Draw {
                texture: sprite.texture,
                quad: Quad::with_uv(center, size.0, color.0, sprite.uv).rotated(bolt.angle),
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
    for (position, previous, size, sprite, shadow, (layer, standing)) in world
        .query::<(
            &Position,
            Option<&Previous>,
            &Size,
            &Sprite,
            &DropShadow,
            (Option<&Layer>, Option<&YSort>),
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
            let own = if standing.is_some() {
                Layer::ACTORS.0
            } else {
                layer.map_or(0, |layer| layer.0)
            };
            let under = own - 1;
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
        let text = prompt_text(prompt.text, interact_key);
        for draw in line.draw_centered(&text, center, prompt.color) {
            sorted.scene.push((Layer(3), draw));
        }
    }
    sorted.finish()
}

/// Prompts are written for the default key ("Press E to ..."); the shown text names the key
/// Interact is bound to right now.
fn prompt_text(text: &str, interact_key: &str) -> String {
    match text.strip_prefix("Press E ") {
        Some(rest) => format!("Press {interact_key} {rest}"),
        None => text.to_owned(),
    }
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

    #[test]
    fn interaction_prompts_name_the_key_interact_is_bound_to() {
        assert_eq!(
            prompt_text("Press E to enter the portal", "X"),
            "Press X to enter the portal"
        );
        assert_eq!(prompt_text("Press E to buy", "Space"), "Press Space to buy");
        assert_eq!(prompt_text("Sold out", "X"), "Sold out");
    }

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

    /// Spawns a sprite tagged by its red channel, so the draw order can be read back.
    fn tagged(world: &mut World, tag: f32, at: Vec2, layer: i32) -> hecs::Entity {
        world.spawn((
            Position(at),
            Size(Vec2::splat(32.0)),
            Color(glam::Vec4::new(tag, 1.0, 1.0, 1.0)),
            Sprite::whole(TextureId::WHITE),
            Layer(layer),
        ))
    }

    fn order(world: &World) -> Vec<f32> {
        collect_draws(world, &Fonts::default(), Vec2::ZERO, "E", 1.0)
            .scene
            .iter()
            .map(|draw| draw.quad.red())
            .collect()
    }

    #[test]
    fn standing_things_are_drawn_back_to_front_by_their_feet() {
        let mut world = World::new();
        let front = tagged(&mut world, 0.1, Vec2::new(100.0, 300.0), 1);
        let back = tagged(&mut world, 0.2, Vec2::new(100.0, 200.0), 1);
        // A short barrel whose feet are lower than the tall statue's, though its centre is higher.
        let barrel = tagged(&mut world, 0.3, Vec2::new(200.0, 250.0), -1);
        let statue = tagged(&mut world, 0.4, Vec2::new(200.0, 260.0), -1);
        let _ = world.insert_one(front, YSort { foot: 16.0 });
        let _ = world.insert_one(back, YSort { foot: 16.0 });
        let _ = world.insert_one(barrel, YSort { foot: 30.0 });
        let _ = world.insert_one(statue, YSort { foot: 10.0 });

        assert_eq!(order(&world), vec![0.2, 0.4, 0.3, 0.1]);
    }

    #[test]
    fn standing_things_share_the_actor_layer_between_the_floor_and_the_bullets() {
        let mut world = World::new();
        tagged(&mut world, 0.5, Vec2::new(100.0, 100.0), 2);
        let barrel = tagged(&mut world, 0.6, Vec2::new(100.0, 100.0), -1);
        let _ = world.insert_one(barrel, YSort { foot: 16.0 });
        tagged(&mut world, 0.7, Vec2::new(100.0, 100.0), -1);
        tagged(&mut world, 0.8, Vec2::new(100.0, 100.0), 1);

        assert_eq!(order(&world), vec![0.7, 0.8, 0.6, 0.5]);
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
