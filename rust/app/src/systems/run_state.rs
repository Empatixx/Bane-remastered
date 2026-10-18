use crate::lore::Identity;
use crate::progress::ChallengeId;
use crate::rng::{Rng, Stream};
use crate::rules::RunRules;

use super::{Atmosphere, Fx, StirSeed};

/// What a run is besides its entities: which run and floor it is, the rules it is played by,
/// the floor's dice and the counters the systems keep between frames. Owned by the in-game
/// scene in its [`Resources`]; the systems that need a piece of it are handed that piece.
///
/// Each field says how long it lasts. "Run" means set by [`RunState::begin`] and kept across
/// floors; "floor" means set again for every floor by [`RunState::begin_floor`].
#[derive(Debug, Default)]
pub(crate) struct RunState {
    /// The run's seed, which every floor's dice are drawn from. Run.
    pub(crate) seed: u64,
    /// The floor being played, from 1. Run: set when it begins, one more for every floor down.
    pub(crate) floor: u32,
    /// The rules the run is played by, from the profile. Run.
    pub(crate) rules: RunRules,
    /// The challenge the run is played under, if any. Run.
    pub(crate) challenge: Option<ChallengeId>,
    /// Whether the last boss fell. Run.
    pub(crate) victory: bool,
    /// The dice the floor is laid out and its waves are drawn with. Floor.
    pub(crate) map_rng: Rng,
    /// The dice for drops, shops, slots and the like. Floor.
    pub(crate) loot_rng: Rng,
    /// The dice for shots and hits. Floor.
    pub(crate) combat_rng: Rng,
    /// `BANE_SLAY`: how long until the boss is felled, for working on the story. Floor.
    pub(crate) slay_in: Option<f32>,
    /// Who the boss of this floor was, while it is being fought: set when its room is entered,
    /// taken when it falls. Never cleared otherwise, so it lasts as long as the scene.
    pub(crate) boss: Option<Identity>,
    /// Kills in a row that dropped nothing, which makes the next drop likelier. Never reset, so
    /// it lasts as long as the scene: across floors and across runs.
    pub(crate) kills_without_drop: u32,
}

impl RunState {
    /// A new run on `floor`: forgets the last run's seed, outcome, rules and challenge.
    pub(crate) fn begin(
        &mut self,
        seed: u64,
        floor: u32,
        rules: RunRules,
        challenge: Option<ChallengeId>,
    ) {
        self.seed = seed;
        self.floor = floor;
        self.victory = false;
        self.rules = rules;
        self.challenge = challenge;
    }

    /// A new floor of the run: its three dice drawn afresh from the run's seed and the floor.
    pub(crate) fn begin_floor(&mut self) {
        self.map_rng = Rng::stream(self.seed, self.floor, Stream::Map);
        self.loot_rng = Rng::stream(self.seed, self.floor, Stream::Loot);
        self.combat_rng = Rng::stream(self.seed, self.floor, Stream::Combat);
    }

    /// The seed the floor's air is drawn with.
    pub(crate) fn air_seed(&self) -> u64 {
        self.seed ^ u64::from(self.floor).rotate_left(32)
    }
}

/// What the floor's air and surfaces do on their own, between frames: the biome's weather and
/// the scatter of what the surfaces give off. Purely for the eye. Floor: a fresh floor starts
/// with a fresh one.
#[derive(Default)]
pub(crate) struct FloorAir {
    /// The floor's weather, once the floor is built.
    pub(crate) atmosphere: Option<Atmosphere>,
    /// The scatter for the embers, glints and bubbles off the surfaces.
    pub(crate) stir: StirSeed,
}

/// Everything the in-game systems share that is not an entity, kept apart by how long it lasts:
/// the run's state, the floor's air, and the signals of the frame.
#[derive(Default)]
pub(crate) struct Resources {
    /// Lasts the run (some fields a floor; see [`RunState`]).
    pub(crate) run: RunState,
    /// Lasts a floor.
    pub(crate) air: FloorAir,
    /// The camera trauma the systems gathered since the camera last took it: drained every
    /// fixed step and every frame.
    pub(crate) fx: Fx,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_run_forgets_the_last_one_but_not_the_drop_streak() {
        let mut run = RunState {
            victory: true,
            challenge: Some(ChallengeId::GlassCannon),
            kills_without_drop: 7,
            ..RunState::default()
        };
        run.begin(42, 1, RunRules::default(), None);
        assert_eq!((run.seed, run.floor, run.victory, run.challenge), (42, 1, false, None));
        assert_eq!(run.kills_without_drop, 7);
    }

    #[test]
    fn every_floor_draws_its_dice_from_the_run_seed_and_the_floor() {
        let mut run = RunState::default();
        run.begin(42, 1, RunRules::default(), None);
        run.begin_floor();
        let first = run.loot_rng.clone().float();
        run.loot_rng.float();
        run.begin_floor();
        assert_eq!(run.loot_rng.clone().float(), first, "the same floor, the same dice");
        run.floor += 1;
        run.begin_floor();
        assert_ne!(run.loot_rng.float(), first, "the next floor has its own");
        assert_eq!(
            run.map_rng.float(),
            Rng::stream(42, 2, Stream::Map).float(),
            "the map dice are the run's stream for the floor"
        );
    }
}
