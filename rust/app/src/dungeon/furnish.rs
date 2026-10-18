use glam::Vec2;

use crate::assets::{Tex, tex};
use crate::tilemap::TILE_SIZE;

use super::template::RoomKind;

const BANE_ROWS: usize = 15;

const SHOPKEEPER_SIZE: Vec2 = Vec2::new(320.0, 360.0);
const TABLE_SIZE: Vec2 = Vec2::new(144.0, 96.0);
const POT_SIZE: Vec2 = Vec2::new(192.0, 192.0);
const SLOT_MACHINE_SIZE: Vec2 = Vec2::new(192.0, 192.0);
const COUNTER_SIZE: Vec2 = Vec2::new(320.0, 112.0);
const SHELF_SIZE: Vec2 = Vec2::new(128.0, 192.0);
const GOODS_SIZE: Vec2 = Vec2::new(128.0, 128.0);
const LAMP_SIZE: Vec2 = Vec2::new(64.0, 128.0);
/// The counter stands in front of the keeper, hiding the hem of the robe.
const COUNTER_TOP: f32 = 330.0;
/// The tables stand a little below the middle of the shop, clear of the counter.
const TABLES_BELOW_MIDDLE: f32 = 32.0;
const TORCH_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const BRICKS_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const CHEST_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const STATUE_SIZE: Vec2 = Vec2::new(256.0, 256.0);
const CANDLESTICK_SIZE: Vec2 = Vec2::new(48.0, 48.0);
const PENTAGRAM_SIZE: Vec2 = Vec2::new(225.0, 120.0);
const BOOK_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const CHAINS_SIZE: Vec2 = Vec2::new(64.0, 128.0);
const KEY_SIZE: Vec2 = Vec2::new(33.0, 36.0);
const SPACE_KEY_SIZE: Vec2 = Vec2::new(99.0, 36.0);
const SPACE_LABEL_GAP_X: f32 = 12.0;
const SPACE_BELOW_MOVEMENT: f32 = 55.0;

const TABLES: usize = 3;
const SHOP_CARPET_COLUMNS: usize = 2;
const SHOP_CARPET_ROWS: usize = 4;
const TREASURE_CARPET_COLUMNS: usize = 7;
const TREASURE_CARPET_ROWS: usize = 5;


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
    Space,
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
            Self::Space => tex::KEYBOARD_SPACE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FixtureKind {
    Shopkeeper,
    Table,
    Pot,
    SlotMachine,
    Counter,
    Shelf { right: bool },
    Goods { barrel: bool },
    Lamp,
    Torch { right: bool },
    /// A pile of loose bricks in the floor's own stone: its row in the sheet (see `Theme::style`).
    Bricks { style: usize },
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

/// A carpet on the floor: the tiles it spans, and which of them it covers, as a shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Carpet {
    pub(crate) origin: (usize, usize),
    pub(crate) columns: usize,
    pub(crate) rows: usize,
    /// Its look, one per floor (see `Theme::style`); the shop's and the treasure rooms' are red.
    pub(crate) style: usize,
    /// Which tiles of its span it covers, a bit each row by row from the top left; `None`
    /// covers them all.
    pub(crate) cover: Option<u128>,
}

impl Carpet {
    /// A carpet covering the whole of `columns` by `rows` tiles from `origin`.
    pub(crate) fn rect(origin: (usize, usize), columns: usize, rows: usize, style: usize) -> Self {
        Self { origin, columns, rows, style, cover: None }
    }

    /// Whether it covers the tile `column`, `row` of its span.
    pub(crate) fn covers(&self, column: isize, row: isize) -> bool {
        if column < 0 || row < 0 || column as usize >= self.columns || row as usize >= self.rows {
            return false;
        }
        self.cover.is_none_or(|cover| cover >> (row as usize * self.columns + column as usize) & 1 == 1)
    }

    /// The tiles it covers, each with the blob tile that draws it: its border running round the
    /// outside of the shape and round the notches where the shape turns in.
    pub(crate) fn tiles(&self) -> Vec<((usize, usize), usize)> {
        let mut tiles = Vec::with_capacity(self.columns * self.rows);
        for row in 0..self.rows as isize {
            for column in 0..self.columns as isize {
                if !self.covers(column, row) {
                    continue;
                }
                let number = super::surface::blob_index(|dx, dy| self.covers(column + dx, row + dy));
                tiles.push(((self.origin.0 + column as usize, self.origin.1 + row as usize), number));
            }
        }
        tiles
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
        Carpet::rect(
            (self.origin.0 + margin_columns, self.origin.1 + margin_rows),
            self.columns - 2 * margin_columns,
            self.rows - 2 * margin_rows,
            0,
        )
    }
}

/// `style` is the floor's look (see `Theme::style`), which the treasure rooms' bricks take.
pub(crate) fn plan(
    kind: RoomKind,
    origin: (usize, usize),
    columns: usize,
    rows: usize,
    style: usize,
) -> Option<Furnishing> {
    let frame = Frame {
        origin,
        columns,
        rows,
    };
    match kind {
        RoomKind::Shop => Some(shop(&frame)),
        RoomKind::Treasure => Some(treasure(&frame, style)),
        RoomKind::Sacrifice => Some(sacrifice(&frame)),
        RoomKind::Start => Some(start(&frame)),
        RoomKind::Normal | RoomKind::Boss | RoomKind::Ambush => None,
    }
}

/// The keeper behind a counter between two shelves of wares, the goods on three draped tables
/// with a lamp between each two, a pile of stock in two corners, two pots and the slot machine.
fn shop(frame: &Frame) -> Furnishing {
    let (width, height) = (frame.width(), frame.height());
    let center = width / 2.0;
    let mut fixtures = vec![
        frame.place(
            FixtureKind::Shopkeeper,
            Vec2::new(
                center - SHOPKEEPER_SIZE.x / 2.0,
                height / 4.0 - SHOPKEEPER_SIZE.y / 2.0,
            ),
            SHOPKEEPER_SIZE,
        ),
        frame.place(
            FixtureKind::Counter,
            Vec2::new(center - COUNTER_SIZE.x / 2.0, COUNTER_TOP),
            COUNTER_SIZE,
        ),
    ];
    for right in [false, true] {
        let gap = COUNTER_SIZE.x / 2.0 + TILE_SIZE;
        let x = if right {
            center + gap
        } else {
            center - gap - SHELF_SIZE.x
        };
        fixtures.push(frame.place(
            FixtureKind::Shelf { right },
            Vec2::new(x, TILE_SIZE),
            SHELF_SIZE,
        ));
    }
    let tables_y = height / 2.0 + TABLES_BELOW_MIDDLE;
    for index in 0..TABLES {
        let center_x = width / 4.0 * (index + 1) as f32;
        fixtures.push(frame.place(
            FixtureKind::Table,
            Vec2::new(center_x - TABLE_SIZE.x / 2.0, tables_y - TABLE_SIZE.y / 2.0),
            TABLE_SIZE,
        ));
    }
    for index in 1..TABLES {
        let between = width / 4.0 * (index as f32 + 0.5);
        fixtures.push(frame.place(
            FixtureKind::Lamp,
            Vec2::new(
                between - LAMP_SIZE.x / 2.0,
                tables_y + TABLE_SIZE.y / 2.0 - LAMP_SIZE.y,
            ),
            LAMP_SIZE,
        ));
    }
    fixtures.push(frame.place(
        FixtureKind::Goods { barrel: false },
        Vec2::splat(TILE_SIZE * 1.5),
        GOODS_SIZE,
    ));
    fixtures.push(frame.place(
        FixtureKind::Goods { barrel: true },
        Vec2::new(
            width - TILE_SIZE - GOODS_SIZE.x,
            height - TILE_SIZE * 1.25 - GOODS_SIZE.y,
        ),
        GOODS_SIZE,
    ));
    let pot_y = height * 4.0 / 5.0 - POT_SIZE.y / 2.0;
    fixtures.push(frame.place(
        FixtureKind::Pot,
        Vec2::new(TILE_SIZE * 2.0, pot_y),
        POT_SIZE,
    ));
    fixtures.push(frame.place(
        FixtureKind::Pot,
        Vec2::new(width - TILE_SIZE * 6.0, pot_y),
        POT_SIZE,
    ));
    fixtures.push(frame.place(
        FixtureKind::SlotMachine,
        Vec2::new(width - SLOT_MACHINE_SIZE.x - TILE_SIZE * 1.25, TILE_SIZE),
        SLOT_MACHINE_SIZE,
    ));
    Furnishing {
        carpet: Some(frame.carpet(SHOP_CARPET_COLUMNS, SHOP_CARPET_ROWS)),
        fixtures,
    }
}

fn treasure(frame: &Frame, style: usize) -> Furnishing {
    let (width, height) = (frame.width(), frame.height());
    let torch =
        |right, x, y| frame.place(FixtureKind::Torch { right }, Vec2::new(x, y), TORCH_SIZE);
    let fixtures = vec![
        torch(false, TILE_SIZE * 5.0, TILE_SIZE * 3.0),
        torch(true, width - TILE_SIZE * 6.0, TILE_SIZE * 3.0),
        torch(false, TILE_SIZE * 5.0, height - TILE_SIZE * 5.0),
        torch(true, width - TILE_SIZE * 6.0, height - TILE_SIZE * 5.0),
        frame.place(
            FixtureKind::Bricks { style },
            Vec2::new(TILE_SIZE * 4.0, TILE_SIZE * 4.5),
            BRICKS_SIZE,
        ),
        frame.place(
            FixtureKind::Bricks { style },
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
    let space_top = MOVEMENT_LABEL_TOP + SPACE_BELOW_MOVEMENT;
    let space_left = label_center - SPACE_KEY_SIZE.x / 2.0;
    fixtures.push(frame.place(
        FixtureKind::Key(Key::Space),
        Vec2::new(space_left, space_top),
        SPACE_KEY_SIZE,
    ));
    fixtures.push(frame.place(
        FixtureKind::Label {
            text: "Dodge",
            centered: false,
        },
        Vec2::new(
            space_left + SPACE_KEY_SIZE.x + SPACE_LABEL_GAP_X,
            space_top + LABEL_GAP.y,
        ),
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
        plan(kind, (10, 10), columns, rows, 0).expect("this room kind is furnished")
    }

    #[test]
    fn a_shop_has_a_keeper_at_a_counter_three_tables_two_pots_and_a_slot_machine() {
        let shop = furnish(RoomKind::Shop, 20, 16);
        assert_eq!(count(&shop, |k| *k == FixtureKind::Shopkeeper), 1);
        assert_eq!(count(&shop, |k| *k == FixtureKind::Counter), 1);
        assert_eq!(count(&shop, |k| matches!(k, FixtureKind::Shelf { .. })), 2);
        assert_eq!(count(&shop, |k| *k == FixtureKind::Table), 3);
        assert_eq!(count(&shop, |k| *k == FixtureKind::Lamp), 2);
        assert_eq!(count(&shop, |k| matches!(k, FixtureKind::Goods { .. })), 2);
        assert_eq!(count(&shop, |k| *k == FixtureKind::Pot), 2);
        assert_eq!(count(&shop, |k| *k == FixtureKind::SlotMachine), 1);
    }

    #[test]
    fn the_counter_stands_in_front_of_the_keeper() {
        let shop = furnish(RoomKind::Shop, 20, 16);
        let bottom = |wanted: FixtureKind| {
            shop.fixtures
                .iter()
                .find(|f| f.kind == wanted)
                .map(|f| f.top_left.y + f.size.y)
                .expect("furnished")
        };
        assert!(bottom(FixtureKind::Counter) > bottom(FixtureKind::Shopkeeper));
    }

    #[test]
    fn treasure_has_four_torches_two_bricks_and_a_chest_on_a_carpet() {
        let treasure = furnish(RoomKind::Treasure, 20, 16);
        let torches = |right: bool| count(&treasure, |k| *k == FixtureKind::Torch { right });
        assert_eq!((torches(false), torches(true)), (2, 2));
        assert_eq!(count(&treasure, |k| matches!(k, FixtureKind::Bricks { .. })), 2);
        assert_eq!(count(&treasure, |k| *k == FixtureKind::Chest), 1);
        let carpet = treasure.carpet.expect("treasure has a carpet");
        assert_eq!(
            (carpet.origin, carpet.columns, carpet.rows),
            ((17, 15), 6, 6)
        );
    }

    #[test]
    fn the_treasure_bricks_take_the_floors_own_row_of_the_sheet() {
        use super::super::Theme;
        let themes = [
            Theme::Stone,
            Theme::Sewer,
            Theme::Moss,
            Theme::Spider,
            Theme::Ice,
            Theme::Lava,
            Theme::Temple,
        ];
        let mut rows = Vec::new();
        for theme in themes {
            let treasure =
                plan(RoomKind::Treasure, (10, 10), 20, 16, theme.style()).expect("furnished");
            for fixture in &treasure.fixtures {
                if let FixtureKind::Bricks { style } = fixture.kind {
                    assert_eq!(style, theme.style(), "{theme:?}");
                    assert!(style < crate::props::BRICK_STYLES, "{theme:?} has no row of bricks");
                    rows.push(style);
                }
            }
        }
        rows.dedup();
        assert_eq!(rows.len(), themes.len(), "every floor's bricks look its own");
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
    fn the_start_room_teaches_ten_keys_including_the_dodge_space_bar() {
        let room = furnish(RoomKind::Start, 20, 16);
        assert_eq!(count(&room, |k| matches!(k, FixtureKind::Key(_))), 10);
        assert_eq!(count(&room, |k| matches!(k, FixtureKind::Label { .. })), 7);
        assert_eq!(count(&room, |k| *k == FixtureKind::Key(Key::Space)), 1);
    }

    #[test]
    fn plain_rooms_are_not_furnished() {
        assert!(plan(RoomKind::Normal, (0, 0), 20, 16, 0).is_none());
        assert!(plan(RoomKind::Boss, (0, 0), 20, 16, 0).is_none());
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
        let shop = plan(RoomKind::Shop, (0, 0), 20, 16, 0).expect("shop is furnished");
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
        let carpet = plan(RoomKind::Shop, (0, 0), 20, 16, 0)
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
        // The blob tile for a tile whose neighbours at these offsets are carpet.
        let blob = |with: &[(isize, isize)]| super::super::surface::blob_index(|dx, dy| with.contains(&(dx, dy)));
        assert_eq!(number(2, 4), Some(blob(&[(1, 0), (0, 1), (1, 1)])));
        assert_eq!(number(17, 4), Some(blob(&[(-1, 0), (0, 1), (-1, 1)])));
        assert_eq!(number(2, 11), Some(blob(&[(1, 0), (0, -1), (1, -1)])));
        assert_eq!(number(17, 11), Some(blob(&[(-1, 0), (0, -1), (-1, -1)])));
        let all: Vec<(isize, isize)> = (-1..=1).flat_map(|dy| (-1..=1).map(move |dx| (dx, dy))).collect();
        assert_eq!(number(8, 8), Some(blob(&all)));
        let top: Vec<(isize, isize)> = all.iter().copied().filter(|&(_, dy)| dy >= 0).collect();
        assert_eq!(number(8, 4), Some(blob(&top)));
        assert_eq!(tiles.len(), 16 * 8);
    }
}
