use std::sync::Arc;

use glam::Vec2;
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::Textures;
use crate::input::Input;
use crate::physics::Physics;
use crate::prefabs;
use crate::systems;
use crate::tilemap::{self, Tileset, TILE_SIZE};

use super::{Scene, SceneId, Transition};

pub(crate) struct InGameScene {
    world: World,
    physics: Physics,
    textures: Textures,
    tileset: Arc<Tileset>,
    bounds: Vec2,
    camera: Vec2,
}

impl InGameScene {
    pub(crate) fn new(textures: Textures, tileset: Arc<Tileset>) -> Self {
        Self {
            world: World::new(),
            physics: Physics::new(),
            textures,
            tileset,
            bounds: Vec2::ZERO,
            camera: Vec2::ZERO,
        }
    }
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

    fn enter(&mut self) {
        let grid = tilemap::arena();
        self.bounds = grid.pixel_size();
        let solid_cells =
            prefabs::tilemap(&mut self.world, &grid, &self.tileset, self.textures.tileset);
        self.physics.add_walls(&solid_cells, TILE_SIZE);
        prefabs::player(&mut self.world, &mut self.physics, self.bounds / 2.0);
        self.camera = systems::follow_player(&self.world, self.bounds);
    }

    fn update(&mut self, input: &Input, dt: f32) -> Transition {
        systems::shoot(
            &mut self.world,
            &mut self.physics,
            input,
            self.camera,
            self.textures.bullet,
            dt,
            systems::ShotRoll::random(),
        );
        systems::animate(&mut self.world, dt);
        systems::finish_impacts(&mut self.world);
        self.camera = systems::follow_player(&self.world, self.bounds);
        if input.just_pressed(KeyCode::Escape) {
            Transition::Switch(SceneId::Lobby)
        } else {
            Transition::None
        }
    }

    fn fixed_update(&mut self, input: &Input, dt: f32) -> Transition {
        systems::steer_player(&mut self.world, input);
        systems::simulate(&mut self.world, &mut self.physics, dt);
        systems::bullet_hits(&mut self.world, &mut self.physics);
        Transition::None
    }
}
