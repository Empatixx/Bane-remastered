use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::components::RunStats;
use crate::systems::Summary;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct Profile {
    pub(crate) balance: u64,
    pub(crate) total_earned: u64,
    pub(crate) runs: u64,
    pub(crate) wins: u64,
    pub(crate) deaths: u64,
    pub(crate) kills: u64,
    pub(crate) items_collected: u64,
    pub(crate) bullets_fired: u64,
    pub(crate) bullets_hit: u64,
    pub(crate) best_floor: u32,
}

impl Profile {
    pub(crate) fn record_run(&mut self, stats: RunStats, summary: &Summary, victory: bool) -> u64 {
        let earned: u64 = summary
            .rewards
            .iter()
            .map(|reward| u64::from(*reward))
            .sum();
        self.balance += earned;
        self.total_earned += earned;
        self.runs += 1;
        if victory {
            self.wins += 1;
        } else {
            self.deaths += 1;
        }
        self.kills += u64::from(stats.kills);
        self.items_collected += u64::from(stats.items);
        self.bullets_fired += u64::from(stats.fired);
        self.bullets_hit += u64::from(stats.hits.min(stats.fired));
        self.best_floor = self.best_floor.max(summary.floor);
        earned
    }

    #[cfg(test)]
    pub(crate) fn spend(&mut self, amount: u64) -> bool {
        if self.balance < amount {
            return false;
        }
        self.balance -= amount;
        true
    }

    fn load(path: &Path) -> Self {
        match fs::read_to_string(path) {
            Ok(text) => serde_json::from_str(&text).unwrap_or_else(|error| {
                tracing::warn!(%error, path = %path.display(), "the profile is unreadable, starting fresh");
                Self::default()
            }),
            Err(_) => Self::default(),
        }
    }

    fn save(&self, path: &Path) {
        let result = serde_json::to_string_pretty(self)
            .map_err(|error| error.to_string())
            .and_then(|text| fs::write(path, text).map_err(|error| error.to_string()));
        if let Err(error) = result {
            tracing::error!(%error, path = %path.display(), "cannot save the profile");
        }
    }
}

#[derive(Debug)]
struct Store {
    profile: Profile,
    path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub(crate) struct SharedProfile(Rc<RefCell<Store>>);

impl SharedProfile {
    pub(crate) fn load(path: &Path) -> Self {
        Self(Rc::new(RefCell::new(Store {
            profile: Profile::load(path),
            path: Some(path.to_path_buf()),
        })))
    }

    #[cfg(test)]
    pub(crate) fn in_memory() -> Self {
        Self(Rc::new(RefCell::new(Store {
            profile: Profile::default(),
            path: None,
        })))
    }

    #[cfg(test)]
    pub(crate) fn snapshot(&self) -> Profile {
        self.0.borrow().profile.clone()
    }

    pub(crate) fn balance(&self) -> u64 {
        self.0.borrow().profile.balance
    }

    pub(crate) fn record_run(&self, stats: RunStats, summary: &Summary, victory: bool) -> u64 {
        let mut store = self.0.borrow_mut();
        let earned = store.profile.record_run(stats, summary, victory);
        if let Some(path) = &store.path {
            store.profile.save(path);
        }
        tracing::info!(
            earned,
            balance = store.profile.balance,
            "the run was saved to the profile"
        );
        earned
    }
}

#[cfg(test)]
mod tests {
    use crate::systems::summarize;

    use super::*;

    fn run(kills: u32, floor: u32) -> (RunStats, Summary) {
        let stats = RunStats {
            fired: 40,
            hits: 20,
            kills,
            items: 4,
        };
        (stats, summarize(stats, floor))
    }

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("bane-profile-{name}-{}.json", std::process::id()))
    }

    #[test]
    fn a_finished_run_adds_its_rewards_to_the_balance_and_the_lifetime_totals() {
        let mut profile = Profile::default();
        let (stats, summary) = run(10, 3);
        let earned = profile.record_run(stats, &summary, false);

        assert_eq!(
            earned,
            summary.rewards.iter().map(|r| u64::from(*r)).sum::<u64>()
        );
        assert!(earned > 0);
        assert_eq!(profile.balance, earned);
        assert_eq!(profile.total_earned, earned);
        assert_eq!((profile.runs, profile.deaths, profile.wins), (1, 1, 0));
        assert_eq!((profile.kills, profile.items_collected), (10, 4));
        assert_eq!((profile.bullets_fired, profile.bullets_hit), (40, 20));
        assert_eq!(profile.best_floor, 3);
    }

    #[test]
    fn totals_accumulate_and_the_best_floor_never_drops() {
        let mut profile = Profile::default();
        let (stats, summary) = run(10, 5);
        profile.record_run(stats, &summary, true);
        let (stats, summary) = run(2, 2);
        profile.record_run(stats, &summary, false);

        assert_eq!((profile.runs, profile.wins, profile.deaths), (2, 1, 1));
        assert_eq!(profile.kills, 12);
        assert_eq!(profile.best_floor, 5);
    }

    #[test]
    fn spending_needs_enough_balance() {
        let mut profile = Profile {
            balance: 10,
            ..Profile::default()
        };
        assert!(!profile.spend(11));
        assert_eq!(profile.balance, 10);
        assert!(profile.spend(4));
        assert_eq!(profile.balance, 6);
    }

    #[test]
    fn the_profile_survives_a_restart() {
        let path = temp_path("restart");
        let _ = fs::remove_file(&path);
        let shared = SharedProfile::load(&path);
        let (stats, summary) = run(7, 4);
        let earned = shared.record_run(stats, &summary, false);

        let reloaded = SharedProfile::load(&path);
        assert_eq!(reloaded.balance(), earned);
        assert_eq!(reloaded.snapshot().kills, 7);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn a_missing_or_broken_file_means_a_fresh_profile_and_old_files_are_tolerated() {
        let missing = temp_path("missing");
        let _ = fs::remove_file(&missing);
        assert_eq!(SharedProfile::load(&missing).snapshot(), Profile::default());

        let broken = temp_path("broken");
        fs::write(&broken, "not json").expect("a temp file");
        assert_eq!(SharedProfile::load(&broken).snapshot(), Profile::default());

        fs::write(&broken, r#"{"balance": 12}"#).expect("a temp file");
        let partial = SharedProfile::load(&broken).snapshot();
        assert_eq!((partial.balance, partial.runs), (12, 0));
        let _ = fs::remove_file(&broken);
    }

    #[test]
    fn an_in_memory_profile_never_touches_the_disk() {
        let shared = SharedProfile::in_memory();
        let (stats, summary) = run(3, 2);
        shared.record_run(stats, &summary, false);
        assert!(shared.balance() > 0);
    }
}
