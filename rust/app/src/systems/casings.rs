use glam::Vec2;
use hecs::{Entity, World};

use crate::components::{Casing, Color, Position};

const GRAVITY: f32 = 900.0;
const BOUNCE: f32 = 0.3;
const REST_SECONDS: f32 = 25.0;
const FADE_SECONDS: f32 = 2.0;
const SETTLE_SPEED: f32 = 90.0;
pub(crate) const MAX_CASINGS: usize = 140;

pub(crate) fn tick_casings(world: &mut World, dt: f32) {
    let mut gone = Vec::new();
    for (entity, casing, position, color) in
        world.query_mut::<(Entity, &mut Casing, &mut Position, &mut Color)>()
    {
        casing.age += dt;
        if !casing.settled {
            casing.rise -= GRAVITY * dt;
            casing.height += casing.rise * dt;
            position.0 += casing.velocity * dt;
            casing.angle += casing.spin * dt;
            if casing.height <= 0.0 {
                casing.height = 0.0;
                if casing.rise < -SETTLE_SPEED {
                    casing.rise = -casing.rise * BOUNCE;
                    casing.velocity *= 0.5;
                    casing.spin *= 0.5;
                } else {
                    casing.settled = true;
                    casing.rise = 0.0;
                    casing.velocity = Vec2::ZERO;
                    casing.spin = 0.0;
                }
            }
        }
        let fade = (casing.age - REST_SECONDS) / FADE_SECONDS;
        color.0.w = (1.0 - fade).clamp(0.0, 1.0);
        if fade >= 1.0 {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
    let mut oldest: Vec<(Entity, f32)> = world
        .query::<(Entity, &Casing)>()
        .iter()
        .map(|(entity, casing)| (entity, casing.age))
        .collect();
    if oldest.len() > MAX_CASINGS {
        oldest.sort_by(|a, b| b.1.total_cmp(&a.1));
        let excess = oldest.len() - MAX_CASINGS;
        for (entity, _) in oldest.into_iter().take(excess) {
            let _ = world.despawn(entity);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prefabs;

    use super::*;

    fn spawn(world: &mut World) -> Entity {
        prefabs::casing(
            world,
            Vec2::new(100.0, 100.0),
            Vec2::X,
            false,
            [0.5; 4],
            crate::render::TextureId::WHITE,
        )
    }

    fn state(world: &World, entity: Entity) -> Option<Casing> {
        world.get::<&Casing>(entity).ok().map(|casing| *casing)
    }

    #[test]
    fn a_casing_flies_up_falls_down_and_comes_to_rest_on_the_floor() {
        let mut world = World::new();
        let casing = spawn(&mut world);
        let start = world
            .get::<&Position>(casing)
            .map(|p| p.0)
            .unwrap_or_default();
        let mut highest: f32 = 0.0;
        for _ in 0..180 {
            tick_casings(&mut world, 1.0 / 60.0);
            if let Some(state) = state(&world, casing) {
                highest = highest.max(state.height);
            }
        }

        let rested = state(&world, casing).expect("still lying there");
        assert!(highest > 5.0, "{highest}");
        assert!(rested.settled && rested.height == 0.0);
        let end = world
            .get::<&Position>(casing)
            .map(|p| p.0)
            .unwrap_or_default();
        assert!(end.distance(start) > 10.0 && end.distance(start) < 120.0);
    }

    #[test]
    fn a_resting_casing_does_not_move_any_more() {
        let mut world = World::new();
        let casing = spawn(&mut world);
        for _ in 0..180 {
            tick_casings(&mut world, 1.0 / 60.0);
        }
        let before = world
            .get::<&Position>(casing)
            .map(|p| p.0)
            .unwrap_or_default();

        for _ in 0..60 {
            tick_casings(&mut world, 1.0 / 60.0);
        }

        let after = world
            .get::<&Position>(casing)
            .map(|p| p.0)
            .unwrap_or_default();
        assert_eq!(before, after);
    }

    #[test]
    fn casings_fade_out_after_a_while_and_are_removed() {
        let mut world = World::new();
        let casing = spawn(&mut world);
        tick_casings(&mut world, REST_SECONDS + FADE_SECONDS / 2.0);
        let alpha = world.get::<&Color>(casing).map(|c| c.0.w).unwrap_or(-1.0);
        assert!(alpha > 0.0 && alpha < 1.0, "{alpha}");

        tick_casings(&mut world, FADE_SECONDS);
        assert!(state(&world, casing).is_none());
    }

    #[test]
    fn there_never_are_more_than_the_limit_and_the_oldest_go_first() {
        let mut world = World::new();
        let first = spawn(&mut world);
        tick_casings(&mut world, 0.5);
        for _ in 0..MAX_CASINGS + 10 {
            spawn(&mut world);
        }

        tick_casings(&mut world, 0.01);

        assert_eq!(world.query::<&Casing>().iter().count(), MAX_CASINGS);
        assert!(state(&world, first).is_none());
    }

    #[test]
    fn a_shotgun_shell_uses_the_second_row_of_the_sheet() {
        let mut world = World::new();
        let pistol = spawn(&mut world);
        let shell = prefabs::casing(
            &mut world,
            Vec2::ZERO,
            Vec2::X,
            true,
            [0.5; 4],
            crate::render::TextureId::WHITE,
        );
        let uv = |entity| {
            world
                .get::<&crate::components::Sprite>(entity)
                .map(|sprite| sprite.uv)
                .unwrap_or_default()
        };

        assert!(uv(shell)[1] > uv(pistol)[1]);
    }
}
