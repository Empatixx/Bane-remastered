//! The training crypt the game opens with: ten small rooms in a row, each teaching one thing,
//! joined by short corridors whose gates open once the lesson is learnt. The rooms are built by
//! hand here rather than generated; what moves in them (pots, rats, the gun on the floor) is set
//! out by the training itself (see `systems::training`) so it can be set out again after a death.

use glam::Vec2;

use crate::tilemap::{TILE_SIZE, TileGrid};

use super::build::{DoorSlot, Dungeon, DungeonRoom};
use super::decor::{Decor, DecorKind, Mount};
use super::furnish::Carpet;
use super::furniture::TableSpot;
use super::layout::Direction;
use super::overhead::{Hanging, HangingKind, Overhead, OverheadKind};
use super::surface::{Surface, Surfaces};
use super::template::RoomKind;
use super::theme::Theme;
use super::waterworks::Waterworks;

/// What each room teaches, in the order they come.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Lesson {
    Move,
    Shoot,
    Loot,
    Dodge,
    Health,
    Traps,
    Cover,
    Artefact,
    Trial,
    Exit,
}

impl Lesson {
    pub(crate) const ALL: [Self; 10] = [
        Self::Move,
        Self::Shoot,
        Self::Loot,
        Self::Dodge,
        Self::Health,
        Self::Traps,
        Self::Cover,
        Self::Artefact,
        Self::Trial,
        Self::Exit,
    ];

    /// The room's floor plan: how big inside, which parts are floor, where the ways in and out
    /// are and where the guide waits. Every lesson has its own, so the crypt is passages,
    /// corridors and halls rather than one room ten times over.
    fn shape(self) -> Shape {
        let whole = |columns, rows, entry, exit, guide| Shape {
            columns,
            rows,
            floor: vec![(0, 0, columns, rows)],
            entry,
            exit,
            guide,
            top: 0,
        };
        match self {
            // A wide passage that zig-zags down: along the top, down the middle, out along the
            // bottom.
            Self::Move => Shape {
                columns: 20,
                rows: 15,
                floor: vec![(0, 0, 13, 5), (8, 0, 13, 15), (8, 10, 20, 15)],
                entry: 1,
                exit: 11,
                guide: (10.5, 3.6),
                top: 2,
            },
            Self::Shoot => Shape { top: 5, ..whole(18, 11, 4, 4, (5.5, 4.0 - 0.2)) },
            Self::Loot => Shape { top: 1, ..whole(16, 11, 3, 6, (5.5, 3.0 - 0.2)) },
            // A corridor, the spikes right across it.
            Self::Dodge => Shape { top: 7, ..whole(22, 5, 1, 1, (5.5, 1.0 - 0.2)) },
            Self::Health => Shape { top: 3, ..whole(15, 10, 2, 5, (5.5, 2.0 - 0.2)) },
            // A long passage of traps and pools.
            Self::Traps => Shape { top: 6, ..whole(27, 9, 3, 4, (5.5, 3.0 - 0.2)) },
            Self::Cover => Shape { top: 1, ..whole(24, 12, 4, 4, (5.5, 4.0 - 0.2)) },
            Self::Artefact => Shape { top: 4, ..whole(15, 10, 3, 3, (5.5, 3.0 - 0.2)) },
            Self::Trial => Shape { top: 0, ..whole(20, 14, 5, 5, (5.5, 5.0 - 0.2)) },
            Self::Exit => Shape { top: 2, ..whole(15, 11, 4, 4, (5.5, 4.0 - 0.2)) },
        }
    }
}

/// One lesson's room, in tiles counted from its top left inside corner.
struct Shape {
    columns: usize,
    rows: usize,
    /// The floor, as rectangles `(column, row, end column, end row)`, the ends not included.
    floor: Vec<(usize, usize, usize, usize)>,
    /// The top one of the rows the way in (on the left wall) and out (on the right) take.
    entry: usize,
    exit: usize,
    /// Where the guide stands, in tiles.
    guide: (f32, f32),
    /// How far down the crypt the room sits, so the corridors between rooms bend.
    top: usize,
}

const MARGIN: usize = 1;
/// The corridor between two rooms, long enough to bend half way.
const CORRIDOR: usize = 8;
/// How many tiles wide the ways in and out, and the corridors, are.
const DOOR: usize = 3;

/// One room of the crypt, and where its lesson happens.
#[derive(Debug, Clone)]
pub(crate) struct LessonRoom {
    pub(crate) lesson: Lesson,
    /// Where the player starts the lesson, and comes back to after a death.
    pub(crate) checkpoint: Vec2,
    /// Where the guide stands to talk the player through it.
    pub(crate) guide: Vec2,
    /// The gate out to the next room, on the room's right wall (none for the last).
    pub(crate) exit: Vec<(usize, usize)>,
    /// The way in from the room before, on the room's left wall (none for the first).
    pub(crate) entry: Vec<(usize, usize)>,
    /// The far end of the room, by the way out: reaching it is the point of some lessons.
    pub(crate) goal: Vec2,
    /// The room's top left inside corner, which `at` counts from.
    pub(crate) origin: (usize, usize),
    /// Bars in the top wall, closing off an alcove of water behind them, there to look at.
    pub(crate) bars: Vec<(usize, usize)>,
    /// Top wall tiles with chains hanging down their face.
    pub(crate) chains: Vec<(usize, usize)>,
}

impl LessonRoom {
    /// The middle of the inside tile `(column, row)`, counted from the room's top left inside
    /// corner.
    pub(crate) fn at(&self, column: usize, row: usize) -> Vec2 {
        (Vec2::new((self.origin.0 + column) as f32, (self.origin.1 + row) as f32) + 0.5) * TILE_SIZE
    }
}

fn decor(kind: DecorKind, tile: (usize, usize)) -> Decor {
    Decor { kind, tile }
}

/// Carves a corridor `DOOR` tiles wide from the way out of one room to the way into the next:
/// across, down or up, and across again. Returns what spans it overhead: planks across its
/// mouth at each end, and a chain hanging down the length of its bend.
fn carve_corridor(grid: &mut TileGrid, from: (usize, usize), to: (usize, usize)) -> Vec<Overhead> {
    let bend = from.0 + (CORRIDOR - DOOR) / 2 + 1;
    for column in from.0..bend + DOOR {
        for row in from.1..from.1 + DOOR {
            grid.carve(column, row);
        }
    }
    for row in from.1.min(to.1)..from.1.max(to.1) + DOOR {
        for column in bend..bend + DOOR {
            grid.carve(column, row);
        }
    }
    for column in bend..=to.0 {
        for row in to.1..to.1 + DOOR {
            grid.carve(column, row);
        }
    }
    let across = |column: usize, row: usize| Overhead {
        kind: OverheadKind::Beams,
        upright: true,
        first: (column, row),
        last: (column, row + DOOR - 1),
    };
    let mut spans = vec![across(from.0 + 1, from.1), across(to.0 - 1, to.1)];
    if from.1 != to.1 {
        spans.push(Overhead {
            kind: OverheadKind::Chain,
            upright: true,
            first: (bend + 1, from.1.min(to.1)),
            last: (bend + 1, from.1.max(to.1) + DOOR - 1),
        });
    }
    spans
}

/// A little dressing for each room, so the crypt is not bare walls: banners and shackles on the
/// walls, cobwebs in the corners, bones and the odd barrel on the floor, a rug here and there.
fn dress(room: &mut DungeonRoom, lesson: Lesson, inside: (usize, usize), shape: &Shape, top: usize) {
    let tile = |column: usize, row: usize| (inside.0 + column, inside.1 + row);
    let wall = |column: usize| (inside.0 + column, top);
    let last = shape.columns - 1;
    let middle = shape.columns / 2;
    // Nothing goes on or right beside something already on that row (a torch, say).
    let mut add = |kind, at: (usize, usize)| {
        let crowded = room
            .decor
            .iter()
            .any(|other| other.tile.1 == at.1 && other.tile.0.abs_diff(at.0) <= 1);
        if !crowded {
            room.decor.push(decor(kind, at));
        }
    };
    match lesson {
        Lesson::Move => {
            add(DecorKind::Cobweb { variant: 0, left: false }, tile(0, 0));
            add(DecorKind::Banner, wall(5));
            add(DecorKind::Bones { variant: 1 }, tile(10, 7));
            add(DecorKind::Cobweb { variant: 1, left: true }, tile(last, 10));
            add(DecorKind::Barrel, tile(17, 13));
        }
        Lesson::Shoot => {
            add(DecorKind::Banner, wall(middle));
            room.rug = Some(Carpet { origin: tile(1, 6), columns: 9, rows: 4, style: 0 });
            add(DecorKind::Cobweb { variant: 1, left: true }, tile(last, 0));
            add(DecorKind::Bones { variant: 0 }, tile(7, 9));
        }
        Lesson::Loot => {
            add(DecorKind::Shackles { variant: 1 }, wall(5));
            add(DecorKind::Shackles { variant: 0 }, wall(10));
            add(DecorKind::Cobweb { variant: 0, left: false }, tile(0, 0));
            add(DecorKind::Barrel, tile(13, 1));
        }
        Lesson::Dodge => {
            add(DecorKind::Bones { variant: 1 }, tile(17, 3));
            add(DecorKind::Cobweb { variant: 1, left: true }, tile(last, 0));
        }
        Lesson::Health => {
            add(DecorKind::Banner, wall(7));
            add(DecorKind::Bones { variant: 0 }, tile(13, 1));
            room.rug = Some(Carpet { origin: tile(3, 1), columns: 8, rows: 6, style: 0 });
        }
        Lesson::Traps => {
            add(DecorKind::Cobweb { variant: 0, left: false }, tile(0, 0));
            add(DecorKind::Bones { variant: 1 }, tile(23, 7));
        }
        Lesson::Cover => {
            add(DecorKind::Banner, wall(12));
            add(DecorKind::Shackles { variant: 1 }, wall(18));
            add(DecorKind::Cobweb { variant: 1, left: true }, tile(last, 0));
        }
        Lesson::Artefact => {
            room.rug = Some(Carpet { origin: tile(4, 1), columns: 8, rows: 6, style: 0 });
            add(DecorKind::Shackles { variant: 0 }, wall(2));
            add(DecorKind::Banner, wall(12));
        }
        Lesson::Trial => {
            add(DecorKind::Banner, wall(6));
            add(DecorKind::Banner, wall(13));
            add(DecorKind::Bones { variant: 0 }, tile(16, 11));
            add(DecorKind::Bones { variant: 1 }, tile(9, 2));
            add(DecorKind::Cobweb { variant: 0, left: false }, tile(0, 0));
        }
        Lesson::Exit => {
            add(DecorKind::Banner, wall(2));
            add(DecorKind::Banner, wall(13));
            room.rug = Some(Carpet { origin: tile(3, 2), columns: 10, rows: 7, style: 0 });
            add(DecorKind::Cobweb { variant: 1, left: true }, tile(last, 0));
        }
    }
}

/// An alcove behind bars above the room: a pool with a pipe pouring into it (or lava), a torch
/// to see it by. It is only there to look at; the bars keep the player out. Returns the bars'
/// tiles in the room's top wall.
fn alcove(
    grid: &mut TileGrid,
    surfaces: &mut Surfaces,
    room: &mut DungeonRoom,
    (first, top): (usize, usize),
    liquid: Surface,
) -> Vec<(usize, usize)> {
    const WIDTH: usize = 5;
    const DEPTH: usize = 3;
    for column in first..first + WIDTH {
        for row in top - DEPTH..top {
            grid.carve(column, row);
            surfaces.insert((column, row), liquid);
        }
        grid.carve(column, top);
    }
    let back = top - DEPTH - 1;
    if liquid == Surface::Water {
        room.waterworks.pipes.push((first + 2, back));
    }
    room.decor.push(decor(DecorKind::Torch { mount: Mount::Top, lit: true }, (first, back)));
    (first..first + WIDTH).map(|column| (column, top)).collect()
}

/// What hangs from the ceiling, and the chains down the walls, room by room: `(kind, inside
/// tile)` and top wall columns.
fn overhead(lesson: Lesson) -> (Vec<(HangingKind, (usize, usize))>, Vec<usize>) {
    use HangingKind::*;
    // Only now and then, and out by the walls rather than over the middle of the room.
    match lesson {
        Lesson::Move => (vec![(Lantern, (1, 3))], vec![9]),
        Lesson::Shoot => (vec![(Lantern, (16, 7))], vec![14]),
        Lesson::Loot => (vec![], vec![2, 13]),
        Lesson::Dodge => (vec![], vec![4, 12]),
        Lesson::Health => (vec![], vec![3, 12]),
        Lesson::Traps => (vec![(Cage, (25, 6))], vec![2]),
        Lesson::Cover => (vec![(Lantern, (1, 9))], vec![4, 15]),
        Lesson::Artefact => (vec![(Cage, (13, 8))], vec![5]),
        Lesson::Trial => (vec![(Cage, (18, 12))], vec![3, 9, 16]),
        Lesson::Exit => (vec![], vec![7]),
    }
}

/// Blood on the floor, inside tiles, in the rooms where there is fighting.
fn blood_of(lesson: Lesson) -> Vec<(usize, usize)> {
    // Each pool a rounded blob a few tiles across, its top left inside tile given.
    let pool = |(column, row): (usize, usize)| {
        [(1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (3, 1), (1, 2), (2, 2)].map(|(dx, dy)| (column + dx, row + dy))
    };
    let pools: &[(usize, usize)] = match lesson {
        Lesson::Health => &[(10, 6)],
        Lesson::Cover => &[(18, 7)],
        Lesson::Trial => &[(2, 1), (14, 10)],
        _ => &[],
    };
    pools.iter().flat_map(|&at| pool(at)).collect()
}

/// Where the alcoves behind bars are: `(first inside column, liquid)`.
fn alcove_of(lesson: Lesson) -> Option<(usize, Surface)> {
    match lesson {
        Lesson::Shoot => Some((3, Surface::Water)),
        Lesson::Dodge => Some((15, Surface::Water)),
        Lesson::Traps => Some((19, Surface::Lava)),
        Lesson::Artefact => Some((7, Surface::Water)),
        _ => None,
    }
}

/// The crypt and its lessons.
pub(crate) fn crypt() -> (Dungeon, Vec<LessonRoom>) {
    let shapes: Vec<Shape> = Lesson::ALL.iter().map(|lesson| lesson.shape()).collect();
    let columns = shapes.iter().map(|shape| shape.columns + 2).sum::<usize>()
        + CORRIDOR * (shapes.len() - 1)
        + 2 * MARGIN;
    let rows = shapes.iter().map(|shape| shape.top + shape.rows + 2).max().unwrap_or(0) + 2 * MARGIN;
    let mut grid = TileGrid::filled(columns, rows);
    let mut surfaces = Surfaces::default();
    let mut rooms = Vec::new();
    let mut lessons = Vec::new();
    let mut left = MARGIN;
    let mut way_out: Option<(usize, usize)> = None;
    for (index, (&lesson, shape)) in Lesson::ALL.iter().zip(&shapes).enumerate() {
        let top = MARGIN + shape.top;
        let width = shape.columns + 2;
        let height = shape.rows + 2;
        let inside = (left + 1, top + 1);
        for &(first_column, first_row, end_column, end_row) in &shape.floor {
            for row in first_row..end_row {
                for column in first_column..end_column {
                    grid.carve(inside.0 + column, inside.1 + row);
                }
            }
        }
        let door = |column: usize, first: usize| -> Vec<(usize, usize)> {
            (0..DOOR).map(|row| (column, inside.1 + first + row)).collect()
        };
        let entry = if index > 0 { door(left, shape.entry) } else { Vec::new() };
        let exit = if index + 1 < Lesson::ALL.len() { door(left + width - 1, shape.exit) } else { Vec::new() };
        if let (Some(from), Some(&to)) = (way_out, entry.first()) {
            let spans = carve_corridor(&mut grid, from, to);
            if let Some(before) = rooms.last_mut() {
                let before: &mut DungeonRoom = before;
                before.overheads.extend(spans);
            }
        }
        let mut slots = Vec::new();
        for (tiles, direction) in [(&entry, Direction::Left), (&exit, Direction::Right)] {
            for &(column, row) in tiles {
                grid.carve(column, row);
            }
            if tiles.len() >= 2 {
                slots.push(DoorSlot { direction, tiles: [tiles[0], tiles[1]] });
            }
        }
        way_out = exit.first().copied();
        let mut room = DungeonRoom {
            kind: if index == 0 { RoomKind::Start } else { RoomKind::Normal },
            cell: (index as i32, 0),
            origin: (left, top),
            columns: width,
            rows: height,
            spawns: Vec::new(),
            doors: slots,
            decor: Vec::new(),
            furnishing: None,
            overheads: Vec::new(),
            hanging: Vec::new(),
            track: None,
            rug: None,
            tables: Vec::new(),
            waterworks: Waterworks::default(),
            pendulum: None,
        };
        let tile = |column: usize, row: usize| (inside.0 + column, inside.1 + row);
        // Torches on the top wall wherever there is floor under them, near each end.
        for column in [1, shape.columns.saturating_sub(2)] {
            let (x, y) = tile(column, 0);
            if !grid.is_wall(x as isize, y as isize) {
                room.decor.push(decor(DecorKind::Torch { mount: Mount::Top, lit: true }, (x, top)));
            }
        }
        match lesson {
            Lesson::Dodge => {
                for column in 8..11 {
                    for row in 0..shape.rows {
                        room.decor.push(decor(DecorKind::Spike, tile(column, row)));
                    }
                }
                room.decor.push(decor(DecorKind::Flamethrower, (inside.0 + 14, top)));
            }
            Lesson::Traps => {
                for column in [6, 11, 16] {
                    room.decor.push(decor(DecorKind::ArrowTrap, (inside.0 + column, top)));
                }
                for row in 0..shape.rows {
                    surfaces.insert(tile(4, row), Surface::Water);
                    surfaces.insert(tile(5, row), Surface::Water);
                }
                for column in 8..11 {
                    for row in 0..4 {
                        surfaces.insert(tile(column, row), Surface::Lava);
                    }
                }
                for column in 13..18 {
                    for row in 0..shape.rows {
                        surfaces.insert(tile(column, row), Surface::Ice);
                    }
                }
                for (column, row) in [(20, 5), (20, 6), (21, 5), (21, 6)] {
                    room.decor.push(decor(DecorKind::Spike, tile(column, row)));
                }
            }
            Lesson::Cover => {
                room.tables.push(TableSpot { tile: tile(7, 3), upright: false });

            }
            _ => {}
        }
        dress(&mut room, lesson, inside, shape, top);
        let bars = match alcove_of(lesson) {
            Some((column, liquid)) => alcove(&mut grid, &mut surfaces, &mut room, (inside.0 + column, top), liquid),
            None => Vec::new(),
        };
        let (hanging, chain_columns) = overhead(lesson);
        room.hanging = hanging
            .into_iter()
            .map(|(kind, (column, row))| Hanging { kind, tile: tile(column, row) })
            .collect();
        // Chains only where the wall is bare: not on a torch, a banner or the bars.
        let chains = chain_columns
            .into_iter()
            .map(|column| (inside.0 + column, top))
            .filter(|&(column, row)| {
                let near = |tile: (usize, usize)| tile.1 == row && tile.0.abs_diff(column) <= 1;
                !room.decor.iter().any(|other| near(other.tile)) && !bars.iter().any(|&bar| near(bar))
            })
            .collect();
        // A splash of the stone floors' blood here and there.
        for (column, row) in blood_of(lesson) {
            surfaces.insert(tile(column, row), Surface::Blood);
        }
        rooms.push(room);
        let (guide_column, guide_row) = shape.guide;
        let middle_of = |first: usize| (inside.1 + first) as f32 + DOOR as f32 / 2.0;
        lessons.push(LessonRoom {
            lesson,
            checkpoint: Vec2::new(inside.0 as f32 + 1.5, middle_of(shape.entry)) * TILE_SIZE,
            guide: Vec2::new(inside.0 as f32 + guide_column, inside.1 as f32 + guide_row) * TILE_SIZE,
            exit,
            entry,
            goal: Vec2::new((left + width - 3) as f32, middle_of(shape.exit)) * TILE_SIZE,
            origin: inside,
            bars,
            chains,
        });
        left += width + CORRIDOR;
    }
    let dungeon = Dungeon {
        grid,
        rooms,
        theme: Theme::Stone,
        surfaces,
        vestibules: Vec::new(),
    };
    (dungeon, lessons)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tile(at: Vec2) -> (usize, usize) {
        ((at.x / TILE_SIZE) as usize, (at.y / TILE_SIZE) as usize)
    }

    #[test]
    fn the_crypt_is_ten_rooms_joined_by_corridors_from_the_first_start_to_the_last_goal() {
        let (dungeon, lessons) = crypt();
        assert_eq!(lessons.len(), 10);
        assert_eq!(dungeon.rooms.len(), 10);
        assert_eq!(dungeon.rooms[0].kind, RoomKind::Start);
        for lesson in &lessons {
            for at in [lesson.checkpoint, lesson.goal, lesson.guide] {
                let (column, row) = tile(at);
                assert!(!dungeon.grid.is_wall(column as isize, row as isize), "{:?} {at:?}", lesson.lesson);
            }
        }
        // Every room's start reaches the last room's goal over open floor.
        let mut seen = std::collections::HashSet::new();
        let mut open = vec![tile(lessons[0].checkpoint)];
        while let Some((column, row)) = open.pop() {
            if !seen.insert((column, row)) {
                continue;
            }
            for (dx, dy) in [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)] {
                let next = (column as isize + dx, row as isize + dy);
                if !dungeon.grid.is_wall(next.0, next.1) {
                    open.push((next.0 as usize, next.1 as usize));
                }
            }
        }
        for lesson in &lessons {
            assert!(seen.contains(&tile(lesson.checkpoint)) && seen.contains(&tile(lesson.goal)), "{:?}", lesson.lesson);
        }
        assert!(lessons[9].exit.is_empty() && lessons[0].entry.is_empty());
    }

    #[test]
    fn every_corridor_has_planks_across_its_mouths_and_a_chain_down_its_bend() {
        let (dungeon, _) = crypt();
        let spans: Vec<&Overhead> = dungeon.rooms.iter().flat_map(|room| &room.overheads).collect();
        let planks = spans.iter().filter(|span| span.kind == OverheadKind::Beams).count();
        let chains = spans.iter().filter(|span| span.kind == OverheadKind::Chain).count();
        assert_eq!(planks, 2 * (Lesson::ALL.len() - 1));
        assert!(chains >= 7);
        for span in spans {
            assert!(span.upright);
            for (column, row) in span.tiles() {
                assert!(!dungeon.grid.is_wall(column as isize, row as isize));
            }
            assert!(dungeon.grid.is_wall(span.first.0 as isize, span.first.1 as isize - 1));
            assert!(dungeon.grid.is_wall(span.last.0 as isize, span.last.1 as isize + 1));
        }
    }

    #[test]
    fn the_rooms_differ_in_shape_and_height() {
        let (dungeon, _) = crypt();
        let shapes: std::collections::HashSet<_> =
            dungeon.rooms.iter().map(|room| (room.columns, room.rows)).collect();
        let tops: std::collections::HashSet<_> = dungeon.rooms.iter().map(|room| room.origin.1).collect();
        assert!(shapes.len() >= 7 && tops.len() >= 4);
    }

    #[test]
    fn the_lessons_rooms_hold_what_they_teach() {
        let (dungeon, lessons) = crypt();
        let room = |lesson: Lesson| &dungeon.rooms[lessons.iter().position(|l| l.lesson == lesson).expect("lesson")];
        assert!(room(Lesson::Dodge).decor.iter().filter(|d| d.kind == DecorKind::Spike).count() >= 15);
        assert!(room(Lesson::Traps).decor.iter().any(|d| d.kind == DecorKind::ArrowTrap));
        assert!(dungeon.surfaces.iter().any(|(_, surface)| surface == Surface::Lava));
        assert!(room(Lesson::Cover).track.is_none() && !room(Lesson::Cover).tables.is_empty());
    }
}
