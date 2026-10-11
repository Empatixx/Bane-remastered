use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, PLAYER};
use crate::components::{
    AmmoPip, AmmoText, Animation, Armageddon, Armor, Backdrop, BarKind, Boss, BossHud, Bullet,
    Button, Candle, Chase, Chest, ChestKind, Clip, Cobweb, CoinText, Color, ContactDamage, Dash,
    DeathBurst, DeathHeal, Despawn, Drag, Enemy, EnemyBullet, Expiring, EyeShooter, Floating, Fuse,
    GazeWind, GunDrop, GunIcon, Hatcher, Health, Hidden, Hitbox, Hits, Homing, Impact,
    Interactable, Interaction, Kin, Layer, Light, LightFade, Lit, Loadout, Minion, NoDrop, OnClick,
    Overlay, Particle, Path, PauseUi, Pickup, Player, PlayerShadow, Position, PriceTag, Prompt,
    RoomId, ScreenSpace, Shop, Size, Smash, Spawning, SpeechBubble, SpeechText, Sprite, StatusBar,
    Text, TextButton, ThenPlay, Trap, UiAction, Velocity, Wallet, Weapon, WebSpitter,
    BULLET_COLUMNS, BULLET_FLIGHT, BULLET_ROWS, PIP_FULL,
};
use crate::dungeon::{Decor, DecorKind, Direction, FixtureKind, Furnishing, Mount};
use crate::enemies::{self, Behavior, EnemyKind, ProjectileKind, Role};
use crate::guns::{MAX_CLIP, PIP_SIZE};
use crate::items::{ItemKind, DESPAWN_SECONDS};
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
        Loadout::starting(),
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

const POT_COLLIDER_OFFSET: Vec2 = Vec2::new(75.0, 78.0);
const POT_COLLIDER_SIZE: Vec2 = Vec2::new(45.0, 45.0);
const BULLET_DAMAGE: i32 = 1;
const BACKDROP_SIZE: Vec2 = Vec2::new(1000.0, 580.0);
const BACKDROP_COLOR: Vec4 = Vec4::new(37.0 / 255.0, 19.0 / 255.0, 26.0 / 255.0, 1.0);
const PRICE_TAG_SIZE: f32 = 32.0;
const PRICE_TAG_LIFT: f32 = 15.0;
const PRICE_COLOR: Vec4 = Vec4::new(212.0 / 255.0, 168.0 / 255.0, 8.0 / 255.0, 1.0);
const SAY_BOX_SIZE: Vec2 = Vec2::new(384.0, 192.0);
const SAY_BOX_SHIFT: f32 = 80.0;
const SAY_BOX_DROP: f32 = 20.0;
const SAY_INSIDE_WIDTH: f32 = 240.0;
const SAY_INSIDE_HEIGHT: f32 = 132.0;
const SAY_COLOR: Vec4 = Vec4::new(200.0 / 255.0, 92.0 / 255.0, 18.0 / 255.0, 1.0);
const DROP_DRAG: f32 = 6.0;
const FLOAT_SECONDS: f32 = 0.5;
const FLOAT_RISE: f32 = 30.0;
const COBWEB_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const COBWEB_FORCE: f32 = 150_000_000.0;
const COBWEB_DRAG: f32 = 1.5;
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
        let texture = textures.get("ingame/carpet_tilemap.tga");
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
            FixtureKind::SlotMachine => place(Prop::SlotMachine, Look::LOSE_ROW),
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
                pot(world, physics, top_left, textures);
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

fn key_hint(world: &mut World, texture: &str, top_left: Vec2, size: Vec2, textures: &Textures) {
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

fn label(world: &mut World, content: &str, position: Vec2, centered: bool) {
    let align = if centered { Align::Center } else { Align::Left };
    world.spawn((
        Position(position),
        Text {
            content: content.to_owned(),
            color: Vec4::ONE,
            size: FontSize::Size22,
            align,
            anchor: Anchor::Baseline,
        },
        Layer(-1),
    ));
}

const SIDE_ROW: usize = 0;
const TOP_ROW: usize = 1;
const DISABLED_SIDE_ROW: usize = 2;
const DISABLED_TOP_ROW: usize = 3;
const SPIKE_DAMAGE: i32 = 1;
const SPIKE_ACTIVE_FRAMES: usize = 2;

pub(crate) fn decor(world: &mut World, decor: Decor, textures: &Textures) -> Entity {
    let top_left = Vec2::new(decor.tile.0 as f32, decor.tile.1 as f32) * tilemap::TILE_SIZE;
    match decor.kind {
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
                        active_frames: SPIKE_ACTIVE_FRAMES,
                    },
                ),
            );
            entity
        }
    }
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

pub(crate) fn enemy(
    world: &mut World,
    kind: EnemyKind,
    position: Vec2,
    floor: u32,
    textures: &Textures,
) -> Entity {
    let spec = kind.spec();
    let size = spec.size();
    let idle = spec.idle_clip();
    let boss = matches!(spec.role, Role::Boss(_));
    let (health, damage) = if boss {
        (spec.health, spec.damage)
    } else {
        (
            enemies::scale_stat(spec.health, enemies::health_multiplier(floor)),
            enemies::scale_stat(spec.damage, enemies::damage_multiplier(floor)),
        )
    };
    let spawn_seconds = if boss {
        enemies::BOSS_SPAWN_SECONDS
    } else {
        enemies::COMMON_SPAWN_SECONDS
    };
    let entity = world.spawn((
        Enemy {
            death: spec.death_clip(),
        },
        Health {
            current: health,
            max: health,
        },
        ContactDamage(damage),
        Chase {
            speed: spec.speed(),
        },
        Path::default(),
        Position(position),
        Velocity::default(),
        Size(size),
        Color(Vec4::new(1.0, 1.0, 1.0, 0.0)),
        Sprite::cell(
            textures.get(spec.texture),
            0,
            idle.row,
            spec.columns,
            spec.rows,
        ),
        Animation::new(spec.columns, spec.rows, idle),
        Layer(1),
        Lit,
        Spawning {
            remaining: spawn_seconds,
            total: spawn_seconds,
        },
    ));
    match spec.behavior {
        Behavior::Plain => {}
        Behavior::Dash => {
            let _ = world.insert_one(
                entity,
                Dash {
                    base_speed: spec.speed(),
                    bonus_speed: enemies::speed_from_force(enemies::DASH_FORCE, size),
                    cycle: 0.0,
                },
            );
        }
        Behavior::Smash(kind) => {
            let _ = world.insert_one(
                entity,
                Smash {
                    kind,
                    count: enemies::SMASH_BULLETS,
                    remaining: 0.0,
                },
            );
        }
        Behavior::SpiderBoss => {
            let _ = world.insert(
                entity,
                (
                    Hatcher {
                        remaining: enemies::HATCH_COOLDOWN,
                        next: 0,
                    },
                    WebSpitter {
                        remaining: enemies::WEB_COOLDOWN,
                        casting: false,
                        idle: spec.idle_clip(),
                    },
                ),
            );
        }
        Behavior::KingSlimeBoss => {
            let _ = world.insert(
                entity,
                (
                    Armageddon {
                        kind: ProjectileKind::Purple,
                        remaining: 0.0,
                        angle: 0.0,
                    },
                    Smash {
                        kind: ProjectileKind::Purple,
                        count: enemies::KING_SMASH_BULLETS,
                        remaining: 0.0,
                    },
                ),
            );
        }
        Behavior::Burst(kind) => {
            let _ = world.insert(entity, (DeathBurst { kind }, Hits::default()));
        }
        Behavior::Shoots(kind) => {
            let _ = world.insert_one(
                entity,
                EyeShooter {
                    kind,
                    idle: spec.idle_clip(),
                    remaining: enemies::SHOOT_COOLDOWN,
                    attacking: false,
                },
            );
        }
        Behavior::HealsKin => {
            let _ = world.insert_one(
                entity,
                DeathHeal {
                    radius: enemies::HEAL_RADIUS,
                    amount: 1,
                },
            );
        }
    }
    if spec.kin {
        let _ = world.insert_one(entity, Kin);
    }
    if boss {
        let _ = world.insert_one(entity, Boss);
    }
    if kind == EnemyKind::KingSlime {
        let _ = world.insert_one(entity, NoDrop);
    }
    entity
}

pub(crate) fn enemy_bullet(
    world: &mut World,
    physics: &mut Physics,
    kind: ProjectileKind,
    position: Vec2,
    direction: Vec2,
    textures: &Textures,
) -> Entity {
    let spec = kind.spec();
    let size = spec.size();
    let velocity = direction.normalize_or_zero() * spec.speed();
    let idle = spec.idle_clip();
    let [red, green, blue, radius, intensity] = spec.light;
    let entity = world.spawn((
        EnemyBullet,
        Impact(spec.impact_clip()),
        ContactDamage(BULLET_DAMAGE),
        Position(position),
        Velocity(velocity),
        Size(size),
        Color(Vec4::ONE),
        Sprite::cell(
            textures.get(spec.texture),
            0,
            idle.row,
            spec.columns,
            spec.rows,
        ),
        Animation::new(spec.columns, spec.rows, idle),
        Layer(2),
        Lit,
        Light::rgb(Vec2::ZERO, red, green, blue, radius, intensity),
        LightFade { base: intensity },
    ));
    if spec.homing {
        let _ = world.insert_one(
            entity,
            Homing {
                aim: position,
                speed: spec.speed(),
            },
        );
    }
    if let Some(seconds) = spec.lifetime {
        let _ = world.insert_one(entity, Fuse { remaining: seconds });
    }
    let body = physics.add_enemy_bullet(entity, position, velocity, size);
    let _ = world.insert_one(entity, body);
    entity
}

pub(crate) fn hatchling(
    world: &mut World,
    physics: &mut Physics,
    kind: EnemyKind,
    position: Vec2,
    room: Option<RoomId>,
    textures: &Textures,
) -> Entity {
    let entity = enemy(world, kind, position, 0, textures);
    crate::systems::finish_spawning(world, physics, entity);
    let _ = world.insert(entity, (Minion, NoDrop));
    if let Some(room) = room {
        let _ = world.insert_one(entity, room);
    }
    entity
}

pub(crate) fn cobweb(
    world: &mut World,
    position: Vec2,
    direction: Vec2,
    textures: &Textures,
) -> Entity {
    let size = COBWEB_SIZE;
    let speed = COBWEB_FORCE / (size.x * size.y) / 60.0;
    let clip = Clip {
        row: 0,
        frames: 4,
        frame_time: 0.07,
        looping: true,
        flipped: false,
    };
    world.spawn((
        Cobweb,
        Position(position),
        Velocity(direction.normalize_or_zero() * speed),
        Drag { rate: COBWEB_DRAG },
        Size(size),
        Color(Vec4::ONE),
        Sprite::cell(textures.get("ingame/cobweb_projectile.tga"), 0, 0, 4, 1),
        Animation::new(4, 1, clip),
        Layer(0),
        Lit,
    ))
}

pub(crate) fn boss_hud(world: &mut World, name: &str, textures: &Textures) {
    let kind = BarKind::Boss;
    let spec = kind.spec();
    let size = spec_texture_size(kind) * spec.scale;
    world.spawn((
        BossHud,
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
    world.spawn((
        BossHud,
        Position(Vec2::new(
            spec.position.x + spec.hole[0] as f32 * spec.scale + 5.0,
            spec.position.y + 30.0,
        )),
        Text {
            content: name.to_owned(),
            color: Vec4::new(1.0, 0.0, 0.0, 1.0),
            size: FontSize::Size22,
            align: Align::Left,
            anchor: Anchor::Baseline,
        },
        Layer(6),
        ScreenSpace,
    ));
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

pub(crate) fn for_sale(world: &mut World, entity: Entity, shop: Shop, prompt_gap: f32) {
    let _ = world.remove_one::<Despawn>(entity);
    let _ = world.insert(
        entity,
        (
            shop,
            Interactable {
                action: Interaction::Buy,
            },
            Prompt {
                text: "Press E to buy",
                gap: prompt_gap,
                color: PROMPT_COLOR,
                size: FontSize::Size22,
            },
        ),
    );
}

pub(crate) fn price_tag(
    world: &mut World,
    owner: Entity,
    texture: &str,
    anchor_top: Vec2,
    price: i32,
    textures: &Textures,
) {
    let size = Vec2::splat(PRICE_TAG_SIZE);
    let center = Vec2::new(anchor_top.x, anchor_top.y - size.y / 2.0 - PRICE_TAG_LIFT);
    world.spawn((
        PriceTag { owner },
        Position(center),
        Size(size),
        Color(Vec4::ONE),
        Sprite::whole(textures.get(texture)),
        Layer(3),
        Lit,
    ));
    world.spawn((
        PriceTag { owner },
        Position(center + Vec2::new(0.0, 6.0)),
        Text {
            content: price.to_string(),
            color: PRICE_COLOR,
            size: FontSize::Size22,
            align: Align::Center,
            anchor: Anchor::Middle,
        },
        Layer(4),
    ));
}

pub(crate) fn shop_speech(world: &mut World, keeper_top_left: Vec2, textures: &Textures) {
    let size = SAY_BOX_SIZE;
    let top_left = keeper_top_left + Vec2::new(SAY_BOX_SHIFT - size.x, SAY_BOX_DROP);
    world.spawn((
        SpeechBubble {
            remaining: 0.0,
            message: "",
        },
        Position(top_left + size / 2.0),
        Size(size),
        Color(Vec4::ONE),
        Sprite::whole(textures.get("ingame/shopkeeper_saybox.tga")),
        Layer(6),
        Hidden,
    ));
    world.spawn((
        SpeechText,
        Position(top_left + Vec2::new(SAY_INSIDE_WIDTH / 2.0, SAY_INSIDE_HEIGHT / 2.0 + 15.0)),
        Text {
            content: String::new(),
            color: SAY_COLOR,
            size: FontSize::Size22,
            align: Align::Center,
            anchor: Anchor::Middle,
        },
        Layer(7),
        Hidden,
    ));
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

pub(crate) fn floating_number(world: &mut World, change: i32, position: Vec2) -> Entity {
    let (content, color) = if change > 0 {
        (format!("+{change}"), Vec4::new(0.0, 1.0, 0.0, 1.0))
    } else {
        (change.abs().to_string(), Vec4::new(1.0, 0.0, 0.0, 1.0))
    };
    world.spawn((
        Floating {
            remaining: FLOAT_SECONDS,
            velocity: Vec2::new(0.0, -FLOAT_RISE / FLOAT_SECONDS),
        },
        Position(position),
        Text {
            content,
            color,
            size: FontSize::Size22,
            align: Align::Left,
            anchor: Anchor::Baseline,
        },
        Layer(4),
    ))
}

pub(crate) fn enemy_in_room(
    world: &mut World,
    kind: EnemyKind,
    position: Vec2,
    floor: u32,
    textures: &Textures,
    room: usize,
) -> Entity {
    let entity = enemy(world, kind, position, floor, textures);
    let _ = world.insert_one(entity, RoomId(room));
    entity
}

#[cfg(test)]
pub(crate) fn ready_enemy(
    world: &mut World,
    physics: &mut Physics,
    kind: EnemyKind,
    position: Vec2,
) -> Entity {
    let entity = enemy(world, kind, position, 0, &Textures::default());
    crate::systems::finish_spawning(world, physics, entity);
    entity
}

pub(crate) fn status_bars(world: &mut World, textures: &Textures) {
    for kind in BarKind::HUD {
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
        BarKind::Boss => Vec2::new(80.0, 19.0),
    }
}

const SLOT_POSITION: Vec2 = Vec2::new(685.0, 450.0);
const SLOT_SIZE: Vec2 = Vec2::new(225.0, 75.0);
const GUN_ICON_SIZE: Vec2 = Vec2::new(41.0, 34.0);
const AMMO_TEXT_POSITION: Vec2 = Vec2::new(900.0, 515.0);
const AMMO_TEXT_COLOR: Vec4 = Vec4::new(226.0 / 255.0, 153.0 / 255.0, 117.0 / 255.0, 1.0);
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

pub(crate) fn weapon_hud(world: &mut World, textures: &Textures) {
    screen_sprite(
        world,
        SLOT_POSITION,
        SLOT_SIZE,
        textures.get("ingame/weaponslot.tga"),
        4,
    );
    let slot_center = SLOT_POSITION + SLOT_SIZE / 2.0;
    let icon = screen_sprite(
        world,
        slot_center - GUN_ICON_SIZE / 2.0,
        GUN_ICON_SIZE,
        textures.get("ingame/guns/pistol.tga"),
        5,
    );
    let _ = world.insert(icon, (GunIcon, Hidden));
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
    for index in 0..MAX_CLIP as i32 {
        world.spawn((
            AmmoPip { index },
            Position(Vec2::ZERO),
            Size(PIP_SIZE),
            Color(Vec4::ONE),
            Sprite::cell(textures.get("ingame/ammo_hud.tga"), 0, 0, 2, 2),
            Animation::new(2, 2, PIP_FULL),
            Layer(5),
            ScreenSpace,
            Hidden,
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
