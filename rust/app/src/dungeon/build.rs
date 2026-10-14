use std::error::Error;

use glam::Vec2;

use crate::tilemap::{TILE_SIZE, TileGrid};

use super::decor::{self, Decor};
use super::furnish::{self, Furnishing};
use super::layout::{Direction, Layout, LayoutSpec};
use super::surface::{self, Surfaces};
use super::template::{Cell, RoomKind, RoomTemplate, Templates, door_columns, door_rows};
use super::theme::Theme;
use crate::rng::Rng;

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
    pub(crate) furnishing: Option<Furnishing>,
}

impl DungeonRoom {
    pub(crate) fn pixel_center(&self) -> Vec2 {
        Vec2::new(
            self.origin.0 as f32 + self.columns as f32 / 2.0,
            self.origin.1 as f32 + self.rows as f32 / 2.0,
        ) * TILE_SIZE
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
    pub(crate) theme: Theme,
    pub(crate) surfaces: Surfaces,
}

impl Dungeon {
    #[cfg(test)]
    pub(crate) fn generate(rng: &mut Rng) -> Result<Self, Box<dyn Error>> {
        Self::generate_themed(rng, Theme::Stone)
    }

    #[allow(dead_code)]
    pub(crate) fn generate_themed(rng: &mut Rng, theme: Theme) -> Result<Self, Box<dyn Error>> {
        Self::generate_for_run(rng, theme, true)
    }

    pub(crate) fn generate_for_run(
        rng: &mut Rng,
        theme: Theme,
        shops: bool,
    ) -> Result<Self, Box<dyn Error>> {
        let templates = Templates::built_in()?;
        let mut spec = LayoutSpec::standard(rng);
        if !shops {
            spec.shops = 0;
        }
        let layout = Layout::generate(&spec, rng).ok_or("could not lay out the rooms")?;
        Self::build(&layout, &templates, theme, rng)
    }

    pub(crate) fn build(
        layout: &Layout,
        templates: &Templates,
        theme: Theme,
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
        let mut picked: Vec<&RoomTemplate> = Vec::with_capacity(layout.rooms.len());
        for placed in &layout.rooms {
            picked.push(
                templates
                    .pick(placed.kind, rng)
                    .ok_or_else(|| format!("no template for {:?}", placed.kind))?,
            );
        }
        let mut widths = vec![0; cells_x];
        let mut heights = vec![0; cells_y];
        for (placed, template) in layout.rooms.iter().zip(&picked) {
            let (x, y) = (
                (placed.cell.0 - min_x) as usize,
                (placed.cell.1 - min_y) as usize,
            );
            widths[x] = widths[x].max(template.columns());
            heights[y] = heights[y].max(template.rows());
        }
        let offsets = |sizes: &[usize]| {
            let mut at = MARGIN;
            sizes
                .iter()
                .map(|size| {
                    let start = at;
                    at += size + CORRIDOR_LENGTH;
                    start
                })
                .collect::<Vec<usize>>()
        };
        let (column_starts, row_starts) = (offsets(&widths), offsets(&heights));
        let columns = widths.iter().sum::<usize>() + (cells_x - 1) * CORRIDOR_LENGTH + 2 * MARGIN;
        let rows = heights.iter().sum::<usize>() + (cells_y - 1) * CORRIDOR_LENGTH + 2 * MARGIN;
        let mut grid = TileGrid::filled(columns, rows);

        let mut rooms = Vec::with_capacity(layout.rooms.len());
        for (placed, template) in layout.rooms.iter().zip(&picked) {
            let (template_columns, template_rows) = (template.columns(), template.rows());
            let (x, y) = (
                (placed.cell.0 - min_x) as usize,
                (placed.cell.1 - min_y) as usize,
            );
            let origin = (
                column_starts[x] + (widths[x] - template_columns) / 2,
                row_starts[y] + (heights[y] - template_rows) / 2,
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
                furnishing: None,
            });
        }

        for (index, placed) in layout.rooms.iter().enumerate() {
            for &(direction, other) in &placed.links {
                if index < other {
                    carve_corridor(&mut grid, &rooms[index], &rooms[other], direction);
                }
            }
        }
        let mix = theme.mix();
        for room in &mut rooms {
            room.furnishing = furnish::plan(room.kind, room.origin, room.columns, room.rows);
            room.decor = decor::plan(&grid, room, &mix, rng);
        }
        let mut surfaces = Surfaces::default();
        if let Some(kind) = theme.surface() {
            for room in &rooms {
                for tile in surface::plan(&grid, room, kind, rng) {
                    surfaces.insert(tile, kind);
                }
            }
        }
        Ok(Self {
            grid,
            rooms,
            theme,
            surfaces,
        })
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
        for room in &self.rooms {
            for door in &room.doors {
                let Some(other) = self.neighbour(room, door.direction) else {
                    continue;
                };
                if corridor_contains(room, other, door.direction, (column, row)) {
                    let (a_min, a_max) = room.bounds();
                    let (b_min, b_max) = other.bounds();
                    let (min, max) = (a_min.min(b_min), a_max.max(b_max));
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

fn corridor_lanes(
    from: &DungeonRoom,
    to: &DungeonRoom,
    direction: Direction,
) -> [((usize, usize), (usize, usize)); 2] {
    let from_tiles = door_tiles(from.origin, from.columns, from.rows, direction);
    let to_tiles = door_tiles(to.origin, to.columns, to.rows, direction.opposite());
    [(from_tiles[0], to_tiles[0]), (from_tiles[1], to_tiles[1])]
}

fn corridor_contains(
    from: &DungeonRoom,
    to: &DungeonRoom,
    direction: Direction,
    tile: (usize, usize),
) -> bool {
    corridor_lanes(from, to, direction)
        .iter()
        .any(|(start, end)| {
            (start.0.min(end.0)..=start.0.max(end.0)).contains(&tile.0)
                && (start.1.min(end.1)..=start.1.max(end.1)).contains(&tile.1)
        })
}

fn carve_corridor(grid: &mut TileGrid, from: &DungeonRoom, to: &DungeonRoom, direction: Direction) {
    for (start, end) in corridor_lanes(from, to, direction) {
        for column in start.0.min(end.0)..=start.0.max(end.0) {
            for row in start.1.min(end.1)..=start.1.max(end.1) {
                grid.carve(column, row);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn a_run_without_shops_gets_no_shop_room_and_the_same_run_with_shops_matches_today() {
        let without =
            Dungeon::generate_for_run(&mut Rng::new(7), Theme::Stone, false).expect("a dungeon");
        assert!(without.rooms.iter().all(|room| room.kind != RoomKind::Shop));

        let with =
            Dungeon::generate_for_run(&mut Rng::new(7), Theme::Stone, true).expect("a dungeon");
        let today = Dungeon::generate_themed(&mut Rng::new(7), Theme::Stone).expect("a dungeon");
        let kinds = |dungeon: &Dungeon| {
            dungeon
                .rooms
                .iter()
                .map(|room| room.kind)
                .collect::<Vec<_>>()
        };
        assert_eq!(kinds(&with), kinds(&today));
    }

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

    #[test]
    fn no_normal_room_is_left_without_enemies() {
        for seed in 0..120 {
            let dungeon = dungeon(seed);
            for room in dungeon
                .rooms
                .iter()
                .filter(|room| room.kind == RoomKind::Normal)
            {
                assert!(
                    room.spawns.len() >= 6,
                    "seed {seed}: {} spawns",
                    room.spawns.len()
                );
            }
        }
    }

    #[test]
    fn big_rooms_fit_inside_the_grid_and_still_connect() {
        let mut biggest = 0;
        for seed in 0..120 {
            let dungeon = dungeon(seed);
            for room in &dungeon.rooms {
                biggest = biggest.max(room.columns * room.rows);
                assert!(room.origin.0 + room.columns < dungeon.grid.columns());
                assert!(room.origin.1 + room.rows < dungeon.grid.rows());
            }
        }
        assert!(biggest >= 36 * 22, "{biggest}");
    }

    #[test]
    fn every_floor_has_one_or_two_ambush_rooms_that_are_dead_ends() {
        for seed in 0..120 {
            let dungeon = dungeon(seed);
            let ambushes: Vec<_> = dungeon
                .rooms
                .iter()
                .filter(|room| room.kind == RoomKind::Ambush)
                .collect();

            assert!((1..=2).contains(&ambushes.len()), "seed {seed}");
            for room in ambushes {
                assert_eq!(room.doors.len(), 1, "seed {seed}");
                assert!(room.spawns.len() >= 16, "seed {seed}");
            }
        }
    }

    #[test]
    fn a_corridor_tile_always_gets_the_camera_region_of_its_own_two_rooms() {
        let mut checked = 0;
        for seed in 0..80 {
            let dungeon = dungeon(seed);
            for (index, room) in dungeon.rooms.iter().enumerate() {
                for door in &room.doors {
                    let Some(other_index) = dungeon.rooms.iter().position(|other| {
                        let (dx, dy) = door.direction.delta();
                        other.cell == (room.cell.0 + dx, room.cell.1 + dy)
                    }) else {
                        continue;
                    };
                    if other_index < index {
                        continue;
                    }
                    let other = &dungeon.rooms[other_index];
                    let (a_min, a_max) = room.bounds();
                    let (b_min, b_max) = other.bounds();
                    let expected = (a_min.min(b_min) * TILE_SIZE, a_max.max(b_max) * TILE_SIZE);
                    let from = door_tiles(room.origin, room.columns, room.rows, door.direction);
                    let to = door_tiles(
                        other.origin,
                        other.columns,
                        other.rows,
                        door.direction.opposite(),
                    );
                    for lane in 0..2 {
                        for column in from[lane].0.min(to[lane].0)..=from[lane].0.max(to[lane].0) {
                            for row in from[lane].1.min(to[lane].1)..=from[lane].1.max(to[lane].1) {
                                if dungeon.room_at(column, row).is_some() {
                                    continue;
                                }
                                assert_eq!(
                                    dungeon.camera_region(column, row),
                                    Some(expected),
                                    "seed {seed}: corridor tile {column},{row}"
                                );
                                checked += 1;
                            }
                        }
                    }
                }
            }
        }
        assert!(checked > 1000, "{checked}");
    }
}
