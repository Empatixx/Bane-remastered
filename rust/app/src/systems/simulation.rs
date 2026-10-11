use hecs::World;

use crate::components::{Body, Position, Velocity};
use crate::physics::Physics;

pub(crate) fn simulate(world: &mut World, physics: &mut Physics, dt: f32) {
    for (body, velocity) in world.query_mut::<(&Body, &Velocity)>() {
        physics.set_velocity(*body, velocity.0);
    }
    physics.step(dt);
    for (body, position) in world.query_mut::<(&Body, &mut Position)>() {
        if let Some(simulated) = physics.position(*body) {
            position.0 = simulated;
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::assets::Textures;
    use crate::prefabs;

    use super::*;

    const TILE: f32 = 64.0;

    fn run(world: &mut World, physics: &mut Physics, steps: usize) {
        for _ in 0..steps {
            simulate(world, physics, 1.0 / 60.0);
        }
    }

    fn player_position(world: &World, player: hecs::Entity) -> Vec2 {
        world
            .get::<&Position>(player)
            .map(|position| position.0)
            .unwrap_or(Vec2::NAN)
    }

    #[test]
    fn free_player_moves_with_its_velocity() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let player = prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(100.0, 100.0),
            &Textures::default(),
        );
        if let Ok(mut velocity) = world.get::<&mut Velocity>(player) {
            velocity.0 = Vec2::new(60.0, 0.0);
        }

        run(&mut world, &mut physics, 60);

        let position = player_position(&world, player);
        assert!((position.x - 160.0).abs() < 1.0, "x = {}", position.x);
        assert!((position.y - 100.0).abs() < 0.5, "y = {}", position.y);
    }

    #[test]
    fn wall_stops_the_player() {
        let mut world = World::new();
        let mut physics = Physics::new();
        physics.add_walls(&[(3, 0)], TILE);
        let player = prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(100.0, 32.0),
            &Textures::default(),
        );
        if let Ok(mut velocity) = world.get::<&mut Velocity>(player) {
            velocity.0 = Vec2::new(240.0, 0.0);
        }

        run(&mut world, &mut physics, 60);

        let position = player_position(&world, player);
        assert!(position.x < 3.0 * TILE - 16.0 + 1.0, "x = {}", position.x);
        assert!(position.x > 3.0 * TILE - 16.0 - 3.0, "x = {}", position.x);
    }

    #[test]
    fn player_slides_along_a_wall() {
        let mut world = World::new();
        let mut physics = Physics::new();
        physics.add_walls(&[(3, 0), (3, 1), (3, 2), (3, 3)], TILE);
        let player = prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(100.0, 32.0),
            &Textures::default(),
        );
        if let Ok(mut velocity) = world.get::<&mut Velocity>(player) {
            velocity.0 = Vec2::new(170.0, 170.0);
        }

        run(&mut world, &mut physics, 60);

        let position = player_position(&world, player);
        assert!(position.x < 3.0 * TILE - 16.0 + 1.0, "x = {}", position.x);
        assert!(position.y > 150.0, "y = {}", position.y);
    }

    #[test]
    fn player_and_enemy_pass_through_each_other() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let player = prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(100.0, 100.0),
            &Textures::default(),
        );
        let rat = prefabs::rat(
            &mut world,
            &mut physics,
            Vec2::new(200.0, 100.0),
            crate::render::TextureId::WHITE,
        );
        if let Ok(mut velocity) = world.get::<&mut Velocity>(player) {
            velocity.0 = Vec2::new(240.0, 0.0);
        }

        run(&mut world, &mut physics, 60);

        let player_x = player_position(&world, player).x;
        let rat_x = player_position(&world, rat).x;
        assert!(player_x > 300.0, "player x = {player_x}");
        assert!((rat_x - 200.0).abs() < 0.5, "rat x = {rat_x}");
    }
}
