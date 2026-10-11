use glam::Vec2;
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::{Assets, BULLET, RAT, TILESET};
use crate::camera::Camera;
use crate::input::Input;
use crate::physics::Physics;
use crate::prefabs;
use crate::systems;
use crate::tilemap::{self, TILE_SIZE};
use crate::viewport::TARGET;

use super::{Scene, SceneId, Transition};

#[derive(Default)]
pub(crate) struct InGameScene {
    world: World,
    physics: Physics,
    camera: Camera,
}

impl Scene for InGameScene {
    fn world(&self) -> &World {
        &self.world
    }

    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn leave(&mut self) {
        self.world.clear();
        self.physics = Physics::new();
    }

    fn camera(&self) -> Vec2 {
        self.camera.position()
    }

    fn enter(&mut self, assets: &Assets) {
        let grid = tilemap::arena();
        let bounds = grid.pixel_size();
        let solid_cells = prefabs::tilemap(
            &mut self.world,
            &grid,
            &assets.tileset,
            assets.textures.get(TILESET),
        );
        self.physics.add_walls(&solid_cells, TILE_SIZE);
        prefabs::player(
            &mut self.world,
            &mut self.physics,
            bounds / 2.0,
            &assets.textures,
        );
        for corner in [(2.5, 2.5), (17.5, 2.5), (2.5, 9.5), (17.5, 9.5)] {
            let position = Vec2::new(corner.0, corner.1) * TILE_SIZE;
            prefabs::rat(
                &mut self.world,
                &mut self.physics,
                position,
                assets.textures.get(RAT),
            );
        }
        self.camera = Camera::new(bounds);
        if let Some(target) = systems::camera_target(&self.world, TARGET / 2.0) {
            self.camera.snap_to(target);
        }
    }

    fn update(&mut self, input: &Input, dt: f32, assets: &Assets) -> Transition {
        systems::shoot(
            &mut self.world,
            &mut self.physics,
            input,
            self.camera.position(),
            assets.textures.get(BULLET),
            dt,
            systems::ShotRoll::random(),
        );
        systems::animate_player(&mut self.world, input);
        systems::remove_expired(&mut self.world);
        if let Some(target) = systems::camera_target(&self.world, input.mouse()) {
            self.camera.follow(target, dt);
        }
        if input.just_pressed(KeyCode::Escape) {
            Transition::Switch(SceneId::Lobby)
        } else {
            Transition::None
        }
    }

    fn fixed_update(&mut self, input: &Input, dt: f32) -> Transition {
        systems::steer_player(&mut self.world, input);
        systems::chase_player(&mut self.world);
        systems::simulate(&mut self.world, &mut self.physics, dt);
        systems::bullet_hits(&mut self.world, &mut self.physics);
        systems::reap_dead(&mut self.world, &mut self.physics);
        Transition::None
    }
}
