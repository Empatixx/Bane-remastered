use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Ghost {
    pub(crate) life: f32,
    pub(crate) cooldown: f32,
    pub(crate) phase: f32,
    pub(crate) home: Vec2,
    pub(crate) velocity: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TurretStage {
    Rising,
    Active,
    Crumbling,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Turret {
    pub(crate) stage: TurretStage,
    pub(crate) stage_time: f32,
    pub(crate) life: f32,
    pub(crate) cooldown: f32,
    pub(crate) recoil: f32,
    pub(crate) aim: usize,
    pub(crate) flipped: bool,
}
