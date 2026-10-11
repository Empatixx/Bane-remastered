use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, PLAYER};
use crate::components::{
    Animation, Bullet, Button, Chase, Clip, Color, Enemy, Health, Interactable, Interaction, Layer,
    OnClick, Player, Position, Prompt, Size, Sprite, Text, UiAction, Velocity, Weapon,
    BULLET_COLUMNS, BULLET_FLIGHT, BULLET_ROWS,
};
use crate::physics::Physics;
use crate::props::{Look, Prop};
use crate::render::{FontSize, TextureId};
use crate::systems::{IDLE, PLAYER_COLUMNS, PLAYER_ROWS};
use crate::tilemap::{self, TileGrid, Tileset};

pub(crate) const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 76.0);

pub(crate) fn player(
    world: &mut World,
    physics: &mut Physics,
    position: Vec2,
    textures: &Textures,
) -> Entity {
    let entity = world.spawn((
        Player,
        Position(position),
        Velocity::default(),
        Weapon::pistol(),
        Size(PLAYER_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(
            textures.get(PLAYER),
            0,
            IDLE.row,
            PLAYER_COLUMNS,
            PLAYER_ROWS,
        ),
        Animation::new(PLAYER_COLUMNS, PLAYER_ROWS, IDLE),
        Layer(1),
    ));
    let body = physics.add_player(entity, position, PLAYER_SIZE);
    let _ = world.insert_one(entity, body);
    entity
}

pub(crate) fn prop(
    world: &mut World,
    prop: Prop,
    top_left: Vec2,
    look: Look,
    textures: &Textures,
) -> Entity {
    let spec = prop.spec();
    let size = spec.size();
    let texture = textures.get(spec.texture);
    let mut sprite = Sprite::cell(texture, 0, look.row, spec.columns, spec.rows);
    if look.flipped {
        sprite.uv = Sprite::flipped(sprite.uv);
    }
    let entity = world.spawn((
        Position(top_left + size / 2.0),
        Size(size),
        Color(Vec4::ONE),
        sprite,
        Layer(-1),
    ));
    if let Some(animated) = spec.animation {
        let clip = Clip {
            row: look.row,
            frames: animated.frames,
            frame_time: animated.frame_time,
            looping: true,
            flipped: look.flipped,
        };
        let _ = world.insert_one(entity, Animation::new(spec.columns, spec.rows, clip));
    }
    entity
}

const PROMPT_COLOR: Vec4 = Vec4::new(222.0 / 255.0, 112.0 / 255.0, 38.0 / 255.0, 1.0);

pub(crate) fn portal(world: &mut World, top_left: Vec2, textures: &Textures) -> Entity {
    let entity = prop(world, Prop::Portal, top_left, Look::DEFAULT, textures);
    let _ = world.insert(
        entity,
        (
            Interactable {
                action: Interaction::EnterPortal,
            },
            Prompt {
                text: "Press E to enter the portal",
                gap: 0.0,
                color: PROMPT_COLOR,
                size: FontSize::Size22,
            },
        ),
    );
    entity
}

pub(crate) fn scientist(world: &mut World, top_left: Vec2, textures: &Textures) -> Entity {
    let entity = prop(world, Prop::Scientist, top_left, Look::DEFAULT, textures);
    let _ = world.insert(
        entity,
        (
            Interactable {
                action: Interaction::Upgrade,
            },
            Prompt {
                text: "Press E to upgrade",
                gap: 30.0,
                color: PROMPT_COLOR,
                size: FontSize::Size22,
            },
        ),
    );
    entity
}

const SPINNER_FRAMES: usize = 8;
const SPINNER_SIZE: Vec2 = Vec2::new(68.0, 72.0);
const SPINNER: Clip = Clip {
    row: 0,
    frames: SPINNER_FRAMES,
    frame_time: 0.1,
    looping: true,
    flipped: false,
};

pub(crate) fn spinner(world: &mut World, position: Vec2, texture: TextureId) -> Entity {
    world.spawn((
        Position(position),
        Size(SPINNER_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(texture, 0, 0, SPINNER_FRAMES, 1),
        Animation::new(SPINNER_FRAMES, 1, SPINNER),
        Layer(1),
    ))
}

const BUTTON_SIZE: Vec2 = Vec2::new(126.0, 63.0);
const BUTTON_TEXT_COLOR: Vec4 = Vec4::new(200.0 / 255.0, 92.0 / 255.0, 18.0 / 255.0, 1.0);

pub(crate) fn button(
    world: &mut World,
    position: Vec2,
    label: &str,
    action: UiAction,
    texture: TextureId,
) -> Entity {
    world.spawn((
        Text {
            content: label.to_owned(),
            color: BUTTON_TEXT_COLOR,
            size: FontSize::Size27,
        },
        Position(position),
        Layer(2),
    ));
    world.spawn((
        Button {
            idle_uv: Sprite::cell_uv(0, 0, 1, 2),
            hover_uv: Sprite::cell_uv(0, 1, 1, 2),
        },
        OnClick(action),
        Position(position),
        Size(BUTTON_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(texture, 0, 0, 1, 2),
        Layer(1),
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

pub(crate) fn tilemap(
    world: &mut World,
    grid: &TileGrid,
    tileset: &Tileset,
    texture: TextureId,
) -> Vec<(usize, usize)> {
    let mut solid_cells = Vec::new();
    for row in 0..grid.rows() {
        for column in 0..grid.columns() {
            let number = tileset.tile_number(grid, column, row);
            let center = Vec2::new(column as f32 + 0.5, row as f32 + 0.5) * tilemap::TILE_SIZE;
            world.spawn((
                Position(center),
                Size(Vec2::splat(tilemap::TILE_SIZE)),
                Color(Vec4::ONE),
                Sprite::cell(
                    texture,
                    number % tileset.columns(),
                    number / tileset.columns(),
                    tileset.columns(),
                    tileset.rows(),
                ),
                Layer(-2),
            ));
            if tileset.is_solid(number) {
                solid_cells.push((column, row));
            }
        }
    }
    solid_cells
}

pub(crate) fn bullet(
    world: &mut World,
    physics: &mut Physics,
    position: Vec2,
    velocity: Vec2,
    damage: i32,
    texture: TextureId,
) -> Entity {
    let size = Vec2::splat(16.0);
    let entity = world.spawn((
        Bullet { damage },
        Position(position),
        Velocity(velocity),
        Size(size),
        Color(Vec4::ONE),
        Sprite::cell(texture, 0, BULLET_FLIGHT.row, BULLET_COLUMNS, BULLET_ROWS),
        Animation::new(BULLET_COLUMNS, BULLET_ROWS, BULLET_FLIGHT),
        Layer(2),
    ));
    let body = physics.add_bullet(entity, position, velocity, size);
    let _ = world.insert_one(entity, body);
    entity
}

const RAT_COLUMNS: usize = 4;
const RAT_ROWS: usize = 2;
const RAT_SIZE: Vec2 = Vec2::new(127.5, 55.5);
const RAT_HEALTH: i32 = 12;
const RAT_SPEED: f32 = 110.0;
const RAT_WALK: Clip = Clip {
    row: 0,
    frames: 4,
    frame_time: 0.12,
    looping: true,
    flipped: false,
};
const RAT_DEATH: Clip = Clip {
    row: 1,
    frames: 3,
    frame_time: 0.15,
    looping: false,
    flipped: false,
};

pub(crate) fn rat(
    world: &mut World,
    physics: &mut Physics,
    position: Vec2,
    texture: TextureId,
) -> Entity {
    let entity = world.spawn((
        Enemy { death: RAT_DEATH },
        Health {
            current: RAT_HEALTH,
        },
        Chase { speed: RAT_SPEED },
        Position(position),
        Velocity::default(),
        Size(RAT_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(texture, 0, RAT_WALK.row, RAT_COLUMNS, RAT_ROWS),
        Animation::new(RAT_COLUMNS, RAT_ROWS, RAT_WALK),
        Layer(1),
    ));
    let body = physics.add_enemy(entity, position, RAT_SIZE);
    let _ = world.insert_one(entity, body);
    entity
}
