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
use super::overhead::{Overhead, OverheadKind};
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
        // A hall laid out like the dungeon's treasure rooms: odd sizes so it has a middle, the
        // ways in and out on its middle rows, the guide by the way in.
        let hall = |columns: usize, rows: usize, top| {
            let door = (rows - DOOR) / 2;
            Shape {
                columns,
                rows,
                floor: vec![(0, 0, columns, rows)],
                entry: door,
                exit: door,
                // Below the path, so his words float between him and the player, clear of the
                // health bars at the top of the screen.
                guide: (4.5, door as f32 + 4.3),
                top,
                hall: true,
            }
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
                hall: false,
            },
            Self::Shoot => hall(19, 11, 5),
            Self::Loot => hall(17, 11, 1),
            // A corridor, the spikes right across its middle.
            Self::Dodge => Shape { hall: false, guide: (5.5, 0.8), ..hall(21, 5, 7) },
            Self::Health => hall(15, 11, 3),
            // A long passage of traps and pools.
            Self::Traps => Shape { hall: false, guide: (2.5, 7.3), ..hall(27, 9, 6) },
            Self::Cover => hall(23, 11, 1),
            Self::Artefact => hall(15, 11, 4),
            Self::Trial => hall(21, 13, 0),
            Self::Exit => hall(15, 11, 2),
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
    /// A plain hall, dressed the same way every time: pillars in its corners, a carpet between
    /// them, torches and banners in pairs.
    hall: bool,
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
/// across, down or up, and across again. Returns what spans it overhead: a chain hanging down
/// the length of its bend.
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
    let mut spans = Vec::new();
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

/// The dressing, the same plan in every hall, like the dungeon's treasure rooms: a block in each
/// corner, a carpet in the middle of some, torches above the blocks, banners either side of the middle,
/// cobwebs in both top corners. Returns the blocks, `(column, row)` of their top left inside
/// tile, for the caller to leave uncarved.
fn dress(room: &mut DungeonRoom, lesson: Lesson, inside: (usize, usize), shape: &Shape, top: usize, alcove: bool) -> Vec<(usize, usize)> {
    let tile = |column: usize, row: usize| (inside.0 + column, inside.1 + row);
    let wall = |column: usize| (inside.0 + column, top);
    let (columns, rows) = (shape.columns, shape.rows);
    let last = columns - 1;
    let middle = columns / 2;
    let mirror = |column: usize| last - column;
    // What hangs on the top wall needs floor below it (the zig-zag passage has wall there).
    let open = |column: usize| {
        shape.floor.iter().any(|&(first, first_row, end, _)| first_row == 0 && (first..end).contains(&column))
    };
    for (column, left) in [(0, false), (last, true)] {
        if open(column) {
            room.decor.push(decor(DecorKind::Cobweb { variant: 0, left }, tile(column, 0)));
        }
    }
    let torch = columns / 4;
    for column in [torch, mirror(torch)] {
        if open(column) {
            room.decor.push(decor(DecorKind::Torch { mount: Mount::Top, lit: true }, wall(column)));
        }
    }
    if !shape.hall {
        return Vec::new();
    }
    // Banners either side of the middle, unless an alcove or the hall's own piece (racks of
    // swords, bookshelves, statues) takes the top wall.
    let wall_taken = matches!(lesson, Lesson::Shoot | Lesson::Cover | Lesson::Exit);
    if !alcove && !wall_taken {
        for column in [middle - 2, mirror(middle - 2)] {
            room.decor.push(decor(DecorKind::Banner, wall(column)));
        }
    }
    // A carpet in the middle, where the lesson happens on it: a tile of floor between it and the
    // corner blocks either side, two between it and the walls above and below.
    // Fire bowls round the hall where there is fighting or healing.
    let bowls: &[(usize, usize)] = match lesson {
        Lesson::Health => &[(3, rows / 2), (columns - 4, rows / 2)],
        Lesson::Trial => &[(5, 2), (columns - 6, 2), (5, rows - 3), (columns - 6, rows - 3)],
        _ => &[],
    };
    for &(column, row) in bowls {
        room.decor.push(decor(DecorKind::FireBowl, tile(column, row)));
    }
    if matches!(lesson, Lesson::Loot | Lesson::Health | Lesson::Exit) {
        room.rug = Some(Carpet::rect(tile(5, 2), columns - 10, rows - 4, 0));
    }
    // The way out stands between two statues instead of four blocks.
    if lesson == Lesson::Exit {
        return Vec::new();
    }
    vec![(2, 2), (columns - 4, 2), (2, rows - 4), (columns - 4, rows - 4)]
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
    const WIDTH: usize = ALCOVE_WIDTH;
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
    for column in [first, first + WIDTH - 1] {
        room.decor.push(decor(DecorKind::Torch { mount: Mount::Top, lit: true }, (column, back)));
    }
    (first..first + WIDTH).map(|column| (column, top)).collect()
}

/// The halls with an alcove behind bars in the middle of their top wall, and what is in it.
fn alcove_of(lesson: Lesson) -> Option<Surface> {
    match lesson {
        Lesson::Health | Lesson::Artefact => Some(Surface::Water),
        Lesson::Traps => Some(Surface::Lava),
        _ => None,
    }
}

const ALCOVE_WIDTH: usize = 5;

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
        match lesson {
            Lesson::Dodge => {
                let middle = shape.columns / 2;
                for column in middle - 1..=middle + 1 {
                    for row in 0..shape.rows {
                        room.decor.push(decor(DecorKind::Spike, tile(column, row)));
                    }
                }
                room.decor.push(decor(DecorKind::Flamethrower, (inside.0 + middle, top)));
            }
            Lesson::Traps => {
                for column in [4, shape.columns - 5] {
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
                room.tables.push(TableSpot { tile: tile(shape.columns / 2 - 1, 3), upright: false });
            }
            _ => {}
        }
        let liquid = alcove_of(lesson);
        let blocks = dress(&mut room, lesson, inside, shape, top, liquid.is_some());
        for (column, row) in blocks {
            for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                grid.fill(inside.0 + column + dx, inside.1 + row + dy);
            }
        }
        let bars = match liquid {
            Some(liquid) => {
                let first = inside.0 + (shape.columns - ALCOVE_WIDTH) / 2;
                alcove(&mut grid, &mut surfaces, &mut room, (first, top), liquid)
            }
            None => Vec::new(),
        };
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
            chains: Vec::new(),
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
    fn corridors_have_a_chain_down_their_bend_and_nothing_else_overhead() {
        let (dungeon, _) = crypt();
        let spans: Vec<&Overhead> = dungeon.rooms.iter().flat_map(|room| &room.overheads).collect();
        assert!(spans.len() >= 5);
        for span in spans {
            assert!(span.upright && span.kind == OverheadKind::Chain);
            for (column, row) in span.tiles() {
                assert!(!dungeon.grid.is_wall(column as isize, row as isize));
            }
        }
    }

    #[test]
    fn the_halls_are_laid_out_the_same_on_both_sides() {
        let (dungeon, lessons) = crypt();
        for (room, lesson) in dungeon.rooms.iter().zip(&lessons) {
            if !lesson.lesson.shape().hall {
                continue;
            }
            let (left, top) = room.origin;
            let (columns, rows) = (room.columns, room.rows);
            for row in top..top + rows {
                for column in left + 1..left + columns - 1 {
                    let mirrored = left + columns - 1 - (column - left);
                    assert_eq!(
                        dungeon.grid.is_wall(column as isize, row as isize),
                        dungeon.grid.is_wall(mirrored as isize, row as isize),
                        "{:?} at {column},{row}",
                        lesson.lesson
                    );
                }
            }
            if let Some(rug) = room.rug {
                assert_eq!(rug.origin.0 - left, left + columns - (rug.origin.0 + rug.columns));
                assert_eq!(rug.origin.1 - top, top + rows - (rug.origin.1 + rug.rows));
                // Nowhere does it touch a wall or a block, not even at a corner.
                for row in rug.origin.1 - 1..=rug.origin.1 + rug.rows {
                    for column in rug.origin.0 - 1..=rug.origin.0 + rug.columns {
                        assert!(!dungeon.grid.is_wall(column as isize, row as isize), "{:?}", lesson.lesson);
                    }
                }
            }
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
