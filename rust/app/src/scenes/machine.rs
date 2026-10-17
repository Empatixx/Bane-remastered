use std::fmt;
use std::ops::ControlFlow;

use crate::assets::Assets;
use crate::audio::AudioCommand;
use crate::input::Input;
use crate::profile::SharedProfile;
use crate::render::{Fonts, Layers, LightSource, Occluders};
use crate::settings::Settings;
use crate::systems;

use super::cinematic::CinematicScene;
use super::ingame::InGameScene;
use super::loading::LoadingScene;
use super::lobby::LobbyScene;
use super::menu::MenuScene;
use super::{Cursor, Scene, SceneId, Transition};

pub(crate) struct Scenes {
    current: SceneId,
    profile: SharedProfile,
    pending: Vec<AudioCommand>,
    loading: LoadingScene,
    menu: MenuScene,
    lobby: LobbyScene,
    ingame: InGameScene,
    intro: CinematicScene,
}

impl fmt::Debug for Scenes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Scenes")
            .field("current", &self.current)
            .finish()
    }
}

impl Scenes {
    pub(crate) fn new(assets: &Assets, profile: SharedProfile) -> Self {
        let mut scenes = Self {
            current: SceneId::Loading,
            profile: profile.clone(),
            pending: Vec::new(),
            loading: LoadingScene::default(),
            menu: MenuScene::default(),
            lobby: LobbyScene::default(),
            ingame: InGameScene::default(),
            intro: CinematicScene::default(),
        };
        scenes.lobby.attach_profile(profile.clone());
        scenes.ingame.attach_profile(profile);
        scenes.scene_mut().enter(assets);
        scenes.queue_music();
        scenes
    }

    pub(crate) fn take_sounds(&mut self) -> Vec<AudioCommand> {
        let mut sounds = std::mem::take(&mut self.pending);
        sounds.extend(self.scene_mut().take_sounds());
        sounds
    }

    fn queue_music(&mut self) {
        if let Some(track) = self.scene().soundtrack() {
            self.pending.push(AudioCommand::Music(track));
        }
    }

    pub(crate) fn update(
        &mut self,
        input: &Input,
        dt: f32,
        assets: &Assets,
        settings: &mut Settings,
    ) -> ControlFlow<()> {
        systems::animate(self.scene_mut().world_mut(), dt);
        systems::chain_animations(self.scene_mut().world_mut());
        systems::tick_openings(self.scene_mut().world_mut(), dt);
        let transition = self.scene_mut().update(input, dt, assets, settings);
        self.apply(transition, assets)
    }

    pub(crate) fn set_blend(&mut self, blend: f32) {
        self.scene_mut().set_blend(blend);
    }

    pub(crate) fn fixed_update(
        &mut self,
        input: &Input,
        dt: f32,
        assets: &Assets,
    ) -> ControlFlow<()> {
        let transition = self.scene_mut().fixed_update(input, dt, assets);
        self.apply(transition, assets)
    }

    pub(crate) fn fx(&self) -> crate::postfx::SceneFx {
        self.scene().fx()
    }

    pub(crate) fn take_hit(&mut self) -> bool {
        self.scene_mut().take_hit()
    }

    pub(crate) fn camera(&self) -> glam::Vec2 {
        self.scene().camera()
    }

    pub(crate) fn lights(&self) -> Vec<LightSource> {
        let scene = self.scene();
        systems::collect_lights(scene.world(), scene.camera(), scene.blend())
    }

    pub(crate) fn occluders(&self) -> Option<std::sync::Arc<Occluders>> {
        self.scene().occluders()
    }

    pub(crate) fn cursor(&self) -> Cursor {
        self.scene().cursor()
    }

    pub(crate) fn draws(&self, fonts: &Fonts, interact_key: &str) -> Layers {
        let scene = self.scene();
        systems::collect_draws(
            scene.world(),
            fonts,
            scene.camera(),
            interact_key,
            scene.blend(),
        )
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
        // The way into the lobby from the menu goes through the training crypt until it is done.
        // The first time, Play opens the cinematic, which leads on into the training crypt.
        let next = if next == SceneId::Lobby && self.current == SceneId::Menu && !self.profile.training_done() {
            SceneId::Intro
        } else {
            next
        };
        self.scene_mut().leave();
        self.ingame.set_training(next == SceneId::Training);
        self.current = next;
        self.scene_mut().enter(assets);
        self.queue_music();
        tracing::info!(scene = ?next, "scene changed");
    }

    fn scene(&self) -> &dyn Scene {
        match self.current {
            SceneId::Loading => &self.loading,
            SceneId::Menu => &self.menu,
            SceneId::Lobby => &self.lobby,
            SceneId::InGame | SceneId::Training => &self.ingame,
            SceneId::Intro => &self.intro,
        }
    }

    fn scene_mut(&mut self) -> &mut dyn Scene {
        match self.current {
            SceneId::Loading => &mut self.loading,
            SceneId::Menu => &mut self.menu,
            SceneId::Lobby => &mut self.lobby,
            SceneId::InGame | SceneId::Training => &mut self.ingame,
            SceneId::Intro => &mut self.intro,
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
        assert!(scenes
            .update(input, 2.0, assets, &mut Settings::in_memory())
            .is_continue());
        input.set_mouse(position);
        assert!(scenes
            .update(input, 0.0, assets, &mut Settings::in_memory())
            .is_continue());
        input.handle_click(MouseButton::Left, ElementState::Pressed);
        let pressed = scenes.update(input, 0.0, assets, &mut Settings::in_memory());
        input.end_frame();
        if pressed.is_break() {
            return pressed;
        }
        scenes.update(input, 0.5, assets, &mut Settings::in_memory())
    }

    fn menu() -> (Scenes, Input, Assets) {
        let assets = loaded_assets();
        let mut scenes = Scenes::new(&assets, SharedProfile::in_memory());
        let input = Input::default();
        for _ in 0..40 {
            assert!(scenes
                .update(&input, 0.1, &assets, &mut Settings::in_memory())
                .is_continue());
        }
        assert_eq!(scenes.current, SceneId::Menu);
        (scenes, input, assets)
    }

    #[test]
    fn loading_waits_until_the_assets_are_loaded() {
        let mut assets = Assets::new().expect("the embedded tileset is valid");
        let mut scenes = Scenes::new(&assets, SharedProfile::in_memory());
        let input = Input::default();

        assert!(scenes
            .update(&input, 1.0, &assets, &mut Settings::in_memory())
            .is_continue());
        assert_eq!(scenes.current, SceneId::Loading);

        assets.mark_loaded();
        for _ in 0..40 {
            assert!(scenes
                .update(&input, 0.1, &assets, &mut Settings::in_memory())
                .is_continue());
        }
        assert_eq!(scenes.current, SceneId::Menu);
    }

    #[test]
    fn play_button_opens_the_training_the_first_time_and_the_lobby_after() {
        let (mut scenes, mut input, assets) = menu();
        let flow = click_at(&mut scenes, &mut input, &assets, Vec2::new(480.0, 240.0));
        assert_eq!(flow, ControlFlow::Continue(()));
        assert_eq!(scenes.current, SceneId::Intro);

        let (mut scenes, mut input, assets) = menu();
        scenes.profile.finish_training();
        click_at(&mut scenes, &mut input, &assets, Vec2::new(480.0, 240.0));
        assert_eq!(scenes.current, SceneId::Lobby);
    }

    #[test]
    fn exit_button_quits() {
        let (mut scenes, mut input, assets) = menu();
        let flow = click_at(&mut scenes, &mut input, &assets, Vec2::new(480.0, 400.0));
        assert_eq!(flow, ControlFlow::Break(()));
    }

    #[test]
    fn click_outside_buttons_does_nothing() {
        let (mut scenes, mut input, assets) = menu();
        assert!(click_at(&mut scenes, &mut input, &assets, Vec2::new(10.0, 10.0)).is_continue());
        assert_eq!(scenes.current, SceneId::Menu);
    }
}
