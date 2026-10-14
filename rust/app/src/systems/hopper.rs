use glam::Vec2;
use hecs::{Entity, World};

use crate::components::{Charge, Clip, Expiring, Frozen, Hopper, Position, Rooted, Spawning};
use crate::enemies;
use crate::systems::golem::{play, player_at, unit};

fn hop_clip() -> Clip {
    Clip {
        row: 2,
        frames: 6,
        frame_time: enemies::HOP_SECONDS / 6.0,
        looping: false,
        flipped: false,
    }
}

pub(crate) fn tick_hoppers(world: &mut World, dt: f32) {
    let target = player_at(world).map(|(_, position, _)| position);
    let hoppers: Vec<(Entity, Vec2, Hopper)> = world
        .query::<(Entity, &Position, &Hopper)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .without::<&Frozen>()
        .iter()
        .map(|(entity, position, hopper)| (entity, position.0, *hopper))
        .collect();
    for (entity, at, mut hopper) in hoppers {
        hopper.timer -= dt;
        if hopper.timer <= 0.0 {
            if hopper.hopping {
                let _ = world.remove_one::<Charge>(entity);
                let _ = world.insert_one(entity, Rooted);
                play(world, entity, hopper.idle);
                let mut seed = (at.x as u32).wrapping_mul(2_654_435_761) ^ (at.y as u32);
                hopper.hopping = false;
                hopper.timer = enemies::HOP_REST * (0.8 + unit(&mut seed) * 0.5);
            } else {
                let direction = target.map_or(Vec2::X, |player| (player - at).normalize_or_zero());
                let _ = world.remove_one::<Rooted>(entity);
                let _ = world.insert_one(entity, Charge { direction });
                play(world, entity, hop_clip());
                hopper.hopping = true;
                hopper.timer = enemies::HOP_SECONDS;
            }
        }
        let _ = world.insert_one(entity, hopper);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hopper_alternates_between_resting_and_leaping() {
        let mut world = World::new();
        let idle = Clip {
            row: 0,
            frames: 4,
            frame_time: 0.1,
            looping: true,
            flipped: false,
        };
        let hopper = world.spawn((
            Position(Vec2::ZERO),
            Rooted,
            Hopper {
                timer: 0.1,
                hopping: false,
                idle,
            },
        ));
        tick_hoppers(&mut world, 0.2);
        assert!(world.get::<&Charge>(hopper).is_ok());
        assert!(world.get::<&Rooted>(hopper).is_err());
        tick_hoppers(&mut world, enemies::HOP_SECONDS + 0.01);
        assert!(world.get::<&Charge>(hopper).is_err());
        assert!(world.get::<&Rooted>(hopper).is_ok());
    }
}
