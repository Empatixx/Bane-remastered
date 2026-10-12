use glam::{Vec2, Vec4};
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::{Assets, TILESET};
use crate::audio::{AudioCommand, SoundEffect, Soundtrack};
use crate::camera::Camera;
use crate::components::{Interaction, UiAction};
use crate::controls::Action;
use crate::input::Input;
use crate::physics::Physics;
use crate::postfx::SceneFx;
use crate::prefabs::{self, PLAYER_SIZE};
use crate::profile::SharedProfile;
use crate::props::{Look, Prop};
use crate::settings::Settings;
use crate::systems;
use crate::tilemap::{self, TILE_SIZE};
use crate::viewport::TARGET;

use super::{Cursor, Pause, Scene, SceneId, Transition, UpgradeMenu};

const POTS: [(f32, f32); 3] = [(14.0, 6.0), (13.0, 6.0), (12.0, 6.0)];

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
    pause: Pause,
    upgrades: UpgradeMenu,
    clock: f32,
    sounds: Vec<AudioCommand>,
    profile: Option<SharedProfile>,
}

impl LobbyScene {
    pub(crate) fn attach_profile(&mut self, profile: SharedProfile) {
        self.profile = Some(profile);
    }

    fn refresh_wallet(&mut self) {
        if let Some(profile) = &self.profile {
            let balance = i32::try_from(profile.balance()).unwrap_or(i32::MAX);
            for wallet in self.world.query_mut::<&mut crate::components::Wallet>() {
                wallet.coins = balance;
            }
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
        self.pause = Pause::default();
        self.upgrades = UpgradeMenu::default();
    }

    fn soundtrack(&self) -> Option<Soundtrack> {
        Some(Soundtrack::ProgressRoom)
    }

    fn take_sounds(&mut self) -> Vec<AudioCommand> {
        std::mem::take(&mut self.sounds)
    }

    fn fx(&self) -> SceneFx {
        SceneFx {
            paused: self.pause.is_active() || self.upgrades.is_open(),
            low_health: systems::player_is_low_on_health(&self.world),
        }
    }

    fn cursor(&self) -> Cursor {
        Cursor::Default
    }

    fn camera(&self) -> Vec2 {
        self.camera.position()
    }

    fn enter(&mut self, assets: &Assets) {
        let grid = tilemap::lobby();
        let bounds = grid.pixel_size();
        let textures = &assets.textures;
        prefabs::backdrop(&mut self.world);

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
            prefabs::pot(&mut self.world, &mut self.physics, top_left, textures);
        }
        prefabs::scientist(&mut self.world, Vec2::new(25.0, 6.5) * TILE_SIZE, textures);

        let player_top_left = bounds / 2.0;
        prefabs::player(
            &mut self.world,
            &mut self.physics,
            player_top_left + PLAYER_SIZE / 2.0,
            textures,
        );

        prefabs::status_bars(&mut self.world, textures);
        prefabs::coin_hud(
            &mut self.world,
            textures,
            "ingame/green_banecoin.tga",
            Vec4::new(236.0 / 255.0, 0.0, 0.0, 1.0),
        );
        self.camera = Camera::new(bounds);
        if let Some(profile) = &self.profile {
            let balance = i32::try_from(profile.balance()).unwrap_or(i32::MAX);
            for wallet in self.world.query_mut::<&mut crate::components::Wallet>() {
                wallet.coins = balance;
            }
        }
        if let Some(target) = systems::camera_target(&self.world, TARGET / 2.0) {
            self.camera.snap_to(target);
        }
        systems::follow_backdrop(&mut self.world, self.camera.position());
    }

    fn update(
        &mut self,
        input: &Input,
        dt: f32,
        assets: &Assets,
        _settings: &mut Settings,
    ) -> Transition {
        if self.upgrades.is_open() {
            if input.just_pressed(KeyCode::Escape) || input.action_pressed(Action::Interact) {
                self.upgrades.close(&mut self.world);
                return Transition::None;
            }
            if let Some(profile) = &self.profile {
                let outcome =
                    self.upgrades
                        .update(&mut self.world, input, assets, profile, &mut self.sounds);
                if outcome.is_some() {
                    self.refresh_wallet();
                }
            }
            return Transition::None;
        }
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
        systems::animate_player(&mut self.world, input);
        systems::tick_invulnerability(&mut self.world, dt);
        self.clock += dt;
        systems::spawn_gaze_wind(
            &mut self.world,
            &self.physics,
            assets.textures.get("ingame/gazewind_particle.tga"),
            dt,
            self.clock,
        );
        systems::update_shadow(&mut self.world, dt);
        systems::update_status_bars(&mut self.world, dt);
        systems::update_weapon_hud(&mut self.world, &assets.textures);
        systems::remove_expired(&mut self.world);
        if let Some(target) = systems::camera_target(&self.world, input.mouse()) {
            self.camera.follow(target, dt);
        }
        systems::follow_backdrop(&mut self.world, self.camera.position());
        match systems::interact(&mut self.world, input) {
            Some(Interaction::EnterPortal) => Transition::Switch(SceneId::InGame),
            Some(Interaction::Upgrade) => {
                if let Some(profile) = &self.profile {
                    self.upgrades.toggle(&mut self.world, assets, profile);
                }
                Transition::None
            }
            _ => Transition::None,
        }
    }

    fn fixed_update(&mut self, input: &Input, dt: f32, _assets: &Assets) -> Transition {
        if self.pause.is_active() || self.upgrades.is_open() {
            return Transition::None;
        }
        systems::steer_player(&mut self.world, input);
        systems::simulate(&mut self.world, &mut self.physics, dt);
        Transition::None
    }
}
