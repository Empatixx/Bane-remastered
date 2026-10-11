use std::sync::Arc;

use glam::Vec2;
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::Textures;
use crate::input::Input;
use crate::prefabs;
use crate::systems;
use crate::tilemap::{self, TileGrid, Tileset, TILE_SIZE};

use super::{Scene, SceneId, Transition};

pub(crate) struct LobbyScene {
    world: World,
    textures: Textures,
    tileset: Arc<Tileset>,
    bounds: Vec2,
    camera: Vec2,
}

impl LobbyScene {
    pub(crate) fn new(textures: Textures, tileset: Arc<Tileset>) -> Self {
        Self {
            world: World::new(),
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

    fn camera(&self) -> Vec2 {
        self.camera
    }

    fn enter(&mut self) {
        let grid: TileGrid = tilemap::lobby();
        self.bounds = grid.pixel_size();
        prefabs::tilemap(&mut self.world, &grid, &self.tileset, self.textures.tileset);
        prefabs::portal(
            &mut self.world,
            Vec2::new(self.bounds.x - 2.5 * TILE_SIZE, self.bounds.y / 2.0),
        );
        prefabs::player(
            &mut self.world,
            Vec2::new(3.5 * TILE_SIZE, self.bounds.y / 2.0),
        );
        self.camera = systems::follow_player(&self.world, self.bounds);
    }

    fn update(&mut self, input: &Input, _dt: f32) -> Transition {
        self.camera = systems::follow_player(&self.world, self.bounds);
        if input.just_pressed(KeyCode::Escape) {
            Transition::Switch(SceneId::Menu)
        } else {
            Transition::None
        }
    }

    fn fixed_update(&mut self, input: &Input, dt: f32) -> Transition {
        systems::steer_player(&mut self.world, input);
        systems::move_entities(&mut self.world, dt);
        if systems::player_in_portal(&self.world) {
            Transition::Switch(SceneId::InGame)
        } else {
            Transition::None
        }
    }
}
