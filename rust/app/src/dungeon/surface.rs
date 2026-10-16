use std::collections::{HashMap, HashSet, VecDeque};

use glam::Vec2;

use crate::rng::Rng;
use crate::tilemap::{TILE_SIZE, TileGrid};

use super::build::DungeonRoom;
use super::decor::{DecorKind, patch_tiles};
use super::template::RoomKind;

const DOOR_CLEARANCE: usize = 3;
const NEIGHBOURS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const DIAGONALS: [(isize, isize); 4] = [(1, -1), (1, 1), (-1, 1), (-1, -1)];
const SHORE_VARIANTS: usize = 2;
#[cfg(test)]
pub(crate) const BLOB_TILES: usize = 47;

fn reduce(code: u8) -> u8 {
    let mut out = code & 0x0F;
    for index in 0..4 {
        let diagonal = 1u8 << (4 + index);
        let first = 1u8 << index;
        let second = 1u8 << ((index + 1) % 4);
        if code & diagonal != 0 && code & first != 0 && code & second != 0 {
            out |= diagonal;
        }
    }
    out
}

fn blob_rank(code: u8) -> usize {
    let wanted = reduce(code);
    (0..wanted)
        .filter(|candidate| reduce(*candidate) == *candidate)
        .count()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Surface {
    Ice,
    Lava,
    Water,
    Bog,
}

impl Surface {
    fn share(self) -> (usize, usize) {
        match self {
            Self::Ice => (22, 40),
            Self::Lava => (8, 14),
            Self::Water => (16, 30),
            Self::Bog => (14, 26),
        }
    }

    fn blob_limit(self) -> usize {
        match self {
            Self::Ice => 5,
            Self::Lava => 3,
            Self::Water => 4,
            Self::Bog => 4,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Surfaces {
    tiles: HashMap<(usize, usize), Surface>,
    /// Tiles under a footbridge: still water to look at, dry to walk on.
    bridged: HashSet<(usize, usize)>,
}

impl Surfaces {
    pub(crate) fn at(&self, tile: (usize, usize)) -> Option<Surface> {
        self.tiles.get(&tile).copied()
    }

    /// What is underfoot at `point`: nothing on a footbridge, though the water runs under it.
    pub(crate) fn at_point(&self, point: Vec2) -> Option<Surface> {
        if point.x < 0.0 || point.y < 0.0 {
            return None;
        }
        let tile = (
            (point.x / TILE_SIZE) as usize,
            (point.y / TILE_SIZE) as usize,
        );
        if self.bridged.contains(&tile) {
            return None;
        }
        self.at(tile)
    }

    pub(crate) fn bridge(&mut self, tile: (usize, usize)) {
        self.bridged.insert(tile);
    }

    pub(crate) fn insert(&mut self, tile: (usize, usize), surface: Surface) {
        self.tiles.insert(tile, surface);
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = ((usize, usize), Surface)> + '_ {
        self.tiles.iter().map(|(tile, surface)| (*tile, *surface))
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.tiles.len()
    }

    fn shifted(tile: (usize, usize), dx: isize, dy: isize) -> Option<(usize, usize)> {
        let column = tile.0 as isize + dx;
        let row = tile.1 as isize + dy;
        (column >= 0 && row >= 0).then_some((column as usize, row as usize))
    }

    pub(crate) fn liquid_row(&self, grid: &TileGrid, tile: (usize, usize)) -> usize {
        let surface = self.at(tile);
        let same = |dx: isize, dy: isize| match Self::shifted(tile, dx, dy) {
            Some(next) => {
                grid.is_wall(next.0 as isize, next.1 as isize) || self.at(next) == surface
            }
            None => true,
        };
        let mut code = 0u8;
        for (bit, (dx, dy)) in NEIGHBOURS.iter().enumerate() {
            if same(*dx, *dy) {
                code |= 1 << bit;
            }
        }
        for (index, (dx, dy)) in DIAGONALS.iter().enumerate() {
            if same(*dx, *dy) {
                code |= 1 << (4 + index);
            }
        }
        blob_rank(code) * 3 + Self::variant(tile)
    }

    pub(crate) fn shore_at(
        &self,
        grid: &TileGrid,
        tile: (usize, usize),
    ) -> Option<(Surface, usize)> {
        if self.at(tile).is_some() || grid.is_wall(tile.0 as isize, tile.1 as isize) {
            return None;
        }
        let mut mask = 0usize;
        let mut surface = None;
        for (bit, (dx, dy)) in NEIGHBOURS.iter().enumerate() {
            if let Some(found) = Self::shifted(tile, *dx, *dy).and_then(|next| self.at(next)) {
                mask |= 1 << bit;
                surface = Some(found);
            }
        }
        if mask != 0 {
            return surface.map(|surface| {
                (
                    surface,
                    mask * SHORE_VARIANTS + Self::variant(tile) % SHORE_VARIANTS,
                )
            });
        }
        for (index, (dx, dy)) in DIAGONALS.iter().enumerate() {
            if let Some(found) = Self::shifted(tile, *dx, *dy).and_then(|next| self.at(next)) {
                return Some((
                    found,
                    (16 + index) * SHORE_VARIANTS + Self::variant(tile) % SHORE_VARIANTS,
                ));
            }
        }
        None
    }

    pub(crate) fn interior_row(tile: (usize, usize)) -> usize {
        blob_rank(u8::MAX) * 3 + Self::variant(tile)
    }

    pub(crate) fn variant(tile: (usize, usize)) -> usize {
        (tile.0.wrapping_mul(31) ^ tile.1.wrapping_mul(17)) % 3
    }
}

fn walkable(grid: &TileGrid, tile: (usize, usize)) -> bool {
    !grid.is_wall(tile.0 as isize, tile.1 as isize)
}

fn near_door(room: &DungeonRoom, tile: (usize, usize)) -> bool {
    room.doors.iter().any(|door| {
        door.tiles
            .iter()
            .any(|&(column, row)| column.abs_diff(tile.0) + row.abs_diff(tile.1) < DOOR_CLEARANCE)
    })
}

fn blocked_tiles(room: &DungeonRoom) -> HashSet<(usize, usize)> {
    let mut blocked: HashSet<(usize, usize)> = room.spawns.iter().copied().collect();
    for decor in &room.decor {
        match decor.kind {
            DecorKind::IcePatch | DecorKind::WebPatch => {
                blocked.extend(patch_tiles(decor.tile));
                blocked.insert((decor.tile.0, decor.tile.1.saturating_sub(1)));
            }
            _ => {
                blocked.insert(decor.tile);
            }
        }
    }
    blocked
}

fn neighbours_of(tile: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    NEIGHBOURS.iter().filter_map(move |(dx, dy)| {
        let column = tile.0 as isize + dx;
        let row = tile.1 as isize + dy;
        (column >= 0 && row >= 0).then_some((column as usize, row as usize))
    })
}

fn stays_connected(grid: &TileGrid, room: &DungeonRoom, covered: &HashSet<(usize, usize)>) -> bool {
    let mut open: Vec<(usize, usize)> = Vec::new();
    for row in room.origin.1..room.origin.1 + room.rows {
        for column in room.origin.0..room.origin.0 + room.columns {
            if walkable(grid, (column, row)) && !covered.contains(&(column, row)) {
                open.push((column, row));
            }
        }
    }
    let Some(&start) = open.first() else {
        return false;
    };
    let wanted: HashSet<(usize, usize)> = open.iter().copied().collect();
    let mut seen = HashSet::from([start]);
    let mut queue = VecDeque::from([start]);
    while let Some(tile) = queue.pop_front() {
        for next in neighbours_of(tile) {
            if wanted.contains(&next) && seen.insert(next) {
                queue.push_back(next);
            }
        }
    }
    seen.len() == wanted.len()
}

pub(crate) fn plan(
    grid: &TileGrid,
    room: &DungeonRoom,
    surface: Surface,
    rng: &mut Rng,
) -> Vec<(usize, usize)> {
    if !matches!(room.kind, RoomKind::Normal | RoomKind::Ambush) {
        return Vec::new();
    }
    let blocked = blocked_tiles(room);
    let allowed = |tile: (usize, usize)| {
        room.contains_inner_tile(tile.0, tile.1, 1)
            && walkable(grid, tile)
            && !blocked.contains(&tile)
            && !near_door(room, tile)
    };
    let floor_tiles = room.columns * room.rows;
    let (low, high) = surface.share();
    let target = floor_tiles * (low + rng.below(high - low + 1)) / 100;
    let mut covered: HashSet<(usize, usize)> = HashSet::new();
    let mut order: Vec<(usize, usize)> = Vec::new();
    for _ in 0..1 + rng.below(surface.blob_limit()) {
        if covered.len() >= target {
            break;
        }
        let seeds: Vec<(usize, usize)> = (room.origin.1..room.origin.1 + room.rows)
            .flat_map(|row| (room.origin.0..room.origin.0 + room.columns).map(move |c| (c, row)))
            .filter(|tile| allowed(*tile) && !covered.contains(tile))
            .collect();
        let Some(&seed) = rng.pick(&seeds) else {
            break;
        };
        let mut blob = vec![seed];
        let size = 4 + rng.below((target / 2).max(4));
        let mut attempts = 0;
        while blob.len() < size && attempts < size * 12 {
            attempts += 1;
            let from = blob[rng.below(blob.len())];
            let next: Vec<(usize, usize)> = neighbours_of(from)
                .filter(|tile| allowed(*tile) && !covered.contains(tile) && !blob.contains(tile))
                .collect();
            if let Some(&tile) = rng.pick(&next) {
                blob.push(tile);
            }
        }
        let mut trial = covered.clone();
        trial.extend(blob.iter().copied());
        if surface == Surface::Lava && !stays_connected(grid, room, &trial) {
            continue;
        }
        covered = trial;
        order.extend(blob);
    }
    order.sort_unstable();
    order.dedup();
    order
}

#[cfg(test)]
mod tests {
    use crate::dungeon::{Dungeon, Theme};

    use super::*;

    fn dungeon(seed: u64, theme: Theme) -> Dungeon {
        let mut rng = Rng::new(seed);
        Dungeon::generate_themed(&mut rng, theme).expect("a dungeon")
    }

    #[test]
    fn each_theme_has_its_own_surface() {
        assert_eq!(Theme::Ice.surface(), Some(Surface::Ice));
        assert_eq!(Theme::Lava.surface(), Some(Surface::Lava));
        assert_eq!(Theme::Sewer.surface(), Some(Surface::Water));
        assert_eq!(Theme::Spider.surface(), None);
        assert_eq!(Theme::Stone.surface(), None);
        assert_eq!(Theme::Moss.surface(), Some(Surface::Bog));
        assert_eq!(Theme::Temple.surface(), None);
    }

    #[test]
    fn themed_floors_get_surfaces_and_plain_ones_do_not() {
        let with: usize = (0..20)
            .map(|seed| dungeon(seed, Theme::Ice).surfaces.len())
            .sum();
        let without: usize = (0..20)
            .map(|seed| dungeon(seed, Theme::Stone).surfaces.len())
            .sum();
        assert!(with > 100, "{with}");
        assert_eq!(without, 0);
    }

    #[test]
    fn surfaces_stay_on_open_floor_away_from_doors_spawns_and_decor() {
        for seed in 0..30 {
            for theme in [Theme::Ice, Theme::Lava, Theme::Sewer, Theme::Moss] {
                let dungeon = dungeon(seed, theme);
                for (tile, _) in dungeon.surfaces.iter() {
                    assert!(!dungeon.grid.is_wall(tile.0 as isize, tile.1 as isize));
                    let room = dungeon
                        .rooms
                        .iter()
                        .find(|room| room.contains_tile(tile.0, tile.1))
                        .expect("a room owns the tile");
                    assert!(matches!(room.kind, RoomKind::Normal | RoomKind::Ambush));
                    assert!(!room.spawns.contains(&tile));
                    assert!(!near_door(room, tile), "{tile:?}");
                    // Frozen skulls are moved onto the ice afterwards; nothing else lies on it.
                    assert!(room.decor.iter().all(|decor| {
                        decor.tile != tile || decor.kind == DecorKind::FrozenSkull
                    }));
                }
            }
        }
    }

    #[test]
    fn lava_never_cuts_a_room_in_two() {
        for seed in 0..40 {
            let dungeon = dungeon(seed, Theme::Lava);
            for room in &dungeon.rooms {
                let covered: HashSet<(usize, usize)> = dungeon
                    .surfaces
                    .iter()
                    .filter(|(tile, _)| room.contains_tile(tile.0, tile.1))
                    .map(|(tile, _)| tile)
                    .collect();
                assert!(
                    covered.is_empty() || stays_connected(&dungeon.grid, room, &covered),
                    "seed {seed}"
                );
            }
        }
    }

    #[test]
    fn there_are_forty_seven_distinct_liquid_shapes() {
        let valid = (0..=255u8).filter(|code| reduce(*code) == *code).count();
        assert_eq!(valid, BLOB_TILES);
        assert_eq!(blob_rank(255), BLOB_TILES - 1);
        assert_eq!(blob_rank(0), 0);
    }

    #[test]
    fn the_surface_sprites_have_a_row_for_every_shape_and_variant() {
        assert_eq!(crate::props::SURFACE_ROWS, BLOB_TILES * 3);
    }

    #[test]
    fn a_diagonal_only_counts_when_both_sides_beside_it_are_liquid() {
        assert_eq!(reduce(0b0001_0001), 0b0000_0001);
        assert_eq!(reduce(0b0001_0011), 0b0001_0011);
    }

    #[test]
    fn liquid_next_to_a_wall_has_no_rim_there_but_liquid_next_to_floor_has() {
        let grid = TileGrid::bordered(8, 8);
        let mut surfaces = Surfaces::default();
        surfaces.insert((3, 3), Surface::Water);
        let alone = surfaces.liquid_row(&grid, (3, 3));
        let mut beside_wall = Surfaces::default();
        beside_wall.insert((1, 1), Surface::Water);
        assert_ne!(alone / 3, beside_wall.liquid_row(&grid, (1, 1)) / 3);
        assert!(beside_wall.liquid_row(&grid, (1, 1)) / 3 > alone / 3);
    }

    #[test]
    fn floor_beside_liquid_gets_a_shore_but_far_floor_and_liquid_do_not() {
        let grid = TileGrid::bordered(10, 10);
        let mut surfaces = Surfaces::default();
        surfaces.insert((4, 4), Surface::Ice);
        assert!(surfaces.shore_at(&grid, (5, 4)).is_some());
        assert!(surfaces.shore_at(&grid, (5, 5)).is_some());
        assert!(surfaces.shore_at(&grid, (7, 7)).is_none());
        assert!(surfaces.shore_at(&grid, (4, 4)).is_none());
    }
}
