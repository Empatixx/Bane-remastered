//! The lava biome's enemies: the ember rat that flares up and rushes, the
//! flame bat that swoops and rises from its ash, the magma slime that crusts
//! over and lobs magma, and the lava worm that bursts up from below.

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::audio::{AudioCommand, SoundEffect};
use crate::clip::Clip;
use crate::components::{
    Animation, AshPile, Burrower, Chase, Color, ContactDamage, DeathAsh, DeathBlast, Dodge, Enemy,
    Expiring, FallKind, FallingRock, Health, Invulnerable, Kindling, Layer, Light, Lit, MagmaShell,
    Player, Position, RockFall, RoomId, Sealed, Size, Spawning, Sprite, SwoopStage, Swooper,
    WormStage,
};
use crate::enemies::{self, EnemyKind};
use crate::geometry;
use crate::physics::Physics;
use crate::prefabs;
use crate::systems::damage::{Damage, DamageSource, deal, hurt_player};

const EMBER_GLOW: [f32; 3] = [255.0, 120.0, 30.0];
const SWOOP_RANGE: f32 = 460.0;
const ASH_SIZE: Vec2 = Vec2::new(48.0, 18.0);
const MAGMA_BALL_SIZE: Vec2 = Vec2::new(34.0, 34.0);
const MAGMA_SHADOW: Vec2 = Vec2::new(64.0, 30.0);
const MOUND_SIZE: Vec2 = Vec2::new(72.0, 21.0);
const WARNING_SIZE: Vec2 = Vec2::new(120.0, 56.0);
const WARNING_COLOR: Vec4 = Vec4::new(1.0, 0.45, 0.15, 0.9);
const WORM_BITE_CLIP: Clip = Clip {
    row: 1,
    frames: 4,
    frame_time: 0.1,
    looping: false,
    flipped: false,
};
const MOUND_CLIP: Clip = Clip {
    row: 0,
    frames: 2,
    frame_time: 0.25,
    looping: true,
    flipped: false,
};
const ASH_CLIP: Clip = Clip {
    row: 0,
    frames: 2,
    frame_time: 0.3,
    looping: true,
    flipped: false,
};
const BALL_CLIP: Clip = Clip {
    row: 0,
    frames: 4,
    frame_time: 0.07,
    looping: true,
    flipped: false,
};

fn player_at(world: &World) -> Option<(Entity, Vec2, Vec2)> {
    super::golem::player_body(world)
}

fn can_be_hit(world: &World, player: Entity) -> bool {
    world.get::<&Dodge>(player).is_err() && world.get::<&Invulnerable>(player).is_err()
}

// ---------- ember rat ----------

/// A hit ember rat flares up, hisses and rushes the player; touching the player
/// it bursts.
pub(crate) fn tick_ember_rats(world: &mut World, sounds: &mut Vec<AudioCommand>, clock: f32) {
    let player = player_at(world);
    let mut lit_now = Vec::new();
    for (entity, kindling, health, chase, color, position, size) in world
        .query_mut::<(
            Entity,
            &mut Kindling,
            &mut Health,
            &mut Chase,
            &mut Color,
            &Position,
            &Size,
        )>()
        .without::<&Spawning>()
        .without::<&Expiring>()
    {
        if !kindling.lit && health.current < health.max && health.current > 0 {
            kindling.lit = true;
            chase.speed = kindling.base_speed * enemies::EMBER_RUSH;
            lit_now.push((entity, position.0));
        }
        if kindling.lit {
            let flicker = 0.08 * (clock * 18.0 + position.0.x * 0.01).sin();
            color.0 = Vec4::new(1.0, 0.78 + flicker, 0.55 + flicker, color.0.w);
            if let Some((_, at, player_size)) = player {
                if health.current > 0 && geometry::overlaps(position.0, size.0, at, player_size) {
                    health.current = 0;
                }
            }
        }
    }
    for (entity, at) in lit_now {
        let [red, green, blue] = EMBER_GLOW;
        let _ = world.insert(
            entity,
            (
                DeathBlast,
                Light::rgb(Vec2::ZERO, red, green, blue, 70.0, 1.0),
            ),
        );
        sounds.push(AudioCommand::At(SoundEffect::HoundInhale, at));
    }
}

/// Lit ember rats burst where they die.
pub(crate) fn death_blasts(world: &mut World, textures: &Textures, sounds: &mut Vec<AudioCommand>) {
    let dying: Vec<(Entity, Vec2)> = world
        .query::<(Entity, &Position, &Health)>()
        .with::<&DeathBlast>()
        .without::<&Expiring>()
        .iter()
        .filter(|(_, _, health)| health.current <= 0)
        .map(|(entity, position, _)| (entity, position.0))
        .collect();
    for (entity, at) in dying {
        let _ = world.remove_one::<DeathBlast>(entity);
        ember_blast(world, entity, at, textures, sounds);
    }
}

fn ember_blast(
    world: &mut World,
    source: Entity,
    center: Vec2,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
) {
    sounds.push(AudioCommand::At(SoundEffect::Explosion, center));
    prefabs::blast(
        world,
        center,
        Vec2::splat(enemies::EMBER_BLAST_REACH * 2.2),
        textures,
    );
    let reach = Vec2::splat(enemies::EMBER_BLAST_REACH * 2.0);
    let victims: Vec<(Entity, bool)> = world
        .query::<(Entity, &Position, &Size, Option<&Player>)>()
        .with::<&Health>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .filter(|(entity, position, size, _)| {
            *entity != source && geometry::overlaps(center, reach, position.0, size.0)
        })
        .map(|(entity, _, _, player)| (entity, player.is_some()))
        .collect();
    for (entity, is_player) in victims {
        if is_player {
            if can_be_hit(world, entity) {
                hurt_player(world, entity, enemies::EMBER_BLAST_DAMAGE, sounds);
            }
        } else if world.get::<&Enemy>(entity).is_ok() {
            deal(
                world,
                Damage {
                    target: entity,
                    amount: enemies::EMBER_BLAST_DAMAGE,
                    source: DamageSource::Explosion,
                },
            );
        }
    }
}

// ---------- flame bat ----------

/// Flame bats circle, flare up for a moment, then swoop fast through the player.
pub(crate) fn tick_swoopers(world: &mut World, sounds: &mut Vec<AudioCommand>, dt: f32) {
    let player = player_at(world).map(|(_, at, _)| at);
    let mut flare = Vec::new();
    let mut calm = Vec::new();
    for (entity, swooper, chase, position) in world
        .query_mut::<(Entity, &mut Swooper, &mut Chase, &Position)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
    {
        swooper.timer -= dt;
        match swooper.stage {
            SwoopStage::Circle => {
                chase.speed = swooper.base_speed;
                let near = player.is_some_and(|at| at.distance(position.0) <= SWOOP_RANGE);
                if swooper.timer <= 0.0 && near {
                    swooper.stage = SwoopStage::Windup;
                    swooper.timer = enemies::SWOOP_WINDUP;
                    chase.speed = 0.0;
                    flare.push(entity);
                    sounds.push(AudioCommand::At(SoundEffect::HoundBreath, position.0));
                }
            }
            SwoopStage::Windup => {
                if swooper.timer <= 0.0 {
                    swooper.stage = SwoopStage::Dive;
                    swooper.timer = enemies::SWOOP_DIVE;
                    chase.speed = swooper.base_speed * enemies::SWOOP_RUSH;
                    sounds.push(AudioCommand::At(SoundEffect::GhostDash, position.0));
                }
            }
            SwoopStage::Dive => {
                if swooper.timer <= 0.0 {
                    swooper.stage = SwoopStage::Circle;
                    swooper.timer = enemies::SWOOP_EVERY;
                    chase.speed = swooper.base_speed;
                    calm.push(entity);
                }
            }
        }
    }
    for entity in flare {
        let [red, green, blue] = EMBER_GLOW;
        let _ = world.insert_one(entity, Light::rgb(Vec2::ZERO, red, green, blue, 80.0, 1.3));
    }
    for entity in calm {
        let _ = world.remove_one::<Light>(entity);
    }
}

/// A dead flame bat leaves a heap of ash.
pub(crate) fn death_ashes(world: &mut World, physics: &mut Physics, textures: &Textures) {
    let dying: Vec<(Entity, Vec2, Option<RoomId>)> = world
        .query::<(Entity, &Position, &Health, Option<&RoomId>)>()
        .with::<&DeathAsh>()
        .without::<&Expiring>()
        .iter()
        .filter(|(_, _, health, _)| health.current <= 0)
        .map(|(entity, position, _, room)| (entity, position.0, room.copied()))
        .collect();
    for (entity, at, room) in dying {
        let _ = world.remove_one::<DeathAsh>(entity);
        let ash = world.spawn((
            AshPile {
                remaining: enemies::ASH_REBIRTH,
            },
            Health {
                current: enemies::ASH_HEALTH,
                max: enemies::ASH_HEALTH,
            },
            Position(at),
            Size(ASH_SIZE),
            Color(Vec4::ONE),
            Sprite::cell(textures.get(tex::ASH_PILE), 0, 0, 2, 1),
            Animation::new(2, 1, ASH_CLIP),
            Layer(-1),
            Lit,
        ));
        if let Some(room) = room {
            let _ = world.insert_one(ash, room);
        }
        let body = physics.add_fixed_object(ash, at, ASH_SIZE);
        let _ = world.insert_one(ash, body);
    }
}

/// Ash that is shot apart is gone; ash left alone rises as a new flame bat.
pub(crate) fn tick_ashes(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    let mut gone = Vec::new();
    let mut risen = Vec::new();
    for (entity, ash, health, position, room) in
        world.query_mut::<(Entity, &mut AshPile, &Health, &Position, Option<&RoomId>)>()
    {
        if health.current <= 0 {
            gone.push((entity, position.0));
            continue;
        }
        ash.remaining -= dt;
        if ash.remaining <= 0.0 {
            risen.push((entity, position.0, room.copied()));
        }
    }
    for (entity, at) in gone {
        physics.detach(world, entity);
        let _ = world.despawn(entity);
        prefabs::blast(world, at, Vec2::splat(56.0), textures);
    }
    for (entity, at, room) in risen {
        physics.detach(world, entity);
        let _ = world.despawn(entity);
        let bat = prefabs::hatchling(world, physics, EnemyKind::FlameBat, at, room, textures);
        // A bat that rose once stays dead the next time.
        let _ = world.remove_one::<DeathAsh>(bat);
        sounds.push(AudioCommand::At(SoundEffect::HoundBreath, at));
    }
}

// ---------- magma slime ----------

/// Magma slimes take turns between molten (fast, open to hits) and crusted
/// (slow, bullets do nothing), and lob magma at the player.
pub(crate) fn tick_magma_slimes(
    world: &mut World,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    let player = player_at(world).map(|(_, at, _)| at);
    let mut turned = Vec::new();
    let mut lobs = Vec::new();
    for (entity, shell, chase, sprite, position, room) in world
        .query_mut::<(
            Entity,
            &mut MagmaShell,
            &mut Chase,
            &mut Sprite,
            &Position,
            Option<&RoomId>,
        )>()
        .without::<&Spawning>()
        .without::<&Expiring>()
    {
        shell.timer -= dt;
        if shell.timer <= 0.0 {
            shell.timer = enemies::MAGMA_TURN;
            shell.crusted = !shell.crusted;
            sprite.texture = textures.get(if shell.crusted {
                tex::MAGMA_SLIME_CRUST
            } else {
                tex::MAGMA_SLIME
            });
            turned.push((entity, shell.crusted));
        }
        chase.speed = if shell.crusted {
            shell.base_speed * enemies::CRUST_SLOWDOWN
        } else {
            shell.base_speed
        };
        shell.mortar -= dt;
        if let Some(target) = player {
            if shell.mortar <= 0.0 && target.distance(position.0) <= enemies::MORTAR_RANGE {
                shell.mortar = enemies::MORTAR_EVERY;
                lobs.push((position.0, target, room.copied()));
            }
        }
    }
    for (entity, crusted) in turned {
        if crusted {
            let _ = world.insert_one(entity, Sealed);
        } else {
            let _ = world.remove_one::<Sealed>(entity);
        }
    }
    for (from, target, room) in lobs {
        sounds.push(AudioCommand::At(SoundEffect::SlimeSmash, from));
        lob_magma(world, textures, from, target, room);
    }
}

pub(crate) fn lob_magma(
    world: &mut World,
    textures: &Textures,
    from: Vec2,
    ground: Vec2,
    room: Option<RoomId>,
) {
    let ball = world.spawn((
        FallingRock,
        Position(from),
        Size(MAGMA_BALL_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(tex::MAGMA_BALL), 0, 0, 4, 2),
        Animation::new(4, 2, BALL_CLIP),
        Layer(4),
        Light::rgb(Vec2::ZERO, 255.0, 130.0, 30.0, 40.0, 0.9),
    ));
    world.spawn((
        RockFall {
            remaining: enemies::MORTAR_FLIGHT,
            duration: enemies::MORTAR_FLIGHT,
            rock: ball,
            room,
            kind: FallKind::Magma,
            from: Some(from),
        },
        Position(ground),
        Size(MAGMA_SHADOW),
        Color(Vec4::ONE),
        Sprite::whole(textures.get(tex::ROCK_SHADOW)),
        Layer(-1),
    ));
}

/// The lobbed magma splashes and hurts the player standing there.
pub(crate) fn magma_lands(
    world: &mut World,
    textures: &Textures,
    ground: Vec2,
    player: Option<(Entity, Vec2, Vec2)>,
    sounds: &mut Vec<AudioCommand>,
) {
    sounds.push(AudioCommand::At(SoundEffect::SlimeBurst, ground));
    prefabs::blast(world, ground, Vec2::splat(90.0), textures);
    let Some((player, at, size)) = player else {
        return;
    };
    let reach = Vec2::splat(enemies::MORTAR_RADIUS * 2.0);
    if can_be_hit(world, player) && geometry::overlaps(ground, reach, at, size) {
        hurt_player(world, player, enemies::MORTAR_DAMAGE, sounds);
    }
}

// ---------- lava worm ----------

/// Lava worms tunnel towards the player as a glowing hump, warn, burst up,
/// bite for a while and go back down.
pub(crate) fn tick_worms(
    world: &mut World,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    let player = player_at(world);
    let mut changes: Vec<(Entity, WormStage, Vec2, Option<Entity>, i32)> = Vec::new();
    let mut bites: Vec<(Entity, Vec2, i32)> = Vec::new();
    for (entity, worm, chase, position) in world
        .query_mut::<(Entity, &mut Burrower, &mut Chase, &Position)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
    {
        worm.timer += dt;
        let near =
            player.is_some_and(|(_, at, _)| at.distance(position.0) <= enemies::WORM_SURFACE_RANGE);
        let next = match worm.stage {
            WormStage::Under => {
                chase.speed = worm.base_speed;
                (near || worm.timer >= enemies::WORM_MAX_UNDER).then_some(WormStage::Warning)
            }
            WormStage::Warning => {
                chase.speed = 0.0;
                (worm.timer >= enemies::WORM_WARNING).then_some(WormStage::Rising)
            }
            WormStage::Rising => {
                chase.speed = 0.0;
                (worm.timer >= enemies::WORM_RISE).then_some(WormStage::Up)
            }
            WormStage::Up => {
                chase.speed = 0.0;
                worm.bite -= dt;
                if worm.bite <= 0.0 {
                    worm.bite = enemies::WORM_BITE_EVERY;
                    bites.push((entity, position.0, worm.damage));
                }
                (worm.timer >= enemies::WORM_UP).then_some(WormStage::Under)
            }
        };
        if let Some(stage) = next {
            changes.push((entity, stage, position.0, worm.warning, worm.damage));
            worm.stage = stage;
            worm.timer = 0.0;
            worm.bite = 0.3;
        }
    }
    for (entity, at, damage) in bites {
        if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
            animation.play(WORM_BITE_CLIP);
        }
        sounds.push(AudioCommand::At(SoundEffect::MimicChomp, at));
        if let Some((player, player_at, _)) = player {
            if can_be_hit(world, player) && player_at.distance(at) <= enemies::WORM_BITE_REACH {
                hurt_player(world, player, damage, sounds);
            }
        }
    }
    for (entity, stage, at, warning, damage) in changes {
        match stage {
            WormStage::Warning => {
                let marker = world.spawn((
                    Position(at),
                    Size(WARNING_SIZE),
                    Color(WARNING_COLOR),
                    Sprite::whole(textures.get(tex::ROCK_SHADOW)),
                    Layer(-1),
                ));
                if let Ok(mut worm) = world.get::<&mut Burrower>(entity) {
                    worm.warning = Some(marker);
                }
            }
            WormStage::Rising => {
                if let Some(marker) = warning {
                    let _ = world.despawn(marker);
                }
                surface(world, entity, textures, damage);
                sounds.push(AudioCommand::At(SoundEffect::GolemSlam, at));
                prefabs::blast(
                    world,
                    at,
                    Vec2::splat(enemies::WORM_ERUPT_REACH * 2.2),
                    textures,
                );
                if let Some((player, player_at, _)) = player {
                    if can_be_hit(world, player)
                        && player_at.distance(at) <= enemies::WORM_ERUPT_REACH
                    {
                        hurt_player(world, player, damage, sounds);
                    }
                }
            }
            WormStage::Under => {
                go_under(world, entity, textures);
                sounds.push(AudioCommand::At(SoundEffect::LeaperLand, at));
            }
            WormStage::Up => {}
        }
    }
}

fn surface(world: &mut World, entity: Entity, textures: &Textures, damage: i32) {
    let spec = EnemyKind::LavaWorm.spec();
    let _ = world.remove_one::<Sealed>(entity);
    let _ = world.insert(
        entity,
        (
            ContactDamage(damage),
            Size(spec.size()),
            Sprite::cell(textures.get(tex::LAVA_WORM), 0, 0, spec.columns, spec.rows),
            Animation::new(spec.columns, spec.rows, enemies::WORM_RISE_CLIP),
        ),
    );
}

fn go_under(world: &mut World, entity: Entity, textures: &Textures) {
    let _ = world.remove_one::<ContactDamage>(entity);
    let _ = world.insert(
        entity,
        (
            Sealed,
            Size(MOUND_SIZE),
            Sprite::cell(textures.get(tex::WORM_MOUND), 0, 0, 2, 1),
            Animation::new(2, 1, MOUND_CLIP),
        ),
    );
}

#[cfg(test)]
mod tests {
    use crate::components::{Armor, Kindling};
    use crate::dungeon::Theme;
    use crate::rng::Rng;

    use super::*;

    fn arena() -> (World, Physics, Entity) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let player = prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(500.0, 500.0),
            &Textures::default(),
        );
        (world, physics, player)
    }

    fn toughness(world: &World, entity: Entity) -> i32 {
        world.get::<&Health>(entity).expect("health").current
            + world.get::<&Armor>(entity).map_or(0, |armor| armor.current)
    }

    fn spawn(world: &mut World, physics: &mut Physics, kind: EnemyKind, at: Vec2) -> Entity {
        prefabs::ready_enemy(world, physics, kind, at)
    }

    fn hit(world: &mut World, target: Entity, amount: i32) {
        deal(
            world,
            Damage {
                target,
                amount,
                source: DamageSource::Bullet,
            },
        );
    }

    #[test]
    fn the_lava_biome_has_its_own_rat_bat_slime_and_worm() {
        let mut rng = Rng::new(3);
        let lava: std::collections::HashSet<EnemyKind> = (0..3000)
            .map(|_| crate::enemies::pick(3, Theme::Lava, &mut rng))
            .collect();
        for kind in [
            EnemyKind::EmberRat,
            EnemyKind::FlameBat,
            EnemyKind::MagmaSlime,
            EnemyKind::LavaWorm,
        ] {
            assert!(lava.contains(&kind), "{kind:?}");
        }
        for kind in [
            EnemyKind::Rat,
            EnemyKind::Bat,
            EnemyKind::Slime,
            EnemyKind::FrostRat,
        ] {
            assert!(!lava.contains(&kind), "{kind:?}");
        }
    }

    #[test]
    fn a_hit_ember_rat_flares_up_rushes_and_bursts_on_the_player() {
        let (mut world, mut physics, player) = arena();
        let at = world.get::<&Position>(player).expect("player").0;
        let rat = spawn(
            &mut world,
            &mut physics,
            EnemyKind::EmberRat,
            at + Vec2::new(300.0, 0.0),
        );
        let calm = world.get::<&Chase>(rat).expect("chase").speed;
        hit(&mut world, rat, 1);
        tick_ember_rats(&mut world, &mut Vec::new(), 0.0);
        assert!(world.get::<&Kindling>(rat).expect("kindling").lit);
        assert!(world.get::<&Chase>(rat).expect("chase").speed > calm * 2.0);

        world.get::<&mut Position>(rat).expect("rat").0 = at;
        let before = toughness(&world, player);
        tick_ember_rats(&mut world, &mut Vec::new(), 0.1);
        assert_eq!(
            world.get::<&Health>(rat).expect("rat").current,
            0,
            "it bursts on contact"
        );
        death_blasts(&mut world, &Textures::default(), &mut Vec::new());
        assert!(toughness(&world, player) < before);
    }

    #[test]
    fn an_unlit_ember_rat_dies_without_a_blast() {
        let (mut world, mut physics, player) = arena();
        let at = world.get::<&Position>(player).expect("player").0;
        let rat = spawn(
            &mut world,
            &mut physics,
            EnemyKind::EmberRat,
            at + Vec2::new(40.0, 0.0),
        );
        world.get::<&mut Health>(rat).expect("rat").current = 0;
        let before = toughness(&world, player);
        death_blasts(&mut world, &Textures::default(), &mut Vec::new());
        assert_eq!(toughness(&world, player), before);
    }

    #[test]
    fn a_flame_bat_winds_up_then_dives_fast_then_circles_again() {
        let (mut world, mut physics, player) = arena();
        let at = world.get::<&Position>(player).expect("player").0;
        let bat = spawn(
            &mut world,
            &mut physics,
            EnemyKind::FlameBat,
            at + Vec2::new(200.0, 0.0),
        );
        let base = world.get::<&Swooper>(bat).expect("swooper").base_speed;
        tick_swoopers(&mut world, &mut Vec::new(), enemies::SWOOP_EVERY * 2.0);
        assert_eq!(
            world.get::<&Swooper>(bat).expect("bat").stage,
            SwoopStage::Windup
        );
        assert_eq!(world.get::<&Chase>(bat).expect("bat").speed, 0.0);
        tick_swoopers(&mut world, &mut Vec::new(), enemies::SWOOP_WINDUP + 0.01);
        assert_eq!(
            world.get::<&Swooper>(bat).expect("bat").stage,
            SwoopStage::Dive
        );
        assert!(world.get::<&Chase>(bat).expect("bat").speed > base * 3.0);
        tick_swoopers(&mut world, &mut Vec::new(), enemies::SWOOP_DIVE + 0.01);
        assert_eq!(
            world.get::<&Swooper>(bat).expect("bat").stage,
            SwoopStage::Circle
        );
    }

    fn bats(world: &World) -> usize {
        world
            .query::<&Enemy>()
            .iter()
            .filter(|enemy| enemy.kind == EnemyKind::FlameBat)
            .count()
    }

    #[test]
    fn a_flame_bat_rises_from_its_ash_once_unless_the_ash_is_shot() {
        let (mut world, mut physics, _player) = arena();
        let textures = Textures::default();
        let bat = spawn(
            &mut world,
            &mut physics,
            EnemyKind::FlameBat,
            Vec2::new(100.0, 100.0),
        );
        world.get::<&mut Health>(bat).expect("bat").current = 0;
        death_ashes(&mut world, &mut physics, &textures);
        let _ = world.despawn(bat);
        assert_eq!(world.query::<&AshPile>().iter().count(), 1);
        tick_ashes(
            &mut world,
            &mut physics,
            &textures,
            &mut Vec::new(),
            enemies::ASH_REBIRTH + 0.1,
        );
        assert_eq!(world.query::<&AshPile>().iter().count(), 0);
        assert_eq!(bats(&world), 1);
        let risen = world
            .query::<(Entity, &Enemy)>()
            .iter()
            .find(|(_, enemy)| enemy.kind == EnemyKind::FlameBat)
            .map(|(entity, _)| entity)
            .expect("risen bat");
        assert!(world.get::<&DeathAsh>(risen).is_err(), "it rises only once");

        let second = spawn(
            &mut world,
            &mut physics,
            EnemyKind::FlameBat,
            Vec2::new(300.0, 100.0),
        );
        world.get::<&mut Health>(second).expect("bat").current = 0;
        death_ashes(&mut world, &mut physics, &textures);
        let _ = world.despawn(second);
        let ash = world
            .query::<Entity>()
            .with::<&AshPile>()
            .iter()
            .next()
            .expect("ash");
        hit(&mut world, ash, enemies::ASH_HEALTH);
        tick_ashes(&mut world, &mut physics, &textures, &mut Vec::new(), 0.1);
        assert_eq!(world.query::<&AshPile>().iter().count(), 0);
        assert_eq!(bats(&world), 1, "shot ash never rises");
    }

    #[test]
    fn a_crusted_magma_slime_shrugs_off_hits_and_a_molten_one_does_not() {
        let (mut world, mut physics, _player) = arena();
        let slime = spawn(
            &mut world,
            &mut physics,
            EnemyKind::MagmaSlime,
            Vec2::new(100.0, 100.0),
        );
        let full = world.get::<&Health>(slime).expect("slime").current;
        tick_magma_slimes(
            &mut world,
            &Textures::default(),
            &mut Vec::new(),
            enemies::MAGMA_TURN + 0.01,
        );
        assert!(world.get::<&MagmaShell>(slime).expect("slime").crusted);
        hit(&mut world, slime, 2);
        assert_eq!(world.get::<&Health>(slime).expect("slime").current, full);
        tick_magma_slimes(
            &mut world,
            &Textures::default(),
            &mut Vec::new(),
            enemies::MAGMA_TURN + 0.01,
        );
        hit(&mut world, slime, 2);
        assert!(world.get::<&Health>(slime).expect("slime").current < full);
    }

    #[test]
    fn a_magma_slime_lobs_magma_that_lands_where_the_player_stood() {
        let (mut world, mut physics, player) = arena();
        let at = world.get::<&Position>(player).expect("player").0;
        spawn(
            &mut world,
            &mut physics,
            EnemyKind::MagmaSlime,
            at + Vec2::new(250.0, 0.0),
        );
        tick_magma_slimes(
            &mut world,
            &Textures::default(),
            &mut Vec::new(),
            enemies::MORTAR_EVERY * 2.0,
        );
        let target = world
            .query::<(&RockFall, &Position)>()
            .iter()
            .find(|(fall, _)| fall.kind == FallKind::Magma)
            .map(|(_, position)| position.0)
            .expect("a lobbed ball");
        assert_eq!(target, at);
        let before = toughness(&world, player);
        crate::systems::tick_rock_falls(
            &mut world,
            &mut physics,
            &crate::navigation::Navigation::default(),
            &Textures::default(),
            &mut Vec::new(),
            enemies::MORTAR_FLIGHT + 0.1,
        );
        assert!(toughness(&world, player) < before);
    }

    #[test]
    fn a_lava_worm_tunnels_unhurt_then_bursts_up_bites_and_can_be_hurt() {
        let (mut world, mut physics, player) = arena();
        let at = world.get::<&Position>(player).expect("player").0;
        let textures = Textures::default();
        let worm = spawn(
            &mut world,
            &mut physics,
            EnemyKind::LavaWorm,
            at + Vec2::new(400.0, 0.0),
        );
        let stage = |world: &World| world.get::<&Burrower>(worm).expect("worm").stage;
        tick_worms(
            &mut world,
            &textures,
            &mut Vec::new(),
            enemies::WORM_RISE + 0.01,
        );
        assert_eq!(stage(&world), WormStage::Up);
        tick_worms(
            &mut world,
            &textures,
            &mut Vec::new(),
            enemies::WORM_UP + 0.01,
        );
        assert_eq!(stage(&world), WormStage::Under);
        let full = world.get::<&Health>(worm).expect("worm").current;
        hit(&mut world, worm, 3);
        assert_eq!(
            world.get::<&Health>(worm).expect("worm").current,
            full,
            "untouchable below"
        );
        assert!(world.get::<&ContactDamage>(worm).is_err());

        world.get::<&mut Position>(worm).expect("worm").0 = at + Vec2::new(30.0, 0.0);
        tick_worms(&mut world, &textures, &mut Vec::new(), 0.01);
        assert_eq!(stage(&world), WormStage::Warning);
        let before = toughness(&world, player);
        tick_worms(
            &mut world,
            &textures,
            &mut Vec::new(),
            enemies::WORM_WARNING + 0.01,
        );
        assert_eq!(stage(&world), WormStage::Rising);
        assert!(toughness(&world, player) < before, "the eruption hurts");
        hit(&mut world, worm, 3);
        assert!(world.get::<&Health>(worm).expect("worm").current < full);
    }
}
