//! Where a floor starts for working on the game: the `BANE_SHOWCASE`, `BANE_AT` and
//! `BANE_OFFSET` switches, read from the environment. None of them is set in a normal run.

use glam::Vec2;

use crate::tilemap::TILE_SIZE;

use super::{DecorKind, Dungeon, DungeonRoom, RoomKind};

/// `BANE_SHOWCASE=1` hangs one of every kind across the biggest normal room, for looking at
/// them all at once, and gives the spot to start the player under them.
pub(crate) fn showcase(dungeon: &mut Dungeon) -> Option<Vec2> {
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
            room.hanging = super::showcase(&grid, room, theme);
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
    showcase_start
}

/// Where the floor starts for working on the game: `start` moved to the spot `BANE_AT` names,
/// or to the `showcase` spot, then by `BANE_OFFSET`. `start` itself when none is set.
pub(crate) fn start_at(dungeon: &Dungeon, mut start: Vec2, showcase: Option<Vec2>) -> Vec2 {
    // `BANE_AT=boss`, `normal`, `treasure`, `shop`, `sacrifice`, `overhead`, `hanging`, `track`,
    // `table`, `rug`, `vestibule`, `water`, `surface` (the thickest patch of lava, water, sand
    // and such), `bowl`, `statue`, `shackles` or `pendulum` starts the floor there (by the first one of those), for working on the
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
                .and_then(|room| open_spot_near_center(dungeon, room))
            {
                start = spot;
            }
        }
        Ok("surface") => {
            let tiles: Vec<(usize, usize)> = dungeon.surfaces.iter().map(|(tile, _)| tile).collect();
            let around = |tile: (usize, usize)| {
                tiles.iter().filter(|other| other.0.abs_diff(tile.0) <= 2 && other.1.abs_diff(tile.1) <= 2).count()
            };
            if let Some(&tile) = tiles.iter().max_by_key(|&&tile| (around(tile), std::cmp::Reverse(tile))) {
                start = (Vec2::new(tile.0 as f32, tile.1 as f32) + 0.5) * TILE_SIZE;
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
    if let Some(spot) = showcase {
        start = spot;
    }
    // `BANE_OFFSET=dx,dy` moves that start by whole tiles, for looking round a room.
    if let Some((dx, dy)) = std::env::var("BANE_OFFSET").ok().and_then(|offset| {
        let (dx, dy) = offset.split_once(',')?;
        Some((dx.trim().parse::<f32>().ok()?, dy.trim().parse::<f32>().ok()?))
    }) {
        start += Vec2::new(dx, dy) * TILE_SIZE;
    }
    start
}

/// The open floor tile closest to the middle of a room, with no wall around it.
fn open_spot_near_center(dungeon: &Dungeon, room: &DungeonRoom) -> Option<Vec2> {
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

#[cfg(test)]
mod tests {
    use crate::rng::Rng;

    use super::*;

    #[test]
    fn the_open_spot_of_a_room_is_inside_it_with_no_wall_around() {
        let dungeon = Dungeon::generate(&mut Rng::new(42)).expect("a dungeon is generated");
        for room in &dungeon.rooms {
            let Some(spot) = open_spot_near_center(&dungeon, room) else {
                continue;
            };
            let (column, row) = ((spot.x / TILE_SIZE) as isize, (spot.y / TILE_SIZE) as isize);
            assert!((room.origin.0 as isize..(room.origin.0 + room.columns) as isize).contains(&column));
            assert!((room.origin.1 as isize..(room.origin.1 + room.rows) as isize).contains(&row));
            for dy in -1..=1 {
                for dx in -1..=1 {
                    assert!(!dungeon.grid.is_wall(column + dx, row + dy));
                }
            }
        }
    }
}
