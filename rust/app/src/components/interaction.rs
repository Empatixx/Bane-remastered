use glam::Vec4;

use crate::render::FontSize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Interaction {
    EnterPortal,
    Upgrade,
    NextFloor,
    PickUpGun,
    PickUpArtefact,
    Buy,
    Spin,
    Quests,
    Weapons,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Interactable {
    pub(crate) action: Interaction,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Prompt {
    pub(crate) text: &'static str,
    pub(crate) gap: f32,
    pub(crate) color: Vec4,
    pub(crate) size: FontSize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct InRange;
