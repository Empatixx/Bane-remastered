use hecs::{Entity, World};

use crate::components::{Body, Player, Position, Previous, Velocity};
use crate::physics::Physics;

pub(crate) fn simulate(world: &mut World, physics: &mut Physics, dt: f32) {
    let orphans = physics.remove_orphans(world);
    if orphans > 0 {
        tracing::warn!(
            orphans,
            "despawned without Physics::despawn, cleaned up the leftovers"
        );
    }
    for (body, velocity) in world.query_mut::<(&Body, &Velocity)>() {
        physics.set_velocity(*body, velocity.0);
    }
    physics.step(dt);
    let mut first_step = Vec::new();
    for (entity, body, position, previous) in
        world.query_mut::<(Entity, &Body, &mut Position, Option<&mut Previous>)>()
    {
        match previous {
            Some(previous) => previous.0 = position.0,
            None => first_step.push((entity, position.0)),
        }
        if let Some(simulated) = physics.position(*body) {
            position.0 = simulated;
        }
    }
    for (entity, at) in first_step {
        let _ = world.insert_one(entity, Previous(at));
    }
    for (body, velocity) in world
        .query_mut::<(&Body, &mut Velocity)>()
        .with::<&Player>()
    {
        if let Some(actual) = physics.velocity(*body) {
            velocity.0 = actual;
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

    #[test]
    fn a_body_left_behind_by_a_plain_despawn_is_cleaned_up_on_the_next_step() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Rat,
            Vec2::new(100.0, 100.0),
        );
        let player = prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(300.0, 300.0),
            &Textures::default(),
        );
        assert_eq!(physics.body_count(), 2);

        let _ = world.despawn(rat);
        simulate(&mut world, &mut physics, 1.0 / 60.0);

        assert_eq!(physics.body_count(), 1);
        assert!(world.get::<&Body>(player).is_ok());
    }

    #[test]
    fn detaching_removes_the_body_and_the_object_collider_but_keeps_the_entity() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Rat,
            Vec2::new(100.0, 100.0),
        );

        physics.detach(&mut world, rat);

        assert_eq!(physics.body_count(), 0);
        assert!(world.contains(rat));
        assert!(world.get::<&Body>(rat).is_err());
        assert_eq!(physics.remove_orphans(&world), 0);
    }

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
    fn simulating_remembers_the_position_from_before_the_step() {
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
        run(&mut world, &mut physics, 2);
        let now = player_position(&world, player);
        let before = world
            .get::<&Previous>(player)
            .map(|p| p.0)
            .unwrap_or(Vec2::NAN);
        assert!(now.x > before.x);
        assert!(before.x > 100.0);
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
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Rat,
            Vec2::new(200.0, 100.0),
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
