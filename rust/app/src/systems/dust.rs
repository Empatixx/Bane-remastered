use std::f32::consts::{PI, TAU};

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::components::{Color, Layer, Lit, Position, Size, Sprite};
use crate::render::TextureId;

pub(crate) const FRAMES: usize = 3;
const SWAY: f32 = 5.0;
const TINT: Vec4 = Vec4::new(1.0, 0.95, 0.85, 0.0);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Dust {
    velocity: Vec2,
    phase: f32,
    age: f32,
    life: f32,
    peak: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DustSpec {
    pub(crate) at: Vec2,
    pub(crate) velocity: Vec2,
    pub(crate) size: f32,
    pub(crate) life: f32,
    pub(crate) peak: f32,
    pub(crate) layer: i32,
    pub(crate) seed: f32,
}

fn fraction(seed: f32) -> f32 {
    ((seed * 12.9898).sin() * 43_758.547).fract().abs()
}

pub(crate) fn spawn_dust(world: &mut World, texture: TextureId, spec: DustSpec) -> Entity {
    let frame = (fraction(spec.seed + 1.0) * FRAMES as f32) as usize % FRAMES;
    world.spawn((
        Dust {
            velocity: spec.velocity,
            phase: fraction(spec.seed + 2.0) * TAU,
            age: 0.0,
            life: spec.life,
            peak: spec.peak,
        },
        Position(spec.at),
        Size(Vec2::splat(spec.size)),
        Color(TINT),
        Sprite::cell(texture, frame, 0, FRAMES, 1),
        Layer(spec.layer),
        Lit,
    ))
}

pub(crate) fn tick_dust(world: &mut World, clock: f32, dt: f32) {
    let mut gone = Vec::new();
    for (entity, dust, position, color) in
        world.query_mut::<(Entity, &mut Dust, &mut Position, &mut Color)>()
    {
        dust.age += dt;
        let sway = (clock * 0.7 + dust.phase).sin() * SWAY;
        position.0 += (dust.velocity + Vec2::new(sway, 0.0)) * dt;
        let progress = (dust.age / dust.life).clamp(0.0, 1.0);
        color.0.w = dust.peak * (progress * PI).sin();
        if dust.age >= dust.life {
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

    fn spec() -> DustSpec {
        DustSpec {
            at: Vec2::new(100.0, 100.0),
            velocity: Vec2::new(0.0, -10.0),
            size: 12.0,
            life: 1.0,
            peak: 0.4,
            layer: 0,
            seed: 3.0,
        }
    }

    fn alpha(world: &World, entity: Entity) -> f32 {
        world.get::<&Color>(entity).map(|c| c.0.w).unwrap_or(-1.0)
    }

    #[test]
    fn dust_fades_in_peaks_in_the_middle_and_is_gone_at_the_end() {
        let mut world = World::new();
        let dust = spawn_dust(&mut world, TextureId::WHITE, spec());
        assert_eq!(alpha(&world, dust), 0.0);

        tick_dust(&mut world, 0.0, 0.5);
        assert!((alpha(&world, dust) - 0.4).abs() < 0.01);

        tick_dust(&mut world, 0.0, 0.25);
        let late = alpha(&world, dust);
        assert!(late > 0.0 && late < 0.4, "{late}");

        tick_dust(&mut world, 0.0, 0.5);
        assert!(world.get::<&Dust>(dust).is_err());
    }

    #[test]
    fn dust_drifts_the_way_it_was_told_to() {
        let mut world = World::new();
        let dust = spawn_dust(&mut world, TextureId::WHITE, spec());

        tick_dust(&mut world, 0.0, 0.5);

        let at = world
            .get::<&Position>(dust)
            .map(|p| p.0)
            .unwrap_or_default();
        assert!(at.y < 100.0 - 4.0, "{at:?}");
    }

    #[test]
    fn different_seeds_pick_different_looks_but_always_a_valid_frame() {
        let mut world = World::new();
        let mut frames = std::collections::HashSet::new();
        for seed in 0..30 {
            let entity = spawn_dust(
                &mut world,
                TextureId::WHITE,
                DustSpec {
                    seed: seed as f32,
                    ..spec()
                },
            );
            let sprite = world.get::<&Sprite>(entity).map(|s| s.uv).ok();
            frames.insert(format!("{sprite:?}"));
        }
        assert!(frames.len() > 1 && frames.len() <= FRAMES);
    }
}
