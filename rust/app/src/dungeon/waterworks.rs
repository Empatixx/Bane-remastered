//! What the pools of the sewer and the lava floor get besides their liquid: outflow pipes (or
//! glowing cracks) in the walls pouring into them, rubbish and barrels (or cooled crusts)
//! drifting on them, and plank footbridges (or basalt bridges) across them.

use std::collections::HashSet;

use crate::rng::Rng;
use crate::tilemap::TileGrid;

use super::build::DungeonRoom;
use super::surface::{Surface, Surfaces};

const MAX_PIPES: usize = 2;
const SECOND_PIPE_PERCENT: usize = 35;
const BRIDGE_PERCENT: usize = 70;
const MIN_BRIDGE: usize = 2;
const MAX_BRIDGE: usize = 6;
const DEBRIS_PERCENT: usize = 75;
const SECOND_DEBRIS_PERCENT: usize = 40;
const RAT_PERCENT: usize = 20;
const BARREL_PERCENT: usize = 35;
const CRUSTS: usize = 3;
/// A pool needs this many tiles before anything floats on it.
const POOL_FOR_FLOATERS: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Debris {
    Plank,
    Crate,
    Bottle,
    Boot,
    RatRaft,
    /// A crust of cooled rock afloat on lava, one of three shapes.
    Crust(usize),
}

impl Debris {
    /// Its column in the debris sheet.
    pub(crate) fn cell(self) -> usize {
        match self {
            Self::Plank => 0,
            Self::Crate => 1,
            Self::Bottle => 2,
            Self::Boot => 3,
            Self::RatRaft => 4,
            Self::Crust(variant) => variant,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FloatingBarrel {
    Plain,
    Explosive,
    Poison,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Floater {
    Debris(Debris),
    Barrel(FloatingBarrel),
}

/// A footbridge over the open water tiles `first..=last` (a row of them, or a column when
/// `upright`), its ends on the dry floor either side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Bridge {
    pub(crate) upright: bool,
    pub(crate) first: (usize, usize),
    pub(crate) last: (usize, usize),
}

impl Bridge {
    pub(crate) fn tiles(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let step = usize::from(self.upright);
        let length = if self.upright {
            self.last.1 + 1 - self.first.1
        } else {
            self.last.0 + 1 - self.first.0
        };
        (0..length).map(move |i| (self.first.0 + i * (1 - step), self.first.1 + i * step))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Waterworks {
    /// Wall tiles with a pipe in their face, the water right below them.
    pub(crate) pipes: Vec<(usize, usize)>,
    pub(crate) floaters: Vec<(Floater, (usize, usize))>,
    pub(crate) bridges: Vec<Bridge>,
}

fn liquid(surfaces: &Surfaces, kind: Surface, (column, row): (usize, usize)) -> bool {
    surfaces.at((column, row)) == Some(kind)
}

fn dry_floor(grid: &TileGrid, surfaces: &Surfaces, (column, row): (usize, usize)) -> bool {
    !grid.is_wall(column as isize, row as isize) && surfaces.at((column, row)).is_none()
}

/// Straight runs of water with dry floor at both ends, across the room's rows and down its
/// columns.
fn crossings(grid: &TileGrid, room: &DungeonRoom, surfaces: &Surfaces, kind: Surface) -> Vec<Bridge> {
    let mut found = Vec::new();
    for upright in [false, true] {
        let (lines, along) = if upright {
            (room.origin.0..room.origin.0 + room.columns, room.origin.1..room.origin.1 + room.rows)
        } else {
            (room.origin.1..room.origin.1 + room.rows, room.origin.0..room.origin.0 + room.columns)
        };
        let tile = |line: usize, at: usize| if upright { (line, at) } else { (at, line) };
        for line in lines {
            let mut at = along.start + 1;
            while at < along.end - 1 {
                if !liquid(surfaces, kind, tile(line, at)) || !dry_floor(grid, surfaces, tile(line, at - 1)) {
                    at += 1;
                    continue;
                }
                let start = at;
                while at < along.end - 1 && liquid(surfaces, kind, tile(line, at)) {
                    at += 1;
                }
                let end = at - 1;
                let length = end + 1 - start;
                if dry_floor(grid, surfaces, tile(line, end + 1))
                    && (MIN_BRIDGE..=MAX_BRIDGE).contains(&length)
                {
                    found.push(Bridge {
                        upright,
                        first: tile(line, start),
                        last: tile(line, end),
                    });
                }
            }
        }
    }
    found
}

/// The pools of the room: its water tiles in connected groups.
fn pools(room: &DungeonRoom, surfaces: &Surfaces, kind: Surface) -> Vec<Vec<(usize, usize)>> {
    let mut seen = HashSet::new();
    let mut found = Vec::new();
    for row in room.origin.1..room.origin.1 + room.rows {
        for column in room.origin.0..room.origin.0 + room.columns {
            if !liquid(surfaces, kind, (column, row)) || !seen.insert((column, row)) {
                continue;
            }
            let mut pool = vec![(column, row)];
            let mut index = 0;
            while index < pool.len() {
                let (c, r) = pool[index];
                index += 1;
                for next in [(c + 1, r), (c.wrapping_sub(1), r), (c, r + 1), (c, r.wrapping_sub(1))] {
                    if room.contains_tile(next.0, next.1) && liquid(surfaces, kind, next) && seen.insert(next) {
                        pool.push(next);
                    }
                }
            }
            found.push(pool);
        }
    }
    found
}

pub(crate) fn plan(
    grid: &TileGrid,
    room: &DungeonRoom,
    surfaces: &Surfaces,
    kind: Surface,
    rng: &mut Rng,
) -> Waterworks {
    let water = |surfaces: &Surfaces, tile: (usize, usize)| liquid(surfaces, kind, tile);
    let mut works = Waterworks::default();
    let pools = pools(room, surfaces, kind);
    if pools.is_empty() {
        return works;
    }
    // Pipes in wall faces right above the water.
    let mut spots: Vec<(usize, usize)> = (room.origin.1..room.origin.1 + room.rows)
        .flat_map(|row| (room.origin.0..room.origin.0 + room.columns).map(move |column| (column, row)))
        .filter(|&(column, row)| {
            grid.is_wall(column as isize, row as isize) && water(surfaces, (column, row + 1))
        })
        .collect();
    let wanted = 1 + usize::from(rng.below(100) < SECOND_PIPE_PERCENT);
    while works.pipes.len() < wanted.min(MAX_PIPES) && !spots.is_empty() {
        let spot = spots.swap_remove(rng.below(spots.len()));
        spots.retain(|other| other.0.abs_diff(spot.0) > 3 || other.1 != spot.1);
        works.pipes.push(spot);
    }
    // A footbridge over one of the pools, clear of the pipes' splashes.
    let crossings: Vec<Bridge> = crossings(grid, room, surfaces, kind)
        .into_iter()
        .filter(|bridge| {
            bridge
                .tiles()
                .all(|tile| !works.pipes.iter().any(|pipe| (pipe.0, pipe.1 + 1) == tile))
        })
        .collect();
    if !crossings.is_empty() && rng.below(100) < BRIDGE_PERCENT {
        works.bridges.push(crossings[rng.below(crossings.len())]);
    }
    let bridged: HashSet<(usize, usize)> =
        works.bridges.iter().flat_map(|bridge| bridge.tiles().collect::<Vec<_>>()).collect();
    // Something afloat on the bigger pools, off the bridge and away from the splashes.
    let lava = kind == Surface::Lava;
    let mut barrel_left = !lava && rng.below(100) < BARREL_PERCENT;
    for pool in pools.iter().filter(|pool| pool.len() >= POOL_FOR_FLOATERS) {
        let mut open: Vec<(usize, usize)> = pool
            .iter()
            .copied()
            .filter(|tile| !bridged.contains(tile))
            .filter(|tile| !works.pipes.iter().any(|pipe| pipe.0.abs_diff(tile.0) <= 1 && tile.1 <= pipe.1 + 2))
            .collect();
        let mut floaters = Vec::new();
        if rng.below(100) < DEBRIS_PERCENT {
            floaters.push(Floater::Debris(pick_debris(lava, rng)));
            if rng.below(100) < SECOND_DEBRIS_PERCENT {
                floaters.push(Floater::Debris(pick_debris(lava, rng)));
            }
        }
        if barrel_left {
            barrel_left = false;
            floaters.push(Floater::Barrel(match rng.below(3) {
                0 => FloatingBarrel::Plain,
                1 => FloatingBarrel::Explosive,
                _ => FloatingBarrel::Poison,
            }));
        }
        for floater in floaters {
            if open.is_empty() {
                break;
            }
            let tile = open.swap_remove(rng.below(open.len()));
            open.retain(|other| other.0.abs_diff(tile.0) + other.1.abs_diff(tile.1) > 2);
            works.floaters.push((floater, tile));
        }
    }
    works
}

fn pick_debris(lava: bool, rng: &mut Rng) -> Debris {
    if lava {
        return Debris::Crust(rng.below(CRUSTS));
    }
    if rng.below(100) < RAT_PERCENT {
        return Debris::RatRaft;
    }
    [Debris::Plank, Debris::Crate, Debris::Bottle, Debris::Boot][rng.below(4)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::{Dungeon, Theme};

    #[test]
    fn sewer_pools_get_pipes_bridges_and_floaters_that_sit_where_they_belong() {
        let (mut pipes, mut bridges, mut floaters) = (0, 0, 0);
        for seed in 0..20 {
            let dungeon =
                Dungeon::generate_themed(&mut Rng::new(seed), Theme::Sewer).expect("a dungeon");
            let water = |tile: (usize, usize)| dungeon.surfaces.at(tile) == Some(Surface::Water);
            for room in &dungeon.rooms {
                let works = &room.waterworks;
                for &(column, row) in &works.pipes {
                    pipes += 1;
                    assert!(dungeon.grid.is_wall(column as isize, row as isize));
                    assert!(water((column, row + 1)));
                }
                for bridge in &works.bridges {
                    bridges += 1;
                    assert!(bridge.tiles().all(water));
                    let point = |(c, r): (usize, usize)| {
                        glam::Vec2::new(c as f32 + 0.5, r as f32 + 0.5) * crate::tilemap::TILE_SIZE
                    };
                    assert!(bridge.tiles().all(|tile| dungeon.surfaces.at_point(point(tile)).is_none()));
                }
                for &(_, tile) in &works.floaters {
                    floaters += 1;
                    assert!(water(tile));
                }
            }
        }
        assert!(pipes > 5 && bridges > 5 && floaters > 10, "{pipes} {bridges} {floaters}");
    }

    #[test]
    fn lava_gets_cracks_bridges_and_crusts_but_no_barrels() {
        let (mut cracks, mut bridges, mut crusts) = (0, 0, 0);
        for seed in 0..30 {
            let dungeon =
                Dungeon::generate_themed(&mut Rng::new(seed), Theme::Lava).expect("a dungeon");
            let lava = |tile: (usize, usize)| dungeon.surfaces.at(tile) == Some(Surface::Lava);
            for room in &dungeon.rooms {
                let works = &room.waterworks;
                cracks += works.pipes.len();
                assert!(works.pipes.iter().all(|&(c, r)| lava((c, r + 1))));
                bridges += works.bridges.len();
                assert!(works.bridges.iter().all(|bridge| bridge.tiles().all(lava)));
                for &(floater, tile) in &works.floaters {
                    crusts += 1;
                    assert!(matches!(floater, Floater::Debris(Debris::Crust(_))));
                    assert!(lava(tile));
                }
            }
        }
        assert!(cracks > 3 && bridges > 3 && crusts > 3, "{cracks} {bridges} {crusts}");
    }

    #[test]
    fn other_floors_have_no_waterworks() {
        for theme in [Theme::Stone, Theme::Moss, Theme::Ice] {
            let dungeon = Dungeon::generate_themed(&mut Rng::new(3), theme).expect("a dungeon");
            assert!(dungeon.rooms.iter().all(|room| room.waterworks == Waterworks::default()));
        }
    }
}
