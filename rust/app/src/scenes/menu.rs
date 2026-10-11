use glam::{Vec2, Vec4};
use hecs::World;

use crate::components::UiAction;
use crate::input::Input;
use crate::prefabs;
use crate::systems;
use crate::viewport::TARGET;

use super::{Scene, SceneId, Transition};

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

    fn enter(&mut self) {
        let x = TARGET.x / 2.0;
        prefabs::block(
            &mut self.world,
            Vec2::new(x, 100.0),
            Vec2::new(320.0, 64.0),
            Vec4::new(0.85, 0.65, 0.15, 1.0),
        );
        prefabs::button(
            &mut self.world,
            Vec2::new(x, 240.0),
            Vec4::new(0.2, 0.55, 0.3, 1.0),
            UiAction::Play,
        );
        prefabs::button(
            &mut self.world,
            Vec2::new(x, 310.0),
            Vec4::new(0.25, 0.35, 0.55, 1.0),
            UiAction::Settings,
        );
        prefabs::button(
            &mut self.world,
            Vec2::new(x, 380.0),
            Vec4::new(0.6, 0.2, 0.2, 1.0),
            UiAction::Exit,
        );
    }

    fn update(&mut self, input: &Input, _dt: f32) -> Transition {
        match systems::update_buttons(&mut self.world, input) {
            Some(UiAction::Play) => Transition::Switch(SceneId::Lobby),
            Some(UiAction::Settings) => {
                tracing::info!("settings are not implemented yet");
                Transition::None
            }
            Some(UiAction::Exit) => Transition::Quit,
            None => Transition::None,
        }
    }
}
