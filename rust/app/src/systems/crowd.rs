use std::collections::HashMap;

use glam::Vec2;
use hecs::{Entity, World};

use crate::components::{Boss, Chase, Expiring, Position, Rooted, Spawning, Velocity};

const PERSONAL_SPACE: f32 = 56.0;
const PUSH: f32 = 1.2;
pub(crate) const SHOOTER_FAR: f32 = 300.0;
pub(crate) const SHOOTER_NEAR: f32 = 200.0;

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
