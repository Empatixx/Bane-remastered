#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UiAction {
    Play,
    Settings,
    Exit,
    Resume,
    Statistics,
    LeaveToMenu,
    SettingsApply,
    SettingsRevert,
    SettingsExit,
    ResolutionNext,
    ResolutionPrevious,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Button {
    pub(crate) idle_uv: [f32; 4],
    pub(crate) hover_uv: [f32; 4],
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OnClick(pub(crate) UiAction);

#[derive(Debug, Clone, Copy)]
pub(crate) struct TextButton {
    pub(crate) idle: glam::Vec4,
    pub(crate) hover: glam::Vec4,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Hitbox {
    pub(crate) offset: glam::Vec2,
    pub(crate) size: glam::Vec2,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PauseUi;
