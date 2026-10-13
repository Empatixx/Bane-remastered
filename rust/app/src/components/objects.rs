use rapier2d::prelude::ColliderHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BreakKind {
    Pot,
    Barrel,
    Explosive,
    Poison,
    Frost,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Breakable {
    pub(crate) kind: BreakKind,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Solid(pub(crate) ColliderHandle);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Broken;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Flame;

#[derive(Debug, Clone, Copy)]
pub(crate) struct SlotMachine {
    pub(crate) spinning: Option<f32>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SlotPrice {
    pub(crate) owner: hecs::Entity,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PoisonPool {
    pub(crate) remaining: f32,
    pub(crate) tick: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PoisonTrail;

#[derive(Debug, Clone, Copy)]
pub(crate) struct TrailLayer {
    pub(crate) spent: f32,
    pub(crate) last: glam::Vec2,
    pub(crate) count: u32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Poisoned;

const FROST_FADE_IN: f32 = 0.2;
const FROST_FADE_OUT: f32 = 0.6;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Frozen {
    pub(crate) remaining: f32,
    pub(crate) total: f32,
}

impl Frozen {
    pub(crate) fn new(seconds: f32) -> Self {
        Self {
            remaining: seconds,
            total: seconds,
        }
    }

    pub(crate) fn strength(&self) -> f32 {
        let fade_in = ((self.total - self.remaining) / FROST_FADE_IN).clamp(0.0, 1.0);
        let fade_out = (self.remaining / FROST_FADE_OUT).clamp(0.0, 1.0);
        fade_in.min(fade_out)
    }
}
