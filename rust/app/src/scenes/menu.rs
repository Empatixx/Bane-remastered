use glam::Vec2;
use hecs::{Entity, World};

use crate::assets::{Assets, LOGO, MENU_PLATE};
use crate::audio::{AudioCommand, SoundEffect, Soundtrack};
use crate::components::UiAction;
use crate::input::Input;
use crate::settings::Settings;
use crate::viewport::TARGET;

use super::loading::{TITLE_CENTER, TITLE_SIZE};
use super::menu_buttons::MenuButtons;
use super::menu_logo::LogoAnim;
use super::menu_stage::Stage;
use super::{Scene, SceneId, SettingsPanel, Transition};

const LOGO_SIZE: Vec2 = Vec2::new(228.0, 108.0);
const LOGO_TOP_LEFT_Y: f32 = 50.0;
const BUTTON_ROWS: [(f32, &str, UiAction); 3] = [
    (240.0, "Play", UiAction::Play),
    (320.0, "Settings", UiAction::Settings),
    (400.0, "Exit", UiAction::Exit),
];

#[derive(Default)]
pub(crate) struct MenuScene {
    world: World,
    panel: SettingsPanel,
    stage: Option<Stage>,
    logo: Option<LogoAnim>,
    buttons: Option<MenuButtons>,
    ui: Vec<Entity>,
    sounds: Vec<AudioCommand>,
}

impl Scene for MenuScene {
    fn world(&self) -> &World {
        &self.world
    }

    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn soundtrack(&self) -> Option<Soundtrack> {
        Some(Soundtrack::MainMenu)
    }

    fn take_sounds(&mut self) -> Vec<AudioCommand> {
        std::mem::take(&mut self.sounds)
    }

    fn camera(&self) -> Vec2 {
        self.stage.as_ref().map_or(Vec2::ZERO, Stage::camera)
    }

    fn enter(&mut self, assets: &Assets) {
        let x = TARGET.x / 2.0;
        self.panel = SettingsPanel::default();
        self.ui.clear();
        self.stage = Some(Stage::build(&mut self.world, assets));
        let logo = LogoAnim::gliding(
            &mut self.world,
            assets.textures.get(LOGO),
            Vec2::new(x, LOGO_TOP_LEFT_Y + LOGO_SIZE.y / 2.0),
            LOGO_SIZE,
            TITLE_CENTER,
            TITLE_SIZE,
        );
        let logo_entity = logo.entity();
        self.logo = Some(logo);
        self.ui.push(logo_entity);
        let buttons = MenuButtons::build(
            &mut self.world,
            &assets.textures,
            MENU_PLATE,
            x,
            &BUTTON_ROWS,
        );
        self.ui.extend(buttons.entities());
        self.buttons = Some(buttons);
        if let Some(stage) = &mut self.stage {
            stage.tick(&mut self.world, assets, 0.0);
            if let Some(logo) = &mut self.logo {
                logo.tick(&mut self.world, stage.camera(), 0.0);
            }
        }
    }

    fn update(
        &mut self,
        input: &Input,
        dt: f32,
        assets: &Assets,
        settings: &mut Settings,
    ) -> Transition {
        if let Some(stage) = &mut self.stage {
            stage.tick(&mut self.world, assets, dt);
            if let Some(logo) = &mut self.logo {
                logo.tick(&mut self.world, stage.camera(), dt);
            }
        }
        if self.panel.is_open() {
            if self.panel.update(&mut self.world, input, settings) {
                self.sounds.push(AudioCommand::Effect(SoundEffect::Click));
            }
            return Transition::None;
        }
        let action = self
            .buttons
            .as_mut()
            .and_then(|buttons| buttons.update(&mut self.world, input, dt, &mut self.sounds));
        match action {
            Some(UiAction::Play) => Transition::Switch(SceneId::Lobby),
            Some(UiAction::Settings) => {
                self.panel.open(&mut self.world, assets, &self.ui);
                Transition::None
            }
            Some(UiAction::Exit) => Transition::Quit,
            _ => Transition::None,
        }
    }
}
