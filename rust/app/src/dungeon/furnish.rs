use glam::Vec2;

use crate::assets::{Tex, tex};
use crate::tilemap::TILE_SIZE;

use super::template::RoomKind;

const BANE_ROWS: usize = 15;

const SHOPKEEPER_SIZE: Vec2 = Vec2::new(320.0, 360.0);
const TABLE_SIZE: Vec2 = Vec2::new(144.0, 96.0);
const POT_SIZE: Vec2 = Vec2::new(192.0, 192.0);
const SLOT_MACHINE_SIZE: Vec2 = Vec2::new(192.0, 192.0);
const TORCH_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const BRICKS_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const CHEST_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const STATUE_SIZE: Vec2 = Vec2::new(256.0, 256.0);
const CANDLESTICK_SIZE: Vec2 = Vec2::new(48.0, 48.0);
const PENTAGRAM_SIZE: Vec2 = Vec2::new(225.0, 120.0);
const BOOK_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const CHAINS_SIZE: Vec2 = Vec2::new(64.0, 128.0);
const KEY_SIZE: Vec2 = Vec2::new(33.0, 36.0);

const TABLES: usize = 3;
const SHOP_CARPET_COLUMNS: usize = 2;
const SHOP_CARPET_ROWS: usize = 4;
const TREASURE_CARPET_COLUMNS: usize = 7;
const TREASURE_CARPET_ROWS: usize = 5;

const TOP_LEFT: usize = 0;
const TOP: [usize; 2] = [1, 2];
const TOP_RIGHT: usize = 3;
const LEFT: [usize; 2] = [4, 7];
const CENTER: usize = 5;
const RIGHT: [usize; 2] = [6, 8];
const BOTTOM_LEFT: usize = 9;
const BOTTOM: [usize; 2] = [10, 11];
const BOTTOM_RIGHT: usize = 12;

const HINT_TOP: f32 = 300.0;
const HINT_STEP: f32 = 50.0;
const LABEL_GAP: Vec2 = Vec2::new(50.0, 30.0);
const MOVEMENT_TOP: f32 = 350.0;
const MOVEMENT_LABEL_TOP: f32 = 470.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Key {
    Q,
    E,
    F,
    One,
    Two,
    W,
    A,
    S,
    D,
}

impl Key {
    pub(crate) fn texture(self) -> Tex {
        match self {
            Self::Q => tex::KEYBOARD_Q,
            Self::E => tex::KEYBOARD_E,
            Self::F => tex::KEYBOARD_F,
            Self::One => tex::KEYBOARD_1,
            Self::Two => tex::KEYBOARD_2,
            Self::W => tex::KEYBOARD_W,
            Self::A => tex::KEYBOARD_A,
            Self::S => tex::KEYBOARD_S,
            Self::D => tex::KEYBOARD_D,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FixtureKind {
    Shopkeeper,
    Table,
    Pot,
    SlotMachine,
    Torch { right: bool },
    Bricks,
    Chest,
    AngelStatue,
    Candlestick,
    Pentagram,
    Book,
    Chains,
    Key(Key),
    Label { text: &'static str, centered: bool },
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
pub(crate) struct Furnishing {
    pub(crate) carpet: Option<Carpet>,
    pub(crate) fixtures: Vec<Fixture>,
}

struct Frame {
    origin: (usize, usize),
    columns: usize,
    rows: usize,
}

impl Frame {
    fn corner(&self) -> Vec2 {
        Vec2::new(self.origin.0 as f32, self.origin.1 as f32) * TILE_SIZE
    }

    fn width(&self) -> f32 {
        self.columns as f32 * TILE_SIZE
    }

    fn height(&self) -> f32 {
        self.rows as f32 * TILE_SIZE
    }

    fn shift(&self) -> f32 {
        (self.rows as f32 - BANE_ROWS as f32) / 2.0 * TILE_SIZE
    }

    fn place(&self, kind: FixtureKind, local: Vec2, size: Vec2) -> Fixture {
        Fixture {
            kind,
            top_left: self.corner() + local,
            size,
        }
    }

    fn tiles(&self, column: f32, row: f32) -> Vec2 {
        Vec2::new(column, row) * TILE_SIZE + Vec2::new(0.0, self.shift())
    }

    fn carpet(&self, margin_columns: usize, margin_rows: usize) -> Carpet {
        Carpet {
            origin: (self.origin.0 + margin_columns, self.origin.1 + margin_rows),
            columns: self.columns - 2 * margin_columns,
            rows: self.rows - 2 * margin_rows,
        }
    }
}

pub(crate) fn plan(
    kind: RoomKind,
    origin: (usize, usize),
    columns: usize,
    rows: usize,
) -> Option<Furnishing> {
    let frame = Frame {
        origin,
        columns,
        rows,
    };
    match kind {
        RoomKind::Shop => Some(shop(&frame)),
        RoomKind::Treasure => Some(treasure(&frame)),
        RoomKind::Sacrifice => Some(sacrifice(&frame)),
        RoomKind::Start => Some(start(&frame)),
        RoomKind::Normal | RoomKind::Boss | RoomKind::Ambush => None,
    }
}

fn shop(frame: &Frame) -> Furnishing {
    let (width, height) = (frame.width(), frame.height());
    let mut fixtures = vec![frame.place(
        FixtureKind::Shopkeeper,
        Vec2::new(
            width / 2.0 - SHOPKEEPER_SIZE.x / 2.0,
            height / 4.0 - SHOPKEEPER_SIZE.y / 2.0,
        ),
        SHOPKEEPER_SIZE,
    )];
    for index in 0..TABLES {
        let center_x = width / 4.0 * (index + 1) as f32;
        fixtures.push(frame.place(
            FixtureKind::Table,
            Vec2::new(
                center_x - TABLE_SIZE.x / 2.0,
                height / 2.0 - TABLE_SIZE.y / 2.0,
            ),
            TABLE_SIZE,
        ));
    }
    let pot_y = height * 4.0 / 5.0 - POT_SIZE.y / 2.0;
    fixtures.push(frame.place(
        FixtureKind::Pot,
        Vec2::new(TILE_SIZE * 2.0, pot_y),
        POT_SIZE,
    ));
    fixtures.push(frame.place(
        FixtureKind::Pot,
        Vec2::new(width - TILE_SIZE * 5.0, pot_y),
        POT_SIZE,
    ));
    fixtures.push(frame.place(
        FixtureKind::SlotMachine,
        Vec2::new(width - SLOT_MACHINE_SIZE.x - 128.0, 64.0),
        SLOT_MACHINE_SIZE,
    ));
    Furnishing {
        carpet: Some(frame.carpet(SHOP_CARPET_COLUMNS, SHOP_CARPET_ROWS)),
        fixtures,
    }
}

fn treasure(frame: &Frame) -> Furnishing {
    let (width, height) = (frame.width(), frame.height());
    let torch =
        |right, x, y| frame.place(FixtureKind::Torch { right }, Vec2::new(x, y), TORCH_SIZE);
    let fixtures = vec![
        torch(false, TILE_SIZE * 5.0, TILE_SIZE * 3.0),
        torch(true, width - TILE_SIZE * 6.0, TILE_SIZE * 3.0),
        torch(false, TILE_SIZE * 5.0, height - TILE_SIZE * 5.0),
        torch(true, width - TILE_SIZE * 6.0, height - TILE_SIZE * 5.0),
        frame.place(
            FixtureKind::Bricks,
            Vec2::new(TILE_SIZE * 4.0, TILE_SIZE * 4.5),
            BRICKS_SIZE,
        ),
        frame.place(
            FixtureKind::Bricks,
            Vec2::new(TILE_SIZE * 3.0, height - TILE_SIZE * 3.5),
            BRICKS_SIZE,
        ),
        frame.place(
            FixtureKind::Chest,
            Vec2::new(
                width / 2.0 - CHEST_SIZE.x / 2.0,
                height / 2.0 - CHEST_SIZE.y / 2.0,
            ),
            CHEST_SIZE,
        ),
    ];
    Furnishing {
        carpet: Some(frame.carpet(TREASURE_CARPET_COLUMNS, TREASURE_CARPET_ROWS)),
        fixtures,
    }
}

fn sacrifice(frame: &Frame) -> Furnishing {
    let (width, height) = (frame.width(), frame.height());
    let at = |kind, column, row, size| frame.place(kind, frame.tiles(column, row), size);
    let fixtures = vec![
        frame.place(
            FixtureKind::AngelStatue,
            Vec2::new(
                width / 2.0 - STATUE_SIZE.x / 2.0,
                height / 2.0 - STATUE_SIZE.y - 27.0,
            ),
            STATUE_SIZE,
        ),
        at(FixtureKind::Candlestick, 8.0, 6.0, CANDLESTICK_SIZE),
        at(FixtureKind::Candlestick, 7.5, 8.0, CANDLESTICK_SIZE),
        at(FixtureKind::Candlestick, 11.0, 6.0, CANDLESTICK_SIZE),
        at(FixtureKind::Candlestick, 11.5, 8.0, CANDLESTICK_SIZE),
        at(FixtureKind::Candlestick, 9.5, 9.0, CANDLESTICK_SIZE),
        frame.place(
            FixtureKind::Pentagram,
            Vec2::new(
                width / 2.0 - PENTAGRAM_SIZE.x / 2.0,
                height / 2.0 - PENTAGRAM_SIZE.y / 2.0,
            ),
            PENTAGRAM_SIZE,
        ),
        at(FixtureKind::Book, 6.0, 6.0, BOOK_SIZE),
        at(FixtureKind::Book, 13.0, 3.0, BOOK_SIZE),
        at(FixtureKind::Book, 11.0, 11.0, BOOK_SIZE),
        at(FixtureKind::Chains, 3.0, 4.0, CHAINS_SIZE),
        at(FixtureKind::Chains, 15.0, 11.0, CHAINS_SIZE),
    ];
    Furnishing {
        carpet: None,
        fixtures,
    }
}

fn start(frame: &Frame) -> Furnishing {
    let middle = frame.width() / 2.0;
    let hints = [
        (Key::Q, "Drop gun"),
        (Key::E, "Interact"),
        (Key::F, "Use artefact"),
        (Key::One, "Switch slot 1"),
        (Key::Two, "Switch slot 2"),
    ];
    let mut fixtures = Vec::new();
    for (index, (key, text)) in hints.into_iter().enumerate() {
        let top = HINT_TOP + index as f32 * HINT_STEP;
        fixtures.push(frame.place(FixtureKind::Key(key), Vec2::new(middle, top), KEY_SIZE));
        fixtures.push(frame.place(
            FixtureKind::Label {
                text,
                centered: false,
            },
            Vec2::new(middle + LABEL_GAP.x, top + LABEL_GAP.y),
            Vec2::ZERO,
        ));
    }
    let movement = [
        (Key::W, -150.0, MOVEMENT_TOP),
        (Key::S, -150.0, MOVEMENT_TOP + HINT_STEP),
        (Key::A, -200.0, MOVEMENT_TOP + HINT_STEP),
        (Key::D, -100.0, MOVEMENT_TOP + HINT_STEP),
    ];
    for (key, dx, top) in movement {
        fixtures.push(frame.place(FixtureKind::Key(key), Vec2::new(middle + dx, top), KEY_SIZE));
    }
    let label_center = middle + (-200.0 + -100.0 + KEY_SIZE.x) / 2.0;
    fixtures.push(frame.place(
        FixtureKind::Label {
            text: "Movement",
            centered: true,
        },
        Vec2::new(label_center, MOVEMENT_LABEL_TOP),
        Vec2::ZERO,
    ));
    Furnishing {
        carpet: None,
        fixtures,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(furnishing: &Furnishing, wanted: impl Fn(&FixtureKind) -> bool) -> usize {
        furnishing
            .fixtures
            .iter()
            .filter(|f| wanted(&f.kind))
            .count()
    }

    fn furnish(kind: RoomKind, columns: usize, rows: usize) -> Furnishing {
        plan(kind, (10, 10), columns, rows).expect("this room kind is furnished")
    }

    #[test]
    fn a_shop_has_one_keeper_three_tables_two_pots_and_a_slot_machine() {
        let shop = furnish(RoomKind::Shop, 20, 16);
        assert_eq!(count(&shop, |k| *k == FixtureKind::Shopkeeper), 1);
        assert_eq!(count(&shop, |k| *k == FixtureKind::Table), 3);
        assert_eq!(count(&shop, |k| *k == FixtureKind::Pot), 2);
        assert_eq!(count(&shop, |k| *k == FixtureKind::SlotMachine), 1);
    }

    #[test]
    fn treasure_has_four_torches_two_bricks_and_a_chest_on_a_carpet() {
        let treasure = furnish(RoomKind::Treasure, 20, 16);
        let torches = |right: bool| count(&treasure, |k| *k == FixtureKind::Torch { right });
        assert_eq!((torches(false), torches(true)), (2, 2));
        assert_eq!(count(&treasure, |k| *k == FixtureKind::Bricks), 2);
        assert_eq!(count(&treasure, |k| *k == FixtureKind::Chest), 1);
        let carpet = treasure.carpet.expect("treasure has a carpet");
        assert_eq!(
            (carpet.origin, carpet.columns, carpet.rows),
            ((17, 15), 6, 6)
        );
    }

    #[test]
    fn a_sacrifice_room_has_a_statue_five_candles_a_pentagram_books_and_chains() {
        let room = furnish(RoomKind::Sacrifice, 20, 16);
        assert_eq!(count(&room, |k| *k == FixtureKind::AngelStatue), 1);
        assert_eq!(count(&room, |k| *k == FixtureKind::Candlestick), 5);
        assert_eq!(count(&room, |k| *k == FixtureKind::Pentagram), 1);
        assert_eq!(count(&room, |k| *k == FixtureKind::Book), 3);
        assert_eq!(count(&room, |k| *k == FixtureKind::Chains), 2);
        assert!(room.carpet.is_none());
    }

    #[test]
    fn the_start_room_teaches_seven_keys() {
        let room = furnish(RoomKind::Start, 20, 16);
        assert_eq!(count(&room, |k| matches!(k, FixtureKind::Key(_))), 9);
        assert_eq!(count(&room, |k| matches!(k, FixtureKind::Label { .. })), 6);
    }

    #[test]
    fn plain_rooms_are_not_furnished() {
        assert!(plan(RoomKind::Normal, (0, 0), 20, 16).is_none());
        assert!(plan(RoomKind::Boss, (0, 0), 20, 16).is_none());
    }

    #[test]
    fn fixtures_stay_inside_the_room() {
        for kind in [
            RoomKind::Shop,
            RoomKind::Treasure,
            RoomKind::Sacrifice,
            RoomKind::Start,
        ] {
            let room = furnish(kind, 20, 16);
            let min = Vec2::new(10.0, 10.0) * TILE_SIZE;
            let max = min + Vec2::new(20.0, 16.0) * TILE_SIZE;
            for fixture in &room.fixtures {
                assert!(fixture.top_left.cmpge(min).all(), "{kind:?} {fixture:?}");
                assert!(
                    (fixture.top_left + fixture.size).cmple(max).all(),
                    "{kind:?} {fixture:?}"
                );
            }
        }
    }

    #[test]
    fn shop_tables_are_evenly_spaced_in_the_middle() {
        let shop = plan(RoomKind::Shop, (0, 0), 20, 16).expect("shop is furnished");
        let tables: Vec<f32> = shop
            .fixtures
            .iter()
            .filter(|f| f.kind == FixtureKind::Table)
            .map(|f| f.top_left.x + f.size.x / 2.0)
            .collect();
        assert_eq!(tables, vec![320.0, 640.0, 960.0]);
    }

    #[test]
    fn the_carpet_is_framed_by_corners_and_edges() {
        let carpet = plan(RoomKind::Shop, (0, 0), 20, 16)
            .and_then(|shop| shop.carpet)
            .expect("shop has a carpet");
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
