use crate::assets::{Tex, tex};
use glam::{Vec2, Vec3, Vec4};

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
    pub(crate) texture: Tex,
    pub(crate) hole: [u32; 4],
    pub(crate) scale: f32,
    pub(crate) color: Vec3,
    pub(crate) position: Vec2,
}

impl BarKind {
    pub(crate) const ALL: [Self; 3] = [Self::Health, Self::Armor, Self::Boss];
    pub(crate) const HUD: [Self; 2] = [Self::Health, Self::Armor];

    /// The bar textures with the hole the loader cuts out of them.
    pub(crate) fn holed_textures() -> [(Tex, [u32; 4]); 3] {
        Self::ALL.map(|kind| {
            let spec = kind.spec();
            (spec.texture, spec.hole)
        })
    }

    pub(crate) fn spec(self) -> BarSpec {
        match self {
            Self::Health => BarSpec {
                texture: tex::HEALTH_BAR,
                hole: [19, 8, 56, 4],
                scale: 3.0,
                color: Vec3::new(0.529, 0.298, 0.262),
                position: Vec2::new(25.0, 25.0),
            },
            Self::Boss => BarSpec {
                texture: tex::BOSSHEALTHBAR,
                hole: [19, 8, 56, 4],
                scale: 4.0,
                color: Vec3::new(0.529, 0.298, 0.262),
                position: Vec2::new(292.0, 15.0),
            },
            Self::Armor => BarSpec {
                texture: tex::ARMOR_BAR,
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
pub(crate) struct GunIcon;

pub(crate) const AMMO_TEXT_COLOR: Vec4 =
    Vec4::new(226.0 / 255.0, 153.0 / 255.0, 117.0 / 255.0, 1.0);

#[derive(Debug, Clone, Copy)]
pub(crate) struct AmmoText;

#[derive(Debug, Clone, Copy)]
pub(crate) struct InfinityMark;

#[derive(Debug, Clone, Copy)]
pub(crate) struct AmmoPip {
    pub(crate) index: i32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct CoinText;

#[derive(Debug, Clone, Copy)]
pub(crate) struct TalentCount;

#[derive(Debug, Clone, Copy)]
pub(crate) struct GoldCount;

#[derive(Debug, Clone, Copy)]
pub(crate) struct EnemyArrow {
    pub(crate) angle: f32,
}

#[cfg(test)]
mod tests {
    use crate::viewport::TARGET;

    use super::*;

    #[test]
    fn the_loader_gets_every_bar_texture_with_its_hole() {
        let holed = BarKind::holed_textures();

        for (kind, (texture, hole)) in BarKind::ALL.iter().zip(holed) {
            assert_eq!(texture, kind.spec().texture);
            assert_eq!(hole, kind.spec().hole);
        }
    }

    #[test]
    fn the_boss_health_bar_is_centered_on_the_screen() {
        let spec = BarKind::Boss.spec();
        let fill_center =
            spec.position.x + (spec.hole[0] as f32 + spec.hole[2] as f32 / 2.0) * spec.scale;
        assert!((fill_center - TARGET.x / 2.0).abs() < 0.5, "{fill_center}");
    }
}
