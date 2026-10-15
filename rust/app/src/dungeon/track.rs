//! A mine track in some combat rooms: a rail over open floor that runs straight or turns once
//! or twice by a right angle, with a buffer stop at each end and a cart standing on it, which the
//! player can send rolling.

use std::collections::HashSet;

use crate::rng::Rng;
use crate::tilemap::TileGrid;

use super::build::DungeonRoom;
use super::surface::Surfaces;
use super::template::RoomKind;

const ROOM_PERCENT: usize = 30;
const MIN_LENGTH: usize = 8;
const MAX_LENGTH: usize = 20;
/// Rails keep this many tiles from the doors, so nobody walks into the room onto a cart.
const DOOR_CLEARANCE: usize = 3;
const ATTEMPTS: usize = 120;
/// How many times a track turns: none, once (most often) or twice.
const TURNS: [usize; 4] = [0, 1, 1, 2];

/// A rail through `path`, a row of tiles each next to the one before; the cart starts on the
/// tile at index `cart`, a straight piece, never an end or a turn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Track {
    pub(crate) path: Vec<(usize, usize)>,
    pub(crate) cart: usize,
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

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
    // A rail tile needs open floor all around it, so the cart never scrapes a wall.
    let usable = |column: usize, row: usize| {
        room.contains_inner_tile(column, row, 1)
            && (-1isize..=1).all(|dy| {
                (-1isize..=1).all(|dx| !grid.is_wall(column as isize + dx, row as isize + dy))
            })
            && surfaces.at((column, row)).is_none()
            && !taken.contains(&(column, row))
            && !near_door(column, row)
    };
    let open: Vec<(usize, usize)> = (room.origin.1..room.origin.1 + room.rows)
        .flat_map(|row| (room.origin.0..room.origin.0 + room.columns).map(move |column| (column, row)))
        .filter(|&(column, row)| usable(column, row))
        .collect();
    if open.is_empty() {
        return None;
    }
    for _ in 0..ATTEMPTS {
        let start = open[rng.below(open.len())];
        let turns = TURNS[rng.below(TURNS.len())];
        let mut direction = DIRECTIONS[rng.below(DIRECTIONS.len())];
        let mut path = vec![start];
        for leg in 0..=turns {
            if leg > 0 {
                // Turn left or right.
                direction = if rng.below(2) == 0 {
                    (-direction.1, direction.0)
                } else {
                    (direction.1, -direction.0)
                };
            }
            let length = if leg == 0 { 4 + rng.below(5) } else { 3 + rng.below(5) };
            for _ in 0..length {
                let &(column, row) = path.last().expect("a path has a start");
                let next = (column as isize + direction.0, row as isize + direction.1);
                if next.0 < 0 || next.1 < 0 {
                    break;
                }
                let next = (next.0 as usize, next.1 as usize);
                // Never back onto the rail or alongside it.
                let crowded = path[..path.len() - 1]
                    .iter()
                    .any(|tile| tile.0.abs_diff(next.0) + tile.1.abs_diff(next.1) <= 1);
                if !usable(next.0, next.1) || crowded || path.len() >= MAX_LENGTH {
                    break;
                }
                path.push(next);
            }
        }
        if path.len() >= MIN_LENGTH {
            // The cart starts on a straight piece, not in a turn.
            let straight: Vec<usize> = (1..path.len() - 1)
                .filter(|&index| {
                    let (before, here, after) = (path[index - 1], path[index], path[index + 1]);
                    before.0 + after.0 == 2 * here.0 && before.1 + after.1 == 2 * here.1
                })
                .collect();
            if let Some(&cart) = rng.pick(&straight) {
                return Some(Track { path, cart });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::{Dungeon, Theme};

    #[test]
    fn tracks_run_over_dry_open_floor_in_some_combat_rooms_and_some_turn() {
        let (mut combat, mut tracked, mut turning) = (0, 0, 0);
        for theme in [Theme::Stone, Theme::Lava, Theme::Sewer, Theme::Ice] {
            for seed in 0..12 {
                let dungeon =
                    Dungeon::generate_themed(&mut Rng::new(seed), theme).expect("a dungeon");
                for room in &dungeon.rooms {
                    if matches!(room.kind, RoomKind::Normal | RoomKind::Ambush) {
                        combat += 1;
                    }
                    let Some(track) = &room.track else {
                        continue;
                    };
                    tracked += 1;
                    assert!(matches!(room.kind, RoomKind::Normal | RoomKind::Ambush));
                    assert!((MIN_LENGTH..=MAX_LENGTH).contains(&track.path.len()));
                    assert!(track.cart > 0 && track.cart < track.path.len() - 1);
                    let (before, here, after) = (
                        track.path[track.cart - 1],
                        track.path[track.cart],
                        track.path[track.cart + 1],
                    );
                    assert!(before.0 + after.0 == 2 * here.0 && before.1 + after.1 == 2 * here.1);
                    for pair in track.path.windows(2) {
                        assert_eq!(pair[0].0.abs_diff(pair[1].0) + pair[0].1.abs_diff(pair[1].1), 1);
                    }
                    let unique: HashSet<_> = track.path.iter().collect();
                    assert_eq!(unique.len(), track.path.len());
                    let straight = track.path.iter().all(|tile| tile.1 == track.path[0].1)
                        || track.path.iter().all(|tile| tile.0 == track.path[0].0);
                    turning += usize::from(!straight);
                    for &(column, row) in &track.path {
                        let (c, r) = (column as isize, row as isize);
                        assert!((-1..=1).all(|dy| (-1..=1).all(|dx| !dungeon.grid.is_wall(c + dx, r + dy))));
                        assert!(dungeon.surfaces.at((column, row)).is_none());
                        assert!(room.decor.iter().all(|decor| decor.tile != (column, row)));
                    }
                }
            }
        }
        let share = tracked as f32 / combat as f32;
        assert!((0.1..0.4).contains(&share), "{share}");
        assert!(turning > 0);
    }
}
