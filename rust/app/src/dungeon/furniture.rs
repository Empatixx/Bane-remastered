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

/// A rug's span, in tiles, at least and at most.
const RUG_COLUMNS: (usize, usize) = (6, 10);
const RUG_ROWS: (usize, usize) = (5, 7);
/// How much the noise roughens a rug's oval, and the fewest tiles a rug may have.
const RUG_ROUGHNESS: f32 = 1.6;
/// How far apart, in tiles, the noise's lobes are.
const RUG_LOBE: usize = 3;
const RUG_FEWEST: usize = 10;

/// A rug for a combat room, in some of them: a shape on the open floor rather than a box, grown
/// from an oval roughened by noise as the water's pools are, then cleaned up so it holds
/// together. No strips a tile thin (every tile of it is in a two-by-two of it), all in one piece,
/// and no holes.
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
        let columns = RUG_COLUMNS.0 + rng.below(RUG_COLUMNS.1 - RUG_COLUMNS.0 + 1);
        let rows = RUG_ROWS.0 + rng.below(RUG_ROWS.1 - RUG_ROWS.0 + 1);
        let origin = (
            room.origin.0 + rng.below(room.columns.saturating_sub(columns).max(1)),
            room.origin.1 + rng.below(room.rows.saturating_sub(rows).max(1)),
        );
        let open = |column: usize, row: usize| floor.roomy(origin.0 + column, origin.1 + row);
        let shape = rug_shape(columns, rows, rng);
        if let Some(shape) = settle(shape, columns, rows, open) {
            let cover = shape.iter().enumerate().filter(|(_, &on)| on).fold(0u128, |cover, (index, _)| cover | 1 << index);
            return Some(Carpet { origin, columns, rows, style, cover: Some(cover) });
        }
    }
    None
}

/// An oval over `columns` by `rows` tiles, roughened by smoothed noise: row by row, whether
/// each tile is in it.
fn rug_shape(columns: usize, rows: usize, rng: &mut Rng) -> Vec<bool> {
    // Value noise: random heights on a coarse lattice, blended between, so it makes lobes and
    // bays a few tiles across rather than specks.
    let lattice = (columns / RUG_LOBE + 2, rows / RUG_LOBE + 2);
    let heights: Vec<f32> = (0..lattice.0 * lattice.1).map(|_| rng.below(1000) as f32 / 1000.0).collect();
    let height = |x: usize, y: usize| heights[y * lattice.0 + x];
    let middle = ((columns as f32 - 1.0) / 2.0, (rows as f32 - 1.0) / 2.0);
    let reach = (columns as f32 / 2.0, rows as f32 / 2.0);
    let mut shape = Vec::with_capacity(columns * rows);
    for row in 0..rows {
        for column in 0..columns {
            let (fx, fy) = (column as f32 / RUG_LOBE as f32, row as f32 / RUG_LOBE as f32);
            let (x0, y0) = (fx as usize, fy as usize);
            let (tx, ty) = (fx - x0 as f32, fy - y0 as f32);
            let top = height(x0, y0) + (height(x0 + 1, y0) - height(x0, y0)) * tx;
            let bottom = height(x0, y0 + 1) + (height(x0 + 1, y0 + 1) - height(x0, y0 + 1)) * tx;
            let noise = top + (bottom - top) * ty;
            let x = (column as f32 - middle.0) / reach.0;
            let y = (row as f32 - middle.1) / reach.1;
            shape.push(x * x + y * y + (noise - 0.5) * RUG_ROUGHNESS < 0.9);
        }
    }
    shape
}

/// `shape` made to hold together on the floor, or `None` if too little of it is left: only on
/// open floor, no strips a tile thin, one piece, no holes.
fn settle(mut shape: Vec<bool>, columns: usize, rows: usize, open: impl Fn(usize, usize) -> bool) -> Option<Vec<bool>> {
    let index = |column: usize, row: usize| row * columns + column;
    for row in 0..rows {
        for column in 0..columns {
            if !open(column, row) {
                shape[index(column, row)] = false;
            }
        }
    }
    let on = |shape: &[bool], column: isize, row: isize| {
        column >= 0 && row >= 0 && (column as usize) < columns && (row as usize) < rows && shape[index(column as usize, row as usize)]
    };
    // Thin bits go: a tile stays only while some two-by-two of the shape holds it.
    loop {
        let thin: Vec<usize> = (0..rows as isize)
            .flat_map(|row| (0..columns as isize).map(move |column| (column, row)))
            .filter(|&(column, row)| on(&shape, column, row))
            .filter(|&(column, row)| {
                ![(-1, -1), (0, -1), (-1, 0), (0, 0)].iter().any(|&(dx, dy)| {
                    [(0, 0), (1, 0), (0, 1), (1, 1)].iter().all(|&(ox, oy)| on(&shape, column + dx + ox, row + dy + oy))
                })
            })
            .map(|(column, row)| index(column as usize, row as usize))
            .collect();
        if thin.is_empty() {
            break;
        }
        for tile in thin {
            shape[tile] = false;
        }
    }
    // One piece: the biggest.
    let mut piece = vec![usize::MAX; shape.len()];
    let mut sizes = Vec::new();
    for start in 0..shape.len() {
        if !shape[start] || piece[start] != usize::MAX {
            continue;
        }
        let label = sizes.len();
        let mut stack = vec![start];
        piece[start] = label;
        let mut size = 0;
        while let Some(tile) = stack.pop() {
            size += 1;
            let (column, row) = ((tile % columns) as isize, (tile / columns) as isize);
            for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                if on(&shape, column + dx, row + dy) {
                    let next = index((column + dx) as usize, (row + dy) as usize);
                    if piece[next] == usize::MAX {
                        piece[next] = label;
                        stack.push(next);
                    }
                }
            }
        }
        sizes.push(size);
    }
    let biggest = (0..sizes.len()).max_by_key(|&label| sizes[label])?;
    for tile in 0..shape.len() {
        shape[tile] = piece[tile] == biggest;
    }
    // No holes: what the outside cannot reach is filled in, if the floor there is open.
    let mut outside = vec![false; shape.len()];
    let mut stack: Vec<usize> = (0..shape.len())
        .filter(|&tile| {
            let (column, row) = (tile % columns, tile / columns);
            (column == 0 || row == 0 || column + 1 == columns || row + 1 == rows) && !shape[tile]
        })
        .collect();
    for &tile in &stack {
        outside[tile] = true;
    }
    while let Some(tile) = stack.pop() {
        let (column, row) = ((tile % columns) as isize, (tile / columns) as isize);
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let (x, y) = (column + dx, row + dy);
            if x >= 0 && y >= 0 && (x as usize) < columns && (y as usize) < rows {
                let next = index(x as usize, y as usize);
                if !shape[next] && !outside[next] {
                    outside[next] = true;
                    stack.push(next);
                }
            }
        }
    }
    for tile in 0..shape.len() {
        if !shape[tile] && !outside[tile] {
            if !open(tile % columns, tile / columns) {
                return None;
            }
            shape[tile] = true;
        }
    }
    (shape.iter().filter(|&&on| on).count() >= RUG_FEWEST).then_some(shape)
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

    #[test]
    fn rugs_are_shapes_in_one_piece_without_holes_or_thin_strips() {
        let mut rng = Rng::new(7);
        let open = |_: usize, _: usize| true;
        let mut shapes = 0;
        let mut not_boxes = 0;
        for _ in 0..200 {
            let (columns, rows) = (5 + rng.below(5), 4 + rng.below(3));
            let Some(shape) = settle(rug_shape(columns, rows, &mut rng), columns, rows, open) else { continue };
            shapes += 1;
            let carpet = Carpet {
                origin: (0, 0),
                columns,
                rows,
                style: 0,
                cover: Some(shape.iter().enumerate().filter(|(_, &on)| on).fold(0, |cover, (index, _)| cover | 1 << index)),
            };
            let covered = carpet.tiles();
            if covered.len() < columns * rows {
                not_boxes += 1;
            }
            for &((column, row), _) in &covered {
                let (column, row) = (column as isize, row as isize);
                let in_square = [(-1, -1), (0, -1), (-1, 0), (0, 0)].iter().any(|&(dx, dy)| {
                    [(0, 0), (1, 0), (0, 1), (1, 1)].iter().all(|&(ox, oy)| carpet.covers(column + dx + ox, row + dy + oy))
                });
                assert!(in_square, "a strip a tile thin at {column},{row}");
            }
            // One piece, and the floor outside reaches every tile it does not cover.
            let reach = |start: (isize, isize), inside: bool| {
                let mut seen = HashSet::from([start]);
                let mut stack = vec![start];
                while let Some((column, row)) = stack.pop() {
                    for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                        let next = (column + dx, row + dy);
                        let within = next.0 >= -1 && next.1 >= -1 && next.0 <= columns as isize && next.1 <= rows as isize;
                        if within && carpet.covers(next.0, next.1) == inside && seen.insert(next) {
                            stack.push(next);
                        }
                    }
                }
                seen.len()
            };
            let first = covered[0].0;
            assert_eq!(reach((first.0 as isize, first.1 as isize), true), covered.len(), "in one piece");
            let around = (columns + 2) * (rows + 2) - covered.len();
            assert_eq!(reach((-1, -1), false), around, "no holes");
        }
        assert!(shapes > 150 && not_boxes > shapes / 2, "{shapes} rugs, {not_boxes} of them not boxes");
    }}
