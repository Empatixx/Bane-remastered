use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use glam::Vec2;

use crate::geometry;
use crate::tilemap::TILE_SIZE;

pub(crate) const CELL: f32 = 16.0;
const STRAIGHT_COST: i32 = 10;
const DIAGONAL_COST: i32 = 14;
const MAX_VISITED: usize = 25_000;
const EDGE: f32 = 0.01;

#[derive(Debug, Default)]
pub(crate) struct Navigation {
    solid: HashSet<(usize, usize)>,
}

impl Navigation {
    pub(crate) fn new(solid_cells: &[(usize, usize)]) -> Self {
        Self {
            solid: solid_cells.iter().copied().collect(),
        }
    }

    pub(crate) fn is_blocked(&self, center: Vec2, size: Vec2) -> bool {
        let min = center - size / 2.0;
        let max = center + size / 2.0;
        if min.x < 0.0 || min.y < 0.0 {
            return true;
        }
        let first_column = (min.x / TILE_SIZE) as usize;
        let first_row = (min.y / TILE_SIZE) as usize;
        let last_column = ((max.x - EDGE) / TILE_SIZE) as usize;
        let last_row = ((max.y - EDGE) / TILE_SIZE) as usize;
        (first_row..=last_row).any(|row| {
            (first_column..=last_column).any(|column| self.solid.contains(&(column, row)))
        })
    }

    pub(crate) fn find_path(
        &self,
        start: Vec2,
        size: Vec2,
        target: Vec2,
        target_size: Vec2,
    ) -> Vec<Vec2> {
        astar(start, size, target, target_size, |center| {
            self.is_blocked(center, size)
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct Open {
    f: i32,
    g: i32,
    node: Node,
}

impl Ord for Open {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f
            .cmp(&self.f)
            .then_with(|| other.g.cmp(&self.g))
            .then_with(|| (other.node.x, other.node.y).cmp(&(self.node.x, self.node.y)))
    }
}

impl PartialOrd for Open {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const STEPS: [(i32, i32, i32); 8] = [
    (1, 0, STRAIGHT_COST),
    (-1, 0, STRAIGHT_COST),
    (0, 1, STRAIGHT_COST),
    (0, -1, STRAIGHT_COST),
    (1, 1, DIAGONAL_COST),
    (-1, -1, DIAGONAL_COST),
    (-1, 1, DIAGONAL_COST),
    (1, -1, DIAGONAL_COST),
];

pub(crate) fn astar(
    start: Vec2,
    size: Vec2,
    target: Vec2,
    target_size: Vec2,
    blocked: impl Fn(Vec2) -> bool,
) -> Vec<Vec2> {
    let at = |node: Node| start + Vec2::new(node.x as f32, node.y as f32) * CELL;
    let heuristic = |node: Node| {
        let point = at(node);
        ((point.x - target.x).abs() + (point.y - target.y).abs()) as i32
    };
    let begin = Node { x: 0, y: 0 };
    let mut open = BinaryHeap::new();
    let mut parents: HashMap<Node, Option<Node>> = HashMap::new();
    let mut costs: HashMap<Node, i32> = HashMap::new();
    open.push(Open {
        f: heuristic(begin),
        g: 0,
        node: begin,
    });
    parents.insert(begin, None);
    costs.insert(begin, 0);

    while let Some(Open { g, node, .. }) = open.pop() {
        if costs.get(&node).is_some_and(|best| *best < g) {
            continue;
        }
        if geometry::overlaps(at(node), size, target, target_size) {
            let mut path = vec![at(node)];
            let mut current = node;
            while let Some(Some(parent)) = parents.get(&current) {
                path.push(at(*parent));
                current = *parent;
            }
            path.reverse();
            return path;
        }
        if parents.len() > MAX_VISITED {
            return Vec::new();
        }
        for (dx, dy, cost) in STEPS {
            let next = Node {
                x: node.x + dx,
                y: node.y + dy,
            };
            let next_cost = g + cost;
            if costs.get(&next).is_some_and(|known| *known <= next_cost) {
                continue;
            }
            if blocked(at(next)) {
                continue;
            }
            costs.insert(next, next_cost);
            parents.insert(next, Some(node));
            open.push(Open {
                f: next_cost + heuristic(next),
                g: next_cost,
                node: next,
            });
        }
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE: Vec2 = Vec2::new(32.0, 32.0);

    fn wall(cells: &[(usize, usize)]) -> Navigation {
        Navigation::new(cells)
    }

    #[test]
    fn a_free_field_gives_a_short_path_ending_at_the_target() {
        let navigation = wall(&[]);
        let path =
            navigation.find_path(Vec2::new(200.0, 200.0), SIZE, Vec2::new(400.0, 200.0), SIZE);
        assert!(path.len() > 5);
        let last = path[path.len() - 1];
        assert!(geometry::overlaps(
            last,
            SIZE,
            Vec2::new(400.0, 200.0),
            SIZE
        ));
    }

    #[test]
    fn the_path_goes_around_a_wall() {
        let blocking: Vec<(usize, usize)> = (0..6).map(|row| (4, row)).collect();
        let navigation = wall(&blocking);
        let start = Vec2::new(200.0, 160.0);
        let target = Vec2::new(400.0, 160.0);
        let path = navigation.find_path(start, SIZE, target, SIZE);
        assert!(!path.is_empty());
        assert!(path
            .iter()
            .all(|point| !navigation.is_blocked(*point, SIZE)));
        assert!(path.iter().any(|point| point.y > 6.0 * TILE_SIZE - 20.0));
    }

    #[test]
    fn an_enclosed_target_has_no_path() {
        let mut cells = Vec::new();
        for column in 5..=9 {
            for row in 5..=9 {
                if column == 5 || column == 9 || row == 5 || row == 9 {
                    cells.push((column, row));
                }
            }
        }
        let navigation = wall(&cells);
        let path = navigation.find_path(
            Vec2::new(150.0, 150.0),
            SIZE,
            Vec2::new(7.5 * TILE_SIZE, 7.5 * TILE_SIZE),
            SIZE,
        );
        assert!(path.is_empty());
    }

    #[test]
    fn blocking_checks_every_tile_the_body_touches() {
        let navigation = wall(&[(2, 2)]);
        assert!(navigation.is_blocked(Vec2::new(150.0, 150.0), Vec2::new(40.0, 40.0)));
        assert!(!navigation.is_blocked(Vec2::new(100.0, 100.0), Vec2::new(40.0, 40.0)));
        assert!(navigation.is_blocked(Vec2::new(5.0, 100.0), Vec2::new(40.0, 40.0)));
    }
}
