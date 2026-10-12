use std::collections::HashMap;
use std::error::Error;

use serde::Deserialize;

use crate::rng::Rng;

pub(crate) const CELL_COLUMNS: usize = 28;
pub(crate) const CELL_ROWS: usize = 20;
pub(crate) const MIN_COLUMNS: usize = 16;
pub(crate) const MIN_ROWS: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub(crate) enum RoomKind {
    Start,
    Normal,
    Treasure,
    Shop,
    Boss,
    Sacrifice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Cell {
    Wall,
    Floor,
    Door,
    Spawn,
}

#[derive(Debug, Deserialize)]
struct TemplateFile {
    name: String,
    kind: RoomKind,
    rows: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct RoomTemplate {
    pub(crate) name: String,
    pub(crate) kind: RoomKind,
    pub(crate) cells: Vec<Vec<Cell>>,
}

impl RoomTemplate {
    pub(crate) fn parse(json: &str) -> Result<Self, Box<dyn Error>> {
        let file: TemplateFile = serde_json::from_str(json)?;
        let name = file.name;
        let rows = file.rows.len();
        let columns = file.rows.first().map_or(0, |line| line.chars().count());
        if !(MIN_ROWS..=CELL_ROWS).contains(&rows) || !rows.is_multiple_of(2) {
            return Err(format!("{name}: rows must be even and {MIN_ROWS}-{CELL_ROWS}").into());
        }
        if !(MIN_COLUMNS..=CELL_COLUMNS).contains(&columns) || !columns.is_multiple_of(2) {
            return Err(
                format!("{name}: columns must be even and {MIN_COLUMNS}-{CELL_COLUMNS}").into(),
            );
        }
        let mut cells = Vec::with_capacity(rows);
        for (row, line) in file.rows.iter().enumerate() {
            let symbols: Vec<char> = line.chars().collect();
            if symbols.len() != columns {
                return Err(format!("{name}: row {row} needs {columns} symbols").into());
            }
            let mut cell_row = Vec::with_capacity(columns);
            for (column, symbol) in symbols.into_iter().enumerate() {
                cell_row.push(match symbol {
                    '#' => Cell::Wall,
                    '.' => Cell::Floor,
                    'E' => Cell::Spawn,
                    'D' => Cell::Door,
                    other => {
                        return Err(
                            format!("{name}: unknown symbol '{other}' at {column},{row}").into(),
                        )
                    }
                });
            }
            cells.push(cell_row);
        }
        let template = Self {
            name,
            kind: file.kind,
            cells,
        };
        template.check_borders()?;
        Ok(template)
    }

    pub(crate) fn columns(&self) -> usize {
        self.cells.first().map_or(0, Vec::len)
    }

    pub(crate) fn rows(&self) -> usize {
        self.cells.len()
    }

    fn check_borders(&self) -> Result<(), Box<dyn Error>> {
        let (columns, rows) = (self.columns(), self.rows());
        for row in 0..rows {
            for column in 0..columns {
                let on_border = row == 0 || column == 0 || row == rows - 1 || column == columns - 1;
                let expected_door = is_door_slot(columns, rows, column, row);
                let cell = self.cells[row][column];
                if expected_door && cell != Cell::Door {
                    return Err(format!("{}: {column},{row} must be a door slot", self.name).into());
                }
                if !expected_door && cell == Cell::Door {
                    return Err(
                        format!("{}: {column},{row} is not a door position", self.name).into(),
                    );
                }
                if on_border && !expected_door && cell != Cell::Wall {
                    return Err(
                        format!("{}: border {column},{row} must be a wall", self.name).into(),
                    );
                }
            }
        }
        Ok(())
    }
}

pub(crate) fn door_columns(columns: usize) -> [usize; 2] {
    [columns / 2 - 1, columns / 2]
}

pub(crate) fn door_rows(rows: usize) -> [usize; 2] {
    [rows / 2 - 1, rows / 2]
}

pub(crate) fn is_door_slot(columns: usize, rows: usize, column: usize, row: usize) -> bool {
    let vertical = (row == 0 || row == rows - 1) && door_columns(columns).contains(&column);
    let horizontal = (column == 0 || column == columns - 1) && door_rows(rows).contains(&row);
    vertical || horizontal
}

const BUILT_IN: [&str; 21] = [
    include_str!("../../../assets/rooms/start.json"),
    include_str!("../../../assets/rooms/normal_01.json"),
    include_str!("../../../assets/rooms/normal_02.json"),
    include_str!("../../../assets/rooms/normal_03.json"),
    include_str!("../../../assets/rooms/normal_04.json"),
    include_str!("../../../assets/rooms/normal_05.json"),
    include_str!("../../../assets/rooms/normal_06.json"),
    include_str!("../../../assets/rooms/normal_07.json"),
    include_str!("../../../assets/rooms/normal_08.json"),
    include_str!("../../../assets/rooms/normal_09.json"),
    include_str!("../../../assets/rooms/normal_10.json"),
    include_str!("../../../assets/rooms/normal_11.json"),
    include_str!("../../../assets/rooms/normal_12.json"),
    include_str!("../../../assets/rooms/normal_13.json"),
    include_str!("../../../assets/rooms/normal_14.json"),
    include_str!("../../../assets/rooms/normal_15.json"),
    include_str!("../../../assets/rooms/normal_16.json"),
    include_str!("../../../assets/rooms/treasure.json"),
    include_str!("../../../assets/rooms/shop.json"),
    include_str!("../../../assets/rooms/sacrifice.json"),
    include_str!("../../../assets/rooms/boss.json"),
];

#[derive(Debug, Clone)]
pub(crate) struct Templates {
    by_kind: HashMap<RoomKind, Vec<RoomTemplate>>,
}

impl Templates {
    pub(crate) fn built_in() -> Result<Self, Box<dyn Error>> {
        Self::parse_all(&BUILT_IN)
    }

    pub(crate) fn parse_all(sources: &[&str]) -> Result<Self, Box<dyn Error>> {
        let mut by_kind: HashMap<RoomKind, Vec<RoomTemplate>> = HashMap::new();
        for source in sources {
            let template = RoomTemplate::parse(source)?;
            by_kind.entry(template.kind).or_default().push(template);
        }
        for kind in [
            RoomKind::Start,
            RoomKind::Normal,
            RoomKind::Treasure,
            RoomKind::Shop,
            RoomKind::Boss,
            RoomKind::Sacrifice,
        ] {
            if !by_kind.contains_key(&kind) {
                return Err(format!("no template for {kind:?}").into());
            }
        }
        Ok(Self { by_kind })
    }

    pub(crate) fn pick(&self, kind: RoomKind, rng: &mut Rng) -> Option<&RoomTemplate> {
        rng.pick(self.by_kind.get(&kind)?)
    }

    #[cfg(test)]
    pub(crate) fn all(&self) -> impl Iterator<Item = &RoomTemplate> {
        self.by_kind.values().flatten()
    }
}

#[cfg(test)]
mod tests {
    use crate::tilemap::{TileGrid, Tileset};

    use super::*;

    #[test]
    fn all_built_in_templates_are_valid_and_cover_every_room_kind() {
        let templates = Templates::built_in().expect("built-in templates are valid");
        let mut rng = Rng::new(1);
        for kind in [
            RoomKind::Start,
            RoomKind::Normal,
            RoomKind::Treasure,
            RoomKind::Shop,
            RoomKind::Boss,
            RoomKind::Sacrifice,
        ] {
            assert!(templates.pick(kind, &mut rng).is_some(), "{kind:?}");
        }
        assert_eq!(templates.all().count(), BUILT_IN.len());
    }

    #[test]
    fn the_templates_come_in_different_sizes() {
        let templates = Templates::built_in().expect("built-in templates are valid");
        let sizes: std::collections::HashSet<(usize, usize)> = templates
            .all()
            .map(|template| (template.columns(), template.rows()))
            .collect();
        assert!(sizes.len() >= 12, "only {} sizes", sizes.len());
    }

    #[test]
    fn every_wall_of_every_template_gets_a_solid_tile() {
        let tileset = Tileset::dungeon().expect("the embedded tileset is valid");
        let templates = Templates::built_in().expect("built-in templates are valid");
        for template in templates.all() {
            let mut grid = TileGrid::filled(template.columns(), template.rows());
            for (row, line) in template.cells.iter().enumerate() {
                for (column, cell) in line.iter().enumerate() {
                    if matches!(cell, Cell::Floor | Cell::Spawn) {
                        grid.carve(column, row);
                    }
                }
            }
            for row in 0..template.rows() {
                for column in 0..template.columns() {
                    if grid.is_interior(column, row) {
                        continue;
                    }
                    let wall = grid.is_wall(column as isize, row as isize);
                    let solid = tileset.is_solid(tileset.tile_number(&grid, column, row));
                    assert_eq!(wall, solid, "{} at {column},{row}", template.name);
                }
            }
        }
    }

    #[test]
    fn a_template_without_door_slots_is_rejected() {
        let rows: Vec<String> = (0..12)
            .map(|row| {
                if row == 0 || row == 11 {
                    "#".repeat(16)
                } else {
                    format!("#{}#", ".".repeat(14))
                }
            })
            .collect();
        let json = serde_json::json!({"name": "broken", "kind": "Normal", "rows": rows});
        assert!(RoomTemplate::parse(&json.to_string()).is_err());
    }

    #[test]
    fn odd_sizes_unknown_symbols_and_wrong_sizes_are_rejected() {
        let json = "{\"name\":\"tiny\",\"kind\":\"Normal\",\"rows\":[\"###\"]}";
        assert!(RoomTemplate::parse(json).is_err());
    }
}
