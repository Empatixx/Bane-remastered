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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PatchKind {
    Ice,
    Web,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FloorPatch(pub(crate) PatchKind);

#[derive(Debug, Clone, Copy)]
pub(crate) struct ArrowTrap {
    pub(crate) timer: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TrapArrow {
    pub(crate) origin_y: f32,
    pub(crate) travelled: f32,
    pub(crate) free_after: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct RippleTimer(pub(crate) f32);

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

/// A pool or cloud left by the player's own rounds, it never hurts the player.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Friendly;

pub(crate) const CHILL_SECONDS: f32 = 2.5;
pub(crate) const CHILL_SLOWDOWN: f32 = 0.5;
pub(crate) const CHILL_STACKS_TO_FREEZE: u32 = 4;
const CHILL_LOOK: f32 = 0.45;
const CHILL_FADE_OUT: f32 = 0.4;

/// Slowed by frost rounds; enough hits in a row freeze the enemy solid.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Chilled {
    pub(crate) remaining: f32,
    pub(crate) stacks: u32,
}

impl Chilled {
    pub(crate) fn strength(&self) -> f32 {
        CHILL_LOOK * (self.remaining / CHILL_FADE_OUT).clamp(0.0, 1.0)
    }
}
