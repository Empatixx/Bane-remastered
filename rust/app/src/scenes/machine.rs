use std::fmt;
use std::ops::ControlFlow;

use crate::assets::Assets;
use crate::input::Input;
use crate::render::{Draw, Fonts};
use crate::settings::Settings;
use crate::systems;

use super::ingame::InGameScene;
use super::loading::LoadingScene;
use super::lobby::LobbyScene;
use super::menu::MenuScene;
use super::{Cursor, Scene, SceneId, Transition};

pub(crate) struct Scenes {
    current: SceneId,
    loading: LoadingScene,
    menu: MenuScene,
    lobby: LobbyScene,
    ingame: InGameScene,
}

impl fmt::Debug for Scenes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Scenes")
            .field("current", &self.current)
            .finish()
    }
}

impl Scenes {
    pub(crate) fn new(assets: &Assets) -> Self {
        let mut scenes = Self {
            current: SceneId::Loading,
            loading: LoadingScene::default(),
            menu: MenuScene::default(),
            lobby: LobbyScene::default(),
            ingame: InGameScene::default(),
        };
        scenes.scene_mut().enter(assets);
        scenes
    }

    pub(crate) fn update(
        &mut self,
        input: &Input,
        dt: f32,
        assets: &Assets,
        settings: &mut Settings,
    ) -> ControlFlow<()> {
        systems::animate(self.scene_mut().world_mut(), dt);
        let transition = self.scene_mut().update(input, dt, assets, settings);
        self.apply(transition, assets)
    }

    pub(crate) fn fixed_update(
        &mut self,
        input: &Input,
        dt: f32,
        assets: &Assets,
    ) -> ControlFlow<()> {
        let transition = self.scene_mut().fixed_update(input, dt);
        self.apply(transition, assets)
    }

    pub(crate) fn cursor(&self) -> Cursor {
        self.scene().cursor()
    }

    pub(crate) fn draws(&self, fonts: &Fonts) -> Vec<Draw> {
        let scene = self.scene();
        systems::collect_draws(scene.world(), fonts, scene.camera())
    }

    fn apply(&mut self, transition: Transition, assets: &Assets) -> ControlFlow<()> {
        match transition {
            Transition::None => {}
            Transition::Switch(next) => self.switch(next, assets),
            Transition::Quit => return ControlFlow::Break(()),
        }
        ControlFlow::Continue(())
    }

    fn switch(&mut self, next: SceneId, assets: &Assets) {
        self.scene_mut().leave();
        self.current = next;
        self.scene_mut().enter(assets);
        tracing::info!(scene = ?next, "scene changed");
    }

    fn scene(&self) -> &dyn Scene {
        match self.current {
            SceneId::Loading => &self.loading,
            SceneId::Menu => &self.menu,
            SceneId::Lobby => &self.lobby,
            SceneId::InGame => &self.ingame,
        }
    }

    fn scene_mut(&mut self) -> &mut dyn Scene {
        match self.current {
            SceneId::Loading => &mut self.loading,
            SceneId::Menu => &mut self.menu,
            SceneId::Lobby => &mut self.lobby,
            SceneId::InGame => &mut self.ingame,
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;
    use winit::event::{ElementState, MouseButton};

    use super::*;

    fn loaded_assets() -> Assets {
        let mut assets = Assets::new().expect("the embedded tileset is valid");
        assets.mark_loaded();
        assets
    }

    fn click_at(
        scenes: &mut Scenes,
        input: &mut Input,
        assets: &Assets,
        position: Vec2,
    ) -> ControlFlow<()> {
        input.set_mouse(position);
        assert!(scenes
            .update(input, 0.0, assets, &mut Settings::in_memory())
            .is_continue());
        input.handle_click(MouseButton::Left, ElementState::Pressed);
        let flow = scenes.update(input, 0.0, assets, &mut Settings::in_memory());
        input.end_frame();
        flow
    }

    fn menu() -> (Scenes, Input, Assets) {
        let assets = loaded_assets();
        let mut scenes = Scenes::new(&assets);
        let input = Input::default();
        assert!(scenes
            .update(&input, 0.0, &assets, &mut Settings::in_memory())
            .is_continue());
        assert_eq!(scenes.current, SceneId::Menu);
        (scenes, input, assets)
    }

    #[test]
    fn loading_waits_until_the_assets_are_loaded() {
        let mut assets = Assets::new().expect("the embedded tileset is valid");
        let mut scenes = Scenes::new(&assets);
        let input = Input::default();

        assert!(scenes
            .update(&input, 1.0, &assets, &mut Settings::in_memory())
            .is_continue());
        assert_eq!(scenes.current, SceneId::Loading);

        assets.mark_loaded();
        assert!(scenes
            .update(&input, 0.0, &assets, &mut Settings::in_memory())
            .is_continue());
        assert_eq!(scenes.current, SceneId::Menu);
    }

    #[test]
    fn play_button_opens_lobby() {
        let (mut scenes, mut input, assets) = menu();
        let flow = click_at(&mut scenes, &mut input, &assets, Vec2::new(480.0, 225.0));
        assert_eq!(flow, ControlFlow::Continue(()));
        assert_eq!(scenes.current, SceneId::Lobby);
    }

    #[test]
    fn exit_button_quits() {
        let (mut scenes, mut input, assets) = menu();
        let flow = click_at(&mut scenes, &mut input, &assets, Vec2::new(480.0, 425.0));
        assert_eq!(flow, ControlFlow::Break(()));
    }

    #[test]
    fn click_outside_buttons_does_nothing() {
        let (mut scenes, mut input, assets) = menu();
        assert!(click_at(&mut scenes, &mut input, &assets, Vec2::new(10.0, 10.0)).is_continue());
        assert_eq!(scenes.current, SceneId::Menu);
    }
}
