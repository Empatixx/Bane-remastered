use std::collections::VecDeque;

use super::template::RoomKind;
use crate::rng::Rng;

const DENSITY: f32 = 0.15;
const ATTEMPTS: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub(crate) const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    pub(crate) fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub(crate) fn delta(self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PlacedRoom {
    pub(crate) kind: RoomKind,
    pub(crate) cell: (i32, i32),
    pub(crate) links: Vec<(Direction, usize)>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct LayoutSpec {
    pub(crate) normals: usize,
    pub(crate) shops: usize,
    pub(crate) treasures: usize,
    pub(crate) sacrifices: usize,
    pub(crate) ambushes: usize,
}

impl LayoutSpec {
    pub(crate) fn standard(rng: &mut Rng) -> Self {
        Self {
            normals: 5,
            shops: 1,
            treasures: 1,
            sacrifices: rng.below(2),
            ambushes: 1 + rng.below(2),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Layout {
    pub(crate) rooms: Vec<PlacedRoom>,
}

impl Layout {
    pub(crate) fn generate(spec: &LayoutSpec, rng: &mut Rng) -> Option<Self> {
        (0..ATTEMPTS).find_map(|_| Self::attempt(spec, rng))
    }

    fn attempt(spec: &LayoutSpec, rng: &mut Rng) -> Option<Self> {
        let mut layout = Self {
            rooms: vec![PlacedRoom {
                kind: RoomKind::Start,
                cell: (0, 0),
                links: Vec::new(),
            }],
        };
        for _ in 0..spec.normals {
            let parent = layout.pick_parent(&[RoomKind::Start, RoomKind::Normal], rng)?;
            layout.attach_random(parent, RoomKind::Normal, rng)?;
        }
        let specials = std::iter::repeat_n(RoomKind::Shop, spec.shops)
            .chain(std::iter::repeat_n(RoomKind::Treasure, spec.treasures))
            .chain(std::iter::repeat_n(RoomKind::Sacrifice, spec.sacrifices))
            .chain(std::iter::repeat_n(RoomKind::Ambush, spec.ambushes));
        for kind in specials {
            let candidates = layout.attachable(&[RoomKind::Normal]);
            let parent = *rng.pick(&candidates)?;
            layout.attach_random(parent, kind, rng)?;
        }
        let boss_parent = layout.farthest_attachable(rng)?;
        layout.attach_random(boss_parent, RoomKind::Boss, rng)?;
        Some(layout)
    }

    fn occupied(&self, cell: (i32, i32)) -> bool {
        self.rooms.iter().any(|room| room.cell == cell)
    }

    fn neighbours_of(&self, index: usize) -> usize {
        let (x, y) = self.rooms[index].cell;
        Direction::ALL
            .iter()
            .filter(|direction| {
                let (dx, dy) = direction.delta();
                self.occupied((x + dx, y + dy))
            })
            .count()
    }

    fn free_directions(&self, index: usize) -> Vec<Direction> {
        let (x, y) = self.rooms[index].cell;
        Direction::ALL
            .into_iter()
            .filter(|direction| {
                let (dx, dy) = direction.delta();
                !self.occupied((x + dx, y + dy))
            })
            .collect()
    }

    fn attachable(&self, kinds: &[RoomKind]) -> Vec<usize> {
        (0..self.rooms.len())
            .filter(|&index| kinds.contains(&self.rooms[index].kind))
            .filter(|&index| !self.free_directions(index).is_empty())
            .collect()
    }

    fn pick_parent(&self, kinds: &[RoomKind], rng: &mut Rng) -> Option<usize> {
        self.attachable(kinds)
            .into_iter()
            .map(|index| {
                let score = rng.float() + DENSITY * self.neighbours_of(index) as f32;
                (index, score)
            })
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(index, _)| index)
    }

    fn farthest_attachable(&self, rng: &mut Rng) -> Option<usize> {
        let distances = self.distances_from(0);
        let candidates = self.attachable(&[RoomKind::Normal]);
        let best = candidates.iter().map(|&index| distances[index]).max()?;
        let farthest: Vec<usize> = candidates
            .into_iter()
            .filter(|&index| distances[index] == best)
            .collect();
        rng.pick(&farthest).copied()
    }

    fn attach_random(&mut self, parent: usize, kind: RoomKind, rng: &mut Rng) -> Option<usize> {
        let directions = self.free_directions(parent);
        let direction = *rng.pick(&directions)?;
        let (x, y) = self.rooms[parent].cell;
        let (dx, dy) = direction.delta();
        let child = self.rooms.len();
        self.rooms.push(PlacedRoom {
            kind,
            cell: (x + dx, y + dy),
            links: vec![(direction.opposite(), parent)],
        });
        self.rooms[parent].links.push((direction, child));
        Some(child)
    }

    pub(crate) fn distances_from(&self, start: usize) -> Vec<usize> {
        let mut distances = vec![usize::MAX; self.rooms.len()];
        distances[start] = 0;
        let mut queue = VecDeque::from([start]);
        while let Some(current) = queue.pop_front() {
            for &(_, next) in &self.rooms[current].links {
                if distances[next] == usize::MAX {
                    distances[next] = distances[current] + 1;
                    queue.push_back(next);
                }
            }
        }
        distances
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn layouts() -> Vec<Layout> {
        (0..200)
            .filter_map(|seed| {
                let mut rng = Rng::new(seed);
                let spec = LayoutSpec::standard(&mut rng);
                Layout::generate(&spec, &mut rng)
            })
            .collect()
    }

    fn count(layout: &Layout, kind: RoomKind) -> usize {
        layout.rooms.iter().filter(|room| room.kind == kind).count()
    }

    #[test]
    fn every_seed_produces_a_layout_with_the_requested_rooms() {
        let layouts = layouts();
        assert_eq!(layouts.len(), 200);
        for layout in &layouts {
            assert_eq!(count(layout, RoomKind::Start), 1);
            assert_eq!(count(layout, RoomKind::Normal), 5);
            assert_eq!(count(layout, RoomKind::Boss), 1);
            assert_eq!(count(layout, RoomKind::Shop), 1);
            assert_eq!(count(layout, RoomKind::Treasure), 1);
        }
    }

    #[test]
    fn rooms_never_share_a_cell_and_every_room_is_reachable() {
        for layout in layouts() {
            let cells: HashSet<_> = layout.rooms.iter().map(|room| room.cell).collect();
            assert_eq!(cells.len(), layout.rooms.len());
            assert!(
                layout
                    .distances_from(0)
                    .iter()
                    .all(|distance| *distance != usize::MAX)
            );
        }
    }

    #[test]
    fn links_are_symmetric_and_point_to_adjacent_cells() {
        for layout in layouts() {
            for (index, room) in layout.rooms.iter().enumerate() {
                for &(direction, other) in &room.links {
                    let (dx, dy) = direction.delta();
                    assert_eq!(
                        layout.rooms[other].cell,
                        (room.cell.0 + dx, room.cell.1 + dy)
                    );
                    assert!(
                        layout.rooms[other]
                            .links
                            .contains(&(direction.opposite(), index))
                    );
                }
            }
        }
    }

    #[test]
    fn special_rooms_are_dead_ends_attached_to_normal_rooms() {
        for layout in layouts() {
            for room in &layout.rooms {
                if matches!(
                    room.kind,
                    RoomKind::Shop | RoomKind::Treasure | RoomKind::Sacrifice | RoomKind::Boss
                ) {
                    assert_eq!(room.links.len(), 1);
                    assert_eq!(layout.rooms[room.links[0].1].kind, RoomKind::Normal);
                }
            }
        }
    }

    #[test]
    fn the_boss_is_as_far_from_the_start_as_any_room_that_could_hold_it() {
        for layout in layouts() {
            let distances = layout.distances_from(0);
            let boss = layout
                .rooms
                .iter()
                .position(|room| room.kind == RoomKind::Boss)
                .expect("a boss room exists");
            let parent_distance = distances[layout.rooms[boss].links[0].1];
            let farthest_normal = layout
                .rooms
                .iter()
                .enumerate()
                .filter(|(_, room)| room.kind == RoomKind::Normal)
                .map(|(index, _)| distances[index])
                .max()
                .unwrap_or(0);
            assert!(parent_distance + 2 >= farthest_normal);
        }
    }

    #[test]
    fn the_same_seed_gives_the_same_layout() {
        let build = |seed| {
            let mut rng = Rng::new(seed);
            let spec = LayoutSpec::standard(&mut rng);
            Layout::generate(&spec, &mut rng).map(|layout| {
                layout
                    .rooms
                    .iter()
                    .map(|room| room.cell)
                    .collect::<Vec<_>>()
            })
        };
        assert_eq!(build(99), build(99));
    }
}
