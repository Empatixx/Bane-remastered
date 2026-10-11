use glam::Vec2;

use crate::audio::SoundEffect;

pub(crate) const MAX_PELLETS: usize = 4;
pub(crate) const MAX_CLIP: usize = 32;
pub(crate) const PIP_SIZE: Vec2 = Vec2::new(24.0, 24.0);
pub(crate) const BURST_DELAY: f32 = 0.05;

const PIPS_RIGHT: f32 = 922.5;
const PIPS_RIGHT_SUBMACHINE: f32 = 936.0;
const PIPS_BOTTOM: f32 = 515.0;
const PIPS_STEP: f32 = 15.0;
const PIPS_STEP_SHOTGUN: f32 = 20.0;
const PIPS_COLUMN_STEP: f32 = 25.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum GunKind {
    Pistol,
    Shotgun,
    Luger,
    Revolver,
    M4,
    Uzi,
    Thompson,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ClipStyle {
    Pistol,
    Shotgun,
    SubMachine,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct GunSpec {
    pub(crate) texture: &'static str,
    pub(crate) icon: Vec2,
    pub(crate) min_damage: i32,
    pub(crate) max_damage: i32,
    pub(crate) inaccuracy: f32,
    pub(crate) fire_interval: f32,
    pub(crate) reload_time: f32,
    pub(crate) clip_size: i32,
    pub(crate) max_ammo: i32,
    pub(crate) auto: bool,
    pub(crate) pellets: usize,
    pub(crate) burst: Option<u32>,
    pub(crate) shoot: SoundEffect,
    pub(crate) reload: SoundEffect,
    pub(crate) clip_style: ClipStyle,
}

const fn gun(
    art: (&'static str, (f32, f32)),
    damage: (i32, i32),
    inaccuracy: f32,
    timing: (f32, f32),
    ammo: (i32, i32),
    sounds: (SoundEffect, SoundEffect),
    clip_style: ClipStyle,
) -> GunSpec {
    GunSpec {
        texture: art.0,
        icon: Vec2::new(art.1 .0, art.1 .1),
        min_damage: damage.0,
        max_damage: damage.1,
        inaccuracy,
        fire_interval: timing.0,
        reload_time: timing.1,
        clip_size: ammo.0,
        max_ammo: ammo.1,
        auto: false,
        pellets: 1,
        burst: None,
        shoot: sounds.0,
        reload: sounds.1,
        clip_style,
    }
}

impl GunKind {
    pub(crate) const ALL: [Self; 7] = [
        Self::Pistol,
        Self::Shotgun,
        Self::Luger,
        Self::Revolver,
        Self::M4,
        Self::Uzi,
        Self::Thompson,
    ];

    pub(crate) fn spec(self) -> GunSpec {
        use ClipStyle::{Pistol, Shotgun, SubMachine};
        use SoundEffect as S;
        match self {
            Self::Pistol => gun(
                ("ingame/guns/pistol.tga", (41.0, 34.0)),
                (1, 3),
                0.1,
                (0.25, 0.7),
                (7, 120),
                (S::PistolShoot, S::PistolReload),
                Pistol,
            ),
            Self::Shotgun => GunSpec {
                pellets: MAX_PELLETS,
                ..gun(
                    ("ingame/guns/shotgun.tga", (97.0, 36.0)),
                    (3, 5),
                    0.1,
                    (0.5, 0.7),
                    (6, 48),
                    (S::ShotgunShoot, S::ShotgunReload),
                    Shotgun,
                )
            },
            Self::Luger => gun(
                ("ingame/guns/luger.tga", (50.0, 38.0)),
                (1, 2),
                0.35,
                (0.25, 0.7),
                (9, 150),
                (S::PistolShoot, S::PistolReload),
                Pistol,
            ),
            Self::Revolver => gun(
                ("ingame/guns/revolver.tga", (71.0, 44.0)),
                (4, 6),
                0.2,
                (0.45, 0.7),
                (6, 60),
                (S::RevolverShoot, S::PistolReload),
                Pistol,
            ),
            Self::M4 => GunSpec {
                auto: true,
                burst: Some(4),
                ..gun(
                    ("ingame/guns/M4.tga", (98.0, 52.0)),
                    (2, 2),
                    0.0,
                    (0.75, 1.3),
                    (32, 250),
                    (S::M4Shoot, S::M4Reload),
                    SubMachine,
                )
            },
            Self::Uzi => GunSpec {
                auto: true,
                ..gun(
                    ("ingame/guns/uzi.tga", (66.0, 58.0)),
                    (1, 2),
                    0.4,
                    (0.1, 0.7),
                    (20, 200),
                    (S::UziShoot, S::PistolReload),
                    SubMachine,
                )
            },
            Self::Thompson => GunSpec {
                auto: true,
                ..gun(
                    ("ingame/guns/thompson.tga", (97.0, 47.0)),
                    (1, 3),
                    0.05,
                    (0.2, 0.85),
                    (30, 300),
                    (S::PistolShoot, S::PistolReload),
                    SubMachine,
                )
            },
        }
    }
}

impl GunSpec {
    pub(crate) fn damage_for(&self, roll: f32) -> i32 {
        self.min_damage + (roll * (self.max_damage - self.min_damage) as f32) as i32
    }

    pub(crate) fn angle_offset(&self, pellet: usize, spread: f32) -> f32 {
        if self.pellets > 1 {
            let sign = if pellet.is_multiple_of(2) { 1.0 } else { -1.0 };
            0.5 * self.inaccuracy * pellet as f32 * sign
        } else {
            0.5 * self.inaccuracy * spread
        }
    }
}

pub(crate) fn pip_layout(style: ClipStyle, clip_size: usize) -> Vec<Vec2> {
    match style {
        ClipStyle::Pistol => (0..clip_size)
            .map(|index| Vec2::new(PIPS_RIGHT, PIPS_BOTTOM - index as f32 * PIPS_STEP))
            .collect(),
        ClipStyle::Shotgun => (0..clip_size)
            .map(|index| Vec2::new(PIPS_RIGHT, PIPS_BOTTOM - index as f32 * PIPS_STEP_SHOTGUN))
            .collect(),
        ClipStyle::SubMachine => {
            let half = clip_size / 2;
            (0..2)
                .flat_map(|column| {
                    (0..half).map(move |index| {
                        Vec2::new(
                            PIPS_RIGHT_SUBMACHINE - column as f32 * PIPS_COLUMN_STEP,
                            PIPS_BOTTOM - index as f32 * PIPS_STEP,
                        )
                    })
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_gun_fits_the_clip_hud() {
        for kind in GunKind::ALL {
            let spec = kind.spec();
            assert!(spec.clip_size as usize <= MAX_CLIP, "{kind:?}");
            let pips = pip_layout(spec.clip_style, spec.clip_size as usize);
            assert_eq!(pips.len(), spec.clip_size as usize, "{kind:?}");
            assert!(spec.pellets <= MAX_PELLETS);
        }
    }

    #[test]
    fn damage_never_reaches_the_maximum_like_in_bane() {
        let pistol = GunKind::Pistol.spec();
        assert_eq!(pistol.damage_for(0.0), 1);
        assert_eq!(pistol.damage_for(0.99), 2);
        let m4 = GunKind::M4.spec();
        assert_eq!(m4.damage_for(0.7), 2);
    }

    #[test]
    fn shotgun_pellets_fan_out_alternating_sides() {
        let shotgun = GunKind::Shotgun.spec();
        let offsets: Vec<f32> = (0..4).map(|i| shotgun.angle_offset(i, 0.9)).collect();
        assert!((offsets[0]).abs() < 1e-6);
        assert!((offsets[1] + 0.05).abs() < 1e-6);
        assert!((offsets[2] - 0.1).abs() < 1e-6);
        assert!((offsets[3] + 0.15).abs() < 1e-6);
    }

    #[test]
    fn single_bullets_use_half_the_inaccuracy_like_bane() {
        let pistol = GunKind::Pistol.spec();
        assert!((pistol.angle_offset(0, 1.0) - 0.05).abs() < 1e-6);
        assert!((pistol.angle_offset(0, -1.0) + 0.05).abs() < 1e-6);
        assert_eq!(GunKind::M4.spec().angle_offset(0, 1.0), 0.0);
    }

    #[test]
    fn submachine_guns_use_two_columns() {
        let pips = pip_layout(ClipStyle::SubMachine, 32);
        assert_eq!(pips.len(), 32);
        assert_eq!(pips[0], Vec2::new(936.0, 515.0));
        assert_eq!(pips[16], Vec2::new(911.0, 515.0));
        assert_eq!(pips[15], Vec2::new(936.0, 515.0 - 15.0 * 15.0));
    }

    #[test]
    fn only_the_m4_bursts_and_only_automatics_hold_the_trigger() {
        let bursting: Vec<GunKind> = GunKind::ALL
            .into_iter()
            .filter(|kind| kind.spec().burst.is_some())
            .collect();
        assert_eq!(bursting, vec![GunKind::M4]);
        let auto: Vec<GunKind> = GunKind::ALL
            .into_iter()
            .filter(|kind| kind.spec().auto)
            .collect();
        assert_eq!(auto, vec![GunKind::M4, GunKind::Uzi, GunKind::Thompson]);
    }
}
