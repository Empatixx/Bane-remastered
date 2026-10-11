use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::components::{
    Button, Color, Layer, LoadingBar, OnClick, Player, Portal, Position, Size, Sprite, UiAction,
    Velocity,
};
use crate::render::TextureId;

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

pub(crate) fn button(world: &mut World, position: Vec2, idle: Vec4, action: UiAction) -> Entity {
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
        Sprite(texture),
        Layer(layer),
    ))
}
