//! A mine track across some combat rooms: a straight rail over open floor with a buffer stop at
//! each end and a cart standing on it, which the player can send rolling.

use std::collections::HashSet;

use crate::rng::Rng;
use crate::tilemap::TileGrid;

use super::build::DungeonRoom;
use super::surface::Surfaces;
use super::template::RoomKind;

const ROOM_PERCENT: usize = 30;
const MIN_LENGTH: usize = 8;
const MAX_LENGTH: usize = 14;
/// Rails keep this many tiles from the doors, so nobody walks into the room onto a cart.
const DOOR_CLEARANCE: usize = 3;

/// A rail over the floor tiles `left..=right` of `row`, the cart starting over `cart`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Track {
    pub(crate) row: usize,
    pub(crate) left: usize,
    pub(crate) right: usize,
    pub(crate) cart: usize,
}

pub(crate) fn plan(
    grid: &TileGrid,
    room: &DungeonRoom,
    surfaces: &Surfaces,
    rng: &mut Rng,
) -> Option<Track> {
    if !matches!(room.kind, RoomKind::Normal | RoomKind::Ambush) || rng.below(100) >= ROOM_PERCENT
    {
        return None;
    }
    let taken: HashSet<(usize, usize)> = room.decor.iter().map(|decor| decor.tile).collect();
    let near_door = |column: usize, row: usize| {
        room.doors.iter().any(|door| {
            door.tiles
                .iter()
                .any(|&(c, r)| c.abs_diff(column) + r.abs_diff(row) < DOOR_CLEARANCE)
        })
    };
    // A rail tile needs open floor around it, so the cart (a tile tall) never scrapes a wall.
    let usable = |column: usize, row: usize| {
        room.contains_inner_tile(column, row, 1)
            && (-1isize..=1).all(|dy| !grid.is_wall(column as isize, row as isize + dy))
            && surfaces.at((column, row)).is_none()
            && !taken.contains(&(column, row))
            && !near_door(column, row)
    };
    let mut runs = Vec::new();
    for row in room.origin.1..room.origin.1 + room.rows {
        let mut column = room.origin.0;
        while column < room.origin.0 + room.columns {
            if !usable(column, row) {
                column += 1;
                continue;
            }
            let left = column;
            while column < room.origin.0 + room.columns && usable(column, row) {
                column += 1;
            }
            if column - left >= MIN_LENGTH {
                runs.push((row, left, column - 1));
            }
        }
    }
    let &(row, mut left, mut right) = rng.pick(&runs)?;
    while right + 1 - left > MAX_LENGTH {
        if rng.below(2) == 0 {
            left += 1;
        } else {
            right -= 1;
        }
    }
    let cart = left + 1 + rng.below(right - left - 1);
    Some(Track {
        row,
        left,
        right,
        cart,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::{Dungeon, Theme};

    #[test]
    fn tracks_run_straight_over_dry_open_floor_in_some_combat_rooms() {
        let (mut combat, mut tracked) = (0, 0);
        for theme in [Theme::Stone, Theme::Lava, Theme::Sewer, Theme::Ice] {
            for seed in 0..12 {
                let dungeon =
                    Dungeon::generate_themed(&mut Rng::new(seed), theme).expect("a dungeon");
                for room in &dungeon.rooms {
                    if matches!(room.kind, RoomKind::Normal | RoomKind::Ambush) {
                        combat += 1;
                    }
                    let Some(track) = room.track else {
                        continue;
                    };
                    tracked += 1;
                    assert!(matches!(room.kind, RoomKind::Normal | RoomKind::Ambush));
                    let length = track.right + 1 - track.left;
                    assert!((MIN_LENGTH..=MAX_LENGTH).contains(&length));
                    assert!(track.cart > track.left && track.cart < track.right);
                    for column in track.left..=track.right {
                        let (c, r) = (column as isize, track.row as isize);
                        assert!((-1..=1).all(|dy| !dungeon.grid.is_wall(c, r + dy)));
                        assert!(dungeon.surfaces.at((column, track.row)).is_none());
                        assert!(room.decor.iter().all(|decor| decor.tile != (column, track.row)));
                    }
                }
            }
        }
        let share = tracked as f32 / combat as f32;
        assert!((0.1..0.4).contains(&share), "{share}");
    }
}
