use glam::Vec2;
use hecs::World;

use crate::components::{Position, Size, Solid, Velocity};
use crate::geometry;

pub(crate) fn move_entities(world: &mut World, dt: f32) {
    let solids: Vec<(Vec2, Vec2)> = world
        .query::<(&Position, &Size)>()
        .with::<&Solid>()
        .iter()
        .map(|(position, size)| (position.0, size.0))
        .collect();
    for (position, velocity, size) in world.query_mut::<(&mut Position, &Velocity, Option<&Size>)>()
    {
        let size = size.map_or(Vec2::ZERO, |size| size.0);
        let delta = velocity.0 * dt;
        for axis in 0..2 {
            position.0[axis] += delta[axis];
            for &(solid_position, solid_size) in &solids {
                if geometry::collides(position.0, size, solid_position, solid_size) {
                    let reach = (size[axis] + solid_size[axis]) / 2.0;
                    position.0[axis] = if delta[axis] > 0.0 {
                        solid_position[axis] - reach
                    } else {
                        solid_position[axis] + reach
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_entities_with_velocity_only() {
        let mut world = World::new();
        let moving = world.spawn((Position(Vec2::ZERO), Velocity(Vec2::new(60.0, -30.0))));
        let still = world.spawn((Position(Vec2::ONE),));

        move_entities(&mut world, 0.5);

        let moved = world.get::<&Position>(moving).map(|p| p.0);
        let stayed = world.get::<&Position>(still).map(|p| p.0);
        assert_eq!(moved.ok(), Some(Vec2::new(30.0, -15.0)));
        assert_eq!(stayed.ok(), Some(Vec2::ONE));
    }

    #[test]
    fn solid_blocks_movement_and_allows_sliding() {
        let mut world = World::new();
        world.spawn((
            Position(Vec2::new(100.0, 0.0)),
            Size(Vec2::splat(20.0)),
            Solid,
        ));
        let mover = world.spawn((
            Position(Vec2::ZERO),
            Size(Vec2::splat(20.0)),
            Velocity(Vec2::new(100.0, 40.0)),
        ));

        move_entities(&mut world, 1.0);

        let position = world.get::<&Position>(mover).map(|p| p.0).ok();
        assert_eq!(position, Some(Vec2::new(80.0, 40.0)));
    }
}
