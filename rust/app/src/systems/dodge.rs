use glam::Vec2;
use hecs::{Entity, World};

use crate::assets::{Textures, PLAYER_DODGE};
use crate::components::{
    Afterimage, Animation, Clip, Color, Dodge, DodgeCooldown, DodgeVisual, Health, Invulnerable,
    Layer, Lit, Player, Position, Previous, Size, Sprite,
};
use crate::controls::Action;
use crate::input::Input;

pub(crate) const DODGE_SECONDS: f32 = 0.36;
pub(crate) const DODGE_SPEED: f32 = 760.0;
const DODGE_COOLDOWN: f32 = 0.9;
const VISUAL_SIZE: Vec2 = Vec2::new(64.0, 76.0);
const VISUAL_FRAMES: usize = 6;
const TRAIL_INTERVAL: f32 = 0.04;
const AFTERIMAGE_SECONDS: f32 = 0.25;
const AFTERIMAGE_COLOR: glam::Vec4 = glam::Vec4::new(0.55, 0.8, 1.0, 0.55);

pub(crate) fn start_dodge(world: &mut World, input: &Input, aim: Vec2) {
    if !input.action_pressed(Action::Dodge) {
        return;
    }
    let ready: Vec<(Entity, Vec2)> = world
        .query::<(Entity, &Position, &Health)>()
        .with::<&Player>()
        .without::<&Dodge>()
        .without::<&DodgeCooldown>()
        .iter()
        .filter(|(_, _, health)| health.current > 0)
        .map(|(entity, position, _)| (entity, position.0))
        .collect();
    for (entity, position) in ready {
        let direction = Some(input.direction())
            .filter(|direction| *direction != Vec2::ZERO)
            .unwrap_or_else(|| (aim - position).normalize_or(Vec2::X));
        let _ = world.insert_one(
            entity,
            Dodge {
                remaining: DODGE_SECONDS,
                direction,
                trail: 0.0,
            },
        );
        let remaining = world
            .get::<&Invulnerable>(entity)
            .map_or(DODGE_SECONDS, |current| {
                current.remaining.max(DODGE_SECONDS)
            });
        let _ = world.insert_one(entity, Invulnerable { remaining });
    }
}

pub(crate) fn tick_dodges(world: &mut World, dt: f32) {
    let mut finished = Vec::new();
    let mut trail = Vec::new();
    for (entity, dodge) in world.query_mut::<(Entity, &mut Dodge)>() {
        dodge.remaining -= dt;
        dodge.trail -= dt;
        if dodge.trail <= 0.0 {
            dodge.trail = TRAIL_INTERVAL;
            trail.push(entity);
        }
        if dodge.remaining <= 0.0 {
            finished.push(entity);
        }
    }
    for _ in trail {
        let copy = world
            .query::<(&Position, &Size, &Sprite, Option<&Layer>)>()
            .with::<&DodgeVisual>()
            .iter()
            .next()
            .map(|(position, size, sprite, layer)| {
                (
                    *position,
                    *size,
                    *sprite,
                    layer.copied().unwrap_or(Layer(0)),
                )
            });
        if let Some((position, size, sprite, layer)) = copy {
            world.spawn((
                position,
                size,
                sprite,
                Color(AFTERIMAGE_COLOR),
                Layer(layer.0 - 1),
                Afterimage {
                    remaining: AFTERIMAGE_SECONDS,
                },
            ));
        }
    }
    for entity in finished {
        let _ = world.remove_one::<Dodge>(entity);
        let _ = world.insert_one(
            entity,
            DodgeCooldown {
                remaining: DODGE_COOLDOWN,
            },
        );
    }
    let mut ready = Vec::new();
    for (entity, cooldown) in world.query_mut::<(Entity, &mut DodgeCooldown)>() {
        cooldown.remaining -= dt;
        if cooldown.remaining <= 0.0 {
            ready.push(entity);
        }
    }
    for entity in ready {
        let _ = world.remove_one::<DodgeCooldown>(entity);
    }
}

pub(crate) fn show_dodge_visual(world: &mut World, textures: &Textures) {
    let dodging: Option<(Entity, Position, Option<Previous>, f32, Layer)> = world
        .query::<(Entity, &Position, Option<&Previous>, &Dodge, Option<&Layer>)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(entity, position, previous, dodge, layer)| {
            (
                entity,
                *position,
                previous.copied(),
                dodge.direction.x,
                layer.copied().unwrap_or(Layer(0)),
            )
        });
    let existing: Vec<Entity> = world
        .query::<Entity>()
        .with::<&DodgeVisual>()
        .iter()
        .collect();
    let Some((player, position, previous, direction_x, layer)) = dodging else {
        for entity in existing {
            let _ = world.despawn(entity);
        }
        for color in world.query_mut::<&mut Color>().with::<&Player>() {
            color.0.w = 1.0;
        }
        return;
    };
    if let Ok(mut color) = world.get::<&mut Color>(player) {
        color.0.w = 0.0;
    }
    let clip = Clip {
        row: 0,
        frames: VISUAL_FRAMES,
        frame_time: DODGE_SECONDS / VISUAL_FRAMES as f32,
        looping: false,
        flipped: direction_x < 0.0,
    };
    if existing.is_empty() {
        let visual = world.spawn((
            DodgeVisual,
            position,
            Size(VISUAL_SIZE),
            Color(glam::Vec4::ONE),
            Sprite::whole(textures.get(PLAYER_DODGE)),
            Animation::new(VISUAL_FRAMES, 1, clip),
            layer,
            Lit,
        ));
        if let Some(previous) = previous {
            let _ = world.insert_one(visual, previous);
        }
        return;
    }
    for entity in existing {
        if let Ok(mut at) = world.get::<&mut Position>(entity) {
            *at = position;
        }
        match previous {
            Some(previous) => {
                let _ = world.insert_one(entity, previous);
            }
            None => {
                let _ = world.remove_one::<Previous>(entity);
            }
        }
    }
}

pub(crate) fn fade_afterimages(world: &mut World, dt: f32) {
    let mut gone = Vec::new();
    for (entity, image, color) in world.query_mut::<(Entity, &mut Afterimage, &mut Color)>() {
        image.remaining -= dt;
        color.0.w = (image.remaining / AFTERIMAGE_SECONDS).clamp(0.0, 1.0) * AFTERIMAGE_COLOR.w;
        if image.remaining <= 0.0 {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

#[cfg(test)]
mod tests {
    use crate::assets::Textures;
    use crate::components::Armor;

    use super::*;

    fn player(world: &mut World) -> Entity {
        world.spawn((
            Player,
            Position(Vec2::new(100.0, 100.0)),
            Health {
                current: 10,
                max: 10,
            },
            Armor { current: 0, max: 0 },
        ))
    }

    fn pressed() -> Input {
        let mut input = Input::default();
        let key = input.bindings().key(Action::Dodge);
        input.press(key);
        input
    }

    #[test]
    fn dodging_starts_towards_the_cursor_when_no_direction_is_held() {
        let mut world = World::new();
        let hero = player(&mut world);

        start_dodge(&mut world, &pressed(), Vec2::new(100.0, 400.0));

        let dodge = world.get::<&Dodge>(hero).map(|d| d.direction).ok();
        assert_eq!(dodge, Some(Vec2::Y));
        assert!(world.get::<&Invulnerable>(hero).is_ok());
    }

    #[test]
    fn a_dodge_ends_by_itself_and_then_needs_a_cooldown() {
        let mut world = World::new();
        let hero = player(&mut world);
        start_dodge(&mut world, &pressed(), Vec2::X * 500.0);

        tick_dodges(&mut world, DODGE_SECONDS + 0.01);
        assert!(world.get::<&Dodge>(hero).is_err());
        assert!(world.get::<&DodgeCooldown>(hero).is_ok());

        start_dodge(&mut world, &pressed(), Vec2::X * 500.0);
        assert!(world.get::<&Dodge>(hero).is_err());

        tick_dodges(&mut world, 5.0);
        start_dodge(&mut world, &pressed(), Vec2::X * 500.0);
        assert!(world.get::<&Dodge>(hero).is_ok());
    }

    #[test]
    fn a_dodge_leaves_fading_afterimages_that_disappear() {
        let mut world = World::new();
        let hero = world.spawn((
            Player,
            Position(Vec2::new(100.0, 100.0)),
            Size(Vec2::new(32.0, 76.0)),
            Sprite::whole(crate::render::TextureId::WHITE),
            Health {
                current: 10,
                max: 10,
            },
        ));
        start_dodge(&mut world, &pressed(), Vec2::X * 500.0);
        show_dodge_visual(&mut world, &Textures::default());
        tick_dodges(&mut world, 0.05);
        tick_dodges(&mut world, 0.05);
        let images = world.query::<&Afterimage>().iter().count();
        assert!(images >= 2, "{images}");
        assert!(world.get::<&Dodge>(hero).is_ok());

        for _ in 0..40 {
            fade_afterimages(&mut world, 0.05);
        }
        assert_eq!(world.query::<&Afterimage>().iter().count(), 0);
    }

    #[test]
    fn the_wide_roll_picture_replaces_the_player_only_while_dodging() {
        let mut world = World::new();
        let hero = world.spawn((
            Player,
            Position(Vec2::new(100.0, 100.0)),
            Color(glam::Vec4::ONE),
            Health {
                current: 10,
                max: 10,
            },
        ));
        let textures = Textures::default();

        show_dodge_visual(&mut world, &textures);
        assert_eq!(world.query::<&DodgeVisual>().iter().count(), 0);

        start_dodge(&mut world, &pressed(), Vec2::X * 500.0);
        show_dodge_visual(&mut world, &textures);
        show_dodge_visual(&mut world, &textures);
        assert_eq!(world.query::<&DodgeVisual>().iter().count(), 1);
        assert_eq!(world.get::<&Color>(hero).map(|c| c.0.w).ok(), Some(0.0));

        let _ = world.remove_one::<Dodge>(hero);
        show_dodge_visual(&mut world, &textures);
        assert_eq!(world.query::<&DodgeVisual>().iter().count(), 0);
        assert_eq!(world.get::<&Color>(hero).map(|c| c.0.w).ok(), Some(1.0));
    }

    #[test]
    fn a_dead_player_cannot_dodge() {
        let mut world = World::new();
        let hero = player(&mut world);
        if let Ok(mut health) = world.get::<&mut Health>(hero) {
            health.current = 0;
        }

        start_dodge(&mut world, &pressed(), Vec2::X * 500.0);

        assert!(world.get::<&Dodge>(hero).is_err());
    }
}
