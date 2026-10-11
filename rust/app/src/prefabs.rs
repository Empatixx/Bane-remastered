use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, PLAYER};
use crate::components::{
    AmmoPip, AmmoText, Animation, Armor, BarKind, Bullet, Button, Chase, Clip, CoinText, Color,
    ContactDamage, Enemy, Expiring, GazeWind, Health, Hitbox, Interactable, Interaction, Layer,
    Light, LightFade, Lit, OnClick, Overlay, Particle, PauseUi, Player, PlayerShadow, Position,
    Prompt, RoomId, ScreenSpace, Size, Sprite, StatusBar, Text, TextButton, ThenPlay, UiAction,
    Velocity, Wallet, Weapon, BULLET_COLUMNS, BULLET_FLIGHT, BULLET_ROWS, PIP_FULL,
};
use crate::dungeon::Direction;
use crate::physics::Physics;
use crate::props::{Look, Prop};
use crate::render::{Align, Anchor, FontSize, Fonts, TextureId};
use crate::systems::{IDLE, PLAYER_COLUMNS, PLAYER_ROWS};
use crate::tilemap::{self, TileGrid, Tileset};
use rapier2d::prelude::ColliderHandle;

pub(crate) const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 76.0);
const PLAYER_HEALTH: i32 = 10;
const PLAYER_ARMOR: i32 = 5;

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
        Health {
            current: PLAYER_HEALTH,
            max: PLAYER_HEALTH,
        },
        Armor {
            current: PLAYER_ARMOR,
            max: PLAYER_ARMOR,
        },
        Wallet { coins: 0 },
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
    let _ = world.insert(
        entity,
        (
            Lit,
            Light::rgb(Vec2::ZERO, 216.0, 156.0, 36.0, 150.0, 0.5),
            PlayerShadow::default(),
            GazeWind::default(),
        ),
    );
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
        Lit,
    ));
    if prop == Prop::Torch {
        let offset = if look.flipped {
            Vec2::new(0.0, -16.0)
        } else {
            Vec2::new(-16.0, -16.0)
        };
        let _ = world.insert_one(entity, Light::rgb(offset, 230.0, 201.0, 114.0, 75.0, 1.25));
    }
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
            Light::rgb(Vec2::ZERO, 128.0, 0.0, 128.0, 50.0, 2.5),
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
) -> (Entity, Entity) {
    let text = world.spawn((
        Text::centered(label, BUTTON_TEXT_COLOR, FontSize::Size27),
        Position(position),
        Layer(2),
    ));
    let button = world.spawn((
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
    ));
    (button, text)
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
            if grid.is_interior(column, row) {
                continue;
            }
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
                Lit,
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
        Lit,
        Light::rgb(Vec2::ZERO, 255.0, 0.0, 0.0, 25.0, 0.75),
        LightFade { base: 0.75 },
    ));
    let body = physics.add_bullet(entity, position, velocity, size);
    let _ = world.insert_one(entity, body);
    entity
}

const RAT_COLUMNS: usize = 4;
const RAT_ROWS: usize = 2;
const RAT_SIZE: Vec2 = Vec2::new(127.5, 55.5);
const RAT_HEALTH: i32 = 12;
const RAT_DAMAGE: i32 = 2;
const RAT_SPEED: f32 = 184.0;
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
            max: RAT_HEALTH,
        },
        ContactDamage(RAT_DAMAGE),
        Chase { speed: RAT_SPEED },
        Position(position),
        Velocity::default(),
        Size(RAT_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(texture, 0, RAT_WALK.row, RAT_COLUMNS, RAT_ROWS),
        Animation::new(RAT_COLUMNS, RAT_ROWS, RAT_WALK),
        Layer(1),
        Lit,
    ));
    let body = physics.add_enemy(entity, position, RAT_SIZE);
    let _ = world.insert_one(entity, body);
    entity
}

pub(crate) fn status_bars(world: &mut World, textures: &Textures) {
    for kind in BarKind::ALL {
        let spec = kind.spec();
        let texture_size = spec_texture_size(kind);
        let size = texture_size * spec.scale;
        world.spawn((
            StatusBar {
                kind,
                fill: 1.0,
                delayed: 1.0,
            },
            Position(spec.position + size / 2.0),
            Size(size),
            Color(Vec4::ONE),
            Sprite::whole(textures.get(spec.texture)),
            Layer(5),
            ScreenSpace,
        ));
    }
}

fn spec_texture_size(kind: BarKind) -> Vec2 {
    match kind {
        BarKind::Health => Vec2::new(76.0, 19.0),
        BarKind::Armor => Vec2::new(83.0, 25.0),
    }
}

const SLOT_POSITION: Vec2 = Vec2::new(685.0, 450.0);
const SLOT_SIZE: Vec2 = Vec2::new(225.0, 75.0);
const GUN_ICON_SIZE: Vec2 = Vec2::new(41.0, 34.0);
const AMMO_TEXT_POSITION: Vec2 = Vec2::new(900.0, 515.0);
const AMMO_TEXT_COLOR: Vec4 = Vec4::new(226.0 / 255.0, 153.0 / 255.0, 117.0 / 255.0, 1.0);
const PIP_START: Vec2 = Vec2::new(922.5, 515.0);
const PIP_STEP: f32 = 15.0;
const PIP_SIZE: Vec2 = Vec2::new(24.0, 24.0);
const COIN_MARGIN: f32 = 10.0;
const COIN_SIZE: Vec2 = Vec2::new(26.0, 26.0);
const COIN_TEXT_GAP: f32 = 15.0;

fn screen_sprite(
    world: &mut World,
    top_left: Vec2,
    size: Vec2,
    texture: TextureId,
    layer: i32,
) -> Entity {
    world.spawn((
        Position(top_left + size / 2.0),
        Size(size),
        Color(Vec4::ONE),
        Sprite::whole(texture),
        Layer(layer),
        ScreenSpace,
    ))
}

pub(crate) fn weapon_hud(world: &mut World, textures: &Textures, clip_size: i32) {
    screen_sprite(
        world,
        SLOT_POSITION,
        SLOT_SIZE,
        textures.get("ingame/weaponslot.tga"),
        4,
    );
    let slot_center = SLOT_POSITION + SLOT_SIZE / 2.0;
    screen_sprite(
        world,
        slot_center - GUN_ICON_SIZE / 2.0,
        GUN_ICON_SIZE,
        textures.get("ingame/guns/pistol.tga"),
        5,
    );
    world.spawn((
        AmmoText,
        Text {
            content: String::new(),
            color: AMMO_TEXT_COLOR,
            size: FontSize::Size22,
            align: Align::Right,
            anchor: Anchor::Baseline,
        },
        Position(AMMO_TEXT_POSITION),
        Layer(6),
        ScreenSpace,
    ));
    for index in 0..clip_size {
        let top_left = Vec2::new(PIP_START.x, PIP_START.y - index as f32 * PIP_STEP);
        world.spawn((
            AmmoPip { index },
            Position(top_left + PIP_SIZE / 2.0),
            Size(PIP_SIZE),
            Color(Vec4::ONE),
            Sprite::cell(textures.get("ingame/ammo_hud.tga"), 0, 0, 2, 2),
            Animation::new(2, 2, PIP_FULL),
            Layer(5),
            ScreenSpace,
        ));
    }
}

pub(crate) fn coin_hud(world: &mut World, textures: &Textures, coin: &str, text_color: Vec4) {
    let top_left = Vec2::new(COIN_MARGIN, 540.0 - COIN_MARGIN - COIN_SIZE.y);
    screen_sprite(world, top_left, COIN_SIZE, textures.get(coin), 5);
    world.spawn((
        CoinText,
        Text {
            content: "0".to_owned(),
            color: text_color,
            size: FontSize::Size22,
            align: Align::Left,
            anchor: Anchor::Middle,
        },
        Position(Vec2::new(
            COIN_MARGIN + COIN_SIZE.x + COIN_TEXT_GAP,
            top_left.y + COIN_SIZE.y / 2.0,
        )),
        Layer(6),
        ScreenSpace,
    ));
}

const PAUSE_BACKGROUND_SIZE: Vec2 = Vec2::new(240.0, 300.0);
const PAUSE_TEXT_COLOR: Vec4 = Vec4::new(222.0 / 255.0, 112.0 / 255.0, 38.0 / 255.0, 1.0);
const PAUSE_HOVER_COLOR: Vec4 = Vec4::new(1.0, 1.0, 0.0, 1.0);
const PAUSE_BUTTONS: [(f32, &str, UiAction); 3] = [
    (225.0, "Resume", UiAction::Resume),
    (275.0, "Statistics", UiAction::Statistics),
    (325.0, "Exit", UiAction::LeaveToMenu),
];

pub(crate) fn pause_ui(world: &mut World, textures: &Textures, fonts: &Fonts) {
    world.spawn((
        PauseUi,
        Overlay,
        Position(Vec2::new(480.0, 270.0)),
        Size(PAUSE_BACKGROUND_SIZE),
        Color(Vec4::ONE),
        Sprite::whole(textures.get("ingame/pausebg.tga")),
        Layer(8),
        ScreenSpace,
    ));
    let line = fonts.get(FontSize::Size22);
    for (baseline, label, action) in PAUSE_BUTTONS {
        let width = line.width(label);
        let height = line.line_height();
        world.spawn((
            PauseUi,
            Overlay,
            OnClick(action),
            TextButton {
                idle: PAUSE_TEXT_COLOR,
                hover: PAUSE_HOVER_COLOR,
            },
            Hitbox {
                offset: Vec2::new(-5.0, -height / 2.0),
                size: Vec2::new(width + 10.0, height + 20.0),
            },
            Text {
                content: label.to_owned(),
                color: PAUSE_TEXT_COLOR,
                size: FontSize::Size22,
                align: Align::Center,
                anchor: Anchor::Baseline,
            },
            Position(Vec2::new(480.0, baseline)),
            Layer(9),
            ScreenSpace,
        ));
    }
}

const GAZE_FRAMES: usize = 3;
const GAZE_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const GAZE_PUFF: Clip = Clip {
    row: 0,
    frames: GAZE_FRAMES,
    frame_time: 0.1,
    looping: false,
    flipped: false,
};

pub(crate) fn gaze_particle(world: &mut World, center: Vec2, texture: TextureId) -> Entity {
    world.spawn((
        Particle,
        Expiring,
        Position(center),
        Size(GAZE_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(texture, 0, 0, GAZE_FRAMES, 1),
        Animation::new(GAZE_FRAMES, 1, GAZE_PUFF),
        Layer(0),
        Lit,
    ))
}

const GATE_COLUMNS: usize = 6;
const GATE_ROWS: usize = 4;
const GATE_FRAME_TIME: f32 = 0.075;

const fn gate_clip(row: usize, frames: usize, looping: bool, flipped: bool) -> Clip {
    Clip {
        row,
        frames,
        frame_time: GATE_FRAME_TIME,
        looping,
        flipped,
    }
}

pub(crate) fn gate(
    world: &mut World,
    physics: &mut Physics,
    tile: (usize, usize),
    direction: Direction,
    textures: &Textures,
) -> (Entity, ColliderHandle) {
    let (close_row, idle_row) = match direction {
        Direction::Up | Direction::Down => (0, 1),
        Direction::Left | Direction::Right => (2, 3),
    };
    let flipped = direction == Direction::Left;
    let close = gate_clip(close_row, 4, false, flipped);
    let idle = gate_clip(idle_row, 6, true, flipped);
    let center = Vec2::new(tile.0 as f32 + 0.5, tile.1 as f32 + 0.5) * tilemap::TILE_SIZE;
    let size = Vec2::splat(tilemap::TILE_SIZE);
    let mut sprite = Sprite::cell(
        textures.get("ingame/pathgate.tga"),
        0,
        close_row,
        GATE_COLUMNS,
        GATE_ROWS,
    );
    if flipped {
        sprite.uv = Sprite::flipped(sprite.uv);
    }
    let entity = world.spawn((
        Position(center),
        Size(size),
        Color(Vec4::ONE),
        sprite,
        Animation::new(GATE_COLUMNS, GATE_ROWS, close),
        ThenPlay(idle),
        Layer(-1),
        Lit,
    ));
    (entity, physics.add_static_box(center, size))
}

pub(crate) fn rat_in_room(
    world: &mut World,
    physics: &mut Physics,
    position: Vec2,
    texture: TextureId,
    room: usize,
) -> Entity {
    let entity = rat(world, physics, position, texture);
    let _ = world.insert_one(entity, RoomId(room));
    entity
}
