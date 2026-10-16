use crate::assets::{Tex, tex};
use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Prop {
    Portal,
    WeaponRack,
    Torch,
    Bones,
    Cobweb,
    Armorstand,
    TaskBoard,
    Pot,
    CraftingTable,
    Box,
    Chains,
    Rock,
    Bookshelf,
    MapTable,
    Scientist,
    Banner,
    Spike,
    TorchOff,
    Shopkeeper,
    ShopTable,
    SlotMachine,
    ShopCounter,
    ShopShelf,
    ShopGoods,
    ShopLamp,
    Chest,
    Bricks,
    AngelStatue,
    Candlestick,
    Pentagram,
    LayingBook,
    Ladder,
    Flamethrower,
    IcePatch,
    WebPatch,
    ArrowTrap,
    SurfaceShade,
    BogShore,
    BogSurface,
    Mushrooms,
    IceShore,
    LavaShore,
    WaterShore,
    IceSurface,
    LavaSurface,
    WaterSurface,
    Icicles,
    IceCrystals,
    FrozenSkull,
    Cocoon,
    TrainingDummy,
    MossDrape,
    MossyBanner,
}

impl Prop {
    /// Props that stand up from the floor, so someone can walk in front of or behind them.
    /// The rest lie flat on the floor or hang on a wall.
    pub(crate) fn stands(self) -> bool {
        matches!(
            self,
            Self::WeaponRack
                | Self::Armorstand
                | Self::TaskBoard
                | Self::Pot
                | Self::CraftingTable
                | Self::Box
                | Self::Rock
                | Self::Bookshelf
                | Self::MapTable
                | Self::Scientist
                | Self::Shopkeeper
                | Self::ShopTable
                | Self::SlotMachine
                | Self::ShopCounter
                | Self::ShopShelf
                | Self::ShopGoods
                | Self::ShopLamp
                | Self::Chest
                | Self::Bricks
                | Self::AngelStatue
                | Self::Candlestick
                | Self::Portal
                | Self::TrainingDummy
                | Self::Mushrooms
                | Self::IceCrystals
                | Self::Cocoon
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Animated {
    pub(crate) frames: usize,
    pub(crate) frame_time: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PropSpec {
    pub(crate) texture: Tex,
    pub(crate) columns: usize,
    pub(crate) rows: usize,
    pub(crate) frame: Vec2,
    pub(crate) scale: f32,
    pub(crate) animation: Option<Animated>,
}

impl PropSpec {
    pub(crate) fn size(&self) -> Vec2 {
        (self.frame * self.scale).floor()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Look {
    pub(crate) row: usize,
    pub(crate) flipped: bool,
}

impl Look {
    pub(crate) const DEFAULT: Self = Self {
        row: 0,
        flipped: false,
    };
    pub(crate) const SECOND_ROW: Self = Self {
        row: 1,
        flipped: false,
    };
    pub(crate) const FLIPPED: Self = Self {
        row: 0,
        flipped: true,
    };

    pub(crate) const fn row(row: usize, flipped: bool) -> Self {
        Self { row, flipped }
    }
}

const fn still(texture: Tex, frame: Vec2, scale: f32) -> PropSpec {
    PropSpec {
        texture,
        columns: 1,
        rows: 1,
        frame,
        scale,
        animation: None,
    }
}

const fn variants(texture: Tex, frame: Vec2, scale: f32) -> PropSpec {
    PropSpec {
        texture,
        columns: 1,
        rows: 2,
        frame,
        scale,
        animation: None,
    }
}

pub(crate) const SURFACE_ROWS: usize = 141;
const SHORE_ROWS: usize = 40;

const fn shore(texture: Tex) -> PropSpec {
    PropSpec {
        texture,
        columns: 1,
        rows: SHORE_ROWS,
        frame: Vec2::new(16.0, 16.0),
        scale: 4.0,
        animation: None,
    }
}

const fn surface(texture: Tex, frames: usize) -> PropSpec {
    PropSpec {
        texture,
        columns: frames,
        rows: SURFACE_ROWS,
        frame: Vec2::new(16.0, 16.0),
        scale: 4.0,
        animation: if frames > 1 {
            Some(Animated {
                frames,
                frame_time: 0.25,
            })
        } else {
            None
        },
    }
}

impl Prop {
    pub(crate) fn spec(self) -> PropSpec {
        match self {
            Self::Portal => PropSpec {
                texture: tex::PORTAL,
                columns: 8,
                rows: 1,
                frame: Vec2::new(96.0, 90.0),
                scale: 2.0,
                animation: Some(Animated {
                    frames: 8,
                    frame_time: 0.125,
                }),
            },
            Self::WeaponRack => still(tex::SWORDS, Vec2::new(26.0, 35.0), 3.0),
            Self::Torch => PropSpec {
                texture: tex::TORCH,
                columns: 4,
                rows: 4,
                frame: Vec2::new(32.0, 32.0),
                scale: 2.0,
                animation: Some(Animated {
                    frames: 4,
                    frame_time: 0.15,
                }),
            },
            Self::Bones => variants(tex::BONES, Vec2::new(16.0, 16.0), 4.0),
            Self::Cobweb => variants(tex::COBWEB, Vec2::new(16.0, 16.0), 4.0),
            Self::Armorstand => still(tex::ARMORSTAND, Vec2::new(17.0, 32.0), 4.0),
            Self::TaskBoard => still(tex::TASKBOARD, Vec2::new(90.0, 60.0), 3.0),
            Self::Pot => PropSpec {
                texture: tex::POT,
                columns: 4,
                rows: 2,
                frame: Vec2::new(64.0, 64.0),
                scale: 3.0,
                animation: None,
            },
            Self::CraftingTable => still(tex::CRAFTINGTABLE, Vec2::new(96.0, 48.0), 3.0),
            Self::Box => still(tex::BOXES, Vec2::new(30.0, 30.0), 3.0),
            Self::Chains => still(tex::CHAINS, Vec2::new(16.0, 32.0), 4.0),
            Self::Rock => variants(tex::ROCK, Vec2::new(16.0, 16.0), 4.0),
            Self::Bookshelf => still(tex::BOOKSHELF, Vec2::new(48.0, 43.0), 4.0),
            Self::MapTable => still(tex::MAPTABLE, Vec2::new(59.0, 38.0), 3.0),
            Self::Banner => PropSpec {
                texture: tex::BANNER,
                columns: 4,
                rows: 1,
                frame: Vec2::new(32.0, 32.0),
                scale: 2.0,
                animation: Some(Animated {
                    frames: 4,
                    frame_time: 0.15,
                }),
            },
            Self::MossyBanner => PropSpec {
                texture: tex::BANNER_MOSSY,
                columns: 4,
                rows: 1,
                frame: Vec2::new(32.0, 32.0),
                scale: 2.0,
                animation: Some(Animated {
                    frames: 4,
                    frame_time: 0.15,
                }),
            },
            Self::Spike => PropSpec {
                texture: tex::SPIKE,
                columns: 4,
                rows: 1,
                frame: Vec2::new(16.0, 16.0),
                scale: 4.0,
                animation: Some(Animated {
                    frames: 4,
                    frame_time: 0.175,
                }),
            },
            Self::TorchOff => PropSpec {
                texture: tex::TORCH,
                columns: 4,
                rows: 4,
                frame: Vec2::new(32.0, 32.0),
                scale: 2.0,
                animation: None,
            },
            Self::Shopkeeper => PropSpec {
                texture: tex::SHOPKEEPER,
                columns: 10,
                rows: 1,
                frame: Vec2::new(80.0, 90.0),
                scale: 4.0,
                animation: Some(Animated {
                    frames: 10,
                    frame_time: 0.17,
                }),
            },
            Self::ShopTable => still(tex::SHOPTABLE, Vec2::new(24.0, 16.0), 6.0),
            Self::SlotMachine => PropSpec {
                texture: tex::SLOTMACHINE,
                columns: crate::sheets::slot::COLUMNS,
                rows: crate::sheets::slot::ROWS,
                frame: Vec2::new(48.0, 48.0),
                scale: 4.0,
                animation: None,
            },
            Self::ShopCounter => still(tex::SHOP_COUNTER, Vec2::new(80.0, 28.0), 4.0),
            Self::ShopShelf => variants(tex::SHOP_SHELF, Vec2::new(32.0, 48.0), 4.0),
            Self::ShopGoods => variants(tex::SHOP_GOODS, Vec2::new(32.0, 32.0), 4.0),
            Self::ShopLamp => PropSpec {
                texture: tex::SHOP_LAMP,
                columns: 4,
                rows: 1,
                frame: Vec2::new(16.0, 32.0),
                scale: 4.0,
                animation: Some(Animated {
                    frames: 4,
                    frame_time: 0.14,
                }),
            },
            Self::Chest => PropSpec {
                texture: tex::CHEST,
                columns: 4,
                rows: 2,
                frame: Vec2::new(16.0, 16.0),
                scale: 4.0,
                animation: Some(Animated {
                    frames: 4,
                    frame_time: 0.175,
                }),
            },
            Self::Bricks => variants(tex::BRICKS, Vec2::new(16.0, 16.0), 4.0),
            Self::AngelStatue => still(tex::ANGEL_STATUE, Vec2::new(64.0, 64.0), 4.0),
            Self::Candlestick => PropSpec {
                texture: tex::CANDLESTICK,
                columns: 4,
                rows: 2,
                frame: Vec2::new(16.0, 16.0),
                scale: 3.0,
                animation: Some(Animated {
                    frames: 4,
                    frame_time: 0.1,
                }),
            },
            Self::Pentagram => still(tex::PENTAGRAM, Vec2::new(150.0, 80.0), 1.5),
            Self::LayingBook => still(tex::LAYING_BOOK, Vec2::new(16.0, 16.0), 4.0),
            Self::Flamethrower => PropSpec {
                texture: tex::FLAMETHROWER,
                columns: 4,
                rows: 1,
                frame: Vec2::new(16.0, 32.0),
                scale: 4.0,
                animation: Some(Animated {
                    frames: 4,
                    frame_time: 0.175,
                }),
            },
            Self::SurfaceShade => still(tex::SURFACE_SHADE, Vec2::new(16.0, 16.0), 4.0),
            Self::BogShore => shore(tex::SHORE_BOG),
            Self::BogSurface => surface(tex::SURFACE_BOG, 1),
            Self::Mushrooms => variants(tex::MUSHROOMS, Vec2::new(16.0, 16.0), 4.0),
            Self::IceShore => shore(tex::SHORE_ICE),
            Self::LavaShore => shore(tex::SHORE_LAVA),
            Self::WaterShore => shore(tex::SHORE_WATER),
            Self::IceSurface => surface(tex::SURFACE_ICE, 1),
            Self::LavaSurface => surface(tex::SURFACE_LAVA, 4),
            Self::WaterSurface => surface(tex::SURFACE_WATER, 8),
            Self::Icicles => PropSpec {
                texture: tex::ICICLES,
                columns: 1,
                rows: 3,
                frame: Vec2::new(16.0, 16.0),
                scale: 4.0,
                animation: None,
            },
            Self::IceCrystals => variants(tex::ICE_CRYSTALS, Vec2::new(16.0, 16.0), 4.0),
            Self::FrozenSkull => still(tex::FROZEN_SKULL, Vec2::new(16.0, 16.0), 4.0),
            Self::Cocoon => variants(tex::COCOON, Vec2::new(16.0, 16.0), 4.0),
            Self::TrainingDummy => still(tex::TRAINING_DUMMY, Vec2::new(24.0, 32.0), 3.0),
            Self::MossDrape => PropSpec {
                texture: tex::MOSS_DRAPE,
                columns: 1,
                rows: 3,
                frame: Vec2::new(16.0, 16.0),
                scale: 4.0,
                animation: None,
            },
            Self::IcePatch => still(tex::ICE_PATCH, Vec2::new(32.0, 32.0), 4.0),
            Self::WebPatch => still(tex::WEB_PATCH, Vec2::new(32.0, 32.0), 4.0),
            Self::ArrowTrap => still(tex::ARROW_TRAP, Vec2::new(16.0, 16.0), 4.0),
            Self::Ladder => still(tex::LADDER, Vec2::new(32.0, 32.0), 3.0),
            Self::Scientist => PropSpec {
                texture: tex::SCIENTIST,
                columns: 6,
                rows: 1,
                frame: Vec2::new(16.0, 40.0),
                scale: 2.5,
                animation: Some(Animated {
                    frames: 6,
                    frame_time: 0.1,
                }),
            },
        }
    }
}
