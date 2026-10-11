mod ingame;
mod loading;
mod lobby;
mod machine;
mod menu;
mod pause;
mod settings_panel;

use glam::Vec2;
use hecs::World;

use crate::assets::Assets;
use crate::audio::{AudioCommand, Soundtrack};
use crate::input::Input;
use crate::postfx::SceneFx;
use crate::settings::Settings;

pub(crate) use machine::Scenes;
pub(crate) use pause::Pause;
pub(crate) use settings_panel::SettingsPanel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SceneId {
    Loading,
    Menu,
    Lobby,
    InGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Cursor {
    Default,
    Crosshair,
    Reload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Transition {
    None,
    Switch(SceneId),
    Quit,
}

pub(crate) trait Scene {
    fn world(&self) -> &World;

    fn world_mut(&mut self) -> &mut World;

    fn enter(&mut self, assets: &Assets);

    fn soundtrack(&self) -> Option<Soundtrack> {
        None
    }

    fn take_sounds(&mut self) -> Vec<AudioCommand> {
        Vec::new()
    }

    fn fx(&self) -> SceneFx {
        SceneFx::default()
    }

    fn take_hit(&mut self) -> bool {
        false
    }

    fn cursor(&self) -> Cursor {
        Cursor::Default
    }

    fn camera(&self) -> Vec2 {
        Vec2::ZERO
    }

    fn leave(&mut self) {
        self.world_mut().clear();
    }

    fn update(
        &mut self,
        _input: &Input,
        _dt: f32,
        _assets: &Assets,
        _settings: &mut Settings,
    ) -> Transition {
        Transition::None
    }

    fn fixed_update(&mut self, _input: &Input, _dt: f32, _assets: &Assets) -> Transition {
        Transition::None
    }
}
