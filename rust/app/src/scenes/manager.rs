use std::fmt;
use std::ops::ControlFlow;

use crate::input::Input;
use crate::render::Quad;
use crate::systems;

use super::ingame::InGameScene;
use super::loading::LoadingScene;
use super::lobby::LobbyScene;
use super::menu::MenuScene;
use super::{Scene, SceneId, Transition};

pub(crate) struct SceneManager {
    current: SceneId,
    loading: LoadingScene,
    menu: MenuScene,
    lobby: LobbyScene,
    ingame: InGameScene,
}

impl fmt::Debug for SceneManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SceneManager")
            .field("current", &self.current)
            .finish()
    }
}

impl SceneManager {
    pub(crate) fn new() -> Self {
        let mut manager = Self {
            current: SceneId::Loading,
            loading: LoadingScene::default(),
            menu: MenuScene::default(),
            lobby: LobbyScene::default(),
            ingame: InGameScene::default(),
        };
        manager.scene_mut().enter();
        manager
    }

    pub(crate) fn update(&mut self, input: &Input, dt: f32) -> ControlFlow<()> {
        let transition = self.scene_mut().update(input, dt);
        self.apply(transition)
    }

    pub(crate) fn fixed_update(&mut self, input: &Input, dt: f32) -> ControlFlow<()> {
        let transition = self.scene_mut().fixed_update(input, dt);
        self.apply(transition)
    }

    pub(crate) fn quads(&self) -> Vec<Quad> {
        systems::collect_quads(self.scene().world())
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

    fn click_at(manager: &mut SceneManager, input: &mut Input, position: Vec2) -> ControlFlow<()> {
        input.set_mouse(position);
        assert!(manager.update(input, 0.0).is_continue());
        input.handle_click(MouseButton::Left, ElementState::Pressed);
        let flow = manager.update(input, 0.0);
        input.end_frame();
        flow
    }

    fn menu() -> (SceneManager, Input) {
        let mut manager = SceneManager::new();
        let input = Input::default();
        assert!(manager.update(&input, 2.0).is_continue());
        assert_eq!(manager.current, SceneId::Menu);
        (manager, input)
    }

    #[test]
    fn loading_switches_to_menu_when_done() {
        let mut manager = SceneManager::new();
        let input = Input::default();
        assert!(manager.update(&input, 0.5).is_continue());
        assert_eq!(manager.current, SceneId::Loading);
        assert!(manager.update(&input, 0.6).is_continue());
        assert_eq!(manager.current, SceneId::Menu);
    }

    #[test]
    fn play_button_opens_lobby() {
        let (mut manager, mut input) = menu();
        let flow = click_at(&mut manager, &mut input, Vec2::new(480.0, 240.0));
        assert_eq!(flow, ControlFlow::Continue(()));
        assert_eq!(manager.current, SceneId::Lobby);
    }

    #[test]
    fn exit_button_quits() {
        let (mut manager, mut input) = menu();
        let flow = click_at(&mut manager, &mut input, Vec2::new(480.0, 380.0));
        assert_eq!(flow, ControlFlow::Break(()));
    }

    #[test]
    fn click_outside_buttons_does_nothing() {
        let (mut manager, mut input) = menu();
        assert!(click_at(&mut manager, &mut input, Vec2::new(10.0, 10.0)).is_continue());
        assert_eq!(manager.current, SceneId::Menu);
    }
}
