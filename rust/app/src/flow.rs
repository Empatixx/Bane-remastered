use std::cmp::Reverse;
use std::collections::BinaryHeap;

use glam::Vec2;

use crate::navigation::{Navigation, CELL};
use crate::physics::Physics;

const RADIUS: i32 = 48;
const SIDE: usize = (RADIUS * 2 + 1) as usize;
const CLEARANCE: Vec2 = Vec2::new(32.0, 32.0);
const STRAIGHT_COST: u32 = 10;
const DIAGONAL_COST: u32 = 14;
const UNREACHED: u32 = u32::MAX;
const LOOKAHEAD: usize = 2;
const MIN_REBUILD_SECONDS: f32 = 0.1;
const MAX_AGE_SECONDS: f32 = 0.5;

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

#[derive(Debug, Default)]
pub(crate) struct FlowField {
    origin: Cell,
    center: Option<Cell>,
    cost: Vec<u32>,
    blocked: Vec<bool>,
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
    pub(crate) fn update(
        &mut self,
        target: Vec2,
        navigation: &Navigation,
        physics: &Physics,
        dt: f32,
    ) {
        self.age += dt;
        let cell = cell_of(target);
        let moved = self.center != Some(cell);
        let due = if self.center.is_none() {
            true
        } else if moved {
            self.age >= MIN_REBUILD_SECONDS
        } else {
            self.age >= MAX_AGE_SECONDS
        };
        if !due {
            return;
        }
        let half = SIDE as f32 * CELL / 2.0;
        let solids = physics.solids_within(
            target - Vec2::splat(half) - CLEARANCE,
            target + Vec2::splat(half) + CLEARANCE,
        );
        self.build(target, navigation, &solids);
    }

    pub(crate) fn build(&mut self, target: Vec2, navigation: &Navigation, solids: &[(Vec2, Vec2)]) {
        let center = cell_of(target);
        self.origin = (center.0 - RADIUS, center.1 - RADIUS);
        self.center = Some(center);
        self.age = 0.0;
        self.blocked.clear();
        self.blocked.resize(SIDE * SIDE, false);
        for row in 0..SIDE {
            for column in 0..SIDE {
                let cell = (self.origin.0 + column as i32, self.origin.1 + row as i32);
                self.blocked[row * SIDE + column] =
                    navigation.is_blocked(middle_of(cell), CLEARANCE);
            }
        }
        for (at, size) in solids {
            let reach = (*size + CLEARANCE) / 2.0;
            let low = cell_of(*at - reach);
            let high = cell_of(*at + reach);
            for y in low.1..=high.1 {
                for x in low.0..=high.0 {
                    if (middle_of((x, y)) - *at).abs().cmple(reach).all() {
                        if let Some(index) = self.index((x, y)) {
                            self.blocked[index] = true;
                        }
                    }
                }
            }
        }
        if let Some(index) = self.index(center) {
            self.blocked[index] = false;
        }
        self.flood(center);
    }

    fn flood(&mut self, center: Cell) {
        self.cost.clear();
        self.cost.resize(SIDE * SIDE, UNREACHED);
        let Some(start) = self.index(center) else {
            return;
        };
        self.cost[start] = 0;
        let mut open = BinaryHeap::new();
        open.push(Reverse((0u32, center)));
        while let Some(Reverse((spent, cell))) = open.pop() {
            let Some(here) = self.index(cell) else {
                continue;
            };
            if self.cost[here] < spent {
                continue;
            }
            for (dx, dy, step) in NEIGHBOURS {
                let next = (cell.0 + dx, cell.1 + dy);
                let Some(there) = self.index(next) else {
                    continue;
                };
                if self.blocked[there] {
                    continue;
                }
                if dx != 0
                    && dy != 0
                    && (self.is_blocked((cell.0 + dx, cell.1))
                        || self.is_blocked((cell.0, cell.1 + dy)))
                {
                    continue;
                }
                let total = spent + step;
                if total < self.cost[there] {
                    self.cost[there] = total;
                    open.push(Reverse((total, next)));
                }
            }
        }
    }

    fn index(&self, cell: Cell) -> Option<usize> {
        let column = cell.0 - self.origin.0;
        let row = cell.1 - self.origin.1;
        ((0..SIDE as i32).contains(&column) && (0..SIDE as i32).contains(&row))
            .then(|| row as usize * SIDE + column as usize)
    }

    fn is_blocked(&self, cell: Cell) -> bool {
        self.index(cell).is_none_or(|index| self.blocked[index])
    }

    fn cost_at(&self, cell: Cell) -> u32 {
        self.index(cell)
            .and_then(|index| self.cost.get(index).copied())
            .unwrap_or(UNREACHED)
    }

    fn line_is_clear(&self, from: Cell, to: Cell) -> bool {
        let (mut x, mut y) = from;
        let dx = (to.0 - from.0).abs();
        let dy = -(to.1 - from.1).abs();
        let step_x = if from.0 < to.0 { 1 } else { -1 };
        let step_y = if from.1 < to.1 { 1 } else { -1 };
        let mut error = dx + dy;
        loop {
            if self.is_blocked((x, y)) {
                return false;
            }
            if (x, y) == to {
                return true;
            }
            let doubled = 2 * error;
            if doubled >= dy {
                error += dy;
                x += step_x;
            }
            if doubled <= dx {
                error += dx;
                y += step_y;
            }
        }
    }

    fn downhill(&self, cell: Cell) -> Option<Cell> {
        let own = self.cost_at(cell);
        NEIGHBOURS
            .iter()
            .filter_map(|(dx, dy, _)| {
                let next = (cell.0 + dx, cell.1 + dy);
                let cost = self.cost_at(next);
                let cuts_corner = *dx != 0
                    && *dy != 0
                    && (self.is_blocked((cell.0 + dx, cell.1))
                        || self.is_blocked((cell.0, cell.1 + dy)));
                (cost < own && !cuts_corner).then_some((cost, next))
            })
            .min_by_key(|(cost, _)| *cost)
            .map(|(_, next)| next)
    }

    pub(crate) fn heading(&self, from: Vec2, target: Vec2) -> Vec2 {
        let straight = (target - from).normalize_or_zero();
        if self.center.is_none() {
            return straight;
        }
        let start = cell_of(from);
        if self.index(start).is_none() || self.line_is_clear(start, cell_of(target)) {
            return straight;
        }
        let mut cell = start;
        let mut moved = false;
        for _ in 0..LOOKAHEAD {
            match self.downhill(cell) {
                Some(next) => {
                    cell = next;
                    moved = true;
                }
                None => break,
            }
        }
        if !moved {
            return straight;
        }
        (middle_of(cell) - from).normalize_or(straight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field(wall: &[(usize, usize)], solids: &[(Vec2, Vec2)], target: Vec2) -> FlowField {
        let mut field = FlowField::default();
        field.build(target, &Navigation::new(wall), solids);
        field
    }

    #[test]
    fn without_a_field_the_heading_is_straight_at_the_target() {
        let field = FlowField::default();
        let heading = field.heading(Vec2::new(100.0, 100.0), Vec2::new(400.0, 100.0));
        assert_eq!(heading, Vec2::X);
    }

    #[test]
    fn in_the_open_the_heading_is_straight_at_the_target() {
        let target = Vec2::new(600.0, 400.0);
        let field = field(&[], &[], target);
        let heading = field.heading(Vec2::new(200.0, 400.0), target);
        assert!((heading - Vec2::X).length() < 0.001, "{heading:?}");
    }

    #[test]
    fn a_wall_between_sends_the_heading_around_its_end() {
        let wall: Vec<(usize, usize)> = (0..5).map(|row| (4, row)).collect();
        let target = Vec2::new(600.0, 160.0);
        let field = field(&wall, &[], target);

        let heading = field.heading(Vec2::new(100.0, 160.0), target);

        assert!(heading.y > 0.3, "{heading:?}");
    }

    #[test]
    fn the_flow_leads_all_the_way_round_a_wall() {
        let wall: Vec<(usize, usize)> = (0..5).map(|row| (4, row)).collect();
        let navigation = Navigation::new(&wall);
        let target = Vec2::new(600.0, 160.0);
        let mut field = FlowField::default();
        field.build(target, &navigation, &[]);

        let mut at = Vec2::new(100.0, 160.0);
        for _ in 0..400 {
            if at.distance(target) < 40.0 {
                break;
            }
            at += field.heading(at, target) * 8.0;
            assert!(!navigation.is_blocked(at, Vec2::splat(8.0)), "{at:?}");
        }
        assert!(at.distance(target) < 40.0, "{at:?}");
    }

    #[test]
    fn a_solid_object_in_the_way_is_walked_around() {
        let target = Vec2::new(600.0, 300.0);
        let crate_at = (Vec2::new(400.0, 300.0), Vec2::new(64.0, 240.0));
        let field = field(&[], &[crate_at], target);

        let heading = field.heading(Vec2::new(250.0, 300.0), target);

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
        let field = field(&cells, &[], target);
        let from = Vec2::new(100.0, 480.0);

        let heading = field.heading(from, target);

        assert!((heading - (target - from).normalize()).length() < 0.001);
    }

    #[test]
    fn the_field_is_rebuilt_only_when_the_target_changes_cell_or_gets_old() {
        let navigation = Navigation::new(&[]);
        let physics = Physics::new();
        let mut field = FlowField::default();
        field.update(Vec2::new(300.0, 300.0), &navigation, &physics, 0.016);
        assert_eq!(field.age, 0.0);

        field.update(Vec2::new(400.0, 300.0), &navigation, &physics, 0.016);
        assert!(field.age > 0.0);

        field.update(Vec2::new(400.0, 300.0), &navigation, &physics, 0.1);
        assert_eq!(field.age, 0.0);
        assert_eq!(field.center, Some(cell_of(Vec2::new(400.0, 300.0))));

        field.update(Vec2::new(401.0, 300.0), &navigation, &physics, 0.3);
        assert!(field.age > 0.0);
        field.update(Vec2::new(401.0, 300.0), &navigation, &physics, 0.3);
        assert_eq!(field.age, 0.0);
    }
}
