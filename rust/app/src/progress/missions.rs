use serde::{Deserialize, Serialize};

use crate::components::RunStats;
use crate::enemies::{EnemyKind, Role};
use crate::rng::Rng;

const HUNTED: [(EnemyKind, &str); 5] = [
    (EnemyKind::Rat, "rats"),
    (EnemyKind::Bat, "bats"),
    (EnemyKind::Slime, "slimes"),
    (EnemyKind::Ghost, "ghosts"),
    (EnemyKind::Snake, "snakes"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "goal", rename_all = "snake_case")]
pub(crate) enum Goal {
    Kills,
    KillsOf { enemy: EnemyKind },
    Boss { enemy: EnemyKind },
    Floor,
    GoldInRun,
    Gold,
    Items,
    Untouched,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GoalKind {
    Kills,
    KillsOf,
    Boss,
    Floor,
    GoldInRun,
    Gold,
    Items,
    Untouched,
}

impl GoalKind {
    pub(crate) const ALL: [Self; 8] = [
        Self::Kills,
        Self::KillsOf,
        Self::Boss,
        Self::Floor,
        Self::GoldInRun,
        Self::Gold,
        Self::Items,
        Self::Untouched,
    ];
}

impl Goal {
    pub(crate) fn kind(self) -> GoalKind {
        match self {
            Self::Kills => GoalKind::Kills,
            Self::KillsOf { .. } => GoalKind::KillsOf,
            Self::Boss { .. } => GoalKind::Boss,
            Self::Floor => GoalKind::Floor,
            Self::GoldInRun => GoalKind::GoldInRun,
            Self::Gold => GoalKind::Gold,
            Self::Items => GoalKind::Items,
            Self::Untouched => GoalKind::Untouched,
        }
    }

    /// Best-of-a-run goals keep the best run; the rest add every run up.
    fn best_of_a_run(self) -> bool {
        matches!(self, Self::Floor | Self::GoldInRun)
    }

    fn measure(self, stats: &RunStats, floor: u32) -> u32 {
        match self {
            Self::Kills => stats.kills,
            Self::KillsOf { enemy } | Self::Boss { enemy } => stats.kills_of(enemy),
            Self::Floor => floor,
            Self::GoldInRun | Self::Gold => stats.gold,
            Self::Items => stats.items,
            Self::Untouched => stats.untouched_floors,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ActiveMission {
    #[serde(flatten)]
    pub(crate) goal: Goal,
    pub(crate) target: u32,
    pub(crate) reward: u32,
    #[serde(default)]
    pub(crate) progress: u32,
}

fn boss_name(enemy: EnemyKind) -> &'static str {
    match enemy.spec().role {
        Role::Boss(name) => name,
        _ => "boss",
    }
}

impl ActiveMission {
    pub(crate) fn done(&self) -> bool {
        self.progress >= self.target
    }

    pub(crate) fn advance(&mut self, stats: &RunStats, floor: u32) {
        let measured = self.goal.measure(stats, floor);
        self.progress = if self.goal.best_of_a_run() {
            self.progress.max(measured)
        } else {
            self.progress.saturating_add(measured)
        };
    }

    pub(crate) fn title(&self) -> String {
        match self.goal {
            Goal::Kills => "Crypt Cleaner".to_owned(),
            Goal::KillsOf { enemy } => format!("{} Hunter", hunted_name(enemy)),
            Goal::Boss { enemy } => format!("{} Slayer", boss_name(enemy)),
            Goal::Floor => "Deep Delver".to_owned(),
            Goal::GoldInRun => "Gold Digger".to_owned(),
            Goal::Gold => "Hoarder".to_owned(),
            Goal::Items => "Scavenger".to_owned(),
            Goal::Untouched => "Untouchable".to_owned(),
        }
    }

    pub(crate) fn detail(&self) -> String {
        let target = self.target;
        match self.goal {
            Goal::Kills => format!("Defeat {target} enemies"),
            Goal::KillsOf { enemy } => format!("Defeat {target} {}", hunted_name(enemy)),
            Goal::Boss { enemy } => format!("Defeat the {}", boss_name(enemy)),
            Goal::Floor => format!("Reach floor {target}"),
            Goal::GoldInRun => format!("Collect {target} gold in a single run"),
            Goal::Gold => format!("Collect {target} gold"),
            Goal::Items => format!("Pick up {target} items"),
            Goal::Untouched => format!("Clear {target} floors without taking damage"),
        }
    }
}

fn hunted_name(enemy: EnemyKind) -> &'static str {
    HUNTED
        .iter()
        .find(|(kind, _)| *kind == enemy)
        .map_or("enemies", |(_, name)| name)
}

fn tier(rng: &mut Rng, targets: [u32; 3]) -> (u32, u32) {
    let index = rng.below(3);
    (targets[index], index as u32 + 1)
}

pub(crate) fn draw(rng: &mut Rng, taken: &[GoalKind]) -> ActiveMission {
    let free: Vec<GoalKind> = GoalKind::ALL
        .into_iter()
        .filter(|kind| !taken.contains(kind))
        .collect();
    let kind = free[rng.below(free.len())];
    let (goal, target, reward) = match kind {
        GoalKind::Kills => {
            let (target, reward) = tier(rng, [50, 120, 250]);
            (Goal::Kills, target, reward)
        }
        GoalKind::KillsOf => {
            let enemy = HUNTED[rng.below(HUNTED.len())].0;
            let (target, reward) = tier(rng, [10, 25, 50]);
            (Goal::KillsOf { enemy }, target, reward)
        }
        GoalKind::Boss => {
            let bosses: Vec<EnemyKind> = EnemyKind::ALL
                .into_iter()
                .filter(|kind| kind.is_boss())
                .collect();
            (
                Goal::Boss {
                    enemy: bosses[rng.below(bosses.len())],
                },
                1,
                2,
            )
        }
        GoalKind::Floor => {
            let (target, reward) = tier(rng, [2, 3, 4]);
            (Goal::Floor, target, reward)
        }
        GoalKind::GoldInRun => {
            let (target, reward) = tier(rng, [40, 80, 140]);
            (Goal::GoldInRun, target, reward)
        }
        GoalKind::Gold => {
            let (target, reward) = tier(rng, [150, 300, 600]);
            (Goal::Gold, target, reward)
        }
        GoalKind::Items => {
            let (target, reward) = tier(rng, [15, 30, 60]);
            (Goal::Items, target, reward)
        }
        GoalKind::Untouched => {
            let (target, reward) = tier(rng, [1, 2, 3]);
            (Goal::Untouched, target, reward)
        }
    };
    ActiveMission {
        goal,
        target,
        reward,
        progress: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::Theme;

    const THEMES: [Theme; 7] = [
        Theme::Stone,
        Theme::Sewer,
        Theme::Moss,
        Theme::Spider,
        Theme::Ice,
        Theme::Lava,
        Theme::Temple,
    ];

    fn stats(kills_of_rats: u32, gold: u32) -> RunStats {
        let mut kills_of = [0; EnemyKind::ALL.len()];
        kills_of[EnemyKind::Rat.index()] = kills_of_rats;
        RunStats {
            kills: kills_of_rats,
            kills_of,
            gold,
            ..RunStats::default()
        }
    }

    fn mission(goal: Goal, target: u32) -> ActiveMission {
        ActiveMission {
            goal,
            target,
            reward: 1,
            progress: 0,
        }
    }

    #[test]
    fn totals_add_up_across_runs_and_best_of_a_run_goals_keep_the_best() {
        let mut total = mission(Goal::Gold, 100);
        let mut best = mission(Goal::GoldInRun, 100);
        for gold in [60, 30] {
            total.advance(&stats(0, gold), 1);
            best.advance(&stats(0, gold), 1);
        }
        assert_eq!(total.progress, 90);
        assert_eq!(best.progress, 60);
        assert!(!total.done());
    }

    #[test]
    fn kind_goals_count_only_their_kind_and_the_floor_goal_reads_the_floor() {
        let mut rats = mission(
            Goal::KillsOf {
                enemy: EnemyKind::Rat,
            },
            3,
        );
        let mut bats = mission(
            Goal::KillsOf {
                enemy: EnemyKind::Bat,
            },
            3,
        );
        let mut floor = mission(Goal::Floor, 3);
        rats.advance(&stats(4, 0), 3);
        bats.advance(&stats(4, 0), 3);
        floor.advance(&stats(4, 0), 3);
        assert!(rats.done() && floor.done());
        assert_eq!(bats.progress, 0);
    }

    #[test]
    fn a_draw_never_repeats_a_kind_that_is_taken() {
        let mut rng = Rng::new(11);
        let taken = [GoalKind::Kills, GoalKind::Gold];
        for _ in 0..200 {
            assert!(!taken.contains(&draw(&mut rng, &taken).goal.kind()));
        }
    }

    #[test]
    fn hunted_enemies_turn_up_somewhere_in_the_dungeon() {
        for (kind, _) in HUNTED {
            let somewhere = (1..=crate::enemies::FINAL_FLOOR)
                .any(|floor| THEMES.iter().any(|theme| kind.available_in(floor, *theme)));
            assert!(somewhere, "{kind:?}");
        }
    }

    #[test]
    fn missions_survive_a_round_trip_through_json() {
        let mission = ActiveMission {
            goal: Goal::Boss {
                enemy: EnemyKind::Golem,
            },
            target: 1,
            reward: 2,
            progress: 0,
        };
        let json = serde_json::to_string(&mission).expect("json");
        assert_eq!(
            serde_json::from_str::<ActiveMission>(&json).expect("parsed"),
            mission
        );
    }
}
