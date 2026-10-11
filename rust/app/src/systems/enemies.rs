use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::components::{
    Animation, Body, Chase, Clip, Color, Dash, Enemy, Expiring, Health, HitFlash, Path, Player,
    Position, Size, Spawning, Velocity,
};
use crate::navigation::{Navigation, CELL};
use crate::physics::Physics;

pub(crate) const SPAWN_SECONDS: f32 = 2.0;
pub(crate) const FLASH_SECONDS: f32 = 1.0;
const FLASH_STRENGTH: f32 = 0.7;
const DASH_PERIOD: f32 = 3.0;
const DASH_DURATION: f32 = 1.0;

fn player_placement(world: &World) -> Option<(Vec2, Vec2)> {
    world
        .query::<(&Position, &Size)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(position, size)| (position.0, size.0))
}

pub(crate) fn plan_paths(world: &mut World, navigation: &Navigation) {
    let Some((target, target_size)) = player_placement(world) else {
        return;
    };
    for (position, size, path) in world
        .query_mut::<(&Position, &Size, &mut Path)>()
        .without::<&Expiring>()
        .without::<&Spawning>()
    {
        let moved = path
            .planned_for
            .is_none_or(|planned| (target - planned).abs().max_element() > CELL);
        if moved {
            path.cells = navigation
                .find_path(position.0, size.0, target, target_size)
                .into();
            path.planned_for = Some(target);
        }
    }
}

pub(crate) fn chase_player(world: &mut World) {
    let target = player_placement(world).map(|(position, _)| position);
    for (position, chase, velocity, path) in world
        .query_mut::<(&Position, &Chase, &mut Velocity, Option<&mut Path>)>()
        .without::<&Expiring>()
        .without::<&Spawning>()
    {
        let Some(target) = target else {
            velocity.0 = Vec2::ZERO;
            continue;
        };
        let waypoint = path.and_then(|path| {
            while path
                .cells
                .front()
                .is_some_and(|next| next.distance(position.0) <= CELL)
            {
                path.cells.pop_front();
            }
            path.cells.front().copied()
        });
        velocity.0 = (waypoint.unwrap_or(target) - position.0).normalize_or_zero() * chase.speed;
    }
}

pub(crate) fn face_player(world: &mut World) {
    let Some((target, _)) = player_placement(world) else {
        return;
    };
    for (position, animation) in world
        .query_mut::<(&Position, &mut Animation)>()
        .with::<&Enemy>()
        .without::<&Expiring>()
    {
        animation.face(target.x < position.0.x);
    }
}

pub(crate) fn tick_spawning(world: &mut World, physics: &mut Physics, dt: f32) {
    let mut finished = Vec::new();
    for (entity, spawning, color) in world.query_mut::<(Entity, &mut Spawning, &mut Color)>() {
        spawning.remaining -= dt;
        color.0.w = (1.0 - spawning.remaining / SPAWN_SECONDS).clamp(0.0, 1.0);
        if spawning.remaining <= 0.0 {
            finished.push(entity);
        }
    }
    for entity in finished {
        finish_spawning(world, physics, entity);
    }
}

pub(crate) fn finish_spawning(world: &mut World, physics: &mut Physics, entity: Entity) {
    let placement = world
        .query_one_mut::<(&Position, &Size)>(entity)
        .map(|(position, size)| (position.0, size.0));
    let _ = world.remove_one::<Spawning>(entity);
    if let Ok(color) = world.query_one_mut::<&mut Color>(entity) {
        color.0 = Vec4::ONE;
    }
    if let Ok((position, size)) = placement {
        let body = physics.add_enemy(entity, position, size);
        let _ = world.insert_one(entity, body);
    }
}

pub(crate) fn dash(world: &mut World, dt: f32) {
    for (dash, chase) in world
        .query_mut::<(&mut Dash, &mut Chase)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
    {
        dash.cycle = (dash.cycle + dt) % DASH_PERIOD;
        chase.speed = if dash.cycle < DASH_DURATION {
            dash.base_speed + dash.bonus_speed
        } else {
            dash.base_speed
        };
    }
}

pub(crate) fn flash_hits(world: &mut World, dt: f32) {
    let mut done = Vec::new();
    for (entity, flash, color) in world.query_mut::<(Entity, &mut HitFlash, &mut Color)>() {
        flash.remaining -= dt;
        let strength = (flash.remaining / FLASH_SECONDS).clamp(0.0, 1.0) * FLASH_STRENGTH;
        color.0 = Vec4::new(1.0, 1.0 - strength, 1.0 - strength, color.0.w);
        if flash.remaining <= 0.0 {
            done.push(entity);
        }
    }
    for entity in done {
        let _ = world.remove_one::<HitFlash>(entity);
    }
}

pub(crate) fn reap_dead(world: &mut World, physics: &mut Physics) {
    let dead: Vec<(Entity, Enemy, Option<Body>)> = world
        .query::<(Entity, &Health, &Enemy, Option<&Body>)>()
        .without::<&Expiring>()
        .without::<&Spawning>()
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
            let facing = animation.flipped();
            animation.play(Clip {
                flipped: facing,
                ..enemy.death
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec4;

    use crate::assets::Textures;
    use crate::enemies::EnemyKind;
    use crate::navigation::Navigation;
    use crate::prefabs;

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
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
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
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
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
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );

        reap_dead(&mut world, &mut physics);

        assert!(world.get::<&Expiring>(rat).is_err());
        assert!(world.get::<&Body>(rat).is_ok());
    }

    #[test]
    fn a_spawning_enemy_is_harmless_until_the_spawn_ends() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::enemy(
            &mut world,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
            0,
            &Textures::default(),
        );

        chase_player(&mut world);
        let velocity = world.get::<&Velocity>(rat).map(|v| v.0).ok();
        assert_eq!(velocity, Some(Vec2::ZERO));
        assert!(world.get::<&Body>(rat).is_err());

        tick_spawning(&mut world, &mut physics, SPAWN_SECONDS / 2.0);
        let alpha = world.get::<&Color>(rat).map(|c| c.0.w).unwrap_or(-1.0);
        assert!((alpha - 0.5).abs() < 0.01, "{alpha}");
        assert!(world.get::<&Spawning>(rat).is_ok());

        tick_spawning(&mut world, &mut physics, SPAWN_SECONDS);
        assert!(world.get::<&Spawning>(rat).is_err());
        assert!(world.get::<&Body>(rat).is_ok());
        assert_eq!(world.get::<&Color>(rat).map(|c| c.0).ok(), Some(Vec4::ONE));
    }

    #[test]
    fn ghosts_dash_for_a_second_out_of_every_three() {
        let (mut world, mut physics) = arena();
        let ghost = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Ghost,
            Vec2::new(300.0, 0.0),
        );
        let base = EnemyKind::Ghost.spec().speed();
        let speed = |world: &World| world.get::<&Chase>(ghost).map(|c| c.speed).unwrap_or(0.0);

        dash(&mut world, 0.5);
        assert!(speed(&world) > base * 3.0);
        dash(&mut world, 1.0);
        assert!((speed(&world) - base).abs() < 0.01);
        dash(&mut world, 1.6);
        assert!(speed(&world) > base * 3.0);
    }

    #[test]
    fn a_hit_tints_the_enemy_red_and_fades_back() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );
        let _ = world.insert_one(
            rat,
            HitFlash {
                remaining: FLASH_SECONDS,
            },
        );

        flash_hits(&mut world, 0.01);
        let tint = world.get::<&Color>(rat).map(|c| c.0).unwrap_or(Vec4::ONE);
        assert!(tint.y < 0.5 && tint.x == 1.0, "{tint:?}");

        flash_hits(&mut world, FLASH_SECONDS);
        assert!(world.get::<&HitFlash>(rat).is_err());
    }

    #[test]
    fn an_enemy_steers_around_a_wall_instead_of_into_it() {
        let mut world = World::new();
        let mut physics = Physics::new();
        prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(600.0, 160.0),
            &Textures::default(),
        );
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(100.0, 160.0),
        );
        let wall: Vec<(usize, usize)> = (0..5).map(|row| (4, row)).collect();
        let navigation = Navigation::new(&wall);

        plan_paths(&mut world, &navigation);
        chase_player(&mut world);

        let path_len = world.get::<&Path>(rat).map(|p| p.cells.len()).unwrap_or(0);
        assert!(path_len > 10, "{path_len}");
        let velocity = world.get::<&Velocity>(rat).map(|v| v.0).unwrap_or_default();
        assert!(velocity.x > 0.0, "{velocity:?}");
    }

    #[test]
    fn a_still_target_keeps_the_planned_path() {
        let mut world = World::new();
        let mut physics = Physics::new();
        prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(600.0, 160.0),
            &Textures::default(),
        );
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(100.0, 160.0),
        );
        let navigation = Navigation::new(&[]);

        plan_paths(&mut world, &navigation);
        let first = world.get::<&Path>(rat).map(|p| p.cells.len()).unwrap_or(0);
        plan_paths(&mut world, &navigation);
        let second = world.get::<&Path>(rat).map(|p| p.cells.len()).unwrap_or(0);

        assert!(first > 0);
        assert_eq!(first, second);
    }

    #[test]
    fn enemies_face_the_player() {
        let (mut world, mut physics) = arena();
        let right = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );
        let left = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(-300.0, 0.0),
        );

        face_player(&mut world);

        let flipped = |entity| world.get::<&Animation>(entity).map(|a| a.flipped());
        assert_eq!(flipped(right).ok(), Some(true));
        assert_eq!(flipped(left).ok(), Some(false));
    }
}
