//! How enemies find their way to the player: a flow field over a window round the player, one
//! for each size of enemy chasing, so a big one is only ever sent through gaps it fits through;
//! and the way through it smoothed, each enemy heading straight for the furthest point down its
//! way that it can walk to in a straight line at its own width, or straight for the player when
//! nothing stands between them.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use glam::Vec2;

use crate::navigation::{CELL, Navigation};
use crate::physics::Physics;

/// The window reaches this many cells each way from the player: over the biggest room.
const RADIUS: i32 = 72;
const SIDE: usize = (RADIUS * 2 + 1) as usize;
const STRAIGHT_COST: u32 = 10;
const DIAGONAL_COST: u32 = 14;
const UNREACHED: u32 = u32::MAX;
const MIN_REBUILD_SECONDS: f32 = 0.1;
const MAX_AGE_SECONDS: f32 = 0.5;
/// An enemy is planned for as this much narrower than it is, each way, so one that only just
/// fits a corridor is still sent down it; its rounded corners slide past the rest.
const SLACK: f32 = 8.0;
/// Sizes are rounded up to this, so enemies of nearly the same size share a field.
const SIZE_STEP: f32 = 16.0;
/// How far down its way an enemy looks for a point it can walk straight to, in cells.
const LOOKAHEAD: usize = 40;
/// Points along a straight walk are checked this far apart.
const SWEEP_STEP: f32 = 8.0;
/// How far round its own cell an enemy pushed off its way looks for the way again, in cells.
const RECOVER: i32 = 4;

const NEIGHBOURS: [(i32, i32, u32); 8] = [
    (1, 0, STRAIGHT_COST),
    (-1, 0, STRAIGHT_COST),
    (0, 1, STRAIGHT_COST),
    (0, -1, STRAIGHT_COST),
    (1, 1, DIAGONAL_COST),
    (-1, -1, DIAGONAL_COST),
    (-1, 1, DIAGONAL_COST),
    (1, -1, DIAGONAL_COST),
];

type Cell = (i32, i32);

/// The size an enemy of `size` is planned as.
pub(crate) fn size_class(size: Vec2) -> Vec2 {
    let planned = (size - Vec2::splat(SLACK * 2.0)).max(Vec2::splat(SIZE_STEP));
    (planned / SIZE_STEP).ceil() * SIZE_STEP
}

/// The way through the window for enemies of one size: where they fit, and how far each cell is
/// from the player.
#[derive(Debug, Clone)]
struct Layer {
    class: Vec2,
    open: Vec<bool>,
    cost: Vec<u32>,
}

#[derive(Debug, Default)]
pub(crate) struct FlowField {
    origin: Cell,
    center: Option<Cell>,
    layers: Vec<Layer>,
    age: f32,
}

fn cell_of(point: Vec2) -> Cell {
    (
        (point.x / CELL).floor() as i32,
        (point.y / CELL).floor() as i32,
    )
}

fn middle_of(cell: Cell) -> Vec2 {
    (Vec2::new(cell.0 as f32, cell.1 as f32) + Vec2::splat(0.5)) * CELL
}

impl FlowField {
    /// Keeps the fields round the player fresh for every size in `classes`: rebuilt when the
    /// player moves to another cell (at most every tenth of a second), when they get old, or
    /// when a size turns up that has no field yet.
    pub(crate) fn update(
        &mut self,
        target: Vec2,
        classes: &[Vec2],
        navigation: &Navigation,
        physics: &Physics,
        dt: f32,
    ) {
        self.age += dt;
        let cell = cell_of(target);
        let moved = self.center != Some(cell);
        let missing = classes
            .iter()
            .any(|class| !self.layers.iter().any(|layer| layer.class == *class));
        let due = self.center.is_none()
            || missing
            || if moved {
                self.age >= MIN_REBUILD_SECONDS
            } else {
                self.age >= MAX_AGE_SECONDS
            };
        if !due {
            return;
        }
        let biggest = classes.iter().copied().fold(Vec2::ZERO, Vec2::max);
        let half = SIDE as f32 * CELL / 2.0;
        let solids = physics.solids_within(
            target - Vec2::splat(half) - biggest,
            target + Vec2::splat(half) + biggest,
        );
        self.build(target, classes, navigation, &solids);
    }

    pub(crate) fn build(
        &mut self,
        target: Vec2,
        classes: &[Vec2],
        navigation: &Navigation,
        solids: &[(Vec2, Vec2)],
    ) {
        let center = cell_of(target);
        self.origin = (center.0 - RADIUS, center.1 - RADIUS);
        self.center = Some(center);
        self.age = 0.0;
        let mut classes = classes.to_vec();
        classes.sort_by(|a, b| (a.x, a.y).partial_cmp(&(b.x, b.y)).unwrap_or(std::cmp::Ordering::Equal));
        classes.dedup();
        self.layers = classes
            .into_iter()
            .map(|class| {
                let mut layer = Layer {
                    class,
                    open: vec![false; SIDE * SIDE],
                    cost: Vec::new(),
                };
                for row in 0..SIDE {
                    for column in 0..SIDE {
                        let at = middle_of((self.origin.0 + column as i32, self.origin.1 + row as i32));
                        layer.open[row * SIDE + column] = !navigation.is_blocked(at, class)
                            && !solids.iter().any(|(solid, size)| {
                                ((at - *solid).abs() * 2.0).cmplt(class + *size).all()
                            });
                    }
                }
                if let Some(index) = self.index(center) {
                    layer.open[index] = true;
                }
                layer.cost = self.flood(center, &layer.open);
                layer
            })
            .collect();
    }

    fn flood(&self, center: Cell, open: &[bool]) -> Vec<u32> {
        let mut cost = vec![UNREACHED; SIDE * SIDE];
        let Some(start) = self.index(center) else {
            return cost;
        };
        let is_open = |cell: Cell| self.index(cell).is_some_and(|index| open[index]);
        cost[start] = 0;
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0u32, center)));
        while let Some(Reverse((spent, cell))) = heap.pop() {
            let Some(here) = self.index(cell) else {
                continue;
            };
            if cost[here] < spent {
                continue;
            }
            for (dx, dy, step) in NEIGHBOURS {
                let next = (cell.0 + dx, cell.1 + dy);
                let Some(there) = self.index(next) else {
                    continue;
                };
                if !open[there] {
                    continue;
                }
                if dx != 0 && dy != 0 && (!is_open((cell.0 + dx, cell.1)) || !is_open((cell.0, cell.1 + dy))) {
                    continue;
                }
                let total = spent + step;
                if total < cost[there] {
                    cost[there] = total;
                    heap.push(Reverse((total, next)));
                }
            }
        }
        cost
    }

    fn index(&self, cell: Cell) -> Option<usize> {
        let column = cell.0 - self.origin.0;
        let row = cell.1 - self.origin.1;
        ((0..SIDE as i32).contains(&column) && (0..SIDE as i32).contains(&row))
            .then(|| row as usize * SIDE + column as usize)
    }

    fn layer(&self, size: Vec2) -> Option<&Layer> {
        let class = size_class(size);
        self.layers.iter().find(|layer| layer.class == class)
    }

    /// The field an enemy of `size` at `from` follows: its own, or, where its own leads nowhere
    /// from there (no gap in the walls as wide as it is), the biggest smaller one that does, so
    /// it still heads for the player and squeezes what it can.
    fn layer_from(&self, from: Vec2, size: Vec2) -> Option<&Layer> {
        let own = self.layer(size)?;
        let cell = cell_of(from);
        let leads = |layer: &Layer| {
            self.cost_in(layer, cell) != UNREACHED
                || (-RECOVER..=RECOVER).any(|dy| {
                    (-RECOVER..=RECOVER).any(|dx| self.cost_in(layer, (cell.0 + dx, cell.1 + dy)) != UNREACHED)
                })
        };
        if leads(own) {
            return Some(own);
        }
        self.layers
            .iter()
            .filter(|layer| layer.class.cmple(own.class).all() && layer.class != own.class)
            .filter(|layer| leads(layer))
            .max_by(|a, b| (a.class.x * a.class.y).total_cmp(&(b.class.x * b.class.y)))
            .or(Some(own))
    }

    fn open_in(&self, layer: &Layer, cell: Cell) -> bool {
        self.index(cell).is_some_and(|index| layer.open[index])
    }

    fn cost_in(&self, layer: &Layer, cell: Cell) -> u32 {
        self.index(cell).map_or(UNREACHED, |index| layer.cost[index])
    }

    /// Whether an enemy can walk in a straight line from one point to another without its
    /// planned body meeting a wall or anything solid.
    fn sweep_is_clear(&self, layer: &Layer, from: Vec2, to: Vec2) -> bool {
        let steps = ((to - from).length() / SWEEP_STEP).ceil().max(1.0) as usize;
        (0..=steps).all(|step| self.open_in(layer, cell_of(from.lerp(to, step as f32 / steps as f32))))
    }

    /// How far, up to `reach`, an enemy of `size` could walk from `from` in direction
    /// `direction` before running into something, as a share of `reach`; one where the window
    /// has no field for its size.
    pub(crate) fn free_along(&self, from: Vec2, direction: Vec2, reach: f32, size: Vec2) -> f32 {
        let Some(layer) = self.layer(size) else {
            return 1.0;
        };
        let steps = (reach / SWEEP_STEP).ceil().max(1.0) as usize;
        for step in 1..=steps {
            let at = from + direction * (reach * step as f32 / steps as f32);
            let cell = cell_of(at);
            if self.index(cell).is_some() && !self.open_in(layer, cell) {
                return (step - 1) as f32 / steps as f32;
            }
        }
        1.0
    }

    fn downhill(&self, layer: &Layer, cell: Cell) -> Option<Cell> {
        let own = self.cost_in(layer, cell);
        NEIGHBOURS
            .iter()
            .filter_map(|(dx, dy, _)| {
                let next = (cell.0 + dx, cell.1 + dy);
                let cost = self.cost_in(layer, next);
                let cuts_corner = *dx != 0
                    && *dy != 0
                    && (!self.open_in(layer, (cell.0 + dx, cell.1))
                        || !self.open_in(layer, (cell.0, cell.1 + dy)));
                (cost < own && !cuts_corner).then_some((cost, next))
            })
            .min_by_key(|(cost, _)| *cost)
            .map(|(_, next)| next)
    }

    /// The nearest cell round `cell` that is on the way to the player, for an enemy shoved off
    /// it into a spot it doesn't fit.
    fn nearest_on_the_way(&self, layer: &Layer, cell: Cell) -> Option<Cell> {
        (1..=RECOVER).find_map(|reach| {
            (-reach..=reach)
                .flat_map(|dy| (-reach..=reach).map(move |dx| (cell.0 + dx, cell.1 + dy)))
                .filter(|&next| self.open_in(layer, next) && self.cost_in(layer, next) != UNREACHED)
                .min_by_key(|&next| self.cost_in(layer, next))
        })
    }

    /// Whether the field leads an enemy of `size` from `from` to the player.
    #[cfg(test)]
    pub(crate) fn leads_from(&self, from: Vec2, size: Vec2) -> bool {
        self.layer(size)
            .is_some_and(|layer| self.cost_in(layer, cell_of(from)) != UNREACHED)
    }

    /// Which way an enemy of `size` at `from` should head to reach `target`.
    pub(crate) fn heading(&self, from: Vec2, target: Vec2, size: Vec2) -> Vec2 {
        let straight = (target - from).normalize_or_zero();
        let Some(layer) = self.layer_from(from, size) else {
            return straight;
        };
        let start = cell_of(from);
        if self.index(start).is_none() || self.sweep_is_clear(layer, from, target) {
            return straight;
        }
        let mut cell = start;
        if !self.open_in(layer, cell) || self.cost_in(layer, cell) == UNREACHED {
            return match self.nearest_on_the_way(layer, cell) {
                Some(next) => (middle_of(next) - from).normalize_or(straight),
                None => straight,
            };
        }
        // Down the way, keeping the furthest point it can still walk straight to.
        let mut goal = None;
        for _ in 0..LOOKAHEAD {
            let Some(next) = self.downhill(layer, cell) else {
                break;
            };
            cell = next;
            let point = middle_of(cell);
            if self.sweep_is_clear(layer, from, point) {
                goal = Some(point);
            } else if goal.is_some() {
                break;
            }
        }
        let goal = goal.unwrap_or_else(|| middle_of(self.downhill(layer, start).unwrap_or(start)));
        (goal - from).normalize_or(straight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL: Vec2 = Vec2::new(40.0, 40.0);

    fn field(wall: &[(usize, usize)], solids: &[(Vec2, Vec2)], target: Vec2, sizes: &[Vec2]) -> FlowField {
        let mut field = FlowField::default();
        let classes: Vec<Vec2> = sizes.iter().map(|&size| size_class(size)).collect();
        field.build(target, &classes, &Navigation::new(wall), solids);
        field
    }

    #[test]
    fn without_a_field_the_heading_is_straight_at_the_target() {
        let field = FlowField::default();
        let heading = field.heading(Vec2::new(100.0, 100.0), Vec2::new(400.0, 100.0), SMALL);
        assert_eq!(heading, Vec2::X);
    }

    #[test]
    fn in_the_open_the_heading_is_straight_at_the_target() {
        let target = Vec2::new(600.0, 400.0);
        let field = field(&[], &[], target, &[SMALL]);
        let heading = field.heading(Vec2::new(200.0, 400.0), target, SMALL);
        assert!((heading - Vec2::X).length() < 0.001, "{heading:?}");
    }

    #[test]
    fn a_wall_between_sends_the_heading_around_its_end() {
        let wall: Vec<(usize, usize)> = (0..5).map(|row| (4, row)).collect();
        let target = Vec2::new(600.0, 160.0);
        let field = field(&wall, &[], target, &[SMALL]);

        let heading = field.heading(Vec2::new(100.0, 160.0), target, SMALL);

        assert!(heading.y > 0.3, "{heading:?}");
    }

    fn walk(field: &FlowField, navigation: &Navigation, mut at: Vec2, target: Vec2, size: Vec2) -> Vec2 {
        for _ in 0..600 {
            if at.distance(target) < 40.0 {
                break;
            }
            at += field.heading(at, target, size) * 6.0;
            assert!(!navigation.is_blocked(at, size - Vec2::splat(SLACK * 2.0 + SIZE_STEP)), "{at:?}");
        }
        at
    }

    #[test]
    fn the_flow_leads_all_the_way_round_a_wall() {
        let wall: Vec<(usize, usize)> = (0..5).map(|row| (4, row)).collect();
        let navigation = Navigation::new(&wall);
        let target = Vec2::new(600.0, 160.0);
        let field = field(&wall, &[], target, &[SMALL]);
        let at = walk(&field, &navigation, Vec2::new(100.0, 160.0), target, SMALL);
        assert!(at.distance(target) < 40.0, "{at:?}");
    }

    #[test]
    fn a_big_enemy_goes_round_by_the_wide_gap_not_the_narrow_one() {
        // A wall across with a gap a tile wide near the top and one three tiles wide lower down.
        let mut wall: Vec<(usize, usize)> = Vec::new();
        for row in 0..14 {
            if row != 2 && !(8..=10).contains(&row) {
                wall.push((6, row));
            }
        }
        let navigation = Navigation::new(&wall);
        let target = Vec2::new(12.0 * 64.0, 2.5 * 64.0);
        let from = Vec2::new(2.0 * 64.0, 2.5 * 64.0);
        let small = Vec2::new(40.0, 40.0);
        let big = Vec2::new(150.0, 150.0);
        let field = field(&wall, &[], target, &[small, big]);

        // The small one goes straight through the narrow gap; the big one heads down for the
        // wide one and gets there.
        assert!(field.heading(from, target, small).x > 0.9);
        assert!(field.heading(from, target, big).y > 0.3);
        let at = walk(&field, &navigation, from, target, big);
        assert!(at.distance(target) < 40.0, "{at:?}");
    }

    #[test]
    fn the_heading_cuts_straight_across_the_open_rather_than_zigzagging() {
        // An L: round a corner and then a long open run, the heading points at the corner's
        // far side rather than at the next cell.
        let wall: Vec<(usize, usize)> = (0..6).map(|row| (4, row)).collect();
        let target = Vec2::new(9.0 * 64.0, 1.0 * 64.0);
        let field = field(&wall, &[], target, &[SMALL]);
        let from = Vec2::new(1.0 * 64.0, 1.0 * 64.0);
        let heading = field.heading(from, target, SMALL);
        // Heads for the wall's foot well below, in one straight line.
        assert!(heading.y > 0.8, "{heading:?}");
    }

    #[test]
    fn a_solid_object_in_the_way_is_walked_around() {
        let target = Vec2::new(600.0, 300.0);
        let crate_at = (Vec2::new(400.0, 300.0), Vec2::new(64.0, 240.0));
        let field = field(&[], &[crate_at], target, &[SMALL]);

        let heading = field.heading(Vec2::new(250.0, 300.0), target, SMALL);

        assert!(heading.y.abs() > 0.3, "{heading:?}");
    }

    #[test]
    fn an_unreachable_target_falls_back_to_the_straight_line() {
        let mut cells = Vec::new();
        for column in 5..=9 {
            for row in 5..=9 {
                if column == 5 || column == 9 || row == 5 || row == 9 {
                    cells.push((column, row));
                }
            }
        }
        let target = Vec2::new(7.5 * 64.0, 7.5 * 64.0);
        let field = field(&cells, &[], target, &[SMALL]);
        let from = Vec2::new(100.0, 480.0);

        let heading = field.heading(from, target, SMALL);

        assert!((heading - (target - from).normalize()).length() < 0.001);
    }

    #[test]
    fn free_along_tells_how_far_one_can_walk_before_a_wall() {
        let wall: Vec<(usize, usize)> = (0..5).map(|row| (4, row)).collect();
        let field = field(&wall, &[], Vec2::new(100.0, 160.0), &[SMALL]);
        let from = Vec2::new(100.0, 160.0);
        assert!(field.free_along(from, Vec2::X, 300.0, SMALL) < 0.6);
        assert_eq!(field.free_along(from, -Vec2::Y, 100.0, SMALL), 1.0);
    }

    #[test]
    fn the_field_is_rebuilt_when_the_target_changes_cell_gets_old_or_a_new_size_turns_up() {
        let navigation = Navigation::new(&[]);
        let physics = Physics::new();
        let mut field = FlowField::default();
        let classes = [size_class(SMALL)];
        field.update(Vec2::new(300.0, 300.0), &classes, &navigation, &physics, 0.016);
        assert_eq!(field.age, 0.0);

        field.update(Vec2::new(400.0, 300.0), &classes, &navigation, &physics, 0.016);
        assert!(field.age > 0.0);

        field.update(Vec2::new(400.0, 300.0), &classes, &navigation, &physics, 0.1);
        assert_eq!(field.age, 0.0);
        assert_eq!(field.center, Some(cell_of(Vec2::new(400.0, 300.0))));

        field.update(Vec2::new(401.0, 300.0), &classes, &navigation, &physics, 0.3);
        assert!(field.age > 0.0);
        field.update(Vec2::new(401.0, 300.0), &classes, &navigation, &physics, 0.3);
        assert_eq!(field.age, 0.0);

        let more = [size_class(SMALL), size_class(Vec2::splat(200.0))];
        field.update(Vec2::new(401.0, 300.0), &more, &navigation, &physics, 0.01);
        assert_eq!(field.age, 0.0);
        assert_eq!(field.layers.len(), 2);
    }
}
