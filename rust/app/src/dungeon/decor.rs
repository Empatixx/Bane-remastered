use std::collections::HashSet;

use crate::tilemap::TileGrid;

use super::build::DungeonRoom;
use super::template::RoomKind;
use super::theme::{Theme, TrapMix};
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
/// The gauntlet's spike rings, as distances (in tiles, the larger of across and down) from the
/// middle of the room: a platform for the chest inside the first, a ring to stand on and time
/// the next between them.
const GAUNTLET_RINGS: [(f32, f32); 2] = [(2.0, 4.0), (5.0, 6.0)];
/// The same with a safe ring two tiles wide, room for ice or webs to wait on.
pub(super) const GAUNTLET_WIDE_RINGS: [(f32, f32); 2] = [(2.0, 3.0), (5.0, 6.0)];
/// The chest's platform in the middle of a gauntlet, clear of traps, as a distance like the rings'.
const GAUNTLET_PLATFORM: f32 = 2.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mount {
    Top,
    FacingRight,
    FacingLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DecorKind {
    Torch {
        mount: Mount,
        lit: bool,
    },
    Bones {
        variant: usize,
    },
    Cobweb {
        variant: usize,
        left: bool,
    },
    Banner,
    /// The moss biome's banner, faded and overgrown.
    MossyBanner,
    /// Moss growing over the top of a wall.
    MossDrape {
        variant: usize,
    },
    Spike,
    /// A spike rising while the plain ones sink, for gauntlets whose spikes take turns.
    OffbeatSpike,
    Pot,
    Barrel,
    ExplosiveBarrel,
    PoisonBarrel,
    FrostBarrel,
    Flamethrower,
    WebPatch,
    ArrowTrap,
    Icicles {
        variant: usize,
    },
    IceCrystals {
        variant: usize,
    },
    FrozenSkull,
    Mushrooms {
        variant: usize,
    },
    Cocoon {
        variant: usize,
    },
    /// A clutch of spider eggs in silk against a wall.
    Eggs {
        variant: usize,
    },
    /// A victim wrapped in webs.
    Wrapped {
        variant: usize,
    },
    /// A bowl of fire on a low pedestal against a wall.
    FireBowl,
    /// Shackles on chains in a wall's face, empty or with a skeleton hung in them.
    Shackles {
        variant: usize,
    },
    /// A piece of a toppled statue: its head, its torso or a column drum.
    StatuePiece {
        variant: usize,
    },
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

/// How a gauntlet lays its spikes between the doors and the chest.
#[derive(Debug, Clone, Copy, PartialEq)]
enum SpikeField {
    /// Rings round the chest (as distances from the middle) with a safe ring between them, all
    /// rising together.
    Rings(&'static [(f32, f32)]),
    /// A checkerboard over the floor, the two colours rising in turn: step onto the squares as
    /// they sink.
    Checker,
    /// One-tile rings from the platform out, each rising out of step with its neighbours, like
    /// ripples round the chest.
    Ripple,
}

/// Each biome's gauntlet.
#[derive(Debug, Clone, Copy)]
struct GauntletStyle {
    spikes: SpikeField,
    /// Every this many tiles of the top wall (away from the doors) a trap, these in turn.
    wall_step: usize,
    wall_traps: &'static [DecorKind],
    /// Webs laid over the safe ring between the spikes, to wade through.
    cover: Option<DecorKind>,
    /// Barrels at the corners of the chest's platform.
    guards: Option<DecorKind>,
}

const ARROWS_AND_FLAMES: &[DecorKind] = &[DecorKind::ArrowTrap, DecorKind::Flamethrower];
const ARROWS: &[DecorKind] = &[DecorKind::ArrowTrap];
const FLAMES: &[DecorKind] = &[DecorKind::Flamethrower];

fn gauntlet_style(theme: Theme, room: &DungeonRoom) -> GauntletStyle {
    let style = |spikes, wall_step, wall_traps| GauntletStyle {
        spikes,
        wall_step,
        wall_traps,
        cover: None,
        guards: None,
    };
    match theme {
        // Picked by where the room lies, not from the floor's random stream, so a gauntlet
        // never changes the rest of the floor.
        Theme::Stone if (room.origin.0 + room.origin.1).is_multiple_of(2) => {
            style(SpikeField::Rings(&GAUNTLET_RINGS), 3, ARROWS_AND_FLAMES)
        }
        Theme::Stone | Theme::Temple => style(SpikeField::Checker, 3, ARROWS_AND_FLAMES),
        Theme::Moss => GauntletStyle {
            guards: Some(DecorKind::PoisonBarrel),
            ..style(SpikeField::Ripple, 3, ARROWS)
        },
        Theme::Sewer => style(SpikeField::Ripple, 2, ARROWS),
        // Its safe ring is ice, laid with the floor's surface.
        Theme::Ice => style(SpikeField::Rings(&GAUNTLET_WIDE_RINGS), 3, ARROWS),
        Theme::Spider => GauntletStyle {
            cover: Some(DecorKind::WebPatch),
            ..style(SpikeField::Rings(&GAUNTLET_WIDE_RINGS), 3, ARROWS)
        },
        Theme::Lava => GauntletStyle {
            guards: Some(DecorKind::ExplosiveBarrel),
            ..style(SpikeField::Checker, 2, FLAMES)
        },
    }
}

/// The gauntlet's traps: spikes between the doors and the chest in the middle of the room, laid
/// out as the biome likes them, and traps along the top wall shooting and burning down across
/// them. No foes: getting through is the fight.
fn gauntlet_traps(planner: &mut Planner<'_>, top_faces: &[Candidate], theme: Theme) -> Vec<Decor> {
    let room = planner.room;
    let style = gauntlet_style(theme, room);
    let middle = (
        room.origin.0 as f32 + room.columns as f32 / 2.0,
        room.origin.1 as f32 + room.rows as f32 / 2.0,
    );
    let reach = |(column, row): (usize, usize)| {
        (column as f32 + 0.5 - middle.0)
            .abs()
            .max((row as f32 + 0.5 - middle.1).abs())
    };
    let mut decor = Vec::new();
    for tile in planner.room_tiles() {
        let (column, row) = (tile.0 as isize, tile.1 as isize);
        let clear = (-SPIKE_WALL_CLEARANCE..=SPIKE_WALL_CLEARANCE)
            .all(|dy| (-SPIKE_WALL_CLEARANCE..=SPIKE_WALL_CLEARANCE).all(|dx| !planner.wall(column + dx, row + dy)));
        let reach = reach(tile);
        if !clear || reach < GAUNTLET_PLATFORM || !planner.free(tile) || !planner.away_from_doors(tile) {
            continue;
        }
        let kind = match style.spikes {
            SpikeField::Rings(rings) => rings
                .iter()
                .any(|&(from, to)| reach > from && reach < to)
                .then_some(DecorKind::Spike),
            SpikeField::Checker => Some(if (tile.0 + tile.1).is_multiple_of(2) {
                DecorKind::Spike
            } else {
                DecorKind::OffbeatSpike
            }),
            SpikeField::Ripple => Some(if (reach as usize).is_multiple_of(2) {
                DecorKind::Spike
            } else {
                DecorKind::OffbeatSpike
            }),
        };
        if let Some(kind) = kind {
            planner.take(tile);
            decor.push(Decor { kind, tile });
        }
    }
    if let Some(kind) = style.cover {
        // Patches two tiles square, packed over the safe ring inside the outer spikes.
        let (from, to) = (GAUNTLET_WIDE_RINGS[0].1, GAUNTLET_WIDE_RINGS[1].0);
        for tile in planner.room_tiles() {
            let (column, row) = (tile.0 as isize, tile.1 as isize);
            let fits = (-1..=2).all(|dy| (-1..=2).all(|dx| !planner.wall(column + dx, row + dy)))
                && patch_tiles(tile).iter().all(|&tile| {
                    planner.free(tile) && planner.away_from_doors(tile) && reach(tile) > from && reach(tile) < to
                });
            if fits {
                for tile in patch_tiles(tile) {
                    planner.take(tile);
                }
                decor.push(Decor { kind, tile });
            }
        }
    }
    if let Some(kind) = style.guards {
        let (left, top) = (middle.0 as usize - 2, middle.1 as usize - 2);
        for tile in [(left, top), (left + 3, top), (left, top + 3), (left + 3, top + 3)] {
            planner.take(tile);
            decor.push(Decor { kind, tile });
        }
    }
    let mut walls: Vec<Candidate> = top_faces
        .iter()
        .filter(|candidate| planner.away_from_doors(candidate.tile))
        .copied()
        .collect();
    walls.sort_by_key(|candidate| candidate.tile.0);
    let step = style.wall_step;
    for (index, spot) in walls.iter().enumerate().filter(|(index, _)| index % step == step / 2) {
        let kind = style.wall_traps[(index / step) % style.wall_traps.len()];
        planner.take(spot.tile);
        decor.push(Decor {
            kind,
            tile: spot.tile,
        });
    }
    decor
}

pub(crate) fn plan(
    grid: &TileGrid,
    room: &DungeonRoom,
    theme: Theme,
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
    if room.kind == RoomKind::Gauntlet {
        return gauntlet_traps(&mut planner, &top_faces, theme);
    }

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
                kind: if mix.mossy {
                    DecorKind::MossyBanner
                } else {
                    DecorKind::Banner
                },
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
        for (kind, amount) in [(DecorKind::WebPatch, mix.web_patches)] {
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
        let themed: [ThemedSlot; 8] = [
            (mix.moss_drapes, &top_faces, 3, |variant| {
                DecorKind::MossDrape { variant }
            }),
            (mix.mushrooms, &hugging, 2, |variant| DecorKind::Mushrooms {
                variant,
            }),
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
            (mix.eggs, &hugging, 2, |variant| DecorKind::Eggs { variant }),
            (mix.wrapped, &open_floor, 2, |variant| DecorKind::Wrapped { variant }),
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
        // Now and then (not in every room) shackles in a wall, a bowl of fire by a wall, or a
        // piece of a statue.
        let rare: [ThemedSlot; 3] = [
            (mix.shackles_percent, &top_faces, 2, |variant| DecorKind::Shackles { variant }),
            (mix.fire_bowl_percent, &hugging, 1, |_| DecorKind::FireBowl),
            (mix.statue_percent, &open_floor, 3, |variant| DecorKind::StatuePiece { variant }),
        ];
        for (percent, spots, variants, make) in rare {
            if percent == 0 || rng.below(100) >= percent {
                continue;
            }
            if let Some(spot) = planner.pick(spots, rng) {
                planner.take(spot.tile);
                decor.push(Decor {
                    kind: make(rng.below(variants)),
                    tile: spot.tile,
                });
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

    const THEMES: [Theme; 7] = [
        Theme::Stone,
        Theme::Sewer,
        Theme::Moss,
        Theme::Spider,
        Theme::Ice,
        Theme::Lava,
        Theme::Temple,
    ];

    fn is_spike(kind: DecorKind) -> bool {
        matches!(kind, DecorKind::Spike | DecorKind::OffbeatSpike)
    }

    fn gauntlets(theme: Theme) -> Vec<DungeonRoom> {
        (0..40)
            .flat_map(|seed| {
                let mut rng = Rng::new(seed);
                Dungeon::generate_themed(&mut rng, theme).expect("a dungeon").rooms
            })
            .filter(|room| room.kind == RoomKind::Gauntlet)
            .collect()
    }

    #[test]
    fn every_biome_has_its_gauntlet_of_spikes_and_wall_traps_round_a_clear_chest() {
        for theme in THEMES {
            let rooms = gauntlets(theme);
            assert!(rooms.len() > 8, "{theme:?}: only {} gauntlets", rooms.len());
            for room in &rooms {
                let count = |kind: fn(DecorKind) -> bool| room.decor.iter().filter(|d| kind(d.kind)).count();
                assert!(count(is_spike) >= 40, "{theme:?}");
                assert!(count(|k| matches!(k, DecorKind::ArrowTrap | DecorKind::Flamethrower)) >= 4, "{theme:?}");
                // The chest's platform in the middle is clear of everything but its guards.
                let middle = (room.origin.0 + room.columns / 2, room.origin.1 + room.rows / 2);
                for d in &room.decor {
                    let near = d.tile.0 + 1 >= middle.0
                        && d.tile.0 <= middle.0
                        && d.tile.1 + 1 >= middle.1
                        && d.tile.1 <= middle.1;
                    assert!(!near, "{theme:?}: {:?} by the chest at {:?}", d.kind, d.tile);
                }
                assert!(room.spawns.is_empty(), "a gauntlet has no foes");
            }
        }
    }

    #[test]
    fn biomes_lay_their_gauntlets_out_differently() {
        let has = |theme, kind: fn(DecorKind) -> bool| {
            gauntlets(theme).iter().all(|room| room.decor.iter().any(|d| kind(d.kind)))
        };
        // Spikes taking turns on a checkerboard or in ripples; plain rings in the ice.
        assert!(has(Theme::Temple, |k| k == DecorKind::OffbeatSpike));
        assert!(has(Theme::Sewer, |k| k == DecorKind::OffbeatSpike));
        assert!(!has(Theme::Ice, |k| k == DecorKind::OffbeatSpike));
        assert!(has(Theme::Spider, |k| k == DecorKind::WebPatch));
        assert!(has(Theme::Lava, |k| k == DecorKind::ExplosiveBarrel));
        assert!(!has(Theme::Lava, |k| k == DecorKind::ArrowTrap));
        assert!(has(Theme::Moss, |k| k == DecorKind::PoisonBarrel));
        assert!(!has(Theme::Sewer, |k| k == DecorKind::Flamethrower));
    }

    #[test]
    fn checkerboard_neighbours_take_turns() {
        for room in gauntlets(Theme::Temple) {
            let kind_at = |tile| room.decor.iter().find(|d| d.tile == tile).map(|d| d.kind);
            for d in room.decor.iter().filter(|d| is_spike(d.kind)) {
                if let Some(right) = kind_at((d.tile.0 + 1, d.tile.1)).filter(|&k| is_spike(k)) {
                    assert_ne!(right, d.kind, "{:?}", d.tile);
                }
            }
        }
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
                if !matches!(room.kind, RoomKind::Normal | RoomKind::Gauntlet) {
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
                // Gauntlets have traps of their own, laid out by biome.
                dungeon
                    .rooms
                    .iter()
                    .filter(|room| room.kind != RoomKind::Gauntlet)
                    .flat_map(|room| &room.decor)
                    .filter(|decor| decor.kind == kind)
                    .count()
            })
            .sum()
    }

    #[test]
    fn moss_floors_have_moss_on_the_walls_and_overgrown_banners_and_others_do_not() {
        let drapes = |theme| {
            (0..3)
                .map(|variant| count_in(theme, DecorKind::MossDrape { variant }))
                .sum::<usize>()
        };
        assert!(drapes(Theme::Moss) > 0);
        assert!(count_in(Theme::Moss, DecorKind::MossyBanner) > 0);
        assert_eq!(count_in(Theme::Moss, DecorKind::Banner), 0);
        for theme in [Theme::Stone, Theme::Ice, Theme::Lava, Theme::Temple] {
            assert_eq!(drapes(theme), 0, "{theme:?}");
            assert_eq!(count_in(theme, DecorKind::MossyBanner), 0, "{theme:?}");
        }
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
        assert!(count_in(Theme::Spider, DecorKind::WebPatch) > 0);
        assert!(
            count_in(Theme::Moss, DecorKind::Mushrooms { variant: 0 })
                + count_in(Theme::Moss, DecorKind::Mushrooms { variant: 1 })
                > 0
        );
        assert_eq!(
            count_in(Theme::Stone, DecorKind::Mushrooms { variant: 0 }),
            0
        );
        assert!(count_in(Theme::Temple, DecorKind::ArrowTrap) > 0);
        assert_eq!(count_in(Theme::Sewer, DecorKind::ArrowTrap), 0);
        assert!(count_in(Theme::Temple, DecorKind::FireBowl) > 0);
        assert_eq!(count_in(Theme::Stone, DecorKind::FireBowl), 0);
        assert!(count_in(Theme::Temple, DecorKind::StatuePiece { variant: 0 }) > 0);
        assert_eq!(count_in(Theme::Spider, DecorKind::StatuePiece { variant: 0 }), 0);
        assert!(
            count_in(Theme::Stone, DecorKind::Shackles { variant: 0 })
                + count_in(Theme::Stone, DecorKind::Shackles { variant: 1 })
                > 0
        );
        assert_eq!(count_in(Theme::Ice, DecorKind::Shackles { variant: 1 }), 0);
    }

    #[test]
    fn the_dungeon_remembers_its_theme() {
        let mut rng = Rng::new(3);
        let dungeon = Dungeon::generate_themed(&mut rng, Theme::Ice).expect("a dungeon");
        assert_eq!(dungeon.theme, Theme::Ice);
    }

    #[test]
    fn decoration_sits_on_valid_tiles_and_never_overlaps() {
        for seed in 0..60 {
            // Every third dungeon is a moss one, for the moss on its walls.
            let dungeon = if seed % 3 == 0 {
                Dungeon::generate_themed(&mut Rng::new(seed), Theme::Moss).expect("a dungeon")
            } else {
                dungeon(seed)
            };
            for room in &dungeon.rooms {
                let mut seen = HashSet::new();
                for decor in &room.decor {
                    assert!(seen.insert(decor.tile), "seed {seed}: overlap {decor:?}");
                    assert!(room.contains_tile(decor.tile.0, decor.tile.1));
                    assert!(!room.spawns.contains(&decor.tile));
                    let (column, row) = (decor.tile.0 as isize, decor.tile.1 as isize);
                    match decor.kind {
                        DecorKind::Banner
                        | DecorKind::MossyBanner
                        | DecorKind::MossDrape { .. }
                        | DecorKind::Flamethrower
                        | DecorKind::ArrowTrap
                        | DecorKind::Icicles { .. }
                        | DecorKind::Shackles { .. }
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
                        DecorKind::WebPatch => {
                            for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                                assert!(!dungeon.grid.is_wall(column + dx, row + dy), "{decor:?}");
                            }
                        }
                        DecorKind::Bones { .. }
                        | DecorKind::Spike
                        | DecorKind::OffbeatSpike
                        | DecorKind::IceCrystals { .. }
                        | DecorKind::FrozenSkull
                        | DecorKind::Mushrooms { .. }
                        | DecorKind::Cocoon { .. }
                        | DecorKind::Eggs { .. }
                        | DecorKind::Wrapped { .. }
                        | DecorKind::FireBowl
                        | DecorKind::StatuePiece { .. } => {
                            assert!(!dungeon.grid.is_wall(column, row), "{decor:?}");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn spikes_keep_their_distance_from_walls_and_doors() {
        for (seed, theme) in (0..40).zip(THEMES.iter().cycle()) {
            let mut rng = Rng::new(seed);
            let dungeon = Dungeon::generate_themed(&mut rng, *theme).expect("a dungeon");
            for room in &dungeon.rooms {
                for decor in room.decor.iter().filter(|d| is_spike(d.kind)) {
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
                    RoomKind::Shop
                        | RoomKind::Treasure
                        | RoomKind::Sacrifice
                        | RoomKind::Start
                        | RoomKind::Gauntlet
                );
                assert_eq!(room.furnishing.is_some(), furnished, "{:?}", room.kind);
                if !matches!(room.kind, RoomKind::Normal | RoomKind::Ambush | RoomKind::Gauntlet) {
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
                    RoomKind::Gauntlet => assert!(room.decor.iter().all(|d| matches!(
                        d.kind,
                        DecorKind::Spike
                            | DecorKind::OffbeatSpike
                            | DecorKind::ArrowTrap
                            | DecorKind::Flamethrower
                            | DecorKind::WebPatch
                            | DecorKind::PoisonBarrel
                            | DecorKind::ExplosiveBarrel
                    ))),
                    _ => assert!(room.decor.is_empty(), "{:?}", room.kind),
                }
            }
        }
        assert!(seen_treasure);
    }
}
