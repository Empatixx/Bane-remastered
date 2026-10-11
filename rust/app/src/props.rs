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
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Animated {
    pub(crate) frames: usize,
    pub(crate) frame_time: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PropSpec {
    pub(crate) texture: &'static str,
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
}

const fn still(texture: &'static str, frame: Vec2, scale: f32) -> PropSpec {
    PropSpec {
        texture,
        columns: 1,
        rows: 1,
        frame,
        scale,
        animation: None,
    }
}

const fn variants(texture: &'static str, frame: Vec2, scale: f32) -> PropSpec {
    PropSpec {
        texture,
        columns: 1,
        rows: 2,
        frame,
        scale,
        animation: None,
    }
}

impl Prop {
    pub(crate) fn spec(self) -> PropSpec {
        match self {
            Self::Portal => PropSpec {
                texture: "ingame/portal.tga",
                columns: 8,
                rows: 1,
                frame: Vec2::new(96.0, 90.0),
                scale: 2.0,
                animation: Some(Animated {
                    frames: 8,
                    frame_time: 0.125,
                }),
            },
            Self::WeaponRack => still("ingame/swords.tga", Vec2::new(26.0, 35.0), 3.0),
            Self::Torch => PropSpec {
                texture: "ingame/torch.tga",
                columns: 4,
                rows: 4,
                frame: Vec2::new(32.0, 32.0),
                scale: 2.0,
                animation: Some(Animated {
                    frames: 4,
                    frame_time: 0.15,
                }),
            },
            Self::Bones => variants("ingame/bones.tga", Vec2::new(16.0, 16.0), 4.0),
            Self::Cobweb => variants("ingame/cobweb.tga", Vec2::new(16.0, 16.0), 4.0),
            Self::Armorstand => still("ingame/armorstand.tga", Vec2::new(17.0, 32.0), 4.0),
            Self::TaskBoard => still("ingame/taskboard.tga", Vec2::new(90.0, 60.0), 3.0),
            Self::Pot => PropSpec {
                texture: "ingame/pot.tga",
                columns: 4,
                rows: 2,
                frame: Vec2::new(64.0, 64.0),
                scale: 3.0,
                animation: None,
            },
            Self::CraftingTable => still("ingame/craftingtable.tga", Vec2::new(96.0, 48.0), 3.0),
            Self::Box => still("ingame/boxes.tga", Vec2::new(30.0, 30.0), 3.0),
            Self::Chains => still("ingame/chains.tga", Vec2::new(16.0, 32.0), 4.0),
            Self::Rock => variants("ingame/rock.tga", Vec2::new(16.0, 16.0), 4.0),
            Self::Bookshelf => still("ingame/bookshelf.tga", Vec2::new(64.0, 57.0), 3.0),
            Self::MapTable => still("ingame/maptable.tga", Vec2::new(59.0, 38.0), 3.0),
            Self::Scientist => PropSpec {
                texture: "ingame/scientist.tga",
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
