use hecs::World;

use crate::components::{Player, RunStats};
use crate::enemies::EnemyKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stat {
    Fired,
    Hit,
    Kill(EnemyKind),
    Item,
    Gold,
    Hurt,
}

pub(crate) fn record(world: &mut World, stat: Stat, amount: u32) {
    if amount == 0 {
        return;
    }
    for stats in world.query_mut::<&mut RunStats>().with::<&Player>() {
        match stat {
            Stat::Fired => stats.fired += amount,
            Stat::Hit => stats.hits += amount,
            Stat::Kill(kind) => {
                stats.kills += amount;
                stats.kills_of[kind.index()] += amount;
            }
            Stat::Item => stats.items += amount,
            Stat::Gold => stats.gold += amount,
            Stat::Hurt => stats.hurt_this_floor += amount,
        }
    }
}

pub(crate) fn close_floor(world: &mut World) {
    for stats in world.query_mut::<&mut RunStats>().with::<&Player>() {
        if stats.hurt_this_floor == 0 {
            stats.untouched_floors += 1;
        }
        stats.hurt_this_floor = 0;
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

/// The accuracy reward only pays in full after this many shots, so one lucky hit isn't worth it.
const ACCURACY_FULL_SHOTS: u32 = 50;

fn growth(floor: u32, factor: f64, amount: f64) -> u32 {
    (factor * f64::from(floor).powf(1.02) * amount.powf(1.01)) as u32
}

fn shot_weight(fired: u32) -> f64 {
    f64::from(fired.min(ACCURACY_FULL_SHOTS)) / f64::from(ACCURACY_FULL_SHOTS)
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
            growth(floor, 2.0, accuracy * 100.0 * shot_weight(stats.fired)),
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
        record(&mut world, Stat::Kill(EnemyKind::Rat), 2);
        record(&mut world, Stat::Item, 3);
        record(&mut world, Stat::Item, 0);
        assert_eq!(
            current(&world),
            RunStats {
                fired: 4,
                hits: 1,
                kills: 2,
                items: 3,
                kills_of: {
                    let mut kills_of = [0; EnemyKind::ALL.len()];
                    kills_of[EnemyKind::Rat.index()] = 2;
                    kills_of
                },
                ..RunStats::default()
            }
        );
    }

    fn world_with_player() -> World {
        let mut world = World::new();
        let mut physics = Physics::new();
        prefabs::player(&mut world, &mut physics, Vec2::ZERO, &Textures::default());
        world
    }

    #[test]
    fn kills_are_counted_per_kind_and_in_total() {
        let mut world = world_with_player();
        record(&mut world, Stat::Kill(EnemyKind::Rat), 2);
        record(&mut world, Stat::Kill(EnemyKind::Golem), 1);
        let stats = current(&world);
        assert_eq!(stats.kills, 3);
        assert_eq!(stats.kills_of(EnemyKind::Rat), 2);
        assert_eq!(stats.kills_of(EnemyKind::Golem), 1);
    }

    #[test]
    fn a_floor_without_hurt_counts_as_untouched_and_a_hurt_one_does_not() {
        let mut world = world_with_player();
        close_floor(&mut world);
        record(&mut world, Stat::Hurt, 1);
        close_floor(&mut world);
        close_floor(&mut world);
        let stats = current(&world);
        assert_eq!(stats.untouched_floors, 2);
        assert_eq!(stats.hurt_this_floor, 0);
    }

    #[test]
    fn coins_are_counted_as_gold() {
        let mut world = world_with_player();
        record(&mut world, Stat::Gold, 5);
        assert_eq!(current(&world).gold, 5);
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
                ..RunStats::default()
            },
            3,
        );
        assert_eq!(summary.rewards[3], 10);
        assert_eq!(summary.rewards[1], growth(3, 2.0, 10.0));
        assert!(summary.rewards[2] > summary.rewards[1] / 2);
        let first_floor = summarize(RunStats::default(), 1);
        assert_eq!(first_floor.rewards, [0, 0, 0, 0]);
    }

    #[test]
    fn one_lucky_hit_is_not_worth_a_full_accuracy_reward() {
        let lucky = summarize(
            RunStats {
                fired: 1,
                hits: 1,
                ..RunStats::default()
            },
            1,
        );
        let steady = summarize(
            RunStats {
                fired: ACCURACY_FULL_SHOTS,
                hits: ACCURACY_FULL_SHOTS,
                ..RunStats::default()
            },
            1,
        );
        assert_eq!(lucky.accuracy_percent, 100);
        assert!(lucky.rewards[0] * 10 < steady.rewards[0]);
        assert_eq!(steady.rewards[0], growth(1, 2.0, 100.0));
    }
}
