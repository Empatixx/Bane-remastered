use std::collections::HashSet;

use crate::tilemap::TileGrid;

use super::build::DungeonRoom;
use super::rng::Rng;
use super::template::RoomKind;

const TORCHES_BASE: usize = 2;
const TILES_PER_EXTRA_TORCH: usize = 180;
const MAX_TORCHES: usize = 5;
const BONES: usize = 2;
const COBWEBS: usize = 2;
const BANNERS: usize = 2;
const MAX_SPIKES: usize = 2;
const DISABLED_TORCH_PERCENT: usize = 10;
const DOOR_CLEARANCE: usize = 3;
const SPIKE_WALL_CLEARANCE: isize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mount {
    Top,
    FacingRight,
    FacingLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DecorKind {
    Torch { mount: Mount, lit: bool },
    Bones { variant: usize },
    Cobweb { variant: usize, left: bool },
    Banner,
    Spike,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Decor {
    pub(crate) kind: DecorKind,
    pub(crate) tile: (usize, usize),
}

struct Planner<'a> {
    grid: &'a TileGrid,
    room: &'a DungeonRoom,
    taken: HashSet<(usize, usize)>,
}

impl Planner<'_> {
    fn wall(&self, column: isize, row: isize) -> bool {
        self.grid.is_wall(column, row)
    }

    fn free(&self, tile: (usize, usize)) -> bool {
        !self.taken.contains(&tile)
    }

    fn away_from_doors(&self, tile: (usize, usize)) -> bool {
        self.room.doors.iter().all(|door| {
            door.tiles.iter().all(|&(column, row)| {
                column.abs_diff(tile.0) + row.abs_diff(tile.1) >= DOOR_CLEARANCE
            })
        })
    }

    fn room_tiles(&self) -> Vec<(usize, usize)> {
        let mut tiles = Vec::new();
        for row in self.room.origin.1..self.room.origin.1 + self.room.rows {
            for column in self.room.origin.0..self.room.origin.0 + self.room.columns {
                tiles.push((column, row));
            }
        }
        tiles
    }

    fn take(&mut self, tile: (usize, usize)) {
        self.taken.insert(tile);
    }

    fn pick(&self, candidates: &[Candidate], rng: &mut Rng) -> Option<Candidate> {
        let usable: Vec<&Candidate> = candidates
            .iter()
            .filter(|candidate| self.free(candidate.tile) && self.away_from_doors(candidate.tile))
            .collect();
        rng.pick(&usable).map(|candidate| **candidate)
    }
}

#[derive(Debug, Clone, Copy)]
struct Candidate {
    tile: (usize, usize),
    mount: Mount,
}

pub(crate) fn plan(grid: &TileGrid, room: &DungeonRoom, rng: &mut Rng) -> Vec<Decor> {
    let mut planner = Planner {
        grid,
        room,
        taken: room.spawns.iter().copied().collect(),
    };
    let tiles = planner.room_tiles();
    let mut decor = Vec::new();

    let top_faces: Vec<Candidate> = tiles
        .iter()
        .filter(|&&(column, row)| {
            planner.wall(column as isize, row as isize)
                && !planner.wall(column as isize, row as isize + 1)
        })
        .map(|&tile| Candidate {
            tile,
            mount: Mount::Top,
        })
        .collect();
    let side_faces: Vec<Candidate> = tiles
        .iter()
        .flat_map(|&(column, row)| {
            let (c, r) = (column as isize, row as isize);
            let mut found = Vec::new();
            if planner.wall(c, r) && !planner.wall(c - 1, r) && column > 0 {
                found.push(Candidate {
                    tile: (column - 1, row),
                    mount: Mount::FacingRight,
                });
            }
            if planner.wall(c, r) && !planner.wall(c + 1, r) {
                found.push(Candidate {
                    tile: (column + 1, row),
                    mount: Mount::FacingLeft,
                });
            }
            found
        })
        .filter(|candidate| room.contains_tile(candidate.tile.0, candidate.tile.1))
        .collect();
    let mut wall_spots = top_faces.clone();
    wall_spots.extend(side_faces.iter().copied());

    let torches =
        (TORCHES_BASE + room.columns * room.rows / TILES_PER_EXTRA_TORCH).min(MAX_TORCHES);
    for _ in 0..torches {
        if let Some(spot) = planner.pick(&wall_spots, rng) {
            planner.take(spot.tile);
            decor.push(Decor {
                kind: DecorKind::Torch {
                    mount: spot.mount,
                    lit: rng.below(100) >= DISABLED_TORCH_PERCENT,
                },
                tile: spot.tile,
            });
        }
    }
    for _ in 0..BANNERS {
        if let Some(spot) = planner.pick(&top_faces, rng) {
            planner.take(spot.tile);
            decor.push(Decor {
                kind: DecorKind::Banner,
                tile: spot.tile,
            });
        }
    }
    for _ in 0..COBWEBS {
        if let Some(spot) = planner.pick(&side_faces, rng) {
            planner.take(spot.tile);
            decor.push(Decor {
                kind: DecorKind::Cobweb {
                    variant: rng.below(2),
                    left: spot.mount == Mount::FacingRight,
                },
                tile: spot.tile,
            });
        }
    }

    let open_floor: Vec<Candidate> = tiles
        .iter()
        .filter(|&&(column, row)| {
            !planner.wall(column as isize, row as isize) && room.contains_inner_tile(column, row, 1)
        })
        .map(|&tile| Candidate {
            tile,
            mount: Mount::Top,
        })
        .collect();
    for _ in 0..BONES {
        if let Some(spot) = planner.pick(&open_floor, rng) {
            planner.take(spot.tile);
            decor.push(Decor {
                kind: DecorKind::Bones {
                    variant: rng.below(2),
                },
                tile: spot.tile,
            });
        }
    }

    if room.kind == RoomKind::Normal {
        let spike_floor: Vec<Candidate> = open_floor
            .iter()
            .filter(|candidate| {
                let (column, row) = (candidate.tile.0 as isize, candidate.tile.1 as isize);
                (-SPIKE_WALL_CLEARANCE..=SPIKE_WALL_CLEARANCE).all(|dy| {
                    (-SPIKE_WALL_CLEARANCE..=SPIKE_WALL_CLEARANCE)
                        .all(|dx| !planner.wall(column + dx, row + dy))
                })
            })
            .copied()
            .collect();
        let spikes = 1 + rng.below(MAX_SPIKES);
        for _ in 0..spikes {
            if let Some(spot) = planner.pick(&spike_floor, rng) {
                planner.take(spot.tile);
                decor.push(Decor {
                    kind: DecorKind::Spike,
                    tile: spot.tile,
                });
            }
        }
    }
    decor
}

#[cfg(test)]
mod tests {
    use crate::dungeon::Dungeon;

    use super::*;

    fn dungeon(seed: u64) -> Dungeon {
        let mut rng = Rng::new(seed);
        Dungeon::generate(&mut rng).expect("a dungeon is generated")
    }

    #[test]
    fn rooms_get_decoration_and_normal_rooms_get_spikes() {
        let mut spikes = 0;
        let mut torches = 0;
        for seed in 0..40 {
            for room in &dungeon(seed).rooms {
                torches += room
                    .decor
                    .iter()
                    .filter(|decor| matches!(decor.kind, DecorKind::Torch { .. }))
                    .count();
                let room_spikes = room
                    .decor
                    .iter()
                    .filter(|decor| decor.kind == DecorKind::Spike)
                    .count();
                if room.kind != RoomKind::Normal {
                    assert_eq!(room_spikes, 0, "{:?} has spikes", room.kind);
                }
                spikes += room_spikes;
            }
        }
        assert!(torches > 40 * 8, "{torches} torches");
        assert!(spikes > 0);
    }

    #[test]
    fn decoration_sits_on_valid_tiles_and_never_overlaps() {
        for seed in 0..40 {
            let dungeon = dungeon(seed);
            for room in &dungeon.rooms {
                let mut seen = HashSet::new();
                for decor in &room.decor {
                    assert!(seen.insert(decor.tile), "seed {seed}: overlap {decor:?}");
                    assert!(room.contains_tile(decor.tile.0, decor.tile.1));
                    assert!(!room.spawns.contains(&decor.tile));
                    let (column, row) = (decor.tile.0 as isize, decor.tile.1 as isize);
                    match decor.kind {
                        DecorKind::Banner
                        | DecorKind::Torch {
                            mount: Mount::Top, ..
                        } => {
                            assert!(dungeon.grid.is_wall(column, row), "{decor:?}");
                            assert!(!dungeon.grid.is_wall(column, row + 1), "{decor:?}");
                        }
                        DecorKind::Torch {
                            mount: Mount::FacingRight,
                            ..
                        }
                        | DecorKind::Cobweb { left: true, .. } => {
                            assert!(!dungeon.grid.is_wall(column, row), "{decor:?}");
                            assert!(dungeon.grid.is_wall(column + 1, row), "{decor:?}");
                        }
                        DecorKind::Torch {
                            mount: Mount::FacingLeft,
                            ..
                        }
                        | DecorKind::Cobweb { left: false, .. } => {
                            assert!(!dungeon.grid.is_wall(column, row), "{decor:?}");
                            assert!(dungeon.grid.is_wall(column - 1, row), "{decor:?}");
                        }
                        DecorKind::Bones { .. } | DecorKind::Spike => {
                            assert!(!dungeon.grid.is_wall(column, row), "{decor:?}");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn spikes_keep_their_distance_from_walls_and_doors() {
        for seed in 0..40 {
            let dungeon = dungeon(seed);
            for room in &dungeon.rooms {
                for decor in room.decor.iter().filter(|d| d.kind == DecorKind::Spike) {
                    let (column, row) = (decor.tile.0 as isize, decor.tile.1 as isize);
                    for dy in -2..=2 {
                        for dx in -2..=2 {
                            assert!(!dungeon.grid.is_wall(column + dx, row + dy));
                        }
                    }
                    for door in &room.doors {
                        for &(c, r) in &door.tiles {
                            assert!(c.abs_diff(decor.tile.0) + r.abs_diff(decor.tile.1) >= 3);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn the_same_seed_decorates_the_same_way() {
        let first: Vec<Vec<Decor>> = dungeon(17).rooms.into_iter().map(|r| r.decor).collect();
        let second: Vec<Vec<Decor>> = dungeon(17).rooms.into_iter().map(|r| r.decor).collect();
        assert_eq!(first, second);
    }

    #[test]
    fn shops_are_furnished_by_the_shop_layout_only() {
        let mut shops = 0;
        for seed in 0..40 {
            for room in &dungeon(seed).rooms {
                let is_shop = room.kind == RoomKind::Shop;
                assert_eq!(room.shop.is_some(), is_shop);
                if is_shop {
                    shops += 1;
                    assert!(room.decor.is_empty());
                    assert_eq!((room.columns, room.rows), (20, 16));
                }
            }
        }
        assert!(shops > 0);
    }
}
