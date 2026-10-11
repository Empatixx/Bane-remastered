use glam::Vec2;
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::{Assets, BULLET, TILESET};
use crate::camera::Camera;
use crate::components::Interaction;
use crate::input::Input;
use crate::physics::Physics;
use crate::prefabs::{self, PLAYER_SIZE};
use crate::props::{Look, Prop};
use crate::systems;
use crate::tilemap::{self, TILE_SIZE};
use crate::viewport::TARGET;

use super::{Scene, SceneId, Transition};

const POTS: [(f32, f32); 3] = [(14.0, 6.0), (13.0, 6.0), (12.0, 6.0)];
const POT_COLLIDER_OFFSET: Vec2 = Vec2::new(75.0, 78.0);
const POT_COLLIDER_SIZE: Vec2 = Vec2::new(45.0, 45.0);

const PROPS: [(Prop, f32, f32, Look); 24] = [
    (Prop::WeaponRack, 22.75, 0.5, Look::DEFAULT),
    (Prop::WeaponRack, 9.25, 0.5, Look::DEFAULT),
    (Prop::Torch, 1.0, 2.0, Look::DEFAULT),
    (Prop::Torch, 10.0, 2.0, Look::FLIPPED),
    (Prop::Torch, 18.0, 2.0, Look::DEFAULT),
    (Prop::Torch, 27.0, 2.0, Look::FLIPPED),
    (Prop::Bones, 20.0, 7.0, Look::SECOND_ROW),
    (Prop::Bones, 23.0, 4.0, Look::DEFAULT),
    (Prop::Cobweb, 10.0, 1.0, Look::SECOND_ROW),
    (Prop::Cobweb, 18.0, 8.0, Look::DEFAULT),
    (Prop::Armorstand, 11.5, 1.75, Look::DEFAULT),
    (Prop::Armorstand, 16.5, 1.75, Look::DEFAULT),
    (Prop::TaskBoard, 18.5, 0.0, Look::DEFAULT),
    (Prop::CraftingTable, 23.5, 0.5, Look::DEFAULT),
    (Prop::Box, 26.0, 7.0, Look::DEFAULT),
    (Prop::Chains, 18.0, 0.0, Look::DEFAULT),
    (Prop::Chains, 5.0, 0.0, Look::DEFAULT),
    (Prop::Rock, 26.0, 4.0, Look::DEFAULT),
    (Prop::Rock, 17.0, 5.0, Look::SECOND_ROW),
    (Prop::Rock, 3.0, 3.0, Look::DEFAULT),
    (Prop::Rock, 8.0, 7.0, Look::SECOND_ROW),
    (Prop::Bookshelf, 2.0, 0.0, Look::DEFAULT),
    (Prop::Bookshelf, 6.0, 0.0, Look::DEFAULT),
    (Prop::MapTable, 5.0, 5.0, Look::DEFAULT),
];

#[derive(Default)]
pub(crate) struct LobbyScene {
    world: World,
    physics: Physics,
    camera: Camera,
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
        self.camera.position()
    }

    fn enter(&mut self, assets: &Assets) {
        let grid = tilemap::lobby();
        let bounds = grid.pixel_size();
        let textures = &assets.textures;

        let solid_cells = prefabs::tilemap(
            &mut self.world,
            &grid,
            &assets.tileset,
            textures.get(TILESET),
        );
        self.physics.add_walls(&solid_cells, TILE_SIZE);

        let portal_top_left = Vec2::new(bounds.x / 2.0 - 96.0, TILE_SIZE / 2.0);
        prefabs::portal(&mut self.world, portal_top_left, textures);
        for (prop, x, y, look) in PROPS {
            let top_left = Vec2::new(x, y) * TILE_SIZE;
            prefabs::prop(&mut self.world, prop, top_left, look, textures);
        }
        for (x, y) in POTS {
            let top_left = Vec2::new(x, y) * TILE_SIZE;
            prefabs::prop(
                &mut self.world,
                Prop::Pot,
                top_left,
                Look::DEFAULT,
                textures,
            );
            let collider_center = top_left + POT_COLLIDER_OFFSET + POT_COLLIDER_SIZE / 2.0;
            self.physics
                .add_static_box(collider_center, POT_COLLIDER_SIZE);
        }
        prefabs::scientist(&mut self.world, Vec2::new(25.0, 6.5) * TILE_SIZE, textures);

        let player_top_left = bounds / 2.0;
        prefabs::player(
            &mut self.world,
            &mut self.physics,
            player_top_left + PLAYER_SIZE / 2.0,
            textures,
        );

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
            return Transition::Switch(SceneId::Menu);
        }
        match systems::interact(&mut self.world, input) {
            Some(Interaction::EnterPortal) => Transition::Switch(SceneId::InGame),
            Some(Interaction::Upgrade) => {
                tracing::info!("the upgrade menu is not implemented yet");
                Transition::None
            }
            None => Transition::None,
        }
    }

    fn fixed_update(&mut self, input: &Input, dt: f32) -> Transition {
        systems::steer_player(&mut self.world, input);
        systems::simulate(&mut self.world, &mut self.physics, dt);
        systems::bullet_hits(&mut self.world, &mut self.physics);
        Transition::None
    }
}
