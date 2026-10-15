//! Things that hang high above a combat room, stretched between two walls: grates, wooden
//! beams, chains, pipes, stone arches, thick webs and metal walkways. They span a gap between
//! two wall blocks in one row, so the player walks underneath them.

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
