//! Draws what lies across the rooms from wall to wall (see [`crate::dungeon::Overhead`]):
//! the pieces sit above everything that moves, throw a shadow on the floor one wall's height
//! below, and turn see-through while the player is under them. Hanging things swing a little
//! on their chains, candles and lanterns flicker and light the room.

use glam::{Vec2, Vec3, Vec4};
use hecs::World;

use crate::assets::{Textures, tex};
use crate::components::{Color, Layer, Light, Lit, Player, Position, Size, Sprite};
use crate::dungeon::{Dungeon, HangingKind, Overhead, OverheadKind};
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
    for room in &dungeon.rooms {
        for span in &room.overheads {
            if span.upright {
                spawn_upright(world, span, textures);
            } else {
                spawn_across(world, span, textures);
            }
        }
    }
}

/// The span covers just the open tiles, its ends meeting the walls' sides.
fn spawn_across(world: &mut World, span: &Overhead, textures: &Textures) {
    let texture = textures.get(tex::OVERHEADS);
    let (left, right, row) = (span.first.0, span.last.0, span.first.1);
    let count = span.length();
    let row_y = (row as f32 + 0.5) * TILE_SIZE;
    let min = Vec2::new(left as f32 * TILE_SIZE, row_y - LIFT - PIECE.y / 2.0);
    let max = Vec2::new((right + 1) as f32 * TILE_SIZE, row_y - LIFT + PIECE.y / 2.0);
    let middle_column = (left + right) / 2;
    for (index, column) in (left..=right).enumerate() {
        let cell = if column == left {
            LEFT_END
        } else if column == right {
            RIGHT_END
        } else if span.kind == OverheadKind::Arch && column == middle_column {
            CENTRE
        } else {
            MIDDLE
        };
        // The ends are fixed to the walls; the middle sags (or the arch rises) between them.
        let t = index as f32 / (count - 1) as f32;
        let sag = span.kind.sag() * (1.0 - (2.0 * t - 1.0).powi(2));
        let anchor = Vec2::new((column as f32 + 0.5) * TILE_SIZE, row_y - LIFT + sag);
        let sprite = Sprite::cell(texture, cell, span.kind.row(), COLUMNS, ROWS);
        spawn_piece(world, anchor, PIECE, sprite, min, max);
    }
}

/// An upright span covers just the open tiles too, from the foot of the block above to the top
/// of the block below.
fn spawn_upright(world: &mut World, span: &Overhead, textures: &Textures) {
    let texture = textures.get(tex::OVERHEADS_UPRIGHT);
    let x = (span.first.0 as f32 + 0.5) * TILE_SIZE;
    let top = span.first.1 as f32 * TILE_SIZE;
    let bottom = (span.last.1 + 1) as f32 * TILE_SIZE;
    let count = span.length();
    let size = Vec2::splat(TILE_SIZE);
    let min = Vec2::new(x - TILE_SIZE / 2.0, top);
    let max = Vec2::new(x + TILE_SIZE / 2.0, bottom);
    for index in 0..count {
        let cell = if index == 0 {
            LEFT_END
        } else if index + 1 == count {
            RIGHT_END
        } else if span.kind == OverheadKind::Arch && index == count / 2 {
            CENTRE
        } else {
            MIDDLE
        };
        let anchor = Vec2::new(x, top + size.y * (index as f32 + 0.5));
        let sprite = Sprite::cell(texture, cell, span.kind.row(), COLUMNS, ROWS);
        spawn_piece(world, anchor, size, sprite, min, max);
    }
}

/// One piece of a span and its shadow on the floor.
fn spawn_piece(world: &mut World, anchor: Vec2, size: Vec2, sprite: Sprite, min: Vec2, max: Vec2) {
    world.spawn((
        OverheadPiece {
            min,
            max,
            alpha: 1.0,
        },
        Position(anchor),
        Size(size),
        Color(Vec4::ONE),
        sprite,
        Layer(LAYER),
        Lit,
    ));
    world.spawn((
        Position(anchor + SHADOW_OFFSET + Vec2::new(0.0, LIFT)),
        Size(size),
        Color(Vec4::new(0.0, 0.0, 0.0, SHADOW_ALPHA)),
        sprite,
        Layer(SHADOW_LAYER),
    ));
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

const HANGING_COLUMNS: usize = 4;
const HANGING_ROWS: usize = 5;
const CHAIN_CELL: usize = 2;
/// The topmost chain piece, fading out upwards in the texture itself.
const CHAIN_TOP_CELL: usize = 3;
/// How high above its floor tile a hanging thing hangs; its shadow lies on the tile.
const HANG: f32 = 120.0;
/// Chain pieces above the thing; the last one fades into the dark of the ceiling.
const CHAIN_PIECES: usize = 2;
/// Hanging things are high up, closer to the camera than the floor, so they show bigger, and
/// the chain widens towards the ceiling (keeping its length).
const THING_SCALE: f32 = 1.35;
const CHAIN_GROWTH: f32 = 0.7;
/// The chain fades out a little more with each piece towards the ceiling.
const CHAIN_FADE: f32 = 0.15;
const SWAY: f32 = 4.0;
const SWAY_RATE: f32 = 1.1;
const FLICKER_RATE: f32 = 7.0;
const HANGING_SHADOW_ALPHA: f32 = 0.28;
const GLOW: Vec3 = Vec3::new(255.0, 196.0, 110.0);

#[derive(Debug, Clone, Copy)]
pub(crate) struct HangingPart {
    base: Vec2,
    /// How much of the swing this part follows: the thing all of it, the chain less towards
    /// the ceiling.
    reach: f32,
    phase: f32,
    /// The thing itself (not a chain piece) and whether it flickers.
    thing: Option<bool>,
    row: usize,
    /// The most opaque this part gets.
    opacity: f32,
    min: Vec2,
    max: Vec2,
    alpha: f32,
}

pub(crate) fn spawn_hanging(world: &mut World, dungeon: &Dungeon, textures: &Textures) {
    let texture = textures.get(tex::HANGING);
    for room in &dungeon.rooms {
        for hanging in &room.hanging {
            let floor = (Vec2::new(hanging.tile.0 as f32, hanging.tile.1 as f32) + 0.5) * TILE_SIZE;
            let at = floor - Vec2::new(0.0, HANG);
            let thing_size = PIECE * THING_SCALE;
            let min = at - thing_size / 2.0;
            let max = at + thing_size / 2.0;
            let phase = (hanging.tile.0 * 7 + hanging.tile.1 * 13) as f32 * 0.37;
            let row = hanging.kind.row();
            let part = |base: Vec2, reach: f32, thing: Option<bool>, opacity: f32| HangingPart {
                base,
                reach,
                phase,
                thing,
                row,
                opacity,
                min,
                max,
                alpha: opacity,
            };
            // Each piece sits on top of the one below it, wider the higher it hangs.
            let mut base = at - Vec2::new(0.0, (thing_size.y - PIECE.y) / 2.0);
            for piece in 1..=CHAIN_PIECES {
                let size = Vec2::new(PIECE.x * (THING_SCALE + CHAIN_GROWTH * piece as f32), PIECE.y);
                base -= Vec2::new(0.0, PIECE.y);
                let reach = 1.0 - piece as f32 / (CHAIN_PIECES + 1) as f32;
                let cell = if piece == CHAIN_PIECES {
                    CHAIN_TOP_CELL
                } else {
                    CHAIN_CELL
                };
                world.spawn((
                    part(base, reach, None, 1.0 - CHAIN_FADE * (piece - 1) as f32),
                    Position(base),
                    Size(size),
                    Color(Vec4::ONE),
                    Sprite::cell(texture, cell, row, HANGING_COLUMNS, HANGING_ROWS),
                    Layer(LAYER),
                    Lit,
                ));
            }
            let sprite = Sprite::cell(texture, 0, row, HANGING_COLUMNS, HANGING_ROWS);
            let thing = world.spawn((
                part(at, 1.0, Some(hanging.kind.glows()), 1.0),
                Position(at),
                Size(thing_size),
                Color(Vec4::ONE),
                sprite,
                Layer(LAYER),
                Lit,
            ));
            if hanging.kind.glows() {
                let _ = world.insert_one(
                    thing,
                    Light::rgb(Vec2::new(0.0, -8.0), GLOW.x, GLOW.y, GLOW.z, 90.0, 1.0),
                );
            }
            world.spawn((
                Position(floor + Vec2::new(SHADOW_OFFSET.x, 0.0)),
                Size(PIECE),
                Color(Vec4::new(0.0, 0.0, 0.0, HANGING_SHADOW_ALPHA)),
                sprite,
                Layer(SHADOW_LAYER),
            ));
        }
    }
}

pub(crate) fn tick_hanging(world: &mut World, clock: f32, dt: f32) {
    let player = world
        .query::<(&Position, &Size)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(position, size)| (position.0 - size.0 / 2.0, position.0 + size.0 / 2.0));
    let fade = (FADE_RATE * dt).min(1.0);
    for (part, position, color, sprite) in
        world.query_mut::<(&mut HangingPart, &mut Position, &mut Color, &mut Sprite)>()
    {
        let swing = (clock * SWAY_RATE + part.phase).sin() * SWAY * part.reach;
        position.0 = part.base + Vec2::new(swing, 0.0);
        if part.thing == Some(true) || (part.thing.is_some() && part.row == HangingKind::Vine.row()) {
            let rate = if part.thing == Some(true) { FLICKER_RATE } else { SWAY_RATE };
            let frame = ((clock * rate + part.phase) as usize) % 2;
            sprite.uv = Sprite::cell_uv(frame, part.row, HANGING_COLUMNS, HANGING_ROWS);
        }
        let under = player.is_some_and(|(low, high)| {
            low.x < part.max.x && high.x > part.min.x && low.y < part.max.y && high.y > part.min.y
        });
        let target = if under {
            part.opacity * SEE_THROUGH
        } else {
            part.opacity
        };
        part.alpha += (target - part.alpha) * fade;
        color.0.w = part.alpha;
    }
}

#[cfg(test)]
mod tests {
    use hecs::Entity;

    use super::*;
    use crate::dungeon::Theme;
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
        let least: usize = dungeon
            .rooms
            .iter()
            .flat_map(|room| &room.overheads)
            .map(Overhead::length)
            .sum();
        let pieces = world.query::<&OverheadPiece>().iter().count();
        assert!(pieces >= least, "{pieces} < {least}");
        let shadows = world
            .query::<&Layer>()
            .iter()
            .filter(|layer| layer.0 == SHADOW_LAYER)
            .count();
        assert_eq!(shadows, pieces);
    }

    #[test]
    fn hanging_things_get_a_chain_a_shadow_and_swing() {
        let textures = Textures::default();
        let dungeon = (0..20)
            .map(|seed| Dungeon::generate_themed(&mut Rng::new(seed), Theme::Stone).expect("a dungeon"))
            .find(|dungeon| dungeon.rooms.iter().any(|room| !room.hanging.is_empty()))
            .expect("a floor with something hanging");
        let mut world = World::new();
        spawn_hanging(&mut world, &dungeon, &textures);
        let count: usize = dungeon.rooms.iter().map(|room| room.hanging.len()).sum();
        assert_eq!(
            world.query::<&HangingPart>().iter().count(),
            count * (CHAIN_PIECES + 1)
        );
        let positions = |world: &World| -> Vec<Vec2> {
            world
                .query::<(&HangingPart, &Position)>()
                .iter()
                .filter(|(part, _)| part.thing.is_some())
                .map(|(_, position)| position.0)
                .collect()
        };
        tick_hanging(&mut world, 0.0, 0.0);
        let before = positions(&world);
        tick_hanging(&mut world, 0.7, 0.0);
        let after = positions(&world);
        assert!(before.iter().zip(&after).any(|(a, b)| (a.x - b.x).abs() > 0.5));
        assert!(before.iter().zip(&after).all(|(a, b)| (a.x - b.x).abs() <= 2.0 * SWAY && a.y == b.y));
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
            (span.first.0 + span.last.0 + 1) as f32 / 2.0,
            (span.first.1 + span.last.1 + 1) as f32 / 2.0,
        ) * TILE_SIZE;
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
