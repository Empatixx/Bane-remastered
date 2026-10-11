use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::components::{
    Button, Color, Layer, LoadingBar, OnClick, Player, Portal, Position, Size, Solid, Sprite, Text,
    UiAction, Velocity,
};
use crate::render::TextureId;
use crate::tilemap::{self, TileGrid};

pub(crate) fn player(world: &mut World, position: Vec2) -> Entity {
    world.spawn((
        Player,
        Position(position),
        Velocity::default(),
        Size(Vec2::new(32.0, 32.0)),
        Color(Vec4::new(0.95, 0.85, 0.2, 1.0)),
        Layer(1),
    ))
}

pub(crate) fn block(world: &mut World, position: Vec2, size: Vec2, color: Vec4) -> Entity {
    world.spawn((Position(position), Size(size), Color(color)))
}

pub(crate) fn portal(world: &mut World, position: Vec2) -> Entity {
    world.spawn((
        Portal,
        Position(position),
        Size(Vec2::new(48.0, 72.0)),
        Color(Vec4::new(0.6, 0.3, 0.9, 1.0)),
    ))
}

pub(crate) fn button(
    world: &mut World,
    position: Vec2,
    idle: Vec4,
    label: &str,
    action: UiAction,
) -> Entity {
    label_at(world, position, label, Layer(2));
    world.spawn((
        Button {
            idle,
            hover: (idle * 1.35).min(Vec4::ONE),
        },
        OnClick(action),
        Position(position),
        Size(Vec2::new(240.0, 48.0)),
        Color(idle),
    ))
}

pub(crate) fn loading_bar(world: &mut World, position: Vec2) -> Entity {
    world.spawn((
        LoadingBar,
        Position(position),
        Size(Vec2::new(0.0, 16.0)),
        Color(Vec4::new(0.3, 0.8, 0.5, 1.0)),
    ))
}

pub(crate) fn sprite(
    world: &mut World,
    position: Vec2,
    size: Vec2,
    texture: TextureId,
    layer: i32,
) -> Entity {
    world.spawn((
        Position(position),
        Size(size),
        Color(Vec4::ONE),
        Sprite::whole(texture),
        Layer(layer),
    ))
}

pub(crate) fn label(world: &mut World, position: Vec2, content: &str) -> Entity {
    label_at(world, position, content, Layer(2))
}

fn label_at(world: &mut World, position: Vec2, content: &str, layer: Layer) -> Entity {
    world.spawn((
        Position(position),
        Text {
            content: content.to_owned(),
            color: Vec4::ONE,
        },
        layer,
    ))
}

pub(crate) fn tilemap(world: &mut World, grid: &TileGrid, tileset: TextureId) {
    for row in 0..grid.rows() {
        for column in 0..grid.columns() {
            let number = tilemap::tile_number(grid, column, row);
            let (tile_column, tile_row) = (
                number % tilemap::TILESET_COLUMNS,
                number / tilemap::TILESET_COLUMNS,
            );
            let center = Vec2::new(column as f32 + 0.5, row as f32 + 0.5) * tilemap::TILE_SIZE;
            let entity = world.spawn((
                Position(center),
                Size(Vec2::splat(tilemap::TILE_SIZE)),
                Color(Vec4::ONE),
                Sprite::cell(
                    tileset,
                    tile_column,
                    tile_row,
                    tilemap::TILESET_COLUMNS,
                    tilemap::TILESET_ROWS,
                ),
                Layer(-2),
            ));
            if tile_row > 0 {
                let _ = world.insert_one(entity, Solid);
            }
        }
    }
}
