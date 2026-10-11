mod autotile;
mod grid;
mod layouts;

pub(crate) use autotile::{tile_number, TILESET_COLUMNS, TILESET_ROWS};
pub(crate) use grid::TileGrid;
pub(crate) use layouts::{arena, lobby};

pub(crate) const TILE_SIZE: f32 = 64.0;
