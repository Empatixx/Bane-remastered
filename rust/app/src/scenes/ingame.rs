use std::sync::Arc;

use glam::{Vec2, Vec4};
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::tex;
use crate::assets::{Assets, BULLET};
use crate::audio::{AudioCommand, SoundEffect, Soundtrack};
use crate::camera::Camera;
use crate::components::{
    BigMap, Blend, Hidden, InfiniteAmmo, Interaction, Loadout, MapSize, Minimap, UiAction,
};
use crate::controls::Action;
use crate::dungeon::{DecorKind, Dungeon, RoomKind, Surfaces, Theme};
use crate::flow::FlowField;
use crate::input::Input;
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::postfx::SceneFx;
use crate::prefabs;
use crate::profile::SharedProfile;
use crate::progress::{ChallengeId, RunOutcome, RunRecord};
use crate::render::Occluders;
use crate::rng::{Rng, Stream};
use crate::rules::RunRules;
use crate::settings::Settings;
use crate::systems;
use crate::tilemap::TILE_SIZE;
use crate::upgrades::Perks;
use crate::viewport::TARGET;
use crate::weapon_upgrades::WeaponMods;

use super::{Cursor, Pause, Scene, SceneId, Transition};

const BOSS_DEFEATED_SHAKE: f32 = 1.0;

fn impact_shake(effect: SoundEffect) -> f32 {
    match effect {
        SoundEffect::Explosion => 0.8,
        SoundEffect::Death => 0.7,
        SoundEffect::PlayerHurt => 0.5,
        _ => 0.0,
    }
}


#[derive(Default)]
pub(crate) struct InGameScene {
    world: World,
    physics: Physics,
    camera: Camera,
    blend: Blend,
    pause: Pause,
    clock: f32,
    sounds: Vec<AudioCommand>,
    throttle: crate::audio::Throttle,
    hit: bool,
    dungeon: Option<Dungeon>,
    occluders: Option<Arc<Occluders>>,
    rooms: Option<systems::RoomRuntime>,
    run_seed: u64,
    map_rng: Rng,
    loot_rng: Rng,
    combat_rng: Rng,
    big_map: bool,
    floor: u32,
    navigation: Navigation,
    surfaces: Surfaces,
    flow: FlowField,
    victory: bool,
    leave_to_menu: bool,
    death: Option<systems::DeathScreen>,
    profile: Option<SharedProfile>,
    rules: RunRules,
    challenge: Option<ChallengeId>,
    kills_without_drop: u32,
    drag_from: Option<Vec2>,
    /// Whether the next start is the training crypt rather than a run.
    training_mode: bool,
    training: Option<systems::Training>,
    /// Where the merchant's words show on this floor, if it has a shop.
    keeper: Option<Vec2>,
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
        self.dungeon = None;
        self.occluders = None;
        self.rooms = None;
        self.big_map = false;
    }

    fn soundtrack(&self) -> Option<Soundtrack> {
        Some(Soundtrack::InGame)
    }

    fn take_sounds(&mut self) -> Vec<AudioCommand> {
        let listener = self
            .world
            .query::<&crate::components::Position>()
            .with::<&crate::components::Player>()
            .iter()
            .next()
            .map(|position| position.0);
        crate::audio::resolve(
            std::mem::take(&mut self.sounds),
            listener,
            &mut self.throttle,
            self.clock,
        )
    }

    fn fx(&self) -> SceneFx {
        SceneFx {
            paused: self.pause.is_active(),
            low_health: systems::player_is_low_on_health(&self.world),
            time_slow: self.time_slow(),
        }
    }

    fn take_hit(&mut self) -> bool {
        std::mem::take(&mut self.hit)
    }

    fn cursor(&self) -> Cursor {
        if self.pause.is_active() || self.big_map || self.death.is_some() {
            return Cursor::Default;
        }
        let reloading = self
            .world
            .query::<&Loadout>()
            .with::<&crate::components::Player>()
            .iter()
            .any(|loadout| loadout.current().is_some_and(|w| w.reloading.is_some()));
        if reloading {
            Cursor::Reload
        } else {
            Cursor::Crosshair
        }
    }

    fn camera(&self) -> Vec2 {
        self.camera.view()
    }

    fn occluders(&self) -> Option<Arc<Occluders>> {
        self.occluders.clone()
    }

    fn set_blend(&mut self, blend: f32) {
        self.blend = Blend(blend);
    }

    fn blend(&self) -> f32 {
        self.blend.0
    }

    fn enter(&mut self, assets: &Assets) {
        self.start_run(Rng::run_seed(), assets);
    }

    fn update(
        &mut self,
        input: &Input,
        dt: f32,
        assets: &Assets,
        settings: &mut Settings,
    ) -> Transition {
        self.camera.update_shake(dt);
        if self.training.is_some() && systems::player_is_dead(&self.world) {
            if let Some(training) = &mut self.training {
                training.respawn(&mut self.world, &mut self.physics, &assets.textures);
            }
        }
        if self.death.is_none() && (self.victory || systems::player_is_dead(&self.world)) {
            self.start_death_screen(assets);
        }
        if let Some(screen) = &mut self.death {
            screen.advance(dt, input.any_key());
            systems::update_death_ui(&mut self.world, screen);
            systems::update_status_bars(&mut self.world, dt);
            if screen.finished() {
                return Transition::Switch(if self.leave_to_menu {
                    SceneId::Menu
                } else {
                    SceneId::Lobby
                });
            }
            systems::follow_backdrop(&mut self.world, self.camera.view());
            return Transition::None;
        }
        if input.action_pressed(Action::Map) && !self.pause.is_active() {
            self.set_big_map(!self.big_map);
        }
        if input.just_pressed(KeyCode::Escape) {
            if self.big_map {
                self.set_big_map(false);
            } else {
                let buttons = self.pause_buttons();
                self.pause.escape(&mut self.world, assets, buttons);
            }
        }
        if self.pause.is_active() {
            let action =
                self.pause
                    .update(&mut self.world, input, assets, settings, &mut self.sounds);
            return match action {
                Some(UiAction::Resume) => {
                    let buttons = self.pause_buttons();
                    self.pause.toggle(&mut self.world, assets, buttons);
                    Transition::None
                }
                Some(UiAction::SkipTraining) => {
                    let buttons = self.pause_buttons();
                    self.pause.toggle(&mut self.world, assets, buttons);
                    self.pause.toggle(&mut self.world, assets, &prefabs::PAUSE_SKIP);
                    Transition::None
                }
                Some(UiAction::ConfirmSkip) => self.finish_training(),
                Some(UiAction::Die) => {
                    self.pause
                        .toggle(&mut self.world, assets, &prefabs::PAUSE_GAME);
                    self.start_death_screen(assets);
                    Transition::None
                }
                Some(UiAction::LeaveToMenu) => {
                    self.pause
                        .toggle(&mut self.world, assets, &prefabs::PAUSE_GAME);
                    self.leave_to_menu = true;
                    self.start_death_screen(assets);
                    Transition::None
                }
                Some(UiAction::ExitGame) => Transition::Quit,
                _ => Transition::None,
            };
        }
        if self.big_map {
            if input.mouse_down() {
                if let Some(from) = self.drag_from {
                    systems::pan_big_maps(&mut self.world, input.mouse() - from);
                }
                self.drag_from = Some(input.mouse());
            } else {
                self.drag_from = None;
            }
        }
        if !self.big_map {
            let recoil = systems::shoot(
                &mut self.world,
                &mut self.physics,
                input,
                systems::ShotContext {
                    camera: self.camera.position(),
                    rounds: systems::RoundTextures::load(&assets.textures),
                    casing_texture: assets.textures.get(tex::CASING),
                    dt,
                    roll: systems::ShotRoll::from(&mut self.combat_rng),
                },
                &mut self.sounds,
            );
            self.camera.add_trauma(recoil);
        }
        systems::tick_casings(&mut self.world, dt);
        systems::animate_player(&mut self.world, input);
        systems::show_dodge_visual(&mut self.world, &assets.textures);
        systems::show_dodge_cooldown(&mut self.world);
        systems::switch_guns(&mut self.world, input, &mut self.sounds);
        if let Some(crate::artefacts::ArtefactKind::RevealMap) = systems::activate_artefact(
            &mut self.world,
            &mut self.physics,
            input,
            assets.textures.get(BULLET),
        ) {
            if let Some(rooms) = &mut self.rooms {
                rooms.reveal_all();
            }
        }
        let seed = crate::rng::cosmetic() * 1000.0;
        if systems::cast_lightning(&mut self.world, &assets.textures, seed) > 0 {
            self.camera.add_trauma(systems::LIGHTNING_SHAKE);
        }
        systems::drop_gun(
            &mut self.world,
            input,
            self.camera.position(),
            &assets.textures,
        );
        systems::glide(&mut self.world, dt);
        systems::open_chests(
            &mut self.world,
            &assets.textures,
            self.loot_rng.float(),
            |bound| self.loot_rng.below(bound),
            &mut self.sounds,
        );
        systems::tick_invulnerability(&mut self.world, dt);
        self.clock += dt;
        let berserk = self
            .world
            .query::<&crate::components::Berserk>()
            .iter()
            .count()
            > 0;
        let (particle, phase) = if berserk {
            (
                tex::ARTEFACTS_BERSERKPOT_PARTICLE,
                crate::artefacts::BERSERK_PHASE,
            )
        } else {
            (tex::GAZEWIND_PARTICLE, 0.0)
        };
        systems::spawn_gaze_wind(
            &mut self.world,
            &self.physics,
            assets.textures.get(particle),
            dt,
            self.clock + phase,
        );
        systems::shed_trails(
            &mut self.world,
            &systems::TrailTextures::load(&assets.textures),
            dt,
            self.clock,
        );
        systems::tick_dust(&mut self.world, self.clock, dt);
        systems::tick_atmosphere(
            &mut self.world,
            systems::AtmosphereTextures::load(&assets.textures),
            self.camera.position(),
            dt,
        );
        systems::tick_overheads(&mut self.world, dt);
        systems::tick_hanging(&mut self.world, self.clock, dt);
        systems::tick_drips(&mut self.world, &assets.textures, dt);
        systems::tick_lightning(&mut self.world, dt);
        systems::update_shadow(&mut self.world, dt);
        systems::update_status_bars(&mut self.world, dt);
        let infinite = self
            .world
            .query::<&InfiniteAmmo>()
            .with::<&crate::components::Player>()
            .iter()
            .next()
            .map(|_| self.clock);
        systems::update_weapon_hud(&mut self.world, &assets.textures, infinite);
        systems::fade_lights(&mut self.world);
        systems::flash_hits(&mut self.world, dt);
        systems::fade_afterimages(&mut self.world, dt);
        systems::hover_drops(&mut self.world, dt);
        systems::outline_drops(&mut self.world);
        systems::float_texts(&mut self.world, dt);
        let mut events = Vec::new();
        let hostile_dt = dt * systems::time_scale(&self.world);
        if let (Some(dungeon), Some(rooms)) = (&self.dungeon, &mut self.rooms) {
            let context = systems::RoomContext {
                dungeon,
                textures: &assets.textures,
                floor: self.floor,
            };
            events = systems::update_rooms(
                rooms,
                &context,
                &mut self.world,
                &mut self.physics,
                &mut self.map_rng,
                hostile_dt,
            );
        }
        self.handle_room_events(events, assets);
        systems::update_boss_hud(&mut self.world);
        systems::bloodlust_heals_on_kill(&mut self.world);
        systems::midas_kill_coins(&mut self.world, &assets.textures);
        systems::enemy_drops(
            &mut self.world,
            &mut self.kills_without_drop,
            &assets.textures,
            |bound| self.loot_rng.below(bound),
        );
        systems::remove_expired(&mut self.world);
        systems::collect_items(&mut self.world, &mut self.sounds);
        systems::age_items(&mut self.world, dt);
        systems::update_price_tags(&mut self.world);
        systems::tick_berserk(&mut self.world, dt);
        systems::apply_heal_percent(&mut self.world, &assets.textures, &mut self.sounds);
        systems::convert_bullets(&mut self.world, &mut self.physics, &assets.textures);
        systems::tick_artefact_effects(&mut self.world, dt);
        systems::spawn_bloodlust_drops(&mut self.world, &assets.textures, dt);
        systems::tick_blood_drops(&mut self.world, dt);
        systems::pulse_bloodlust(&mut self.world);
        systems::spawn_summons(&mut self.world, &assets.textures);
        systems::update_shield_aura(&mut self.world, &assets.textures);
        systems::update_artefact_hud(&mut self.world, &assets.textures);
        systems::update_slot_prices(&mut self.world);
        systems::tick_slot_machines(
            &mut self.world,
            &assets.textures,
            dt,
            &mut self.sounds,
            |bound| self.loot_rng.below(bound),
        );
        systems::flicker_flames(&mut self.world);
        systems::tick_speech(&mut self.world, dt);
        systems::fit_speech(&mut self.world, &assets.fonts);
        systems::fit_price_boxes(&mut self.world, &assets.fonts);
        if let (Some(dungeon), Some(rooms)) = (&self.dungeon, &self.rooms) {
            systems::update_minimaps(&mut self.world, rooms.entered(), rooms.current(), dt);
            if let Some(position) = player_tile(&self.world).filter(|_| self.training.is_none()) {
                if let Some((min, max)) = dungeon.camera_region(position.0, position.1) {
                    self.camera.glide_bounds(min, max, dt);
                }
            }
        }
        if let Some(target) = systems::camera_target(&self.world, input.mouse(), self.blend.0) {
            self.camera.follow(target, dt);
        }
        systems::follow_backdrop(&mut self.world, self.camera.view());
        systems::update_enemy_arrows(
            &mut self.world,
            self.camera.view(),
            assets.textures.get(tex::ENEMY_ARROW),
        );
        match systems::interact(&mut self.world, input) {
            Some(Interaction::NextFloor) => self.next_floor(assets),
            Some(Interaction::Spin) => systems::spin(&mut self.world, &mut self.sounds),
            Some(Interaction::Buy) => {
                systems::buy(&mut self.world, &assets.textures, &mut self.sounds);
            }
            Some(Interaction::PickUpArtefact) => {
                systems::pick_up_artefact(&mut self.world, &assets.textures, &mut self.sounds);
            }
            Some(Interaction::PickUpGun) => {
                systems::pick_up_gun(&mut self.world, &assets.textures, &mut self.sounds);
            }
            Some(Interaction::EnterPortal) => {
                if let Some(training) = &mut self.training {
                    training.enter_portal();
                }
            }
            _ => {}
        }
        self.feel_impacts();
        if let Some(training) = &mut self.training {
            let event = training.update(
                &mut self.world,
                &mut self.physics,
                &assets.textures,
                &mut self.sounds,
                dt,
            );
            if event == systems::TrainingEvent::Finished {
                return self.finish_training();
            }
        }
        Transition::None
    }

    fn fixed_update(&mut self, input: &Input, dt: f32, assets: &Assets) -> Transition {
        if self.pause.is_active() || self.death.is_some() {
            return Transition::None;
        }
        let hostile_dt = dt * systems::time_scale(&self.world);
        systems::cast_hit_roll(&mut self.world, self.combat_rng.float());
        systems::start_dodge(
            &mut self.world,
            input,
            input.mouse() + self.camera.position(),
            &mut self.sounds,
        );
        systems::steer_player_on(&mut self.world, input, &self.surfaces, dt);
        systems::tick_dodges(&mut self.world, dt);
        systems::tick_spawning(&mut self.world, &mut self.physics, hostile_dt);
        systems::dash(&mut self.world, &mut self.sounds, hostile_dt);
        systems::update_flow(
            &mut self.flow,
            &self.world,
            &self.physics,
            &self.navigation,
            dt,
        );
        systems::chase_player(&mut self.world, &self.flow, hostile_dt);
        systems::tick_golem(
            &mut self.world,
            &mut self.physics,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        systems::tick_rock_falls(
            &mut self.world,
            &mut self.physics,
            &self.navigation,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        systems::tick_icicle_bats(&mut self.world, &assets.textures, hostile_dt);
        systems::tick_ember_rats(&mut self.world, &mut self.sounds, self.clock);
        systems::tick_swoopers(&mut self.world, &mut self.sounds, hostile_dt);
        systems::tick_magma_slimes(&mut self.world, &assets.textures, &mut self.sounds, hostile_dt);
        systems::tick_worms(
            &mut self.world,
            &mut self.physics,
            &self.navigation,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        systems::tick_ashes(
            &mut self.world,
            &mut self.physics,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        systems::tick_hoppers(&mut self.world, hostile_dt);
        systems::tick_baits(&mut self.world, &assets.textures, &mut self.sounds, hostile_dt);
        systems::tick_mimic(
            &mut self.world,
            &mut self.physics,
            &self.navigation,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        systems::tick_hound(
            &mut self.world,
            &mut self.physics,
            &self.navigation,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        let boss_room = self.dungeon.as_ref().and_then(|dungeon| {
            dungeon
                .rooms
                .iter()
                .find(|room| room.kind == RoomKind::Boss)
                .map(|room| (room.origin.0, room.origin.1, room.columns, room.rows))
        });
        systems::tick_magma_spread(
            &mut self.world,
            &self.navigation,
            boss_room,
            &assets.textures,
            hostile_dt,
        );
        systems::tick_magma(&mut self.world, &mut self.sounds, hostile_dt);
        systems::tick_ogre(
            &mut self.world,
            &mut self.physics,
            &self.navigation,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        systems::tick_argus(
            &mut self.world,
            &mut self.physics,
            &self.navigation,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        systems::tick_laser(
            &mut self.world,
            &self.navigation,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        systems::tick_flames(
            &mut self.world,
            &self.navigation,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        systems::tick_orbiters(&mut self.world, &mut self.physics, hostile_dt);
        systems::pick_pet_targets(&mut self.world);
        systems::plan_pet_paths(&mut self.world, &mut self.physics, &self.navigation);
        systems::steer_pets(&mut self.world);
        systems::face_player(&mut self.world);
        systems::tick_smashes(
            &mut self.world,
            &mut self.physics,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        systems::tick_eye_shooters(
            &mut self.world,
            &mut self.physics,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        systems::steer_homing(&mut self.world, hostile_dt);
        systems::burn_fuses(&mut self.world, &mut self.physics, hostile_dt);
        systems::tick_hatchers(
            &mut self.world,
            &mut self.physics,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        systems::tick_web_spitters(
            &mut self.world,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        systems::tick_leapers(&mut self.world, &mut self.sounds, hostile_dt);
        systems::tick_armageddon(
            &mut self.world,
            &mut self.physics,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        systems::slow_bullets_in_webs(&mut self.world, &mut self.physics, dt);
        systems::tick_carts(&mut self.world, &mut self.physics, dt);
        systems::tick_waterworks(
            &mut self.world,
            &mut self.physics,
            &self.surfaces,
            &assets.textures,
            self.clock,
            dt,
        );
        systems::tick_pendulums(&mut self.world, &mut self.sounds, self.clock, hostile_dt);
        systems::shove_tables(
            &mut self.world,
            &mut self.physics,
            &assets.textures,
            &mut self.sounds,
            input.direction(),
        );
        systems::simulate(&mut self.world, &mut self.physics, dt);
        systems::watch_progress(&mut self.world, hostile_dt);
        systems::burn_out_rounds(&mut self.world, &mut self.physics, dt);
        systems::bullet_hits(
            &mut self.world,
            &mut self.physics,
            &assets.textures,
            &mut self.sounds,
        );
        systems::pet_bites(&mut self.world, dt);
        systems::tick_ghosts(&mut self.world, dt);
        systems::tick_turrets(&mut self.world, &mut self.physics, &assets.textures, dt);
        systems::trap_hits_enemies(&mut self.world, dt);
        systems::tick_arrow_traps(
            &mut self.world,
            &self.navigation,
            &assets.textures,
            &mut self.sounds,
            hostile_dt,
        );
        systems::break_objects(
            &mut self.world,
            &mut self.physics,
            &assets.textures,
            &mut self.sounds,
            |bound| self.loot_rng.below(bound),
        );
        systems::leave_poison_trails(&mut self.world, &assets.textures);
        systems::tick_poison(&mut self.world, dt, &mut self.sounds);
        systems::tick_frozen(&mut self.world, dt);
        systems::tick_chilled(&mut self.world, dt);
        systems::toughen_enemies(
            &mut self.world,
            self.rules.enemy_health,
            self.rules.enemy_damage,
        );
        systems::resolve_deaths(
            &mut self.world,
            &mut self.physics,
            &assets.textures,
            &mut self.sounds,
        );
        if systems::burn_on_lava(&mut self.world, &self.surfaces, &mut self.sounds) {
            self.hit = true;
        }
        systems::sync_liquids(&mut self.world, self.clock);
        systems::ripple_on_water(&mut self.world, &self.surfaces, &assets.textures, dt);
        systems::kick_up_surfaces(&mut self.world, &self.surfaces, dt);
        if systems::contact_damage(&mut self.world, &mut self.sounds) {
            self.hit = true;
        }
        systems::enemy_bullet_hits(&mut self.world, &mut self.physics);
        systems::batter_tables(&mut self.world, hostile_dt);
        systems::break_tables(
            &mut self.world,
            &mut self.physics,
            &assets.textures,
            &mut self.sounds,
            dt,
        );
        self.feel_impacts();
        Transition::None
    }
}

impl InGameScene {
    fn start_run(&mut self, seed: u64, assets: &Assets) {
        self.floor = 1;
        self.run_seed = seed;
        tracing::info!(seed, "a new run starts, set BANE_SEED to replay it");
        self.victory = false;
        self.leave_to_menu = false;
        self.death = None;
        self.rules = self
            .profile
            .as_ref()
            .map_or_else(RunRules::default, SharedProfile::rules);
        self.challenge = self.profile.as_ref().and_then(SharedProfile::challenge);
        self.build(assets);
        let perks = self.perks();
        systems::apply_run_perks(&mut self.world, perks);
    }

    fn perks(&self) -> Perks {
        // The training is the same for everyone: no upgrades bought yet count in it.
        if self.training_mode {
            return Perks::default();
        }
        let owned = self
            .profile
            .as_ref()
            .map_or_else(Perks::default, SharedProfile::perks);
        self.rules.shape(owned)
    }

    fn time_slow(&self) -> f32 {
        let slowed = self
            .world
            .query::<&crate::components::TimeSlow>()
            .with::<&crate::components::Player>()
            .iter()
            .next()
            .is_some();
        if slowed { 1.0 } else { 0.0 }
    }

    /// Shakes the screen for the hard hits of this step, read from the sounds they made.
    fn feel_impacts(&mut self) {
        for command in &self.sounds {
            if let AudioCommand::Effect(effect) = command {
                self.camera.add_trauma(impact_shake(*effect));
            }
        }
    }

    /// Whether the scene next starts the training crypt rather than a run.
    pub(crate) fn set_training(&mut self, on: bool) {
        self.training_mode = on;
    }

    fn pause_buttons(&self) -> &'static [(&'static str, UiAction)] {
        if self.training.is_some() {
            &prefabs::PAUSE_TRAINING
        } else {
            &prefabs::PAUSE_GAME
        }
    }

    /// The training is over, learnt or skipped: noted in the profile, on to the lobby.
    fn finish_training(&mut self) -> Transition {
        if let Some(profile) = &self.profile {
            profile.finish_training();
        }
        Transition::Switch(SceneId::Lobby)
    }

    pub(crate) fn attach_profile(&mut self, profile: SharedProfile) {
        self.profile = Some(profile);
    }

    fn start_death_screen(&mut self, assets: &Assets) {
        if self.death.is_some() {
            return;
        }
        let stats = systems::current_stats(&self.world);
        let summary = systems::summarize(stats, self.floor);
        tracing::info!(victory = self.victory, ?summary, "the run is over");
        let mut note = None;
        let mut attempt = 1;
        if let Some(profile) = &self.profile {
            profile.record_run(stats, &summary, self.victory);
            let outcome = profile.finish_run(&RunRecord {
                stats,
                floor: self.floor,
                victory: self.victory,
                challenge: self.challenge,
            });
            note = progress_note(&outcome);
            attempt = profile.runs();
        }
        let record = if self.victory {
            "Subject 41. The wound is open. Recorded.".to_owned()
        } else {
            format!("Subject 41. Attempt {attempt}. Recorded.")
        };
        systems::clear_artefact_effects(&mut self.world);
        prefabs::death_ui(&mut self.world, self.victory, record, &assets.textures);
        self.death = Some(systems::DeathScreen::new(summary).with_note(note));
        self.set_big_map(false);
    }

    fn build(&mut self, assets: &Assets) {
        self.map_rng = Rng::stream(self.run_seed, self.floor, Stream::Map);
        self.loot_rng = Rng::stream(self.run_seed, self.floor, Stream::Loot);
        self.combat_rng = Rng::stream(self.run_seed, self.floor, Stream::Combat);
        tracing::info!(floor = self.floor, "generating the dungeon");
        // `BANE_THEME=lava` (or any theme name) forces every floor's biome, for working on the game.
        let theme = std::env::var("BANE_THEME")
            .ok()
            .and_then(|name| Theme::parse(&name))
            .unwrap_or_else(|| Theme::for_floor(self.run_seed, self.floor));
        self.training = None;
        let generated = if self.training_mode {
            let (dungeon, lessons) = crate::dungeon::crypt();
            let mut training = systems::Training::new(lessons);
            // `BANE_LESSON=n` starts the training at its n-th lesson (from 1), for working on it.
            if let Some(lesson) = std::env::var("BANE_LESSON").ok().and_then(|n| n.trim().parse::<usize>().ok()) {
                training.start_at(lesson.saturating_sub(1));
            }
            self.training = Some(training);
            Ok(dungeon)
        } else {
            Dungeon::generate_for_run(&mut self.map_rng, theme, self.rules.shops)
        };
        let mut dungeon = match generated {
            Ok(dungeon) => dungeon,
            Err(error) => {
                tracing::error!(%error, "cannot generate the dungeon");
                return;
            }
        };
        // `BANE_SHOWCASE=1` hangs one of every kind across the biggest normal room and starts the
        // player under them, for looking at them all at once.
        let mut showcase_start = None;
        if std::env::var_os("BANE_SHOWCASE").is_some() {
            let grid = dungeon.grid.clone();
            let theme = dungeon.theme;
            if let Some(room) = dungeon
                .rooms
                .iter_mut()
                .filter(|room| room.kind == RoomKind::Normal)
                .max_by_key(|room| room.columns * room.rows)
            {
                room.hanging = crate::dungeon::showcase(&grid, room, theme);
                // By the middle one, as high up the floor as it goes towards where they hang
                // (at most five tiles), so the row and its chains are in view.
                // `BANE_SHOWCASE=<n>` stands by the n-th one instead.
                let pick = std::env::var("BANE_SHOWCASE")
                    .ok()
                    .and_then(|value| value.parse::<usize>().ok())
                    .unwrap_or(room.hanging.len() / 2);
                if let Some(middle) = room.hanging.get(pick) {
                    let (column, row) = middle.tile;
                    let up = (1..=5)
                        .take_while(|&up| row >= up && !grid.is_wall(column as isize, (row - up) as isize))
                        .last()
                        .unwrap_or(0);
                    let tile = Vec2::new(column as f32, (row - up) as f32);
                    showcase_start = Some((tile + Vec2::new(0.5, 0.5)) * TILE_SIZE);
                }
            }
        }
        let bounds = dungeon.grid.pixel_size();
        let textures = &assets.textures;
        let perks = self.perks();
        let [red, green, blue] = dungeon.theme.backdrop();
        prefabs::backdrop_colored(
            &mut self.world,
            glam::Vec4::new(
                f32::from(red) / 255.0,
                f32::from(green) / 255.0,
                f32::from(blue) / 255.0,
                1.0,
            ),
        );
        let solid_cells = prefabs::tilemap(
            &mut self.world,
            &dungeon.grid,
            assets.tileset_of(dungeon.theme.wide_tiles()),
            assets.textures.get(dungeon.theme.tileset_texture()),
        );
        self.physics.add_walls(&solid_cells, TILE_SIZE);
        let grid = &dungeon.grid;
        self.occluders = Some(Arc::new(Occluders::new(
            grid.columns(),
            grid.rows(),
            TILE_SIZE,
            |column, row| grid.is_wall(column as isize, row as isize),
        )));
        self.navigation = Navigation::new(&solid_cells);
        self.surfaces = dungeon.surfaces.clone();
        for (tile, surface) in dungeon.surfaces.iter() {
            prefabs::surface_tile(
                &mut self.world,
                &dungeon.grid,
                &dungeon.surfaces,
                tile,
                surface,
                textures,
            );
        }
        let mut shores = std::collections::BTreeSet::new();
        for (tile, _) in dungeon.surfaces.iter() {
            for dy in -1isize..=1 {
                for dx in -1isize..=1 {
                    let column = tile.0 as isize + dx;
                    let row = tile.1 as isize + dy;
                    if column >= 0 && row >= 0 {
                        shores.insert((column as usize, row as usize));
                    }
                }
            }
        }
        for tile in shores {
            if let Some((surface, row)) = dungeon.surfaces.shore_at(&dungeon.grid, tile) {
                prefabs::shore_tile(&mut self.world, surface, tile, row, textures);
            }
        }
        self.flow = FlowField::default();
        let mut start = self
            .training
            .as_ref()
            .and_then(systems::Training::start_position)
            .or_else(|| dungeon.start_position())
            .unwrap_or(bounds / 2.0);
        // `BANE_AT=boss`, `normal`, `treasure`, `shop`, `sacrifice`, `overhead`, `hanging`, `track`,
        // `table`, `rug`, `vestibule`, `water`, `bowl`, `statue`, `shackles` or `pendulum` starts the floor there (by the first one of those), for working on the
        // game.
        match std::env::var("BANE_AT").as_deref() {
            Ok("boss") => {
                if let Some(room) = dungeon
                    .rooms
                    .iter()
                    .find(|room| room.kind == RoomKind::Boss)
                {
                    start = room.pixel_center() + Vec2::new(0.0, 160.0);
                }
            }
            Ok(name @ ("normal" | "treasure" | "shop" | "sacrifice")) => {
                let kind = match name {
                    "treasure" => RoomKind::Treasure,
                    "shop" => RoomKind::Shop,
                    "sacrifice" => RoomKind::Sacrifice,
                    _ => RoomKind::Normal,
                };
                if let Some(spot) = dungeon
                    .rooms
                    .iter()
                    .filter(|room| room.kind == kind)
                    .max_by_key(|room| room.columns * room.rows)
                    .and_then(|room| open_spot_near_center(&dungeon, room))
                {
                    start = spot;
                }
            }
            Ok("hanging") => {
                if let Some(hanging) = dungeon.rooms.iter().flat_map(|room| &room.hanging).next() {
                    let tile = Vec2::new(hanging.tile.0 as f32, hanging.tile.1 as f32);
                    start = (tile + Vec2::new(0.5, 1.2)) * TILE_SIZE;
                }
            }
            Ok("rug") => {
                if let Some(rug) = dungeon.rooms.iter().find_map(|room| room.rug.as_ref()) {
                    let middle = Vec2::new(
                        rug.origin.0 as f32 + rug.columns as f32 / 2.0,
                        rug.origin.1 as f32 + rug.rows as f32 / 2.0,
                    );
                    start = middle * TILE_SIZE;
                }
            }
            Ok("bog") => {
                // Deep in a pool: a tile with the surface all round it, the lowest first.
                let deep = dungeon
                    .surfaces
                    .iter()
                    .filter(|&((column, row), _)| {
                        (column.saturating_sub(2)..=column + 2).all(|c| {
                            (row.saturating_sub(1)..=row + 1)
                                .all(|r| dungeon.surfaces.at((c, r)).is_some())
                        })
                    })
                    .map(|(tile, _)| tile)
                    .min();
                if let Some((column, row)) = deep {
                    start = Vec2::new(column as f32 + 0.5, row as f32) * TILE_SIZE;
                }
            }
            Ok("table") => {
                if let Some(spot) = dungeon.rooms.iter().flat_map(|room| &room.tables).next() {
                    // Beside its long side, on the open floor it would fall onto.
                    let (column, row) = (spot.tile.0 as f32, spot.tile.1 as f32);
                    let beside = if spot.upright {
                        Vec2::new(column + 1.9, row + 1.0)
                    } else {
                        Vec2::new(column + 1.0, row + 1.8)
                    };
                    start = beside * TILE_SIZE;
                }
            }
            Ok("track") => {
                if let Some(track) = dungeon.rooms.iter().find_map(|room| room.track.as_ref()) {
                    let (column, row) = track.path[track.cart];
                    // Beside the cart, off the rail: the rail has open floor all round it.
                    start = Vec2::new(column as f32 + 0.5, row as f32 + 1.5) * TILE_SIZE;
                }
            }
            Ok("water") => {
                // Beside the first footbridge, on the bank at its start; on a floor without
                // bridges, a little below the first pipe.
                let works = || dungeon.rooms.iter().map(|room| &room.waterworks);
                if let Some(bridge) = works().flat_map(|works| &works.bridges).next() {
                    let (column, row) = bridge.first;
                    let bank = if bridge.upright { (column, row - 1) } else { (column - 1, row) };
                    start = Vec2::new(bank.0 as f32 + 0.5, bank.1 as f32 + 0.5) * TILE_SIZE;
                } else if let Some(&(column, row)) = works().flat_map(|works| &works.pipes).next() {
                    start = Vec2::new(column as f32 + 1.5, row as f32 + 3.0) * TILE_SIZE;
                }
            }
            Ok(at @ ("bowl" | "statue" | "shackles")) => {
                let wanted = |kind: &DecorKind| match at {
                    "bowl" => *kind == DecorKind::FireBowl,
                    "shackles" => matches!(kind, DecorKind::Shackles { .. }),
                    _ => matches!(kind, DecorKind::StatuePiece { .. }),
                };
                if let Some(decor) =
                    dungeon.rooms.iter().flat_map(|room| &room.decor).find(|decor| wanted(&decor.kind))
                {
                    let (column, row) = decor.tile;
                    start = Vec2::new(column as f32 + 0.5, row as f32 + 2.5) * TILE_SIZE;
                }
            }
            Ok("pendulum") => {
                if let Some(span) = dungeon.rooms.iter().find_map(|room| room.pendulum) {
                    let middle = (span.first.0 + span.last.0 + 1) as f32 / 2.0;
                    start = Vec2::new(middle, span.first.1 as f32 + 3.0) * TILE_SIZE;
                }
            }
            Ok("vestibule") => {
                if let Some(vestibule) = dungeon.vestibules.first() {
                    let (column, row) = vestibule.center();
                    start = Vec2::new(column, row) * TILE_SIZE;
                }
            }
            Ok(at @ ("overhead" | "upright")) => {
                let upright = at == "upright";
                if let Some(span) = dungeon
                    .rooms
                    .iter()
                    .flat_map(|room| &room.overheads)
                    .find(|span| span.upright == upright)
                {
                    // Beside the middle of it, on the open floor.
                    let middle = Vec2::new(
                        (span.first.0 + span.last.0 + 1) as f32 / 2.0,
                        (span.first.1 + span.last.1 + 1) as f32 / 2.0,
                    );
                    let beside = if upright { Vec2::new(1.5, 0.0) } else { Vec2::new(0.0, 2.0) };
                    start = (middle + beside) * TILE_SIZE;
                }
            }
            _ => {}
        }
        if let Some(spot) = showcase_start {
            start = spot;
        }
        // `BANE_OFFSET=dx,dy` moves that start by whole tiles, for looking round a room.
        if let Some((dx, dy)) = std::env::var("BANE_OFFSET").ok().and_then(|offset| {
            let (dx, dy) = offset.split_once(',')?;
            Some((dx.trim().parse::<f32>().ok()?, dy.trim().parse::<f32>().ok()?))
        }) {
            start += Vec2::new(dx, dy) * TILE_SIZE;
        }
        let player = prefabs::player(&mut self.world, &mut self.physics, start, &assets.textures);
        // `BANE_GOD=1` keeps the player from being hurt, for watching enemies at work.
        if std::env::var_os("BANE_GOD").is_some() {
            let _ = self.world.insert_one(
                player,
                crate::components::Invulnerable { remaining: f32::MAX },
            );
        }
        // `BANE_COINS=n` starts the floor with that many coins, for trying out the shop.
        if let Some(coins) = std::env::var("BANE_COINS").ok().and_then(|n| n.trim().parse().ok()) {
            let wallets = self.world.query_mut::<&mut crate::components::Wallet>();
            for wallet in wallets.with::<&crate::components::Player>() {
                wallet.coins = coins;
            }
        }
        systems::apply_floor_perks(&mut self.world, perks);
        let weapon_mods = self
            .profile
            .as_ref()
            .map_or_else(WeaponMods::default, SharedProfile::weapon_mods);
        systems::apply_weapon_mods(&mut self.world, weapon_mods);
        if perks.pet {
            prefabs::pet(&mut self.world, &mut self.physics, start, &assets.textures);
        }
        for room in &dungeon.rooms {
            if let Some(furnishing) = &room.furnishing {
                prefabs::furnishing(
                    &mut self.world,
                    &mut self.physics,
                    furnishing,
                    perks.gamble_machine,
                    textures,
                );
            }
            for decor in &room.decor {
                prefabs::decor(&mut self.world, &mut self.physics, *decor, textures);
            }
        }
        for vestibule in &dungeon.vestibules {
            for decor in &vestibule.decor {
                prefabs::decor(&mut self.world, &mut self.physics, *decor, textures);
            }
        }
        systems::spawn_overheads(&mut self.world, &dungeon, textures);
        systems::spawn_hanging(&mut self.world, &dungeon, textures);
        systems::spawn_furniture(&mut self.world, &mut self.physics, &dungeon, textures);
        systems::spawn_tracks(&mut self.world, &mut self.physics, &dungeon, textures);
        systems::spawn_waterworks(&mut self.world, &mut self.physics, &dungeon, textures);
        systems::spawn_prison(&mut self.world, &dungeon, textures);
        systems::spawn_atmosphere(
            &mut self.world,
            &dungeon,
            self.run_seed ^ u64::from(self.floor).rotate_left(32),
        );
        systems::stock_rooms(
            &mut self.world,
            &dungeon,
            self.floor,
            perks.discount,
            textures,
            |bound| self.loot_rng.below(bound),
        );
        self.keeper = None;
        if let Some(keeper) = dungeon
            .rooms
            .iter()
            .filter_map(|room| room.furnishing.as_ref())
            .flat_map(|furnishing| furnishing.fixtures.iter())
            .find(|fixture| fixture.kind == crate::dungeon::FixtureKind::Shopkeeper)
        {
            let head = keeper.top_left
                + Vec2::new(crate::props::Prop::Shopkeeper.spec().size().x / 2.0, 0.0);
            self.keeper = Some(prefabs::speech_anchor(head));
        }
        // One bubble for whoever speaks on the floor: the merchant, or an altar.
        prefabs::minimal_speech(&mut self.world, Vec2::ZERO);
        self.rooms = Some(systems::RoomRuntime::new(&dungeon));
        // The training needs no map: its rooms run one after another.
        if self.training.is_none() {
            spawn_minimaps(&mut self.world, &dungeon, textures);
        }
        self.camera = Camera::new(bounds);
        if self.training.is_some() {
            // Free over the whole crypt, so it follows the player round the bends of its corridors.
            self.camera.set_bounds(Vec2::ZERO, bounds);
        } else if let Some(room) = dungeon.rooms.first() {
            if let Some((min, max)) = dungeon.camera_region(room.origin.0 + 1, room.origin.1 + 1) {
                self.camera.set_bounds(min, max);
            }
        }
        systems::immobilize_traps(&mut self.world, perks, |bound| self.loot_rng.below(bound));
        self.dungeon = Some(dungeon);
        prefabs::status_bars(&mut self.world, textures);
        prefabs::coin_hud(
            &mut self.world,
            textures,
            tex::BANECOIN,
            Vec4::new(1.0, 216.0 / 255.0, 0.0, 1.0),
        );
        prefabs::weapon_hud(&mut self.world, textures);
        prefabs::artefact_hud(&mut self.world, textures);
        if let Some(training) = &mut self.training {
            training.begin(&mut self.world, &mut self.physics, textures);
        }
        if let Some(target) = systems::camera_target(&self.world, TARGET / 2.0, 1.0) {
            self.camera.snap_to(target);
        }
        systems::follow_backdrop(&mut self.world, self.camera.view());
    }

    fn next_floor(&mut self, assets: &Assets) {
        systems::close_stats_floor(&mut self.world);
        let carried = systems::carried_stats(&self.world);
        self.world.clear();
        self.physics = Physics::new();
        self.dungeon = None;
        self.rooms = None;
        self.big_map = false;
        self.floor += 1;
        tracing::info!(floor = self.floor, "entering the next floor");
        self.build(assets);
        systems::restore_stats(&mut self.world, carried);
        self.sounds.push(AudioCommand::Music(Soundtrack::InGame));
    }

    fn handle_room_events(&mut self, events: Vec<systems::RoomEvent>, assets: &Assets) {
        for event in events {
            match event {
                systems::RoomEvent::RoomCleared => {
                    systems::regenerate_armor(&mut self.world);
                }
                systems::RoomEvent::WaveStarted { number, total } => {
                    if total > 1 {
                        prefabs::wave_banner(&mut self.world, number, total);
                    }
                }
                systems::RoomEvent::AmbushCleared { center } => {
                    let spot = systems::chest_spot(&self.navigation, center);
                    prefabs::chest(
                        &mut self.world,
                        crate::components::ChestKind::Weapon,
                        spot,
                        &assets.textures,
                    );
                }
                systems::RoomEvent::ShopEntered => {
                    let visit = self.profile.as_ref().map_or(1, SharedProfile::visit_shop);
                    let line = systems::merchant_line(visit, self.loot_rng.below(64));
                    match self.keeper {
                        Some(anchor) => systems::say_at(&mut self.world, anchor, line, systems::SPEECH_SECONDS + 1.0),
                        None => systems::say(&mut self.world, line),
                    }
                }
                systems::RoomEvent::SacrificeEntered { room } => {
                    // The altar speaks over the angel at the room's head.
                    let statue = self.dungeon.as_ref().and_then(|dungeon| {
                        dungeon.rooms.get(room)?.furnishing.as_ref()?.fixtures.iter().find_map(|fixture| {
                            (fixture.kind == crate::dungeon::FixtureKind::AngelStatue)
                                .then(|| fixture.top_left + Vec2::new(fixture.size.x / 2.0, 0.0))
                        })
                    });
                    if let Some(head) = statue {
                        let line = systems::ALTAR[self.loot_rng.below(systems::ALTAR.len())];
                        systems::say_at(&mut self.world, prefabs::speech_anchor(head), line, systems::ALTAR_SECONDS);
                    }
                }
                systems::RoomEvent::BossEntered => {
                    self.sounds.push(AudioCommand::Music(Soundtrack::Boss));
                }
                systems::RoomEvent::BossDefeated { center } => {
                    self.camera.add_trauma(BOSS_DEFEATED_SHAKE);
                    self.sounds.push(AudioCommand::Music(Soundtrack::InGame));
                    let textures = &assets.textures;
                    let chest_top_left = center - Vec2::new(32.0, 3.0 * TILE_SIZE);
                    prefabs::chest(
                        &mut self.world,
                        crate::components::ChestKind::Artefact,
                        chest_top_left,
                        textures,
                    );
                    if self.floor >= crate::enemies::FINAL_FLOOR {
                        tracing::info!("the last boss is defeated");
                        systems::close_stats_floor(&mut self.world);
                        self.victory = true;
                    } else {
                        let ladder_top_left = center - Vec2::new(48.0, 0.0);
                        let theme = self
                            .dungeon
                            .as_ref()
                            .map_or(Theme::Stone, |dungeon| dungeon.theme);
                        prefabs::ladder(&mut self.world, ladder_top_left, theme, textures);
                    }
                }
            }
        }
    }

    fn set_big_map(&mut self, open: bool) {
        if open && !self.big_map {
            self.sounds.push(AudioCommand::Effect(SoundEffect::ShowMap));
            systems::reset_big_map_pan(&mut self.world);
        }
        self.big_map = open;
        self.drag_from = None;
        let big: Vec<hecs::Entity> = self
            .world
            .query::<hecs::Entity>()
            .with::<&BigMap>()
            .iter()
            .collect();
        for entity in big {
            if open {
                let _ = self.world.remove_one::<Hidden>(entity);
            } else {
                let _ = self.world.insert_one(entity, Hidden);
            }
        }
    }
}

/// The open floor tile closest to the middle of a room, with no wall around it.
fn open_spot_near_center(dungeon: &Dungeon, room: &crate::dungeon::DungeonRoom) -> Option<Vec2> {
    let center = room.pixel_center();
    (room.origin.1..room.origin.1 + room.rows)
        .flat_map(|row| {
            (room.origin.0..room.origin.0 + room.columns).map(move |column| (column, row))
        })
        .filter(|&(column, row)| {
            (-1isize..=1).all(|dy| {
                (-1isize..=1).all(|dx| {
                    !dungeon
                        .grid
                        .is_wall(column as isize + dx, row as isize + dy)
                })
            })
        })
        .map(|(column, row)| (Vec2::new(column as f32, row as f32) + 0.5) * TILE_SIZE)
        .min_by(|a, b| a.distance(center).total_cmp(&b.distance(center)))
}

fn player_tile(world: &World) -> Option<(usize, usize)> {
    world
        .query::<&crate::components::Position>()
        .with::<&crate::components::Player>()
        .iter()
        .next()
        .map(|position| {
            (
                (position.0.x / TILE_SIZE) as usize,
                (position.0.y / TILE_SIZE) as usize,
            )
        })
}

fn spawn_minimaps(world: &mut World, dungeon: &Dungeon, textures: &crate::assets::Textures) {
    let rooms = systems::rooms_for(dungeon.rooms.iter().map(|room| {
        (
            room.cell,
            room.kind,
            room.doors.iter().map(|door| door.direction).collect(),
        )
    }));
    let start = rooms.first().map_or(Vec2::ZERO, |room| {
        Vec2::new(room.cell.0 as f32, room.cell.1 as f32)
    });
    for size in [MapSize::Small, MapSize::Big] {
        let minimap = Minimap {
            rooms: rooms.clone(),
            current: 0,
            shown: start,
            pan: Vec2::ZERO,
            glow: 0.0,
            icons: textures.get(tex::MINIMAP_ICONS),
            size,
        };
        let (center, frame_size, layer) = match size {
            MapSize::Small => (Vec2::new(865.5, 94.5), Vec2::splat(125.0), 4),
            MapSize::Big => (Vec2::new(480.0, 270.0), Vec2::splat(400.0), 6),
        };
        let frame = world.spawn((
            crate::components::Position(center),
            crate::components::Size(frame_size),
            crate::components::Color(Vec4::ONE),
            crate::components::Sprite::whole(textures.get(tex::MINIMAP_LAYOUT)),
            crate::components::Layer(layer),
            crate::components::ScreenSpace,
        ));
        let map = world.spawn((minimap, crate::components::ScreenSpace));
        if size == MapSize::Big {
            for entity in [frame, map] {
                let _ = world.insert(entity, (BigMap, Hidden));
            }
        }
    }
}

fn progress_note(outcome: &RunOutcome) -> Option<String> {
    let mut parts = Vec::new();
    if let Some((title, talents)) = outcome.challenge {
        parts.push(format!("{title} complete! +{talents} talents"));
    }
    match outcome.missions_done.len() {
        0 => {}
        1 => parts.push("1 mission ready on the board".to_owned()),
        count => parts.push(format!("{count} missions ready on the board")),
    }
    (!parts.is_empty()).then(|| parts.join("   "))
}

#[cfg(test)]
mod tests {
    use crate::components::{
        Currency, DeathBurst, Expiring, Health, Hits, Player, PoisonPool, Position, PriceTag, Shop,
        Trap,
    };
    use crate::enemies::EnemyKind;

    use super::*;

    #[test]
    fn explosions_and_hurt_shake_the_view_but_not_the_aim() {
        let mut scene = InGameScene {
            camera: Camera::new(Vec2::new(5000.0, 5000.0)),
            ..InGameScene::default()
        };
        scene.camera.snap_to(Vec2::new(500.0, 500.0));
        scene
            .sounds
            .push(AudioCommand::Effect(SoundEffect::Explosion));

        scene.feel_impacts();
        scene.camera.update_shake(0.05);

        assert_ne!(scene.camera(), scene.camera.position());
        assert_eq!(scene.camera.position(), Vec2::new(500.0, 500.0));
        assert_eq!(impact_shake(SoundEffect::Coin), 0.0);
    }

    #[test]
    fn an_enemy_killed_by_poison_still_bursts_before_it_is_reaped() {
        let mut assets = Assets::new().expect("the embedded tileset is valid");
        assets.mark_loaded();
        let mut scene = InGameScene::default();
        scene.enter(&assets);
        let spot = scene
            .world
            .query::<&Position>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|position| position.0)
            .expect("the dungeon has a player");
        let slime = prefabs::ready_enemy(
            &mut scene.world,
            &mut scene.physics,
            EnemyKind::RedSlime,
            spot,
        );
        if let Ok(mut health) = scene.world.get::<&mut Health>(slime) {
            health.current = 1;
        }
        let _ = scene.world.insert_one(slime, Hits(1));
        let pool = prefabs::poison_pool(&mut scene.world, spot, &assets.textures);
        if let Ok(mut pool) = scene.world.get::<&mut PoisonPool>(pool) {
            pool.tick = 0.0;
        }

        scene.fixed_update(&Input::default(), 1.0 / 60.0, &assets);

        assert!(scene.world.get::<&Expiring>(slime).is_ok());
        assert!(scene.world.get::<&DeathBurst>(slime).is_err());
    }

    fn fingerprint(seed: u64, assets: &Assets) -> Vec<String> {
        let mut scene = InGameScene::default();
        scene.start_run(seed, assets);
        fingerprint_of(&mut scene)
    }

    fn fingerprint_of(scene: &mut InGameScene) -> Vec<String> {
        let mut lines: Vec<String> = scene
            .world
            .query::<(&Position, Option<&Shop>, Option<&PriceTag>, Option<&Trap>)>()
            .iter()
            .filter(|(_, shop, tag, trap)| shop.is_some() || tag.is_some() || trap.is_some())
            .map(|(position, shop, _, trap)| {
                format!(
                    "{:?} {:?} {}",
                    position.0,
                    shop.map(|s| s.price),
                    trap.is_some()
                )
            })
            .collect();
        lines.sort();
        lines.push(format!(
            "{:?}",
            scene.dungeon.as_ref().map(|d| d.grid.pixel_size())
        ));
        lines
    }

    #[test]
    fn the_same_seed_builds_the_same_floor_with_the_same_shops_and_traps() {
        let assets = Assets::new().expect("the embedded tileset is valid");
        let first = fingerprint(42, &assets);
        assert!(first.len() > 1);
        assert_eq!(first, fingerprint(42, &assets));
        assert_ne!(first, fingerprint(7, &assets));
    }

    fn player_max_health(scene: &InGameScene) -> Option<i32> {
        scene
            .world
            .query::<&Health>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|health| health.max)
    }

    fn scene_with_challenge(challenge: ChallengeId) -> InGameScene {
        let profile = SharedProfile::in_memory();
        profile.toggle_challenge(challenge);
        let mut scene = InGameScene::default();
        scene.attach_profile(profile);
        scene
    }

    #[test]
    fn a_pistol_only_run_starts_with_the_pistol_in_hand() {
        let assets = Assets::new().expect("the embedded tileset is valid");
        let mut scene = scene_with_challenge(ChallengeId::PistolOnly);
        scene.start_run(7, &assets);
        let held = scene
            .world
            .query::<&Loadout>()
            .with::<&crate::components::Player>()
            .iter()
            .find_map(|loadout| loadout.current().map(|weapon| weapon.kind));
        assert_eq!(held, Some(crate::guns::GunKind::Pistol));
    }

    #[test]
    fn a_challenge_run_starts_with_its_rules_and_neutral_runs_match_today() {
        let assets = Assets::new().expect("the embedded tileset is valid");
        let mut plain = InGameScene::default();
        plain.attach_profile(SharedProfile::in_memory());
        plain.start_run(7, &assets);
        assert_eq!(fingerprint_of(&mut plain), fingerprint(7, &assets));

        let mut scene = scene_with_challenge(ChallengeId::GlassCannon);
        scene.start_run(7, &assets);
        assert_eq!(player_max_health(&scene), Some(1));
        assert_eq!(scene.challenge, Some(ChallengeId::GlassCannon));
    }

    #[test]
    fn glass_cannon_stays_at_one_heart_on_the_next_floor() {
        let assets = Assets::new().expect("the embedded tileset is valid");
        let mut scene = scene_with_challenge(ChallengeId::GlassCannon);
        scene.start_run(7, &assets);
        scene.next_floor(&assets);
        assert_eq!(player_max_health(&scene), Some(1));
    }

    #[test]
    fn merchants_strike_floors_have_no_shop() {
        let assets = Assets::new().expect("the embedded tileset is valid");
        let mut scene = scene_with_challenge(ChallengeId::MerchantsStrike);
        scene.start_run(7, &assets);
        let coin_offers = scene
            .world
            .query::<&Shop>()
            .iter()
            .filter(|shop| shop.currency == Currency::Coins)
            .count();
        assert_eq!(coin_offers, 0);
    }

    #[test]
    fn a_run_is_recorded_once_however_often_the_death_screen_starts() {
        let assets = Assets::new().expect("the embedded tileset is valid");
        let profile = SharedProfile::in_memory();
        let mut scene = InGameScene::default();
        scene.attach_profile(profile.clone());
        scene.start_run(7, &assets);

        scene.start_death_screen(&assets);
        let (runs, board) = (profile.snapshot().runs, profile.board());
        scene.start_death_screen(&assets);

        assert_eq!(runs, 1);
        assert_eq!(profile.snapshot().runs, 1);
        assert_eq!(profile.board(), board);
    }

    #[test]
    fn the_progress_note_names_the_challenge_and_the_missions() {
        assert_eq!(progress_note(&RunOutcome::default()), None);
        let outcome = RunOutcome {
            missions_done: vec!["a".to_owned(), "b".to_owned()],
            challenge: Some(("Glass Cannon", 50)),
        };
        assert_eq!(
            progress_note(&outcome).as_deref(),
            Some("Glass Cannon complete! +50 talents   2 missions ready on the board")
        );
    }

    fn paused_game() -> (InGameScene, Assets) {
        let mut assets = Assets::new().expect("the embedded tileset is valid");
        assets.mark_loaded();
        let mut scene = InGameScene::default();
        scene.enter(&assets);
        let mut input = Input::default();
        input.press(KeyCode::Escape);
        scene.update(&input, 0.0, &assets, &mut Settings::in_memory());
        assert!(scene.pause.is_active());
        (scene, assets)
    }

    fn press_button(scene: &mut InGameScene, assets: &Assets, label: &str) -> Transition {
        let at = scene
            .world
            .query::<(&Position, &crate::components::Text)>()
            .with::<&crate::components::TextButton>()
            .iter()
            .find(|(_, text)| text.content == label)
            .map(|(position, _)| position.0)
            .expect("the button is on the pause menu");
        let mut input = Input::default();
        input.set_mouse(at);
        input.handle_click(
            winit::event::MouseButton::Left,
            winit::event::ElementState::Pressed,
        );
        scene.update(&input, 0.0, assets, &mut Settings::in_memory())
    }

    #[test]
    fn the_pause_menu_offers_resume_settings_main_menu_die_and_exit_game() {
        let (scene, _assets) = paused_game();

        let mut labels: Vec<String> = scene
            .world
            .query::<&crate::components::Text>()
            .with::<&crate::components::TextButton>()
            .iter()
            .map(|text| text.content.clone())
            .collect();
        labels.sort();

        assert_eq!(
            labels,
            vec!["Die", "Exit Game", "Main Menu", "Resume", "Settings"]
        );
    }

    #[test]
    fn main_menu_ends_the_run_like_a_death_and_then_goes_to_the_main_menu() {
        let (mut scene, assets) = paused_game();
        let profile = SharedProfile::in_memory();
        scene.profile = Some(profile.clone());
        let runs = profile.snapshot().runs;

        let transition = press_button(&mut scene, &assets, "Main Menu");

        assert!(matches!(transition, Transition::None));
        assert!(scene.death.is_some(), "the run summary is shown first");
        assert!(!scene.pause.is_active());
        assert_eq!(profile.snapshot().runs, runs + 1, "the run is recorded");
        let mut until_done = Transition::None;
        for _ in 0..600 {
            let mut input = Input::default();
            input.press(KeyCode::Space);
            until_done = scene.update(&input, 0.1, &assets, &mut Settings::in_memory());
            if until_done != Transition::None {
                break;
            }
        }
        assert_eq!(until_done, Transition::Switch(SceneId::Menu));
    }

    #[test]
    fn dying_from_the_pause_menu_still_returns_to_the_lobby() {
        let (mut scene, assets) = paused_game();
        press_button(&mut scene, &assets, "Die");
        let mut until_done = Transition::None;
        for _ in 0..600 {
            let mut input = Input::default();
            input.press(KeyCode::Space);
            until_done = scene.update(&input, 0.1, &assets, &mut Settings::in_memory());
            if until_done != Transition::None {
                break;
            }
        }
        assert_eq!(until_done, Transition::Switch(SceneId::Lobby));
    }

    #[test]
    fn die_ends_the_run_and_shows_the_death_screen() {
        let (mut scene, assets) = paused_game();
        assert!(scene.death.is_none());

        let transition = press_button(&mut scene, &assets, "Die");

        assert!(matches!(transition, Transition::None));
        assert!(scene.death.is_some());
        assert!(!scene.pause.is_active());
    }

    #[test]
    fn exit_game_quits_the_whole_game() {
        let (mut scene, assets) = paused_game();

        let transition = press_button(&mut scene, &assets, "Exit Game");

        assert!(matches!(transition, Transition::Quit));
    }

    #[test]
    fn resume_closes_the_pause_menu_and_keeps_playing() {
        let (mut scene, assets) = paused_game();

        let transition = press_button(&mut scene, &assets, "Resume");

        assert!(matches!(transition, Transition::None));
        assert!(!scene.pause.is_active());
        assert!(scene.death.is_none());
    }

    #[test]
    fn the_death_screen_ends_the_artefact_effects() {
        let (mut scene, assets) = paused_game();
        let player = scene
            .world
            .query::<hecs::Entity>()
            .with::<&crate::components::Player>()
            .iter()
            .next()
            .expect("player");
        let _ = scene
            .world
            .insert_one(player, crate::components::TimeSlow { remaining: 5.0 });

        press_button(&mut scene, &assets, "Die");

        assert!(scene.death.is_some());
        assert!(scene
            .world
            .get::<&crate::components::TimeSlow>(player)
            .is_err());
    }
}
