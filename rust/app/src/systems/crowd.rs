use std::collections::HashMap;

use glam::Vec2;
use hecs::{Entity, World};

use crate::components::{
    Boss, Chase, ContactDamage, Expiring, Lurk, Player, Position, Rooted, Smash, Spawning, Velocity,
};

const PERSONAL_SPACE: f32 = 56.0;
const PUSH: f32 = 1.2;
pub(crate) const MAX_RUSHERS: usize = 4;
pub(crate) const RING_RADIUS: f32 = 190.0;
pub(crate) const RING_SLACK: f32 = 40.0;
pub(crate) const SHOOTER_FAR: f32 = 300.0;
pub(crate) const SHOOTER_NEAR: f32 = 200.0;
const INCUMBENT_BONUS: f32 = 0.75;
const GOLDEN_ANGLE: f32 = 2.399_963;

pub(crate) fn ring_point(player: Vec2, entity: Entity) -> Vec2 {
    let angle = entity.id() as f32 * GOLDEN_ANGLE;
    player + Vec2::new(angle.cos(), angle.sin()) * RING_RADIUS
}

pub(crate) fn assign_attackers(world: &mut World) {
    let Some(player) = world
        .query::<&Position>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|position| position.0)
    else {
        return;
    };
    let mut candidates: Vec<(Entity, f32, bool)> = world
        .query::<(Entity, &Position, &Chase, Option<&Lurk>)>()
        .with::<&ContactDamage>()
        .without::<&Boss>()
        .without::<&Smash>()
        .without::<&Rooted>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .filter(|(_, _, chase, _)| chase.speed > 0.0)
        .map(|(entity, position, _, lurk)| {
            let distance = position.0.distance(player);
            let score = if lurk.is_none() {
                distance * INCUMBENT_BONUS
            } else {
                distance
            };
            (entity, score, lurk.is_some())
        })
        .collect();
    candidates.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.id().cmp(&b.0.id())));
    for (rank, (entity, _, lurking)) in candidates.into_iter().enumerate() {
        let should_lurk = rank >= MAX_RUSHERS;
        if should_lurk && !lurking {
            let _ = world.insert_one(entity, Lurk);
        } else if !should_lurk && lurking {
            let _ = world.remove_one::<Lurk>(entity);
        }
    }
}

fn bucket(position: Vec2) -> (i32, i32) {
    (
        (position.x / PERSONAL_SPACE).floor() as i32,
        (position.y / PERSONAL_SPACE).floor() as i32,
    )
}

pub(crate) fn separate_chasers(world: &mut World) {
    let crowd: Vec<(Entity, Vec2, bool)> = world
        .query::<(Entity, &Position, Option<&Rooted>, Option<&Boss>)>()
        .with::<&Chase>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .map(|(entity, position, rooted, boss)| {
            (entity, position.0, rooted.is_some() || boss.is_some())
        })
        .collect();
    let mut grid: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
    for (index, (_, position, _)) in crowd.iter().enumerate() {
        grid.entry(bucket(*position)).or_default().push(index);
    }
    for (index, (entity, position, rooted)) in crowd.iter().enumerate() {
        if *rooted {
            continue;
        }
        let home = bucket(*position);
        let mut push = Vec2::ZERO;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let Some(others) = grid.get(&(home.0 + dx, home.1 + dy)) else {
                    continue;
                };
                for &other in others {
                    if other == index {
                        continue;
                    }
                    let away = *position - crowd[other].1;
                    let distance = away.length();
                    if distance >= PERSONAL_SPACE {
                        continue;
                    }
                    let direction = if distance < 0.001 {
                        let angle = (index as f32 * 2.399_963).sin_cos();
                        Vec2::new(angle.1, angle.0)
                    } else {
                        away / distance
                    };
                    push += direction * (1.0 - distance / PERSONAL_SPACE);
                }
            }
        }
        if push == Vec2::ZERO {
            continue;
        }
        if let Ok((velocity, chase)) = world.query_one_mut::<(&mut Velocity, &Chase)>(*entity) {
            let limit = chase.speed.max(velocity.0.length());
            let steered = velocity.0 + push * chase.speed * PUSH;
            velocity.0 = steered.clamp_length_max(limit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rat(world: &mut World, at: Vec2, velocity: Vec2) -> Entity {
        world.spawn((Position(at), Velocity(velocity), Chase { speed: 100.0 }))
    }

    fn velocity(world: &World, entity: Entity) -> Vec2 {
        world
            .get::<&Velocity>(entity)
            .map(|v| v.0)
            .unwrap_or_default()
    }

    fn biter(world: &mut World, at: Vec2) -> Entity {
        world.spawn((
            Position(at),
            Velocity::default(),
            Chase { speed: 100.0 },
            ContactDamage(1),
        ))
    }

    fn lurking(world: &World) -> usize {
        world.query::<&Lurk>().iter().count()
    }

    #[test]
    fn a_few_enemies_all_attack() {
        let mut world = World::new();
        world.spawn((Player, Position(Vec2::ZERO)));
        for index in 0..MAX_RUSHERS {
            biter(&mut world, Vec2::new(300.0 + index as f32 * 40.0, 0.0));
        }

        assign_attackers(&mut world);

        assert_eq!(lurking(&world), 0);
    }

    #[test]
    fn only_the_nearest_few_attack_and_the_rest_wait() {
        let mut world = World::new();
        world.spawn((Player, Position(Vec2::ZERO)));
        let near: Vec<Entity> = (0..MAX_RUSHERS)
            .map(|index| biter(&mut world, Vec2::new(200.0 + index as f32 * 10.0, 0.0)))
            .collect();
        let far: Vec<Entity> = (0..3)
            .map(|index| biter(&mut world, Vec2::new(800.0 + index as f32 * 10.0, 0.0)))
            .collect();

        assign_attackers(&mut world);

        assert!(near.iter().all(|e| world.get::<&Lurk>(*e).is_err()));
        assert!(far.iter().all(|e| world.get::<&Lurk>(*e).is_ok()));
    }

    #[test]
    fn a_waiting_enemy_is_called_in_when_an_attacker_dies() {
        let mut world = World::new();
        world.spawn((Player, Position(Vec2::ZERO)));
        let attackers: Vec<Entity> = (0..MAX_RUSHERS)
            .map(|index| biter(&mut world, Vec2::new(200.0 + index as f32 * 10.0, 0.0)))
            .collect();
        let waiter = biter(&mut world, Vec2::new(600.0, 0.0));
        assign_attackers(&mut world);
        assert!(world.get::<&Lurk>(waiter).is_ok());

        let _ = world.despawn(attackers[0]);
        assign_attackers(&mut world);

        assert!(world.get::<&Lurk>(waiter).is_err());
    }

    #[test]
    fn an_attacker_keeps_its_place_against_a_slightly_closer_waiter() {
        let mut world = World::new();
        world.spawn((Player, Position(Vec2::ZERO)));
        for index in 0..MAX_RUSHERS - 1 {
            biter(&mut world, Vec2::new(150.0 + index as f32 * 10.0, 0.0));
        }
        let incumbent = biter(&mut world, Vec2::new(300.0, 0.0));
        let challenger = biter(&mut world, Vec2::new(0.0, 280.0));
        let _ = world.insert_one(challenger, Lurk);

        assign_attackers(&mut world);

        assert!(world.get::<&Lurk>(incumbent).is_err());
        assert!(world.get::<&Lurk>(challenger).is_ok());
    }

    #[test]
    fn shooting_enemies_never_take_a_melee_slot_or_wait_on_the_ring() {
        let mut world = World::new();
        world.spawn((Player, Position(Vec2::ZERO)));
        for index in 0..MAX_RUSHERS {
            biter(&mut world, Vec2::new(100.0 + index as f32, 0.0));
        }
        let slime = biter(&mut world, Vec2::new(900.0, 0.0));
        let _ = world.insert_one(
            slime,
            Smash {
                kind: crate::enemies::ProjectileKind::Green,
                count: 5,
                remaining: 0.0,
            },
        );

        assign_attackers(&mut world);

        assert!(world.get::<&Lurk>(slime).is_err());
    }

    #[test]
    fn bosses_and_stationary_enemies_never_take_part() {
        let mut world = World::new();
        world.spawn((Player, Position(Vec2::ZERO)));
        for index in 0..MAX_RUSHERS {
            biter(&mut world, Vec2::new(100.0 + index as f32, 0.0));
        }
        let boss = biter(&mut world, Vec2::new(900.0, 0.0));
        let _ = world.insert_one(boss, Boss);
        let turret = world.spawn((
            Position(Vec2::new(950.0, 0.0)),
            Velocity::default(),
            Chase { speed: 0.0 },
            ContactDamage(1),
        ));

        assign_attackers(&mut world);

        assert!(world.get::<&Lurk>(boss).is_err());
        assert!(world.get::<&Lurk>(turret).is_err());
    }

    #[test]
    fn two_enemies_on_top_of_each_other_are_pushed_apart() {
        let mut world = World::new();
        let a = rat(&mut world, Vec2::new(100.0, 100.0), Vec2::new(-100.0, 0.0));
        let b = rat(&mut world, Vec2::new(110.0, 100.0), Vec2::new(-100.0, 0.0));

        separate_chasers(&mut world);

        assert!(velocity(&world, a).x < velocity(&world, b).x);
    }

    #[test]
    fn enemies_that_are_far_apart_do_not_influence_each_other() {
        let mut world = World::new();
        let a = rat(&mut world, Vec2::new(100.0, 100.0), Vec2::new(-100.0, 0.0));
        rat(&mut world, Vec2::new(400.0, 100.0), Vec2::new(-100.0, 0.0));

        separate_chasers(&mut world);

        assert_eq!(velocity(&world, a), Vec2::new(-100.0, 0.0));
    }

    #[test]
    fn separation_never_makes_an_enemy_faster_than_it_was_allowed_to_be() {
        let mut world = World::new();
        let a = rat(&mut world, Vec2::new(100.0, 100.0), Vec2::new(-100.0, 0.0));
        for step in 0..6 {
            rat(
                &mut world,
                Vec2::new(100.0 + step as f32 * 3.0, 100.0),
                Vec2::new(-100.0, 0.0),
            );
        }

        separate_chasers(&mut world);

        assert!(velocity(&world, a).length() <= 100.0 + 0.001);
    }

    #[test]
    fn stacked_enemies_at_the_same_point_still_get_separated() {
        let mut world = World::new();
        let a = rat(&mut world, Vec2::new(100.0, 100.0), Vec2::ZERO);
        let b = rat(&mut world, Vec2::new(100.0, 100.0), Vec2::ZERO);

        separate_chasers(&mut world);

        assert_ne!(velocity(&world, a), velocity(&world, b));
        assert!(velocity(&world, a).length() > 1.0);
    }

    #[test]
    fn a_rooted_enemy_stays_put_but_still_pushes_the_others() {
        let mut world = World::new();
        let moving = rat(&mut world, Vec2::new(100.0, 100.0), Vec2::new(-100.0, 0.0));
        let rooted = rat(&mut world, Vec2::new(90.0, 100.0), Vec2::ZERO);
        let _ = world.insert_one(rooted, Rooted);

        separate_chasers(&mut world);

        assert_eq!(velocity(&world, rooted), Vec2::ZERO);
        assert!(velocity(&world, moving).x > -100.0);
    }
}
