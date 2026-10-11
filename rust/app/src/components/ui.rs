use glam::Vec4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UiAction {
    Play,
    Settings,
    Exit,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Button {
    pub(crate) idle: Vec4,
    pub(crate) hover: Vec4,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OnClick(pub(crate) UiAction);
