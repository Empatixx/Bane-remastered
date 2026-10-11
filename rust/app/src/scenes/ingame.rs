use glam::{Vec2, Vec4};
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::{Assets, BULLET, RAT, TILESET};
use crate::audio::{AudioCommand, SoundEffect, Soundtrack};
use crate::camera::Camera;
use crate::components::{BigMap, Hidden, MapSize, Minimap, UiAction, Weapon};
use crate::dungeon::{Dungeon, Rng};
use crate::input::Input;
use crate::physics::Physics;
use crate::postfx::SceneFx;
use crate::prefabs;
use crate::settings::Settings;
use crate::systems;
use crate::tilemap::TILE_SIZE;
use crate::viewport::TARGET;

use super::{Cursor, Pause, Scene, SceneId, Transition};

#[derive(Default)]
pub(crate) struct InGameScene {
    world: World,
    physics: Physics,
    camera: Camera,
    pause: Pause,
    clock: f32,
    sounds: Vec<AudioCommand>,
    hit: bool,
    dungeon: Option<Dungeon>,
    rooms: Option<systems::RoomRuntime>,
    rng: Rng,
    big_map: bool,
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
        if self.pause.is_active() || self.big_map {
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
        let solid_cells = prefabs::tilemap(
            &mut self.world,
            &dungeon.grid,
            &assets.tileset,
            assets.textures.get(TILESET),
        );
        self.physics.add_walls(&solid_cells, TILE_SIZE);
        let start = dungeon.start_position().unwrap_or(bounds / 2.0);
        prefabs::player(&mut self.world, &mut self.physics, start, &assets.textures);
        self.rooms = Some(systems::RoomRuntime::new(&dungeon));
        spawn_minimaps(&mut self.world, &dungeon, textures);
        self.camera = Camera::new(bounds);
        if let Some(room) = dungeon.rooms.first() {
            if let Some((min, max)) = dungeon.camera_region(room.origin.0 + 1, room.origin.1 + 1) {
                self.camera.set_bounds(min, max);
            }
        }
        self.dungeon = Some(dungeon);
        prefabs::status_bars(&mut self.world, textures);
        prefabs::coin_hud(
            &mut self.world,
            textures,
            "ingame/banecoin.tga",
            Vec4::new(1.0, 216.0 / 255.0, 0.0, 1.0),
        );
        prefabs::weapon_hud(&mut self.world, textures, Weapon::pistol().clip_size);
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
        if input.just_pressed(KeyCode::Tab) && !self.pause.is_active() {
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
        systems::update_weapon_hud(&mut self.world);
        systems::fade_lights(&mut self.world);
        if let (Some(dungeon), Some(rooms)) = (&self.dungeon, &mut self.rooms) {
            let context = systems::RoomContext {
                dungeon,
                textures: &assets.textures,
                rat_texture: assets.textures.get(RAT),
            };
            systems::update_rooms(
                rooms,
                &context,
                &mut self.world,
                &mut self.physics,
                &mut self.rng,
            );
        }
        systems::remove_expired(&mut self.world);
        if let (Some(dungeon), Some(rooms)) = (&self.dungeon, &self.rooms) {
            systems::update_minimaps(&mut self.world, rooms.entered(), rooms.current(), dt);
            if let Some(position) = player_tile(&self.world) {
                if let Some((min, max)) = dungeon.camera_region(position.0, position.1) {
                    self.camera.glide_bounds(min, max, dt);
                }
            }
        }
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
        if systems::contact_damage(&mut self.world, &mut self.sounds) {
            self.hit = true;
        }
        if systems::player_is_dead(&self.world) {
            Transition::Switch(SceneId::Lobby)
        } else {
            Transition::None
        }
    }
}

impl InGameScene {
    fn set_big_map(&mut self, open: bool) {
        self.big_map = open;
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
