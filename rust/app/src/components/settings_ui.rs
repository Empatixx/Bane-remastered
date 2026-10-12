use crate::controls::Action;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SettingsTabKind {
    Video,
    Audio,
    Controls,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Hidden;

#[derive(Debug, Clone, Copy)]
pub(crate) struct SettingsUi;

#[derive(Debug, Clone, Copy)]
pub(crate) struct TabOnly(pub(crate) SettingsTabKind);

#[derive(Debug, Clone, Copy)]
pub(crate) struct TabButton(pub(crate) SettingsTabKind);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Toggle {
    Vsync,
    Fullscreen,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Checkbox {
    pub(crate) toggle: Toggle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Volume {
    Overall,
    Music,
    Effects,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SliderHandle {
    pub(crate) volume: Volume,
    pub(crate) left: f32,
    pub(crate) track_width: f32,
    pub(crate) dragging: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SliderValue(pub(crate) Volume);

#[derive(Debug, Clone, Copy)]
pub(crate) struct ResolutionText;

#[derive(Debug, Clone, Copy)]
pub(crate) struct KeyBindSlot {
    pub(crate) action: Action,
    pub(crate) listening: bool,
}
