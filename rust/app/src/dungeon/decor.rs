use std::collections::HashSet;

use crate::tilemap::TileGrid;

use super::build::DungeonRoom;
use super::template::RoomKind;
use super::theme::TrapMix;
use crate::rng::Rng;

const TORCHES_BASE: usize = 2;
const TILES_PER_EXTRA_TORCH: usize = 180;
const MAX_TORCHES: usize = 5;
const NORMAL_BONES: usize = 2;
const NORMAL_BANNERS: usize = 2;
const TREASURE_BONES: usize = 3;
const TREASURE_COBWEBS: usize = 3;
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
    Pot,
    Barrel,
    ExplosiveBarrel,
    PoisonBarrel,
    FrostBarrel,
    Flamethrower,
    FireGeyser,
    IcePatch,
    WebPatch,
    ArrowTrap,
    Icicles { variant: usize },
    IceCrystals { variant: usize },
    FrozenSkull,
    Cocoon { variant: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Decor {
    pub(crate) kind: DecorKind,
    pub(crate) tile: (usize, usize),
}

struct Counts {
    torches: usize,
    banners: usize,
    cobwebs: usize,
    bones: usize,
    spikes: bool,
    objects: bool,
}

impl Counts {
    fn for_room(room: &DungeonRoom, mix: &TrapMix) -> Self {
        match room.kind {
            RoomKind::Normal => Self {
                torches: (TORCHES_BASE + room.columns * room.rows / TILES_PER_EXTRA_TORCH)
                    .min(MAX_TORCHES),
                banners: NORMAL_BANNERS,
                cobwebs: mix.cobwebs,
                bones: NORMAL_BONES,
                spikes: true,
                objects: true,
            },
            RoomKind::Ambush => Self {
                torches: (TORCHES_BASE + room.columns * room.rows / TILES_PER_EXTRA_TORCH)
                    .min(MAX_TORCHES),
                banners: NORMAL_BANNERS,
                cobwebs: 0,
                bones: NORMAL_BONES,
                spikes: false,
                objects: true,
            },
            RoomKind::Treasure => Self {
                torches: 0,
                banners: 0,
                cobwebs: TREASURE_COBWEBS,
                bones: TREASURE_BONES,
                spikes: false,
                objects: false,
            },
            _ => Self {
                torches: 0,
                banners: 0,
                cobwebs: 0,
                bones: 0,
                spikes: false,
                objects: false,
            },
        }
    }
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

type ThemedSlot<'a> = (usize, &'a [Candidate], usize, fn(usize) -> DecorKind);

pub(super) fn patch_tiles(tile: (usize, usize)) -> [(usize, usize); 4] {
    [
        tile,
        (tile.0 + 1, tile.1),
        (tile.0, tile.1 + 1),
        (tile.0 + 1, tile.1 + 1),
    ]
}

#[derive(Debug, Clone, Copy)]
struct Candidate {
    tile: (usize, usize),
    mount: Mount,
}

pub(crate) fn plan(
    grid: &TileGrid,
    room: &DungeonRoom,
    mix: &TrapMix,
    rng: &mut Rng,
) -> Vec<Decor> {
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

    let counts = Counts::for_room(room, mix);
    for _ in 0..counts.torches {
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
    for _ in 0..counts.banners {
        if let Some(spot) = planner.pick(&top_faces, rng) {
            planner.take(spot.tile);
            decor.push(Decor {
                kind: DecorKind::Banner,
                tile: spot.tile,
            });
        }
    }
    for _ in 0..counts.cobwebs {
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
    for _ in 0..counts.bones {
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

    if counts.spikes {
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
        let spikes = 1 + rng.below(mix.spikes);
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
    if counts.objects {
        let patch_floor: Vec<Candidate> = open_floor
            .iter()
            .filter(|candidate| {
                let (column, row) = (candidate.tile.0 as isize, candidate.tile.1 as isize);
                (-1..=2).all(|dy| (-1..=2).all(|dx| !planner.wall(column + dx, row + dy)))
            })
            .copied()
            .collect();
        for (kind, amount) in [
            (DecorKind::FireGeyser, mix.geysers),
            (DecorKind::IcePatch, mix.ice_patches),
            (DecorKind::WebPatch, mix.web_patches),
        ] {
            if amount == 0 {
                continue;
            }
            for _ in 0..1 + rng.below(amount) {
                let usable: Vec<&Candidate> = patch_floor
                    .iter()
                    .filter(|candidate| {
                        patch_tiles(candidate.tile)
                            .iter()
                            .all(|&tile| planner.free(tile) && planner.away_from_doors(tile))
                    })
                    .collect();
                if let Some(spot) = rng.pick(&usable).map(|candidate| **candidate) {
                    for tile in patch_tiles(spot.tile) {
                        planner.take(tile);
                    }
                    decor.push(Decor {
                        kind,
                        tile: spot.tile,
                    });
                }
            }
        }
        let hugging: Vec<Candidate> = open_floor
            .iter()
            .filter(|candidate| {
                let (column, row) = (candidate.tile.0 as isize, candidate.tile.1 as isize);
                [(1, 0), (-1, 0), (0, 1), (0, -1)]
                    .iter()
                    .any(|(dx, dy)| planner.wall(column + dx, row + dy))
            })
            .copied()
            .collect();
        let themed: [ThemedSlot; 4] = [
            (mix.icicles, &top_faces, 3, |variant| DecorKind::Icicles {
                variant,
            }),
            (mix.crystals, &hugging, 2, |variant| {
                DecorKind::IceCrystals { variant }
            }),
            (mix.frozen_skulls, &open_floor, 1, |_| {
                DecorKind::FrozenSkull
            }),
            (mix.cocoons, &hugging, 2, |variant| DecorKind::Cocoon {
                variant,
            }),
        ];
        for (amount, spots, variants, make) in themed {
            if amount == 0 {
                continue;
            }
            for _ in 0..1 + rng.below(amount) {
                if let Some(spot) = planner.pick(spots, rng) {
                    planner.take(spot.tile);
                    decor.push(Decor {
                        kind: make(rng.below(variants)),
                        tile: spot.tile,
                    });
                }
            }
        }
        if mix.arrow_traps > 0 {
            for _ in 0..1 + rng.below(mix.arrow_traps) {
                if let Some(spot) = planner.pick(&top_faces, rng) {
                    planner.take(spot.tile);
                    decor.push(Decor {
                        kind: DecorKind::ArrowTrap,
                        tile: spot.tile,
                    });
                }
            }
        }
        for _ in 0..mix.flamethrowers {
            if let Some(spot) = planner.pick(&top_faces, rng) {
                planner.take(spot.tile);
                decor.push(Decor {
                    kind: DecorKind::Flamethrower,
                    tile: spot.tile,
                });
            }
        }
        let roomy: Vec<Candidate> = open_floor
            .iter()
            .filter(|candidate| {
                let (column, row) = (candidate.tile.0 as isize, candidate.tile.1 as isize);
                [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)]
                    .iter()
                    .all(|(dx, dy)| !planner.wall(column + dx, row + dy))
            })
            .copied()
            .collect();
        let batches = [
            (DecorKind::ExplosiveBarrel, rng.below(mix.explosive_barrels)),
            (DecorKind::Barrel, rng.below(mix.barrels)),
            (DecorKind::Pot, rng.below(mix.pots)),
            (DecorKind::PoisonBarrel, rng.below(mix.poison_barrels)),
            (DecorKind::FrostBarrel, rng.below(mix.frost_barrels)),
        ];
        for (kind, total) in batches {
            for _ in 0..total {
                if let Some(spot) = planner.pick(&roomy, rng) {
                    planner.take(spot.tile);
                    decor.push(Decor {
                        kind,
                        tile: spot.tile,
                    });
                }
            }
        }
    }
    decor
}

#[cfg(test)]
mod tests {
    use crate::dungeon::{Dungeon, Theme};

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

    fn count_in(theme: Theme, kind: DecorKind) -> usize {
        (0..40)
            .map(|seed| {
                let mut rng = Rng::new(seed);
                let dungeon = Dungeon::generate_themed(&mut rng, theme).expect("a dungeon");
                dungeon
                    .rooms
                    .iter()
                    .flat_map(|room| &room.decor)
                    .filter(|decor| decor.kind == kind)
                    .count()
            })
            .sum()
    }

    #[test]
    fn each_theme_leans_on_its_own_hazards() {
        assert_eq!(count_in(Theme::Sewer, DecorKind::Flamethrower), 0);
        assert!(
            count_in(Theme::Lava, DecorKind::Flamethrower)
                > 2 * count_in(Theme::Stone, DecorKind::Flamethrower)
        );
        assert_eq!(count_in(Theme::Lava, DecorKind::FrostBarrel), 0);
        assert!(
            count_in(Theme::Ice, DecorKind::FrostBarrel)
                > count_in(Theme::Stone, DecorKind::FrostBarrel)
        );
        assert!(
            count_in(Theme::Sewer, DecorKind::PoisonBarrel)
                > count_in(Theme::Stone, DecorKind::PoisonBarrel)
        );
        assert!(
            count_in(Theme::Temple, DecorKind::Spike) > count_in(Theme::Stone, DecorKind::Spike)
        );
        assert!(count_in(Theme::Lava, DecorKind::FireGeyser) > 0);
        assert!(count_in(Theme::Spider, DecorKind::WebPatch) > 0);
        assert!(count_in(Theme::Temple, DecorKind::ArrowTrap) > 0);
        assert_eq!(count_in(Theme::Stone, DecorKind::FireGeyser), 0);
        assert_eq!(count_in(Theme::Sewer, DecorKind::ArrowTrap), 0);
    }

    #[test]
    fn the_dungeon_remembers_its_theme() {
        let mut rng = Rng::new(3);
        let dungeon = Dungeon::generate_themed(&mut rng, Theme::Ice).expect("a dungeon");
        assert_eq!(dungeon.theme, Theme::Ice);
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
                        | DecorKind::Flamethrower
                        | DecorKind::ArrowTrap
                        | DecorKind::Icicles { .. }
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
                        DecorKind::Pot
                        | DecorKind::Barrel
                        | DecorKind::ExplosiveBarrel
                        | DecorKind::PoisonBarrel
                        | DecorKind::FrostBarrel => {
                            for (dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
                                assert!(!dungeon.grid.is_wall(column + dx, row + dy), "{decor:?}");
                            }
                        }
                        DecorKind::FireGeyser | DecorKind::IcePatch | DecorKind::WebPatch => {
                            for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                                assert!(!dungeon.grid.is_wall(column + dx, row + dy), "{decor:?}");
                            }
                        }
                        DecorKind::Bones { .. }
                        | DecorKind::Spike
                        | DecorKind::IceCrystals { .. }
                        | DecorKind::FrozenSkull
                        | DecorKind::Cocoon { .. } => {
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
    fn furnished_rooms_have_no_random_torches_and_plain_ones_have_nothing() {
        let mut seen_treasure = false;
        for seed in 0..40 {
            for room in &dungeon(seed).rooms {
                let furnished = matches!(
                    room.kind,
                    RoomKind::Shop | RoomKind::Treasure | RoomKind::Sacrifice | RoomKind::Start
                );
                assert_eq!(room.furnishing.is_some(), furnished, "{:?}", room.kind);
                if !matches!(room.kind, RoomKind::Normal | RoomKind::Ambush) {
                    assert_eq!((room.columns, room.rows), (20, 16), "{:?}", room.kind);
                }
                match room.kind {
                    RoomKind::Treasure => {
                        seen_treasure = true;
                        assert!(room.decor.len() <= 6);
                        assert!(room.decor.iter().all(|d| matches!(
                            d.kind,
                            DecorKind::Bones { .. } | DecorKind::Cobweb { .. }
                        )));
                    }
                    RoomKind::Normal | RoomKind::Ambush => {}
                    _ => assert!(room.decor.is_empty(), "{:?}", room.kind),
                }
            }
        }
        assert!(seen_treasure);
    }
}
