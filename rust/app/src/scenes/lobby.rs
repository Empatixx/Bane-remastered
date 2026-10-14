use std::sync::Arc;

use glam::{Vec2, Vec4};
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::tex;
use crate::assets::{Assets, TILESET};
use crate::audio::{AudioCommand, SoundEffect, Soundtrack};
use crate::camera::Camera;
use crate::components::{Blend, Interaction, UiAction};
use crate::controls::Action;
use crate::input::Input;
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::postfx::SceneFx;
use crate::prefabs::{self, PLAYER_SIZE};
use crate::profile::SharedProfile;
use crate::props::{Look, Prop};
use crate::render::Occluders;
use crate::settings::Settings;
use crate::systems;
use crate::tilemap::{self, TILE_SIZE};
use crate::upgrades::Perks;
use crate::viewport::TARGET;

use super::quest_board::BoardAction;
use super::weapon_table::Outcome as WeaponOutcome;
use super::{Cursor, Pause, QuestBoard, Scene, SceneId, Transition, UpgradeMenu, WeaponTable};

/// `BANE_RICH=1` tops the profile up on every lobby visit, for trying out upgrades.
const RICH_VAR: &str = "BANE_RICH";
const RICH_TALENTS: u64 = 1000;
const RICH_GOLD: u64 = 5000;
const TALENT_COUNTER_COLOR: Vec4 = Vec4::new(222.0 / 255.0, 112.0 / 255.0, 38.0 / 255.0, 1.0);
const GOLD_COUNTER_COLOR: Vec4 = Vec4::new(1.0, 216.0 / 255.0, 0.0, 1.0);
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
    navigation: Navigation,
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

    fn refresh_counters(&mut self) {
        if let Some(profile) = &self.profile {
            systems::update_lobby_counters(&mut self.world, profile.balance(), profile.gold());
        }
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
        self.navigation = Navigation::default();
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
            time_slow: 0.0,
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
        self.navigation = Navigation::new(&solid_cells);
        self.occluders = Some(Arc::new(Occluders::new(
            grid.columns(),
            grid.rows(),
            TILE_SIZE,
            |column, row| grid.is_wall(column as isize, row as isize),
        )));

        let portal_top_left = Vec2::new(bounds.x / 2.0 - 96.0, TILE_SIZE / 2.0);
        prefabs::portal(&mut self.world, portal_top_left, textures);
        systems::tint_portal(
            &mut self.world,
            self.profile.as_ref().and_then(SharedProfile::accent),
        );
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
        let scientist = prefabs::scientist(&mut self.world, scientist_at, textures);
        if let Some(profile) = &self.profile {
            let repertoire = systems::Repertoire::of(&profile.snapshot().standing());
            let _ = self.world.insert_one(scientist, repertoire);
        }
        let head = scientist_at + Vec2::new(Prop::Scientist.spec().size().x / 2.0, 0.0);
        prefabs::minimal_speech(&mut self.world, head);

        let player_top_left = bounds / 2.0;
        prefabs::player(
            &mut self.world,
            &mut self.physics,
            player_top_left + PLAYER_SIZE / 2.0,
            textures,
        );

        let perks = self
            .profile
            .as_ref()
            .map_or_else(Perks::default, SharedProfile::perks);
        if let Some(profile) = &self.profile {
            systems::apply_weapon_mods(&mut self.world, profile.weapon_mods());
        }
        if perks.pet {
            let beside = player_top_left + PLAYER_SIZE / 2.0 + Vec2::new(-80.0, 0.0);
            prefabs::pet(&mut self.world, &mut self.physics, beside, textures);
        }

        prefabs::status_bars(&mut self.world, textures);
        prefabs::lobby_counters(
            &mut self.world,
            textures,
            TALENT_COUNTER_COLOR,
            GOLD_COUNTER_COLOR,
        );
        self.camera = Camera::new(bounds);
        if let Some(profile) = &self.profile {
            if std::env::var_os(RICH_VAR).is_some() {
                profile.top_up(RICH_TALENTS);
                profile.top_up_gold(RICH_GOLD);
            }
        }
        self.refresh_wallet();
        self.refresh_counters();
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
        settings: &mut Settings,
    ) -> Transition {
        self.refresh_counters();
        if self.quests.is_open() || self.weapons.is_open() {
            if input.just_pressed(KeyCode::Escape) || input.action_pressed(Action::Interact) {
                self.quests.close(&mut self.world);
                self.weapons.close(&mut self.world);
                return Transition::None;
            }
            if let Some(profile) = self.profile.clone() {
                match self.quests.update(&mut self.world, input, assets, &profile) {
                    Some(BoardAction::Claimed) => {
                        self.refresh_wallet();
                        self.refresh_counters();
                        systems::say(&mut self.world, "Talents earned!");
                        self.sounds.push(AudioCommand::Effect(SoundEffect::Coin));
                    }
                    Some(BoardAction::Toggled) => {
                        systems::tint_portal(&mut self.world, profile.accent());
                        self.sounds.push(AudioCommand::Effect(SoundEffect::Click));
                    }
                    None => {}
                }
                let outcome = self.weapons.update(
                    &mut self.world,
                    input,
                    assets,
                    &profile,
                    &mut self.sounds,
                    dt,
                );
                if outcome.is_some() {
                    systems::apply_weapon_mods(&mut self.world, profile.weapon_mods());
                    self.refresh_counters();
                }
                if outcome == Some(WeaponOutcome::Bought) {
                    systems::say(&mut self.world, systems::PURCHASE);
                }
            }
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
            self.pause
                .escape(&mut self.world, assets, &prefabs::PAUSE_LOBBY);
        }
        if self.pause.is_active() {
            let action =
                self.pause
                    .update(&mut self.world, input, assets, settings, &mut self.sounds);
            return match action {
                Some(UiAction::Resume) => {
                    self.pause
                        .toggle(&mut self.world, assets, &prefabs::PAUSE_LOBBY);
                    Transition::None
                }
                Some(UiAction::LeaveToMenu) => Transition::Switch(SceneId::Menu),
                Some(UiAction::ExitGame) => Transition::Quit,
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
        systems::update_weapon_hud(&mut self.world, &assets.textures, None);
        systems::tick_ambience(
            &mut self.world,
            assets.textures.get(tex::GAZEWIND_PARTICLE),
            self.camera.position(),
            dt,
        );
        systems::tick_dust(&mut self.world, self.clock, dt);
        if let Some(remark) = systems::chat(&mut self.world, dt) {
            if let Some(profile) = &self.profile {
                profile.heard(remark);
            }
        }
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
                if let Some(profile) = &self.profile {
                    self.quests.toggle(&mut self.world, assets, profile);
                }
                Transition::None
            }
            Some(Interaction::Weapons) => {
                if let Some(profile) = &self.profile {
                    self.weapons.toggle(&mut self.world, assets, profile);
                }
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
            &mut self.sounds,
        );
        systems::steer_player(&mut self.world, input, dt);
        systems::tick_dodges(&mut self.world, dt);
        systems::pick_pet_targets(&mut self.world);
        systems::plan_pet_paths(&mut self.world, &mut self.physics, &self.navigation);
        systems::steer_pets(&mut self.world);
        systems::simulate(&mut self.world, &mut self.physics, dt);
        Transition::None
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{CoinText, Text, Wallet};
    use crate::progress::{ChallengeId, RunRecord};
    use crate::scenes::quest_board;

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
    fn the_scientist_tells_a_waiting_remark_once_and_the_profile_forgets_it() {
        let assets = Assets::new().expect("assets");
        let profile = SharedProfile::in_memory();
        let stats = crate::components::RunStats::default();
        profile.record_run(stats, &systems::summarize(stats, 1), false);
        assert_eq!(
            profile.snapshot().remarks,
            vec![crate::progress::Remark::Deaths(1)]
        );
        let mut scene = LobbyScene::default();
        scene.attach_profile(profile.clone());
        scene.enter(&assets);
        let scientist = scene
            .world
            .query::<&crate::components::Position>()
            .with::<&crate::components::Chatty>()
            .iter()
            .next()
            .map(|position| position.0)
            .expect("the scientist");
        for position in scene
            .world
            .query_mut::<&mut crate::components::Position>()
            .with::<&crate::components::Player>()
        {
            position.0 = scientist;
        }

        scene.update(&Input::default(), 0.016, &assets, &mut Settings::in_memory());

        let said = scene
            .world
            .query::<&crate::components::SpeechBubble>()
            .iter()
            .next()
            .map(|bubble| bubble.message);
        assert_eq!(said, Some("Back already! Good. Good. Nothing is lost."));
        assert!(profile.snapshot().remarks.is_empty());
    }

    #[test]
    fn the_lobby_hands_out_no_free_talents_or_gold() {
        let (scene, _assets) = lobby();

        let profile = scene.profile.as_ref().expect("profile");
        assert_eq!(profile.balance(), 0);
        assert_eq!(profile.gold(), 0);
        let coins: Vec<i32> = scene
            .world
            .query::<&Wallet>()
            .iter()
            .map(|wallet| wallet.coins)
            .collect();
        assert_eq!(coins, vec![0]);
    }

    #[test]
    fn the_lobby_hud_shows_the_talents_and_the_gold_of_the_profile() {
        let (scene, _assets) = lobby();

        let talents: Vec<String> = scene
            .world
            .query::<&Text>()
            .with::<&crate::components::TalentCount>()
            .iter()
            .map(|text| text.content.clone())
            .collect();
        let gold: Vec<String> = scene
            .world
            .query::<&Text>()
            .with::<&crate::components::GoldCount>()
            .iter()
            .map(|text| text.content.clone())
            .collect();

        assert_eq!(talents, vec!["0".to_owned()]);
        assert_eq!(gold, vec!["0".to_owned()]);
    }

    #[test]
    fn the_lobby_hud_follows_the_gold_when_it_changes() {
        let (mut scene, _assets) = lobby();
        let profile = scene.profile.clone().expect("profile");

        profile.top_up_gold(7);
        scene.refresh_counters();

        let gold: Vec<String> = scene
            .world
            .query::<&Text>()
            .with::<&crate::components::GoldCount>()
            .iter()
            .map(|text| text.content.clone())
            .collect();
        assert_eq!(gold, vec!["7".to_owned()]);
    }

    #[test]
    fn the_pet_waits_in_the_lobby_only_for_a_player_who_owns_it() {
        let (without, _assets) = lobby();
        assert_eq!(
            without
                .world
                .query::<&crate::components::Pet>()
                .iter()
                .count(),
            0
        );

        let assets = Assets::new().expect("assets");
        let profile = SharedProfile::in_memory();
        profile.top_up(RICH_TALENTS);
        for _ in 0..8 {
            for upgrade in crate::upgrades::Upgrade::ALL {
                profile.buy(upgrade);
            }
        }
        assert!(profile.perks().pet);
        let mut scene = LobbyScene::default();
        scene.attach_profile(profile);
        scene.enter(&assets);

        assert_eq!(
            scene
                .world
                .query::<&crate::components::Pet>()
                .iter()
                .count(),
            1
        );
    }

    #[test]
    fn the_lobby_pause_menu_offers_resume_settings_main_menu_and_exit_game_and_exit_game_quits() {
        let (mut scene, assets) = lobby();
        let mut input = Input::default();
        input.press(KeyCode::Escape);
        scene.update(&input, 0.0, &assets, &mut Settings::in_memory());

        let mut labels: Vec<String> = scene
            .world
            .query::<&Text>()
            .with::<&crate::components::TextButton>()
            .iter()
            .map(|text| text.content.clone())
            .collect();
        labels.sort();
        assert_eq!(labels, vec!["Exit Game", "Main Menu", "Resume", "Settings"]);

        let at = scene
            .world
            .query::<(&crate::components::Position, &Text)>()
            .with::<&crate::components::TextButton>()
            .iter()
            .find(|(_, text)| text.content == "Exit Game")
            .map(|(position, _)| position.0)
            .expect("the button");
        let mut click = Input::default();
        click.set_mouse(at);
        click.handle_click(
            winit::event::MouseButton::Left,
            winit::event::ElementState::Pressed,
        );
        let transition = scene.update(&click, 0.0, &assets, &mut Settings::in_memory());

        assert!(matches!(transition, Transition::Quit));
    }

    #[test]
    fn clicking_a_challenge_note_selects_it_and_clicking_again_cancels_it() {
        let (mut scene, assets) = lobby();
        let profile = scene.profile.clone().expect("profile");
        scene.quests.toggle(&mut scene.world, &assets, &profile);
        let click = |at: Vec2| {
            let mut input = Input::default();
            input.set_mouse(at);
            input.handle_click(
                winit::event::MouseButton::Left,
                winit::event::ElementState::Pressed,
            );
            input
        };
        let tab = quest_board::tab_top_left(quest_board::Tab::Challenges) + Vec2::splat(5.0);
        scene.update(&click(tab), 0.0, &assets, &mut Settings::in_memory());
        let note = quest_board::note_top_left(0) + Vec2::splat(20.0);

        scene.update(&click(note), 0.0, &assets, &mut Settings::in_memory());
        assert_eq!(profile.challenge(), Some(ChallengeId::ALL[0]));

        scene.update(&click(note), 0.0, &assets, &mut Settings::in_memory());
        assert_eq!(profile.challenge(), None);
    }

    #[test]
    fn clicking_a_finished_mission_pays_its_talents_once() {
        let (mut scene, assets) = lobby();
        let profile = scene.profile.clone().expect("profile");
        let huge = crate::components::RunStats {
            fired: 1_000_000,
            hits: 1_000_000,
            kills: 1_000_000,
            items: 1_000_000,
            kills_of: [1_000_000; crate::enemies::EnemyKind::ALL.len()],
            gold: 1_000_000,
            untouched_floors: 1_000_000,
            ..Default::default()
        };
        profile.finish_run(&RunRecord {
            stats: huge,
            floor: 50,
            victory: true,
            challenge: None,
        });
        let finished = profile.board().missions[0].clone();
        assert!(finished.done);
        let before = profile.balance();
        scene.quests.toggle(&mut scene.world, &assets, &profile);
        let mut input = Input::default();
        input.set_mouse(quest_board::note_top_left(0) + Vec2::splat(20.0));
        input.handle_click(
            winit::event::MouseButton::Left,
            winit::event::ElementState::Pressed,
        );

        scene.update(&input, 0.0, &assets, &mut Settings::in_memory());
        assert_eq!(profile.balance(), before + u64::from(finished.talents));
        assert!(!profile.board().missions[0].done);

        scene.update(&input, 0.0, &assets, &mut Settings::in_memory());
        assert_eq!(profile.balance(), before + u64::from(finished.talents));
    }
}
