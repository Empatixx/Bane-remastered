use glam::Vec2;
use hecs::World;

use crate::assets::Assets;
use crate::input::Input;
use crate::prefabs;
use crate::viewport::TARGET;

use super::{Scene, SceneId, Transition};

const SPINNER_CENTER: Vec2 = Vec2::new(TARGET.x / 2.0, 380.0);

#[derive(Default)]
pub(crate) struct LoadingScene {
    world: World,
}

impl Scene for LoadingScene {
    fn world(&self) -> &World {
        &self.world
    }

    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn enter(&mut self, assets: &Assets) {
        prefabs::sprite(
            &mut self.world,
            TARGET / 2.0,
            TARGET,
            assets.textures.loading_background,
            -1,
        );
        prefabs::spinner(
            &mut self.world,
            SPINNER_CENTER,
            assets.textures.loading_spinner,
        );
    }

    fn update(&mut self, _input: &Input, _dt: f32, assets: &Assets) -> Transition {
        if assets.is_loaded() {
            Transition::Switch(SceneId::Menu)
        } else {
            Transition::None
        }
    }
}
