use std::error::Error;

use serde::Deserialize;

use super::TileGrid;

const DUNGEON: &str = include_str!("../../../assets/tilesets/dungeon.json");

#[derive(Debug, Deserialize)]
struct TilesetFile {
    columns: usize,
    rows: usize,
    solid_from_row: usize,
    rules: Vec<RuleFile>,
}

#[derive(Debug, Deserialize)]
struct RuleFile {
    name: String,
    pattern: [String; 3],
    tiles: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Floor,
    Any,
}

impl Cell {
    fn parse(symbol: char) -> Option<Self> {
        match symbol {
            '#' => Some(Self::Wall),
            '.' => Some(Self::Floor),
            '?' => Some(Self::Any),
            _ => None,
        }
    }

    fn accepts(self, wall: bool) -> bool {
        match self {
            Self::Wall => wall,
            Self::Floor => !wall,
            Self::Any => true,
        }
    }
}

#[derive(Debug)]
struct Rule {
    pattern: [Cell; 9],
    tiles: Vec<usize>,
}

impl Rule {
    fn matches(&self, neighborhood: &[bool; 9]) -> bool {
        self.pattern
            .iter()
            .zip(neighborhood)
            .all(|(cell, wall)| cell.accepts(*wall))
    }
}

#[derive(Debug)]
pub(crate) struct Tileset {
    columns: usize,
    rows: usize,
    solid_from_row: usize,
    rules: Vec<Rule>,
}

impl Tileset {
    pub(crate) fn dungeon() -> Result<Self, Box<dyn Error>> {
        Self::parse(DUNGEON)
    }

    pub(crate) fn parse(json: &str) -> Result<Self, Box<dyn Error>> {
        let file: TilesetFile = serde_json::from_str(json)?;
        let rules = file
            .rules
            .iter()
            .map(|rule| parse_rule(rule, file.columns * file.rows))
            .collect::<Result<Vec<_>, _>>()?;
        let tileset = Self {
            columns: file.columns,
            rows: file.rows,
            solid_from_row: file.solid_from_row,
            rules,
        };
        tileset.check_covers_every_neighborhood()?;
        Ok(tileset)
    }

    pub(crate) fn columns(&self) -> usize {
        self.columns
    }

    pub(crate) fn rows(&self) -> usize {
        self.rows
    }

    pub(crate) fn is_solid(&self, tile_number: usize) -> bool {
        tile_number / self.columns >= self.solid_from_row
    }

    pub(crate) fn tile_number(&self, grid: &TileGrid, column: usize, row: usize) -> usize {
        let neighborhood = neighborhood(grid, column, row);
        self.rules
            .iter()
            .find(|rule| rule.matches(&neighborhood))
            .map_or(0, |rule| {
                rule.tiles[variant_index(column, row, rule.tiles.len())]
            })
    }

    fn check_covers_every_neighborhood(&self) -> Result<(), Box<dyn Error>> {
        for bits in 0u16..512 {
            let neighborhood: [bool; 9] = std::array::from_fn(|i| bits & (1 << i) != 0);
            if !self.rules.iter().any(|rule| rule.matches(&neighborhood)) {
                return Err(format!("no tileset rule matches the neighborhood {bits:09b}").into());
            }
        }
        Ok(())
    }
}

fn parse_rule(rule: &RuleFile, tile_count: usize) -> Result<Rule, Box<dyn Error>> {
    let mut pattern = [Cell::Any; 9];
    for (row, line) in rule.pattern.iter().enumerate() {
        let symbols: Vec<char> = line.chars().collect();
        if symbols.len() != 3 {
            return Err(format!("rule {}: pattern rows need 3 symbols", rule.name).into());
        }
        for (column, symbol) in symbols.into_iter().enumerate() {
            pattern[row * 3 + column] = Cell::parse(symbol)
                .ok_or_else(|| format!("rule {}: unknown symbol '{symbol}'", rule.name))?;
        }
    }
    if rule.tiles.is_empty() || rule.tiles.iter().any(|tile| *tile >= tile_count) {
        return Err(format!("rule {}: tiles are empty or outside the tileset", rule.name).into());
    }
    Ok(Rule {
        pattern,
        tiles: rule.tiles.clone(),
    })
}

fn neighborhood(grid: &TileGrid, column: usize, row: usize) -> [bool; 9] {
    let (x, y) = (column as isize, row as isize);
    std::array::from_fn(|i| {
        let (dx, dy) = ((i % 3) as isize - 1, (i / 3) as isize - 1);
        grid.is_wall(x + dx, y + dy)
    })
}

fn variant_index(column: usize, row: usize, count: usize) -> usize {
    let hash = (column as u32).wrapping_mul(73_856_093) ^ (row as u32).wrapping_mul(19_349_663);
    (hash as usize) % count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dungeon() -> Tileset {
        Tileset::dungeon().expect("the embedded tileset is valid")
    }

    #[test]
    fn walls_use_the_wall_row_and_floor_uses_the_floor_row() {
        let tileset = dungeon();
        let grid = TileGrid::bordered(8, 6);
        for row in 0..grid.rows() {
            for column in 0..grid.columns() {
                let number = tileset.tile_number(&grid, column, row);
                assert_eq!(
                    tileset.is_solid(number),
                    grid.is_wall(column as isize, row as isize),
                    "tile {column},{row} -> {number}"
                );
            }
        }
    }

    #[test]
    fn same_position_always_gets_the_same_variant() {
        let tileset = dungeon();
        let grid = TileGrid::bordered(8, 6);
        assert_eq!(
            tileset.tile_number(&grid, 3, 0),
            tileset.tile_number(&grid, 3, 0)
        );
    }

    #[test]
    fn outside_the_grid_counts_as_wall() {
        let grid = TileGrid::bordered(4, 4);
        assert!(grid.is_wall(-1, 0));
        assert!(grid.is_wall(0, 9));
    }

    #[test]
    fn rejects_rules_that_leave_a_neighborhood_uncovered() {
        let json = r#"{"columns":2,"rows":1,"solid_from_row":1,
            "rules":[{"name":"only_walls","pattern":["???","?#?","???"],"tiles":[0]}]}"#;
        assert!(Tileset::parse(json).is_err());
    }

    #[test]
    fn rejects_tiles_outside_the_tileset() {
        let json = r#"{"columns":2,"rows":1,"solid_from_row":1,
            "rules":[{"name":"any","pattern":["???","?.?","???"],"tiles":[5]}]}"#;
        assert!(Tileset::parse(json).is_err());
    }

    #[test]
    fn every_wall_of_the_built_in_layouts_gets_a_solid_tile() {
        let tileset = dungeon();
        for grid in [crate::tilemap::lobby()] {
            for row in 0..grid.rows() {
                for column in 0..grid.columns() {
                    let wall = grid.is_wall(column as isize, row as isize);
                    let solid = tileset.is_solid(tileset.tile_number(&grid, column, row));
                    assert_eq!(wall, solid, "cell {column},{row}");
                }
            }
        }
    }
}
