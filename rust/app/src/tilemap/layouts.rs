use super::TileGrid;

pub(crate) fn lobby() -> TileGrid {
    let mut grid = TileGrid::bordered(29, 10);
    for column in (11..13).chain(16..18) {
        for row in [1, 2, 7, 8] {
            grid.fill(column, row);
        }
    }
    for column in 11..18 {
        grid.fill(column, 8);
    }
    grid
}

pub(crate) fn arena() -> TileGrid {
    let mut grid = TileGrid::bordered(20, 12);
    for (column, row) in [(6, 4), (13, 4), (6, 7), (13, 7)] {
        for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
            grid.fill(column + dx, row + dy);
        }
    }
    grid
}
