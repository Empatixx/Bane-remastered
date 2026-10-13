use std::sync::Arc;

use glam::Vec2;
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::tex;
use crate::assets::{Assets, TILESET};
use crate::audio::{AudioCommand, SoundEffect, Soundtrack};
use crate::camera::Camera;
use crate::components::{Blend, Interaction, UiAction};
use crate::controls::Action;
use crate::input::Input;
use crate::physics::Physics;
use crate::postfx::SceneFx;
use crate::prefabs::{self, PLAYER_SIZE};
use crate::profile::SharedProfile;
use crate::props::{Look, Prop};
use crate::render::Occluders;
use crate::settings::Settings;
use crate::systems;
use crate::tilemap::{self, TILE_SIZE};
use crate::viewport::TARGET;

use super::{Cursor, Pause, QuestBoard, Scene, SceneId, Transition, UpgradeMenu, WeaponTable};

const TALENTS_TO_TRY_OUT: u64 = 1000;
const POTS: [(f32, f32); 3] = [(14.0, 6.0), (13.0, 6.0), (12.0, 6.0)];

const PROPS: [(Prop, f32, f32, Look); 22] = [
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
    blend: Blend,
    pause: Pause,
    upgrades: UpgradeMenu,
    quests: QuestBoard,
    weapons: WeaponTable,
    occluders: Option<Arc<Occluders>>,
    clock: f32,
    sounds: Vec<AudioCommand>,
    profile: Option<SharedProfile>,
}

impl LobbyScene {
    fn panel_is_open(&self) -> bool {
        self.upgrades.is_open() || self.quests.is_open() || self.weapons.is_open()
    }

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
        self.quests = QuestBoard::default();
        self.weapons = WeaponTable::default();
        self.occluders = None;
    }

    fn occluders(&self) -> Option<Arc<Occluders>> {
        self.occluders.clone()
    }

    fn soundtrack(&self) -> Option<Soundtrack> {
        Some(Soundtrack::ProgressRoom)
    }

    fn take_sounds(&mut self) -> Vec<AudioCommand> {
        std::mem::take(&mut self.sounds)
    }

    fn fx(&self) -> SceneFx {
        SceneFx {
            paused: self.pause.is_active() || self.panel_is_open(),
            low_health: systems::player_is_low_on_health(&self.world),
        }
    }

    fn cursor(&self) -> Cursor {
        Cursor::Default
    }

    fn camera(&self) -> Vec2 {
        self.camera.position()
    }

    fn set_blend(&mut self, blend: f32) {
        self.blend = Blend(blend);
    }

    fn blend(&self) -> f32 {
        self.blend.0
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
        self.occluders = Some(Arc::new(Occluders::new(
            grid.columns(),
            grid.rows(),
            TILE_SIZE,
            |column, row| grid.is_wall(column as isize, row as isize),
        )));

        let portal_top_left = Vec2::new(bounds.x / 2.0 - 96.0, TILE_SIZE / 2.0);
        prefabs::portal(&mut self.world, portal_top_left, textures);
        systems::spawn_ambience(
            &mut self.world,
            textures.get(tex::GAZEWIND_PARTICLE),
            bounds,
            portal_top_left + Prop::Portal.spec().size() / 2.0,
            0x1d57,
        );
        for (prop, x, y, look) in PROPS {
            let top_left = Vec2::new(x, y) * TILE_SIZE;
            prefabs::prop(&mut self.world, prop, top_left, look, textures);
        }
        prefabs::task_board(&mut self.world, Vec2::new(18.5, 0.0) * TILE_SIZE, textures);
        prefabs::weapon_table(&mut self.world, Vec2::new(23.5, 0.5) * TILE_SIZE, textures);
        for (x, y) in POTS {
            let top_left = Vec2::new(x, y) * TILE_SIZE;
            prefabs::pot(&mut self.world, &mut self.physics, top_left, textures);
        }
        let scientist_at = Vec2::new(25.0, 6.5) * TILE_SIZE;
        prefabs::scientist(&mut self.world, scientist_at, textures);
        let head = scientist_at + Vec2::new(Prop::Scientist.spec().size().x / 2.0, 0.0);
        prefabs::minimal_speech(&mut self.world, head);

        let player_top_left = bounds / 2.0;
        prefabs::player(
            &mut self.world,
            &mut self.physics,
            player_top_left + PLAYER_SIZE / 2.0,
            textures,
        );

        prefabs::status_bars(&mut self.world, textures);
        self.camera = Camera::new(bounds);
        if let Some(profile) = &self.profile {
            profile.top_up(TALENTS_TO_TRY_OUT);
        }
        self.refresh_wallet();
        if let Some(target) = systems::camera_target(&self.world, TARGET / 2.0, 1.0) {
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
        if self.quests.is_open() || self.weapons.is_open() {
            if input.just_pressed(KeyCode::Escape) || input.action_pressed(Action::Interact) {
                self.quests.close(&mut self.world);
                self.weapons.close(&mut self.world);
                return Transition::None;
            }
            self.quests.update(&mut self.world, input, assets);
            self.weapons.update(&mut self.world, input, assets, dt);
            return Transition::None;
        }
        if self.upgrades.is_open() {
            if input.just_pressed(KeyCode::Escape) || input.action_pressed(Action::Interact) {
                self.upgrades.close(&mut self.world);
                return Transition::None;
            }
            if let Some(profile) = &self.profile {
                let outcome = self.upgrades.update(
                    &mut self.world,
                    input,
                    assets,
                    profile,
                    &mut self.sounds,
                    dt,
                );
                if outcome.is_some() {
                    self.refresh_wallet();
                    systems::say(&mut self.world, systems::PURCHASE);
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
        systems::show_dodge_visual(&mut self.world, &assets.textures);
        systems::show_dodge_cooldown(&mut self.world);
        systems::fade_afterimages(&mut self.world, dt);
        systems::tick_invulnerability(&mut self.world, dt);
        self.clock += dt;
        systems::spawn_gaze_wind(
            &mut self.world,
            &self.physics,
            assets.textures.get(tex::GAZEWIND_PARTICLE),
            dt,
            self.clock,
        );
        systems::update_shadow(&mut self.world, dt);
        systems::update_status_bars(&mut self.world, dt);
        systems::update_weapon_hud(&mut self.world, &assets.textures);
        systems::tick_ambience(
            &mut self.world,
            assets.textures.get(tex::GAZEWIND_PARTICLE),
            self.camera.position(),
            dt,
        );
        systems::tick_dust(&mut self.world, self.clock, dt);
        systems::chat(&mut self.world, dt);
        systems::tick_speech(&mut self.world, dt);
        systems::fit_speech(&mut self.world, &assets.fonts);
        systems::remove_expired(&mut self.world);
        if let Some(target) = systems::camera_target(&self.world, input.mouse(), self.blend.0) {
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
            Some(Interaction::Quests) => {
                self.quests.toggle(&mut self.world, assets);
                Transition::None
            }
            Some(Interaction::Weapons) => {
                self.weapons.toggle(&mut self.world, assets);
                Transition::None
            }
            _ => Transition::None,
        }
    }

    fn fixed_update(&mut self, input: &Input, dt: f32, _assets: &Assets) -> Transition {
        if self.pause.is_active() || self.panel_is_open() {
            return Transition::None;
        }
        systems::start_dodge(
            &mut self.world,
            input,
            input.mouse() + self.camera.position(),
        );
        systems::steer_player(&mut self.world, input, dt);
        systems::tick_dodges(&mut self.world, dt);
        systems::simulate(&mut self.world, &mut self.physics, dt);
        Transition::None
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{CoinText, Text, Wallet};

    use super::*;

    fn lobby() -> (LobbyScene, Assets) {
        let assets = Assets::new().expect("assets");
        let mut scene = LobbyScene::default();
        scene.attach_profile(SharedProfile::in_memory());
        scene.enter(&assets);
        (scene, assets)
    }

    #[test]
    fn the_lobby_shows_no_balance_counter_neither_on_the_left_nor_at_the_scientist() {
        let (scene, _assets) = lobby();

        assert_eq!(scene.world.query::<&CoinText>().iter().count(), 0);
        let counters = scene
            .world
            .query::<&Text>()
            .iter()
            .filter(|text| text.content.starts_with("Talents"))
            .count();
        assert_eq!(counters, 0);
    }

    #[test]
    fn the_player_starts_the_lobby_with_a_thousand_talents_to_try_things_out() {
        let (scene, _assets) = lobby();

        let profile = scene.profile.as_ref().expect("profile");
        assert_eq!(profile.balance(), TALENTS_TO_TRY_OUT);
        let coins: Vec<i32> = scene
            .world
            .query::<&Wallet>()
            .iter()
            .map(|wallet| wallet.coins)
            .collect();
        assert_eq!(coins, vec![TALENTS_TO_TRY_OUT as i32]);
    }
}
