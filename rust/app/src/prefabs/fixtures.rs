use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::artefacts::ArtefactKind;
use crate::assets::Textures;
use crate::assets::{Tex, tex};
use crate::components::{
    Animation, ArtefactDrop, Backdrop, BodyOffset, BreakKind, Breakable, Candle, Chatty, Chest,
    ChestKind, Clip, Color, ContactDamage, Despawn, Drag, Flame, Fragile, Gate, GunDrop, Health,
    Interactable, Interaction, Layer, Light, Lit, Pickup, Position, Prompt, Size, SlotMachine,
    SlotPrice, Solid, Sprite, Text, ThenPlay, Trap, Velocity, Weapon,
};
use crate::dungeon::{Decor, DecorKind, Direction, FixtureKind, Furnishing, Mount};
use crate::items::{DESPAWN_SECONDS, ItemKind};
use crate::objects::{EXPLOSION_DAMAGE, OBJECT_HEALTH, OBJECT_SIZE};
use crate::physics::Physics;
use crate::props::{Look, Prop};
use crate::render::{FontSize, TextureId};
use crate::systems::{SLOT_COLUMNS, SLOT_IDLE, SLOT_ROWS, SPIN_COST};
use crate::tilemap::{self, TileGrid, Tileset};
use rapier2d::prelude::ColliderHandle;

use super::{PRICE_COLOR, PRICE_TAG_SIZE, PROMPT_COLOR, label};

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
    if prop == Prop::Candlestick {
        let _ = world.insert_one(
            entity,
            Light::rgb(Vec2::new(0.0, -24.0), 212.0, 0.0, 0.0, 30.0, 2.0),
        );
    }
    if let Some(animated) = spec.animation {
        let clip = Clip {
            row: look.row,
            frames: animated.frames,
            frame_time: animated.frame_time,
            looping: true,
            flipped: look.flipped,
        };
        let animation = Animation::new(spec.columns, spec.rows, clip);
        let animation = if prop == Prop::Shopkeeper {
            animation.ping_pong()
        } else {
            animation
        };
        let _ = world.insert_one(entity, animation);
    }
    entity
}

const POT_COLLIDER_OFFSET: Vec2 = Vec2::new(75.0, 78.0);

const POT_COLLIDER_SIZE: Vec2 = Vec2::new(45.0, 45.0);

const BACKDROP_SIZE: Vec2 = Vec2::new(1000.0, 580.0);

const BACKDROP_COLOR: Vec4 = Vec4::new(37.0 / 255.0, 19.0 / 255.0, 26.0 / 255.0, 1.0);
const FLAME_DAMAGE: i32 = 1;
const FLAME_LAST_FRAME: usize = 3;
const SLOT_PROMPT_GAP: f32 = -10.0;
const DROP_DRAG: f32 = 6.0;
const CARPET_COLUMNS: usize = 4;
const CARPET_ROWS: usize = 4;
const CARPET_LAYER: i32 = -2;
const KEY_FRAMES: usize = 2;
const KEY_FRAME_TIME: f32 = 0.2;

pub(crate) fn pot(
    world: &mut World,
    physics: &mut Physics,
    top_left: Vec2,
    textures: &Textures,
) -> Entity {
    let entity = prop(world, Prop::Pot, top_left, Look::DEFAULT, textures);
    let collider_center = top_left + POT_COLLIDER_OFFSET + POT_COLLIDER_SIZE / 2.0;
    physics.add_static_box(collider_center, POT_COLLIDER_SIZE);
    entity
}

pub(crate) fn furnishing(
    world: &mut World,
    physics: &mut Physics,
    furnishing: &Furnishing,
    textures: &Textures,
) {
    if let Some(carpet) = &furnishing.carpet {
        let texture = textures.get(tex::CARPET_TILEMAP);
        for ((column, row), number) in carpet.tiles() {
            let center = Vec2::new(column as f32 + 0.5, row as f32 + 0.5) * tilemap::TILE_SIZE;
            world.spawn((
                Position(center),
                Size(Vec2::splat(tilemap::TILE_SIZE)),
                Color(Vec4::ONE),
                Sprite::cell(
                    texture,
                    number % CARPET_COLUMNS,
                    number / CARPET_COLUMNS,
                    CARPET_COLUMNS,
                    CARPET_ROWS,
                ),
                Layer(CARPET_LAYER),
                Lit,
            ));
        }
    }
    for fixture in &furnishing.fixtures {
        let top_left = fixture.top_left;
        let mut place = |kind: Prop, look: Look| {
            prop(world, kind, top_left, look, textures);
        };
        match fixture.kind {
            FixtureKind::Shopkeeper => place(Prop::Shopkeeper, Look::DEFAULT),
            FixtureKind::Table => place(Prop::ShopTable, Look::DEFAULT),
            FixtureKind::SlotMachine => {
                slot_machine(world, top_left, textures);
            }
            FixtureKind::Torch { right } => {
                let look = if right { Look::FLIPPED } else { Look::DEFAULT };
                place(Prop::Torch, look);
            }
            FixtureKind::Bricks => place(Prop::Bricks, Look::DEFAULT),
            FixtureKind::Chest => {
                chest(world, ChestKind::Weapon, top_left, textures);
            }
            FixtureKind::AngelStatue => place(Prop::AngelStatue, Look::DEFAULT),
            FixtureKind::Candlestick => {
                let candle = prop(world, Prop::Candlestick, top_left, Look::DEFAULT, textures);
                let _ = world.insert_one(candle, Candle);
            }
            FixtureKind::Pentagram => place(Prop::Pentagram, Look::DEFAULT),
            FixtureKind::Book => place(Prop::LayingBook, Look::DEFAULT),
            FixtureKind::Chains => place(Prop::Chains, Look::DEFAULT),
            FixtureKind::Pot => {
                breakable(world, physics, BreakKind::Pot, top_left, textures);
            }
            FixtureKind::Key(key) => {
                key_hint(world, key.texture(), top_left, fixture.size, textures);
            }
            FixtureKind::Label { text, centered } => {
                label(world, text, top_left, centered);
            }
        }
    }
}

fn key_hint(world: &mut World, texture: Tex, top_left: Vec2, size: Vec2, textures: &Textures) {
    let clip = Clip {
        row: 0,
        frames: KEY_FRAMES,
        frame_time: KEY_FRAME_TIME,
        looping: true,
        flipped: false,
    };
    world.spawn((
        Position(top_left + size / 2.0),
        Size(size),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(texture), 0, 0, KEY_FRAMES, 1),
        Animation::new(KEY_FRAMES, 1, clip),
        Layer(-1),
        Lit,
    ));
}

const SIDE_ROW: usize = 0;
const TOP_ROW: usize = 1;
const DISABLED_SIDE_ROW: usize = 2;
const DISABLED_TOP_ROW: usize = 3;
const SPIKE_DAMAGE: i32 = 1;
const SPIKE_ACTIVE_FRAMES: usize = 2;

pub(crate) fn decor(
    world: &mut World,
    physics: &mut Physics,
    decor: Decor,
    textures: &Textures,
) -> Entity {
    let top_left = Vec2::new(decor.tile.0 as f32, decor.tile.1 as f32) * tilemap::TILE_SIZE;
    let centered = top_left - Vec2::splat(tilemap::TILE_SIZE);
    match decor.kind {
        DecorKind::Pot => breakable(world, physics, BreakKind::Pot, centered, textures),
        DecorKind::Barrel => breakable(world, physics, BreakKind::Barrel, centered, textures),
        DecorKind::ExplosiveBarrel => {
            breakable(world, physics, BreakKind::Explosive, centered, textures)
        }
        DecorKind::PoisonBarrel => breakable(world, physics, BreakKind::Poison, centered, textures),
        DecorKind::FrostBarrel => breakable(world, physics, BreakKind::Frost, centered, textures),
        DecorKind::Flamethrower => flamethrower(world, top_left, textures),
        DecorKind::Torch { mount, lit } => {
            let (prop_kind, row) = match (lit, mount) {
                (true, Mount::Top) => (Prop::Torch, TOP_ROW),
                (true, _) => (Prop::Torch, SIDE_ROW),
                (false, Mount::Top) => (Prop::TorchOff, DISABLED_TOP_ROW),
                (false, _) => (Prop::TorchOff, DISABLED_SIDE_ROW),
            };
            let look = Look::row(row, mount == Mount::FacingRight);
            prop(world, prop_kind, top_left, look, textures)
        }
        DecorKind::Bones { variant } => prop(
            world,
            Prop::Bones,
            top_left,
            Look::row(variant, false),
            textures,
        ),
        DecorKind::Cobweb { variant, left } => prop(
            world,
            Prop::Cobweb,
            top_left,
            Look::row(variant, left),
            textures,
        ),
        DecorKind::Banner => prop(world, Prop::Banner, top_left, Look::DEFAULT, textures),
        DecorKind::Spike => {
            let entity = prop(world, Prop::Spike, top_left, Look::DEFAULT, textures);
            let _ = world.insert(
                entity,
                (
                    ContactDamage(SPIKE_DAMAGE),
                    Trap {
                        from_frame: 0,
                        until_frame: SPIKE_ACTIVE_FRAMES,
                    },
                ),
            );
            entity
        }
    }
}

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
            Chatty::default(),
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

pub(crate) fn task_board(world: &mut World, top_left: Vec2, textures: &Textures) -> Entity {
    let entity = prop(world, Prop::TaskBoard, top_left, Look::DEFAULT, textures);
    let _ = world.insert(
        entity,
        (
            Interactable {
                action: Interaction::Quests,
            },
            Prompt {
                text: "Press E to read the board",
                gap: 30.0,
                color: PROMPT_COLOR,
                size: FontSize::Size22,
            },
        ),
    );
    entity
}

pub(crate) fn weapon_table(world: &mut World, top_left: Vec2, textures: &Textures) -> Entity {
    let entity = prop(
        world,
        Prop::CraftingTable,
        top_left,
        Look::DEFAULT,
        textures,
    );
    let _ = world.insert(
        entity,
        (
            Interactable {
                action: Interaction::Weapons,
            },
            Prompt {
                text: "Press E to upgrade weapons",
                gap: 30.0,
                color: PROMPT_COLOR,
                size: FontSize::Size22,
            },
        ),
    );
    entity
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
                Layer(-3),
                Lit,
            ));
            if tileset.is_solid(number) {
                solid_cells.push((column, row));
            }
        }
    }
    solid_cells
}

pub(crate) fn ladder(world: &mut World, top_left: Vec2, textures: &Textures) -> Entity {
    let entity = prop(world, Prop::Ladder, top_left, Look::DEFAULT, textures);
    let _ = world.insert(
        entity,
        (
            Interactable {
                action: Interaction::NextFloor,
            },
            Prompt {
                text: "Press E to enter next floor",
                gap: 0.0,
                color: PROMPT_COLOR,
                size: FontSize::Size22,
            },
        ),
    );
    entity
}

pub(crate) fn item(
    world: &mut World,
    kind: ItemKind,
    center: Vec2,
    despawns: bool,
    velocity: Vec2,
    textures: &Textures,
) -> Entity {
    let spec = kind.spec();
    let entity = world.spawn((
        Pickup(kind),
        Position(center),
        Size(spec.size),
        Color(Vec4::ONE),
        Sprite::cell(
            textures.get(spec.texture),
            0,
            spec.clip.row,
            spec.columns,
            spec.rows,
        ),
        Animation::new(spec.columns, spec.rows, spec.clip),
        Layer(0),
        Lit,
    ));
    if despawns {
        let _ = world.insert_one(
            entity,
            Despawn {
                remaining: DESPAWN_SECONDS,
            },
        );
    }
    if velocity != Vec2::ZERO {
        let _ = world.insert(entity, (Velocity(velocity), Drag { rate: DROP_DRAG }));
    }
    entity
}

pub(crate) fn breakable(
    world: &mut World,
    physics: &mut Physics,
    kind: BreakKind,
    top_left: Vec2,
    textures: &Textures,
) -> Entity {
    let clip = Clip {
        row: 0,
        frames: 1,
        frame_time: 1.0,
        looping: true,
        flipped: false,
    };
    let entity = world.spawn((
        Breakable { kind },
        Health {
            current: OBJECT_HEALTH,
            max: OBJECT_HEALTH,
        },
        Position(top_left + OBJECT_SIZE / 2.0),
        Size(OBJECT_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(kind.texture()), 0, 0, 4, 2),
        Animation::new(4, 2, clip),
        Layer(-1),
        Lit,
    ));
    let collider = kind.collider();
    let collider_center = top_left + collider.offset + collider.size / 2.0;
    let body = physics.add_object_body(entity, collider_center, collider.size);
    let _ = world.insert(
        entity,
        (
            Solid(body.collider),
            body,
            BodyOffset(top_left + OBJECT_SIZE / 2.0 - collider_center),
        ),
    );
    entity
}

pub(crate) fn flamethrower(world: &mut World, top_left: Vec2, textures: &Textures) -> Entity {
    let entity = prop(world, Prop::Flamethrower, top_left, Look::DEFAULT, textures);
    let _ = world.insert(
        entity,
        (
            Flame,
            ContactDamage(FLAME_DAMAGE),
            Trap {
                from_frame: FLAME_LAST_FRAME,
                until_frame: FLAME_LAST_FRAME + 1,
            },
            Light::rgb(Vec2::ZERO, 255.0, 0.0, 0.0, 50.0, 0.0),
        ),
    );
    entity
}

pub(crate) fn slot_machine(world: &mut World, top_left: Vec2, textures: &Textures) -> Entity {
    let entity = prop(world, Prop::SlotMachine, top_left, Look::LOSE_ROW, textures);
    let size = Prop::SlotMachine.spec().size();
    let _ = world.insert(
        entity,
        (
            SlotMachine { spinning: None },
            Animation::new(SLOT_COLUMNS, SLOT_ROWS, SLOT_IDLE),
            Interactable {
                action: Interaction::Spin,
            },
            Prompt {
                text: "Press E to spin",
                gap: SLOT_PROMPT_GAP,
                color: PROMPT_COLOR,
                size: FontSize::Size22,
            },
        ),
    );
    let tag = Vec2::splat(PRICE_TAG_SIZE);
    let center = Vec2::new(top_left.x + size.x / 2.0, top_left.y + tag.y / 2.0);
    world.spawn((
        SlotPrice { owner: entity },
        Position(center),
        Size(tag),
        Color(Vec4::ONE),
        Sprite::whole(textures.get(tex::SHOP_PRICE_HUD)),
        Layer(3),
        Lit,
    ));
    world.spawn((
        SlotPrice { owner: entity },
        Position(center + Vec2::new(0.0, 6.0)),
        Text::centered(SPIN_COST.to_string(), PRICE_COLOR, FontSize::Size22),
        Layer(4),
    ));
    entity
}

pub(crate) fn artefact_drop(
    world: &mut World,
    kind: ArtefactKind,
    center: Vec2,
    velocity: Vec2,
    textures: &Textures,
) -> Entity {
    let entity = world.spawn((
        ArtefactDrop { kind },
        Position(center),
        Size(kind.icon()),
        Color(Vec4::ONE),
        Sprite::whole(textures.get(kind.texture())),
        Layer(0),
        Lit,
        Interactable {
            action: Interaction::PickUpArtefact,
        },
    ));
    if velocity != Vec2::ZERO {
        let _ = world.insert(entity, (Velocity(velocity), Drag { rate: DROP_DRAG }));
    }
    entity
}

pub(crate) fn chest(
    world: &mut World,
    kind: ChestKind,
    top_left: Vec2,
    textures: &Textures,
) -> Entity {
    let entity = prop(world, Prop::Chest, top_left, Look::DEFAULT, textures);
    let _ = world.insert_one(
        entity,
        Chest {
            kind,
            opened: false,
        },
    );
    entity
}

pub(crate) fn gun_drop(
    world: &mut World,
    weapon: Weapon,
    center: Vec2,
    velocity: Vec2,
    textures: &Textures,
) -> Entity {
    let spec = weapon.spec();
    world.spawn((
        GunDrop { weapon },
        Position(center),
        Velocity(velocity),
        Drag { rate: DROP_DRAG },
        Size(spec.icon),
        Color(Vec4::ONE),
        Sprite::whole(textures.get(spec.texture)),
        Layer(0),
        Lit,
        Interactable {
            action: Interaction::PickUpGun,
        },
    ))
}

pub(crate) fn backdrop(world: &mut World) -> Entity {
    world.spawn((
        Backdrop,
        Position(Vec2::ZERO),
        Size(BACKDROP_SIZE),
        Color(BACKDROP_COLOR),
        Sprite::whole(TextureId::WHITE),
        Layer(-10),
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
        textures.get(tex::PATHGATE),
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
        Gate(close),
        ThenPlay(idle),
        Layer(-1),
        Lit,
    ));
    (entity, physics.add_static_box(center, size))
}

pub(crate) fn cracked_wall(
    world: &mut World,
    physics: &mut Physics,
    tiles: [(usize, usize); 2],
    direction: Direction,
    textures: &Textures,
) -> Entity {
    let vertical = matches!(direction, Direction::Left | Direction::Right);
    let center = tiles
        .iter()
        .map(|(column, row)| Vec2::new(*column as f32 + 0.5, *row as f32 + 0.5))
        .sum::<Vec2>()
        / 2.0
        * tilemap::TILE_SIZE;
    let size = if vertical {
        Vec2::new(tilemap::TILE_SIZE, tilemap::TILE_SIZE * 2.0)
    } else {
        Vec2::new(tilemap::TILE_SIZE * 2.0, tilemap::TILE_SIZE)
    };
    let texture = if vertical {
        tex::CRACKED_WALL_V
    } else {
        tex::CRACKED_WALL_H
    };
    let clip = Clip {
        row: 0,
        frames: 1,
        frame_time: 1.0,
        looping: true,
        flipped: false,
    };
    let entity = world.spawn((
        Breakable {
            kind: BreakKind::Cracked,
        },
        Fragile,
        Health {
            current: EXPLOSION_DAMAGE,
            max: EXPLOSION_DAMAGE,
        },
        Position(center),
        Size(size),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(texture), 0, 0, 4, 2),
        Animation::new(4, 2, clip),
        Layer(-1),
        Lit,
    ));
    let handle = physics.add_static_box(center, size);
    let _ = world.insert_one(entity, Solid(handle));
    entity
}
