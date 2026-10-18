//! The one place that moves the bodies of the simulation.
//!
//! # Who writes a body
//!
//! [`simulate`] is the single owner of the rapier bodies during a step. It drives every body
//! whose entity has `Velocity` from that component, steps, and copies the bodies back into
//! `Position` (and the player's `Velocity`). Systems that want a body somewhere else, or moving
//! otherwise, ask for it with a request component on the entity, which `simulate` carries out
//! and removes before it steps:
//!
//! - `Teleport(at)`: the body is put at `at`. The system writes `Position` itself at once, so
//!   the systems after it in the step see the new spot.
//! - `MoveTo(at)`: where a body moved only by the game (a cart) gets to over the step.
//! - `SetVelocity(v)`: the velocity of a body whose entity has no `Velocity`. An entity with
//!   `Velocity` writes that component instead, which `simulate` applies every step anyway.
//!
//! A request waits for the next `simulate`, so it is only for a write that nothing reads
//! before then: nothing between the request and the step asks the physics where that body is
//! or how fast it goes (`Physics::position`, `Physics::velocity`), and colliders only follow
//! their bodies in the step itself. When something later in the same step or frame must see
//! the body already moved, the system uses `Physics::place` or `Physics::halt`, which write
//! the body and its components together at once (a training respawn, whose frame reads the
//! player's body speed for the gaze wind before the next step).

use hecs::{Entity, World};

use crate::components::{
    Body, BodyOffset, CHILL_SLOWDOWN, Chilled, Enemy, EnemyBullet, MoveTo, Player, Position,
    Previous, SetVelocity, Teleport, Velocity,
};
use crate::physics::Physics;

/// Carries out the requests the systems left for the bodies since the last step (see the
/// module docs), and drops them, also from entities that lost their body meanwhile.
fn carry_out_requests(world: &mut World, physics: &mut Physics) {
    let mut teleported = Vec::new();
    for (entity, teleport, body) in world.query_mut::<(Entity, &Teleport, Option<&Body>)>() {
        if let Some(body) = body {
            physics.teleport(*body, teleport.0);
        }
        teleported.push(entity);
    }
    for entity in teleported {
        let _ = world.remove_one::<Teleport>(entity);
    }
    let mut moved = Vec::new();
    for (entity, move_to, body) in world.query_mut::<(Entity, &MoveTo, Option<&Body>)>() {
        if let Some(body) = body {
            physics.move_to(*body, move_to.0);
        }
        moved.push(entity);
    }
    for entity in moved {
        let _ = world.remove_one::<MoveTo>(entity);
    }
    let mut set = Vec::new();
    for (entity, velocity, body) in world.query_mut::<(Entity, &SetVelocity, Option<&Body>)>() {
        if let Some(body) = body {
            physics.set_velocity(*body, velocity.0);
        }
        set.push(entity);
    }
    for entity in set {
        let _ = world.remove_one::<SetVelocity>(entity);
    }
}

pub(crate) fn simulate(world: &mut World, physics: &mut Physics, dt: f32) {
    let orphans = physics.remove_orphans(world);
    if orphans > 0 {
        tracing::warn!(
            orphans,
            "despawned without Physics::despawn, cleaned up the leftovers"
        );
    }
    carry_out_requests(world, physics);
    let time_scale = super::time_scale(world);
    for (body, velocity, chilled, enemy, enemy_bullet) in world.query_mut::<(
        &Body,
        &Velocity,
        Option<&Chilled>,
        Option<&Enemy>,
        Option<&EnemyBullet>,
    )>() {
        let mut slowdown = if chilled.is_some() {
            CHILL_SLOWDOWN
        } else {
            1.0
        };
        if enemy.is_some() || enemy_bullet.is_some() {
            slowdown *= time_scale;
        }
        physics.set_velocity(*body, velocity.0 * slowdown);
    }
    physics.step(dt);
    let mut first_step = Vec::new();
    for (entity, body, position, previous, offset) in world.query_mut::<(
        Entity,
        &Body,
        &mut Position,
        Option<&mut Previous>,
        Option<&BodyOffset>,
    )>() {
        match previous {
            Some(previous) => previous.0 = position.0,
            None => first_step.push((entity, position.0)),
        }
        if let Some(simulated) = physics.position(*body) {
            position.0 = simulated + offset.map_or(glam::Vec2::ZERO, |offset| offset.0);
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

    /// Moves a player, a rat and an enemy bullet for one second and returns how far each went.
    fn travelled_in_a_second(time_slow: bool) -> (f32, f32, f32) {
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
            Vec2::new(100.0, 400.0),
        );
        let shot = prefabs::enemy_bullet(
            &mut world,
            &mut physics,
            crate::enemies::ProjectileKind::Green,
            Vec2::new(100.0, 700.0),
            Vec2::X,
            &Textures::default(),
        );
        for entity in [player, rat] {
            if let Ok(mut velocity) = world.get::<&mut Velocity>(entity) {
                velocity.0 = Vec2::new(60.0, 0.0);
            }
        }
        if time_slow {
            let remaining = crate::artefacts::TIME_SLOW_SECONDS / 2.0;
            let _ = world.insert_one(player, crate::components::TimeSlow { remaining });
        }
        run(&mut world, &mut physics, 60);
        (
            player_position(&world, player).x - 100.0,
            player_position(&world, rat).x - 100.0,
            player_position(&world, shot).x - 100.0,
        )
    }

    #[test]
    fn the_hourglass_slows_enemies_and_their_bullets_but_not_the_player() {
        let (player, rat, shot) = travelled_in_a_second(false);
        let (slow_player, slow_rat, slow_shot) = travelled_in_a_second(true);
        let scale = crate::artefacts::TIME_SLOW_SCALE;

        assert!(
            (slow_player - player).abs() < 0.5,
            "{slow_player} vs {player}"
        );
        assert!((slow_rat - rat * scale).abs() < 1.0, "{slow_rat} vs {rat}");
        assert!(shot > 100.0, "shot = {shot}");
        assert!(
            (slow_shot - shot * scale).abs() < 2.0,
            "{slow_shot} vs {shot}"
        );
    }

    fn barrel_scene(barrel_at: Vec2) -> (World, Physics, Entity, Entity) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        let player = prefabs::player(&mut world, &mut physics, Vec2::new(0.0, 0.0), &textures);
        let barrel = prefabs::breakable(
            &mut world,
            &mut physics,
            crate::components::BreakKind::Barrel,
            barrel_at,
            &textures,
        );
        (world, physics, player, barrel)
    }

    fn walk_right(world: &mut World, physics: &mut Physics, player: Entity, steps: usize) {
        for _ in 0..steps {
            if let Ok(mut velocity) = world.get::<&mut Velocity>(player) {
                velocity.0 = Vec2::new(300.0, 0.0);
            }
            simulate(world, physics, 1.0 / 60.0);
        }
    }

    fn position(world: &World, entity: Entity) -> Vec2 {
        world
            .get::<&Position>(entity)
            .map(|position| position.0)
            .unwrap_or_default()
    }

    #[test]
    fn a_barrel_stands_still_until_somebody_pushes_it() {
        let (mut world, mut physics, _player, barrel) = barrel_scene(Vec2::new(300.0, -64.0));
        let start = position(&world, barrel);

        for _ in 0..60 {
            simulate(&mut world, &mut physics, 1.0 / 60.0);
        }

        assert!(position(&world, barrel).distance(start) < 0.5);
    }

    #[test]
    fn the_player_pushes_a_barrel_and_it_slows_down_and_stops_after_being_let_go() {
        let (mut world, mut physics, player, barrel) = barrel_scene(Vec2::new(70.0, -64.0));
        let start = position(&world, barrel);

        walk_right(&mut world, &mut physics, player, 60);
        let pushed = position(&world, barrel);
        assert!(pushed.x > start.x + 60.0, "{start:?} -> {pushed:?}");
        assert!((pushed.y - start.y).abs() < 20.0);

        if let Ok(mut velocity) = world.get::<&mut Velocity>(player) {
            velocity.0 = Vec2::ZERO;
        }
        for _ in 0..120 {
            simulate(&mut world, &mut physics, 1.0 / 60.0);
        }
        let rested = position(&world, barrel);
        for _ in 0..30 {
            simulate(&mut world, &mut physics, 1.0 / 60.0);
        }
        assert!(position(&world, barrel).distance(rested) < 0.5);
    }

    #[test]
    fn the_sprite_keeps_its_offset_from_the_collider_while_the_barrel_moves() {
        let (mut world, mut physics, player, barrel) = barrel_scene(Vec2::new(70.0, -64.0));
        let offset = world
            .get::<&BodyOffset>(barrel)
            .map(|offset| offset.0)
            .unwrap_or_default();
        walk_right(&mut world, &mut physics, player, 30);

        let body = world
            .get::<&Body>(barrel)
            .map(|body| *body)
            .expect("a body");
        let collider = physics.position(body).unwrap_or_default();

        assert_eq!(position(&world, barrel), collider + offset);
    }

    #[test]
    fn a_pushed_barrel_stops_at_a_wall() {
        let (mut world, mut physics, player, barrel) = barrel_scene(Vec2::new(70.0, -64.0));
        physics.add_static_box(Vec2::new(300.0, -32.0), Vec2::new(32.0, 400.0));

        walk_right(&mut world, &mut physics, player, 240);

        let body = world
            .get::<&Body>(barrel)
            .map(|body| *body)
            .expect("a body");
        let collider = physics.position(body).unwrap_or_default();
        assert!(collider.x < 300.0 - 16.0, "{collider:?}");
    }

    #[test]
    fn two_barrels_do_not_pass_through_each_other() {
        let (mut world, mut physics, player, first) = barrel_scene(Vec2::new(70.0, -64.0));
        let second = prefabs::breakable(
            &mut world,
            &mut physics,
            crate::components::BreakKind::Barrel,
            Vec2::new(190.0, -64.0),
            &Textures::default(),
        );

        walk_right(&mut world, &mut physics, player, 120);

        assert!(position(&world, second).x > position(&world, first).x + 40.0);
    }

    #[test]
    fn the_obstacle_list_follows_a_moved_barrel() {
        let (mut world, mut physics, player, barrel) = barrel_scene(Vec2::new(70.0, -64.0));
        let before = physics.solids_within(Vec2::splat(-1000.0), Vec2::splat(1000.0));
        walk_right(&mut world, &mut physics, player, 60);
        let after = physics.solids_within(Vec2::splat(-1000.0), Vec2::splat(1000.0));

        assert_eq!(before.len(), after.len());
        assert!(after[0].0.x > before[0].0.x + 40.0);
        let _ = barrel;
    }

    fn body_of(world: &World, entity: Entity) -> Body {
        *world.get::<&Body>(entity).expect("a body")
    }

    #[test]
    fn a_teleport_request_moves_the_body_in_the_next_step_and_is_dropped() {
        let (mut world, mut physics, _player, barrel) = barrel_scene(Vec2::new(300.0, -64.0));
        let body = body_of(&world, barrel);
        let start = physics.position(body).unwrap_or_default();
        let spot = start + Vec2::new(200.0, 100.0);

        let _ = world.insert_one(barrel, Teleport(spot));
        assert_eq!(physics.position(body), Some(start), "waits for the step");
        simulate(&mut world, &mut physics, 1.0 / 60.0);

        assert!(physics.position(body).unwrap_or_default().distance(spot) < 0.5);
        assert!(world.get::<&Teleport>(barrel).is_err());
    }

    #[test]
    fn a_velocity_request_drives_a_body_without_velocity_for_the_step() {
        let (mut world, mut physics, _player, barrel) = barrel_scene(Vec2::new(300.0, -64.0));
        let body = body_of(&world, barrel);
        let start = physics.position(body).unwrap_or_default();

        let _ = world.insert_one(barrel, SetVelocity(Vec2::new(120.0, 0.0)));
        simulate(&mut world, &mut physics, 1.0 / 60.0);

        assert!(physics.position(body).unwrap_or_default().x > start.x + 1.0);
        assert!(world.get::<&SetVelocity>(barrel).is_err());
    }

    #[test]
    fn a_move_request_carries_a_moving_object_there_over_the_step() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let cart = world.spawn((Position(Vec2::new(100.0, 100.0)),));
        let body = physics.add_moving_object(cart, Vec2::new(100.0, 100.0), Vec2::splat(40.0));
        let _ = world.insert_one(cart, body);

        let _ = world.insert_one(cart, MoveTo(Vec2::new(110.0, 100.0)));
        simulate(&mut world, &mut physics, 1.0 / 60.0);

        assert!(position(&world, cart).distance(Vec2::new(110.0, 100.0)) < 0.01);
        assert!(world.get::<&MoveTo>(cart).is_err());
    }

    #[test]
    fn a_request_left_on_an_entity_without_a_body_is_dropped() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let ghost = world.spawn((Teleport(Vec2::ONE), MoveTo(Vec2::ONE), SetVelocity(Vec2::ONE)));

        simulate(&mut world, &mut physics, 1.0 / 60.0);

        assert!(world.get::<&Teleport>(ghost).is_err());
        assert!(world.get::<&MoveTo>(ghost).is_err());
        assert!(world.get::<&SetVelocity>(ghost).is_err());
    }

    #[test]
    fn placing_and_halting_write_the_body_and_the_components_at_once() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let player = prefabs::player(&mut world, &mut physics, Vec2::ZERO, &Textures::default());
        walk_right(&mut world, &mut physics, player, 10);
        let spot = Vec2::new(500.0, 200.0);

        physics.place(&mut world, player, spot);
        physics.halt(&mut world, player);

        let body = body_of(&world, player);
        assert_eq!(physics.position(body), Some(spot));
        assert_eq!(physics.velocity(body), Some(Vec2::ZERO));
        assert_eq!(position(&world, player), spot);
        assert_eq!(world.get::<&Velocity>(player).map(|v| v.0).ok(), Some(Vec2::ZERO));
    }
}
