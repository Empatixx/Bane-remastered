//! Things that hang high above a combat room, stretched between two walls: grates, wooden
//! beams, chains, pipes, stone arches, thick webs and metal walkways. They span a gap between
//! two wall blocks in one row, so the player walks underneath them. Others hang from the
//! ceiling on a chain or a thread over open floor: chandeliers, cages, cocoons, lanterns, vines.

use crate::rng::Rng;
use crate::tilemap::TileGrid;

use super::build::DungeonRoom;
use super::template::RoomKind;
use super::theme::Theme;

const MIN_SPAN: usize = 3;
const MAX_SPAN: usize = 9;
const MAX_PER_ROOM: usize = 2;
const ROOM_CHANCE_PERCENT: usize = 75;
/// Rows kept between two spans in one room, so they never stack into a ceiling.
const ROW_GAP: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OverheadKind {
    Grate,
    Beams,
    Chain,
    Pipes,
    Arch,
    Web,
    Walkway,
}

impl OverheadKind {
    /// The kind's row in the overhead sheet.
    pub(crate) fn row(self) -> usize {
        match self {
            Self::Grate => 0,
            Self::Beams => 1,
            Self::Chain => 2,
            Self::Pipes => 3,
            Self::Arch => 4,
            Self::Web => 5,
            Self::Walkway => 6,
        }
    }

    /// How far the middle hangs down (positive) or rises (negative), in world units.
    pub(crate) fn sag(self) -> f32 {
        match self {
            Self::Chain => 18.0,
            Self::Web => 12.0,
            Self::Arch => -20.0,
            _ => 0.0,
        }
    }
}

impl Theme {
    pub(crate) fn overhead_kinds(self) -> &'static [OverheadKind] {
        use OverheadKind::*;
        match self {
            Self::Stone => &[Grate, Beams, Chain],
            Self::Temple => &[Arch, Arch, Chain, Beams],
            Self::Moss => &[Beams, Chain],
            Self::Sewer => &[Pipes, Pipes, Grate],
            Self::Spider => &[Web, Web, Beams],
            Self::Ice => &[Chain, Grate],
            Self::Lava => &[Walkway, Walkway, Chain],
        }
    }
}

/// One thing hanging over the floor tiles `left..=right` of `row`, with a wall at each end.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Overhead {
    pub(crate) kind: OverheadKind,
    pub(crate) row: usize,
    pub(crate) left: usize,
    pub(crate) right: usize,
}

/// The gaps between two walls in a row of the room that something could hang across. Of the
/// rows a wall block spans, only its top one is kept, so a span sits on top of the blocks.
fn spans(grid: &TileGrid, room: &DungeonRoom) -> Vec<(usize, usize, usize)> {
    let wall = |column: usize, row: usize| grid.is_wall(column as isize, row as isize);
    let mut found = Vec::new();
    for row in room.origin.1 + 1..room.origin.1 + room.rows - 1 {
        let mut column = room.origin.0 + 1;
        while column < room.origin.0 + room.columns - 1 {
            if wall(column, row) || !wall(column - 1, row) {
                column += 1;
                continue;
            }
            let left = column;
            while column < room.origin.0 + room.columns - 1 && !wall(column, row) {
                column += 1;
            }
            let right = column - 1;
            let length = right + 1 - left;
            let closed = wall(right + 1, row);
            let on_top = !(wall(left - 1, row - 1) && wall(right + 1, row - 1));
            if closed && on_top && (MIN_SPAN..=MAX_SPAN).contains(&length) {
                found.push((row, left, right));
            }
        }
    }
    found
}

pub(crate) fn plan(grid: &TileGrid, room: &DungeonRoom, theme: Theme, rng: &mut Rng) -> Vec<Overhead> {
    if !matches!(room.kind, RoomKind::Normal | RoomKind::Ambush)
        || rng.below(100) >= ROOM_CHANCE_PERCENT
    {
        return Vec::new();
    }
    let mut open = spans(grid, room);
    let kinds = theme.overhead_kinds();
    let mut planned: Vec<Overhead> = Vec::new();
    while planned.len() < MAX_PER_ROOM && !open.is_empty() {
        let (row, left, right) = open.swap_remove(rng.below(open.len()));
        if planned
            .iter()
            .any(|other| other.row.abs_diff(row) < ROW_GAP)
        {
            continue;
        }
        let kind = kinds[rng.below(kinds.len())];
        planned.push(Overhead {
            kind,
            row,
            left,
            right,
        });
    }
    planned
}

const HANGING_CLEARANCE: isize = 2;
const HANGING_SPACING: usize = 4;
const MAX_HANGING: usize = 2;
/// Only some combat rooms have anything hanging, so it stays a detail rather than a pattern.
const HANGING_ROOM_PERCENT: usize = 35;
/// Rooms at least this big (in tiles) may get a second one.
const ROOMY_FOR_TWO: usize = 500;
const SECOND_HANGING_PERCENT: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HangingKind {
    Chandelier,
    Cage,
    Cocoon,
    Lantern,
    Vine,
}

impl HangingKind {
    /// The kind's row in the hanging sheet.
    pub(crate) fn row(self) -> usize {
        match self {
            Self::Chandelier => 0,
            Self::Cage => 1,
            Self::Cocoon => 2,
            Self::Lantern => 3,
            Self::Vine => 4,
        }
    }

    /// Candles and lanterns flicker between their two pictures and light the room.
    pub(crate) fn glows(self) -> bool {
        matches!(self, Self::Chandelier | Self::Lantern)
    }
}

impl Theme {
    pub(crate) fn hanging_kinds(self) -> &'static [HangingKind] {
        use HangingKind::*;
        match self {
            Self::Stone => &[Chandelier, Cage],
            Self::Temple => &[Chandelier, Chandelier, Cage],
            Self::Moss => &[Vine, Vine, Lantern],
            Self::Sewer => &[Lantern, Cage],
            Self::Spider => &[Cocoon, Cocoon, Cage],
            Self::Ice => &[Lantern, Chandelier],
            Self::Lava => &[Cage, Lantern],
        }
    }
}

/// Something hanging from the ceiling above the floor tile `tile`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Hanging {
    pub(crate) kind: HangingKind,
    pub(crate) tile: (usize, usize),
}

/// Now and then something hanging over the open floor of a combat room (a second one only in a
/// big room), clear of the walls, of each other and of the spans already lying across the room.
pub(crate) fn plan_hanging(
    grid: &TileGrid,
    room: &DungeonRoom,
    theme: Theme,
    spans: &[Overhead],
    rng: &mut Rng,
) -> Vec<Hanging> {
    if !matches!(room.kind, RoomKind::Normal | RoomKind::Ambush)
        || rng.below(100) >= HANGING_ROOM_PERCENT
    {
        return Vec::new();
    }
    let wanted = if room.columns * room.rows >= ROOMY_FOR_TWO
        && rng.below(100) < SECOND_HANGING_PERCENT
    {
        MAX_HANGING
    } else {
        1
    };
    let clear = |column: usize, row: usize| {
        (-HANGING_CLEARANCE..=HANGING_CLEARANCE).all(|dy| {
            (-HANGING_CLEARANCE..=HANGING_CLEARANCE)
                .all(|dx| !grid.is_wall(column as isize + dx, row as isize + dy))
        })
    };
    let under_span = |column: usize, row: usize| {
        spans.iter().any(|span| {
            row.abs_diff(span.row) <= 2 && column + 1 >= span.left && column <= span.right + 1
        })
    };
    let mut open: Vec<(usize, usize)> = (room.origin.1..room.origin.1 + room.rows)
        .flat_map(|row| (room.origin.0..room.origin.0 + room.columns).map(move |column| (column, row)))
        .filter(|&(column, row)| clear(column, row) && !under_span(column, row))
        .collect();
    let kinds = theme.hanging_kinds();
    let mut planned = Vec::new();
    for _ in 0..wanted {
        if open.is_empty() {
            break;
        }
        let tile = open.swap_remove(rng.below(open.len()));
        open.retain(|other| {
            other.0.abs_diff(tile.0) >= HANGING_SPACING || other.1.abs_diff(tile.1) >= HANGING_SPACING
        });
        planned.push(Hanging {
            kind: kinds[rng.below(kinds.len())],
            tile,
        });
    }
    planned
}

/// One of every hanging kind in a row across the middle of a room, whatever its biome, for
/// looking at them all at once (`BANE_SHOWCASE`).
pub(crate) fn showcase(grid: &TileGrid, room: &DungeonRoom) -> Vec<Hanging> {
    use HangingKind::*;
    let kinds = [Chandelier, Cage, Cocoon, Lantern, Vine];
    let clear = |column: usize, row: usize| {
        (-1isize..=1).all(|dy| (-1isize..=1).all(|dx| !grid.is_wall(column as isize + dx, row as isize + dy)))
    };
    // Low enough in the room that what hangs above the row is still inside it.
    let middle = room.origin.1 + room.rows / 2 + 2;
    let rows = (0..room.rows / 2)
        .flat_map(|step| [middle + step, middle.saturating_sub(step)])
        .filter(|&row| row >= room.origin.1 + 4 && row + 2 < room.origin.1 + room.rows);
    for row in rows {
        let columns: Vec<usize> = (room.origin.0..room.origin.0 + room.columns)
            .filter(|&column| clear(column, row))
            .collect();
        let mut picked: Vec<usize> = Vec::new();
        for column in columns {
            if picked.last().is_none_or(|last| column >= last + 3) {
                picked.push(column);
            }
        }
        if picked.len() >= kinds.len() {
            let start = (picked.len() - kinds.len()) / 2;
            return kinds
                .iter()
                .zip(&picked[start..])
                .map(|(&kind, &column)| Hanging {
                    kind,
                    tile: (column, row),
                })
                .collect();
        }
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::Dungeon;

    const ALL: [Theme; 7] = [
        Theme::Stone,
        Theme::Sewer,
        Theme::Moss,
        Theme::Spider,
        Theme::Ice,
        Theme::Lava,
        Theme::Temple,
    ];

    #[test]
    fn spans_hang_between_two_walls_over_open_floor_in_combat_rooms_only() {
        let mut total = 0;
        for theme in ALL {
            for seed in 0..6 {
                let dungeon =
                    Dungeon::generate_themed(&mut Rng::new(seed), theme).expect("a dungeon");
                let grid = &dungeon.grid;
                for room in &dungeon.rooms {
                    if !room.overheads.is_empty() {
                        assert!(matches!(room.kind, RoomKind::Normal | RoomKind::Ambush));
                    }
                    assert!(room.overheads.len() <= MAX_PER_ROOM);
                    for span in &room.overheads {
                        total += 1;
                        let row = span.row as isize;
                        assert!(grid.is_wall(span.left as isize - 1, row));
                        assert!(grid.is_wall(span.right as isize + 1, row));
                        assert!((span.left..=span.right).all(|c| !grid.is_wall(c as isize, row)));
                        assert!((MIN_SPAN..=MAX_SPAN).contains(&(span.right + 1 - span.left)));
                        assert!(theme.overhead_kinds().contains(&span.kind));
                        assert!(room.contains_tile(span.left, span.row));
                    }
                }
            }
        }
        assert!(total > 20, "{total}");
    }

    #[test]
    fn hanging_things_float_over_open_floor_apart_from_each_other() {
        let mut total = 0;
        for theme in ALL {
            for seed in 0..4 {
                let dungeon =
                    Dungeon::generate_themed(&mut Rng::new(seed), theme).expect("a dungeon");
                for room in &dungeon.rooms {
                    if !room.hanging.is_empty() {
                        assert!(matches!(room.kind, RoomKind::Normal | RoomKind::Ambush));
                    }
                    assert!(room.hanging.len() <= MAX_HANGING);
                    for (index, hanging) in room.hanging.iter().enumerate() {
                        total += 1;
                        let (c, r) = (hanging.tile.0 as isize, hanging.tile.1 as isize);
                        assert!((-2..=2).all(|dy| (-2..=2).all(|dx| !dungeon.grid.is_wall(c + dx, r + dy))));
                        assert!(theme.hanging_kinds().contains(&hanging.kind));
                        for other in &room.hanging[index + 1..] {
                            assert!(
                                other.tile.0.abs_diff(hanging.tile.0) >= HANGING_SPACING
                                    || other.tile.1.abs_diff(hanging.tile.1) >= HANGING_SPACING
                            );
                        }
                    }
                }
            }
        }
        assert!(total > 10, "{total}");
    }

    #[test]
    fn only_some_rooms_have_anything_hanging() {
        let (mut rooms, mut hung) = (0, 0);
        for seed in 0..30 {
            let dungeon =
                Dungeon::generate_themed(&mut Rng::new(seed), Theme::Moss).expect("a dungeon");
            for room in &dungeon.rooms {
                if matches!(room.kind, RoomKind::Normal | RoomKind::Ambush) {
                    rooms += 1;
                    hung += usize::from(!room.hanging.is_empty());
                }
            }
        }
        let share = hung as f32 / rooms as f32;
        assert!((0.15..0.55).contains(&share), "{share}");
    }

    #[test]
    fn two_spans_in_a_room_keep_apart() {
        for seed in 0..20 {
            let dungeon =
                Dungeon::generate_themed(&mut Rng::new(seed), Theme::Stone).expect("a dungeon");
            for room in &dungeon.rooms {
                if let [a, b] = room.overheads.as_slice() {
                    assert!(a.row.abs_diff(b.row) >= ROW_GAP);
                }
            }
        }
    }
}
