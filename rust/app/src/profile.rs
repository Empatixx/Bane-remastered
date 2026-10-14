use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use serde::{Deserialize, Deserializer, Serialize};

use crate::components::RunStats;
use crate::storage::{self, Loaded};
use crate::systems::Summary;
use crate::guns::GunKind;
use crate::upgrades::{self, Perks, Purchase, Upgrade};
use crate::weapon_upgrades::{WeaponMods, WeaponProgress};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct Profile {
    pub(crate) balance: u64,
    pub(crate) gold: u64,
    pub(crate) total_earned: u64,
    pub(crate) runs: u64,
    pub(crate) wins: u64,
    pub(crate) deaths: u64,
    pub(crate) kills: u64,
    pub(crate) items_collected: u64,
    pub(crate) bullets_fired: u64,
    pub(crate) bullets_hit: u64,
    pub(crate) best_floor: u32,
    #[serde(deserialize_with = "known_upgrades")]
    pub(crate) unlocked: BTreeSet<Upgrade>,
    #[serde(deserialize_with = "known_weapons")]
    pub(crate) weapons: BTreeMap<GunKind, WeaponProgress>,
}

const PROFILE_VERSION: u32 = 1;

#[derive(Serialize)]
struct Versioned<'a> {
    version: u32,
    #[serde(flatten)]
    profile: &'a Profile,
}

#[derive(Deserialize)]
struct Stored {
    #[serde(default)]
    version: u32,
    #[serde(flatten)]
    profile: Profile,
}

fn known_upgrades<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<BTreeSet<Upgrade>, D::Error> {
    let names = Vec::<serde_json::Value>::deserialize(deserializer)?;
    Ok(names
        .iter()
        .filter_map(|name| match Upgrade::deserialize(name) {
            Ok(upgrade) => Some(upgrade),
            Err(error) => {
                tracing::warn!(%error, %name, "skipping an unknown upgrade from the profile");
                None
            }
        })
        .collect())
}

fn known_weapons<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<BTreeMap<GunKind, WeaponProgress>, D::Error> {
    let entries = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
    Ok(entries
        .into_iter()
        .filter_map(|(name, progress)| {
            let kind = GunKind::deserialize(serde_json::Value::String(name.clone()));
            match (kind, WeaponProgress::deserialize(progress)) {
                (Ok(kind), Ok(progress)) => Some((kind, progress)),
                (Err(error), _) | (_, Err(error)) => {
                    tracing::warn!(%error, %name, "skipping an unknown weapon from the profile");
                    None
                }
            }
        })
        .collect())
}

impl Profile {
    pub(crate) fn record_run(&mut self, stats: RunStats, summary: &Summary, victory: bool) -> u64 {
        let earned: u64 = summary
            .rewards
            .iter()
            .map(|reward| u64::from(*reward))
            .sum();
        self.gold += earned;
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

    pub(crate) fn buy(&mut self, upgrade: Upgrade) -> Purchase {
        upgrades::purchase(&mut self.unlocked, &mut self.balance, upgrade)
    }

    pub(crate) fn owns(&self, upgrade: Upgrade) -> bool {
        upgrades::owns(&self.unlocked, upgrade)
    }

    pub(crate) fn can_buy(&self, upgrade: Upgrade) -> bool {
        upgrades::can_buy(&self.unlocked, upgrade)
    }

    pub(crate) fn perks(&self) -> Perks {
        Perks::from_unlocked(&self.unlocked)
    }

    pub(crate) fn weapon(&self, kind: GunKind) -> WeaponProgress {
        self.weapons.get(&kind).cloned().unwrap_or_default()
    }

    pub(crate) fn buy_weapon_node(&mut self, kind: GunKind, index: usize) -> Purchase {
        let progress = self.weapons.entry(kind).or_default();
        progress.buy(kind, index, &mut self.gold)
    }

    pub(crate) fn reroll_weapon(&mut self, kind: GunKind) -> bool {
        let progress = self.weapons.entry(kind).or_default();
        progress.reroll(kind, &mut self.gold)
    }

    pub(crate) fn weapon_mods(&self) -> WeaponMods {
        WeaponMods::from_progress(&self.weapons)
    }

    fn load(path: &Path) -> (Self, bool) {
        match storage::load_json::<Stored>(path) {
            Loaded::Parsed(stored) if stored.version > PROFILE_VERSION => {
                tracing::error!(
                    version = stored.version,
                    path = %path.display(),
                    "the profile comes from a newer game, it will not be overwritten"
                );
                (stored.profile, false)
            }
            Loaded::Parsed(stored) => (stored.profile, true),
            Loaded::Missing | Loaded::Corrupt => (Self::default(), true),
            Loaded::Unreadable => (Self::default(), false),
        }
    }

    fn save(&self, path: &Path) {
        let versioned = Versioned {
            version: PROFILE_VERSION,
            profile: self,
        };
        let result = serde_json::to_string_pretty(&versioned)
            .map_err(|error| error.to_string())
            .and_then(|text| storage::write_atomic(path, &text).map_err(|error| error.to_string()));
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

impl Store {
    fn save(&self) {
        if let Some(path) = &self.path {
            self.profile.save(path);
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SharedProfile(Rc<RefCell<Store>>);

impl SharedProfile {
    pub(crate) fn load(path: &Path) -> Self {
        let (profile, writable) = Profile::load(path);
        Self(Rc::new(RefCell::new(Store {
            profile,
            path: writable.then(|| path.to_path_buf()),
        })))
    }

    #[cfg(test)]
    pub(crate) fn in_memory() -> Self {
        Self(Rc::new(RefCell::new(Store {
            profile: Profile::default(),
            path: None,
        })))
    }

    pub(crate) fn snapshot(&self) -> Profile {
        self.0.borrow().profile.clone()
    }

    pub(crate) fn perks(&self) -> Perks {
        self.0.borrow().profile.perks()
    }

    pub(crate) fn buy(&self, upgrade: Upgrade) -> Purchase {
        let mut store = self.0.borrow_mut();
        let result = store.profile.buy(upgrade);
        if result == Purchase::Bought {
            if let Some(path) = &store.path {
                store.profile.save(path);
            }
        }
        result
    }

    pub(crate) fn weapon(&self, kind: GunKind) -> WeaponProgress {
        self.0.borrow().profile.weapon(kind)
    }

    pub(crate) fn weapon_mods(&self) -> WeaponMods {
        self.0.borrow().profile.weapon_mods()
    }

    pub(crate) fn buy_weapon_node(&self, kind: GunKind, index: usize) -> Purchase {
        let mut store = self.0.borrow_mut();
        let result = store.profile.buy_weapon_node(kind, index);
        if result == Purchase::Bought {
            store.save();
        }
        result
    }

    pub(crate) fn reroll_weapon(&self, kind: GunKind) -> bool {
        let mut store = self.0.borrow_mut();
        let rerolled = store.profile.reroll_weapon(kind);
        if rerolled {
            store.save();
        }
        rerolled
    }

    pub(crate) fn balance(&self) -> u64 {
        self.0.borrow().profile.balance
    }

    pub(crate) fn gold(&self) -> u64 {
        self.0.borrow().profile.gold
    }

    pub(crate) fn top_up_gold(&self, minimum: u64) {
        let mut store = self.0.borrow_mut();
        if store.profile.gold >= minimum {
            return;
        }
        store.profile.gold = minimum;
        if let Some(path) = &store.path {
            store.profile.save(path);
        }
    }

    pub(crate) fn top_up(&self, minimum: u64) {
        let mut store = self.0.borrow_mut();
        if store.profile.balance >= minimum {
            return;
        }
        store.profile.balance = minimum;
        if let Some(path) = &store.path {
            store.profile.save(path);
        }
    }

    pub(crate) fn record_run(&self, stats: RunStats, summary: &Summary, victory: bool) -> u64 {
        let mut store = self.0.borrow_mut();
        let earned = store.profile.record_run(stats, summary, victory);
        if let Some(path) = &store.path {
            store.profile.save(path);
        }
        tracing::info!(
            earned,
            gold = store.profile.gold,
            "the run was saved to the profile"
        );
        earned
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::systems::summarize;
    use crate::upgrades::TALENT_PRICE;

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
    fn a_finished_run_adds_its_rewards_to_the_gold_and_the_lifetime_totals() {
        let mut profile = Profile::default();
        let (stats, summary) = run(10, 3);
        let earned = profile.record_run(stats, &summary, false);

        assert_eq!(
            earned,
            summary.rewards.iter().map(|r| u64::from(*r)).sum::<u64>()
        );
        assert!(earned > 0);
        assert_eq!(profile.gold, earned);
        assert_eq!(profile.balance, 0);
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
    fn upgrades_are_paid_from_the_balance_and_remembered_across_restarts() {
        let path = temp_path("upgrades");
        let _ = fs::remove_file(&path);
        let shared = SharedProfile::load(&path);
        shared.top_up(10);

        assert_eq!(shared.buy(Upgrade::Discount), Purchase::Bought);
        assert_eq!(shared.balance(), 10 - TALENT_PRICE);
        assert!(shared.perks().discount);

        let reloaded = SharedProfile::load(&path);
        assert!(reloaded.snapshot().owns(Upgrade::Discount));
        assert_eq!(reloaded.balance(), 10 - TALENT_PRICE);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn gold_from_a_run_is_remembered_across_restarts_and_the_test_top_up_never_lowers_it() {
        let path = temp_path("gold");
        let _ = fs::remove_file(&path);
        let shared = SharedProfile::load(&path);
        let (stats, summary) = run(10, 3);
        let earned = shared.record_run(stats, &summary, false);
        assert!(earned > 0);
        assert_eq!(shared.gold(), earned);
        shared.top_up_gold(earned - 1);
        assert_eq!(shared.gold(), earned);
        shared.top_up_gold(earned + 100);

        let reloaded = SharedProfile::load(&path);
        assert_eq!(reloaded.gold(), earned + 100);
        assert_eq!(reloaded.balance(), 0);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn a_profile_from_before_gold_starts_with_none() {
        let path = temp_path("no-gold");
        fs::write(&path, r#"{"version": 1, "balance": 30}"#).expect("a temp file");

        let shared = SharedProfile::load(&path);

        assert_eq!((shared.balance(), shared.gold()), (30, 0));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn the_profile_survives_a_restart() {
        let path = temp_path("restart");
        let _ = fs::remove_file(&path);
        let shared = SharedProfile::load(&path);
        let (stats, summary) = run(7, 4);
        let earned = shared.record_run(stats, &summary, false);

        let reloaded = SharedProfile::load(&path);
        assert_eq!(reloaded.gold(), earned);
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
        remove_backups(&broken);

        fs::write(&broken, r#"{"balance": 12}"#).expect("a temp file");
        let partial = SharedProfile::load(&broken).snapshot();
        assert_eq!((partial.balance, partial.runs), (12, 0));
        let _ = fs::remove_file(&broken);
    }

    #[test]
    fn a_broken_profile_is_kept_as_a_backup_instead_of_being_overwritten() {
        let path = temp_path("backup");
        fs::write(&path, r#"{"balance": 12"#).expect("a temp file");

        let shared = SharedProfile::load(&path);
        let (stats, summary) = run(1, 1);
        shared.record_run(stats, &summary, false);

        assert_eq!(remove_backups(&path), vec![r#"{"balance": 12"#.to_owned()]);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn weapon_trees_are_paid_with_gold_and_remembered_across_restarts() {
        let path = temp_path("weapons");
        let shared = SharedProfile::load(&path);
        shared.top_up_gold(1000);
        let kind = GunKind::Uzi;
        let root = shared
            .weapon(kind)
            .tree(kind)
            .nodes
            .iter()
            .position(|node| node.parent.is_none())
            .expect("a root");

        assert_eq!(shared.buy_weapon_node(kind, root), Purchase::Bought);
        assert_eq!(shared.gold(), 900);
        assert!(shared.reroll_weapon(GunKind::Pistol));
        assert_eq!(shared.gold(), 800);

        let reloaded = SharedProfile::load(&path);
        assert!(reloaded.weapon(kind).owns(root));
        assert_eq!(reloaded.weapon(GunKind::Pistol).rerolls, 1);
        assert_eq!(reloaded.gold(), 800);
        assert_ne!(
            reloaded.weapon_mods().of(kind),
            crate::weapon_upgrades::GunMods::default()
        );
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn an_unknown_weapon_is_skipped_without_losing_the_rest_of_the_profile() {
        let path = temp_path("unknown_weapon");
        fs::write(
            &path,
            r#"{"version": 1, "gold": 30, "weapons": {"uzi": {"rerolls": 2, "owned": [0]}, "laser": {"rerolls": 1}}}"#,
        )
        .expect("write");

        let profile = SharedProfile::load(&path).snapshot();

        assert_eq!(profile.gold, 30);
        assert_eq!(profile.weapons.len(), 1);
        assert_eq!(profile.weapon(GunKind::Uzi).rerolls, 2);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn an_unknown_upgrade_is_skipped_without_losing_the_rest_of_the_profile() {
        let path = temp_path("unknown");
        fs::write(
            &path,
            r#"{"version": 1, "balance": 30, "unlocked": ["discount", "removed_upgrade"]}"#,
        )
        .expect("a temp file");

        let profile = SharedProfile::load(&path).snapshot();

        assert_eq!(profile.balance, 30);
        assert_eq!(profile.unlocked, BTreeSet::from([Upgrade::Discount]));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn the_saved_profile_carries_its_version() {
        let path = temp_path("version");
        let _ = fs::remove_file(&path);
        let (stats, summary) = run(1, 1);
        SharedProfile::load(&path).record_run(stats, &summary, false);

        let saved: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&path).expect("the saved profile"))
                .expect("valid json");
        assert_eq!(saved["version"], PROFILE_VERSION);
        assert_eq!(saved["runs"], 1);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn a_profile_from_a_newer_game_is_read_but_never_overwritten() {
        let path = temp_path("newer");
        let text = r#"{"version": 99, "balance": 5}"#;
        fs::write(&path, text).expect("a temp file");

        let shared = SharedProfile::load(&path);
        assert_eq!(shared.balance(), 5);
        let (stats, summary) = run(1, 1);
        shared.record_run(stats, &summary, false);

        assert_eq!(fs::read_to_string(&path).expect("the file"), text);
        let _ = fs::remove_file(&path);
    }

    fn remove_backups(path: &Path) -> Vec<String> {
        let prefix = format!(
            "{}.corrupt-",
            path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default()
        );
        let mut contents = Vec::new();
        let dir = path.parent().expect("a temp dir");
        for entry in fs::read_dir(dir).expect("the temp dir").flatten() {
            let backup = entry.path();
            let matches = backup
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(&prefix));
            if matches {
                contents.push(fs::read_to_string(&backup).unwrap_or_default());
                let _ = fs::remove_file(backup);
            }
        }
        contents
    }

    #[test]
    fn an_in_memory_profile_never_touches_the_disk() {
        let shared = SharedProfile::in_memory();
        let (stats, summary) = run(3, 2);
        shared.record_run(stats, &summary, false);
        assert!(shared.gold() > 0);
    }
}
