use glam::Vec2;
use hecs::World;

use crate::assets::{Assets, BUTTON, LOGO, MENU_BACKGROUND};
use crate::components::UiAction;
use crate::input::Input;
use crate::prefabs;
use crate::systems;
use crate::viewport::TARGET;

use super::{Scene, SceneId, Transition};

const LOGO_SIZE: Vec2 = Vec2::new(228.0, 108.0);
const LOGO_TOP_LEFT_Y: f32 = 50.0;
const BUTTON_ROWS: [(f32, &str, UiAction); 3] = [
    (225.0, "Play", UiAction::Play),
    (325.0, "Settings", UiAction::Settings),
    (425.0, "Exit", UiAction::Exit),
];

#[derive(Default)]
pub(crate) struct MenuScene {
    world: World,
}

impl Scene for MenuScene {
    fn world(&self) -> &World {
        &self.world
    }

    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn enter(&mut self, assets: &Assets) {
        let x = TARGET.x / 2.0;
        prefabs::sprite(
            &mut self.world,
            TARGET / 2.0,
            TARGET,
            assets.textures.get(MENU_BACKGROUND),
            -1,
        );
        prefabs::sprite(
            &mut self.world,
            Vec2::new(x, LOGO_TOP_LEFT_Y + LOGO_SIZE.y / 2.0),
            LOGO_SIZE,
            assets.textures.get(LOGO),
            0,
        );
        for (y, label, action) in BUTTON_ROWS {
            prefabs::button(
                &mut self.world,
                Vec2::new(x, y),
                label,
                action,
                assets.textures.get(BUTTON),
            );
        }
    }

    fn update(&mut self, input: &Input, _dt: f32, _assets: &Assets) -> Transition {
        match systems::update_buttons(&mut self.world, input) {
            Some(UiAction::Play) => Transition::Switch(SceneId::Lobby),
            Some(UiAction::Settings) => {
                tracing::info!("settings are not implemented yet");
                Transition::None
            }
            Some(UiAction::Exit) => Transition::Quit,
            _ => Transition::None,
        }
    }
}
