use glam::{Vec2, Vec4};
use hecs::World;
use winit::keyboard::KeyCode;

use crate::input::Input;
use crate::prefabs;
use crate::systems;
use crate::viewport::TARGET;

use super::{Scene, SceneId, Transition};

#[derive(Default)]
pub(crate) struct LobbyScene {
    world: World,
}

impl Scene for LobbyScene {
    fn world(&self) -> &World {
        &self.world
    }

    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn enter(&mut self) {
        prefabs::block(
            &mut self.world,
            Vec2::new(TARGET.x / 2.0, TARGET.y - 20.0),
            Vec2::new(TARGET.x, 40.0),
            Vec4::new(0.2, 0.22, 0.28, 1.0),
        );
        prefabs::portal(&mut self.world, Vec2::new(TARGET.x - 100.0, 230.0));
        prefabs::player(&mut self.world, Vec2::new(150.0, 230.0));
    }

    fn update(&mut self, input: &Input, _dt: f32) -> Transition {
        if input.just_pressed(KeyCode::Escape) {
            Transition::Switch(SceneId::Menu)
        } else {
            Transition::None
        }
    }

    fn fixed_update(&mut self, input: &Input, dt: f32) -> Transition {
        systems::steer_player(&mut self.world, input);
        systems::apply_velocity(&mut self.world, dt);
        if systems::player_in_portal(&self.world) {
            Transition::Switch(SceneId::InGame)
        } else {
            Transition::None
        }
    }
}
