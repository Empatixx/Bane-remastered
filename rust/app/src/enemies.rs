use glam::Vec2;

use crate::components::Clip;
use crate::dungeon::Rng;

pub(crate) const SPEED_PER_FORCE: f32 = 184.0 / 6_500_000.0;
const HEALTH_GROWTH_PER_FLOOR: f32 = 0.12;
const DAMAGE_GROWTH_PER_FLOOR: f32 = 0.08;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum EnemyKind {
    Rat,
    Snake,
    Bat,
    Ghost,
    DemonEye,
    RedSpiderling,
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
    pub(crate) health: i32,
    pub(crate) damage: i32,
    pub(crate) force: f32,
    pub(crate) min_floor: u32,
    pub(crate) dashes: bool,
}

impl EnemySpec {
    pub(crate) fn size(&self) -> Vec2 {
        (self.frame * self.scale).floor()
    }

    pub(crate) fn speed(&self) -> f32 {
        self.force * SPEED_PER_FORCE
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
                row: 1,
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
        health: stats.0,
        damage: stats.1,
        force: stats.2,
        min_floor: 0,
        dashes: false,
    }
}

impl EnemyKind {
    pub(crate) const ALL: [Self; 6] = [
        Self::Rat,
        Self::Snake,
        Self::Bat,
        Self::Ghost,
        Self::DemonEye,
        Self::RedSpiderling,
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
                dashes: true,
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
            Self::RedSpiderling => EnemySpec {
                min_floor: 2,
                ..spec(
                    "ingame/spiderling.tga",
                    (5, 2),
                    (32.0, 20.0),
                    3.0,
                    (5, 0.12),
                    Some((3, 0.15)),
                    (3, 2, 5_500_000.0),
                )
            },
        }
    }

    pub(crate) fn available_on(self, floor: u32) -> bool {
        floor >= self.spec().min_floor
    }
}

pub(crate) fn pick(floor: u32, rng: &mut Rng) -> EnemyKind {
    let pool: Vec<EnemyKind> = EnemyKind::ALL
        .into_iter()
        .filter(|kind| kind.available_on(floor))
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
    }

    #[test]
    fn picking_respects_the_floor() {
        let mut rng = Rng::new(7);
        for _ in 0..200 {
            let kind = pick(0, &mut rng);
            assert!(kind.available_on(0), "{kind:?}");
        }
        let mut seen = std::collections::HashSet::new();
        for _ in 0..400 {
            seen.insert(pick(5, &mut rng));
        }
        assert_eq!(seen.len(), EnemyKind::ALL.len());
    }

    #[test]
    fn speeds_follow_bane_move_forces() {
        assert!((EnemyKind::Rat.spec().speed() - 184.0).abs() < 0.01);
        assert!((EnemyKind::Bat.spec().speed() - 410.5).abs() < 0.5);
    }

    #[test]
    fn enemies_without_a_death_animation_play_their_idle_once() {
        let clip = EnemyKind::DemonEye.spec().death_clip();
        assert!(!clip.looping);
        assert_eq!(clip.row, 0);
        assert_eq!(EnemyKind::Rat.spec().death_clip().row, 1);
    }
}
