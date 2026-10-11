use glam::Vec2;

use crate::components::Clip;
use crate::dungeon::Rng;

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

pub(crate) fn speed_from_force(force: f32, size: Vec2) -> f32 {
    force / (DAMPING * size.x * size.y)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ProjectileKind {
    Green,
    Red,
    Purple,
    EyeWatcher,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ProjectileSpec {
    pub(crate) texture: &'static str,
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
            Self::Green => slime_bullet("ingame/green_slimebullet.tga", [0.0, 200.0, 0.0]),
            Self::Red => slime_bullet("ingame/red_slimebullet.tga", [255.0, 215.0, 0.0]),
            Self::Purple => slime_bullet("ingame/purple_slimebullet.tga", [255.0, 0.0, 255.0]),
            Self::EyeWatcher => ProjectileSpec {
                texture: "ingame/eye_watcher_projectile.tga",
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
        }
    }
}

const fn slime_bullet(texture: &'static str, color: [f32; 3]) -> ProjectileSpec {
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
    Spider,
    KingSlime,
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
    Smash(ProjectileKind),
    Burst(ProjectileKind),
    Shoots(ProjectileKind),
    HealsKin,
    SpiderBoss,
    KingSlimeBoss,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct EnemySpec {
    pub(crate) texture: &'static str,
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
    texture: &'static str,
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
    }
}

const fn spiderling(texture: &'static str, rows: usize, stats: (i32, i32, f32)) -> EnemySpec {
    EnemySpec {
        min_floor: 2,
        kin: true,
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
    pub(crate) const ALL: [Self; 13] = [
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
        Self::EyeWatcher,
        Self::Spider,
        Self::KingSlime,
    ];

    pub(crate) fn spec(self) -> EnemySpec {
        match self {
            Self::Rat => spec(
                "ingame/rat.tga",
                (4, 2),
                (85.0, 37.0),
                1.5,
                (4, 0.12),
                Some((3, 0.15)),
                (12, 2, 6_500_000.0),
            ),
            Self::Snake => spec(
                "ingame/snake.tga",
                (9, 2),
                (64.0, 64.0),
                1.5,
                (9, 0.125),
                Some((5, 0.1)),
                (8, 3, 8_000_000.0),
            ),
            Self::Bat => spec(
                "ingame/bat.tga",
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
                    "ingame/ghost.tga",
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
                    "ingame/slime.tga",
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
                    "ingame/redslime.tga",
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
                    "ingame/demoneye.tga",
                    (4, 1),
                    (76.0, 64.0),
                    1.5,
                    (4, 0.1),
                    None,
                    (11, 3, 14_500_000.0),
                )
            },
            Self::RedSpiderling => spiderling("ingame/spiderling.tga", 2, (3, 2, 5_500_000.0)),
            Self::GreenSpiderling => EnemySpec {
                behavior: Behavior::HealsKin,
                ..spiderling("ingame/green_spiderling.tga", 2, (5, 2, 5_500_000.0))
            },
            Self::PurpleSpiderling => {
                spiderling("ingame/purple_spiderling.tga", 3, (3, 1, 6_500_000.0))
            }
            Self::Spider => EnemySpec {
                min_floor: 1,
                max_floor: 1,
                death_row: 2,
                role: Role::Boss("Spider king"),
                behavior: Behavior::SpiderBoss,
                ..spec(
                    "ingame/spider.tga",
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
                    "ingame/kingslime.tga",
                    (6, 2),
                    (64.0, 48.0),
                    4.0,
                    (4, 0.175),
                    Some((6, 0.065)),
                    (90, 2, 10_000_000.0),
                )
            },
            Self::EyeWatcher => EnemySpec {
                min_floor: 3,
                behavior: Behavior::Shoots(ProjectileKind::EyeWatcher),
                ..spec(
                    "ingame/eye_watcher.tga",
                    (5, 2),
                    (32.0, 32.0),
                    3.0,
                    (4, 0.1),
                    None,
                    (4, 1, 0.0),
                )
            },
        }
    }

    pub(crate) fn available_on(self, floor: u32) -> bool {
        let spec = self.spec();
        floor >= spec.min_floor && floor <= spec.max_floor
    }

    pub(crate) fn is_boss(self) -> bool {
        matches!(self.spec().role, Role::Boss(_))
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

pub(crate) fn pick(floor: u32, rng: &mut Rng) -> EnemyKind {
    pick_where(floor, rng, |kind| !kind.is_boss())
}

pub(crate) fn pick_boss(floor: u32, rng: &mut Rng) -> EnemyKind {
    pick_where(floor, rng, EnemyKind::is_boss)
}

fn pick_where(floor: u32, rng: &mut Rng, wanted: impl Fn(EnemyKind) -> bool) -> EnemyKind {
    let pool: Vec<EnemyKind> = EnemyKind::ALL
        .into_iter()
        .filter(|kind| kind.available_on(floor) && wanted(*kind))
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
        assert!(!EnemyKind::RedSpiderling.available_on(1));
        assert!(EnemyKind::RedSpiderling.available_on(2));
        assert!(!EnemyKind::DemonEye.available_on(2));
        assert!(EnemyKind::DemonEye.available_on(3));
        assert!(!EnemyKind::EyeWatcher.available_on(2));
        assert!(EnemyKind::Slime.available_on(0));
    }

    #[test]
    fn picking_respects_the_floor() {
        let mut rng = Rng::new(7);
        for _ in 0..200 {
            let kind = pick(0, &mut rng);
            assert!(kind.available_on(0), "{kind:?}");
        }
        let mut seen = std::collections::HashSet::new();
        for _ in 0..800 {
            seen.insert(pick(5, &mut rng));
        }
        assert_eq!(seen.len(), EnemyKind::ALL.len() - 2);
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
    fn enemies_without_a_death_animation_play_their_idle_once() {
        let clip = EnemyKind::DemonEye.spec().death_clip();
        assert!(!clip.looping);
        assert_eq!(clip.row, 0);
        assert_eq!(EnemyKind::Rat.spec().death_clip().row, 1);
    }

    #[test]
    fn the_first_floor_has_the_spider_and_later_floors_the_king_slime() {
        let mut rng = Rng::new(3);
        for _ in 0..20 {
            assert_eq!(pick_boss(1, &mut rng), EnemyKind::Spider);
            assert_eq!(pick_boss(2, &mut rng), EnemyKind::KingSlime);
            assert_eq!(pick_boss(5, &mut rng), EnemyKind::KingSlime);
        }
    }

    #[test]
    fn bosses_never_appear_among_common_enemies() {
        let mut rng = Rng::new(11);
        for _ in 0..500 {
            assert!(!pick(1, &mut rng).is_boss());
        }
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
