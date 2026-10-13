use glam::Vec2;
use hecs::World;

use crate::components::{Hidden, Light, Position, Previous, blended};
use crate::render::{LightSource, MAX_LIGHTS};
use crate::viewport::TARGET;

pub(crate) fn collect_lights(world: &World, camera: Vec2, blend: f32) -> Vec<LightSource> {
    let view_center = camera + TARGET / 2.0;
    let mut lights: Vec<LightSource> = world
        .query::<(&Position, Option<&Previous>, &Light)>()
        .without::<&Hidden>()
        .iter()
        .map(|(position, previous, light)| LightSource {
            position: blended(position.0, previous, blend) + light.offset,
            color: light.color,
            radius: light.radius,
            intensity: light.intensity,
        })
        .collect();
    if lights.len() > MAX_LIGHTS {
        lights.sort_by(|a, b| {
            a.position
                .distance_squared(view_center)
                .total_cmp(&b.position.distance_squared(view_center))
        });
        lights.truncate(MAX_LIGHTS);
    }
    lights
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::*;

    fn world_with_lights(count: usize) -> World {
        let mut world = World::new();
        for index in 0..count {
            world.spawn((
                Position(Vec2::new(index as f32 * 100.0, 0.0)),
                Light {
                    offset: Vec2::new(0.0, 10.0),
                    color: Vec3::ONE,
                    radius: 50.0,
                    intensity: 1.0,
                },
            ));
        }
        world
    }

    #[test]
    fn light_position_is_the_entity_position_plus_the_offset() {
        let world = world_with_lights(1);
        let lights = collect_lights(&world, Vec2::ZERO, 1.0);
        assert_eq!(lights[0].position, Vec2::new(0.0, 10.0));
    }

    #[test]
    fn too_many_lights_keep_the_ones_nearest_to_the_view_centre() {
        let world = world_with_lights(MAX_LIGHTS + 20);
        let lights = collect_lights(&world, Vec2::new(0.0, 0.0), 1.0);
        assert_eq!(lights.len(), MAX_LIGHTS);
        assert!(
            lights
                .iter()
                .all(|light| light.position.x <= 100.0 * MAX_LIGHTS as f32)
        );
    }
}
