use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Assets, BUTTON, LOGO};
use crate::audio::{AudioCommand, SoundEffect, Soundtrack};
use crate::components::{Color, Layer, Position, ScreenSpace, Size, UiAction};
use crate::input::Input;
use crate::prefabs;
use crate::settings::Settings;
use crate::systems;
use crate::viewport::TARGET;

use super::menu_stage::Stage;
use super::{Scene, SceneId, SettingsPanel, Transition};

const LOGO_SIZE: Vec2 = Vec2::new(228.0, 108.0);
const LOGO_TOP_LEFT_Y: f32 = 50.0;
const SHADE_WIDTH: f32 = 0.4;
const SHADE_COLOR: Vec4 = Vec4::new(0.03, 0.02, 0.05, 0.45);
const BUTTON_ROWS: [(f32, &str, UiAction); 3] = [
    (225.0, "Play", UiAction::Play),
    (325.0, "Settings", UiAction::Settings),
    (425.0, "Exit", UiAction::Exit),
];

#[derive(Default)]
pub(crate) struct MenuScene {
    world: World,
    panel: SettingsPanel,
    stage: Option<Stage>,
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
        let shade = self.world.spawn((
            Position(TARGET / 2.0),
            Size(Vec2::new(TARGET.x * SHADE_WIDTH, TARGET.y)),
            Color(SHADE_COLOR),
            Layer(-1),
            ScreenSpace,
        ));
        let logo = prefabs::sprite(
            &mut self.world,
            Vec2::new(x, LOGO_TOP_LEFT_Y + LOGO_SIZE.y / 2.0),
            LOGO_SIZE,
            assets.textures.get(LOGO),
            0,
        );
        let _ = self.world.insert_one(logo, ScreenSpace);
        self.ui.extend([shade, logo]);
        for (y, label, action) in BUTTON_ROWS {
            let (button, text) = prefabs::button(
                &mut self.world,
                Vec2::new(x, y),
                label,
                action,
                assets.textures.get(BUTTON),
            );
            let _ = self.world.insert_one(button, ScreenSpace);
            let _ = self.world.insert_one(text, ScreenSpace);
            self.ui.extend([button, text]);
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
        }
        if self.panel.is_open() {
            if self.panel.update(&mut self.world, input, settings) {
                self.sounds.push(AudioCommand::Effect(SoundEffect::Click));
            }
            return Transition::None;
        }
        let action = systems::update_buttons(&mut self.world, input);
        if action.is_some() {
            self.sounds.push(AudioCommand::Effect(SoundEffect::Click));
        }
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
