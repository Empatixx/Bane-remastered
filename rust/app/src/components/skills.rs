use glam::Vec2;

use crate::components::Clip;
use crate::enemies::ProjectileKind;

#[derive(Debug, Clone, Copy)]
pub(crate) struct EnemyBullet;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Impact(pub(crate) Clip);

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Hits(pub(crate) u32);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Smash {
    pub(crate) kind: ProjectileKind,
    pub(crate) count: usize,
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Boss;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Minion;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Rooted;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Hatcher {
    pub(crate) remaining: f32,
    pub(crate) next: usize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct WebSpitter {
    pub(crate) remaining: f32,
    pub(crate) casting: bool,
    pub(crate) idle: Clip,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Cobweb;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Drag {
    pub(crate) rate: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Armageddon {
    pub(crate) kind: ProjectileKind,
    pub(crate) remaining: f32,
    pub(crate) angle: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DeathCloud;

#[derive(Debug, Clone, Copy)]
pub(crate) struct DeathBurst {
    pub(crate) kind: ProjectileKind,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct EyeShooter {
    pub(crate) kind: ProjectileKind,
    pub(crate) idle: Clip,
    pub(crate) remaining: f32,
    pub(crate) attacking: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Homing {
    pub(crate) aim: Vec2,
    pub(crate) speed: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Fuse {
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DeathHeal {
    pub(crate) radius: f32,
    pub(crate) amount: i32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Kin;
