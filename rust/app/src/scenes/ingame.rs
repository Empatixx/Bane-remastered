use glam::Vec2;
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::Assets;
use crate::input::Input;
use crate::physics::Physics;
use crate::prefabs;
use crate::systems;
use crate::tilemap::{self, TILE_SIZE};

use super::{Scene, SceneId, Transition};

#[derive(Default)]
pub(crate) struct InGameScene {
    world: World,
    physics: Physics,
    bounds: Vec2,
    camera: Vec2,
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
        self.camera
    }

    fn enter(&mut self, assets: &Assets) {
        let grid = tilemap::arena();
        self.bounds = grid.pixel_size();
        let solid_cells = prefabs::tilemap(
            &mut self.world,
            &grid,
            &assets.tileset,
            assets.textures.tileset,
        );
        self.physics.add_walls(&solid_cells, TILE_SIZE);
        prefabs::player(&mut self.world, &mut self.physics, self.bounds / 2.0);
        for corner in [(2.5, 2.5), (17.5, 2.5), (2.5, 9.5), (17.5, 9.5)] {
            let position = Vec2::new(corner.0, corner.1) * TILE_SIZE;
            prefabs::rat(
                &mut self.world,
                &mut self.physics,
                position,
                assets.textures.rat,
            );
        }
        self.camera = systems::follow_player(&self.world, self.bounds);
    }

    fn update(&mut self, input: &Input, dt: f32, assets: &Assets) -> Transition {
        systems::shoot(
            &mut self.world,
            &mut self.physics,
            input,
            self.camera,
            assets.textures.bullet,
            dt,
            systems::ShotRoll::random(),
        );
        systems::remove_expired(&mut self.world);
        self.camera = systems::follow_player(&self.world, self.bounds);
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
