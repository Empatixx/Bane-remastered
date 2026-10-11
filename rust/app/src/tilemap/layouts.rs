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
