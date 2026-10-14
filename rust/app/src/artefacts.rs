use crate::assets::{tex, Tex};
use glam::Vec2;

pub(crate) const BERSERK_SECONDS: f32 = 15.0;
pub(crate) const BERSERK_SPEED: f32 = 1.3;
pub(crate) const RING_BULLETS: usize = 50;
pub(crate) const RING_TURN: f32 = 0.15;
pub(crate) const RING_DAMAGE: i32 = 2;
pub(crate) const BERSERK_PHASE: f32 = 0.75;

pub(crate) const TIME_SLOW_SECONDS: f32 = 6.0;
#[allow(dead_code)]
pub(crate) const TIME_SLOW_SCALE: f32 = 0.35;
pub(crate) const BLOODLUST_SECONDS: f32 = 10.0;
pub(crate) const CHALICE_HEAL: f32 = 0.5;
pub(crate) const PHANTOM_SECONDS: f32 = 5.0;
pub(crate) const MIDAS_SECONDS: f32 = 6.0;
pub(crate) const INFINITE_AMMO_SECONDS: f32 = 8.0;

const ICON_SCALE: f32 = 2.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ArtefactKind {
    RingOfDeath,
    ArmorRefill,
    ShieldHorn,
    BerserkPot,
    RevealMap,
    HourglassOfAsh,
    BloodChalice,
    PhantomCloak,
    GraveLantern,
    MidasCoin,
    ThunderIdol,
    InfinityQuiver,
    BoneTurret,
}

impl ArtefactKind {
    pub(crate) const ALL: [Self; 13] = [
        Self::RingOfDeath,
        Self::ArmorRefill,
        Self::ShieldHorn,
        Self::BerserkPot,
        Self::RevealMap,
        Self::HourglassOfAsh,
        Self::BloodChalice,
        Self::PhantomCloak,
        Self::GraveLantern,
        Self::MidasCoin,
        Self::ThunderIdol,
        Self::InfinityQuiver,
        Self::BoneTurret,
    ];

    pub(crate) fn texture(self) -> Tex {
        match self {
            Self::RingOfDeath => tex::ARTEFACTS_RINGOFDEATH,
            Self::ArmorRefill => tex::ARTEFACTS_ARMORREFILL,
            Self::ShieldHorn => tex::ARTEFACTS_SHIELDHORN,
            Self::BerserkPot => tex::ARTEFACTS_BERSERKPOT,
            Self::RevealMap => tex::ARTEFACTS_REVEAL_MAP,
            Self::HourglassOfAsh => tex::ARTEFACTS_HOURGLASS_OF_ASH,
            Self::BloodChalice => tex::ARTEFACTS_BLOOD_CHALICE,
            Self::PhantomCloak => tex::ARTEFACTS_PHANTOM_CLOAK,
            Self::GraveLantern => tex::ARTEFACTS_GRAVE_LANTERN,
            Self::MidasCoin => tex::ARTEFACTS_MIDAS_COIN,
            Self::ThunderIdol => tex::ARTEFACTS_THUNDER_IDOL,
            Self::InfinityQuiver => tex::ARTEFACTS_INFINITY_QUIVER,
            Self::BoneTurret => tex::ARTEFACTS_BONE_TURRET,
        }
    }

    pub(crate) fn icon(self) -> Vec2 {
        let source = match self {
            Self::RingOfDeath => Vec2::new(25.0, 25.0),
            Self::ShieldHorn => Vec2::new(34.0, 32.0),
            _ => Vec2::new(32.0, 32.0),
        };
        source * ICON_SCALE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icons_are_drawn_at_double_size() {
        assert_eq!(ArtefactKind::RingOfDeath.icon(), Vec2::new(50.0, 50.0));
        assert_eq!(ArtefactKind::ShieldHorn.icon(), Vec2::new(68.0, 64.0));
        assert_eq!(ArtefactKind::RevealMap.icon(), Vec2::new(64.0, 64.0));
    }

    #[test]
    fn there_are_thirteen_distinct_artefacts() {
        let textures: std::collections::HashSet<_> = ArtefactKind::ALL
            .iter()
            .map(|kind| kind.texture())
            .collect();
        assert_eq!(textures.len(), 13);
    }
}
