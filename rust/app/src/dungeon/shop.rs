use glam::Vec2;

use crate::tilemap::TILE_SIZE;

const SHOPKEEPER_SIZE: Vec2 = Vec2::new(320.0, 360.0);
const TABLE_SIZE: Vec2 = Vec2::new(144.0, 96.0);
const POT_SIZE: Vec2 = Vec2::new(192.0, 192.0);
const SLOT_MACHINE_SIZE: Vec2 = Vec2::new(192.0, 192.0);
const TABLES: usize = 3;
const CARPET_MARGIN_COLUMNS: usize = 2;
const CARPET_MARGIN_ROWS: usize = 4;
const POT_LEFT_TILES: f32 = 2.0;
const POT_RIGHT_TILES: f32 = 5.0;
const SLOT_MACHINE_RIGHT_GAP: f32 = 128.0;
const SLOT_MACHINE_TOP_GAP: f32 = 64.0;

const TOP_LEFT: usize = 0;
const TOP: [usize; 2] = [1, 2];
const TOP_RIGHT: usize = 3;
const LEFT: [usize; 2] = [4, 7];
const CENTER: usize = 5;
const RIGHT: [usize; 2] = [6, 8];
const BOTTOM_LEFT: usize = 9;
const BOTTOM: [usize; 2] = [10, 11];
const BOTTOM_RIGHT: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FixtureKind {
    Shopkeeper,
    Table,
    Pot,
    SlotMachine,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Fixture {
    pub(crate) kind: FixtureKind,
    pub(crate) top_left: Vec2,
    pub(crate) size: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Carpet {
    pub(crate) origin: (usize, usize),
    pub(crate) columns: usize,
    pub(crate) rows: usize,
}

impl Carpet {
    pub(crate) fn tiles(&self) -> Vec<((usize, usize), usize)> {
        let mut tiles = Vec::with_capacity(self.columns * self.rows);
        for row in 0..self.rows {
            for column in 0..self.columns {
                let number = self.tile_number(column, row);
                tiles.push(((self.origin.0 + column, self.origin.1 + row), number));
            }
        }
        tiles
    }

    fn tile_number(&self, column: usize, row: usize) -> usize {
        let variant = (column + row) % 2;
        let first = column == 0;
        let last = column + 1 == self.columns;
        let top = row == 0;
        let bottom = row + 1 == self.rows;
        match (first, last, top, bottom) {
            (true, _, true, _) => TOP_LEFT,
            (_, true, true, _) => TOP_RIGHT,
            (true, _, _, true) => BOTTOM_LEFT,
            (_, true, _, true) => BOTTOM_RIGHT,
            (_, _, true, _) => TOP[variant],
            (_, _, _, true) => BOTTOM[variant],
            (true, ..) => LEFT[variant],
            (_, true, ..) => RIGHT[variant],
            _ => CENTER,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ShopLayout {
    pub(crate) carpet: Carpet,
    pub(crate) fixtures: Vec<Fixture>,
}

pub(crate) fn plan(origin: (usize, usize), columns: usize, rows: usize) -> ShopLayout {
    let corner = Vec2::new(origin.0 as f32, origin.1 as f32) * TILE_SIZE;
    let width = columns as f32 * TILE_SIZE;
    let height = rows as f32 * TILE_SIZE;
    let place = |kind, local: Vec2, size: Vec2| Fixture {
        kind,
        top_left: corner + local,
        size,
    };

    let mut fixtures = vec![place(
        FixtureKind::Shopkeeper,
        Vec2::new(
            width / 2.0 - SHOPKEEPER_SIZE.x / 2.0,
            height / 4.0 - SHOPKEEPER_SIZE.y / 2.0,
        ),
        SHOPKEEPER_SIZE,
    )];
    for index in 0..TABLES {
        let center_x = width / 4.0 * (index + 1) as f32;
        fixtures.push(place(
            FixtureKind::Table,
            Vec2::new(
                center_x - TABLE_SIZE.x / 2.0,
                height / 2.0 - TABLE_SIZE.y / 2.0,
            ),
            TABLE_SIZE,
        ));
    }
    let pot_y = height * 4.0 / 5.0 - POT_SIZE.y / 2.0;
    fixtures.push(place(
        FixtureKind::Pot,
        Vec2::new(TILE_SIZE * POT_LEFT_TILES, pot_y),
        POT_SIZE,
    ));
    fixtures.push(place(
        FixtureKind::Pot,
        Vec2::new(width - TILE_SIZE * POT_RIGHT_TILES, pot_y),
        POT_SIZE,
    ));
    fixtures.push(place(
        FixtureKind::SlotMachine,
        Vec2::new(
            width - SLOT_MACHINE_SIZE.x - SLOT_MACHINE_RIGHT_GAP,
            SLOT_MACHINE_TOP_GAP,
        ),
        SLOT_MACHINE_SIZE,
    ));

    ShopLayout {
        carpet: Carpet {
            origin: (
                origin.0 + CARPET_MARGIN_COLUMNS,
                origin.1 + CARPET_MARGIN_ROWS,
            ),
            columns: columns - 2 * CARPET_MARGIN_COLUMNS,
            rows: rows - 2 * CARPET_MARGIN_ROWS,
        },
        fixtures,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(layout: &ShopLayout, kind: FixtureKind) -> usize {
        layout.fixtures.iter().filter(|f| f.kind == kind).count()
    }

    #[test]
    fn a_shop_has_one_keeper_three_tables_two_pots_and_a_slot_machine() {
        let layout = plan((10, 10), 20, 16);
        assert_eq!(kinds(&layout, FixtureKind::Shopkeeper), 1);
        assert_eq!(kinds(&layout, FixtureKind::Table), 3);
        assert_eq!(kinds(&layout, FixtureKind::Pot), 2);
        assert_eq!(kinds(&layout, FixtureKind::SlotMachine), 1);
    }

    #[test]
    fn fixtures_stay_inside_the_room() {
        let layout = plan((10, 10), 20, 16);
        let min = Vec2::new(10.0, 10.0) * TILE_SIZE;
        let max = min + Vec2::new(20.0, 16.0) * TILE_SIZE;
        for fixture in &layout.fixtures {
            assert!(fixture.top_left.cmpge(min).all(), "{fixture:?}");
            assert!(
                (fixture.top_left + fixture.size).cmple(max).all(),
                "{fixture:?}"
            );
        }
    }

    #[test]
    fn tables_are_evenly_spaced_in_the_middle_of_the_room() {
        let layout = plan((0, 0), 20, 16);
        let tables: Vec<f32> = layout
            .fixtures
            .iter()
            .filter(|f| f.kind == FixtureKind::Table)
            .map(|f| f.top_left.x + f.size.x / 2.0)
            .collect();
        assert_eq!(tables, vec![320.0, 640.0, 960.0]);
    }

    #[test]
    fn the_carpet_is_framed_by_corners_and_edges() {
        let layout = plan((0, 0), 20, 16);
        let carpet = layout.carpet;
        assert_eq!(
            (carpet.origin, carpet.columns, carpet.rows),
            ((2, 4), 16, 8)
        );
        let tiles = carpet.tiles();
        let number = |column: usize, row: usize| {
            tiles
                .iter()
                .find(|(tile, _)| *tile == (column, row))
                .map(|(_, number)| *number)
        };
        assert_eq!(number(2, 4), Some(TOP_LEFT));
        assert_eq!(number(17, 4), Some(TOP_RIGHT));
        assert_eq!(number(2, 11), Some(BOTTOM_LEFT));
        assert_eq!(number(17, 11), Some(BOTTOM_RIGHT));
        assert_eq!(number(8, 8), Some(CENTER));
        assert!(TOP.contains(&number(8, 4).unwrap_or(99)));
        assert!(LEFT.contains(&number(2, 8).unwrap_or(99)));
        assert_eq!(tiles.len(), 16 * 8);
    }
}
