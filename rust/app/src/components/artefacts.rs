use crate::artefacts::ArtefactKind;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ArtefactSlot {
    pub(crate) current: Option<ArtefactKind>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ArtefactDrop {
    pub(crate) kind: ArtefactKind,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ArtefactIcon;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Shielded;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ShieldAura;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Berserk {
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TimeSlow {
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Bloodlust {
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Phantom {
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct InfiniteAmmo {
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct MidasTouch {
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SummonGhosts;

#[derive(Debug, Clone, Copy)]
pub(crate) struct CastLightning;

#[derive(Debug, Clone, Copy)]
pub(crate) struct PlaceTurret;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ConvertBullets;

#[derive(Debug, Clone, Copy)]
pub(crate) struct HealPercent(pub(crate) f32);
