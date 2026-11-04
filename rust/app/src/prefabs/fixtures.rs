use glam::{Vec2, Vec3, Vec4};
use hecs::{Entity, World};

use crate::artefacts::ArtefactKind;
use crate::assets::Textures;
use crate::assets::{Tex, tex};
use crate::components::{
    YSort,
    Animation, ArrowTrap, ArtefactDrop, Backdrop, BodyOffset, BreakKind, Breakable, Candle, Chatty,
    Chest, ChestKind, Clip, Color, ContactDamage, Despawn, Drag, Flame, FloorPatch, Gate, GunDrop,
    Health, Interactable, Interaction, Layer, Light, Liquid, Lit, PatchKind, Pickup, Position,
    PriceBox, PriceDigits, Prompt, Size, SlotMachine, SlotPrice, Solid, Sprite, Text, ThenPlay,
    TrainingDummy, Trap, Velocity, Weapon,
};
use crate::dungeon::{
    Carpet, Decor, DecorKind, Direction, FixtureKind, Furnishing, Mount, Surface, Surfaces, Theme,
};
use crate::items::{DESPAWN_SECONDS, ItemKind};
use crate::objects::{OBJECT_HEALTH, OBJECT_SIZE};
use crate::physics::Physics;
use crate::props::{Look, Prop};
use crate::render::{FontSize, TextureId};
use crate::objects::SPIN_COST;
use crate::sheets::slot;
use crate::tilemap::{self, TileGrid, Tileset};
use rapier2d::prelude::ColliderHandle;

use super::{PRICE_COLOR, PRICE_TAG_SIZE, PROMPT_COLOR, label};

pub(crate) const PORTAL_LIGHT: Vec3 = Vec3::new(128.0, 0.0, 128.0);

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
    if prop.stands() {
        let _ = world.insert_one(entity, YSort { foot: prop.foot() });
    }
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
        let _ = world.insert_one(entity, Animation::new(spec.columns, spec.rows, clip));
    }
    entity
}

const BACKDROP_SIZE: Vec2 = Vec2::new(1000.0, 580.0);

const BACKDROP_COLOR: Vec4 = Vec4::new(37.0 / 255.0, 19.0 / 255.0, 26.0 / 255.0, 1.0);
const FLAME_DAMAGE: i32 = 1;
const FLAME_LAST_FRAME: usize = 3;
const SLOT_PROMPT_GAP: f32 = -10.0;
const DROP_DRAG: f32 = 6.0;
/// Each look of carpet is a block of the 47 blob tiles (tools/make_carpets.py), eight across
/// and six down, one look under another.
const CARPET_COLUMNS: usize = 8;
const CARPET_ROWS: usize = 6;
const CARPET_STYLES: usize = 7;
const CARPET_LAYER: i32 = -2;
/// The floor's tiles, under everything.
const FLOOR_TILE_LAYER: i32 = -4;
/// Pools of liquid on the floor and their shores: above the tiles, under what floats on them.
const LIQUID_LAYER: i32 = -3;
const KEY_FRAMES: usize = 2;
const KEY_FRAME_TIME: f32 = 0.2;

pub(crate) fn pot(
    world: &mut World,
    physics: &mut Physics,
    top_left: Vec2,
    textures: &Textures,
) -> Entity {
    solid_prop(world, physics, Prop::Pot, top_left, Look::DEFAULT, textures)
}

/// A prop solid only where it stands on the floor (see [`Prop::base`]): the hero walks up behind
/// its top. A prop without a base is not solid at all.
pub(crate) fn solid_prop(
    world: &mut World,
    physics: &mut Physics,
    kind: Prop,
    top_left: Vec2,
    look: Look,
    textures: &Textures,
) -> Entity {
    let entity = prop(world, kind, top_left, look, textures);
    if let Some((middle, size)) = kind.footprint(look.flipped) {
        physics.add_static_box(top_left + middle, size);
    }
    entity
}

/// Big enough that no single hit can empty it; the dummy refills it every tick.
pub(crate) const DUMMY_HEALTH: i32 = 1_000_000;
/// What of the dummy bullets hit: its straw body, from the picture's top-left corner. Only its
/// base on the floor stops the hero (see [`Prop::base`]).
const DUMMY_FIGURE_OFFSET: Vec2 = Vec2::new(15.0, 9.0);
const DUMMY_FIGURE_SIZE: Vec2 = Vec2::new(42.0, 57.0);
const DUMMY_READOUT_LIFT: f32 = 40.0;
const DUMMY_READOUT_LAYER: i32 = 4;

pub(crate) fn training_dummy(
    world: &mut World,
    physics: &mut Physics,
    top_left: Vec2,
    textures: &Textures,
) -> Entity {
    let size = Prop::TrainingDummy.spec().size();
    let readout = world.spawn((
        Position(top_left + Vec2::new(size.x / 2.0, -DUMMY_READOUT_LIFT)),
        Text::centered(String::new(), PROMPT_COLOR, FontSize::Size22),
        Layer(DUMMY_READOUT_LAYER),
    ));
    let entity = prop(world, Prop::TrainingDummy, top_left, Look::DEFAULT, textures);
    let figure = top_left + DUMMY_FIGURE_OFFSET + DUMMY_FIGURE_SIZE / 2.0;
    let (middle, base) = Prop::TrainingDummy.footprint(false).unwrap_or((figure - top_left, DUMMY_FIGURE_SIZE));
    let body = physics.add_fixed_object(entity, top_left + middle, base);
    physics.add_figure(entity, body, figure - (top_left + middle), DUMMY_FIGURE_SIZE);
    let _ = world.insert(
        entity,
        (
            TrainingDummy {
                readout,
                recent: Vec::new(),
            },
            Health {
                current: DUMMY_HEALTH,
                max: DUMMY_HEALTH,
            },
            Solid(body.collider),
            body,
        ),
    );
    entity
}

/// `gamble_machine` is the talent of that name: without it the shop has no slot machine.
/// A carpet laid over the floor tiles it covers, with its border and corners.
pub(crate) fn carpet(world: &mut World, carpet: &Carpet, textures: &Textures) {
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
                carpet.style * CARPET_ROWS + number / CARPET_COLUMNS,
                CARPET_COLUMNS,
                CARPET_ROWS * CARPET_STYLES,
            ),
            Layer(CARPET_LAYER),
            Lit,
        ));
    }
}

/// The flame of a shop lamp, from the middle of its sprite.
const LAMP_LIGHT_OFFSET: Vec2 = Vec2::new(0.0, -52.0);

pub(crate) fn furnishing(
    world: &mut World,
    physics: &mut Physics,
    furnishing: &Furnishing,
    gamble_machine: bool,
    textures: &Textures,
) {
    if let Some(carpet) = &furnishing.carpet {
        self::carpet(world, carpet, textures);
    }
    for fixture in &furnishing.fixtures {
        let top_left = fixture.top_left;
        let mut place = |kind: Prop, look: Look| {
            prop(world, kind, top_left, look, textures);
        };
        match fixture.kind {
            FixtureKind::Shopkeeper => {
                let keeper = prop(world, Prop::Shopkeeper, top_left, Look::DEFAULT, textures);
                let _ = world.insert_one(keeper, crate::components::Shopkeeper);
            }
            FixtureKind::Table => {
                solid_prop(world, physics, Prop::ShopTable, top_left, Look::DEFAULT, textures);
            }
            FixtureKind::Counter => {
                solid_prop(world, physics, Prop::ShopCounter, top_left, Look::DEFAULT, textures);
            }
            FixtureKind::Goods { barrel } => {
                let look = Look::row(usize::from(barrel), false);
                solid_prop(world, physics, Prop::ShopGoods, top_left, look, textures);
            }
            FixtureKind::Lamp => {
                let lamp = solid_prop(world, physics, Prop::ShopLamp, top_left, Look::DEFAULT, textures);
                let _ = world.insert_one(lamp, Light::rgb(LAMP_LIGHT_OFFSET, 255.0, 196.0, 110.0, 110.0, 1.2));
            }
            FixtureKind::SlotMachine => {
                if gamble_machine {
                    slot_machine(world, top_left, textures);
                    if let Some((middle, size)) = Prop::SlotMachine.footprint(false) {
                        physics.add_static_box(top_left + middle, size);
                    }
                }
            }
            FixtureKind::Torch { right } => {
                let look = if right { Look::FLIPPED } else { Look::DEFAULT };
                place(Prop::Torch, look);
            }
            FixtureKind::Bricks { style } => place(Prop::Bricks, Look::row(style, false)),
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

/// A key drawn on the floor, its two frames blinking, showing what to press there.
pub(crate) fn key_hint(world: &mut World, texture: Tex, top_left: Vec2, size: Vec2, textures: &Textures) -> Entity {
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
    ))
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
        DecorKind::Icicles { variant } => prop(
            world,
            Prop::Icicles,
            top_left,
            Look::row(variant, false),
            textures,
        ),
        DecorKind::IceCrystals { variant } => prop(
            world,
            Prop::IceCrystals,
            top_left,
            Look::row(variant, false),
            textures,
        ),
        DecorKind::FrozenSkull => prop(world, Prop::FrozenSkull, top_left, Look::DEFAULT, textures),
        DecorKind::Mushrooms { variant } => {
            let entity = prop(
                world,
                Prop::Mushrooms,
                top_left,
                Look::row(variant, false),
                textures,
            );
            let _ = world.insert_one(
                entity,
                Light::rgb(Vec2::new(0.0, 8.0), 120.0, 230.0, 150.0, 55.0, 0.8),
            );
            entity
        }
        DecorKind::Cocoon { variant } => prop(
            world,
            Prop::Cocoon,
            top_left,
            Look::row(variant, false),
            textures,
        ),
        DecorKind::Eggs { variant } => prop(
            world,
            Prop::SpiderEggs,
            top_left,
            Look::row(variant, false),
            textures,
        ),
        DecorKind::Wrapped { variant } => prop(
            world,
            Prop::SpiderWrapped,
            top_left,
            Look::row(variant, false),
            textures,
        ),
        DecorKind::FireBowl => {
            let entity = prop(world, Prop::FireBowl, top_left, Look::DEFAULT, textures);
            let _ = world.insert_one(
                entity,
                Light::rgb(Vec2::new(0.0, -20.0), 255.0, 150.0, 60.0, 85.0, 1.1),
            );
            entity
        }
        DecorKind::Shackles { variant } => prop(
            world,
            Prop::Shackles,
            top_left,
            Look::row(variant, false),
            textures,
        ),
        DecorKind::StatuePiece { variant } => prop(
            world,
            Prop::StatuePiece,
            top_left,
            Look::row(variant, false),
            textures,
        ),
        DecorKind::WebPatch => {
            floor_patch(world, Prop::WebPatch, PatchKind::Web, top_left, textures)
        }
        DecorKind::ArrowTrap => {
            let entity = prop(world, Prop::ArrowTrap, top_left, Look::DEFAULT, textures);
            let _ = world.insert_one(
                entity,
                ArrowTrap {
                    timer: ARROW_TRAP_START + (decor.tile.0 % 5) as f32 * 0.35,
                },
            );
            entity
        }
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
        DecorKind::MossyBanner => prop(world, Prop::MossyBanner, top_left, Look::DEFAULT, textures),
        DecorKind::MossDrape { variant } => prop(
            world,
            Prop::MossDrape,
            top_left,
            Look::row(variant, false),
            textures,
        ),
        DecorKind::Spike | DecorKind::OffbeatSpike => {
            let entity = prop(world, Prop::Spike, top_left, Look::DEFAULT, textures);
            if decor.kind == DecorKind::OffbeatSpike {
                if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
                    // Half a cycle on: up while the plain spikes are down.
                    let cycle = animation.clip_frames() as f32 * animation.clip().frame_time;
                    animation.advance(cycle / 2.0);
                }
            }
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
            Light::rgb(
                Vec2::ZERO,
                PORTAL_LIGHT.x,
                PORTAL_LIGHT.y,
                PORTAL_LIGHT.z,
                50.0,
                2.5,
            ),
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

const TRAINING_SIGN_LIFT: f32 = 12.0;

/// The book of lessons on the lobby's desk: reading it goes through the training crypt again.
pub(crate) fn training_book(world: &mut World, top_left: Vec2, textures: &Textures) -> Entity {
    let entity = prop(world, Prop::LayingBook, top_left, Look::DEFAULT, textures);
    let size = Prop::LayingBook.spec().size();
    // A sign over it, so it reads as the way to the training from across the room.
    world.spawn((
        Position(top_left + Vec2::new(size.x / 2.0, -TRAINING_SIGN_LIFT)),
        Text::centered("Training".to_owned(), PROMPT_COLOR, FontSize::Size22),
        Layer(2),
    ));
    let _ = world.insert(
        entity,
        (
            Interactable {
                action: Interaction::Training,
            },
            Prompt {
                text: "Press E to train again",
                gap: 20.0,
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

const AMMO_CRATE_SIZE: Vec2 = Vec2::new(80.0, 80.0);

/// A box of every kind of ammo; the range uses it to fill the gun back up.
pub(crate) fn ammo_crate(world: &mut World, top_left: Vec2, textures: &Textures) -> Entity {
    world.spawn((
        Position(top_left + AMMO_CRATE_SIZE / 2.0),
        Size(AMMO_CRATE_SIZE),
        Color(Vec4::ONE),
        Sprite::whole(textures.get(tex::AMMOBOX_UNIVERSAL)),
        Layer(-1),
        Lit,
        Interactable {
            action: Interaction::RefillAmmo,
        },
        Prompt {
            text: "Press E to refill ammo",
            gap: 10.0,
            color: PROMPT_COLOR,
            size: FontSize::Size22,
        },
    ))
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
            let entity = world.spawn((
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
                Layer(FLOOR_TILE_LAYER),
                Lit,
            ));
            // A wall walked up to from above stands in front of the walker: sorted by its foot
            // like anyone standing, so it hides the feet of whoever is pressed against it.
            if grid.is_front_wall(column, row) {
                let _ = world.insert_one(entity, YSort::bottom(Vec2::splat(tilemap::TILE_SIZE)));
            }
            if tileset.is_solid(number) {
                solid_cells.push((column, row));
            }
        }
    }
    solid_cells
}

pub(crate) fn ladder(
    world: &mut World,
    top_left: Vec2,
    theme: Theme,
    textures: &Textures,
) -> Entity {
    let entity = prop(world, Prop::Ladder, top_left, Look::DEFAULT, textures);
    let _ = world.insert_one(
        entity,
        Sprite::cell(textures.get(theme.ladder_texture()), 0, 0, 1, 1),
    );
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
    let collider = kind.collider();
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
        // Drawn in order by the bottom of its base, where it meets the floor.
        YSort {
            foot: collider.offset.y + collider.size.y - OBJECT_SIZE.y / 2.0,
        },
    ));
    let collider_center = top_left + collider.offset + collider.size / 2.0;
    let body = physics.add_object_body(entity, collider_center, collider.size);
    let figure = kind.figure();
    let figure_center = top_left + figure.offset + figure.size / 2.0;
    physics.add_figure(entity, body, figure_center - collider_center, figure.size);
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

const PATCH_HITBOX: f32 = 96.0;
const ARROW_TRAP_START: f32 = 0.8;

fn patch_hitbox(world: &mut World, visual: Entity, side: f32) -> Entity {
    let center = world
        .get::<&Position>(visual)
        .map_or(Vec2::ZERO, |position| position.0);
    world.spawn((Position(center), Size(Vec2::splat(side))))
}

fn floor_patch(
    world: &mut World,
    patch: Prop,
    kind: PatchKind,
    top_left: Vec2,
    textures: &Textures,
) -> Entity {
    let entity = prop(world, patch, top_left, Look::DEFAULT, textures);
    let _ = world.insert_one(entity, Layer(-2));
    let hitbox = patch_hitbox(world, entity, PATCH_HITBOX);
    let _ = world.insert_one(hitbox, FloorPatch(kind));
    entity
}

pub(crate) fn surface_tile(
    world: &mut World,
    grid: &TileGrid,
    surfaces: &Surfaces,
    tile: (usize, usize),
    surface: Surface,
    textures: &Textures,
) -> Entity {
    let prop_kind = match surface {
        Surface::Ice => Prop::IceSurface,
        Surface::Lava => Prop::LavaSurface,
        Surface::Water => Prop::WaterSurface,
        Surface::Bog => Prop::BogSurface,
        Surface::Sand => Prop::SandSurface,
        Surface::Blood => Prop::BloodSurface,
    };
    let row = surfaces.liquid_row(grid, tile);
    let top_left = Vec2::new(tile.0 as f32, tile.1 as f32) * tilemap::TILE_SIZE;
    let entity = prop(world, prop_kind, top_left, Look::row(row, false), textures);
    let _ = world.insert(
        entity,
        (
            Layer(LIQUID_LAYER),
            Liquid {
                kind: match surface {
                    Surface::Water => 1.0,
                    Surface::Ice => 2.0,
                    Surface::Lava => 3.0,
                    Surface::Bog => 4.0,
                    Surface::Sand => 5.0,
                    Surface::Blood => 6.0,
                },
                time: 0.0,
            },
        ),
    );
    if tile.1 > 0 && grid.is_wall(tile.0 as isize, tile.1 as isize - 1) {
        let shade = prop(world, Prop::SurfaceShade, top_left, Look::DEFAULT, textures);
        let _ = world.insert_one(shade, Layer(LIQUID_LAYER + 1));
    }
    if surface == Surface::Lava && (tile.0 + tile.1 * 3).is_multiple_of(4) {
        let _ = world.insert_one(
            entity,
            Light::rgb(Vec2::ZERO, 255.0, 110.0, 20.0, 80.0, 0.9),
        );
    }
    entity
}

pub(crate) fn shore_tile(
    world: &mut World,
    surface: Surface,
    tile: (usize, usize),
    row: usize,
    textures: &Textures,
) -> Entity {
    let prop_kind = match surface {
        Surface::Ice => Prop::IceShore,
        Surface::Lava => Prop::LavaShore,
        Surface::Water => Prop::WaterShore,
        Surface::Bog => Prop::BogShore,
        Surface::Sand => Prop::SandShore,
        Surface::Blood => Prop::BloodShore,
    };
    let top_left = Vec2::new(tile.0 as f32, tile.1 as f32) * tilemap::TILE_SIZE;
    let entity = prop(world, prop_kind, top_left, Look::row(row, false), textures);
    let _ = world.insert_one(entity, Layer(LIQUID_LAYER));
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
    let entity = prop(
        world,
        Prop::SlotMachine,
        top_left,
        Look::row(slot::IDLE.row, false),
        textures,
    );
    let size = Prop::SlotMachine.spec().size();
    let _ = world.insert(
        entity,
        (
            SlotMachine { spinning: None },
            Animation::new(slot::COLUMNS, slot::ROWS, slot::IDLE),
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
        PriceBox(entity),
        Position(center),
        Size(tag),
        Color(Vec4::ONE),
        Sprite::whole(textures.get(tex::SHOP_PRICE_HUD)),
        Layer(3),
        Lit,
    ));
    world.spawn((
        SlotPrice { owner: entity },
        PriceDigits(entity),
        Position(center),
        Text::inked(SPIN_COST.to_string(), PRICE_COLOR, FontSize::Size22),
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
    backdrop_colored(world, BACKDROP_COLOR)
}

pub(crate) fn backdrop_colored(world: &mut World, color: Vec4) -> Entity {
    world.spawn((
        Backdrop,
        Position(Vec2::ZERO),
        Size(BACKDROP_SIZE),
        Color(color),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::Fixture;

    fn machines_in_a_shop(gamble_machine: bool) -> usize {
        let mut world = World::new();
        let mut physics = Physics::new();
        let shop = Furnishing {
            carpet: None,
            fixtures: vec![Fixture {
                kind: FixtureKind::SlotMachine,
                top_left: Vec2::ZERO,
                size: Prop::SlotMachine.spec().size(),
            }],
        };
        furnishing(
            &mut world,
            &mut physics,
            &shop,
            gamble_machine,
            &Textures::default(),
        );
        world.query_mut::<&SlotMachine>().into_iter().count()
    }

    #[test]
    fn the_slot_machine_stands_in_the_shop_only_with_the_gamble_machine_talent() {
        assert_eq!(machines_in_a_shop(true), 1);
        assert_eq!(machines_in_a_shop(false), 0);
    }
    const SOLID: [Prop; 10] = [
        Prop::Pot,
        Prop::ShopCounter,
        Prop::ShopGoods,
        Prop::ShopLamp,
        Prop::ShopTable,
        Prop::SlotMachine,
        Prop::TrainingDummy,
        Prop::Armorstand,
        Prop::Box,
        Prop::AngelStatue,
    ];

    #[test]
    fn solid_props_are_solid_only_low_at_the_bottom_of_their_picture() {
        for kind in SOLID {
            let size = kind.spec().size();
            let (middle, base) = kind.footprint(false).expect("a base");
            assert!(base.y >= 12.0, "{kind:?} {base}");
            assert!(base.y <= size.y * 0.4, "{kind:?} {base} of {size}");
            assert!(base.x <= size.x, "{kind:?}");
            assert!(middle.y + base.y / 2.0 <= size.y, "{kind:?}");
            // Drawn in order by the bottom of the base.
            assert_eq!(kind.foot(), middle.y + base.y / 2.0 - size.y / 2.0, "{kind:?}");
        }
        let (left, _) = Prop::ShopGoods.footprint(false).expect("a base");
        let (right, _) = Prop::ShopGoods.footprint(true).expect("a base");
        assert_eq!(left.x + right.x, Prop::ShopGoods.spec().size().x);
    }

    /// Where the hero's feet stop walking down onto the solid prop `kind` from well above it.
    fn feet_stop_above(kind: Prop) -> (f32, Vec2, Vec2) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        let top_left = Vec2::new(400.0, 400.0);
        solid_prop(&mut world, &mut physics, kind, top_left, Look::DEFAULT, &textures);
        let size = kind.spec().size();
        let (middle, _) = kind.footprint(false).expect("a base");
        let start = Vec2::new(top_left.x + middle.x, top_left.y - 120.0);
        let player = crate::prefabs::player(&mut world, &mut physics, start, &textures);
        for _ in 0..90 {
            if let Ok(mut velocity) = world.get::<&mut Velocity>(player) {
                velocity.0 = Vec2::new(0.0, 300.0);
            }
            crate::systems::simulate(&mut world, &mut physics, 1.0 / 60.0);
        }
        let at = world.get::<&Position>(player).map(|p| p.0).expect("the hero");
        let feet = crate::prefabs::player_feet(at).y + crate::prefabs::PLAYER_FEET.y / 2.0;
        (feet, top_left, size)
    }

    #[test]
    fn the_hero_walks_in_behind_the_top_of_a_lamp_or_a_table_and_stops_at_its_base() {
        for kind in [Prop::ShopLamp, Prop::ShopTable, Prop::AngelStatue] {
            let (feet, top_left, size) = feet_stop_above(kind);
            let (middle, base) = kind.footprint(false).expect("a base");
            let base_top = top_left.y + middle.y - base.y / 2.0;
            // Past the top half of the picture, behind it ...
            assert!(feet > top_left.y + size.y / 2.0, "{kind:?}: feet {feet}, top {}", top_left.y);
            // ... but not into its base, and so drawn behind it.
            assert!(feet <= base_top + 1.0, "{kind:?}: feet {feet}, base {base_top}");
            assert!(feet < top_left.y + size.y / 2.0 + kind.foot(), "{kind:?}");
        }
    }
}
