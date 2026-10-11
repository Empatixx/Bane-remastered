use hecs::World;

use crate::components::{Body, Drag, Position, Velocity};

const STOP_SPEED: f32 = 1.0;

pub(crate) fn glide(world: &mut World, dt: f32) {
    for (position, velocity, drag) in world
        .query_mut::<(&mut Position, &mut Velocity, &Drag)>()
        .without::<&Body>()
    {
        position.0 += velocity.0 * dt;
        velocity.0 *= (-drag.rate * dt).exp();
        if velocity.0.length() < STOP_SPEED {
            velocity.0 = glam::Vec2::ZERO;
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::assets::Textures;
    use crate::prefabs;

    use super::*;

    #[test]
    fn a_thrown_thing_slides_and_stops() {
        let mut world = World::new();
        let web = prefabs::cobweb(&mut world, Vec2::ZERO, Vec2::X, &Textures::default());
        for _ in 0..600 {
            glide(&mut world, 1.0 / 60.0);
        }
        let x = world.get::<&Position>(web).map(|p| p.0.x).unwrap_or(0.0);
        let speed = world
            .get::<&Velocity>(web)
            .map(|v| v.0.length())
            .unwrap_or(1.0);
        assert!(x > 300.0 && x < 500.0, "{x}");
        assert_eq!(speed, 0.0);
    }

    #[test]
    fn bodies_are_left_to_the_physics() {
        let mut world = World::new();
        let mut physics = crate::physics::Physics::new();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Rat,
            Vec2::new(10.0, 10.0),
        );
        let _ = world.insert_one(rat, Drag { rate: 5.0 });
        let _ = world.insert_one(rat, Velocity(Vec2::new(100.0, 0.0)));
        glide(&mut world, 0.1);
        assert_eq!(
            world.get::<&Position>(rat).map(|p| p.0).ok(),
            Some(Vec2::new(10.0, 10.0))
        );
    }
}
