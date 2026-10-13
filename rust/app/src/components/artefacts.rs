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
