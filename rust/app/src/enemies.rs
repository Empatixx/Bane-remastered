use glam::Vec2;

use crate::assets::{Tex, tex};
use crate::components::Clip;
use crate::dungeon::Theme;
use crate::rng::Rng;

const DAMPING: f32 = 5.0;
const STEP_RATE: f32 = 60.0;
const HEALTH_GROWTH_PER_FLOOR: f32 = 0.12;
const DAMAGE_GROWTH_PER_FLOOR: f32 = 0.08;

pub(crate) const DASH_FORCE: f32 = 24_500_000.0;
pub(crate) const HEAL_RADIUS: f32 = 200.0;
pub(crate) const SMASH_COOLDOWN: f32 = 2.0;
pub(crate) const SMASH_BULLETS: usize = 5;
pub(crate) const SHOOT_COOLDOWN: f32 = 3.0;
pub(crate) const HATCH_COOLDOWN: f32 = 2.5;
pub(crate) const WEB_COOLDOWN: f32 = 5.0;
pub(crate) const ARMAGEDDON_COOLDOWN: f32 = 0.15;
pub(crate) const ARMAGEDDON_TURN: f32 = 0.25;
pub(crate) const KING_SMASH_BULLETS: usize = 25;
pub(crate) const COMMON_SPAWN_SECONDS: f32 = 2.0;
pub(crate) const BOSS_SPAWN_SECONDS: f32 = 1.5;
pub(crate) const FINAL_FLOOR: u32 = 5;
pub(crate) const GOLEM_ROCK_RING: usize = 16;
pub(crate) const GOLEM_BOULDER_DAMAGE: i32 = 2;
pub(crate) const GOLEM_ROCK_DAMAGE: i32 = 2;
pub(crate) const GOLEM_FIRST_COOLDOWN: f32 = 2.0;
pub(crate) const GOLEM_COOLDOWN: f32 = 2.2;
pub(crate) const GOLEM_RAGE_COOLDOWN: f32 = 1.3;
pub(crate) const GOLEM_RAGE_AT: f32 = 0.5;
pub(crate) const GOLEM_RAGE_SPEED: f32 = 1.35;
pub(crate) const GOLEM_SLAM_WINDUP: f32 = 0.45;
pub(crate) const GOLEM_SLAM_RECOVER: f32 = 0.45;
pub(crate) const GOLEM_SECOND_RING: f32 = 0.35;
pub(crate) const GOLEM_THROW_WINDUP: f32 = 0.45;
pub(crate) const GOLEM_THROW_RECOVER: f32 = 0.25;
pub(crate) const GOLEM_RAIN_WINDUP: f32 = 0.4;
pub(crate) const GOLEM_RAIN_RECOVER: f32 = 0.5;
pub(crate) const GOLEM_LASER_RECOVER: f32 = 0.6;
pub(crate) const GOLEM_RAIN_SECONDS: f32 = 2.4;
pub(crate) const GOLEM_RAIN_EVERY: f32 = 0.24;
pub(crate) const GOLEM_RAIN_DELAY: f32 = 0.7;
pub(crate) const GOLEM_RAIN_DAMAGE: i32 = 2;
pub(crate) const GOLEM_RAIN_RADIUS: f32 = 74.0;
pub(crate) const GOLEM_RAIN_SPREAD: f32 = 300.0;
pub(crate) const GOLEM_LASER_CHARGE: f32 = 1.0;
pub(crate) const GOLEM_LASER_SECONDS: f32 = 1.9;
pub(crate) const GOLEM_LASER_TURN: f32 = 0.7;
pub(crate) const GOLEM_LASER_RAGE_TURN: f32 = 1.1;
pub(crate) const GOLEM_LASER_REACH: f32 = 1100.0;
pub(crate) const GOLEM_LASER_WIDTH: f32 = 26.0;
pub(crate) const GOLEM_LASER_DAMAGE: i32 = 2;
pub(crate) const GOLEM_NEAR: f32 = 340.0;

pub(crate) fn speed_from_force(force: f32, size: Vec2) -> f32 {
    force / (DAMPING * size.x * size.y)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ProjectileKind {
    Green,
    Red,
    Purple,
    EyeWatcher,
    GolemRock,
    Boulder,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ProjectileSpec {
    pub(crate) texture: Tex,
    pub(crate) columns: usize,
    pub(crate) rows: usize,
    pub(crate) frame: Vec2,
    pub(crate) scale: f32,
    pub(crate) idle: (usize, f32),
    pub(crate) impact: (usize, f32),
    pub(crate) light: [f32; 5],
    pub(crate) force: f32,
    pub(crate) homing: bool,
    pub(crate) lifetime: Option<f32>,
}

impl ProjectileSpec {
    pub(crate) fn size(&self) -> Vec2 {
        (self.frame * self.scale).floor()
    }

    pub(crate) fn speed(&self) -> f32 {
        let size = self.size();
        self.force / (size.x * size.y) / STEP_RATE
    }

    pub(crate) fn idle_clip(&self) -> Clip {
        Clip {
            row: 0,
            frames: self.idle.0,
            frame_time: self.idle.1,
            looping: true,
            flipped: false,
        }
    }

    pub(crate) fn impact_clip(&self) -> Clip {
        Clip {
            row: 1,
            frames: self.impact.0,
            frame_time: self.impact.1,
            looping: false,
            flipped: false,
        }
    }
}

impl ProjectileKind {
    pub(crate) fn spec(self) -> ProjectileSpec {
        match self {
            Self::Green => slime_bullet(tex::GREEN_SLIMEBULLET, [0.0, 200.0, 0.0]),
            Self::Red => slime_bullet(tex::RED_SLIMEBULLET, [255.0, 215.0, 0.0]),
            Self::Purple => slime_bullet(tex::PURPLE_SLIMEBULLET, [255.0, 0.0, 255.0]),
            Self::EyeWatcher => ProjectileSpec {
                texture: tex::EYE_WATCHER_PROJECTILE,
                columns: 3,
                rows: 2,
                frame: Vec2::new(16.0, 16.0),
                scale: 3.0,
                idle: (3, 0.07),
                impact: (3, 0.085),
                light: [250.0, 240.0, 0.0, 30.0, 1.0],
                force: 35_000_000.0,
                homing: true,
                lifetime: Some(5.0),
            },
            Self::GolemRock => ProjectileSpec {
                texture: tex::GOLEM_ROCK,
                columns: 4,
                rows: 2,
                frame: Vec2::new(6.0, 6.0),
                scale: 6.0,
                idle: (4, 0.08),
                impact: (4, 0.07),
                light: [255.0, 190.0, 80.0, 18.0, 0.4],
                force: 24_000_000.0,
                homing: false,
                lifetime: Some(2.4),
            },
            Self::Boulder => ProjectileSpec {
                texture: tex::GOLEM_BOULDER,
                columns: 4,
                rows: 2,
                frame: Vec2::new(16.0, 16.0),
                scale: 4.0,
                idle: (4, 0.09),
                impact: (4, 0.08),
                light: [255.0, 190.0, 80.0, 28.0, 0.5],
                force: 62_000_000.0,
                homing: false,
                lifetime: Some(3.2),
            },
        }
    }
}

const fn slime_bullet(texture: Tex, color: [f32; 3]) -> ProjectileSpec {
    ProjectileSpec {
        texture,
        columns: 4,
        rows: 2,
        frame: Vec2::new(16.0, 16.0),
        scale: 1.0,
        idle: (4, 0.07),
        impact: (4, 0.07),
        light: [color[0], color[1], color[2], 25.0, 0.75],
        force: 5_000_000.0,
        homing: false,
        lifetime: None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum EnemyKind {
    Rat,
    Snake,
    Bat,
    Ghost,
    Slime,
    RedSlime,
    DemonEye,
    RedSpiderling,
    GreenSpiderling,
    PurpleSpiderling,
    EyeWatcher,
    BloatedSpider,
    StiltSpider,
    Spider,
    KingSlime,
    Golem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Role {
    Common,
    Boss(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Behavior {
    Plain,
    Dash,
    PoisonTrail,
    Smash(ProjectileKind),
    Burst(ProjectileKind),
    Shoots(ProjectileKind),
    HealsKin,
    ToxicBurst,
    Leaps,
    SpiderBoss,
    KingSlimeBoss,
    GolemBoss,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct EnemySpec {
    pub(crate) texture: Tex,
    pub(crate) columns: usize,
    pub(crate) rows: usize,
    pub(crate) frame: Vec2,
    pub(crate) scale: f32,
    pub(crate) idle_frames: usize,
    pub(crate) idle_time: f32,
    pub(crate) death: Option<(usize, f32)>,
    pub(crate) death_row: usize,
    pub(crate) health: i32,
    pub(crate) damage: i32,
    pub(crate) force: f32,
    pub(crate) min_floor: u32,
    pub(crate) max_floor: u32,
    pub(crate) role: Role,
    pub(crate) behavior: Behavior,
    pub(crate) kin: bool,
    pub(crate) themes: &'static [Theme],
}

impl EnemySpec {
    pub(crate) fn size(&self) -> Vec2 {
        (self.frame * self.scale).floor()
    }

    pub(crate) fn speed(&self) -> f32 {
        speed_from_force(self.force, self.size())
    }

    pub(crate) fn idle_clip(&self) -> Clip {
        Clip {
            row: 0,
            frames: self.idle_frames,
            frame_time: self.idle_time,
            looping: true,
            flipped: false,
        }
    }

    pub(crate) fn death_clip(&self) -> Clip {
        match self.death {
            Some((frames, frame_time)) => Clip {
                row: self.death_row,
                frames,
                frame_time,
                looping: false,
                flipped: false,
            },
            None => Clip {
                looping: false,
                ..self.idle_clip()
            },
        }
    }
}

const fn spec(
    texture: Tex,
    grid: (usize, usize),
    frame: (f32, f32),
    scale: f32,
    idle: (usize, f32),
    death: Option<(usize, f32)>,
    stats: (i32, i32, f32),
) -> EnemySpec {
    EnemySpec {
        texture,
        columns: grid.0,
        rows: grid.1,
        frame: Vec2::new(frame.0, frame.1),
        scale,
        idle_frames: idle.0,
        idle_time: idle.1,
        death,
        death_row: 1,
        health: stats.0,
        damage: stats.1,
        force: stats.2,
        min_floor: 0,
        max_floor: u32::MAX,
        role: Role::Common,
        behavior: Behavior::Plain,
        kin: false,
        themes: &[],
    }
}

const fn spiderling(texture: Tex, rows: usize, stats: (i32, i32, f32)) -> EnemySpec {
    EnemySpec {
        min_floor: 1,
        kin: true,
        themes: &[Theme::Spider],
        ..spec(
            texture,
            (5, rows),
            (32.0, 20.0),
            3.0,
            (5, 0.12),
            Some((3, 0.15)),
            stats,
        )
    }
}

impl EnemyKind {
    pub(crate) const ALL: [Self; 16] = [
        Self::Rat,
        Self::Snake,
        Self::Bat,
        Self::Ghost,
        Self::Slime,
        Self::RedSlime,
        Self::DemonEye,
        Self::RedSpiderling,
        Self::GreenSpiderling,
        Self::PurpleSpiderling,
        Self::BloatedSpider,
        Self::StiltSpider,
        Self::EyeWatcher,
        Self::Golem,
        Self::Spider,
        Self::KingSlime,
    ];

    pub(crate) fn spec(self) -> EnemySpec {
        match self {
            Self::Rat => spec(
                tex::RAT,
                (4, 2),
                (85.0, 37.0),
                1.5,
                (4, 0.12),
                Some((3, 0.15)),
                (12, 2, 6_500_000.0),
            ),
            Self::Snake => EnemySpec {
                behavior: Behavior::PoisonTrail,
                ..spec(
                    tex::SNAKE,
                    (9, 2),
                    (64.0, 64.0),
                    1.5,
                    (9, 0.125),
                    Some((5, 0.1)),
                    (8, 3, 8_000_000.0),
                )
            },
            Self::Bat => spec(
                tex::BAT,
                (4, 2),
                (64.0, 64.0),
                1.5,
                (4, 0.125),
                Some((4, 0.1)),
                (9, 2, 14_500_000.0),
            ),
            Self::Ghost => EnemySpec {
                behavior: Behavior::Dash,
                ..spec(
                    tex::GHOST,
                    (5, 2),
                    (22.0, 28.0),
                    4.0,
                    (4, 0.1),
                    Some((5, 0.085)),
                    (7, 2, 6_500_000.0),
                )
            },
            Self::Slime => EnemySpec {
                behavior: Behavior::Smash(ProjectileKind::Green),
                ..spec(
                    tex::SLIME,
                    (6, 2),
                    (32.0, 24.0),
                    3.0,
                    (4, 0.175),
                    Some((6, 0.065)),
                    (9, 1, 1_000_000.0),
                )
            },
            Self::RedSlime => EnemySpec {
                behavior: Behavior::Burst(ProjectileKind::Red),
                ..spec(
                    tex::REDSLIME,
                    (6, 2),
                    (64.0, 48.0),
                    1.5,
                    (4, 0.175),
                    Some((6, 0.065)),
                    (9, 1, 1_000_000.0),
                )
            },
            Self::DemonEye => EnemySpec {
                min_floor: 3,
                ..spec(
                    tex::DEMONEYE,
                    (5, 2),
                    (76.0, 64.0),
                    1.5,
                    (4, 0.1),
                    Some((5, 0.1)),
                    (11, 3, 14_500_000.0),
                )
            },
            Self::RedSpiderling => spiderling(tex::SPIDERLING, 2, (3, 2, 5_500_000.0)),
            Self::GreenSpiderling => EnemySpec {
                behavior: Behavior::HealsKin,
                ..spiderling(tex::GREEN_SPIDERLING, 2, (5, 2, 5_500_000.0))
            },
            Self::PurpleSpiderling => spiderling(tex::PURPLE_SPIDERLING, 3, (3, 1, 6_500_000.0)),
            Self::BloatedSpider => EnemySpec {
                behavior: Behavior::ToxicBurst,
                kin: false,
                ..spiderling(tex::BLOATED_SPIDER, 2, (5, 1, 4_800_000.0))
            },
            Self::StiltSpider => EnemySpec {
                behavior: Behavior::Leaps,
                death_row: 1,
                kin: false,
                min_floor: 1,
                themes: &[Theme::Spider],
                ..spec(
                    tex::STILT_SPIDER,
                    (5, 5),
                    (40.0, 32.0),
                    3.0,
                    (5, 0.14),
                    Some((3, 0.18)),
                    (7, 1, 3_600_000.0),
                )
            },
            Self::Spider => EnemySpec {
                min_floor: 1,
                max_floor: 1,
                death_row: 2,
                role: Role::Boss("Spider king"),
                behavior: Behavior::SpiderBoss,
                ..spec(
                    tex::SPIDER,
                    (5, 3),
                    (64.0, 40.0),
                    3.0,
                    (5, 0.12),
                    Some((3, 0.15)),
                    (75, 2, 32_500_000.0),
                )
            },
            Self::KingSlime => EnemySpec {
                min_floor: 2,
                role: Role::Boss("King slime"),
                behavior: Behavior::KingSlimeBoss,
                ..spec(
                    tex::KINGSLIME,
                    (6, 2),
                    (64.0, 48.0),
                    4.0,
                    (4, 0.175),
                    Some((6, 0.065)),
                    (90, 2, 10_000_000.0),
                )
            },
            Self::Golem => EnemySpec {
                min_floor: 2,
                death_row: 7,
                themes: &[Theme::Stone, Theme::Temple, Theme::Lava],
                role: Role::Boss("Stone golem"),
                behavior: Behavior::GolemBoss,
                ..spec(
                    tex::GOLEM,
                    (14, 8),
                    (100.0, 100.0),
                    3.0,
                    (4, 0.175),
                    Some((14, 0.09)),
                    (110, 2, 17_000_000.0),
                )
            },
            Self::EyeWatcher => EnemySpec {
                min_floor: 3,
                death_row: 2,
                behavior: Behavior::Shoots(ProjectileKind::EyeWatcher),
                ..spec(
                    tex::EYE_WATCHER,
                    (5, 3),
                    (32.0, 32.0),
                    3.0,
                    (4, 0.1),
                    Some((5, 0.085)),
                    (4, 1, 0.0),
                )
            },
        }
    }

    pub(crate) fn available_on(self, floor: u32) -> bool {
        let spec = self.spec();
        floor >= spec.min_floor && floor <= spec.max_floor
    }

    pub(crate) fn available_in(self, floor: u32, theme: Theme) -> bool {
        self.available_on(floor) && {
            let themes = self.spec().themes;
            themes.is_empty() || themes.contains(&theme)
        }
    }

    pub(crate) fn is_boss(self) -> bool {
        matches!(self.spec().role, Role::Boss(_))
    }
}

pub(crate) const LEAP_FIRST_COOLDOWN: f32 = 1.4;
pub(crate) const LEAP_COOLDOWN: f32 = 2.4;
pub(crate) const LEAP_MIN_DISTANCE: f32 = 150.0;
pub(crate) const LEAP_MAX_DISTANCE: f32 = 420.0;
pub(crate) const LEAP_CROUCH_SECONDS: f32 = 0.5;
pub(crate) const LEAP_AIR_SECONDS: f32 = 0.5;
pub(crate) const LEAP_REST_SECONDS: f32 = 0.8;
pub(crate) const LEAP_LANDING_RADIUS: f32 = 112.0;

pub(crate) fn crouch_clip() -> Clip {
    Clip {
        row: 2,
        frames: 2,
        frame_time: LEAP_CROUCH_SECONDS / 2.0,
        looping: false,
        flipped: false,
    }
}

pub(crate) fn air_clip() -> Clip {
    Clip {
        row: 3,
        frames: 1,
        frame_time: LEAP_AIR_SECONDS,
        looping: true,
        flipped: false,
    }
}

pub(crate) fn landing_clip() -> Clip {
    Clip {
        row: 4,
        frames: 2,
        frame_time: LEAP_REST_SECONDS / 2.0,
        looping: false,
        flipped: false,
    }
}

pub(crate) fn spell_clip() -> Clip {
    Clip {
        row: 1,
        frames: 3,
        frame_time: 0.15,
        looping: false,
        flipped: false,
    }
}

pub(crate) fn attack_clip() -> Clip {
    Clip {
        row: 1,
        frames: 5,
        frame_time: 0.075,
        looping: false,
        flipped: false,
    }
}

pub(crate) fn pick(floor: u32, theme: Theme, rng: &mut Rng) -> EnemyKind {
    pick_where(floor, theme, rng, |kind| !kind.is_boss())
}

const BOSS_VAR: &str = "BANE_BOSS";

fn boss_named(name: &str) -> Option<EnemyKind> {
    match name.trim().to_ascii_lowercase().as_str() {
        "golem" => Some(EnemyKind::Golem),
        "spider" => Some(EnemyKind::Spider),
        "slime" | "kingslime" => Some(EnemyKind::KingSlime),
        _ => None,
    }
}

pub(crate) fn pick_boss(floor: u32, theme: Theme, rng: &mut Rng) -> EnemyKind {
    if let Some(kind) = std::env::var(BOSS_VAR)
        .ok()
        .and_then(|name| boss_named(&name))
    {
        return kind;
    }
    pick_where(floor, theme, rng, EnemyKind::is_boss)
}

fn pick_where(
    floor: u32,
    theme: Theme,
    rng: &mut Rng,
    wanted: impl Fn(EnemyKind) -> bool,
) -> EnemyKind {
    let pool: Vec<EnemyKind> = EnemyKind::ALL
        .into_iter()
        .filter(|kind| kind.available_in(floor, theme) && wanted(*kind))
        .collect();
    pool[rng.below(pool.len())]
}

pub(crate) fn health_multiplier(floor: u32) -> f32 {
    1.0 + HEALTH_GROWTH_PER_FLOOR * floor as f32
}

pub(crate) fn damage_multiplier(floor: u32) -> f32 {
    1.0 + DAMAGE_GROWTH_PER_FLOOR * floor as f32
}

pub(crate) fn scale_stat(base: i32, multiplier: f32) -> i32 {
    ((base as f32 * multiplier).round() as i32).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_zero_is_neutral_and_later_floors_grow() {
        assert_eq!(scale_stat(12, health_multiplier(0)), 12);
        assert_eq!(scale_stat(12, health_multiplier(5)), 19);
        assert_eq!(scale_stat(2, damage_multiplier(5)), 3);
        assert_eq!(scale_stat(1, 0.1), 1);
    }

    #[test]
    fn tougher_enemies_appear_only_on_later_floors() {
        assert!(EnemyKind::Rat.available_on(0));
        assert!(EnemyKind::RedSpiderling.available_on(1));
        assert!(EnemyKind::RedSpiderling.available_in(1, Theme::Spider));
        assert!(!EnemyKind::RedSpiderling.available_in(1, Theme::Stone));
        assert!(!EnemyKind::BloatedSpider.available_in(3, Theme::Lava));
        assert!(EnemyKind::Rat.available_in(1, Theme::Lava));
        assert!(!EnemyKind::DemonEye.available_on(2));
        assert!(EnemyKind::DemonEye.available_on(3));
        assert!(!EnemyKind::EyeWatcher.available_on(2));
        assert!(EnemyKind::Slime.available_on(0));
    }

    #[test]
    fn picking_respects_the_floor() {
        let mut rng = Rng::new(7);
        for _ in 0..200 {
            let kind = pick(0, Theme::Stone, &mut rng);
            assert!(kind.available_in(0, Theme::Stone), "{kind:?}");
        }
        let mut spider_floor = std::collections::HashSet::new();
        let mut stone_floor = std::collections::HashSet::new();
        for _ in 0..1500 {
            spider_floor.insert(pick(5, Theme::Spider, &mut rng));
            stone_floor.insert(pick(5, Theme::Stone, &mut rng));
        }
        assert_eq!(spider_floor.len(), EnemyKind::ALL.len() - 3);
        assert_eq!(stone_floor.len(), EnemyKind::ALL.len() - 3 - 5);
        assert!(!stone_floor.contains(&EnemyKind::BloatedSpider));
        assert!(!stone_floor.contains(&EnemyKind::StiltSpider));
        assert!(spider_floor.contains(&EnemyKind::StiltSpider));
    }

    #[test]
    fn speed_follows_force_over_body_area() {
        assert!((EnemyKind::Rat.spec().speed() - 186.0).abs() < 1.0);
        assert!((EnemyKind::Bat.spec().speed() - 314.0).abs() < 2.0);
        assert!((EnemyKind::Slime.spec().speed() - 29.0).abs() < 1.0);
        assert_eq!(EnemyKind::EyeWatcher.spec().speed(), 0.0);
    }

    #[test]
    fn projectile_speeds_follow_the_single_step_impulse() {
        let slime = ProjectileKind::Green.spec().speed();
        let eye = ProjectileKind::EyeWatcher.spec().speed();
        assert!((slime - 325.0).abs() < 2.0, "{slime}");
        assert!((eye - 253.0).abs() < 2.0, "{eye}");
    }

    #[test]
    fn every_enemy_has_a_death_animation_that_fits_inside_its_sheet() {
        for kind in EnemyKind::ALL {
            let spec = kind.spec();
            let (frames, _) = spec
                .death
                .unwrap_or_else(|| panic!("{kind:?} has no death"));
            assert!(spec.death_row < spec.rows, "{kind:?}");
            assert!(frames <= spec.columns, "{kind:?}");
            let clip = spec.death_clip();
            assert!(!clip.looping, "{kind:?}");
            assert_ne!(clip.row, 0, "{kind:?} would play its idle row");
        }
    }

    #[test]
    fn the_first_floor_has_the_spider_and_later_floors_the_king_slime_or_the_golem() {
        let mut rng = Rng::new(3);
        for _ in 0..20 {
            assert_eq!(pick_boss(1, Theme::Spider, &mut rng), EnemyKind::Spider);
            assert_eq!(pick_boss(2, Theme::Ice, &mut rng), EnemyKind::KingSlime);
            assert_eq!(pick_boss(5, Theme::Sewer, &mut rng), EnemyKind::KingSlime);
        }
    }

    #[test]
    fn stone_temple_and_lava_floors_can_have_the_golem_and_others_never() {
        let mut rng = Rng::new(5);
        for theme in [Theme::Stone, Theme::Temple, Theme::Lava] {
            let seen: std::collections::HashSet<EnemyKind> =
                (0..200).map(|_| pick_boss(3, theme, &mut rng)).collect();
            assert!(seen.contains(&EnemyKind::Golem), "{theme:?}");
            assert!(seen.contains(&EnemyKind::KingSlime), "{theme:?}");
        }
        for theme in [Theme::Ice, Theme::Sewer, Theme::Moss] {
            assert!((0..100).all(|_| pick_boss(3, theme, &mut rng) == EnemyKind::KingSlime));
        }
    }

    #[test]
    fn boss_names_for_previews_are_parsed() {
        assert_eq!(boss_named(" Golem "), Some(EnemyKind::Golem));
        assert_eq!(boss_named("slime"), Some(EnemyKind::KingSlime));
        assert_eq!(boss_named("nothing"), None);
    }

    #[test]
    fn bosses_never_appear_among_common_enemies() {
        let mut rng = Rng::new(11);
        for _ in 0..500 {
            assert!(!pick(1, Theme::Spider, &mut rng).is_boss());
        }
    }

    #[test]
    fn the_bloated_spider_bursts_into_a_cloud_when_it_dies() {
        assert_eq!(
            EnemyKind::BloatedSpider.spec().behavior,
            Behavior::ToxicBurst
        );
        assert_eq!(EnemyKind::BloatedSpider.spec().themes, &[Theme::Spider]);
    }

    #[test]
    fn the_spider_dies_on_its_third_row() {
        assert_eq!(EnemyKind::Spider.spec().death_clip().row, 2);
        assert!((EnemyKind::Spider.spec().speed() - 282.0).abs() < 1.0);
        assert!((EnemyKind::KingSlime.spec().speed() - 40.6).abs() < 0.5);
    }

    #[test]
    fn only_spiderlings_count_as_kin() {
        let kin: Vec<EnemyKind> = EnemyKind::ALL
            .into_iter()
            .filter(|kind| kind.spec().kin)
            .collect();
        assert_eq!(
            kin,
            vec![
                EnemyKind::RedSpiderling,
                EnemyKind::GreenSpiderling,
                EnemyKind::PurpleSpiderling
            ]
        );
    }
}
