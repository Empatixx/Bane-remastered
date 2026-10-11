#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChestKind {
    Weapon,
    Artefact,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Chest {
    pub(crate) kind: ChestKind,
    pub(crate) opened: bool,
}
