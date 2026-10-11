use glam::{Vec2, Vec4};
use hecs::World;
use winit::keyboard::KeyCode;

use crate::input::Input;
use crate::prefabs;
use crate::systems;

use super::{Scene, SceneId, Transition};

#[derive(Default)]
pub(crate) struct InGameScene {
    world: World,
}

impl Scene for InGameScene {
    fn world(&self) -> &World {
        &self.world
    }

    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn enter(&mut self) {
        prefabs::block(
            &mut self.world,
            Vec2::new(200.0, 200.0),
            Vec2::new(64.0, 64.0),
            Vec4::new(0.9, 0.3, 0.2, 1.0),
        );
        prefabs::block(
            &mut self.world,
            Vec2::new(400.0, 300.0),
            Vec2::new(120.0, 40.0),
            Vec4::new(0.2, 0.7, 0.4, 1.0),
        );
        prefabs::block(
            &mut self.world,
            Vec2::new(600.0, 150.0),
            Vec2::new(80.0, 80.0),
            Vec4::new(0.3, 0.5, 0.9, 1.0),
        );
        prefabs::player(&mut self.world, Vec2::new(100.0, 100.0));
    }

    fn update(&mut self, input: &Input, _dt: f32) -> Transition {
        if input.just_pressed(KeyCode::Escape) {
            Transition::Switch(SceneId::Lobby)
        } else {
            Transition::None
        }
    }

    fn fixed_update(&mut self, input: &Input, dt: f32) -> Transition {
        systems::steer_player(&mut self.world, input);
        systems::apply_velocity(&mut self.world, dt);
        Transition::None
    }
}
