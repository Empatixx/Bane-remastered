use rapier2d::prelude::ColliderHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BreakKind {
    Pot,
    Barrel,
    Explosive,
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
