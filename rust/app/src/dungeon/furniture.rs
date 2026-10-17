//! What fills some combat rooms: a rug on the floor, and a table or two the player can flip over
//! for cover. Kept to a minority of rooms, so a room with them stands out.

use std::collections::HashSet;

use crate::rng::Rng;
use crate::tilemap::TileGrid;

use super::build::DungeonRoom;
use super::furnish::Carpet;
use super::surface::Surfaces;
use super::template::RoomKind;

const RUG_PERCENT: usize = 35;
const TABLE_PERCENT: usize = 45;
const SECOND_TABLE_PERCENT: usize = 35;
const DOOR_CLEARANCE: usize = 3;
/// Tables keep this many tiles apart, so each is its own bit of cover.
const TABLE_SPACING: usize = 3;
const ATTEMPTS: usize = 60;
/// A turned table, standing lengthwise down the room, in this share of tables.
const UPRIGHT_PERCENT: usize = 45;

/// A two-tile table standing over `tile` and the tile to its right, or (`upright`) the tile
/// below it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TableSpot {
    pub(crate) tile: (usize, usize),
    pub(crate) upright: bool,
}

impl TableSpot {
    pub(crate) fn tiles(self) -> [(usize, usize); 2] {
        let (column, row) = self.tile;
        if self.upright {
            [(column, row), (column, row + 1)]
        } else {
            [(column, row), (column + 1, row)]
        }
    }
}

struct Floor<'a> {
    grid: &'a TileGrid,
    room: &'a DungeonRoom,
    surfaces: &'a Surfaces,
    taken: HashSet<(usize, usize)>,
}

impl Floor<'_> {
    fn open(&self, column: usize, row: usize) -> bool {
        self.room.contains_inner_tile(column, row, 1)
            && !self.grid.is_wall(column as isize, row as isize)
            && self.surfaces.at((column, row)).is_none()
            && !self.taken.contains(&(column, row))
            && self.room.doors.iter().all(|door| {
                door.tiles
                    .iter()
                    .all(|&(c, r)| c.abs_diff(column) + r.abs_diff(row) >= DOOR_CLEARANCE)
            })
    }

    /// Open floor with nothing solid in the ring of tiles around it either.
    fn roomy(&self, column: usize, row: usize) -> bool {
        self.open(column, row)
            && (-1isize..=1).all(|dy| {
                (-1isize..=1).all(|dx| !self.grid.is_wall(column as isize + dx, row as isize + dy))
            })
    }
}

pub(crate) fn plan_rug(
    grid: &TileGrid,
    room: &DungeonRoom,
    surfaces: &Surfaces,
    style: usize,
    rng: &mut Rng,
) -> Option<Carpet> {
    if !matches!(room.kind, RoomKind::Normal | RoomKind::Ambush) || rng.below(100) >= RUG_PERCENT {
        return None;
    }
    let floor = Floor {
        grid,
        room,
        surfaces,
        taken: HashSet::new(),
    };
    for _ in 0..ATTEMPTS {
        let columns = 4 + rng.below(5);
        let rows = 3 + rng.below(3);
        let origin = (
            room.origin.0 + rng.below(room.columns.saturating_sub(columns).max(1)),
            room.origin.1 + rng.below(room.rows.saturating_sub(rows).max(1)),
        );
        let fits = (0..rows).all(|dy| {
            (0..columns).all(|dx| floor.roomy(origin.0 + dx, origin.1 + dy))
        });
        if fits {
            return Some(Carpet {
                origin,
                columns,
                rows,
                style,
            });
        }
    }
    None
}

pub(crate) fn plan_tables(
    grid: &TileGrid,
    room: &DungeonRoom,
    surfaces: &Surfaces,
    rng: &mut Rng,
) -> Vec<TableSpot> {
    if !matches!(room.kind, RoomKind::Normal | RoomKind::Ambush)
        || rng.below(100) >= TABLE_PERCENT
    {
        return Vec::new();
    }
    let wanted = if rng.below(100) < SECOND_TABLE_PERCENT { 2 } else { 1 };
    let floor = Floor {
        grid,
        room,
        surfaces,
        taken: room.decor.iter().map(|decor| decor.tile).collect(),
    };
    // A table needs room to fall either way: open floor along both of its long sides too.
    let fits = |spot: TableSpot| {
        spot.tiles().iter().all(|&(column, row)| {
            let across: [(isize, isize); 3] = if spot.upright {
                [(-1, 0), (0, 0), (1, 0)]
            } else {
                [(0, -1), (0, 0), (0, 1)]
            };
            across.iter().all(|&(dx, dy)| {
                let (c, r) = (column as isize + dx, row as isize + dy);
                c >= 0 && r >= 0 && floor.roomy(c as usize, r as usize)
            }) && !room.spawns.contains(&(column, row))
        })
    };
    let mut spots: Vec<(usize, usize)> = (room.origin.1..room.origin.1 + room.rows)
        .flat_map(|row| (room.origin.0..room.origin.0 + room.columns).map(move |column| (column, row)))
        .collect();
    let mut planned = Vec::new();
    while planned.len() < wanted && !spots.is_empty() {
        let tile = spots.swap_remove(rng.below(spots.len()));
        let upright = rng.below(100) < UPRIGHT_PERCENT;
        let spot = [upright, !upright]
            .into_iter()
            .map(|upright| TableSpot { tile, upright })
            .find(|&spot| fits(spot));
        let Some(spot) = spot else {
            continue;
        };
        spots.retain(|other| {
            other.0.abs_diff(tile.0) > TABLE_SPACING || other.1.abs_diff(tile.1) > TABLE_SPACING
        });
        planned.push(spot);
    }
    planned
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::{Dungeon, Theme};

    #[test]
    fn some_combat_rooms_get_a_rug_and_tables_on_open_floor() {
        let (mut combat, mut rugs, mut tabled) = (0, 0, 0);
        for seed in 0..24 {
            let dungeon =
                Dungeon::generate_themed(&mut Rng::new(seed), Theme::Stone).expect("a dungeon");
            for room in &dungeon.rooms {
                let combat_room = matches!(room.kind, RoomKind::Normal | RoomKind::Ambush);
                combat += usize::from(combat_room);
                if let Some(rug) = &room.rug {
                    assert!(combat_room);
                    rugs += 1;
                    for ((column, row), _) in rug.tiles() {
                        assert!(!dungeon.grid.is_wall(column as isize, row as isize));
                        assert!(dungeon.surfaces.at((column, row)).is_none());
                    }
                }
                tabled += usize::from(!room.tables.is_empty());
                assert!(room.tables.len() <= 2);
                for table in &room.tables {
                    assert!(combat_room);
                    for (column, row) in table.tiles() {
                        for d in -1..=1 {
                            let (dx, dy) = if table.upright { (d, 0) } else { (0, d) };
                            assert!(!dungeon.grid.is_wall(column as isize + dx, row as isize + dy));
                        }
                        assert!(dungeon.surfaces.at((column, row)).is_none());
                        assert!(room.decor.iter().all(|decor| decor.tile != (column, row)));
                    }
                }
            }
        }
        let rug_share = rugs as f32 / combat as f32;
        let table_share = tabled as f32 / combat as f32;
        assert!((0.15..0.55).contains(&rug_share), "{rug_share}");
        assert!((0.2..0.65).contains(&table_share), "{table_share}");
    }
}
