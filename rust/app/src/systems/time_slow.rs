use hecs::World;

use crate::artefacts::{TIME_SLOW_SCALE, TIME_SLOW_SECONDS};
use crate::components::{Player, TimeSlow};

const EASE_SECONDS: f32 = 0.3;

/// How fast hostile things run: 1.0 normally, `TIME_SLOW_SCALE` while the player holds the
/// hourglass, eased over `EASE_SECONDS` at both ends.
pub(crate) fn time_scale(world: &World) -> f32 {
    let Some(remaining) = world
        .query::<&TimeSlow>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|slow| slow.remaining)
    else {
        return 1.0;
    };
    let elapsed = TIME_SLOW_SECONDS - remaining;
    let strength = (elapsed.min(remaining) / EASE_SECONDS).clamp(0.0, 1.0);
    1.0 - (1.0 - TIME_SLOW_SCALE) * strength
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::assets::Textures;
    use crate::physics::Physics;
    use crate::prefabs;

    use super::*;

    fn slowed(remaining: f32) -> f32 {
        let mut world = World::new();
        let player = prefabs::player(
            &mut world,
            &mut Physics::new(),
            Vec2::ZERO,
            &Textures::default(),
        );
        let _ = world.insert_one(player, TimeSlow { remaining });
        time_scale(&world)
    }

    #[test]
    fn time_runs_normally_without_the_hourglass() {
        assert_eq!(time_scale(&World::new()), 1.0);
    }

    #[test]
    fn the_hourglass_slows_time_to_its_scale_once_eased_in() {
        assert!((slowed(TIME_SLOW_SECONDS / 2.0) - TIME_SLOW_SCALE).abs() < 1e-6);
    }

    #[test]
    fn the_slowdown_eases_in_and_out() {
        let start = slowed(TIME_SLOW_SECONDS);
        let entering = slowed(TIME_SLOW_SECONDS - EASE_SECONDS / 2.0);
        let leaving = slowed(EASE_SECONDS / 2.0);
        assert!((start - 1.0).abs() < 1e-6);
        assert!(entering < 1.0 && entering > TIME_SLOW_SCALE);
        assert!(leaving < 1.0 && leaving > TIME_SLOW_SCALE);
    }
}
