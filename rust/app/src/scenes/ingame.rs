use glam::{Vec2, Vec4};
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::{Assets, BULLET, TILESET};
use crate::audio::{AudioCommand, SoundEffect, Soundtrack};
use crate::camera::Camera;
use crate::components::{BigMap, Blend, Hidden, Interaction, Loadout, MapSize, Minimap, UiAction};
use crate::controls::Action;
use crate::dungeon::{Dungeon, Rng};
use crate::input::Input;
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::postfx::SceneFx;
use crate::prefabs;
use crate::profile::SharedProfile;
use crate::settings::Settings;
use crate::systems;
use crate::tilemap::TILE_SIZE;
use crate::upgrades::Perks;
use crate::viewport::TARGET;

use super::{Cursor, Pause, Scene, SceneId, Transition};

const SLOT_MACHINE_COINS: i32 = 100;

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
    rooms: Option<systems::RoomRuntime>,
    rng: Rng,
    big_map: bool,
    floor: u32,
    navigation: Navigation,
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
        self.camera.position()
    }

    fn set_blend(&mut self, blend: f32) {
        self.blend = Blend(blend);
    }

    fn blend(&self) -> f32 {
        self.blend.0
    }

    fn enter(&mut self, assets: &Assets) {
        self.floor = 1;
        self.victory = false;
        self.death = None;
        self.build(assets);
        let perks = self
            .profile
            .as_ref()
            .map_or_else(Perks::default, SharedProfile::perks);
        systems::apply_run_perks(&mut self.world, perks);
    }

    fn update(
        &mut self,
        input: &Input,
        dt: f32,
        assets: &Assets,
        _settings: &mut Settings,
    ) -> Transition {
        if self.death.is_none() && (self.victory || systems::player_is_dead(&self.world)) {
            self.start_death_screen(assets);
        }
        if let Some(screen) = &mut self.death {
            screen.advance(dt, input.any_key());
            systems::update_death_ui(&mut self.world, screen);
            if screen.finished() {
                return Transition::Switch(SceneId::Lobby);
            }
            systems::follow_backdrop(&mut self.world, self.camera.position());
            return Transition::None;
        }
        if input.action_pressed(Action::Map) && !self.pause.is_active() {
            self.set_big_map(!self.big_map);
        }
        if input.just_pressed(KeyCode::Escape) {
            if self.big_map {
                self.set_big_map(false);
            } else {
                self.pause.toggle(&mut self.world, assets);
            }
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
        }
        systems::animate_player(&mut self.world, input);
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
            rand::random_range(0.0..1.0),
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
                "ingame/artefacts/berserkpot-particle.tga",
                crate::artefacts::BERSERK_PHASE,
            )
        } else {
            ("ingame/gazewind_particle.tga", 0.0)
        };
        systems::spawn_gaze_wind(
            &mut self.world,
            &self.physics,
            assets.textures.get(particle),
            dt,
            self.clock + phase,
        );
        systems::update_shadow(&mut self.world, dt);
        systems::update_status_bars(&mut self.world, dt);
        systems::update_weapon_hud(&mut self.world, &assets.textures);
        systems::fade_lights(&mut self.world);
        systems::flash_hits(&mut self.world, dt);
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
                &mut self.rng,
            );
        }
        self.handle_room_events(events, assets);
        systems::update_boss_hud(&mut self.world);
        systems::enemy_drops(
            &mut self.world,
            &mut self.kills_without_drop,
            &assets.textures,
            |bound| rand::random_range(0..bound),
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
            |bound| rand::random_range(0..bound),
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
        systems::follow_backdrop(&mut self.world, self.camera.position());
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
        Transition::None
    }

    fn fixed_update(&mut self, input: &Input, dt: f32, assets: &Assets) -> Transition {
        if self.pause.is_active() || self.death.is_some() {
            return Transition::None;
        }
        systems::steer_player(&mut self.world, input);
        systems::tick_spawning(&mut self.world, &mut self.physics, dt);
        systems::dash(&mut self.world, dt);
        systems::plan_paths(&mut self.world, &self.navigation);
        systems::chase_player(&mut self.world);
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
        systems::bullet_hits(&mut self.world, &mut self.physics, &mut self.sounds);
        systems::pet_bites(&mut self.world, dt);
        systems::trap_hits_enemies(&mut self.world, dt);
        systems::death_bursts(&mut self.world, &mut self.physics, &assets.textures);
        systems::death_heals(&mut self.world);
        systems::break_objects(
            &mut self.world,
            &mut self.physics,
            &assets.textures,
            &mut self.sounds,
            |bound| rand::random_range(0..bound),
        );
        systems::clean_up_after_boss(&mut self.world);
        systems::reap_dead(&mut self.world, &mut self.physics);
        if systems::contact_damage(&mut self.world, &mut self.sounds) {
            self.hit = true;
        }
        systems::enemy_bullet_hits(&mut self.world, &mut self.physics);
        Transition::None
    }
}

impl InGameScene {
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
        self.rng = Rng::from_clock();
        tracing::info!(seed = self.rng.seed(), "generating the dungeon");
        let dungeon = match Dungeon::generate(&mut self.rng) {
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
        self.navigation = Navigation::new(&solid_cells);
        let start = dungeon.start_position().unwrap_or(bounds / 2.0);
        prefabs::player(&mut self.world, &mut self.physics, start, &assets.textures);
        systems::apply_floor_perks(&mut self.world, perks);
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
        systems::stock_rooms(
            &mut self.world,
            &dungeon,
            self.floor,
            perks.discount,
            textures,
            |bound| rand::random_range(0..bound),
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
        systems::immobilize_traps(&mut self.world, perks, |bound| rand::random_range(0..bound));
        self.dungeon = Some(dungeon);
        prefabs::status_bars(&mut self.world, textures);
        prefabs::coin_hud(
            &mut self.world,
            textures,
            "ingame/banecoin.tga",
            Vec4::new(1.0, 216.0 / 255.0, 0.0, 1.0),
        );
        prefabs::weapon_hud(&mut self.world, textures);
        prefabs::artefact_hud(&mut self.world, textures);
        if let Some(target) = systems::camera_target(&self.world, TARGET / 2.0, 1.0) {
            self.camera.snap_to(target);
        }
        systems::follow_backdrop(&mut self.world, self.camera.position());
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
            icons: textures.get("ingame/minimap_icons.tga"),
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
            crate::components::Sprite::whole(textures.get("ingame/minimap_layout.tga")),
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
