use glam::{Vec2, Vec4};
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::{Assets, BULLET, RAT, TILESET};
use crate::audio::{AudioCommand, SoundEffect, Soundtrack};
use crate::camera::Camera;
use crate::components::{UiAction, Weapon};
use crate::input::Input;
use crate::physics::Physics;
use crate::prefabs;
use crate::settings::Settings;
use crate::systems;
use crate::tilemap::{self, TILE_SIZE};
use crate::viewport::TARGET;

use super::{Cursor, Pause, Scene, SceneId, Transition};

#[derive(Default)]
pub(crate) struct InGameScene {
    world: World,
    physics: Physics,
    camera: Camera,
    pause: Pause,
    sounds: Vec<AudioCommand>,
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
        self.pause = Pause::default();
    }

    fn soundtrack(&self) -> Option<Soundtrack> {
        Some(Soundtrack::InGame)
    }

    fn take_sounds(&mut self) -> Vec<AudioCommand> {
        std::mem::take(&mut self.sounds)
    }

    fn cursor(&self) -> Cursor {
        if self.pause.is_active() {
            return Cursor::Default;
        }
        let reloading = self
            .world
            .query::<&Weapon>()
            .with::<&crate::components::Player>()
            .iter()
            .any(|weapon| weapon.reloading.is_some());
        if reloading {
            Cursor::Reload
        } else {
            Cursor::Crosshair
        }
    }

    fn camera(&self) -> Vec2 {
        self.camera.position()
    }

    fn enter(&mut self, assets: &Assets) {
        let grid = tilemap::arena();
        let bounds = grid.pixel_size();
        let textures = &assets.textures;
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
        prefabs::status_bars(&mut self.world, textures);
        prefabs::coin_hud(
            &mut self.world,
            textures,
            "ingame/banecoin.tga",
            Vec4::new(1.0, 216.0 / 255.0, 0.0, 1.0),
        );
        prefabs::weapon_hud(&mut self.world, textures, Weapon::pistol().clip_size);
        self.camera = Camera::new(bounds);
        if let Some(target) = systems::camera_target(&self.world, TARGET / 2.0) {
            self.camera.snap_to(target);
        }
    }

    fn update(
        &mut self,
        input: &Input,
        dt: f32,
        assets: &Assets,
        _settings: &mut Settings,
    ) -> Transition {
        if input.just_pressed(KeyCode::Escape) {
            self.pause.toggle(&mut self.world, assets);
        }
        if self.pause.is_active() {
            let action = self.pause.update(&mut self.world, input);
            if action.is_some() {
                self.sounds.push(AudioCommand::Effect(SoundEffect::Click));
            }
            return match action {
                Some(UiAction::Resume) => {
                    self.pause.toggle(&mut self.world, assets);
                    Transition::None
                }
                Some(UiAction::LeaveToMenu) => Transition::Switch(SceneId::Menu),
                _ => Transition::None,
            };
        }
        systems::shoot(
            &mut self.world,
            &mut self.physics,
            input,
            systems::ShotContext {
                camera: self.camera.position(),
                bullet_texture: assets.textures.get(BULLET),
                dt,
                roll: systems::ShotRoll::random(),
            },
            &mut self.sounds,
        );
        systems::animate_player(&mut self.world, input);
        systems::tick_invulnerability(&mut self.world, dt);
        systems::update_status_bars(&mut self.world, dt);
        systems::update_weapon_hud(&mut self.world);
        systems::remove_expired(&mut self.world);
        if let Some(target) = systems::camera_target(&self.world, input.mouse()) {
            self.camera.follow(target, dt);
        }
        Transition::None
    }

    fn fixed_update(&mut self, input: &Input, dt: f32) -> Transition {
        if self.pause.is_active() {
            return Transition::None;
        }
        systems::steer_player(&mut self.world, input);
        systems::chase_player(&mut self.world);
        systems::simulate(&mut self.world, &mut self.physics, dt);
        systems::bullet_hits(&mut self.world, &mut self.physics, &mut self.sounds);
        systems::reap_dead(&mut self.world, &mut self.physics);
        systems::contact_damage(&mut self.world, &mut self.sounds);
        if systems::player_is_dead(&self.world) {
            Transition::Switch(SceneId::Lobby)
        } else {
            Transition::None
        }
    }
}
