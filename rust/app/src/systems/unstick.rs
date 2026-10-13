use hecs::{Entity, World};

use crate::components::{Chase, Detour, Expiring, Position, Progress, Spawning, Velocity};

const CHECK_SECONDS: f32 = 0.3;
const MIN_EXPECTED: f32 = 8.0;
const MIN_PROGRESS: f32 = 0.25;
pub(crate) const DETOUR_SECONDS: f32 = 0.6;
pub(crate) const DETOUR_ANGLE: f32 = 1.2;

pub(crate) fn watch_progress(world: &mut World, dt: f32) {
    let missing: Vec<(Entity, Position)> = world
        .query::<(Entity, &Position)>()
        .with::<&Chase>()
        .without::<&Progress>()
        .without::<&Spawning>()
        .iter()
        .map(|(entity, position)| (entity, *position))
        .collect();
    for (entity, position) in missing {
        let turn = if entity.id() % 2 == 0 { 1.0 } else { -1.0 };
        let _ = world.insert_one(
            entity,
            Progress {
                anchor: position.0,
                elapsed: 0.0,
                turn,
            },
        );
    }
    let mut stuck = Vec::new();
    for (entity, position, velocity, progress, detour) in world
        .query_mut::<(Entity, &Position, &Velocity, &mut Progress, Option<&Detour>)>()
        .without::<&Expiring>()
    {
        progress.elapsed += dt;
        if progress.elapsed < CHECK_SECONDS {
            continue;
        }
        let expected = velocity.0.length() * progress.elapsed;
        let moved = position.0.distance(progress.anchor);
        if detour.is_none() && expected > MIN_EXPECTED && moved < expected * MIN_PROGRESS {
            stuck.push((entity, progress.turn));
            progress.turn = -progress.turn;
        }
        progress.anchor = position.0;
        progress.elapsed = 0.0;
    }
    for (entity, turn) in stuck {
        let _ = world.insert_one(
            entity,
            Detour {
                remaining: DETOUR_SECONDS,
                turn,
            },
        );
    }
    let mut done = Vec::new();
    for (entity, detour) in world.query_mut::<(Entity, &mut Detour)>() {
        detour.remaining -= dt;
        if detour.remaining <= 0.0 {
            done.push(entity);
        }
    }
    for entity in done {
        let _ = world.remove_one::<Detour>(entity);
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use super::*;

    fn runner(world: &mut World, velocity: Vec2) -> Entity {
        world.spawn((
            Position(Vec2::new(100.0, 100.0)),
            Velocity(velocity),
            Chase { speed: 100.0 },
        ))
    }

    fn run(world: &mut World, seconds: f32) {
        let steps = (seconds / (1.0 / 60.0)).ceil() as usize;
        for _ in 0..steps {
            watch_progress(world, 1.0 / 60.0);
        }
    }

    #[test]
    fn an_enemy_that_is_told_to_run_but_does_not_move_takes_a_detour() {
        let mut world = World::new();
        let rat = runner(&mut world, Vec2::new(100.0, 0.0));

        run(&mut world, 0.7);

        assert!(world.get::<&Detour>(rat).is_ok());
    }

    #[test]
    fn an_enemy_that_keeps_moving_never_detours() {
        let mut world = World::new();
        let rat = runner(&mut world, Vec2::new(100.0, 0.0));
        for _ in 0..60 {
            if let Ok(mut position) = world.get::<&mut Position>(rat) {
                position.0.x += 100.0 / 60.0;
            }
            watch_progress(&mut world, 1.0 / 60.0);
        }

        assert!(world.get::<&Detour>(rat).is_err());
    }

    #[test]
    fn an_enemy_that_is_told_to_stand_still_is_not_stuck() {
        let mut world = World::new();
        let rat = runner(&mut world, Vec2::ZERO);

        run(&mut world, 2.0);

        assert!(world.get::<&Detour>(rat).is_err());
    }

    #[test]
    fn detours_end_and_successive_ones_turn_opposite_ways() {
        let mut world = World::new();
        let rat = runner(&mut world, Vec2::new(100.0, 0.0));
        let mut turns = Vec::new();
        let mut had = false;
        for _ in 0..240 {
            watch_progress(&mut world, 1.0 / 60.0);
            let now = world.get::<&Detour>(rat).map(|d| d.turn).ok();
            if let (Some(turn), false) = (now, had) {
                turns.push(turn);
            }
            had = now.is_some();
        }

        assert!(turns.len() >= 3, "{turns:?}");
        assert!(
            turns.windows(2).all(|pair| pair[1] == -pair[0]),
            "{turns:?}"
        );
    }
}
