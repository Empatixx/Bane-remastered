use hecs::World;

use crate::components::{Player, RunStats};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stat {
    Fired,
    Hit,
    Kill,
    Item,
}

pub(crate) fn record(world: &mut World, stat: Stat, amount: u32) {
    if amount == 0 {
        return;
    }
    for stats in world.query_mut::<&mut RunStats>().with::<&Player>() {
        match stat {
            Stat::Fired => stats.fired += amount,
            Stat::Hit => stats.hits += amount,
            Stat::Kill => stats.kills += amount,
            Stat::Item => stats.items += amount,
        }
    }
}

pub(crate) fn current(world: &World) -> RunStats {
    world
        .query::<&RunStats>()
        .with::<&Player>()
        .iter()
        .next()
        .copied()
        .unwrap_or_default()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Summary {
    pub(crate) accuracy_percent: u32,
    pub(crate) kills: u32,
    pub(crate) items: u32,
    pub(crate) floor: u32,
    pub(crate) rewards: [u32; 4],
}

fn growth(floor: u32, factor: f64, amount: f64) -> u32 {
    (factor * f64::from(floor).powf(1.02) * amount.powf(1.01)) as u32
}

pub(crate) fn summarize(stats: RunStats, floor: u32) -> Summary {
    let accuracy = if stats.fired == 0 {
        0.0
    } else {
        f64::from(stats.hits.min(stats.fired)) / f64::from(stats.fired)
    };
    Summary {
        accuracy_percent: (accuracy * 100.0) as u32,
        kills: stats.kills,
        items: stats.items,
        floor,
        rewards: [
            growth(floor, 2.0, accuracy * 100.0),
            growth(floor, 2.0, f64::from(stats.kills)),
            growth(floor, 4.0, f64::from(stats.items)),
            floor.saturating_sub(1) * 5,
        ],
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::assets::Textures;
    use crate::physics::Physics;
    use crate::prefabs;

    use super::*;

    #[test]
    fn recording_adds_to_the_players_counters() {
        let mut world = World::new();
        let mut physics = Physics::new();
        prefabs::player(&mut world, &mut physics, Vec2::ZERO, &Textures::default());
        record(&mut world, Stat::Fired, 4);
        record(&mut world, Stat::Hit, 1);
        record(&mut world, Stat::Kill, 2);
        record(&mut world, Stat::Item, 3);
        record(&mut world, Stat::Item, 0);
        assert_eq!(
            current(&world),
            RunStats {
                fired: 4,
                hits: 1,
                kills: 2,
                items: 3
            }
        );
    }

    #[test]
    fn accuracy_is_a_whole_percentage_and_never_divides_by_zero() {
        let none = summarize(RunStats::default(), 1);
        assert_eq!(none.accuracy_percent, 0);
        let half = summarize(
            RunStats {
                fired: 10,
                hits: 5,
                ..RunStats::default()
            },
            1,
        );
        assert_eq!(half.accuracy_percent, 50);
    }

    #[test]
    fn rewards_follow_bane_formulas() {
        let summary = summarize(
            RunStats {
                fired: 100,
                hits: 50,
                kills: 10,
                items: 5,
            },
            3,
        );
        assert_eq!(summary.rewards[3], 10);
        assert_eq!(summary.rewards[1], growth(3, 2.0, 10.0));
        assert!(summary.rewards[2] > summary.rewards[1] / 2);
        let first_floor = summarize(RunStats::default(), 1);
        assert_eq!(first_floor.rewards, [0, 0, 0, 0]);
    }
}
