use glam::Vec2;

use crate::assets::{tex, Tex};
use crate::audio::SoundEffect;

pub(crate) const MAX_PELLETS: usize = 4;
pub(crate) const MAX_CLIP: usize = 40;
pub(crate) const PIP_SIZE: Vec2 = Vec2::new(24.0, 24.0);
pub(crate) const BURST_DELAY: f32 = 0.05;

const PIPS_RIGHT: f32 = 922.5;
const PIPS_RIGHT_SUBMACHINE: f32 = 936.0;
const PIPS_BOTTOM: f32 = 515.0;
const PIPS_STEP: f32 = 15.0;
const PIPS_STEP_SHOTGUN: f32 = 20.0;
const PIPS_COLUMN_STEP: f32 = 25.0;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub(crate) enum GunKind {
    Pistol,
    Shotgun,
    Luger,
    Revolver,
    M4,
    Uzi,
    Thompson,
    Sniper,
    Rpg,
    Minigun,
    Crossbow,
    Flamethrower,
    Blaster,
    PlasmaRifle,
    TeslaGun,
    Railgun,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// What a round does on top of its damage, unlocked at the end of a weapon tree.
pub(crate) enum Mutation {
    Piercing,
    Explosive,
    Venom,
    Frost,
    Ricochet,
    Deadeye,
}

impl Mutation {
    pub(crate) const ALL: [Self; 6] = [
        Self::Piercing,
        Self::Explosive,
        Self::Venom,
        Self::Frost,
        Self::Ricochet,
        Self::Deadeye,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ClipStyle {
    Pistol,
    Shotgun,
    SubMachine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// What leaves the barrel: how it looks, how fast it flies and what it does when it lands.
pub(crate) enum Round {
    Bullet,
    Tracer,
    Slug,
    Rocket,
    Bolt,
    Flame,
    Laser,
    Plasma,
    Spark,
    Rail,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct RoundSpec {
    pub(crate) texture: Tex,
    pub(crate) size: Vec2,
    pub(crate) hitbox: Vec2,
    pub(crate) speed: f32,
    /// How many enemies it passes through before it stops.
    pub(crate) pierce: u32,
    /// Seconds it flies before it burns out on its own.
    pub(crate) lifetime: Option<f32>,
    pub(crate) blast: bool,
    /// Hurts what stands around the hit with half the damage, without a fiery blast.
    pub(crate) splash: bool,
    /// How many more enemies a hit jumps to as lightning.
    pub(crate) chains: u32,
    pub(crate) light: [f32; 3],
    /// Motes the round sheds behind itself while it flies.
    pub(crate) trail: Option<Trail>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Trail {
    pub(crate) texture: Tex,
    pub(crate) tint: [f32; 3],
    pub(crate) size: f32,
    pub(crate) life: f32,
    /// How far sideways a mote is thrown, in pixels per second.
    pub(crate) spread: f32,
    /// Seconds between two motes.
    pub(crate) every: f32,
}

impl Trail {
    pub(crate) const VOLT: Self = Self {
        texture: tex::ROUND_MOTE_VOLT,
        tint: [0.65, 0.85, 1.0],
        size: 14.0,
        life: 0.35,
        spread: 90.0,
        every: 0.02,
    };
    pub(crate) const LASER: Self = Self {
        texture: tex::ROUND_MOTE_LASER,
        tint: [1.0, 0.5, 0.55],
        size: 12.0,
        life: 0.3,
        spread: 20.0,
        every: 0.015,
    };
}

impl Round {
    pub(crate) const ALL: [Self; 10] = [
        Self::Bullet,
        Self::Tracer,
        Self::Slug,
        Self::Rocket,
        Self::Bolt,
        Self::Flame,
        Self::Laser,
        Self::Plasma,
        Self::Spark,
        Self::Rail,
    ];

    pub(crate) fn spec(self) -> RoundSpec {
        let round = |texture, size: (f32, f32), speed| RoundSpec {
            texture,
            size: Vec2::new(size.0, size.1),
            hitbox: Vec2::splat(16.0),
            speed,
            pierce: 0,
            lifetime: None,
            blast: false,
            splash: false,
            chains: 0,
            light: [255.0, 0.0, 0.0],
            trail: None,
        };
        match self {
            Self::Bullet => round(tex::BULLET, (16.0, 16.0), 700.0),
            Self::Tracer => RoundSpec {
                hitbox: Vec2::splat(12.0),
                light: [255.0, 200.0, 0.0],
                ..round(tex::ROUND_TRACER, (16.0, 16.0), 900.0)
            },
            Self::Slug => RoundSpec {
                hitbox: Vec2::splat(12.0),
                pierce: 2,
                light: [80.0, 200.0, 255.0],
                ..round(tex::ROUND_SLUG, (32.0, 16.0), 1500.0)
            },
            Self::Rocket => RoundSpec {
                blast: true,
                light: [255.0, 140.0, 0.0],
                ..round(tex::ROUND_ROCKET, (24.0, 24.0), 480.0)
            },
            Self::Bolt => RoundSpec {
                hitbox: Vec2::splat(12.0),
                pierce: 1,
                light: [255.0, 120.0, 60.0],
                ..round(tex::ROUND_BOLT, (32.0, 16.0), 950.0)
            },
            Self::Flame => RoundSpec {
                hitbox: Vec2::splat(20.0),
                pierce: u32::MAX,
                lifetime: Some(0.45),
                light: [255.0, 110.0, 0.0],
                ..round(tex::ROUND_FLAME, (24.0, 24.0), 380.0)
            },
            Self::Laser => RoundSpec {
                hitbox: Vec2::splat(12.0),
                light: [255.0, 30.0, 40.0],
                trail: Some(Trail::LASER),
                ..round(tex::ROUND_LASER, (32.0, 16.0), 1300.0)
            },
            Self::Plasma => RoundSpec {
                splash: true,
                light: [60.0, 255.0, 90.0],
                ..round(tex::ROUND_PLASMA, (24.0, 24.0), 650.0)
            },
            Self::Spark => RoundSpec {
                chains: 2,
                light: [80.0, 160.0, 255.0],
                trail: Some(Trail::VOLT),
                ..round(tex::ROUND_SPARK, (24.0, 24.0), 600.0)
            },
            Self::Rail => RoundSpec {
                hitbox: Vec2::splat(24.0),
                pierce: u32::MAX,
                light: [60.0, 220.0, 240.0],
                ..round(tex::ROUND_RAIL, (48.0, 16.0), 1800.0)
            },
        }
    }

    /// Long rounds point where they fly; the round bullet looks the same from every side.
    pub(crate) fn turns(self) -> bool {
        !matches!(
            self,
            Self::Bullet | Self::Flame | Self::Plasma | Self::Spark
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct GunSpec {
    pub(crate) texture: Tex,
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
    pub(crate) shake: f32,
    pub(crate) damage_scale: f32,
    pub(crate) crit_chance: f32,
    pub(crate) mutation: Option<Mutation>,
    pub(crate) round: Round,
}

const fn gun(
    art: (Tex, (f32, f32)),
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
        shake: 0.35,
        damage_scale: 1.0,
        crit_chance: 0.0,
        mutation: None,
        round: Round::Bullet,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AmmoClass {
    Pistol,
    Shotgun,
}

impl GunKind {
    pub(crate) fn ammo_class(self) -> AmmoClass {
        if self == Self::Shotgun {
            AmmoClass::Shotgun
        } else {
            AmmoClass::Pistol
        }
    }

    pub(crate) const ALL: [Self; 16] = [
        Self::Pistol,
        Self::Shotgun,
        Self::Luger,
        Self::Revolver,
        Self::M4,
        Self::Uzi,
        Self::Thompson,
        Self::Sniper,
        Self::Rpg,
        Self::Minigun,
        Self::Crossbow,
        Self::Flamethrower,
        Self::Blaster,
        Self::PlasmaRifle,
        Self::TeslaGun,
        Self::Railgun,
    ];

    /// The stable name of the weapon, the same one the profile is saved under.
    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::Pistol => "pistol",
            Self::Shotgun => "shotgun",
            Self::Luger => "luger",
            Self::Revolver => "revolver",
            Self::M4 => "m4",
            Self::Uzi => "uzi",
            Self::Thompson => "thompson",
            Self::Sniper => "sniper",
            Self::Rpg => "rpg",
            Self::Minigun => "minigun",
            Self::Crossbow => "crossbow",
            Self::Flamethrower => "flamethrower",
            Self::Blaster => "blaster",
            Self::PlasmaRifle => "plasma_rifle",
            Self::TeslaGun => "tesla_gun",
            Self::Railgun => "railgun",
        }
    }

    pub(crate) fn spec(self) -> GunSpec {
        use ClipStyle::{Pistol, Shotgun, SubMachine};
        use SoundEffect as S;
        match self {
            Self::Pistol => gun(
                (tex::GUNS_PISTOL, (40.0, 34.0)),
                (1, 3),
                0.1,
                (0.25, 0.7),
                (7, 120),
                (S::PistolShoot, S::PistolReload),
                Pistol,
            ),
            Self::Shotgun => GunSpec {
                pellets: MAX_PELLETS,
                shake: 0.6,
                ..gun(
                    (tex::GUNS_SHOTGUN, (96.0, 36.0)),
                    (3, 5),
                    0.1,
                    (0.5, 0.7),
                    (6, 48),
                    (S::ShotgunShoot, S::ShotgunReload),
                    Shotgun,
                )
            },
            Self::Luger => GunSpec {
                shake: 0.3,
                ..gun(
                    (tex::GUNS_LUGER, (50.0, 38.0)),
                    (1, 2),
                    0.35,
                    (0.25, 0.7),
                    (9, 150),
                    (S::PistolShoot, S::PistolReload),
                    Pistol,
                )
            },
            Self::Revolver => GunSpec {
                shake: 0.5,
                ..gun(
                    (tex::GUNS_REVOLVER, (70.0, 44.0)),
                    (4, 6),
                    0.2,
                    (0.45, 0.7),
                    (6, 60),
                    (S::RevolverShoot, S::PistolReload),
                    Pistol,
                )
            },
            Self::M4 => GunSpec {
                auto: true,
                burst: Some(4),
                shake: 0.2,
                ..gun(
                    (tex::GUNS_M4, (98.0, 52.0)),
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
                shake: 0.12,
                ..gun(
                    (tex::GUNS_UZI, (66.0, 58.0)),
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
                shake: 0.18,
                ..gun(
                    (tex::GUNS_THOMPSON, (98.0, 46.0)),
                    (1, 3),
                    0.05,
                    (0.2, 0.85),
                    (30, 300),
                    (S::PistolShoot, S::PistolReload),
                    SubMachine,
                )
            },
            Self::Sniper => GunSpec {
                shake: 0.8,
                crit_chance: 0.2,
                round: Round::Slug,
                ..gun(
                    (tex::GUNS_SNIPER, (128.0, 34.0)),
                    (9, 13),
                    0.0,
                    (1.1, 1.6),
                    (5, 40),
                    (S::SniperShoot, S::SniperReload),
                    Shotgun,
                )
            },
            Self::Rpg => GunSpec {
                shake: 1.2,
                round: Round::Rocket,
                ..gun(
                    (tex::GUNS_RPG, (122.0, 36.0)),
                    (8, 11),
                    0.05,
                    (1.4, 1.8),
                    (1, 12),
                    (S::RpgShoot, S::RpgReload),
                    Shotgun,
                )
            },
            Self::Minigun => GunSpec {
                auto: true,
                shake: 0.06,
                round: Round::Tracer,
                ..gun(
                    (tex::GUNS_MINIGUN, (104.0, 46.0)),
                    (1, 2),
                    0.3,
                    (0.06, 2.2),
                    (40, 400),
                    (S::MinigunShoot, S::MinigunReload),
                    SubMachine,
                )
            },
            Self::Crossbow => GunSpec {
                shake: 0.25,
                round: Round::Bolt,
                ..gun(
                    (tex::GUNS_CROSSBOW, (88.0, 42.0)),
                    (5, 8),
                    0.0,
                    (0.6, 1.0),
                    (3, 45),
                    (S::CrossbowShoot, S::CrossbowReload),
                    Shotgun,
                )
            },
            Self::Flamethrower => GunSpec {
                auto: true,
                shake: 0.03,
                round: Round::Flame,
                ..gun(
                    (tex::GUNS_FLAMETHROWER, (98.0, 44.0)),
                    (1, 2),
                    0.5,
                    (0.07, 1.5),
                    (40, 400),
                    (S::FlamethrowerShoot, S::FlamethrowerReload),
                    SubMachine,
                )
            },
            Self::Blaster => GunSpec {
                shake: 0.2,
                round: Round::Laser,
                ..gun(
                    (tex::GUNS_BLASTER, (56.0, 38.0)),
                    (2, 4),
                    0.05,
                    (0.22, 1.0),
                    (12, 144),
                    (S::BlasterShoot, S::BlasterReload),
                    Pistol,
                )
            },
            Self::PlasmaRifle => GunSpec {
                auto: true,
                shake: 0.12,
                round: Round::Plasma,
                ..gun(
                    (tex::GUNS_PLASMA_RIFLE, (104.0, 46.0)),
                    (2, 3),
                    0.12,
                    (0.16, 1.4),
                    (24, 240),
                    (S::PlasmaShoot, S::PlasmaReload),
                    SubMachine,
                )
            },
            Self::TeslaGun => GunSpec {
                shake: 0.3,
                round: Round::Spark,
                ..gun(
                    (tex::GUNS_TESLA_GUN, (96.0, 50.0)),
                    (3, 5),
                    0.1,
                    (0.55, 1.2),
                    (8, 80),
                    (S::TeslaShoot, S::TeslaReload),
                    Shotgun,
                )
            },
            Self::Railgun => GunSpec {
                shake: 1.0,
                round: Round::Rail,
                ..gun(
                    (tex::GUNS_RAILGUN, (132.0, 40.0)),
                    (14, 20),
                    0.0,
                    (1.6, 2.0),
                    (3, 24),
                    (S::RailgunShoot, S::RailgunReload),
                    Shotgun,
                )
            },
        }
    }
}

impl GunSpec {
    pub(crate) fn damage_for(&self, roll: f32) -> i32 {
        self.min_damage + (roll * (self.max_damage - self.min_damage) as f32) as i32
    }

    /// The damage with the bonus of the weapon tree; the fraction the bonus adds becomes a
    /// whole point as often as it is large, so small bonuses still count on weak guns.
    pub(crate) fn scaled_damage(&self, roll: f32, fraction_roll: f32) -> i32 {
        let damage = self.damage_for(roll) as f32 * self.damage_scale;
        let whole = damage.floor();
        whole as i32 + i32::from(damage - whole > fraction_roll)
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
            let half = clip_size.div_ceil(2);
            (0..2)
                .flat_map(|column| {
                    let rows = if column == 0 { half } else { clip_size - half };
                    (0..rows).map(move |index| {
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
    fn holding_the_trigger_never_pins_the_screen_shake() {
        for kind in GunKind::ALL {
            let spec = kind.spec();
            let per_second = spec.shake / spec.fire_interval;
            assert!(
                per_second < crate::camera::TRAUMA_DECAY,
                "{kind:?} adds {per_second} trauma a second"
            );
        }
    }

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
    fn an_odd_clip_still_shows_a_pip_for_every_round() {
        for clip in [33, 35, 40] {
            assert_eq!(pip_layout(ClipStyle::SubMachine, clip).len(), clip);
        }
    }

    #[test]
    fn the_tree_bonus_turns_its_fraction_into_whole_points_by_chance() {
        let pistol = GunSpec {
            damage_scale: 1.25,
            ..GunKind::Pistol.spec()
        };
        assert_eq!(pistol.scaled_damage(0.0, 0.1), 2);
        assert_eq!(pistol.scaled_damage(0.0, 0.9), 1);
        assert_eq!(GunKind::Pistol.spec().scaled_damage(0.0, 0.0), 1);
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
        assert_eq!(
            auto,
            vec![
                GunKind::M4,
                GunKind::Uzi,
                GunKind::Thompson,
                GunKind::Minigun,
                GunKind::Flamethrower,
                GunKind::PlasmaRifle
            ]
        );
    }
}
