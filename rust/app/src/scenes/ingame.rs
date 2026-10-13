use std::sync::Arc;

use glam::{Vec2, Vec4};
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::tex;
use crate::assets::{Assets, BULLET, TILESET};
use crate::audio::{AudioCommand, SoundEffect, Soundtrack};
use crate::camera::Camera;
use crate::components::{BigMap, Blend, Hidden, Interaction, Loadout, MapSize, Minimap, UiAction};
use crate::controls::Action;
use crate::dungeon::Dungeon;
use crate::flow::FlowField;
use crate::input::Input;
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::postfx::SceneFx;
use crate::prefabs;
use crate::profile::SharedProfile;
use crate::render::Occluders;
use crate::rng::{Rng, Stream};
use crate::settings::Settings;
use crate::systems;
use crate::tilemap::TILE_SIZE;
use crate::upgrades::Perks;
use crate::viewport::TARGET;

use super::{Cursor, Pause, Scene, SceneId, Transition};

const SLOT_MACHINE_COINS: i32 = 100;
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
    flow: FlowField,
    victory: bool,
    death: Option<systems::DeathScreen>,
    profile: Option<SharedProfile>,
    kills_without_drop: u32,
    drag_from: Option<Vec2>,
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
        std::mem::take(&mut self.sounds)
    }

    fn fx(&self) -> SceneFx {
        SceneFx {
            paused: self.pause.is_active(),
            low_health: systems::player_is_low_on_health(&self.world),
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
        _settings: &mut Settings,
    ) -> Transition {
        self.camera.update_shake(dt);
        if self.death.is_none() && (self.victory || systems::player_is_dead(&self.world)) {
            self.start_death_screen(assets);
        }
        if let Some(screen) = &mut self.death {
            screen.advance(dt, input.any_key());
            systems::update_death_ui(&mut self.world, screen);
            if screen.finished() {
                return Transition::Switch(SceneId::Lobby);
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
                self.pause
                    .toggle(&mut self.world, assets, &prefabs::PAUSE_GAME);
            }
        }
        if self.pause.is_active() {
            let action = self.pause.update(&mut self.world, input);
            if action.is_some() {
                self.sounds.push(AudioCommand::Effect(SoundEffect::Click));
            }
            return match action {
                Some(UiAction::Resume) => {
                    self.pause
                        .toggle(&mut self.world, assets, &prefabs::PAUSE_GAME);
                    Transition::None
                }
                Some(UiAction::Die) => {
                    self.pause
                        .toggle(&mut self.world, assets, &prefabs::PAUSE_GAME);
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
                    bullet_texture: assets.textures.get(BULLET),
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
        systems::tick_dust(&mut self.world, self.clock, dt);
        systems::update_shadow(&mut self.world, dt);
        systems::update_status_bars(&mut self.world, dt);
        systems::update_weapon_hud(&mut self.world, &assets.textures);
        systems::fade_lights(&mut self.world);
        systems::flash_hits(&mut self.world, dt);
        systems::fade_afterimages(&mut self.world, dt);
        systems::hover_drops(&mut self.world, dt);
        systems::outline_drops(&mut self.world);
        systems::float_texts(&mut self.world, dt);
        let mut events = Vec::new();
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
            );
        }
        self.handle_room_events(events, assets);
        systems::update_boss_hud(&mut self.world);
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
        if let (Some(dungeon), Some(rooms)) = (&self.dungeon, &self.rooms) {
            systems::update_minimaps(&mut self.world, rooms.entered(), rooms.current(), dt);
            if let Some(position) = player_tile(&self.world) {
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
            _ => {}
        }
        self.feel_impacts();
        Transition::None
    }

    fn fixed_update(&mut self, input: &Input, dt: f32, assets: &Assets) -> Transition {
        if self.pause.is_active() || self.death.is_some() {
            return Transition::None;
        }
        systems::start_dodge(
            &mut self.world,
            input,
            input.mouse() + self.camera.position(),
        );
        systems::steer_player(&mut self.world, input, dt);
        systems::tick_dodges(&mut self.world, dt);
        systems::tick_spawning(&mut self.world, &mut self.physics, dt);
        systems::dash(&mut self.world, dt);
        systems::update_flow(
            &mut self.flow,
            &self.world,
            &self.physics,
            &self.navigation,
            dt,
        );
        systems::chase_player(&mut self.world, &self.flow);
        systems::separate_chasers(&mut self.world);
        systems::pick_pet_targets(&mut self.world);
        systems::plan_pet_paths(&mut self.world, &mut self.physics, &self.navigation);
        systems::steer_pets(&mut self.world);
        systems::face_player(&mut self.world);
        systems::tick_smashes(&mut self.world, &mut self.physics, &assets.textures, dt);
        systems::tick_eye_shooters(&mut self.world, &mut self.physics, &assets.textures, dt);
        systems::steer_homing(&mut self.world, dt);
        systems::burn_fuses(&mut self.world, &mut self.physics, dt);
        systems::tick_hatchers(&mut self.world, &mut self.physics, &assets.textures, dt);
        systems::tick_web_spitters(&mut self.world, &assets.textures, dt);
        systems::tick_armageddon(&mut self.world, &mut self.physics, &assets.textures, dt);
        systems::simulate(&mut self.world, &mut self.physics, dt);
        systems::watch_progress(&mut self.world, dt);
        systems::bullet_hits(&mut self.world, &mut self.physics, &mut self.sounds);
        systems::pet_bites(&mut self.world, dt);
        systems::trap_hits_enemies(&mut self.world, dt);
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
        systems::resolve_deaths(&mut self.world, &mut self.physics, &assets.textures);
        if systems::contact_damage(&mut self.world, &mut self.sounds) {
            self.hit = true;
        }
        systems::enemy_bullet_hits(&mut self.world, &mut self.physics);
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
        self.death = None;
        self.build(assets);
        let perks = self
            .profile
            .as_ref()
            .map_or_else(Perks::default, SharedProfile::perks);
        systems::apply_run_perks(&mut self.world, perks);
    }

    /// Shakes the screen for the hard hits of this step, read from the sounds they made.
    fn feel_impacts(&mut self) {
        for command in &self.sounds {
            if let AudioCommand::Effect(effect) = command {
                self.camera.add_trauma(impact_shake(*effect));
            }
        }
    }

    pub(crate) fn attach_profile(&mut self, profile: SharedProfile) {
        self.profile = Some(profile);
    }

    fn start_death_screen(&mut self, assets: &Assets) {
        let stats = systems::current_stats(&self.world);
        let summary = systems::summarize(stats, self.floor);
        tracing::info!(victory = self.victory, ?summary, "the run is over");
        if let Some(profile) = &self.profile {
            profile.record_run(stats, &summary, self.victory);
        }
        prefabs::death_ui(&mut self.world, self.victory, &assets.textures);
        self.death = Some(systems::DeathScreen::new(summary));
        self.set_big_map(false);
    }

    fn build(&mut self, assets: &Assets) {
        self.map_rng = Rng::stream(self.run_seed, self.floor, Stream::Map);
        self.loot_rng = Rng::stream(self.run_seed, self.floor, Stream::Loot);
        self.combat_rng = Rng::stream(self.run_seed, self.floor, Stream::Combat);
        tracing::info!(floor = self.floor, "generating the dungeon");
        let dungeon = match Dungeon::generate(&mut self.map_rng) {
            Ok(dungeon) => dungeon,
            Err(error) => {
                tracing::error!(%error, "cannot generate the dungeon");
                return;
            }
        };
        let bounds = dungeon.grid.pixel_size();
        let textures = &assets.textures;
        let perks = self
            .profile
            .as_ref()
            .map_or_else(Perks::default, SharedProfile::perks);
        prefabs::backdrop(&mut self.world);
        let solid_cells = prefabs::tilemap(
            &mut self.world,
            &dungeon.grid,
            &assets.tileset,
            assets.textures.get(TILESET),
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
        self.flow = FlowField::default();
        let start = dungeon.start_position().unwrap_or(bounds / 2.0);
        prefabs::player(&mut self.world, &mut self.physics, start, &assets.textures);
        systems::apply_floor_perks(&mut self.world, perks);
        if perks.pet {
            prefabs::pet(&mut self.world, &mut self.physics, start, &assets.textures);
        }
        for room in &dungeon.rooms {
            if let Some(furnishing) = &room.furnishing {
                prefabs::furnishing(&mut self.world, &mut self.physics, furnishing, textures);
            }
            for decor in &room.decor {
                prefabs::decor(&mut self.world, &mut self.physics, *decor, textures);
            }
        }
        systems::stock_rooms(
            &mut self.world,
            &dungeon,
            self.floor,
            perks.discount,
            textures,
            |bound| self.loot_rng.below(bound),
        );
        if let Some(keeper) = dungeon
            .rooms
            .iter()
            .filter_map(|room| room.furnishing.as_ref())
            .flat_map(|furnishing| furnishing.fixtures.iter())
            .find(|fixture| fixture.kind == crate::dungeon::FixtureKind::Shopkeeper)
        {
            prefabs::shop_speech(&mut self.world, keeper.top_left, textures);
            if perks.gamble_machine {
                for wallet in self.world.query_mut::<&mut crate::components::Wallet>() {
                    wallet.coins += SLOT_MACHINE_COINS;
                }
            }
        }
        self.rooms = Some(systems::RoomRuntime::new(&dungeon));
        spawn_minimaps(&mut self.world, &dungeon, textures);
        self.camera = Camera::new(bounds);
        if let Some(room) = dungeon.rooms.first() {
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
        if let Some(target) = systems::camera_target(&self.world, TARGET / 2.0, 1.0) {
            self.camera.snap_to(target);
        }
        systems::follow_backdrop(&mut self.world, self.camera.view());
    }

    fn next_floor(&mut self, assets: &Assets) {
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
                systems::RoomEvent::ShopEntered => {
                    systems::say(&mut self.world, "Welcome to my shop!");
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
                        self.victory = true;
                    } else {
                        let ladder_top_left = center - Vec2::new(48.0, 0.0);
                        prefabs::ladder(&mut self.world, ladder_top_left, textures);
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

#[cfg(test)]
mod tests {
    use crate::components::{
        DeathBurst, Expiring, Health, Hits, Player, PoisonPool, Position, PriceTag, Shop, Trap,
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
    fn the_pause_menu_offers_resume_die_and_exit_game() {
        let (scene, _assets) = paused_game();

        let mut labels: Vec<String> = scene
            .world
            .query::<&crate::components::Text>()
            .with::<&crate::components::TextButton>()
            .iter()
            .map(|text| text.content.clone())
            .collect();
        labels.sort();

        assert_eq!(labels, vec!["Die", "Exit Game", "Resume"]);
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
}
