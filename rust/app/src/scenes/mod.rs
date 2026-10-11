mod ingame;
mod loading;
mod lobby;
mod machine;
mod menu;

use glam::Vec2;
use hecs::World;

use crate::assets::Assets;
use crate::input::Input;

pub(crate) use machine::Scenes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SceneId {
    Loading,
    Menu,
    Lobby,
    InGame,
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

    fn camera(&self) -> Vec2 {
        Vec2::ZERO
    }

    fn leave(&mut self) {
        self.world_mut().clear();
    }

    fn update(&mut self, _input: &Input, _dt: f32, _assets: &Assets) -> Transition {
        Transition::None
    }

    fn fixed_update(&mut self, _input: &Input, _dt: f32) -> Transition {
        Transition::None
    }
}
