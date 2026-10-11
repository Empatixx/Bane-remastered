use super::TileGrid;

pub(crate) const TILESET_COLUMNS: usize = 21;
pub(crate) const TILESET_ROWS: usize = 2;

const TOP_WALL: &[usize] = &[22, 23, 24, 25, 22, 23, 24, 25, 36, 37];
const BOTTOM_WALL: &[usize] = &[30, 31];
const LEFT_WALL: &[usize] = &[27, 39];
const RIGHT_WALL: &[usize] = &[28, 40];
const WALL: &[usize] = &[41];

const BOTTOM_OUTER_CORNER: &[usize] = TOP_WALL;
const TOP_LEFT_OUTER_CORNER: &[usize] = &[35];
const TOP_RIGHT_OUTER_CORNER: &[usize] = &[38];

const BOTTOM_RIGHT_INNER_CORNER: &[usize] = &[34];
const BOTTOM_LEFT_INNER_CORNER: &[usize] = &[29];
const TOP_RIGHT_INNER_CORNER: &[usize] = &[26];
const TOP_LEFT_INNER_CORNER: &[usize] = &[21];

const FLOOR_NEAR_LEFT_WALL: &[usize] = &[4];
const FLOOR_NEAR_RIGHT_WALL: &[usize] = &[7];
const FLOOR_NEAR_TOP_WALL: &[usize] = &[1, 2];
const FLOOR_NEAR_BOTTOM_WALL: &[usize] = &[9, 10];

const FLOOR_NEAR_LEFT_TOP_CORNER: &[usize] = &[0];
const FLOOR_NEAR_RIGHT_TOP_CORNER: &[usize] = &[3];
const FLOOR_NEAR_LEFT_BOTTOM_CORNER: &[usize] = &[8];
const FLOOR_NEAR_RIGHT_BOTTOM_CORNER: &[usize] = &[11];

const FLOOR: &[usize] = &[5, 6, 6, 6, 6, 6, 12, 13, 14, 15, 16, 17, 18, 19];

struct Neighbors {
    current: bool,
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
    top_left: bool,
    top_right: bool,
    bottom_left: bool,
    bottom_right: bool,
}

impl Neighbors {
    fn around(grid: &TileGrid, column: usize, row: usize) -> Self {
        let (x, y) = (column as isize, row as isize);
        Self {
            current: grid.is_wall(x, y),
            top: grid.is_wall(x, y - 1),
            bottom: grid.is_wall(x, y + 1),
            left: grid.is_wall(x - 1, y),
            right: grid.is_wall(x + 1, y),
            top_left: grid.is_wall(x - 1, y - 1),
            top_right: grid.is_wall(x + 1, y - 1),
            bottom_left: grid.is_wall(x - 1, y + 1),
            bottom_right: grid.is_wall(x + 1, y + 1),
        }
    }
}

pub(crate) fn tile_number(grid: &TileGrid, column: usize, row: usize) -> usize {
    let n = Neighbors::around(grid, column, row);
    let variants = if n.current {
        wall_variants(&n)
    } else {
        floor_variants(&n)
    };
    variants[variant_index(column, row, variants.len())]
}

fn wall_variants(n: &Neighbors) -> &'static [usize] {
    let Neighbors {
        top,
        bottom,
        left,
        right,
        top_left,
        top_right,
        bottom_left,
        bottom_right,
        ..
    } = *n;
    if top && bottom && right && left && top_left && top_right && bottom_left && bottom_right {
        WALL
    } else if top && bottom && right && !left {
        RIGHT_WALL
    } else if top && bottom && left && !right {
        LEFT_WALL
    } else if top && right && left && !bottom {
        TOP_WALL
    } else if bottom && right && left && !top {
        BOTTOM_WALL
    } else if bottom && left && !bottom_left {
        TOP_RIGHT_INNER_CORNER
    } else if bottom && right && !bottom_right {
        TOP_LEFT_INNER_CORNER
    } else if top && left && !top_left {
        BOTTOM_RIGHT_INNER_CORNER
    } else if top && right && !top_right {
        BOTTOM_LEFT_INNER_CORNER
    } else if !top && !left && right && bottom {
        TOP_LEFT_OUTER_CORNER
    } else if !top && left && !right && bottom {
        TOP_RIGHT_OUTER_CORNER
    } else if (top && !left && right && !bottom) || (top && left && !right && !bottom) {
        BOTTOM_OUTER_CORNER
    } else {
        FLOOR
    }
}

fn floor_variants(n: &Neighbors) -> &'static [usize] {
    match (n.top, n.bottom, n.left, n.right) {
        (true, _, _, true) => FLOOR_NEAR_RIGHT_TOP_CORNER,
        (true, _, true, _) => FLOOR_NEAR_LEFT_TOP_CORNER,
        (_, true, _, true) => FLOOR_NEAR_RIGHT_BOTTOM_CORNER,
        (_, true, true, _) => FLOOR_NEAR_LEFT_BOTTOM_CORNER,
        (_, true, _, _) => FLOOR_NEAR_BOTTOM_WALL,
        (true, _, _, _) => FLOOR_NEAR_TOP_WALL,
        (_, _, true, _) => FLOOR_NEAR_LEFT_WALL,
        (_, _, _, true) => FLOOR_NEAR_RIGHT_WALL,
        _ => FLOOR,
    }
}

fn variant_index(column: usize, row: usize, count: usize) -> usize {
    let hash = (column as u32).wrapping_mul(73_856_093) ^ (row as u32).wrapping_mul(19_349_663);
    (hash as usize) % count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_solid_tile(number: usize) -> bool {
        number / TILESET_COLUMNS > 0
    }

    #[test]
    fn walls_use_the_wall_row_and_floor_uses_the_floor_row() {
        let grid = TileGrid::bordered(8, 6);
        for row in 0..grid.rows() {
            for column in 0..grid.columns() {
                let number = tile_number(&grid, column, row);
                assert_eq!(
                    is_solid_tile(number),
                    grid.is_wall(column as isize, row as isize),
                    "tile {column},{row} -> {number}"
                );
            }
        }
    }

    #[test]
    fn same_position_always_gets_the_same_variant() {
        let grid = TileGrid::bordered(8, 6);
        assert_eq!(tile_number(&grid, 3, 0), tile_number(&grid, 3, 0));
    }

    #[test]
    fn outside_the_grid_counts_as_wall() {
        let grid = TileGrid::bordered(4, 4);
        assert!(grid.is_wall(-1, 0));
        assert!(grid.is_wall(0, 9));
    }
}
