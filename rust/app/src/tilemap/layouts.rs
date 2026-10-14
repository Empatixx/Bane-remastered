use super::TileGrid;

/// Columns of the lobby's main hall, its walls included.
pub(crate) const HALL_COLUMNS: usize = 29;
/// The first floor column of the shooting range east of the hall.
pub(crate) const RANGE_START: usize = HALL_COLUMNS + 1;
/// Floor columns of the shooting range.
const RANGE_COLUMNS: usize = 11;
/// The rows where the wall between the hall and the range is open.
pub(crate) const RANGE_DOOR: [usize; 2] = [4, 5];
const LOBBY_ROWS: usize = 10;

/// The main hall, and through a door in its east wall the shooting range.
pub(crate) fn lobby() -> TileGrid {
    let mut grid = TileGrid::bordered(RANGE_START + RANGE_COLUMNS + 1, LOBBY_ROWS);
    for column in HALL_COLUMNS - 1..RANGE_START {
        for row in 1..LOBBY_ROWS - 1 {
            if !RANGE_DOOR.contains(&row) {
                grid.fill(column, row);
            }
        }
    }
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

pub(crate) fn crypt() -> TileGrid {
    TileGrid::bordered(30, 12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_range_opens_off_the_hall_through_its_door_only() {
        let grid = lobby();
        let wall = (HALL_COLUMNS - 1) as isize;
        for row in 1..(LOBBY_ROWS - 1) as isize {
            let open = RANGE_DOOR.contains(&(row as usize));
            assert_eq!(!grid.is_wall(wall, row), open, "row {row}");
            assert_eq!(!grid.is_wall(wall + 1, row), open, "row {row}");
        }
        for column in RANGE_START..RANGE_START + RANGE_COLUMNS {
            assert!(!grid.is_wall(column as isize, 3));
        }
        assert!(grid.is_wall((RANGE_START + RANGE_COLUMNS) as isize, 3));
    }
}
