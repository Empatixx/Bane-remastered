use std::error::Error;

use glam::Vec2;

use crate::tilemap::{TileGrid, TILE_SIZE};

use super::decor::{self, Decor};
use super::layout::{Direction, Layout, LayoutSpec};
use super::rng::Rng;
use super::shop::{self, ShopLayout};
use super::template::{
    door_columns, door_rows, Cell, RoomKind, Templates, CELL_COLUMNS, CELL_ROWS,
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
    pub(crate) cell: (i32, i32),
    pub(crate) origin: (usize, usize),
    pub(crate) columns: usize,
    pub(crate) rows: usize,
    pub(crate) spawns: Vec<(usize, usize)>,
    pub(crate) doors: Vec<DoorSlot>,
    pub(crate) decor: Vec<Decor>,
    pub(crate) shop: Option<ShopLayout>,
}

impl DungeonRoom {
    #[cfg(test)]
    pub(crate) fn center(&self, tile_size: f32) -> Vec2 {
        Vec2::new(
            self.origin.0 as f32 + self.columns as f32 / 2.0,
            self.origin.1 as f32 + self.rows as f32 / 2.0,
        ) * tile_size
    }

    pub(crate) fn contains_tile(&self, column: usize, row: usize) -> bool {
        column >= self.origin.0
            && column < self.origin.0 + self.columns
            && row >= self.origin.1
            && row < self.origin.1 + self.rows
    }

    pub(crate) fn contains_inner_tile(&self, column: usize, row: usize, margin: usize) -> bool {
        column >= self.origin.0 + margin
            && column + margin < self.origin.0 + self.columns
            && row >= self.origin.1 + margin
            && row + margin < self.origin.1 + self.rows
    }

    fn bounds(&self) -> (Vec2, Vec2) {
        (
            Vec2::new(self.origin.0 as f32, self.origin.1 as f32),
            Vec2::new(
                (self.origin.0 + self.columns) as f32,
                (self.origin.1 + self.rows) as f32,
            ),
        )
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
        let columns = cells_x * CELL_COLUMNS + (cells_x - 1) * CORRIDOR_LENGTH + 2 * MARGIN;
        let rows = cells_y * CELL_ROWS + (cells_y - 1) * CORRIDOR_LENGTH + 2 * MARGIN;
        let mut grid = TileGrid::filled(columns, rows);

        let cell_origin = |cell: (i32, i32)| {
            (
                (cell.0 - min_x) as usize * (CELL_COLUMNS + CORRIDOR_LENGTH) + MARGIN,
                (cell.1 - min_y) as usize * (CELL_ROWS + CORRIDOR_LENGTH) + MARGIN,
            )
        };

        let mut rooms = Vec::with_capacity(layout.rooms.len());
        for placed in &layout.rooms {
            let template = templates
                .pick(placed.kind, rng)
                .ok_or_else(|| format!("no template for {:?}", placed.kind))?;
            let (template_columns, template_rows) = (template.columns(), template.rows());
            let corner = cell_origin(placed.cell);
            let origin = (
                corner.0 + (CELL_COLUMNS - template_columns) / 2,
                corner.1 + (CELL_ROWS - template_rows) / 2,
            );
            let mut spawns = Vec::new();
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
                            let direction =
                                door_direction(template_columns, template_rows, column, row);
                            if placed.links.iter().any(|(linked, _)| *linked == direction) {
                                grid.carve(global_column, global_row);
                            }
                        }
                    }
                }
            }
            let doors = placed
                .links
                .iter()
                .map(|&(direction, _)| DoorSlot {
                    direction,
                    tiles: door_tiles(origin, template_columns, template_rows, direction),
                })
                .collect();
            rooms.push(DungeonRoom {
                kind: placed.kind,
                cell: placed.cell,
                origin,
                columns: template_columns,
                rows: template_rows,
                spawns,
                doors,
                decor: Vec::new(),
                shop: None,
            });
        }

        for (index, placed) in layout.rooms.iter().enumerate() {
            for &(direction, other) in &placed.links {
                if index < other {
                    carve_corridor(&mut grid, &rooms[index], &rooms[other], direction);
                }
            }
        }
        for room in &mut rooms {
            if room.kind == RoomKind::Shop {
                room.shop = Some(shop::plan(room.origin, room.columns, room.rows));
            } else {
                room.decor = decor::plan(&grid, room, rng);
            }
        }
        Ok(Self { grid, rooms })
    }

    pub(crate) fn start_position(&self) -> Option<Vec2> {
        let room = self.start_room()?;
        let center = (
            room.origin.0 + room.columns / 2,
            room.origin.1 + room.rows / 2,
        );
        let clear = |column: usize, row: usize| {
            (-1..=1).all(|dy| {
                (-1..=1).all(|dx| !self.grid.is_wall(column as isize + dx, row as isize + dy))
            })
        };
        let reach = room.columns.max(room.rows);
        for radius in 0..reach {
            for dy in -(radius as isize)..=(radius as isize) {
                for dx in -(radius as isize)..=(radius as isize) {
                    if dx.abs().max(dy.abs()) != radius as isize {
                        continue;
                    }
                    let column = (center.0 as isize + dx) as usize;
                    let row = (center.1 as isize + dy) as usize;
                    if room.contains_inner_tile(column, row, 2) && clear(column, row) {
                        return Some(Vec2::new(column as f32 + 0.5, row as f32 + 0.5) * TILE_SIZE);
                    }
                }
            }
        }
        None
    }

    pub(crate) fn start_room(&self) -> Option<&DungeonRoom> {
        self.rooms.iter().find(|room| room.kind == RoomKind::Start)
    }

    pub(crate) fn camera_region(&self, column: usize, row: usize) -> Option<(Vec2, Vec2)> {
        if let Some(index) = self.room_at(column, row) {
            let (min, max) = self.rooms[index].bounds();
            return Some((min * TILE_SIZE, max * TILE_SIZE));
        }
        let point = Vec2::new(column as f32 + 0.5, row as f32 + 0.5);
        for room in &self.rooms {
            for door in &room.doors {
                let Some(other) = self.neighbour(room, door.direction) else {
                    continue;
                };
                let (a_min, a_max) = room.bounds();
                let (b_min, b_max) = other.bounds();
                let (min, max) = (a_min.min(b_min), a_max.max(b_max));
                if point.cmpge(min).all() && point.cmplt(max).all() {
                    return Some((min * TILE_SIZE, max * TILE_SIZE));
                }
            }
        }
        None
    }

    fn neighbour(&self, room: &DungeonRoom, direction: Direction) -> Option<&DungeonRoom> {
        let (dx, dy) = direction.delta();
        let cell = (room.cell.0 + dx, room.cell.1 + dy);
        self.rooms.iter().find(|other| other.cell == cell)
    }

    pub(crate) fn room_at(&self, column: usize, row: usize) -> Option<usize> {
        self.rooms
            .iter()
            .position(|room| room.contains_tile(column, row))
    }
}

fn door_direction(columns: usize, rows: usize, column: usize, row: usize) -> Direction {
    if row == 0 {
        Direction::Up
    } else if row == rows - 1 {
        Direction::Down
    } else if column == 0 {
        Direction::Left
    } else {
        debug_assert_eq!(column, columns - 1);
        Direction::Right
    }
}

fn door_tiles(
    origin: (usize, usize),
    columns: usize,
    rows: usize,
    direction: Direction,
) -> [(usize, usize); 2] {
    match direction {
        Direction::Up => door_columns(columns).map(|column| (origin.0 + column, origin.1)),
        Direction::Down => {
            door_columns(columns).map(|column| (origin.0 + column, origin.1 + rows - 1))
        }
        Direction::Left => door_rows(rows).map(|row| (origin.0, origin.1 + row)),
        Direction::Right => door_rows(rows).map(|row| (origin.0 + columns - 1, origin.1 + row)),
    }
}

fn carve_corridor(grid: &mut TileGrid, from: &DungeonRoom, to: &DungeonRoom, direction: Direction) {
    let from_tiles = door_tiles(from.origin, from.columns, from.rows, direction);
    let to_tiles = door_tiles(to.origin, to.columns, to.rows, direction.opposite());
    for lane in 0..2 {
        let (start, end) = (from_tiles[lane], to_tiles[lane]);
        let (columns, rows) = (
            start.0.min(end.0)..=start.0.max(end.0),
            start.1.min(end.1)..=start.1.max(end.1),
        );
        for column in columns {
            for row in rows.clone() {
                grid.carve(column, row);
            }
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
        for seed in 0..80 {
            let dungeon = dungeon(seed);
            let start = dungeon.start_room().expect("there is a start room");
            let from = start
                .doors
                .first()
                .map(|door| door.tiles[0])
                .expect("a door");
            let seen = reachable_floor(&dungeon, from);
            for room in &dungeon.rooms {
                for door in &room.doors {
                    for &(column, row) in &door.tiles {
                        assert!(seen[row][column], "seed {seed}: unreachable door");
                    }
                }
                for &(column, row) in &room.spawns {
                    assert!(seen[row][column], "seed {seed}: unreachable spawn");
                }
            }
        }
    }

    #[test]
    fn rooms_come_in_different_sizes_inside_the_same_cell_grid() {
        let sizes: std::collections::HashSet<(usize, usize)> = (0..40)
            .flat_map(|seed| dungeon(seed).rooms)
            .map(|room| (room.columns, room.rows))
            .collect();
        assert!(sizes.len() >= 10, "only {} sizes", sizes.len());
    }

    #[test]
    fn rooms_never_overlap_each_other() {
        for seed in 0..60 {
            let dungeon = dungeon(seed);
            for (index, a) in dungeon.rooms.iter().enumerate() {
                for b in dungeon.rooms.iter().skip(index + 1) {
                    let (a_min, a_max) = a.bounds();
                    let (b_min, b_max) = b.bounds();
                    let apart = a_max.x <= b_min.x
                        || b_max.x <= a_min.x
                        || a_max.y <= b_min.y
                        || b_max.y <= a_min.y;
                    assert!(apart, "seed {seed}");
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
                let tiles = door_tiles(room.origin, room.columns, room.rows, direction);
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
                    assert!(!room.contains_inner_tile(column, row, 1));
                }
            }
        }
    }

    #[test]
    fn the_camera_area_is_the_room_itself_and_in_a_corridor_both_connected_rooms() {
        let dungeon = dungeon(8);
        let start = dungeon.start_room().expect("there is a start room");
        let door = start.doors.first().expect("the start room has a door");
        let inside = dungeon
            .camera_region(start.origin.0 + 3, start.origin.1 + 3)
            .expect("inside the room");
        assert_eq!(
            inside.1 - inside.0,
            Vec2::new(start.columns as f32, start.rows as f32) * TILE_SIZE
        );

        let (column, row) = door.tiles[0];
        let (dx, dy) = door.direction.delta();
        let in_gap = (
            (column as isize + dx as isize * 2) as usize,
            (row as isize + dy as isize * 2) as usize,
        );
        let corridor = dungeon
            .camera_region(in_gap.0, in_gap.1)
            .expect("in the corridor");
        assert!(corridor.1.x - corridor.0.x >= inside.1.x - inside.0.x);
        assert!(corridor.1.y - corridor.0.y >= inside.1.y - inside.0.y);
        assert!(
            corridor.1 - corridor.0 != inside.1 - inside.0,
            "the corridor area spans two rooms"
        );
    }

    #[test]
    fn the_player_starts_on_clear_floor_inside_the_start_room() {
        for seed in 0..60 {
            let dungeon = dungeon(seed);
            let position = dungeon.start_position().expect("a start position");
            let (column, row) = (
                (position.x / TILE_SIZE) as usize,
                (position.y / TILE_SIZE) as usize,
            );
            assert_eq!(
                dungeon
                    .room_at(column, row)
                    .map(|index| dungeon.rooms[index].kind),
                Some(RoomKind::Start)
            );
            assert!(!dungeon.grid.is_wall(column as isize, row as isize));
        }
    }
}
