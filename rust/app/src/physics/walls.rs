#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CellRect {
    pub(super) column: usize,
    pub(super) row: usize,
    pub(super) width: usize,
    pub(super) height: usize,
}

pub(super) fn merge_cells(cells: &[(usize, usize)]) -> Vec<CellRect> {
    let columns = cells
        .iter()
        .map(|&(column, _)| column + 1)
        .max()
        .unwrap_or(0);
    let rows = cells.iter().map(|&(_, row)| row + 1).max().unwrap_or(0);
    let mut free = vec![vec![false; columns]; rows];
    for &(column, row) in cells {
        free[row][column] = true;
    }

    let mut rects = Vec::new();
    for row in 0..rows {
        let mut column = 0;
        while column < columns {
            if !free[row][column] {
                column += 1;
                continue;
            }
            let mut width = 1;
            while column + width < columns && free[row][column + width] {
                width += 1;
            }
            let mut height = 1;
            while row + height < rows && (column..column + width).all(|c| free[row + height][c]) {
                height += 1;
            }
            for covered in free.iter_mut().skip(row).take(height) {
                covered[column..column + width].fill(false);
            }
            rects.push(CellRect {
                column,
                row,
                width,
                height,
            });
            column += width;
        }
    }
    rects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_solid_block_becomes_one_rectangle() {
        let cells: Vec<_> = (0..3)
            .flat_map(|row| (0..4).map(move |column| (column, row)))
            .collect();
        let rects = merge_cells(&cells);
        assert_eq!(
            rects,
            vec![CellRect {
                column: 0,
                row: 0,
                width: 4,
                height: 3
            }]
        );
    }

    #[test]
    fn rectangles_cover_every_cell_exactly_once() {
        let cells = [
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ];
        let rects = merge_cells(&cells);
        let covered: usize = rects.iter().map(|r| r.width * r.height).sum();
        assert_eq!(covered, cells.len());
    }

    #[test]
    fn empty_input_gives_no_rectangles() {
        assert!(merge_cells(&[]).is_empty());
    }
}
