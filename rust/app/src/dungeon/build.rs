use std::error::Error;

use glam::Vec2;

use crate::tilemap::TileGrid;

use super::layout::{Direction, Layout, LayoutSpec};
use super::rng::Rng;
use super::template::{
    Cell, RoomKind, Templates, DOOR_COLUMNS, DOOR_ROWS, ROOM_COLUMNS, ROOM_ROWS,
};

pub(crate) const CORRIDOR_LENGTH: usize = 4;
const MARGIN: usize = 1;

#[derive(Debug, Clone)]
pub(crate) struct DoorSlot {
    pub(crate) direction: Direction,
    pub(crate) tiles: [(usize, usize); 2],
}

#[derive(Debug, Clone)]
pub(crate) struct DungeonRoom {
    pub(crate) kind: RoomKind,
    pub(crate) origin: (usize, usize),
    pub(crate) spawns: Vec<(usize, usize)>,
    pub(crate) doors: Vec<DoorSlot>,
}

impl DungeonRoom {
    pub(crate) fn center(&self, tile_size: f32) -> Vec2 {
        Vec2::new(
            self.origin.0 as f32 + ROOM_COLUMNS as f32 / 2.0,
            self.origin.1 as f32 + ROOM_ROWS as f32 / 2.0,
        ) * tile_size
    }

    pub(crate) fn contains_tile(&self, column: usize, row: usize) -> bool {
        column >= self.origin.0
            && column < self.origin.0 + ROOM_COLUMNS
            && row >= self.origin.1
            && row < self.origin.1 + ROOM_ROWS
    }
}

#[derive(Debug)]
pub(crate) struct Dungeon {
    pub(crate) grid: TileGrid,
    pub(crate) rooms: Vec<DungeonRoom>,
}

impl Dungeon {
    pub(crate) fn generate(rng: &mut Rng) -> Result<Self, Box<dyn Error>> {
        let templates = Templates::built_in()?;
        let spec = LayoutSpec::standard(rng);
        let layout = Layout::generate(&spec, rng).ok_or("could not lay out the rooms")?;
        Self::build(&layout, &templates, rng)
    }

    pub(crate) fn build(
        layout: &Layout,
        templates: &Templates,
        rng: &mut Rng,
    ) -> Result<Self, Box<dyn Error>> {
        let min_x = layout
            .rooms
            .iter()
            .map(|room| room.cell.0)
            .min()
            .unwrap_or(0);
        let max_x = layout
            .rooms
            .iter()
            .map(|room| room.cell.0)
            .max()
            .unwrap_or(0);
        let min_y = layout
            .rooms
            .iter()
            .map(|room| room.cell.1)
            .min()
            .unwrap_or(0);
        let max_y = layout
            .rooms
            .iter()
            .map(|room| room.cell.1)
            .max()
            .unwrap_or(0);
        let cells_x = (max_x - min_x) as usize + 1;
        let cells_y = (max_y - min_y) as usize + 1;
        let columns = cells_x * ROOM_COLUMNS + (cells_x - 1) * CORRIDOR_LENGTH + 2 * MARGIN;
        let rows = cells_y * ROOM_ROWS + (cells_y - 1) * CORRIDOR_LENGTH + 2 * MARGIN;
        let mut grid = TileGrid::filled(columns, rows);

        let origin_of = |cell: (i32, i32)| {
            (
                (cell.0 - min_x) as usize * (ROOM_COLUMNS + CORRIDOR_LENGTH) + MARGIN,
                (cell.1 - min_y) as usize * (ROOM_ROWS + CORRIDOR_LENGTH) + MARGIN,
            )
        };

        let mut rooms = Vec::with_capacity(layout.rooms.len());
        for placed in &layout.rooms {
            let template = templates
                .pick(placed.kind, rng)
                .ok_or_else(|| format!("no template for {:?}", placed.kind))?;
            let origin = origin_of(placed.cell);
            let mut spawns = Vec::new();
            let mut doors = Vec::new();
            for (row, line) in template.cells.iter().enumerate() {
                for (column, cell) in line.iter().enumerate() {
                    let (global_column, global_row) = (origin.0 + column, origin.1 + row);
                    match cell {
                        Cell::Wall => {}
                        Cell::Floor => grid.carve(global_column, global_row),
                        Cell::Spawn => {
                            grid.carve(global_column, global_row);
                            spawns.push((global_column, global_row));
                        }
                        Cell::Door => {
                            let direction = door_direction(column, row);
                            if placed.links.iter().any(|(linked, _)| *linked == direction) {
                                grid.carve(global_column, global_row);
                            }
                        }
                    }
                }
            }
            for &(direction, _) in &placed.links {
                doors.push(DoorSlot {
                    direction,
                    tiles: door_tiles(origin, direction),
                });
            }
            rooms.push(DungeonRoom {
                kind: placed.kind,
                origin,
                spawns,
                doors,
            });
        }

        for (index, placed) in layout.rooms.iter().enumerate() {
            for &(direction, other) in &placed.links {
                if index < other {
                    carve_corridor(&mut grid, rooms[index].origin, direction);
                }
            }
        }
        Ok(Self { grid, rooms })
    }

    pub(crate) fn start_room(&self) -> Option<&DungeonRoom> {
        self.rooms.iter().find(|room| room.kind == RoomKind::Start)
    }

    pub(crate) fn room_at(&self, column: usize, row: usize) -> Option<usize> {
        self.rooms
            .iter()
            .position(|room| room.contains_tile(column, row))
    }
}

fn door_direction(column: usize, row: usize) -> Direction {
    if row == 0 {
        Direction::Up
    } else if row == ROOM_ROWS - 1 {
        Direction::Down
    } else if column == 0 {
        Direction::Left
    } else {
        Direction::Right
    }
}

fn door_tiles(origin: (usize, usize), direction: Direction) -> [(usize, usize); 2] {
    match direction {
        Direction::Up => DOOR_COLUMNS.map(|column| (origin.0 + column, origin.1)),
        Direction::Down => DOOR_COLUMNS.map(|column| (origin.0 + column, origin.1 + ROOM_ROWS - 1)),
        Direction::Left => DOOR_ROWS.map(|row| (origin.0, origin.1 + row)),
        Direction::Right => DOOR_ROWS.map(|row| (origin.0 + ROOM_COLUMNS - 1, origin.1 + row)),
    }
}

fn carve_corridor(grid: &mut TileGrid, origin: (usize, usize), direction: Direction) {
    for step in 0..CORRIDOR_LENGTH {
        for lane in 0..2 {
            let (column, row) = match direction {
                Direction::Right => (origin.0 + ROOM_COLUMNS + step, origin.1 + DOOR_ROWS[lane]),
                Direction::Left => (
                    origin.0 - CORRIDOR_LENGTH + step,
                    origin.1 + DOOR_ROWS[lane],
                ),
                Direction::Down => (origin.0 + DOOR_COLUMNS[lane], origin.1 + ROOM_ROWS + step),
                Direction::Up => (
                    origin.0 + DOOR_COLUMNS[lane],
                    origin.1 - CORRIDOR_LENGTH + step,
                ),
            };
            grid.carve(column, row);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    fn dungeon(seed: u64) -> Dungeon {
        let mut rng = Rng::new(seed);
        Dungeon::generate(&mut rng).expect("a dungeon is generated")
    }

    fn reachable_floor(dungeon: &Dungeon, start: (usize, usize)) -> Vec<Vec<bool>> {
        let (columns, rows) = (dungeon.grid.columns(), dungeon.grid.rows());
        let mut seen = vec![vec![false; columns]; rows];
        let mut queue = VecDeque::from([start]);
        seen[start.1][start.0] = true;
        while let Some((column, row)) = queue.pop_front() {
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (x, y) = (column as isize + dx, row as isize + dy);
                if dungeon.grid.is_wall(x, y) {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if !seen[y][x] {
                    seen[y][x] = true;
                    queue.push_back((x, y));
                }
            }
        }
        seen
    }

    #[test]
    fn every_room_is_walkable_from_the_start_room() {
        for seed in 0..60 {
            let dungeon = dungeon(seed);
            let start = dungeon.start_room().expect("there is a start room");
            let from = (start.origin.0 + 2, start.origin.1 + 2);
            let seen = reachable_floor(&dungeon, from);
            for room in &dungeon.rooms {
                let center = (room.origin.0 + ROOM_COLUMNS / 2, room.origin.1 + 2);
                assert!(
                    seen[center.1][center.0],
                    "seed {seed}: {:?} room is not reachable",
                    room.kind
                );
                for &(column, row) in &room.spawns {
                    assert!(seen[row][column], "seed {seed}: unreachable spawn");
                }
            }
        }
    }

    #[test]
    fn rooms_without_a_link_stay_closed_on_that_side() {
        let dungeon = dungeon(5);
        for room in &dungeon.rooms {
            for direction in Direction::ALL {
                let linked = room.doors.iter().any(|door| door.direction == direction);
                let tiles = door_tiles(room.origin, direction);
                for (column, row) in tiles {
                    assert_eq!(!dungeon.grid.is_wall(column as isize, row as isize), linked);
                }
            }
        }
    }

    #[test]
    fn the_door_tiles_sit_on_the_room_border() {
        let dungeon = dungeon(11);
        for room in &dungeon.rooms {
            for door in &room.doors {
                for &(column, row) in &door.tiles {
                    assert!(room.contains_tile(column, row));
                }
            }
        }
    }

    #[test]
    fn the_start_room_is_the_one_the_player_begins_in() {
        let dungeon = dungeon(3);
        let start = dungeon.start_room().expect("there is a start room");
        let center = start.center(64.0);
        let column = (center.x / 64.0) as usize;
        let row = (center.y / 64.0) as usize;
        assert_eq!(
            dungeon
                .room_at(column, row)
                .map(|index| dungeon.rooms[index].kind),
            Some(RoomKind::Start)
        );
    }
}
