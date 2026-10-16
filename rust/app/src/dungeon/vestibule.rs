//! A small room halfway along some of the corridors between rooms: the corridor widens into a
//! chamber a few tiles across, with a pot or a barrel in its side bays, now and then something
//! of the floor's biome, a torch on its wall or a cobweb in a corner. The way through stays
//! clear, everything stands off to the sides.

use crate::rng::Rng;
use crate::tilemap::TileGrid;

use super::build::DungeonRoom;
use super::decor::{Decor, DecorKind, Mount};
use super::theme::Theme;

/// How many of the corridors that have room for one get a vestibule.
const PERCENT: usize = 40;
/// Open corridor kept at each end of the chamber, so the doors still show as a corridor.
const NECK: usize = 1;
const MIN_LENGTH: usize = 3;
const MAX_LENGTH: usize = 5;
const TORCH_PERCENT: usize = 60;
const COBWEB_PERCENT: usize = 30;
const SECOND_OBJECT_PERCENT: usize = 50;
const BIOME_PERCENT: usize = 60;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Vestibule {
    /// The two rooms the corridor joins, by index.
    pub(crate) rooms: (usize, usize),
    /// The chamber's floor, corners included.
    pub(crate) min: (usize, usize),
    pub(crate) max: (usize, usize),
    pub(crate) decor: Vec<Decor>,
}

impl Vestibule {
    pub(crate) fn contains(&self, (column, row): (usize, usize)) -> bool {
        (self.min.0..=self.max.0).contains(&column) && (self.min.1..=self.max.1).contains(&row)
    }

    pub(crate) fn center(&self) -> (f32, f32) {
        (
            (self.min.0 + self.max.0 + 1) as f32 / 2.0,
            (self.min.1 + self.max.1 + 1) as f32 / 2.0,
        )
    }
}

/// A corridor's two lanes, each from the door tile of one room to that of the other.
pub(crate) type Lanes = [((usize, usize), (usize, usize)); 2];

/// The corridor's open tiles, given as the two lanes running from door to door, whether it runs
/// across the screen, and the first and last tile along it that lies outside both rooms.
struct Corridor {
    across: bool,
    /// Along the corridor: its first and last tile between the two doors.
    along: (usize, usize),
    /// Across it: the two lanes.
    lanes: (usize, usize),
}

fn corridor(lanes: &Lanes) -> Option<Corridor> {
    let ((a_start, a_end), (b_start, b_end)) = (lanes[0], lanes[1]);
    let across = a_start.1 == a_end.1 && b_start.1 == b_end.1 && a_start.1 != b_start.1;
    let upright = a_start.0 == a_end.0 && b_start.0 == b_end.0 && a_start.0 != b_start.0;
    if across {
        let (low, high) = (a_start.0.min(a_end.0), a_start.0.max(a_end.0));
        Some(Corridor {
            across,
            along: (low + 1, high - 1),
            lanes: (a_start.1.min(b_start.1), a_start.1.max(b_start.1)),
        })
    } else if upright {
        let (low, high) = (a_start.1.min(a_end.1), a_start.1.max(a_end.1));
        Some(Corridor {
            across,
            along: (low + 1, high - 1),
            lanes: (a_start.0.min(b_start.0), a_start.0.max(b_start.0)),
        })
    } else {
        // The doors don't line up, so the corridor bends: leave it as it is.
        None
    }
}

/// Now and then widens the corridor joining two rooms into a vestibule, carving it out of the
/// rock between them, and dresses it.
pub(crate) fn plan(
    grid: &mut TileGrid,
    rooms: &[DungeonRoom],
    joined: (usize, usize),
    lanes: &Lanes,
    theme: Theme,
    rng: &mut Rng,
) -> Option<Vestibule> {
    let corridor = corridor(lanes)?;
    let gap = corridor.along.1 + 1 - corridor.along.0;
    if gap < MIN_LENGTH + 2 * NECK || rng.below(100) >= PERCENT {
        return None;
    }
    let length = (gap - 2 * NECK).min(MAX_LENGTH);
    let start = corridor.along.0 + (gap - length) / 2;
    let widen = 1 + rng.below(2);
    let (side_low, side_high) = (corridor.lanes.0.checked_sub(widen)?, corridor.lanes.1 + widen);
    // Both ways round: (along, across) to (column, row).
    let tile = |along: usize, across: usize| {
        if corridor.across {
            (along, across)
        } else {
            (across, along)
        }
    };
    let (min, max) = (
        tile(start, side_low),
        tile(start + length - 1, side_high),
    );
    // The chamber and the wall round it lie in the rock between the rooms, clear of any other
    // corridor.
    let lane_tile = |(column, row): (usize, usize)| {
        let (along, across) = if corridor.across { (column, row) } else { (row, column) };
        (corridor.along.0..=corridor.along.1).contains(&along)
            && (corridor.lanes.0..=corridor.lanes.1).contains(&across)
    };
    for row in min.1.checked_sub(1)?..=max.1 + 1 {
        for column in min.0.checked_sub(1)?..=max.0 + 1 {
            if rooms.iter().any(|room| room.contains_tile(column, row)) {
                return None;
            }
            if !lane_tile((column, row)) && !grid.is_wall(column as isize, row as isize) {
                return None;
            }
        }
    }
    for row in min.1..=max.1 {
        for column in min.0..=max.0 {
            grid.carve(column, row);
        }
    }
    let mut vestibule = Vestibule {
        rooms: joined,
        min,
        max,
        decor: Vec::new(),
    };
    dress(&mut vestibule, &corridor, theme, rng);
    Some(vestibule)
}

/// One or two objects in the bays either side of the way through, sometimes something of the
/// biome, a torch on the wall and a cobweb in a corner.
fn dress(vestibule: &mut Vestibule, corridor: &Corridor, theme: Theme, rng: &mut Rng) {
    let (min, max) = (vestibule.min, vestibule.max);
    let bay = |(column, row): &(usize, usize)| {
        let across = if corridor.across { *row } else { *column };
        !(corridor.lanes.0..=corridor.lanes.1).contains(&across)
    };
    let mut bays: Vec<(usize, usize)> = (min.1..=max.1)
        .flat_map(|row| (min.0..=max.0).map(move |column| (column, row)))
        .filter(bay)
        .collect();
    let take = |rng: &mut Rng, bays: &mut Vec<(usize, usize)>| {
        if bays.is_empty() {
            return None;
        }
        let tile = bays.swap_remove(rng.below(bays.len()));
        // Nothing else right beside it.
        bays.retain(|other| other.0.abs_diff(tile.0) + other.1.abs_diff(tile.1) > 1);
        Some(tile)
    };
    let objects = 1 + usize::from(rng.below(100) < SECOND_OBJECT_PERCENT);
    for _ in 0..objects {
        let kind = if rng.below(2) == 0 {
            DecorKind::Pot
        } else {
            DecorKind::Barrel
        };
        if let Some(tile) = take(rng, &mut bays) {
            vestibule.decor.push(Decor { kind, tile });
        }
    }
    if rng.below(100) < BIOME_PERCENT {
        let variant = rng.below(2);
        let kind = match theme {
            Theme::Moss => DecorKind::Mushrooms { variant },
            Theme::Spider => DecorKind::Cocoon { variant },
            Theme::Ice => DecorKind::IceCrystals { variant },
            _ => DecorKind::Bones { variant },
        };
        if let Some(tile) = take(rng, &mut bays) {
            vestibule.decor.push(Decor { kind, tile });
        }
    }
    if rng.below(100) < TORCH_PERCENT {
        // On the wall facing the way the corridor runs past: the top wall of a corridor across
        // the screen, a side wall of one running down it.
        let middle = (min.0 + max.0) / 2;
        let (tile, mount) = if corridor.across {
            ((middle, min.1 - 1), Mount::Top)
        } else {
            ((min.0, (min.1 + max.1) / 2), Mount::FacingLeft)
        };
        vestibule.decor.push(Decor {
            kind: DecorKind::Torch { mount, lit: true },
            tile,
        });
    }
    if rng.below(100) < COBWEB_PERCENT {
        // A cobweb's `left` means the wall it clings to is on its right.
        let left_corner = rng.below(2) == 0;
        let tile = if left_corner { (min.0, min.1) } else { (max.0, min.1) };
        if !vestibule.decor.iter().any(|decor| decor.tile == tile) {
            vestibule.decor.push(Decor {
                kind: DecorKind::Cobweb {
                    variant: rng.below(2),
                    left: !left_corner,
                },
                tile,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::Dungeon;

    #[test]
    fn some_corridors_widen_into_a_vestibule_that_stays_open_and_dressed_at_its_sides() {
        let (mut corridors, mut vestibules) = (0, 0);
        for seed in 0..20 {
            let dungeon =
                Dungeon::generate_themed(&mut Rng::new(seed), Theme::Moss).expect("a dungeon");
            corridors += dungeon.rooms.iter().map(|room| room.doors.len()).sum::<usize>() / 2;
            for vestibule in &dungeon.vestibules {
                vestibules += 1;
                for row in vestibule.min.1..=vestibule.max.1 {
                    for column in vestibule.min.0..=vestibule.max.0 {
                        assert!(!dungeon.grid.is_wall(column as isize, row as isize));
                        assert!(dungeon.room_at(column, row).is_none());
                        assert!(dungeon.camera_region(column, row).is_some());
                    }
                }
                assert!(!vestibule.decor.is_empty());
                let (a, b) = vestibule.rooms;
                assert_ne!(a, b);
            }
        }
        let share = vestibules as f32 / corridors as f32;
        assert!((0.2..0.6).contains(&share), "{vestibules} of {corridors}");
    }
}
