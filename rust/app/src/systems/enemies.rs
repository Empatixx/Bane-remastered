use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::components::{
    Animation, Body, Boss, Chase, Clip, Color, Dash, Detour, Enemy, Expiring, Health, HitFlash,
    Lurk, Outline, Player, Position, Rooted, Size, Smash, Spawning, Velocity,
};
use crate::flow::FlowField;
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::systems::crowd::{ring_point, RING_RADIUS, RING_SLACK, SHOOTER_FAR, SHOOTER_NEAR};
use crate::systems::unstick::DETOUR_ANGLE;

pub(crate) const FLASH_SECONDS: f32 = 1.0;
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

pub(crate) fn update_flow(
    flow: &mut FlowField,
    world: &World,
    physics: &Physics,
    navigation: &Navigation,
    dt: f32,
) {
    if let Some((target, _)) = player_placement(world) {
        flow.update(target, navigation, physics, dt);
    }
}

pub(crate) fn chase_player(world: &mut World, flow: &FlowField) {
    let target = player_placement(world).map(|(position, _)| position);
    for (entity, position, chase, velocity, rooted, lurk, shooter, detour) in world
        .query_mut::<(
            Entity,
            &Position,
            &Chase,
            &mut Velocity,
            Option<&Rooted>,
            Option<&Lurk>,
            Option<(&Smash, Option<&Boss>)>,
            Option<&Detour>,
        )>()
        .without::<&Expiring>()
        .without::<&Spawning>()
    {
        if rooted.is_some() {
            velocity.0 = Vec2::ZERO;
            continue;
        }
        let Some(target) = target else {
            velocity.0 = Vec2::ZERO;
            continue;
        };
        let distance = position.0.distance(target);
        if shooter.is_some_and(|(_, boss)| boss.is_none()) && distance < SHOOTER_FAR {
            velocity.0 = if distance < SHOOTER_NEAR {
                (position.0 - target).normalize_or_zero() * chase.speed
            } else {
                Vec2::ZERO
            };
            continue;
        }
        let waiting = lurk.is_some() && position.0.distance(target) < RING_RADIUS + RING_SLACK;
        velocity.0 = if waiting {
            let spot = ring_point(target, entity);
            if position.0.distance(spot) <= RING_SLACK / 2.0 {
                Vec2::ZERO
            } else {
                (spot - position.0).normalize_or_zero() * chase.speed
            }
        } else {
            flow.heading(position.0, target) * chase.speed
        };
        if let Some(detour) = detour {
            velocity.0 = Vec2::from_angle(detour.turn * DETOUR_ANGLE).rotate(velocity.0);
        }
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
        color.0.w = (1.0 - spawning.remaining / spawning.total).clamp(0.0, 1.0);
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
    let mut fading = Vec::new();
    let mut done = Vec::new();
    for (entity, flash) in world.query_mut::<(Entity, &mut HitFlash)>() {
        flash.remaining -= dt;
        if flash.remaining <= 0.0 {
            done.push(entity);
        } else {
            let strength = (flash.remaining / FLASH_SECONDS).clamp(0.0, 1.0);
            fading.push((entity, strength));
        }
    }
    for (entity, strength) in fading {
        let _ = world.insert_one(entity, Outline(Vec4::new(1.0, 0.0, 0.0, strength)));
    }
    for entity in done {
        let _ = world.remove_one::<HitFlash>(entity);
        let _ = world.remove_one::<Outline>(entity);
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
    use crate::enemies::{EnemyKind, COMMON_SPAWN_SECONDS};
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

        chase_player(&mut world, &FlowField::default());

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

        chase_player(&mut world, &FlowField::default());
        let velocity = world.get::<&Velocity>(rat).map(|v| v.0).ok();
        assert_eq!(velocity, Some(Vec2::ZERO));
        assert!(world.get::<&Body>(rat).is_err());

        tick_spawning(&mut world, &mut physics, COMMON_SPAWN_SECONDS / 2.0);
        let alpha = world.get::<&Color>(rat).map(|c| c.0.w).unwrap_or(-1.0);
        assert!((alpha - 0.5).abs() < 0.01, "{alpha}");
        assert!(world.get::<&Spawning>(rat).is_ok());

        tick_spawning(&mut world, &mut physics, COMMON_SPAWN_SECONDS);
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
    fn a_hit_outlines_the_enemy_in_red_and_fades_back() {
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
        let outline = world
            .get::<&Outline>(rat)
            .map(|o| o.0)
            .unwrap_or(Vec4::ZERO);
        assert!(
            outline.x == 1.0 && outline.y == 0.0 && outline.w > 0.9,
            "{outline:?}"
        );
        assert_eq!(world.get::<&Color>(rat).map(|c| c.0).ok(), Some(Vec4::ONE));

        flash_hits(&mut world, FLASH_SECONDS);
        assert!(world.get::<&HitFlash>(rat).is_err());
        assert!(world.get::<&Outline>(rat).is_err());
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
        let mut flow = FlowField::default();

        update_flow(&mut flow, &world, &physics, &navigation, 0.016);
        chase_player(&mut world, &flow);

        let velocity = world.get::<&Velocity>(rat).map(|v| v.0).unwrap_or_default();
        assert!(velocity.x > 0.0 && velocity.y > 0.0, "{velocity:?}");
    }

    #[test]
    fn a_waiting_enemy_holds_its_spot_on_the_ring_but_a_distant_one_still_closes_in() {
        let (mut world, mut physics) = arena();
        let player = Vec2::ZERO;
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(900.0, 0.0),
        );
        let _ = world.insert_one(rat, Lurk);
        let flow = FlowField::default();

        chase_player(&mut world, &flow);
        let far = world.get::<&Velocity>(rat).map(|v| v.0).unwrap_or_default();
        assert!(far.x < 0.0, "{far:?}");

        let spot = ring_point(player, rat);
        if let Ok(mut position) = world.get::<&mut Position>(rat) {
            position.0 = spot;
        }
        chase_player(&mut world, &flow);
        let held = world.get::<&Velocity>(rat).map(|v| v.0).unwrap_or_default();
        assert_eq!(held, Vec2::ZERO);

        if let Ok(mut position) = world.get::<&mut Position>(rat) {
            position.0 = spot * 1.15;
        }
        chase_player(&mut world, &flow);
        let drifting = world.get::<&Velocity>(rat).map(|v| v.0).unwrap_or_default();
        assert!(drifting.dot(spot) < 0.0, "{drifting:?}");
    }

    #[test]
    fn a_slime_closes_in_to_shooting_range_then_holds_and_backs_off_when_pressed() {
        let (mut world, mut physics) = arena();
        let slime = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Slime,
            Vec2::new(600.0, 0.0),
        );
        let flow = FlowField::default();
        let velocity = |world: &World| {
            world
                .get::<&Velocity>(slime)
                .map(|v| v.0)
                .unwrap_or_default()
        };
        let place = |world: &mut World, x: f32| {
            if let Ok(mut position) = world.get::<&mut Position>(slime) {
                position.0 = Vec2::new(x, 0.0);
            }
        };

        chase_player(&mut world, &flow);
        assert!(velocity(&world).x < 0.0);

        place(&mut world, SHOOTER_NEAR + 40.0);
        chase_player(&mut world, &flow);
        assert_eq!(velocity(&world), Vec2::ZERO);

        place(&mut world, SHOOTER_NEAR - 40.0);
        chase_player(&mut world, &flow);
        assert!(velocity(&world).x > 0.0);
    }

    #[test]
    fn an_enemy_on_a_detour_turns_away_from_its_heading() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );
        let _ = world.insert_one(
            rat,
            Detour {
                remaining: 0.5,
                turn: 1.0,
            },
        );

        chase_player(&mut world, &FlowField::default());

        let velocity = world.get::<&Velocity>(rat).map(|v| v.0).unwrap_or_default();
        assert!(velocity.y.abs() > velocity.x.abs() * 0.5, "{velocity:?}");
        assert!(velocity.x < 0.0, "{velocity:?}");
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
