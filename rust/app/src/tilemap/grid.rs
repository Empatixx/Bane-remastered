use glam::Vec2;

use super::TILE_SIZE;

#[derive(Debug, Clone)]
pub(crate) struct TileGrid {
    walls: Vec<Vec<bool>>,
}

impl TileGrid {
    pub(crate) fn bordered(columns: usize, rows: usize) -> Self {
        let walls = (0..rows)
            .map(|y| {
                (0..columns)
                    .map(|x| x == 0 || y == 0 || x == columns - 1 || y == rows - 1)
                    .collect()
            })
            .collect();
        Self { walls }
    }

    pub(crate) fn fill(&mut self, column: usize, row: usize) {
        if let Some(cell) = self.walls.get_mut(row).and_then(|r| r.get_mut(column)) {
            *cell = true;
        }
    }

    pub(crate) fn columns(&self) -> usize {
        self.walls.first().map_or(0, Vec::len)
    }

    pub(crate) fn rows(&self) -> usize {
        self.walls.len()
    }

    pub(crate) fn is_wall(&self, column: isize, row: isize) -> bool {
        let (Ok(column), Ok(row)) = (usize::try_from(column), usize::try_from(row)) else {
            return true;
        };
        self.walls
            .get(row)
            .and_then(|r| r.get(column))
            .copied()
            .unwrap_or(true)
    }

    pub(crate) fn pixel_size(&self) -> Vec2 {
        Vec2::new(self.columns() as f32, self.rows() as f32) * TILE_SIZE
    }
}
