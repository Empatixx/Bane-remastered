use std::f32::consts::TAU;

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{tex, Tex};
use crate::assets::{Assets, TILESET};
use crate::components::{Animation, Color, Layer, Lit, Position, ScreenSpace, Size, Sprite};
use crate::enemies::EnemyKind;
use crate::prefabs;
use crate::props::{Look, Prop};
use crate::rng::Rng;
use crate::systems;
use crate::tilemap::{self, TILE_SIZE};
use crate::viewport::TARGET;

const PAN_PERIOD: f32 = 150.0;
const BOB_PERIOD: f32 = 97.0;
const CRITTER_FIRST: f32 = 2.5;
const CRITTER_PAUSE: (f32, f32) = (5.0, 11.0);
const CRITTER_MARGIN: f32 = 180.0;
const CRITTER_SPEED_SCALE: f32 = 0.5;
const CRITTER_LAYER: i32 = 1;
const CRITTER_KINDS: [EnemyKind; 3] = [EnemyKind::Rat, EnemyKind::Rat, EnemyKind::Bat];
const PARTICLE_TEXTURE: Tex = tex::GAZEWIND_PARTICLE;
const AMBIENCE_SEED: u64 = 0x5eed;
const DIM: Vec4 = Vec4::new(0.02, 0.01, 0.04, 0.5);
const DIM_LAYER: i32 = -1;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Critter {
    velocity: Vec2,
}

pub(crate) struct Stage {
    bounds: Vec2,
    clock: f32,
    until_critter: f32,
    rng: Rng,
    /// The dust and fireflies' dice and clock, for as long as this room stands.
    ambience: systems::Ambience,
}

impl Stage {
    pub(crate) fn build(world: &mut World, assets: &Assets) -> Self {
        let grid = tilemap::crypt();
        let bounds = grid.pixel_size();
        let textures = &assets.textures;
        prefabs::backdrop(world);
        prefabs::tilemap(world, &grid, &assets.tileset, textures.get(TILESET));
        let altar = Vec2::new(bounds.x / 2.0, bounds.y * 0.55);
        let spot = |x: f32, y: f32| altar + Vec2::new(x, y);
        let placed: [(Prop, Vec2, Look); 29] = [
            (Prop::AngelStatue, spot(-128.0, -283.0), Look::DEFAULT),
            (Prop::Pentagram, spot(-112.5, -60.0), Look::DEFAULT),
            (Prop::Candlestick, spot(-194.0, -40.0), Look::DEFAULT),
            (Prop::Candlestick, spot(146.0, -40.0), Look::DEFAULT),
            (Prop::Candlestick, spot(-214.0, 40.0), Look::DEFAULT),
            (Prop::Candlestick, spot(166.0, 40.0), Look::DEFAULT),
            (Prop::Candlestick, spot(-24.0, 80.0), Look::DEFAULT),
            (Prop::LayingBook, spot(-330.0, -20.0), Look::DEFAULT),
            (Prop::LayingBook, spot(270.0, 110.0), Look::DEFAULT),
            (Prop::LayingBook, spot(-140.0, 170.0), Look::DEFAULT),
            (Prop::Chains, Vec2::new(4.0, 0.0) * TILE_SIZE, Look::DEFAULT),
            (Prop::Chains, Vec2::new(9.0, 0.0) * TILE_SIZE, Look::DEFAULT),
            (
                Prop::Chains,
                Vec2::new(21.0, 0.0) * TILE_SIZE,
                Look::DEFAULT,
            ),
            (
                Prop::Chains,
                Vec2::new(26.0, 0.0) * TILE_SIZE,
                Look::DEFAULT,
            ),
            (Prop::Torch, Vec2::new(1.0, 2.0) * TILE_SIZE, Look::DEFAULT),
            (Prop::Torch, Vec2::new(1.0, 8.0) * TILE_SIZE, Look::DEFAULT),
            (Prop::Torch, Vec2::new(28.0, 2.0) * TILE_SIZE, Look::FLIPPED),
            (Prop::Torch, Vec2::new(28.0, 8.0) * TILE_SIZE, Look::FLIPPED),
            (
                Prop::Torch,
                Vec2::new(11.0, 1.0) * TILE_SIZE,
                Look::SECOND_ROW,
            ),
            (
                Prop::Torch,
                Vec2::new(18.0, 1.0) * TILE_SIZE,
                Look::SECOND_ROW,
            ),
            (Prop::Bones, Vec2::new(6.0, 9.0) * TILE_SIZE, Look::DEFAULT),
            (
                Prop::Bones,
                Vec2::new(23.0, 8.0) * TILE_SIZE,
                Look::SECOND_ROW,
            ),
            (
                Prop::Bones,
                Vec2::new(14.0, 10.0) * TILE_SIZE,
                Look::DEFAULT,
            ),
            (Prop::Cobweb, Vec2::new(1.0, 1.0) * TILE_SIZE, Look::DEFAULT),
            (
                Prop::Cobweb,
                Vec2::new(28.0, 1.0) * TILE_SIZE,
                Look::FLIPPED,
            ),
            (Prop::Rock, Vec2::new(4.0, 6.0) * TILE_SIZE, Look::DEFAULT),
            (
                Prop::Rock,
                Vec2::new(25.0, 5.0) * TILE_SIZE,
                Look::SECOND_ROW,
            ),
            (Prop::Box, Vec2::new(3.0, 9.0) * TILE_SIZE, Look::DEFAULT),
            (Prop::Box, Vec2::new(26.0, 9.0) * TILE_SIZE, Look::DEFAULT),
        ];
        for (prop, top_left, look) in placed {
            prefabs::prop(world, prop, top_left, look, textures);
        }
        let ambience = systems::spawn_ambience(
            world,
            textures.get(PARTICLE_TEXTURE),
            bounds,
            altar,
            AMBIENCE_SEED,
        );
        world.spawn((
            Position(TARGET / 2.0),
            Size(TARGET),
            Color(DIM),
            Layer(DIM_LAYER),
            ScreenSpace,
        ));
        Self {
            bounds,
            clock: 0.0,
            until_critter: CRITTER_FIRST,
            rng: Rng::from_clock(),
            ambience,
        }
    }

    pub(crate) fn camera(&self) -> Vec2 {
        let room = (self.bounds - TARGET).max(Vec2::ZERO);
        Vec2::new(
            room.x * (0.5 - 0.5 * (self.clock * TAU / PAN_PERIOD).cos()),
            room.y * (0.5 - 0.5 * (self.clock * TAU / BOB_PERIOD).cos()),
        )
    }

    pub(crate) fn tick(&mut self, world: &mut World, assets: &Assets, dt: f32) {
        self.clock += dt;
        let camera = self.camera();
        systems::follow_backdrop(world, camera);
        systems::tick_ambience(world, &mut self.ambience, assets.textures.get(PARTICLE_TEXTURE), camera, dt);
        systems::tick_dust(world, self.clock, dt);
        self.until_critter -= dt;
        if self.until_critter <= 0.0 {
            self.until_critter =
                CRITTER_PAUSE.0 + (CRITTER_PAUSE.1 - CRITTER_PAUSE.0) * self.rng.float();
            self.release_critter(world, assets, camera);
        }
        run_critters(world, camera, dt);
    }

    fn release_critter(&mut self, world: &mut World, assets: &Assets, camera: Vec2) {
        let kind = CRITTER_KINDS[self.rng.below(CRITTER_KINDS.len())];
        let spec = kind.spec();
        let rightwards = self.rng.below(2) == 0;
        let floor = TILE_SIZE * 2.0 + (self.bounds.y - TILE_SIZE * 4.0) * self.rng.float();
        let y = floor.clamp(camera.y + 80.0, camera.y + TARGET.y - 80.0);
        let x = if rightwards {
            camera.x - CRITTER_MARGIN / 2.0
        } else {
            camera.x + TARGET.x + CRITTER_MARGIN / 2.0
        };
        let speed = spec.speed() * CRITTER_SPEED_SCALE;
        let direction = if rightwards { 1.0 } else { -1.0 };
        let entity = spawn_critter(
            world,
            assets,
            kind,
            Vec2::new(x, y),
            Vec2::new(direction * speed, 0.0),
        );
        if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
            animation.face(!rightwards);
        }
    }
}

fn spawn_critter(
    world: &mut World,
    assets: &Assets,
    kind: EnemyKind,
    at: Vec2,
    velocity: Vec2,
) -> Entity {
    let spec = kind.spec();
    world.spawn((
        Critter { velocity },
        Position(at),
        Size(spec.size()),
        Color(Vec4::ONE),
        Sprite::cell(
            assets.textures.get(spec.texture),
            0,
            0,
            spec.columns,
            spec.rows,
        ),
        Animation::new(spec.columns, spec.rows, spec.idle_clip()),
        Layer(CRITTER_LAYER),
        Lit,
    ))
}

fn run_critters(world: &mut World, camera: Vec2, dt: f32) {
    let low = camera.x - CRITTER_MARGIN * 2.0;
    let high = camera.x + TARGET.x + CRITTER_MARGIN * 2.0;
    let mut gone = Vec::new();
    for (entity, critter, position) in world.query_mut::<(Entity, &Critter, &mut Position)>() {
        position.0 += critter.velocity * dt;
        if position.0.x < low || position.0.x > high {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stage() -> Stage {
        Stage {
            bounds: Vec2::new(1920.0, 768.0),
            clock: 0.0,
            until_critter: 100.0,
            rng: Rng::new(3),
            ambience: systems::spawn_ambience(
                &mut World::new(),
                crate::render::TextureId::WHITE,
                Vec2::ZERO,
                Vec2::ZERO,
                3,
            ),
        }
    }

    #[test]
    fn the_camera_pans_slowly_across_the_room_and_stays_inside_it() {
        let mut stage = stage();
        let mut left = f32::MAX;
        let mut right: f32 = 0.0;
        for step in 0..18_000 {
            stage.clock = step as f32 / 60.0;
            let camera = stage.camera();
            left = left.min(camera.x);
            right = right.max(camera.x);
            assert!(camera.x >= 0.0 && camera.x <= 1920.0 - TARGET.x + 0.01);
            assert!(camera.y >= 0.0 && camera.y <= 768.0 - TARGET.y + 0.01);
        }
        assert!(left < 5.0 && right > 1920.0 - TARGET.x - 5.0);
    }

    #[test]
    fn a_small_room_does_not_pan_at_all() {
        let mut stage = stage();
        stage.bounds = TARGET;
        stage.clock = 12.0;
        assert_eq!(stage.camera(), Vec2::ZERO);
    }

    #[test]
    fn critters_run_across_and_leave_the_scene() {
        let mut world = World::new();
        let rat = world.spawn((
            Critter {
                velocity: Vec2::new(200.0, 0.0),
            },
            Position(Vec2::new(-90.0, 300.0)),
        ));

        run_critters(&mut world, Vec2::ZERO, 1.0);
        let at = world.get::<&Position>(rat).map(|p| p.0).unwrap_or_default();
        assert!(at.x > 100.0);

        for _ in 0..10 {
            run_critters(&mut world, Vec2::ZERO, 1.0);
        }
        assert!(world.get::<&Critter>(rat).is_err());
    }
}
