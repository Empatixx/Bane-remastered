use std::collections::BTreeSet;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};

use super::challenges::ChallengeId;
use super::missions::{self, ActiveMission};
use crate::components::RunStats;
use crate::rng::Rng;
use crate::rules::RunRules;

pub(crate) const MISSION_SLOTS: usize = 3;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct ProgressState {
    #[serde(deserialize_with = "known")]
    missions: Vec<ActiveMission>,
    #[serde(deserialize_with = "known_option")]
    selected: Option<ChallengeId>,
    #[serde(deserialize_with = "known")]
    completed: BTreeSet<ChallengeId>,
    seed: u64,
    draws: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct RunRecord {
    pub(crate) stats: RunStats,
    pub(crate) floor: u32,
    pub(crate) victory: bool,
    pub(crate) challenge: Option<ChallengeId>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct RunOutcome {
    pub(crate) missions_done: Vec<String>,
    pub(crate) challenge: Option<(&'static str, u32)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NoteView {
    pub(crate) title: String,
    pub(crate) detail: String,
    pub(crate) progress: u32,
    pub(crate) goal: u32,
    pub(crate) talents: u32,
    pub(crate) done: bool,
    pub(crate) active: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct BoardView {
    pub(crate) missions: Vec<NoteView>,
    pub(crate) challenges: Vec<NoteView>,
}

/// Reads a list, dropping the entries this version of the game does not know.
pub(crate) fn known<'de, D, T, C>(deserializer: D) -> Result<C, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
    C: FromIterator<T>,
{
    let values = Vec::<serde_json::Value>::deserialize(deserializer)?;
    Ok(values
        .into_iter()
        .filter_map(|value| match serde_json::from_value(value.clone()) {
            Ok(parsed) => Some(parsed),
            Err(error) => {
                tracing::warn!(%error, %value, "skipping an unknown progress entry");
                None
            }
        })
        .collect())
}

fn known_option<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(
        value.and_then(|value| match serde_json::from_value(value.clone()) {
            Ok(parsed) => Some(parsed),
            Err(error) => {
                tracing::warn!(%error, %value, "ignoring an unknown progress value");
                None
            }
        }),
    )
}

impl ProgressState {
    /// Gives a fresh profile its seed and keeps the board full; call after loading.
    /// Says whether anything changed, so the caller can save it.
    pub(crate) fn settle(&mut self, seed: u64) -> bool {
        let mut changed = false;
        if self.seed == 0 {
            self.seed = seed.max(1);
            changed = true;
        }
        if self.missions.len() > MISSION_SLOTS {
            self.missions.truncate(MISSION_SLOTS);
            changed = true;
        }
        while self.missions.len() < MISSION_SLOTS {
            let mission = self.draw();
            self.missions.push(mission);
            changed = true;
        }
        changed
    }

    fn draw(&mut self) -> ActiveMission {
        let taken: Vec<_> = self
            .missions
            .iter()
            .map(|mission| mission.goal.kind())
            .collect();
        let mut rng = Rng::new(self.seed ^ self.draws.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        self.draws = self.draws.wrapping_add(1);
        missions::draw(&mut rng, &taken)
    }

    pub(crate) fn completed(&self, id: ChallengeId) -> bool {
        self.completed.contains(&id)
    }

    pub(crate) fn selected(&self) -> Option<ChallengeId> {
        self.selected
    }

    pub(crate) fn rules(&self) -> RunRules {
        self.selected
            .map_or_else(RunRules::default, |id| id.spec().rules)
    }

    pub(crate) fn accent(&self) -> Option<[f32; 3]> {
        self.selected.map(|id| id.spec().accent)
    }

    pub(crate) fn toggle(&mut self, id: ChallengeId) {
        self.selected = if self.selected == Some(id) {
            None
        } else {
            Some(id)
        };
    }

    pub(crate) fn claim(&mut self, slot: usize) -> Option<u32> {
        let mission = *self.missions.get(slot)?;
        if !mission.done() {
            return None;
        }
        self.missions.remove(slot);
        let fresh = self.draw();
        self.missions.insert(slot, fresh);
        Some(mission.reward)
    }

    pub(crate) fn finish_run(&mut self, record: &RunRecord) -> RunOutcome {
        let mut outcome = RunOutcome::default();
        for mission in &mut self.missions {
            let was_done = mission.done();
            mission.advance(&record.stats, record.floor);
            if mission.done() && !was_done {
                outcome.missions_done.push(mission.title());
            }
        }
        if let Some(id) = record.challenge {
            if !self.completed.contains(&id) && id.reached(record.floor, record.victory) {
                self.completed.insert(id);
                let spec = id.spec();
                outcome.challenge = Some((spec.title, spec.reward));
            }
        }
        outcome
    }

    pub(crate) fn board(&self) -> BoardView {
        BoardView {
            missions: self
                .missions
                .iter()
                .map(|mission| NoteView {
                    title: mission.title(),
                    detail: mission.detail(),
                    progress: mission.progress.min(mission.target),
                    goal: mission.target,
                    talents: mission.reward,
                    done: mission.done(),
                    active: false,
                })
                .collect(),
            challenges: ChallengeId::ALL
                .into_iter()
                .map(|id| {
                    let spec = id.spec();
                    let done = self.completed.contains(&id);
                    NoteView {
                        title: spec.title.to_owned(),
                        detail: spec.detail.to_owned(),
                        progress: u32::from(done),
                        goal: 1,
                        talents: spec.reward,
                        done,
                        active: self.selected == Some(id),
                    }
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enemies::EnemyKind;

    fn settled() -> ProgressState {
        let mut state = ProgressState::default();
        assert!(state.settle(42));
        state
    }

    fn record(floor: u32, victory: bool, challenge: Option<ChallengeId>) -> RunRecord {
        let mut kills_of = [0; EnemyKind::ALL.len()];
        kills_of[EnemyKind::Rat.index()] = 500;
        let stats = RunStats {
            kills: 500,
            kills_of,
            gold: 1000,
            items: 100,
            untouched_floors: 3,
            ..RunStats::default()
        };
        RunRecord {
            stats,
            floor,
            victory,
            challenge,
        }
    }

    #[test]
    fn a_fresh_board_has_three_missions_of_different_kinds() {
        let state = settled();
        let kinds: Vec<_> = state.missions.iter().map(|m| m.goal.kind()).collect();
        assert_eq!(kinds.len(), MISSION_SLOTS);
        assert!(kinds
            .iter()
            .enumerate()
            .all(|(i, k)| !kinds[i + 1..].contains(k)));
    }

    #[test]
    fn claiming_pays_once_and_pins_a_fresh_mission_in_the_same_slot() {
        let mut state = settled();
        assert_eq!(state.claim(0), None, "nothing is done yet");
        state.missions[0].progress = state.missions[0].target;
        let reward = state.missions[0].reward;
        assert_eq!(state.claim(0), Some(reward));
        assert_eq!(state.missions.len(), MISSION_SLOTS);
        assert!(!state.missions[0].done());
        assert_eq!(state.claim(7), None);
    }

    #[test]
    fn a_finished_run_moves_the_missions_and_reports_the_new_ones() {
        let mut state = settled();
        let outcome = state.finish_run(&record(4, false, None));
        assert!(!outcome.missions_done.is_empty());
        assert_eq!(outcome.challenge, None);
    }

    #[test]
    fn a_challenge_pays_only_the_first_time_and_only_at_its_floor() {
        let mut state = settled();
        state.toggle(ChallengeId::Horde);
        assert_eq!(
            state
                .finish_run(&record(3, false, Some(ChallengeId::Horde)))
                .challenge,
            None
        );
        assert_eq!(
            state
                .finish_run(&record(4, false, Some(ChallengeId::Horde)))
                .challenge,
            Some(("Horde", 4))
        );
        assert_eq!(
            state
                .finish_run(&record(5, true, Some(ChallengeId::Horde)))
                .challenge,
            None
        );
        assert!(state
            .board()
            .challenges
            .iter()
            .any(|note| note.title == "Horde" && note.done));
    }

    #[test]
    fn toggling_selects_switches_and_cancels() {
        let mut state = settled();
        assert_eq!(state.rules(), RunRules::default());
        assert_eq!(state.accent(), None);
        state.toggle(ChallengeId::Horde);
        assert_eq!(state.selected(), Some(ChallengeId::Horde));
        state.toggle(ChallengeId::Bane);
        assert_eq!(state.selected(), Some(ChallengeId::Bane));
        assert_eq!(state.rules(), ChallengeId::Bane.spec().rules);
        state.toggle(ChallengeId::Bane);
        assert_eq!(state.selected(), None);
        assert_eq!(
            state
                .board()
                .challenges
                .iter()
                .filter(|note| note.active)
                .count(),
            0
        );
    }

    #[test]
    fn unknown_entries_in_a_save_are_dropped_and_the_board_refills() {
        let json = r#"{
            "missions": [{"goal": "kills", "target": 50, "reward": 1, "progress": 7},
                         {"goal": "removed_goal", "target": 1, "reward": 1}],
            "selected": "removed_challenge",
            "completed": ["horde", "removed_challenge"],
            "seed": 5
        }"#;
        let mut state: ProgressState = serde_json::from_str(json).expect("parsed");
        state.settle(9);
        assert_eq!(state.missions.len(), MISSION_SLOTS);
        assert_eq!(state.missions[0].progress, 7);
        assert_eq!(state.selected(), None);
        assert_eq!(state.completed, BTreeSet::from([ChallengeId::Horde]));
    }
}
