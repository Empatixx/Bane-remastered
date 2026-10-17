//! The stone floor is a prison: now and then a pendulum blade swings across a gap between two
//! blocks, hung from a beam laid over it.

use std::ops::RangeInclusive;

use crate::rng::Rng;
use crate::tilemap::TileGrid;

use super::build::DungeonRoom;
use super::overhead::{self, Overhead, OverheadKind};
use super::template::RoomKind;

/// How often a stone floor has a pendulum at all; it never has more than one.
const PENDULUM_FLOOR_PERCENT: usize = 70;
const PENDULUM_SPAN: RangeInclusive<usize> = 3..=6;

/// On most stone floors, one combat room gets a pendulum over a gap across it between two
/// blocks, clear of what already lies across the room and in a room with room for one more;
/// the beam it hangs from joins the room's overheads, and the room remembers the span.
pub(crate) fn plan_pendulum(grid: &TileGrid, rooms: &mut [DungeonRoom], rng: &mut Rng) {
    if rng.below(100) >= PENDULUM_FLOOR_PERCENT {
        return;
    }
    let mut candidates: Vec<(usize, Overhead)> = Vec::new();
    for (index, room) in rooms.iter().enumerate() {
        if !matches!(room.kind, RoomKind::Normal | RoomKind::Ambush)
            || room.overheads.len() >= overhead::MAX_PER_ROOM
        {
            continue;
        }
        for (upright, first, last) in overhead::spans(grid, room) {
            let span = Overhead {
                kind: OverheadKind::Beams,
                upright,
                first,
                last,
            };
            if !upright
                && PENDULUM_SPAN.contains(&span.length())
                && !room.overheads.iter().any(|other| overhead::too_close(other, &span))
            {
                candidates.push((index, span));
            }
        }
    }
    if let Some(&(index, span)) = rng.pick(&candidates) {
        rooms[index].overheads.push(span);
        rooms[index].pendulum = Some(span);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::{Dungeon, Theme};

    #[test]
    fn a_stone_floor_has_at_most_one_pendulum_under_a_beam_across_a_gap() {
        let mut floors_with_one = 0;
        for seed in 0..20 {
            let dungeon = Dungeon::generate_themed(&mut Rng::new(seed), Theme::Stone).expect("a dungeon");
            let pendulums: Vec<(&DungeonRoom, Overhead)> = dungeon
                .rooms
                .iter()
                .filter_map(|room| room.pendulum.map(|span| (room, span)))
                .collect();
            assert!(pendulums.len() <= 1);
            for (room, span) in pendulums {
                floors_with_one += 1;
                assert!(!span.upright && PENDULUM_SPAN.contains(&span.length()));
                assert!(room.overheads.contains(&span));
                assert!(span.tiles().all(|(c, r)| !dungeon.grid.is_wall(c as isize, r as isize)));
            }
        }
        assert!(floors_with_one > 5, "{floors_with_one}");
        let other = Dungeon::generate_themed(&mut Rng::new(1), Theme::Moss).expect("a dungeon");
        assert!(other.rooms.iter().all(|room| room.pendulum.is_none()));
    }
}
