use std::sync::Arc;

use glam::Vec2;
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::Textures;
use crate::input::Input;
use crate::physics::Physics;
use crate::prefabs;
use crate::systems;
use crate::tilemap::{self, TileGrid, Tileset, TILE_SIZE};

use super::{Scene, SceneId, Transition};

pub(crate) struct LobbyScene {
    world: World,
    physics: Physics,
    textures: Textures,
    tileset: Arc<Tileset>,
    bounds: Vec2,
    camera: Vec2,
}

impl LobbyScene {
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

impl Scene for LobbyScene {
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
        let grid: TileGrid = tilemap::lobby();
        self.bounds = grid.pixel_size();
        let solid_cells =
            prefabs::tilemap(&mut self.world, &grid, &self.tileset, self.textures.tileset);
        self.physics.add_walls(&solid_cells, TILE_SIZE);
        prefabs::portal(
            &mut self.world,
            Vec2::new(self.bounds.x - 2.5 * TILE_SIZE, self.bounds.y / 2.0),
        );
        prefabs::player(
            &mut self.world,
            &mut self.physics,
            Vec2::new(3.5 * TILE_SIZE, self.bounds.y / 2.0),
        );
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
            Transition::Switch(SceneId::Menu)
        } else {
            Transition::None
        }
    }

    fn fixed_update(&mut self, input: &Input, dt: f32) -> Transition {
        systems::steer_player(&mut self.world, input);
        systems::simulate(&mut self.world, &mut self.physics, dt);
        systems::bullet_hits(&mut self.world, &mut self.physics);
        if systems::player_in_portal(&self.world) {
            Transition::Switch(SceneId::InGame)
        } else {
            Transition::None
        }
    }
}
