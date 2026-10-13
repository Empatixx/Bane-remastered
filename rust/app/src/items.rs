use glam::Vec2;

use crate::assets::{tex, Tex};
use crate::components::Clip;

pub(crate) const DESPAWN_SECONDS: f32 = 10.0;
pub(crate) const BLINK_SECONDS: f32 = 2.0;
pub(crate) const BLINK_PERIOD: f32 = 0.2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum AmmoBox {
    Pistol,
    Shotgun,
    SubMachine,
    Universal,
}

impl AmmoBox {
    pub(crate) fn percent(self) -> f32 {
        match self {
            Self::Pistol | Self::SubMachine => 0.10,
            Self::Shotgun => 0.25,
            Self::Universal => 0.5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Vital {
    Health,
    Armor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PotionSize {
    Small,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ItemKind {
    Coin,
    Ammo(AmmoBox),
    Potion(Vital, PotionSize),
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ItemSpec {
    pub(crate) texture: Tex,
    pub(crate) columns: usize,
    pub(crate) rows: usize,
    pub(crate) size: Vec2,
    pub(crate) clip: Clip,
}

const fn clip(row: usize, frames: usize, frame_time: f32) -> Clip {
    Clip {
        row,
        frames,
        frame_time,
        looping: true,
        flipped: false,
    }
}

impl ItemKind {
    pub(crate) const COMMON: [Self; 4] = [
        Self::Coin,
        Self::Ammo(AmmoBox::Pistol),
        Self::Ammo(AmmoBox::Shotgun),
        Self::Ammo(AmmoBox::SubMachine),
    ];

    pub(crate) fn spec(self) -> ItemSpec {
        match self {
            Self::Coin => ItemSpec {
                texture: tex::BANECOIN_ANIM,
                columns: 4,
                rows: 1,
                size: Vec2::new(24.0, 24.0),
                clip: clip(0, 4, 0.1),
            },
            Self::Ammo(AmmoBox::Universal) => ItemSpec {
                texture: tex::AMMOBOX_UNIVERSAL,
                columns: 1,
                rows: 1,
                size: Vec2::new(48.0, 48.0),
                clip: clip(0, 1, 1.0),
            },
            Self::Ammo(box_kind) => ItemSpec {
                texture: if box_kind == AmmoBox::Shotgun {
                    tex::AMMOBOX_SHOTGUN
                } else {
                    tex::AMMOBOX_PISTOL
                },
                columns: 8,
                rows: 1,
                size: Vec2::new(36.0, 36.0),
                clip: clip(0, 8, 0.1),
            },
            Self::Potion(vital, PotionSize::Small) => ItemSpec {
                texture: tex::HEALING_POT,
                columns: 4,
                rows: 2,
                size: Vec2::new(60.0, 60.0),
                clip: clip(vital_row(vital), 4, 0.125),
            },
            Self::Potion(vital, PotionSize::Large) => ItemSpec {
                texture: tex::LARGE_HEALING_POT,
                columns: 1,
                rows: 2,
                size: Vec2::new(64.0, 64.0),
                clip: clip(vital_row(vital), 1, 1.0),
            },
        }
    }

    pub(crate) fn recover_amount(self) -> i32 {
        match self {
            Self::Potion(_, PotionSize::Small) => 1,
            Self::Potion(_, PotionSize::Large) => 4,
            _ => 0,
        }
    }
}

fn vital_row(vital: Vital) -> usize {
    match vital {
        Vital::Health => 0,
        Vital::Armor => 1,
    }
}

pub(crate) fn drop_threshold_reached(kills_without_drop: u32, roll: usize) -> bool {
    kills_without_drop as usize + roll > GUARANTEED_DROP
}

const GUARANTEED_DROP: usize = 4;
pub(crate) const DROP_ROLL_RANGE: usize = 6;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ammo_boxes_follow_bane_percentages() {
        assert_eq!(AmmoBox::Pistol.percent(), 0.10);
        assert_eq!(AmmoBox::Shotgun.percent(), 0.25);
        assert_eq!(AmmoBox::SubMachine.percent(), 0.10);
        assert_eq!(AmmoBox::Universal.percent(), 0.5);
    }

    #[test]
    fn potions_are_one_or_four_points() {
        assert_eq!(
            ItemKind::Potion(Vital::Health, PotionSize::Small).recover_amount(),
            1
        );
        assert_eq!(
            ItemKind::Potion(Vital::Armor, PotionSize::Large).recover_amount(),
            4
        );
        assert_eq!(ItemKind::Coin.recover_amount(), 0);
    }

    #[test]
    fn armor_potions_use_the_second_row() {
        let health = ItemKind::Potion(Vital::Health, PotionSize::Small).spec();
        let armor = ItemKind::Potion(Vital::Armor, PotionSize::Large).spec();
        assert_eq!((health.clip.row, armor.clip.row), (0, 1));
    }

    #[test]
    fn the_fifth_kill_always_drops_and_early_kills_only_sometimes() {
        assert!(drop_threshold_reached(5, 0));
        assert!(!drop_threshold_reached(1, 2));
        assert!(drop_threshold_reached(1, 4));
        assert!(!drop_threshold_reached(4, 0));
    }
}
