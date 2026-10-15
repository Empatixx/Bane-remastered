//! Draws what lies across the rooms from wall top to wall top (see [`crate::dungeon::Overhead`]):
//! the pieces sit above everything that moves, throw a shadow on the floor one wall's height
//! below, and turn see-through while the player is under them.

use glam::{Vec2, Vec4};
use hecs::World;

use crate::assets::{Textures, tex};
use crate::components::{Color, Layer, Lit, Player, Position, Size, Sprite};
use crate::dungeon::{Dungeon, OverheadKind};
use crate::tilemap::TILE_SIZE;

const COLUMNS: usize = 4;
const ROWS: usize = 7;
const LEFT_END: usize = 0;
const MIDDLE: usize = 1;
const RIGHT_END: usize = 2;
const CENTRE: usize = 3;

/// One piece is a 16x24 cell at the tiles' pixel scale.
const PIECE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE * 1.5);
const LAYER: i32 = 4;
const SHADOW_LAYER: i32 = 0;
/// The pieces lie on the top of the walls, so they sit this far above the middle of their row.
const LIFT: f32 = 0.0;
/// The shadow falls one wall's height below the piece, which is what reads as height.
const SHADOW_OFFSET: Vec2 = Vec2::new(6.0, TILE_SIZE);
const SHADOW_ALPHA: f32 = 0.3;
const SEE_THROUGH: f32 = 0.45;
const FADE_RATE: f32 = 8.0;

#[derive(Debug, Clone, Copy)]
pub(crate) struct OverheadPiece {
    /// The corners of the whole span.
    min: Vec2,
    max: Vec2,
    alpha: f32,
}

pub(crate) fn spawn_overheads(world: &mut World, dungeon: &Dungeon, textures: &Textures) {
    let texture = textures.get(tex::OVERHEADS);
    for room in &dungeon.rooms {
        for span in &room.overheads {
            let first = span.left - 1;
            let last = span.right + 1;
            let count = last + 1 - first;
            let row_y = (span.row as f32 + 0.5) * TILE_SIZE;
            let min = Vec2::new(first as f32 * TILE_SIZE, row_y - LIFT - PIECE.y / 2.0);
            let max = Vec2::new((last + 1) as f32 * TILE_SIZE, row_y - LIFT + PIECE.y / 2.0);
            let middle_column = (span.left + span.right) / 2;
            for (index, column) in (first..=last).enumerate() {
                let cell = if column == first {
                    LEFT_END
                } else if column == last {
                    RIGHT_END
                } else if span.kind == OverheadKind::Arch && column == middle_column {
                    CENTRE
                } else {
                    MIDDLE
                };
                let t = (index as f32 + 0.5) / count as f32;
                let sag = span.kind.sag() * (1.0 - (2.0 * t - 1.0).powi(2));
                let anchor = Vec2::new((column as f32 + 0.5) * TILE_SIZE, row_y - LIFT + sag);
                let sprite = Sprite::cell(texture, cell, span.kind.row(), COLUMNS, ROWS);
                world.spawn((
                    OverheadPiece {
                        min,
                        max,
                        alpha: 1.0,
                    },
                    Position(anchor),
                    Size(PIECE),
                    Color(Vec4::ONE),
                    sprite,
                    Layer(LAYER),
                    Lit,
                ));
                world.spawn((
                    Position(anchor + SHADOW_OFFSET + Vec2::new(0.0, LIFT)),
                    Size(PIECE),
                    Color(Vec4::new(0.0, 0.0, 0.0, SHADOW_ALPHA)),
                    sprite,
                    Layer(SHADOW_LAYER),
                ));
            }
        }
    }
}

pub(crate) fn tick_overheads(world: &mut World, dt: f32) {
    let player = world
        .query::<(&Position, &Size)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(position, size)| (position.0 - size.0 / 2.0, position.0 + size.0 / 2.0));
    let fade = (FADE_RATE * dt).min(1.0);
    for (piece, color) in world.query_mut::<(&mut OverheadPiece, &mut Color)>() {
        let (min, max) = (piece.min, piece.max);
        let under = player.is_some_and(|(low, high)| {
            low.x < max.x && high.x > min.x && low.y < max.y && high.y > min.y
        });
        let target = if under { SEE_THROUGH } else { 1.0 };
        piece.alpha += (target - piece.alpha) * fade;
        color.0.w = piece.alpha;
    }
}

#[cfg(test)]
mod tests {
    use hecs::Entity;

    use super::*;
    use crate::dungeon::{Overhead, Theme};
    use crate::rng::Rng;

    fn floor_with_overheads() -> (World, Dungeon, Overhead) {
        let textures = Textures::default();
        for seed in 0..40 {
            let dungeon = Dungeon::generate_themed(&mut Rng::new(seed), Theme::Stone)
                .expect("a dungeon");
            let span = dungeon.rooms.iter().flat_map(|room| room.overheads.clone()).next();
            if let Some(span) = span {
                let mut world = World::new();
                spawn_overheads(&mut world, &dungeon, &textures);
                return (world, dungeon, span);
            }
        }
        panic!("no floor with something overhead");
    }

    #[test]
    fn every_span_gets_its_ends_middle_and_a_shadow_for_each_piece() {
        let (world, dungeon, _) = floor_with_overheads();
        let pieces: usize = dungeon
            .rooms
            .iter()
            .flat_map(|room| &room.overheads)
            .map(|span| span.right + 1 - span.left + 2)
            .sum();
        assert_eq!(world.query::<&OverheadPiece>().iter().count(), pieces);
        let shadows = world
            .query::<&Layer>()
            .iter()
            .filter(|layer| layer.0 == SHADOW_LAYER)
            .count();
        assert_eq!(shadows, pieces);
    }

    fn piece_alpha(world: &World) -> Vec<f32> {
        world
            .query::<(&OverheadPiece, &Color)>()
            .iter()
            .map(|(_, color)| color.0.w)
            .collect()
    }

    #[test]
    fn a_span_turns_see_through_while_the_player_is_under_it() {
        let (mut world, _, span) = floor_with_overheads();
        let under = Vec2::new(
            (span.left + span.right + 1) as f32 / 2.0 * TILE_SIZE,
            (span.row as f32 + 0.5) * TILE_SIZE - LIFT,
        );
        world.spawn((Player, Position(under), Size(Vec2::splat(64.0))));
        for _ in 0..60 {
            tick_overheads(&mut world, 1.0 / 60.0);
        }
        let near = piece_alpha(&world);
        assert!(near.iter().any(|alpha| (alpha - SEE_THROUGH).abs() < 0.02));

        let player = world
            .query::<Entity>()
            .with::<&Player>()
            .iter()
            .next()
            .expect("player");
        let _ = world.insert_one(player, Position(under + Vec2::new(0.0, 2000.0)));
        for _ in 0..60 {
            tick_overheads(&mut world, 1.0 / 60.0);
        }
        assert!(piece_alpha(&world).iter().all(|alpha| *alpha > 0.98));
    }
}
