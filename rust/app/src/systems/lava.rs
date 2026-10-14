//! The lava biome's enemies: the ember rat that flares up and rushes, the
//! flame bat that swoops and rises from its ash, the magma slime that crusts
//! over and lobs magma, and the lava worm that bursts up from below.

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::audio::{AudioCommand, SoundEffect};
use crate::clip::Clip;
use crate::components::{
    Animation, AshPile, Burrower, Chase, Color, ContactDamage, DeathAsh, Dodge, Expiring, FallKind,
    FallingRock, Health, Hidden, Invulnerable, Kindling, Layer, Light, Lit, MagmaShell, Position,
    RockFall, RoomId, Sealed, Size, Spawning, Sprite, SwoopStage, Swooper, WormStage,
};
use crate::enemies::{self, EnemyKind, ProjectileKind};
use crate::geometry;
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::prefabs;
use crate::systems::damage::hurt_player;
use crate::systems::hostile::Hostile;

const EMBER_GLOW: [f32; 3] = [255.0, 120.0, 30.0];
const SWOOP_RANGE: f32 = 460.0;
const ASH_SIZE: Vec2 = Vec2::new(48.0, 18.0);
const MAGMA_BALL_SIZE: Vec2 = Vec2::new(34.0, 34.0);
const MAGMA_SHADOW: Vec2 = Vec2::new(64.0, 30.0);
const SPLASH_SIZE: Vec2 = Vec2::new(64.0, 64.0);
/// Where the worm's mouth is, from its centre, and how much floor it needs.
const WORM_MOUTH: Vec2 = Vec2::new(0.0, -34.0);
const WORM_FOOTPRINT: Vec2 = Vec2::new(64.0, 40.0);
const WORM_SPIT_CLIP: Clip = Clip {
    row: 1,
    frames: 4,
    frame_time: 0.1,
    looping: false,
    flipped: false,
};
const WORM_SINK_CLIP: Clip = Clip {
    row: 3,
    frames: 4,
    frame_time: 0.08,
    looping: false,
    flipped: false,
};
/// Only the glowing hole, before the worm comes up.
const WORM_HOLE_CLIP: Clip = Clip {
    row: 0,
    frames: 1,
    frame_time: 1.0,
    looping: true,
    flipped: false,
};
const SPLASH_CLIP: Clip = Clip {
    row: 1,
    frames: 4,
    frame_time: 0.07,
    looping: false,
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

/// A hit ember rat flares up, hisses and rushes the player.
pub(crate) fn tick_ember_rats(world: &mut World, sounds: &mut Vec<AudioCommand>, clock: f32) {
    let mut lit_now = Vec::new();
    for (entity, kindling, health, chase, color, position) in world
        .query_mut::<(
            Entity,
            &mut Kindling,
            &Health,
            &mut Chase,
            &mut Color,
            &Position,
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
        }
    }
    for (entity, at) in lit_now {
        let [red, green, blue] = EMBER_GLOW;
        let _ = world.insert_one(entity, Light::rgb(Vec2::ZERO, red, green, blue, 70.0, 1.0));
        sounds.push(AudioCommand::At(SoundEffect::HoundInhale, at));
    }
}

// ---------- flame bat ----------

/// Flame bats circle, flare up for a moment, then swoop fast through the player.
pub(crate) fn tick_swoopers(world: &mut World, hostile: &mut Hostile<'_>) {
    let sounds = &mut *hostile.sounds;
    let dt = hostile.dt;
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
pub(crate) fn tick_ashes(world: &mut World, hostile: &mut Hostile<'_>) {
    let physics = &mut *hostile.physics;
    let textures = hostile.textures;
    let sounds = &mut *hostile.sounds;
    let dt = hostile.dt;
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
        sounds.push(AudioCommand::At(SoundEffect::EnemyHit, at));
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
pub(crate) fn tick_magma_slimes(world: &mut World, hostile: &mut Hostile<'_>) {
    let textures = hostile.textures;
    let sounds = &mut *hostile.sounds;
    let dt = hostile.dt;
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
    world.spawn((
        Expiring,
        Position(ground),
        Size(SPLASH_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(tex::MAGMA_BALL), 0, 1, 4, 2),
        Animation::new(4, 2, SPLASH_CLIP),
        Layer(3),
        Light::rgb(Vec2::ZERO, 255.0, 130.0, 30.0, 50.0, 0.8),
    ));
    let Some((player, at, size)) = player else {
        return;
    };
    let reach = Vec2::splat(enemies::MORTAR_RADIUS * 2.0);
    if can_be_hit(world, player) && geometry::overlaps(ground, reach, at, size) {
        hurt_player(world, player, enemies::MORTAR_DAMAGE, sounds);
    }
}

// ---------- lava worm ----------

/// Lava worms come up out of the floor near the player, spit a few magma
/// shots, sink, and come up somewhere else. Only while up can they be hit.
pub(crate) fn tick_worms(world: &mut World, hostile: &mut Hostile<'_>) {
    let physics = &mut *hostile.physics;
    let navigation = hostile.navigation;
    let textures = hostile.textures;
    let sounds = &mut *hostile.sounds;
    let dt = hostile.dt;
    let player = player_at(world).map(|(_, at, _)| at);
    let mut changes: Vec<(Entity, WormStage, Vec2, Burrower)> = Vec::new();
    let mut shots: Vec<(Entity, Vec2)> = Vec::new();
    for (entity, worm, chase, position) in world
        .query_mut::<(Entity, &mut Burrower, &mut Chase, &Position)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
    {
        chase.speed = 0.0;
        worm.timer += dt;
        let next = match worm.stage {
            WormStage::Hidden => (worm.timer >= enemies::WORM_HIDDEN).then_some(WormStage::Showing),
            WormStage::Showing => (worm.timer >= enemies::WORM_SHOW).then_some(WormStage::Rising),
            WormStage::Rising => (worm.timer >= enemies::WORM_RISE).then_some(WormStage::Up),
            WormStage::Up => {
                worm.shot -= dt;
                if worm.shot <= 0.0 && worm.shots_left > 0 {
                    worm.shots_left -= 1;
                    worm.shot = enemies::WORM_SHOT_EVERY;
                    shots.push((entity, position.0));
                }
                (worm.shots_left == 0 && worm.shot <= 0.0).then_some(WormStage::Sinking)
            }
            WormStage::Sinking => (worm.timer >= enemies::WORM_SINK).then_some(WormStage::Hidden),
        };
        if let Some(stage) = next {
            worm.stage = stage;
            worm.timer = 0.0;
            if stage == WormStage::Up {
                worm.shots_left = enemies::WORM_SHOTS;
                worm.shot = 0.25;
            }
            if stage == WormStage::Showing {
                worm.emerged = worm.emerged.wrapping_add(1);
            }
            changes.push((entity, stage, position.0, *worm));
        }
    }
    for (entity, from) in shots {
        if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
            animation.play(WORM_SPIT_CLIP);
        }
        let Some(target) = player else {
            continue;
        };
        let mouth = from + WORM_MOUTH;
        sounds.push(AudioCommand::At(SoundEffect::SlimeSmash, mouth));
        prefabs::enemy_bullet(
            world,
            physics,
            ProjectileKind::Magma,
            mouth,
            target - mouth,
            textures,
        );
    }
    for (entity, stage, at, worm) in changes {
        match stage {
            WormStage::Sinking => {
                let _ = world.remove_one::<ContactDamage>(entity);
                let _ = world.insert_one(entity, Sealed);
                if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
                    animation.play(WORM_SINK_CLIP);
                }
                sounds.push(AudioCommand::At(SoundEffect::LeaperLand, at));
            }
            WormStage::Hidden => {
                physics.detach(world, entity);
                let _ = world.insert_one(entity, Hidden);
            }
            WormStage::Showing => {
                let spot = player.map_or(at, |target| {
                    emerge_spot(navigation, target, at, worm.emerged)
                });
                if let Ok(mut position) = world.get::<&mut Position>(entity) {
                    position.0 = spot;
                }
                let _ = world.remove_one::<Hidden>(entity);
                if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
                    animation.play(WORM_HOLE_CLIP);
                }
            }
            WormStage::Rising => {
                let size = world.get::<&Size>(entity).map_or(Vec2::ZERO, |size| size.0);
                let body = physics.add_enemy(entity, at, size);
                let _ = world.insert(entity, (body, ContactDamage(worm.damage)));
                let _ = world.remove_one::<Sealed>(entity);
                if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
                    animation.play(enemies::WORM_RISE_CLIP);
                }
                sounds.push(AudioCommand::At(SoundEffect::LeaperCrouch, at));
            }
            WormStage::Up => {}
        }
    }
}

/// Somewhere on open floor a fair way from the player, on a different side each time.
fn emerge_spot(navigation: &Navigation, player: Vec2, fallback: Vec2, emerged: u32) -> Vec2 {
    let turn = emerged.wrapping_mul(2_654_435_761) as f32 / u32::MAX as f32;
    for attempt in 0..8 {
        let angle = std::f32::consts::TAU * (turn + attempt as f32 * 0.37);
        for distance in [enemies::WORM_EMERGE_FAR, enemies::WORM_EMERGE_NEAR] {
            let spot = player + Vec2::from_angle(angle) * distance;
            if !navigation.is_blocked(spot, WORM_FOOTPRINT) {
                return spot;
            }
        }
    }
    fallback
}

#[cfg(test)]
mod tests {
    use crate::components::{Armor, Enemy, Kindling};
    use crate::dungeon::Theme;
    use crate::rng::Rng;
    use crate::systems::damage::{Damage, DamageSource, deal};

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
    fn a_hit_ember_rat_flares_up_and_rushes_without_blowing_up() {
        let (mut world, mut physics, player) = arena();
        let at = world.get::<&Position>(player).expect("player").0;
        let rat = spawn(
            &mut world,
            &mut physics,
            EnemyKind::EmberRat,
            at + Vec2::new(300.0, 0.0),
        );
        let calm = world.get::<&Chase>(rat).expect("chase").speed;
        tick_ember_rats(&mut world, &mut Vec::new(), 0.0);
        assert!(!world.get::<&Kindling>(rat).expect("kindling").lit);
        hit(&mut world, rat, 1);
        tick_ember_rats(&mut world, &mut Vec::new(), 0.0);
        assert!(world.get::<&Kindling>(rat).expect("kindling").lit);
        assert!(world.get::<&Chase>(rat).expect("chase").speed > calm * 2.0);
        assert!(world.get::<&Light>(rat).is_ok());

        world.get::<&mut Position>(rat).expect("rat").0 = at;
        tick_ember_rats(&mut world, &mut Vec::new(), 0.1);
        assert!(
            world.get::<&Health>(rat).expect("rat").current > 0,
            "it bites, it does not burst"
        );
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
        crate::systems::hostile::tick_for_test(
            &mut world,
            &mut physics,
            enemies::SWOOP_EVERY * 2.0,
            tick_swoopers,
        );
        assert_eq!(
            world.get::<&Swooper>(bat).expect("bat").stage,
            SwoopStage::Windup
        );
        assert_eq!(world.get::<&Chase>(bat).expect("bat").speed, 0.0);
        crate::systems::hostile::tick_for_test(
            &mut world,
            &mut physics,
            enemies::SWOOP_WINDUP + 0.01,
            tick_swoopers,
        );
        assert_eq!(
            world.get::<&Swooper>(bat).expect("bat").stage,
            SwoopStage::Dive
        );
        assert!(world.get::<&Chase>(bat).expect("bat").speed > base * 3.0);
        crate::systems::hostile::tick_for_test(
            &mut world,
            &mut physics,
            enemies::SWOOP_DIVE + 0.01,
            tick_swoopers,
        );
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
        crate::systems::hostile::tick_for_test(
            &mut world,
            &mut physics,
            enemies::ASH_REBIRTH + 0.1,
            tick_ashes,
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
        crate::systems::hostile::tick_for_test(&mut world, &mut physics, 0.1, tick_ashes);
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
        crate::systems::hostile::tick_for_test(
            &mut world,
            &mut physics,
            enemies::MAGMA_TURN + 0.01,
            tick_magma_slimes,
        );
        assert!(world.get::<&MagmaShell>(slime).expect("slime").crusted);
        hit(&mut world, slime, 2);
        assert_eq!(world.get::<&Health>(slime).expect("slime").current, full);
        crate::systems::hostile::tick_for_test(
            &mut world,
            &mut physics,
            enemies::MAGMA_TURN + 0.01,
            tick_magma_slimes,
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
        crate::systems::hostile::tick_for_test(
            &mut world,
            &mut physics,
            enemies::MORTAR_EVERY * 2.0,
            tick_magma_slimes,
        );
        let target = world
            .query::<(&RockFall, &Position)>()
            .iter()
            .find(|(fall, _)| fall.kind == FallKind::Magma)
            .map(|(_, position)| position.0)
            .expect("a lobbed ball");
        assert_eq!(target, at);
        let before = toughness(&world, player);
        crate::systems::hostile::tick_for_test(
            &mut world,
            &mut physics,
            enemies::MORTAR_FLIGHT + 0.1,
            crate::systems::tick_rock_falls,
        );
        assert!(toughness(&world, player) < before);
    }

    fn worm_stage(world: &World, worm: Entity) -> WormStage {
        world.get::<&Burrower>(worm).expect("worm").stage
    }

    fn step(world: &mut World, physics: &mut Physics, seconds: f32) {
        crate::systems::hostile::tick_for_test(world, physics, seconds, tick_worms);
    }

    fn magma_shots(world: &World) -> usize {
        world
            .query::<&crate::components::EnemyBullet>()
            .iter()
            .count()
    }

    #[test]
    fn a_lava_worm_rises_spits_magma_at_the_player_and_comes_up_elsewhere() {
        let (mut world, mut physics, player) = arena();
        let at = world.get::<&Position>(player).expect("player").0;
        let start = at + Vec2::new(400.0, 0.0);
        let worm = spawn(&mut world, &mut physics, EnemyKind::LavaWorm, start);
        step(&mut world, &mut physics, enemies::WORM_RISE + 0.01);
        assert_eq!(worm_stage(&world, worm), WormStage::Up);
        for _ in 0..enemies::WORM_SHOTS {
            step(&mut world, &mut physics, enemies::WORM_SHOT_EVERY);
        }
        assert_eq!(magma_shots(&world), enemies::WORM_SHOTS as usize);
        step(&mut world, &mut physics, enemies::WORM_SHOT_EVERY);
        assert_eq!(worm_stage(&world, worm), WormStage::Sinking);
        step(&mut world, &mut physics, enemies::WORM_SINK + 0.01);
        assert_eq!(worm_stage(&world, worm), WormStage::Hidden);
        assert!(world.get::<&Hidden>(worm).is_ok());
        assert!(
            world.get::<&crate::components::Body>(worm).is_err(),
            "nothing to shoot at below"
        );

        step(&mut world, &mut physics, enemies::WORM_HIDDEN + 0.01);
        assert_eq!(worm_stage(&world, worm), WormStage::Showing);
        let spot = world.get::<&Position>(worm).expect("worm").0;
        assert_ne!(spot, start, "it comes up somewhere else");
        let away = spot.distance(at);
        assert!((enemies::WORM_EMERGE_NEAR - 1.0..=enemies::WORM_EMERGE_FAR + 1.0).contains(&away));
        step(&mut world, &mut physics, enemies::WORM_SHOW + 0.01);
        assert_eq!(worm_stage(&world, worm), WormStage::Rising);
        assert!(world.get::<&crate::components::Body>(worm).is_ok());
    }

    #[test]
    fn a_lava_worm_can_only_be_hurt_while_it_is_up() {
        let (mut world, mut physics, _player) = arena();
        let worm = spawn(
            &mut world,
            &mut physics,
            EnemyKind::LavaWorm,
            Vec2::new(900.0, 500.0),
        );
        step(&mut world, &mut physics, enemies::WORM_RISE + 0.01);
        let full = world.get::<&Health>(worm).expect("worm").current;
        hit(&mut world, worm, 2);
        assert!(world.get::<&Health>(worm).expect("worm").current < full);
        let wounded = world.get::<&Health>(worm).expect("worm").current;
        for _ in 0..=enemies::WORM_SHOTS {
            step(&mut world, &mut physics, enemies::WORM_SHOT_EVERY);
        }
        assert_eq!(worm_stage(&world, worm), WormStage::Sinking);
        hit(&mut world, worm, 2);
        assert_eq!(world.get::<&Health>(worm).expect("worm").current, wounded);
    }
}
