//! Things that hang high above a combat room, stretched between two walls: grates, wooden
//! beams, chains, pipes, stone arches, thick webs, metal walkways and lianas. They span a gap between
//! two wall blocks in one row, or down the screen from one block to the block below it, so the
//! player walks underneath them. Others hang from the ceiling on a chain or a thread over open
//! floor: chandeliers, cages, cocoons, lanterns, vines.

use crate::rng::Rng;
use crate::tilemap::TileGrid;

use super::build::DungeonRoom;
use super::surface::{Surface, Surfaces};
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
    /// A liana slung between two blocks, sagging in the middle.
    Vines,
    /// A few strands of silk strung between two blocks.
    Strands,
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
            Self::Vines => 7,
            Self::Strands => 8,
        }
    }

    /// How far the middle hangs down (positive) or rises (negative), in world units.
    pub(crate) fn sag(self) -> f32 {
        match self {
            Self::Chain => 18.0,
            Self::Vines => 22.0,
            Self::Strands => 8.0,
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
            Self::Moss => &[Vines, Vines, Chain],
            Self::Sewer => &[Pipes, Pipes, Grate],
            Self::Spider => &[Web, Strands, Strands, Beams],
            Self::Ice => &[Chain, Grate],
            Self::Lava => &[Walkway, Walkway, Chain],
        }
    }
}

/// One thing hanging over the open floor tiles `first..=last` (a row of them, or a column of
/// them when `upright`), with a wall block at each end.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Overhead {
    pub(crate) kind: OverheadKind,
    /// Running down the screen, from the block above to the block below, rather than across.
    pub(crate) upright: bool,
    pub(crate) first: (usize, usize),
    pub(crate) last: (usize, usize),
}

impl Overhead {
    /// How many floor tiles it lies over.
    pub(crate) fn length(&self) -> usize {
        if self.upright {
            self.last.1 + 1 - self.first.1
        } else {
            self.last.0 + 1 - self.first.0
        }
    }

    /// Whether the tile is within `margin` tiles of the span or the blocks at its ends.
    pub(crate) fn near(&self, (column, row): (usize, usize), margin: usize) -> bool {
        let (x, y) = if self.upright {
            (margin, margin + 1)
        } else {
            (margin + 1, margin)
        };
        column + x >= self.first.0
            && column <= self.last.0 + x
            && row + y >= self.first.1
            && row <= self.last.1 + y
    }

    pub(crate) fn tiles(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let step = usize::from(self.upright);
        (0..self.length()).map(move |i| (self.first.0 + i * (1 - step), self.first.1 + i * step))
    }
}

/// A gap something could lie across: whether it runs down the screen, and its first and last
/// open tiles.
type Gap = (bool, (usize, usize), (usize, usize));

/// The gaps between two walls in a row of the room that something could hang across. Of the
/// rows a wall block spans, only its top one is kept, so a span sits on top of the blocks.
fn spans(grid: &TileGrid, room: &DungeonRoom) -> Vec<Gap> {
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
            // Both ends rest on a block standing in the room: the room's own walls show no
            // top to lie on, only their thin edge running down the screen.
            let on_blocks = room.contains_inner_tile(left - 1, row, 1)
                && room.contains_inner_tile(right + 1, row, 1);
            if closed && on_top && on_blocks && (MIN_SPAN..=MAX_SPAN).contains(&length) {
                found.push((false, (left, row), (right, row)));
            }
        }
    }
    found.extend(upright_spans(grid, room));
    found
}

/// The gaps between a block and the block below it in a column of the room, for something to
/// run down the screen from the top of one to the top of the other.
fn upright_spans(grid: &TileGrid, room: &DungeonRoom) -> Vec<Gap> {
    let wall = |column: usize, row: usize| grid.is_wall(column as isize, row as isize);
    let mut found = Vec::new();
    for column in room.origin.0 + 1..room.origin.0 + room.columns - 1 {
        let mut row = room.origin.1 + 2;
        while row < room.origin.1 + room.rows - 1 {
            if wall(column, row) || !wall(column, row - 1) {
                row += 1;
                continue;
            }
            let top = row;
            while row < room.origin.1 + room.rows - 1 && !wall(column, row) {
                row += 1;
            }
            let bottom = row - 1;
            let length = bottom + 1 - top;
            let closed = wall(column, bottom + 1);
            // The block above shows its top above its front face, so it is more than a row deep.
            let deep = wall(column, top - 2);
            let on_blocks = room.contains_inner_tile(column, top - 1, 1)
                && room.contains_inner_tile(column, bottom + 1, 1);
            if closed && deep && on_blocks && (MIN_SPAN..=MAX_SPAN).contains(&length) {
                found.push((true, (column, top), (column, bottom)));
            }
        }
    }
    found
}

/// Two spans the same way keep a few rows (or columns) apart; one across and one upright don't
/// cross or touch.
fn too_close(a: &Overhead, b: &Overhead) -> bool {
    if a.upright == b.upright {
        let line = |span: &Overhead| if span.upright { span.first.0 } else { span.first.1 };
        line(a).abs_diff(line(b)) < ROW_GAP
    } else {
        a.tiles().any(|tile| b.near(tile, 2))
    }
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
        let (upright, first, last) = open.swap_remove(rng.below(open.len()));
        let candidate = Overhead {
            kind: kinds[0],
            upright,
            first,
            last,
        };
        if planned.iter().any(|other| too_close(other, &candidate)) {
            continue;
        }
        planned.push(Overhead {
            kind: kinds[rng.below(kinds.len())],
            ..candidate
        });
    }
    planned
}

const HANGING_CLEARANCE: isize = 2;
/// How many rows above its floor tile a hanging thing shows.
const HANGING_LIFT_ROWS: usize = 4;
const HANGING_SPACING: usize = 4;
const MAX_HANGING: usize = 2;
/// Only some combat rooms have anything hanging, so it stays a detail rather than a pattern.
const HANGING_ROOM_PERCENT: usize = 35;
/// Rooms at least this big (in tiles) may get a second one.
const ROOMY_FOR_TWO: usize = 500;
const SECOND_HANGING_PERCENT: usize = 40;
/// How often, on the lava floor, the thing hangs over a pool of lava as a crucible.
const CRUCIBLE_PERCENT: usize = 55;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HangingKind {
    Chandelier,
    Cage,
    Cocoon,
    Lantern,
    Vine,
    /// A brazier of glowing coals.
    Brazier,
    /// A crucible of molten metal; it hangs only over lava.
    Crucible,
    /// A charred skeleton hung on hooks.
    Hooks,
    /// An obsidian stalactite with glowing veins.
    Stalactite,
    /// Roots hung with glowing mushrooms.
    GlowRoots,
    /// A wasps' nest on a branch, a few wasps about it.
    Hive,
    /// A cage with a skeleton, overgrown with moss and flowers.
    MossCage,
    /// A swarm of fireflies hovering in the air, on no chain at all.
    Fireflies,
    /// A chandelier of icicles.
    IceChandelier,
    /// A skeleton frozen in a block of ice.
    FrozenBlock,
    /// A great icicle.
    BigIcicle,
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
            Self::Brazier => 5,
            Self::Crucible => 6,
            Self::Hooks => 7,
            Self::Stalactite => 8,
            Self::GlowRoots => 9,
            Self::Hive => 10,
            Self::MossCage => 11,
            Self::Fireflies => 12,
            Self::IceChandelier => 13,
            Self::FrozenBlock => 14,
            Self::BigIcicle => 15,
        }
    }
}

impl Theme {
    pub(crate) fn hanging_kinds(self) -> &'static [HangingKind] {
        use HangingKind::*;
        match self {
            Self::Stone => &[Chandelier, Cage],
            Self::Temple => &[Chandelier, Chandelier, Cage],
            Self::Moss => &[Vine, GlowRoots, GlowRoots, Hive, MossCage, Fireflies],
            Self::Sewer => &[Lantern, Cage],
            Self::Spider => &[Cocoon, Cocoon, Cage],
            Self::Ice => &[IceChandelier, IceChandelier, FrozenBlock, BigIcicle, Lantern],
            Self::Lava => &[Brazier, Brazier, Hooks, Stalactite, Cage],
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
    surfaces: &Surfaces,
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
    // The thing shows a few rows above the tile it hangs over, so neither may be near a span.
    let under_span = |column: usize, row: usize| {
        spans.iter().any(|span| {
            span.near((column, row), 2) || span.near((column, row.saturating_sub(HANGING_LIFT_ROWS)), 2)
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
        // On the lava floor, now and then a crucible over a pool of lava.
        let over_lava: Vec<usize> = (0..open.len())
            .filter(|&index| theme == Theme::Lava && surfaces.at(open[index]) == Some(Surface::Lava))
            .collect();
        let crucible = !over_lava.is_empty() && rng.below(100) < CRUCIBLE_PERCENT;
        let index = if crucible {
            over_lava[rng.below(over_lava.len())]
        } else {
            rng.below(open.len())
        };
        let tile = open.swap_remove(index);
        open.retain(|other| {
            other.0.abs_diff(tile.0) >= HANGING_SPACING || other.1.abs_diff(tile.1) >= HANGING_SPACING
        });
        let kind = if crucible {
            HangingKind::Crucible
        } else {
            kinds[rng.below(kinds.len())]
        };
        planned.push(Hanging { kind, tile });
    }
    planned
}

/// One of every hanging kind in a row across the middle of a room, for looking at them all at
/// once (`BANE_SHOWCASE`): the lava, moss and ice floors' own, or the rest on any other floor.
pub(crate) fn showcase(grid: &TileGrid, room: &DungeonRoom, theme: Theme) -> Vec<Hanging> {
    use HangingKind::*;
    let kinds = match theme {
        Theme::Lava => [Brazier, Crucible, Hooks, Stalactite, Cage],
        Theme::Moss => [GlowRoots, Hive, MossCage, Fireflies, Vine],
        Theme::Ice => [IceChandelier, FrozenBlock, BigIcicle, Lantern, Cage],
        _ => [Chandelier, Cage, Cocoon, Lantern, Vine],
    };
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
        let (mut total, mut upright) = (0, 0);
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
                        upright += usize::from(span.upright);
                        let step = if span.upright { (0, 1) } else { (1, 0) };
                        let before = (span.first.0 - step.0, span.first.1 - step.1);
                        let after = (span.last.0 + step.0, span.last.1 + step.1);
                        let wall = |(c, r): (usize, usize)| grid.is_wall(c as isize, r as isize);
                        assert!(wall(before) && wall(after));
                        assert!(span.tiles().all(|tile| !wall(tile)));
                        assert!((MIN_SPAN..=MAX_SPAN).contains(&span.length()));
                        assert!(theme.overhead_kinds().contains(&span.kind));
                        assert!(room.contains_tile(span.first.0, span.first.1));
                        assert!(room.contains_inner_tile(before.0, before.1, 1));
                        assert!(room.contains_inner_tile(after.0, after.1, 1));
                    }
                }
            }
        }
        assert!(total > 20, "{total}");
        assert!(upright > 5 && upright < total, "{upright} of {total}");
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
                        if hanging.kind == HangingKind::Crucible {
                            assert_eq!(theme, Theme::Lava);
                            assert_eq!(dungeon.surfaces.at(hanging.tile), Some(Surface::Lava));
                        } else {
                            assert!(theme.hanging_kinds().contains(&hanging.kind));
                        }
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
                    assert!(!too_close(a, b));
                }
            }
        }
    }
}
