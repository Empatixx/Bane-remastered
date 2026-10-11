use glam::Vec2;
use hecs::{Entity, World};

use crate::components::{
    Animation, Body, Chase, Enemy, Expiring, Health, Player, Position, Velocity,
};
use crate::physics::Physics;

pub(crate) fn chase_player(world: &mut World) {
    let target = world
        .query::<&Position>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|position| position.0);
    for (position, chase, velocity) in world
        .query_mut::<(&Position, &Chase, &mut Velocity)>()
        .without::<&Expiring>()
    {
        velocity.0 = match target {
            Some(target) => (target - position.0).normalize_or_zero() * chase.speed,
            None => Vec2::ZERO,
        };
    }
}

pub(crate) fn reap_dead(world: &mut World, physics: &mut Physics) {
    let dead: Vec<(Entity, Enemy, Option<Body>)> = world
        .query::<(Entity, &Health, &Enemy, Option<&Body>)>()
        .without::<&Expiring>()
        .iter()
        .filter(|(_, health, _, _)| health.current <= 0)
        .map(|(entity, _, enemy, body)| (entity, *enemy, body.copied()))
        .collect();
    for (entity, enemy, body) in dead {
        if let Some(body) = body {
            physics.remove(body);
            let _ = world.remove_one::<Body>(entity);
        }
        let _ = world.insert_one(entity, Expiring);
        if let Ok(mut velocity) = world.get::<&mut Velocity>(entity) {
            velocity.0 = Vec2::ZERO;
        }
        if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
            animation.play(enemy.death);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assets::Textures;
    use crate::prefabs;
    use crate::render::TextureId;

    use super::*;

    fn arena() -> (World, Physics) {
        let mut world = World::new();
        let mut physics = Physics::new();
        prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(0.0, 0.0),
            &Textures::default(),
        );
        (world, physics)
    }

    #[test]
    fn enemy_runs_towards_the_player() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::rat(
            &mut world,
            &mut physics,
            Vec2::new(300.0, 0.0),
            TextureId::WHITE,
        );

        chase_player(&mut world);

        let velocity = world.get::<&Velocity>(rat).map(|v| v.0).ok();
        assert!(
            velocity.is_some_and(|v| v.x < 0.0 && v.y.abs() < 0.001),
            "{velocity:?}"
        );
    }

    #[test]
    fn enemy_without_health_dies_and_loses_its_body() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::rat(
            &mut world,
            &mut physics,
            Vec2::new(300.0, 0.0),
            TextureId::WHITE,
        );
        if let Ok(mut health) = world.get::<&mut Health>(rat) {
            health.current = 0;
        }

        reap_dead(&mut world, &mut physics);

        assert!(world.get::<&Expiring>(rat).is_ok());
        assert!(world.get::<&Body>(rat).is_err());
    }

    #[test]
    fn living_enemy_is_left_alone() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::rat(
            &mut world,
            &mut physics,
            Vec2::new(300.0, 0.0),
            TextureId::WHITE,
        );

        reap_dead(&mut world, &mut physics);

        assert!(world.get::<&Expiring>(rat).is_err());
        assert!(world.get::<&Body>(rat).is_ok());
    }
}
