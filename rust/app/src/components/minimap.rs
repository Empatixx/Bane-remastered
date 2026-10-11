use glam::Vec2;

use crate::dungeon::{Direction, RoomKind};
use crate::render::TextureId;

#[derive(Debug, Clone)]
pub(crate) struct MiniRoom {
    pub(crate) cell: (i32, i32),
    pub(crate) kind: RoomKind,
    pub(crate) links: Vec<Direction>,
    pub(crate) visited: bool,
    pub(crate) seen: bool,
    pub(crate) distance: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MapSize {
    Small,
    Big,
}

#[derive(Debug, Clone)]
pub(crate) struct Minimap {
    pub(crate) rooms: Vec<MiniRoom>,
    pub(crate) current: usize,
    pub(crate) shown: Vec2,
    pub(crate) glow: f32,
    pub(crate) icons: TextureId,
    pub(crate) size: MapSize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BigMap;
