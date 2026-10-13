use std::f32::consts::{FRAC_PI_2, PI, TAU};

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::components::{DropShadow, Hidden, Light, Position, Sprite};
use crate::render::{Draw, Quad};

const MAX_DISTANCE: f32 = 400.0;
const SPEED: f32 = 8.0;
const MIN_INTENSITY: f32 = 0.03;
const MAX_INTENSITY: f32 = 0.15;
const VISIBLE_INTENSITY: f32 = 0.01;
const FADE_OUT_TARGET: f32 = 0.0;

pub(crate) fn update_shadow(world: &mut World, dt: f32) {
    let lights: Vec<(Entity, Vec2, f32)> = world
        .query::<(Entity, &Position, &Light)>()
        .without::<&Hidden>()
        .iter()
        .map(|(entity, position, light)| (entity, position.0 + light.offset, light.intensity))
        .collect();
    let t = (SPEED * dt).min(1.0);
    for (entity, position, shadow) in world.query_mut::<(Entity, &Position, &mut DropShadow)>() {
        let center = position.0;
        let strongest = lights
            .iter()
            .filter(|(owner, _, _)| *owner != entity)
            .filter_map(|&(_, source, intensity)| {
                let distance = center.distance(source);
                (distance <= MAX_DISTANCE).then(|| {
                    let strength = intensity / (distance * distance * 0.0001 + 1.0);
                    (strength, source, distance, intensity)
                })
            })
            .max_by(|a, b| a.0.total_cmp(&b.0));
        let (target_intensity, target_length) = match strongest {
            Some((_, source, distance, light_intensity)) => {
                let away = center - source;
                if away.length() > 0.001 {
                    let target_angle = away.to_angle() - FRAC_PI_2;
                    shadow.angle += wrap(target_angle - shadow.angle) * t;
                }
                let factor = 1.0 - (distance / MAX_DISTANCE).powi(2);
                (
                    (factor * light_intensity * 0.3).clamp(MIN_INTENSITY, MAX_INTENSITY),
                    (50.0 - distance / 10.0).clamp(10.0, 35.0),
                )
            }
            None => (FADE_OUT_TARGET, shadow.length),
        };
        shadow.intensity += (target_intensity - shadow.intensity) * t;
        shadow.length += (target_length - shadow.length) * t;
    }
}

fn wrap(mut difference: f32) -> f32 {
    while difference > PI {
        difference -= TAU;
    }
    while difference < -PI {
        difference += TAU;
    }
    difference
}

pub(crate) fn shadow_draw(
    position: Vec2,
    size: Vec2,
    sprite: &Sprite,
    shadow: &DropShadow,
) -> Option<Draw> {
    if shadow.intensity < VISIBLE_INTENSITY {
        return None;
    }
    let feet = Vec2::new(position.x, position.y + size.y / 2.0);
    let far = size.y + shadow.length * 0.5;
    let (sin, cos) = shadow.angle.sin_cos();
    let middle = far / 2.0;
    let center = feet + Vec2::new(-middle * sin, middle * cos);
    let [u0, v0, u1, v1] = sprite.uv;
    Some(Draw {
        texture: sprite.texture,
        quad: Quad::with_uv(
            center,
            Vec2::new(size.x, far),
            Vec4::new(0.0, 0.0, 0.0, shadow.intensity),
            [u0, v1, u1, v0],
        )
        .rotated(shadow.angle),
    })
}

#[cfg(test)]
mod tests {
    use crate::assets::Textures;
    use crate::physics::Physics;
    use crate::prefabs;

    use super::*;

    fn world_with_player_and_light(light_at: Vec2) -> World {
        let mut world = World::new();
        let mut physics = Physics::new();
        prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(500.0, 300.0),
            &Textures::default(),
        );
        world.spawn((
            Position(light_at),
            Light {
                offset: Vec2::ZERO,
                color: glam::Vec3::ONE,
                radius: 75.0,
                intensity: 1.25,
            },
        ));
        world
    }

    #[test]
    fn an_object_drops_its_shadow_away_from_the_player_light() {
        let mut world = World::new();
        let mut physics = Physics::new();
        prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(500.0, 300.0),
            &Textures::default(),
        );
        let barrel = world.spawn((Position(Vec2::new(500.0, 400.0)), DropShadow::default()));

        for _ in 0..60 {
            update_shadow(&mut world, 1.0 / 60.0);
        }

        let shadow = world
            .get::<&DropShadow>(barrel)
            .map(|s| *s)
            .unwrap_or_default();
        assert!(shadow.intensity > VISIBLE_INTENSITY);
        assert!(
            shadow.angle.abs() < 0.05,
            "points down, away from the player: {}",
            shadow.angle
        );
    }

    #[test]
    fn the_own_light_of_an_entity_does_not_drop_its_shadow() {
        let mut world = World::new();
        let candle = world.spawn((
            Position(Vec2::new(100.0, 100.0)),
            Light {
                offset: Vec2::ZERO,
                color: glam::Vec3::ONE,
                radius: 30.0,
                intensity: 2.0,
            },
            DropShadow::default(),
        ));

        update_shadow(&mut world, 1.0);

        let shadow = world
            .get::<&DropShadow>(candle)
            .map(|s| *s)
            .unwrap_or_default();
        assert!(shadow.intensity < VISIBLE_INTENSITY);
    }

    fn shadow(world: &World) -> DropShadow {
        world
            .query::<&DropShadow>()
            .iter()
            .next()
            .copied()
            .unwrap_or_default()
    }

    #[test]
    fn a_light_on_the_left_casts_the_shadow_to_the_right() {
        let mut world = world_with_player_and_light(Vec2::new(400.0, 300.0));
        for _ in 0..120 {
            update_shadow(&mut world, 1.0 / 60.0);
        }
        let shadow = shadow(&world);
        let draw = shadow_draw(
            Vec2::new(500.0, 300.0),
            Vec2::new(32.0, 76.0),
            &Sprite::whole(crate::render::TextureId::WHITE),
            &shadow,
        );
        assert!(shadow.intensity > VISIBLE_INTENSITY);
        assert!(draw.is_some());
        assert!(
            (shadow.angle + FRAC_PI_2).abs() < 0.05,
            "angle {}",
            shadow.angle
        );
    }

    #[test]
    fn without_any_light_nearby_the_shadow_fades_away() {
        let mut world = world_with_player_and_light(Vec2::new(5000.0, 300.0));
        for _ in 0..120 {
            update_shadow(&mut world, 1.0 / 60.0);
        }
        assert!(shadow(&world).intensity < VISIBLE_INTENSITY);
    }

    #[test]
    fn angles_take_the_short_way_round() {
        assert!((wrap(3.0 * PI) - PI).abs() < 0.001 || (wrap(3.0 * PI) + PI).abs() < 0.001);
        assert!((wrap(-4.0) - (TAU - 4.0)).abs() < 0.001);
    }
}
