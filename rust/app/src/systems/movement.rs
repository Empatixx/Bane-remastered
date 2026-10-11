use hecs::World;

use crate::components::{Position, Velocity};

pub(crate) fn apply_velocity(world: &mut World, dt: f32) {
    for (position, velocity) in world.query_mut::<(&mut Position, &Velocity)>() {
        position.0 += velocity.0 * dt;
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use super::*;

    #[test]
    fn moves_entities_with_velocity_only() {
        let mut world = World::new();
        let moving = world.spawn((Position(Vec2::ZERO), Velocity(Vec2::new(60.0, -30.0))));
        let still = world.spawn((Position(Vec2::ONE),));

        apply_velocity(&mut world, 0.5);

        let moved = world.get::<&Position>(moving).map(|p| p.0);
        let stayed = world.get::<&Position>(still).map(|p| p.0);
        assert_eq!(moved.ok(), Some(Vec2::new(30.0, -15.0)));
        assert_eq!(stayed.ok(), Some(Vec2::ONE));
    }
}
