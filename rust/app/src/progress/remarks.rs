use serde::{Deserialize, Serialize};

use super::challenges::ChallengeId;
use crate::enemies::EnemyKind;

/// How many one-time remarks wait for the next lobby visit at most.
pub(crate) const MAX_REMARKS: usize = 4;
pub(crate) const DEATH_MILESTONES: [u64; 6] = [1, 5, 10, 20, 50, 100];
pub(crate) const KILL_MILESTONES: [u64; 4] = [100, 500, 1000, 5000];
pub(crate) const DEPTH_MILESTONES: [u32; 4] = [2, 3, 4, 5];

/// Something the Scientist says once, on the lobby visit after it happened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Remark {
    /// The `n`-th win; the first one is special.
    Win(u64),
    FirstBoss(EnemyKind),
    /// A new personal best floor.
    Depth(u32),
    Challenge(ChallengeId),
    Deaths(u64),
    Kills(u64),
}

impl Remark {
    /// Lower goes first.
    fn rank(self) -> u8 {
        match self {
            Self::Win(_) => 0,
            Self::FirstBoss(_) => 1,
            Self::Depth(_) => 2,
            Self::Challenge(_) => 3,
            Self::Deaths(_) => 4,
            Self::Kills(_) => 5,
        }
    }
}

/// The lifetime counters a run can move.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Tally {
    pub(crate) deaths: u64,
    pub(crate) wins: u64,
    pub(crate) kills: u64,
    pub(crate) best_floor: u32,
}

fn crossed<T: Copy + PartialOrd>(milestones: &[T], before: T, after: T) -> Option<T> {
    milestones
        .iter()
        .copied()
        .filter(|milestone| before < *milestone && *milestone <= after)
        .last()
}

/// What is worth a remark after a run moved the counters from `before` to `after`.
pub(crate) fn after_run(before: Tally, after: Tally, new_bosses: &[EnemyKind]) -> Vec<Remark> {
    let mut remarks = Vec::new();
    let won = after.wins > before.wins;
    if won {
        remarks.push(Remark::Win(after.wins));
    }
    remarks.extend(new_bosses.iter().map(|boss| Remark::FirstBoss(*boss)));
    if let Some(floor) = crossed(&DEPTH_MILESTONES, before.best_floor, after.best_floor) {
        // reaching the bottom by winning is the win remark's to tell
        if !(won && floor == *DEPTH_MILESTONES.last().unwrap_or(&0)) {
            remarks.push(Remark::Depth(floor));
        }
    }
    if let Some(deaths) = crossed(&DEATH_MILESTONES, before.deaths, after.deaths) {
        remarks.push(Remark::Deaths(deaths));
    }
    if let Some(kills) = crossed(&KILL_MILESTONES, before.kills, after.kills) {
        remarks.push(Remark::Kills(kills));
    }
    remarks
}

/// Adds fresh remarks to the waiting ones, most important first, keeping only a few.
pub(crate) fn queue(waiting: &mut Vec<Remark>, fresh: impl IntoIterator<Item = Remark>) {
    for remark in fresh {
        if !waiting.contains(&remark) {
            waiting.push(remark);
        }
    }
    waiting.sort_by_key(|remark| remark.rank());
    waiting.truncate(MAX_REMARKS);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tally(deaths: u64, wins: u64, kills: u64, best_floor: u32) -> Tally {
        Tally {
            deaths,
            wins,
            kills,
            best_floor,
        }
    }

    #[test]
    fn the_first_death_on_the_first_floor_earns_only_the_first_death_remark() {
        let remarks = after_run(tally(0, 0, 0, 0), tally(1, 0, 12, 1), &[]);

        assert_eq!(remarks, vec![Remark::Deaths(1)]);
    }

    #[test]
    fn a_new_best_floor_is_remarked_once_with_the_deepest_milestone_crossed() {
        assert_eq!(
            after_run(tally(1, 0, 0, 1), tally(2, 0, 0, 4), &[]),
            vec![Remark::Depth(4)]
        );
        assert!(after_run(tally(1, 0, 0, 4), tally(2, 0, 0, 3), &[]).is_empty());
        assert!(after_run(tally(1, 0, 0, 4), tally(2, 0, 0, 4), &[]).is_empty());
    }

    #[test]
    fn a_win_speaks_for_the_bottom_floor_and_outranks_everything_else() {
        let remarks = after_run(
            tally(4, 0, 90, 3),
            tally(4, 1, 140, 5),
            &[EnemyKind::Hellhound],
        );

        assert_eq!(
            remarks,
            vec![
                Remark::Win(1),
                Remark::FirstBoss(EnemyKind::Hellhound),
                Remark::Kills(100)
            ]
        );
    }

    #[test]
    fn dying_on_the_bottom_floor_still_counts_as_reaching_it() {
        let remarks = after_run(tally(6, 0, 0, 4), tally(7, 0, 0, 5), &[]);

        assert_eq!(remarks, vec![Remark::Depth(5)]);
    }

    #[test]
    fn death_milestones_fire_only_when_crossed() {
        assert_eq!(
            after_run(tally(4, 0, 0, 1), tally(5, 0, 0, 1), &[]),
            vec![Remark::Deaths(5)]
        );
        assert!(after_run(tally(5, 0, 0, 1), tally(6, 0, 0, 1), &[]).is_empty());
    }

    #[test]
    fn the_queue_keeps_the_most_important_few_without_repeats() {
        let mut waiting = vec![Remark::Kills(100), Remark::Deaths(5)];
        queue(
            &mut waiting,
            [
                Remark::Deaths(5),
                Remark::FirstBoss(EnemyKind::Argus),
                Remark::Depth(3),
                Remark::Win(1),
            ],
        );

        assert_eq!(
            waiting,
            vec![
                Remark::Win(1),
                Remark::FirstBoss(EnemyKind::Argus),
                Remark::Depth(3),
                Remark::Deaths(5),
            ]
        );
    }

    #[test]
    fn a_remark_survives_a_round_trip_through_json() {
        let remarks = vec![
            Remark::FirstBoss(EnemyKind::KingSlime),
            Remark::Challenge(ChallengeId::Bane),
        ];
        let text = serde_json::to_string(&remarks).expect("json");

        let back: Vec<Remark> = serde_json::from_str(&text).expect("parsed");
        assert_eq!(back, remarks);
    }
}
