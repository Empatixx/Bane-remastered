use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use serde::{Deserialize, Deserializer, Serialize};

use crate::components::RunStats;
use crate::progress::{BoardView, ChallengeId, ProgressState, RunOutcome, RunRecord};
use crate::rng::Rng;
use crate::rules::RunRules;
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
    #[serde(deserialize_with = "lenient_progress")]
    pub(crate) progress: ProgressState,
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

fn lenient_progress<'de, D: Deserializer<'de>>(deserializer: D) -> Result<ProgressState, D::Error> {
    let value = serde_json::Value::deserialize(deserializer)?;
    Ok(serde_json::from_value(value).unwrap_or_else(|error| {
        tracing::warn!(%error, "the saved progress is unreadable, starting it afresh");
        ProgressState::default()
    }))
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
        progress.reroll(&mut self.gold)
    }

    pub(crate) fn weapon_mods(&self) -> WeaponMods {
        WeaponMods::from_progress(&self.weapons)
    }

    /// Pays back and resets the weapon trees an older generator grew; says whether any was.
    fn settle_weapon_trees(&mut self) -> bool {
        let mut settled = false;
        for (kind, progress) in &mut self.weapons {
            if let Some(refund) = progress.settle(*kind) {
                tracing::info!(?kind, refund, "the weapon tree changed, its gold was paid back");
                self.gold += refund;
                settled = true;
            }
        }
        settled
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
        let (mut profile, writable) = Profile::load(path);
        let settled = profile.settle_weapon_trees();
        let drawn = profile.progress.settle(Rng::from_clock().next_u64());
        let store = Store {
            profile,
            path: writable.then(|| path.to_path_buf()),
        };
        if settled || drawn {
            store.save();
        }
        Self(Rc::new(RefCell::new(store)))
    }

    #[cfg(test)]
    pub(crate) fn in_memory() -> Self {
        let mut profile = Profile::default();
        profile.progress.settle(1);
        Self(Rc::new(RefCell::new(Store {
            profile,
            path: None,
        })))
    }

    fn change<T>(&self, change: impl FnOnce(&mut Profile) -> T) -> T {
        let mut store = self.0.borrow_mut();
        let result = change(&mut store.profile);
        store.save();
        result
    }

    pub(crate) fn rules(&self) -> RunRules {
        self.0.borrow().profile.progress.rules()
    }

    pub(crate) fn challenge(&self) -> Option<ChallengeId> {
        self.0.borrow().profile.progress.selected()
    }

    pub(crate) fn accent(&self) -> Option<[f32; 3]> {
        self.0.borrow().profile.progress.accent()
    }

    pub(crate) fn board(&self) -> BoardView {
        self.0.borrow().profile.progress.board()
    }

    pub(crate) fn toggle_challenge(&self, id: ChallengeId) {
        self.change(|profile| profile.progress.toggle(id));
    }

    pub(crate) fn claim_mission(&self, slot: usize) -> Option<u32> {
        self.change(|profile| {
            let paid = profile.progress.claim(slot)?;
            profile.balance += u64::from(paid);
            Some(paid)
        })
    }

    pub(crate) fn finish_run(&self, record: &RunRecord) -> RunOutcome {
        self.change(|profile| {
            let outcome = profile.progress.finish_run(record);
            if let Some((_, talents)) = outcome.challenge {
                profile.balance += u64::from(talents);
            }
            outcome
        })
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

    use super::*;

    fn run(kills: u32, floor: u32) -> (RunStats, Summary) {
        let stats = RunStats {
            fired: 40,
            hits: 20,
            kills,
            items: 4,
            ..RunStats::default()
        };
        (stats, summarize(stats, floor))
    }

    fn without_progress(shared: SharedProfile) -> Profile {
        Profile {
            progress: ProgressState::default(),
            ..shared.snapshot()
        }
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
        let price = Upgrade::Discount.def().price;
        assert_eq!(shared.balance(), 10 - price);
        assert!(shared.perks().discount);

        let reloaded = SharedProfile::load(&path);
        assert!(reloaded.snapshot().owns(Upgrade::Discount));
        assert_eq!(reloaded.balance(), 10 - price);
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
        assert_eq!(
            without_progress(SharedProfile::load(&missing)),
            Profile::default()
        );

        let broken = temp_path("broken");
        fs::write(&broken, "not json").expect("a temp file");
        assert_eq!(
            without_progress(SharedProfile::load(&broken)),
            Profile::default()
        );
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
    fn a_weapon_tree_from_an_older_generator_is_paid_back_when_the_profile_loads() {
        let path = temp_path("old_tree");
        let old = crate::weapon_tree::generate_unversioned(GunKind::Shotgun, 0);
        let root = old
            .nodes
            .iter()
            .position(|node| node.parent.is_none())
            .expect("a root");
        fs::write(
            &path,
            format!(r#"{{"version": 1, "gold": 30, "weapons": {{"shotgun": {{"owned": [{root}]}}}}}}"#),
        )
        .expect("write");

        let loaded = SharedProfile::load(&path);

        assert_eq!(loaded.gold(), 130);
        assert!(loaded.weapon(GunKind::Shotgun).owned.is_empty());
        assert_eq!(SharedProfile::load(&path).gold(), 130, "saved after the refund");
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn an_unknown_weapon_is_skipped_without_losing_the_rest_of_the_profile() {
        let path = temp_path("unknown_weapon");
        fs::write(
            &path,
            r#"{"version": 1, "gold": 30, "weapons": {"uzi": {"rerolls": 2, "owned": [0], "spent": 100, "generator": 1}, "laser": {"rerolls": 1}}}"#,
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
    fn a_profile_from_before_progress_loads_with_a_full_board() {
        let path = temp_path("no-progress");
        fs::write(&path, r#"{"version": 1, "balance": 30}"#).expect("a temp file");
        let shared = SharedProfile::load(&path);
        assert_eq!(shared.balance(), 30);
        assert_eq!(shared.board().missions.len(), 3);
        assert_eq!(shared.challenge(), None);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn a_fresh_board_is_saved_at_once_so_a_restart_keeps_it() {
        let path = temp_path("fresh-board");
        let _ = fs::remove_file(&path);
        let first = SharedProfile::load(&path).board();

        let saved: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&path).expect("the saved profile"))
                .expect("valid json");
        assert_eq!(
            saved["progress"]["missions"].as_array().map(Vec::len),
            Some(3)
        );
        assert_eq!(SharedProfile::load(&path).board(), first);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn a_profile_from_a_newer_game_gets_a_board_in_memory_but_stays_untouched() {
        let path = temp_path("newer-board");
        let text = r#"{"version": 99, "balance": 5}"#;
        fs::write(&path, text).expect("a temp file");

        let shared = SharedProfile::load(&path);

        assert_eq!(shared.board().missions.len(), 3);
        assert_eq!(fs::read_to_string(&path).expect("the file"), text);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn an_unknown_mission_is_dropped_and_the_board_is_refilled() {
        let path = temp_path("unknown-mission");
        fs::write(
            &path,
            r#"{"version": 1, "progress": {"seed": 7, "missions": [{"goal": "removed_goal", "target": 5, "reward": 1}, {"goal": "kills", "target": 50, "reward": 2, "progress": 3}]}}"#,
        )
        .expect("a temp file");

        let board = SharedProfile::load(&path).board();

        assert_eq!(board.missions.len(), 3);
        assert!(board
            .missions
            .iter()
            .any(|note| note.progress == 3 && note.goal == 50));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn a_garbage_progress_keeps_the_rest_of_the_profile() {
        let path = temp_path("bad-progress");
        fs::write(&path, r#"{"version": 1, "balance": 30, "progress": 12}"#).expect("a temp file");
        let shared = SharedProfile::load(&path);
        assert_eq!(shared.balance(), 30);
        assert_eq!(shared.board().missions.len(), 3);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn the_selected_challenge_and_its_reward_survive_a_restart() {
        let path = temp_path("challenge");
        let _ = fs::remove_file(&path);
        let shared = SharedProfile::load(&path);
        shared.toggle_challenge(ChallengeId::Horde);
        let (stats, _) = run(1, 4);
        let outcome = shared.finish_run(&RunRecord {
            stats,
            floor: 4,
            victory: false,
            challenge: shared.challenge(),
        });
        assert_eq!(outcome.challenge, Some(("Horde", 4)));
        assert_eq!(shared.balance(), 4);
        let missions = shared.board().missions;
        assert_eq!(missions.len(), 3);

        let reloaded = SharedProfile::load(&path);
        assert_eq!(reloaded.challenge(), Some(ChallengeId::Horde));
        assert_eq!(reloaded.balance(), 4);
        assert!(reloaded
            .board()
            .challenges
            .iter()
            .any(|note| note.title == "Horde" && note.done));
        assert_eq!(reloaded.board().missions, missions);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn claiming_a_mission_pays_talents_into_the_balance() {
        let shared = SharedProfile::in_memory();
        assert_eq!(shared.claim_mission(0), None);
        let mut stats = RunStats {
            kills: 10_000,
            gold: 10_000,
            items: 10_000,
            untouched_floors: 10,
            ..RunStats::default()
        };
        for kind in crate::enemies::EnemyKind::ALL {
            stats.kills_of[kind.index()] = 10_000;
        }
        shared.finish_run(&RunRecord {
            stats,
            floor: 5,
            victory: true,
            challenge: None,
        });
        let paid = shared.claim_mission(0).expect("a finished mission");
        assert_eq!(shared.balance(), u64::from(paid));
    }

    #[test]
    fn an_in_memory_profile_never_touches_the_disk() {
        let shared = SharedProfile::in_memory();
        let (stats, summary) = run(3, 2);
        shared.record_run(stats, &summary, false);
        assert!(shared.gold() > 0);
    }
}
