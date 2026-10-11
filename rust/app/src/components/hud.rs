use glam::{Vec2, Vec3};

use super::animation::Clip;

pub(crate) const PIP_FULL: Clip = Clip {
    row: 0,
    frames: 1,
    frame_time: 1.0,
    looping: true,
    flipped: false,
};

pub(crate) const PIP_EMPTY: Clip = Clip {
    row: 1,
    frames: 2,
    frame_time: 0.1,
    looping: false,
    flipped: false,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BarKind {
    Health,
    Armor,
    Boss,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BarSpec {
    pub(crate) texture: &'static str,
    pub(crate) hole: [u32; 4],
    pub(crate) scale: f32,
    pub(crate) color: Vec3,
    pub(crate) position: Vec2,
}

impl BarKind {
    pub(crate) const ALL: [Self; 3] = [Self::Health, Self::Armor, Self::Boss];
    pub(crate) const HUD: [Self; 2] = [Self::Health, Self::Armor];

    pub(crate) fn spec(self) -> BarSpec {
        match self {
            Self::Health => BarSpec {
                texture: "ingame/healthBar.tga",
                hole: [19, 8, 56, 4],
                scale: 3.0,
                color: Vec3::new(0.529, 0.298, 0.262),
                position: Vec2::new(25.0, 25.0),
            },
            Self::Boss => BarSpec {
                texture: "ingame/bosshealthbar.tga",
                hole: [19, 8, 56, 4],
                scale: 4.0,
                color: Vec3::new(0.529, 0.298, 0.262),
                position: Vec2::new(320.0, 15.0),
            },
            Self::Armor => BarSpec {
                texture: "ingame/armorBar.tga",
                hole: [21, 12, 55, 4],
                scale: 2.0,
                color: Vec3::new(0.603, 0.670, 0.709),
                position: Vec2::new(75.0, 62.5),
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct StatusBar {
    pub(crate) kind: BarKind,
    pub(crate) fill: f32,
    pub(crate) delayed: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BossHud;

#[derive(Debug, Clone, Copy)]
pub(crate) struct AmmoText;

#[derive(Debug, Clone, Copy)]
pub(crate) struct AmmoPip {
    pub(crate) index: i32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct CoinText;
