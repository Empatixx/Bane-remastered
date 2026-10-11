#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UiAction {
    Play,
    Settings,
    Exit,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Button {
    pub(crate) idle_uv: [f32; 4],
    pub(crate) hover_uv: [f32; 4],
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OnClick(pub(crate) UiAction);
