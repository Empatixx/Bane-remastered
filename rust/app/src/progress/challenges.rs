use serde::{Deserialize, Serialize};

use crate::guns::GunKind;
use crate::rules::RunRules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChallengeId {
    GlassCannon,
    PistolOnly,
    Horde,
    MerchantsStrike,
    TwoLeftFeet,
    Bane,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Challenge {
    pub(crate) id: ChallengeId,
    pub(crate) title: &'static str,
    pub(crate) detail: &'static str,
    pub(crate) target_floor: u32,
    pub(crate) reward: u32,
    pub(crate) accent: [f32; 3],
    pub(crate) rules: RunRules,
}

const HORDE: RunRules = RunRules {
    player_max_health: None,
    player_damage: 1.0,
    enemy_health: 1.6,
    enemy_damage: 1.3,
    only_gun: None,
    shops: true,
    dodge: true,
    armor_regen: true,
};
const NEUTRAL: RunRules = RunRules {
    enemy_health: 1.0,
    enemy_damage: 1.0,
    ..HORDE
};

impl ChallengeId {
    #[allow(dead_code)]
    pub(crate) const ALL: [Self; 6] = [
        Self::GlassCannon,
        Self::PistolOnly,
        Self::Horde,
        Self::MerchantsStrike,
        Self::TwoLeftFeet,
        Self::Bane,
    ];

    pub(crate) fn spec(self) -> Challenge {
        let (title, detail, target_floor, reward, accent, rules) = match self {
            Self::GlassCannon => (
                "Glass Cannon",
                "One heart, double damage. Reach floor 3",
                3,
                4,
                [255.0, 70.0, 70.0],
                RunRules {
                    player_max_health: Some(1),
                    player_damage: 2.0,
                    ..NEUTRAL
                },
            ),
            Self::PistolOnly => (
                "Pistol Only",
                "No gun but the pistol. Reach floor 3",
                3,
                3,
                [230.0, 200.0, 90.0],
                RunRules {
                    only_gun: Some(GunKind::Pistol),
                    ..NEUTRAL
                },
            ),
            Self::Horde => (
                "Horde",
                "Tougher, harder hitting enemies. Reach floor 4",
                4,
                4,
                [90.0, 200.0, 90.0],
                HORDE,
            ),
            Self::MerchantsStrike => (
                "Merchant's Strike",
                "No shops. Reach floor 4",
                4,
                3,
                [90.0, 160.0, 255.0],
                RunRules {
                    shops: false,
                    ..NEUTRAL
                },
            ),
            Self::TwoLeftFeet => (
                "Two Left Feet",
                "No dodging. Reach floor 3",
                3,
                3,
                [255.0, 150.0, 40.0],
                RunRules {
                    dodge: false,
                    ..NEUTRAL
                },
            ),
            Self::Bane => (
                "Bane",
                "Horde without armor regen. Beat the last boss",
                5,
                6,
                [200.0, 30.0, 30.0],
                RunRules {
                    armor_regen: false,
                    ..HORDE
                },
            ),
        };
        Challenge {
            id: self,
            title,
            detail,
            target_floor,
            reward,
            accent,
            rules,
        }
    }

    pub(crate) fn reached(self, floor: u32, victory: bool) -> bool {
        let target = self.spec().target_floor;
        if target >= crate::enemies::FINAL_FLOOR {
            victory
        } else {
            floor >= target
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_neutral_rules_are_the_default_rules() {
        assert_eq!(NEUTRAL, RunRules::default());
    }

    #[test]
    fn every_challenge_changes_the_rules_and_has_a_reachable_target() {
        for id in ChallengeId::ALL {
            let spec = id.spec();
            assert_ne!(spec.rules, RunRules::default(), "{id:?}");
            assert!(
                (2..=crate::enemies::FINAL_FLOOR).contains(&spec.target_floor),
                "{id:?}"
            );
            assert!(spec.reward > 0);
        }
    }

    #[test]
    fn the_final_floor_target_needs_the_victory_and_others_need_only_the_floor() {
        assert!(ChallengeId::GlassCannon.reached(3, false));
        assert!(!ChallengeId::GlassCannon.reached(2, false));
        assert!(!ChallengeId::Bane.reached(5, false));
        assert!(ChallengeId::Bane.reached(5, true));
    }
}
