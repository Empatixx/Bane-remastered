use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::artefacts::ArtefactKind;
use crate::assets::tex;
use crate::audio::SoundEffect;
use crate::assets::{PET, PLAYER, Textures};
use crate::components::{
    YSort,
    Animation, ArgusAttack, ArgusBoss, ArgusStage, Armageddon, Armor, ArtefactSlot, BULLET_COLUMNS,
    BULLET_FLIGHT, BULLET_ROWS, Boss, BossBrain, Bullet,
    Enrage, Chase, Cobweb, Color, ContactDamage, Dash,
    Burrower, Chills, DeathAsh, DeathBurst, DeathCloud, DeathHeal, DeathSplit, Drag, DropShadow,
    Enemy, Kindling, MagmaShell, SwoopStage, Swooper, WormStage,
    EnemyBullet, EyeShooter, Fuse, IcicleDropper,
    GazeWind, GolemAttack, GolemBoss, GolemStage, Hatcher, Heading, Health, Hits, Homing, Hopper,
    HoundAttack, HoundBoss, HoundStage, Impact, Kin, Layer, LeapStage, Leaper, Light, LightFade,
    Lit, Loadout, MagmaSpreader, MimicAttack, MimicBoss, MimicStage, Minion, NoDrop, OgreAttack,
    OgreBoss, OgreStage, Path, Pet, Player, Position, RoomId, Rooted, RunStats, Size, Smash,
    Spawning, Sprite, TrailLayer, Trailing, Velocity, Wallet, WebSpitter,
};
use crate::enemies::{self, Behavior, EnemyKind, ProjectileKind, Role};
use crate::physics::Physics;
use crate::render::TextureId;
use crate::sheets::{pet, player};

pub(crate) const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 76.0);

pub(crate) const PET_SIZE: Vec2 = Vec2::new(96.0, 96.0);
const PET_GAZE_SPEED: f32 = 150.0;
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
        ArtefactSlot {
            current: Some(ArtefactKind::RevealMap),
        },
        RunStats::default(),
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
            player::IDLE.row,
            player::COLUMNS,
            player::ROWS,
        ),
        Animation::new(player::COLUMNS, player::ROWS, player::IDLE),
        Layer(1),
    ));
    let _ = world.insert(
        entity,
        (
            Lit,
            Light::rgb(Vec2::ZERO, 216.0, 156.0, 36.0, 150.0, 0.5),
            DropShadow::default(),
            GazeWind::default(),
            YSort::bottom(PLAYER_SIZE),
            crate::components::SandSunk::default(),
        ),
    );
    let body = physics.add_player(entity, position, PLAYER_SIZE);
    let _ = world.insert_one(entity, body);
    entity
}

pub(crate) fn pet(
    world: &mut World,
    physics: &mut Physics,
    position: Vec2,
    textures: &Textures,
) -> Entity {
    let entity = world.spawn((
        Pet::default(),
        Path::default(),
        Position(position),
        Velocity::default(),
        Size(PET_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(PET), 0, pet::IDLE.row, pet::COLUMNS, pet::ROWS),
        Animation::new(pet::COLUMNS, pet::ROWS, pet::IDLE),
        Layer(1),
        Lit,
        GazeWind::above(PET_GAZE_SPEED),
        YSort::bottom(PET_SIZE),
    ));
    let body = physics.add_pet(entity, position, PET_SIZE);
    let _ = world.insert_one(entity, body);
    entity
}

const BULLET_DAMAGE: i32 = 1;

const COBWEB_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const COBWEB_FORCE: f32 = 150_000_000.0;
const COBWEB_DRAG: f32 = 1.5;

pub(crate) fn bullet(
    world: &mut World,
    physics: &mut Physics,
    position: Vec2,
    velocity: Vec2,
    bullet: Bullet,
    texture: TextureId,
) -> Entity {
    let spec = bullet.round.spec();
    let [red, green, blue] = spec.light;
    let entity = world.spawn((
        bullet,
        Position(position),
        Velocity(velocity),
        Size(spec.size),
        Color(Vec4::ONE),
        Sprite::cell(texture, 0, BULLET_FLIGHT.row, BULLET_COLUMNS, BULLET_ROWS),
        Animation::new(BULLET_COLUMNS, BULLET_ROWS, BULLET_FLIGHT),
        Layer(2),
        Lit,
        Light::rgb(Vec2::ZERO, red, green, blue, 25.0, 0.75),
        LightFade { base: 0.75 },
    ));
    if bullet.round.turns() {
        let _ = world.insert_one(entity, Heading(velocity.to_angle()));
    }
    if spec.trail.is_some() {
        let _ = world.insert_one(entity, Trailing::default());
    }
    let body = physics.add_bullet(entity, position, velocity, spec.hitbox);
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
    let health = enemies::scale_stat(spec.health, enemies::health_multiplier(floor));
    let damage = enemies::scale_stat(spec.damage, enemies::damage_multiplier(floor));
    let spawn_seconds = if boss {
        enemies::BOSS_SPAWN_SECONDS
    } else {
        enemies::COMMON_SPAWN_SECONDS
    };
    let entity = world.spawn((
        Enemy {
            death: spec.death_clip(),
            kind,
        },
        Health {
            current: health,
            max: health,
        },
        ContactDamage(damage),
        Chase {
            speed: spec.speed(),
        },
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
        YSort::bottom(size),
    ));
    if spec.reform_row.is_some() {
        let _ = world.insert_one(entity, crate::components::Revives);
    }
    match spec.behavior {
        Behavior::Plain => {}
        Behavior::PoisonTrail => {
            let _ = world.insert_one(
                entity,
                TrailLayer {
                    spent: 0.0,
                    last: position,
                    count: 0,
                },
            );
        }
        Behavior::Dash => {
            let _ = world.insert_one(
                entity,
                Dash {
                    base_speed: spec.speed(),
                    bonus_speed: enemies::speed_from_force(enemies::DASH_FORCE, size),
                    cycle: 0.0,
                    trail: 0.0,
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
        Behavior::Chills => {
            let _ = world.insert_one(entity, Chills);
        }
        Behavior::DropsIcicles => {
            // Bats in one room do not all drop at once.
            let stagger = (position.x as u32 ^ position.y as u32) % 10;
            let _ = world.insert_one(
                entity,
                IcicleDropper {
                    remaining: enemies::ICICLE_COOLDOWN * (0.5 + stagger as f32 * 0.1),
                },
            );
        }
        Behavior::Kindles => {
            let _ = world.insert_one(
                entity,
                Kindling {
                    lit: false,
                    base_speed: spec.speed(),
                },
            );
        }
        Behavior::Swoops => {
            let stagger = (position.x as u32 ^ position.y as u32) % 10;
            let _ = world.insert(
                entity,
                (
                    Swooper {
                        stage: SwoopStage::Circle,
                        timer: enemies::SWOOP_EVERY * (0.5 + stagger as f32 * 0.1),
                        base_speed: spec.speed(),
                    },
                    DeathAsh,
                ),
            );
        }
        Behavior::Molten => {
            let stagger = (position.x as u32 ^ position.y as u32) % 10;
            let _ = world.insert_one(
                entity,
                MagmaShell {
                    crusted: false,
                    timer: enemies::MAGMA_TURN,
                    mortar: enemies::MORTAR_EVERY * (0.4 + stagger as f32 * 0.1),
                    base_speed: spec.speed(),
                },
            );
        }
        Behavior::Burrows => {
            let _ = world.insert_one(
                entity,
                Burrower {
                    stage: WormStage::Rising,
                    timer: 0.0,
                    shot: 0.0,
                    shots_left: 0,
                    damage,
                    emerged: 0,
                },
            );
            let _ = world.insert_one(
                entity,
                Animation::new(spec.columns, spec.rows, enemies::WORM_RISE_CLIP),
            );
        }
        Behavior::Hops => {
            let _ = world.insert_one(entity, Rooted);
            let _ = world.insert_one(
                entity,
                Hopper {
                    timer: 0.2 + (position.x as u32 % 7) as f32 * 0.06,
                    hopping: false,
                    idle,
                },
            );
        }
        Behavior::MimicBoss => {
            let _ = world.insert_one(
                entity,
                MimicBoss {
                    brain: BossBrain::new(
                        MimicStage::Walk,
                        MimicAttack::Hop,
                        enemies::MIMIC_FIRST_COOLDOWN,
                        0x7A3C_91D5 ^ (position.x as u32).wrapping_mul(23) ^ (position.y as u32),
                        idle,
                    ),
                    charging: false,
                },
            );
            let _ = world.insert_one(
                entity,
                Enrage {
                    at: enemies::MIMIC_RAGE_AT,
                    speed: enemies::MIMIC_RAGE_SPEED,
                    sound: SoundEffect::MimicRage,
                    tint: Some(enemies::MIMIC_RAGE_TINT),
                },
            );
        }
        Behavior::HoundBoss => {
            let _ = world.insert_one(entity, MagmaSpreader::default());
            let _ = world.insert_one(
                entity,
                HoundBoss {
                    brain: BossBrain::new(
                        HoundStage::Prowl,
                        HoundAttack::Lunge,
                        enemies::HOUND_FIRST_COOLDOWN,
                        0x27D4_EB2F ^ (position.x as u32).wrapping_mul(19) ^ (position.y as u32),
                        idle,
                    ),
                    angles: [0.0; 3],
                    charging: false,
                },
            );
            let _ = world.insert_one(
                entity,
                Enrage {
                    at: enemies::HOUND_RAGE_AT,
                    speed: enemies::HOUND_RAGE_SPEED,
                    sound: SoundEffect::HoundRage,
                    tint: Some(enemies::HOUND_RAGE_TINT),
                },
            );
        }
        Behavior::OgreBoss => {
            let _ = world.insert_one(
                entity,
                OgreBoss {
                    brain: BossBrain::new(
                        OgreStage::Walk,
                        OgreAttack::Jump,
                        enemies::OGRE_FIRST_COOLDOWN,
                        0xC2B2_AE35 ^ (position.x as u32).wrapping_mul(13) ^ (position.y as u32),
                        idle,
                    ),
                },
            );
            let _ = world.insert_one(
                entity,
                Enrage {
                    at: enemies::OGRE_RAGE_AT,
                    speed: enemies::OGRE_RAGE_SPEED,
                    sound: SoundEffect::OgreRage,
                    tint: Some(enemies::OGRE_RAGE_TINT),
                },
            );
        }
        Behavior::ArgusBoss => {
            let _ = world.insert_one(
                entity,
                ArgusBoss {
                    brain: BossBrain::new(
                        ArgusStage::Float,
                        ArgusAttack::Volley,
                        enemies::ARGUS_FIRST_COOLDOWN,
                        0x85EB_CA6B ^ (position.x as u32).wrapping_mul(17) ^ (position.y as u32),
                        idle,
                    ),
                    pulse: 0.0,
                    pulses: 0,
                    angle: 0.0,
                    attacks: 0,
                },
            );
            let _ = world.insert_one(
                entity,
                Enrage {
                    at: enemies::ARGUS_RAGE_AT,
                    speed: enemies::ARGUS_RAGE_SPEED,
                    sound: SoundEffect::ArgusRage,
                    tint: None,
                },
            );
        }
        Behavior::GolemBoss => {
            let _ = world.insert_one(
                entity,
                GolemBoss {
                    brain: BossBrain::new(
                        GolemStage::Walk,
                        GolemAttack::Slam,
                        enemies::GOLEM_FIRST_COOLDOWN,
                        0x9E37_79B9 ^ (position.x as u32).wrapping_mul(31) ^ (position.y as u32),
                        idle,
                    ),
                    pulse: 0.0,
                    rain_dropped: 0,
                    second_ring: 0.0,
                    beam_angle: 0.0,
                },
            );
            let _ = world.insert_one(
                entity,
                Enrage {
                    at: enemies::GOLEM_RAGE_AT,
                    speed: enemies::GOLEM_RAGE_SPEED,
                    sound: SoundEffect::GolemRage,
                    tint: Some(enemies::GOLEM_RAGE_TINT),
                },
            );
        }
        Behavior::Leaps => {
            let _ = world.insert_one(
                entity,
                Leaper {
                    stage: LeapStage::Stalk,
                    timer: enemies::LEAP_FIRST_COOLDOWN,
                    target: position,
                    damage,
                    idle,
                },
            );
        }
        Behavior::ToxicBurst => {
            let _ = world.insert_one(entity, DeathCloud);
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
    if kind == EnemyKind::SnowSlime {
        let _ = world.insert_one(
            entity,
            DeathSplit {
                kind: EnemyKind::SnowSlimeling,
                count: enemies::SNOW_SPLIT,
            },
        );
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
    if kind == ProjectileKind::Snow {
        let _ = world.insert_one(entity, Chills);
    }
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
    make_ready(world, physics, entity);
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
    world.spawn((
        Cobweb,
        Position(position),
        Velocity(direction.normalize_or_zero() * speed),
        Drag { rate: COBWEB_DRAG },
        Size(size),
        Color(Vec4::ONE),
        Sprite::whole(textures.get(tex::COBWEB_PROJECTILE)),
        Layer(0),
        Lit,
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

/// Puts an enemy's body into the simulation, at its `Position` and `Size`: the last piece of an
/// enemy built ready ([`make_ready`]), and what `systems::tick_spawning` gives a risen one.
pub(crate) fn enemy_body(world: &mut World, physics: &mut Physics, entity: Entity) {
    let placement = world
        .query_one_mut::<(&Position, &Size)>(entity)
        .map(|(position, size)| (position.0, size.0));
    if let Ok((position, size)) = placement {
        let body = physics.add_enemy(entity, position, size);
        let _ = world.insert_one(entity, body);
    }
}

/// Makes a just-built enemy ready at once, without rising from the floor (a hatchling, a
/// test's enemy): no `Spawning`, full colour, its body in the simulation.
pub(crate) fn make_ready(world: &mut World, physics: &mut Physics, entity: Entity) {
    let _ = world.remove_one::<Spawning>(entity);
    if let Ok(color) = world.query_one_mut::<&mut Color>(entity) {
        color.0 = Vec4::ONE;
    }
    enemy_body(world, physics, entity);
}

#[cfg(test)]
pub(crate) fn ready_enemy(
    world: &mut World,
    physics: &mut Physics,
    kind: EnemyKind,
    position: Vec2,
) -> Entity {
    let entity = enemy(world, kind, position, 0, &Textures::default());
    make_ready(world, physics, entity);
    entity
}
