use std::collections::HashMap;
use std::error::Error;

use serde::Deserialize;

use super::rng::Rng;

pub(crate) const ROOM_COLUMNS: usize = 20;
pub(crate) const ROOM_ROWS: usize = 15;
pub(crate) const DOOR_COLUMNS: [usize; 2] = [9, 10];
pub(crate) const DOOR_ROWS: [usize; 2] = [7, 8];

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
        if file.rows.len() != ROOM_ROWS {
            return Err(format!("{name}: expected {ROOM_ROWS} rows").into());
        }
        let mut cells = Vec::with_capacity(ROOM_ROWS);
        for (row, line) in file.rows.iter().enumerate() {
            let symbols: Vec<char> = line.chars().collect();
            if symbols.len() != ROOM_COLUMNS {
                return Err(format!("{name}: row {row} needs {ROOM_COLUMNS} symbols").into());
            }
            let mut cell_row = Vec::with_capacity(ROOM_COLUMNS);
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

    fn check_borders(&self) -> Result<(), Box<dyn Error>> {
        for row in 0..ROOM_ROWS {
            for column in 0..ROOM_COLUMNS {
                let on_border =
                    row == 0 || column == 0 || row == ROOM_ROWS - 1 || column == ROOM_COLUMNS - 1;
                let expected_door = is_door_slot(column, row);
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

pub(crate) fn is_door_slot(column: usize, row: usize) -> bool {
    let vertical = (row == 0 || row == ROOM_ROWS - 1) && DOOR_COLUMNS.contains(&column);
    let horizontal = (column == 0 || column == ROOM_COLUMNS - 1) && DOOR_ROWS.contains(&row);
    vertical || horizontal
}

const BUILT_IN: [&str; 8] = [
    include_str!("../../../assets/rooms/start.json"),
    include_str!("../../../assets/rooms/normal_a.json"),
    include_str!("../../../assets/rooms/normal_b.json"),
    include_str!("../../../assets/rooms/normal_c.json"),
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
}

#[cfg(test)]
mod tests {
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
    }

    #[test]
    fn a_template_without_door_slots_is_rejected() {
        let rows: Vec<String> = (0..ROOM_ROWS)
            .map(|row| {
                if row == 0 || row == ROOM_ROWS - 1 {
                    "#".repeat(ROOM_COLUMNS)
                } else {
                    format!("#{}#", ".".repeat(ROOM_COLUMNS - 2))
                }
            })
            .collect();
        let json = serde_json::json!({"name": "broken", "kind": "Normal", "rows": rows});
        assert!(RoomTemplate::parse(&json.to_string()).is_err());
    }

    #[test]
    fn unknown_symbols_and_wrong_sizes_are_rejected() {
        let json = "{\"name\":\"tiny\",\"kind\":\"Normal\",\"rows\":[\"###\"]}";
        assert!(RoomTemplate::parse(json).is_err());
    }
}
