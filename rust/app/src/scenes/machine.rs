use std::fmt;
use std::ops::ControlFlow;

use crate::assets::Textures;
use crate::input::Input;
use crate::render::{Draw, Font};
use crate::systems;

use super::ingame::InGameScene;
use super::loading::LoadingScene;
use super::lobby::LobbyScene;
use super::menu::MenuScene;
use super::{Scene, SceneId, Transition};

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
    pub(crate) fn new(textures: Textures) -> Self {
        let mut scenes = Self {
            current: SceneId::Loading,
            loading: LoadingScene::default(),
            menu: MenuScene::new(textures),
            lobby: LobbyScene::new(textures),
            ingame: InGameScene::new(textures),
        };
        scenes.scene_mut().enter();
        scenes
    }

    pub(crate) fn update(&mut self, input: &Input, dt: f32) -> ControlFlow<()> {
        let transition = self.scene_mut().update(input, dt);
        self.apply(transition)
    }

    pub(crate) fn fixed_update(&mut self, input: &Input, dt: f32) -> ControlFlow<()> {
        let transition = self.scene_mut().fixed_update(input, dt);
        self.apply(transition)
    }

    pub(crate) fn draws(&self, font: &Font) -> Vec<Draw> {
        let scene = self.scene();
        systems::collect_draws(scene.world(), font, scene.camera())
    }

    fn apply(&mut self, transition: Transition) -> ControlFlow<()> {
        match transition {
            Transition::None => {}
            Transition::Switch(next) => self.switch(next),
            Transition::Quit => return ControlFlow::Break(()),
        }
        ControlFlow::Continue(())
    }

    fn switch(&mut self, next: SceneId) {
        self.scene_mut().leave();
        self.current = next;
        self.scene_mut().enter();
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

    fn click_at(scenes: &mut Scenes, input: &mut Input, position: Vec2) -> ControlFlow<()> {
        input.set_mouse(position);
        assert!(scenes.update(input, 0.0).is_continue());
        input.handle_click(MouseButton::Left, ElementState::Pressed);
        let flow = scenes.update(input, 0.0);
        input.end_frame();
        flow
    }

    fn menu() -> (Scenes, Input) {
        let mut scenes = Scenes::new(Textures::default());
        let input = Input::default();
        assert!(scenes.update(&input, 2.0).is_continue());
        assert_eq!(scenes.current, SceneId::Menu);
        (scenes, input)
    }

    #[test]
    fn loading_switches_to_menu_when_done() {
        let mut scenes = Scenes::new(Textures::default());
        let input = Input::default();
        assert!(scenes.update(&input, 0.5).is_continue());
        assert_eq!(scenes.current, SceneId::Loading);
        assert!(scenes.update(&input, 0.6).is_continue());
        assert_eq!(scenes.current, SceneId::Menu);
    }

    #[test]
    fn play_button_opens_lobby() {
        let (mut scenes, mut input) = menu();
        let flow = click_at(&mut scenes, &mut input, Vec2::new(480.0, 240.0));
        assert_eq!(flow, ControlFlow::Continue(()));
        assert_eq!(scenes.current, SceneId::Lobby);
    }

    #[test]
    fn exit_button_quits() {
        let (mut scenes, mut input) = menu();
        let flow = click_at(&mut scenes, &mut input, Vec2::new(480.0, 380.0));
        assert_eq!(flow, ControlFlow::Break(()));
    }

    #[test]
    fn click_outside_buttons_does_nothing() {
        let (mut scenes, mut input) = menu();
        assert!(click_at(&mut scenes, &mut input, Vec2::new(10.0, 10.0)).is_continue());
        assert_eq!(scenes.current, SceneId::Menu);
    }
}
