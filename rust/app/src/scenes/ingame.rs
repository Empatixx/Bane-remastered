use std::sync::Arc;

use glam::Vec2;
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::Textures;
use crate::input::Input;
use crate::prefabs;
use crate::systems;
use crate::tilemap::{self, Tileset};

use super::{Scene, SceneId, Transition};

pub(crate) struct InGameScene {
    world: World,
    textures: Textures,
    tileset: Arc<Tileset>,
    bounds: Vec2,
    camera: Vec2,
}

impl InGameScene {
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

impl Scene for InGameScene {
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
        let grid = tilemap::arena();
        self.bounds = grid.pixel_size();
        prefabs::tilemap(&mut self.world, &grid, &self.tileset, self.textures.tileset);
        prefabs::player(&mut self.world, self.bounds / 2.0);
        self.camera = systems::follow_player(&self.world, self.bounds);
    }

    fn update(&mut self, input: &Input, _dt: f32) -> Transition {
        self.camera = systems::follow_player(&self.world, self.bounds);
        if input.just_pressed(KeyCode::Escape) {
            Transition::Switch(SceneId::Lobby)
        } else {
            Transition::None
        }
    }

    fn fixed_update(&mut self, input: &Input, dt: f32) -> Transition {
        systems::steer_player(&mut self.world, input);
        systems::move_entities(&mut self.world, dt);
        Transition::None
    }
}
