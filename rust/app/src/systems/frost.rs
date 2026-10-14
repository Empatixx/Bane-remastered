//! The ice biome's tricks: chilling bites and snowballs, icicles dropped by
//! bats and snow slimes breaking apart.

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::components::{
    CHILL_SECONDS, Chilled, Color, DeathSplit, Expiring, Frozen, Health, IcicleDropper, Layer,
    Position, RockFall, RoomId, Size, Spawning, Sprite,
};
use crate::enemies;
use crate::physics::Physics;
use crate::prefabs;

const ICICLE_SIZE: Vec2 = Vec2::new(30.0, 45.0);
const ICICLE_SHADOW: Vec2 = Vec2::new(56.0, 26.0);
const ICICLE_HEIGHT: f32 = 340.0;

/// One more chill on the player: slower for a while, frozen solid after a few in a row.
pub(crate) fn chill_player(world: &mut World, player: Entity) {
    if world.get::<&Frozen>(player).is_ok() {
        return;
    }
    let stacks = world
        .get::<&Chilled>(player)
        .map_or(0, |chilled| chilled.stacks)
        + 1;
    if stacks >= enemies::PLAYER_FREEZE_STACKS {
        let _ = world.remove_one::<Chilled>(player);
        let _ = world.insert_one(player, Frozen::new(enemies::PLAYER_FREEZE_SECONDS));
    } else {
        let _ = world.insert_one(
            player,
            Chilled {
                remaining: CHILL_SECONDS,
                stacks,
            },
        );
    }
}

/// Icicle bats near the player drop an icicle on where the player stands.
pub(crate) fn tick_icicle_bats(world: &mut World, textures: &Textures, dt: f32) {
    let Some((_, player, _)) = super::golem::player_body(world) else {
        return;
    };
    let mut drops = Vec::new();
    for (position, dropper, room) in world
        .query_mut::<(&Position, &mut IcicleDropper, Option<&RoomId>)>()
        .without::<&Frozen>()
        .without::<&Spawning>()
        .without::<&Expiring>()
    {
        dropper.remaining -= dt;
        if dropper.remaining <= 0.0 && position.0.distance(player) <= enemies::ICICLE_RANGE {
            dropper.remaining = enemies::ICICLE_COOLDOWN;
            drops.push(room.copied());
        }
    }
    for room in drops {
        drop_icicle(world, textures, player, room);
    }
}

/// The shadow shows where it will land; the icicle falls into it.
pub(crate) fn drop_icicle(
    world: &mut World,
    textures: &Textures,
    ground: Vec2,
    room: Option<RoomId>,
) {
    let icicle = world.spawn((
        crate::components::FallingRock,
        Position(ground - Vec2::new(0.0, ICICLE_HEIGHT)),
        Size(ICICLE_SIZE),
        Color(Vec4::ONE),
        Sprite::whole(textures.get(tex::FALLING_ICICLE)),
        Layer(4),
    ));
    world.spawn((
        RockFall {
            remaining: enemies::ICICLE_FALL,
            duration: enemies::ICICLE_FALL,
            rock: icicle,
            room,
            kind: crate::components::FallKind::Icicle,
            from: None,
        },
        Position(ground),
        Size(ICICLE_SHADOW),
        Color(Vec4::ONE),
        Sprite::whole(textures.get(tex::ROCK_SHADOW)),
        Layer(-1),
    ));
}

/// Snow slimes break into slimelings that scatter a little.
pub(crate) fn death_splits(world: &mut World, physics: &mut Physics, textures: &Textures) {
    let dying: Vec<(Entity, Vec2, DeathSplit, Option<RoomId>)> = world
        .query::<(Entity, &Position, &DeathSplit, &Health, Option<&RoomId>)>()
        .without::<&Expiring>()
        .iter()
        .filter(|(.., health, _)| health.current <= 0)
        .map(|(entity, position, split, _, room)| (entity, position.0, *split, room.copied()))
        .collect();
    for (entity, at, split, room) in dying {
        let _ = world.remove_one::<DeathSplit>(entity);
        for index in 0..split.count {
            let angle = std::f32::consts::TAU * index as f32 / split.count as f32;
            let offset = Vec2::from_angle(angle) * 26.0;
            prefabs::hatchling(world, physics, split.kind, at + offset, room, textures);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{ContactDamage, FallingRock};
    use crate::dungeon::Theme;
    use crate::enemies::{EnemyKind, ProjectileKind};
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

    fn health(world: &World, entity: Entity) -> i32 {
        world.get::<&Health>(entity).expect("health").current
    }

    /// Health and armor together: a hit takes armor first.
    fn toughness(world: &World, entity: Entity) -> i32 {
        health(world, entity)
            + world
                .get::<&crate::components::Armor>(entity)
                .map_or(0, |armor| armor.current)
    }

    fn kill(world: &mut World, entity: Entity) {
        world.get::<&mut Health>(entity).expect("health").current = 0;
    }

    #[test]
    fn the_ice_biome_has_its_own_rat_bat_and_slime_and_the_others_stay_away() {
        let mut rng = Rng::new(9);
        let ice: std::collections::HashSet<EnemyKind> = (0..2000)
            .map(|_| crate::enemies::pick(3, Theme::Ice, &mut rng))
            .collect();
        for kind in [
            EnemyKind::FrostRat,
            EnemyKind::IcicleBat,
            EnemyKind::SnowSlime,
        ] {
            assert!(ice.contains(&kind), "{kind:?}");
        }
        for kind in [
            EnemyKind::Rat,
            EnemyKind::Bat,
            EnemyKind::Slime,
            EnemyKind::SnowSlimeling,
        ] {
            assert!(!ice.contains(&kind), "{kind:?}");
        }
        let stone: std::collections::HashSet<EnemyKind> = (0..2000)
            .map(|_| crate::enemies::pick(3, Theme::Stone, &mut rng))
            .collect();
        assert!(stone.contains(&EnemyKind::Rat));
        assert!(!stone.contains(&EnemyKind::FrostRat));
    }

    #[test]
    fn chills_in_a_row_slow_the_player_and_then_freeze_them() {
        let (mut world, _physics, player) = arena();
        chill_player(&mut world, player);
        chill_player(&mut world, player);
        assert_eq!(world.get::<&Chilled>(player).expect("chilled").stacks, 2);
        assert!(world.get::<&Frozen>(player).is_err());
        chill_player(&mut world, player);
        assert!(world.get::<&Frozen>(player).is_ok());
        assert!(world.get::<&Chilled>(player).is_err());
    }

    #[test]
    fn a_frost_rat_bite_hurts_and_chills() {
        let (mut world, mut physics, player) = arena();
        let at = world.get::<&Position>(player).expect("player").0;
        let rat = prefabs::ready_enemy(&mut world, &mut physics, EnemyKind::FrostRat, at);
        assert!(world.get::<&ContactDamage>(rat).is_ok());
        let before = toughness(&world, player);
        assert!(crate::systems::damage::contact_damage(
            &mut world,
            &mut Vec::new()
        ));
        assert!(toughness(&world, player) < before);
        assert!(world.get::<&Chilled>(player).is_ok());
    }

    #[test]
    fn snowballs_chill_and_green_slime_balls_do_not() {
        let (mut world, mut physics, _player) = arena();
        let snow = prefabs::enemy_bullet(
            &mut world,
            &mut physics,
            ProjectileKind::Snow,
            Vec2::ZERO,
            Vec2::X,
            &Textures::default(),
        );
        let green = prefabs::enemy_bullet(
            &mut world,
            &mut physics,
            ProjectileKind::Green,
            Vec2::ZERO,
            Vec2::X,
            &Textures::default(),
        );
        assert!(world.get::<&crate::components::Chills>(snow).is_ok());
        assert!(world.get::<&crate::components::Chills>(green).is_err());
    }

    #[test]
    fn a_dead_snow_slime_breaks_in_two() {
        let (mut world, mut physics, _player) = arena();
        let slime = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::SnowSlime,
            Vec2::new(900.0, 100.0),
        );
        kill(&mut world, slime);
        death_splits(&mut world, &mut physics, &Textures::default());
        let slimelings = world
            .query::<&crate::components::Enemy>()
            .iter()
            .filter(|enemy| enemy.kind == EnemyKind::SnowSlimeling)
            .count();
        assert_eq!(slimelings, enemies::SNOW_SPLIT as usize);
    }

    #[test]
    fn an_icicle_bat_near_the_player_drops_an_icicle_that_hurts_and_chills_on_landing() {
        let (mut world, mut physics, player) = arena();
        let at = world.get::<&Position>(player).expect("player").0;
        prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::IcicleBat,
            at + Vec2::new(200.0, 0.0),
        );
        tick_icicle_bats(
            &mut world,
            &Textures::default(),
            enemies::ICICLE_COOLDOWN * 2.0,
        );
        assert_eq!(world.query::<&FallingRock>().iter().count(), 1);

        let before = toughness(&world, player);
        crate::systems::tick_rock_falls(
            &mut world,
            &mut physics,
            &crate::navigation::Navigation::default(),
            &Textures::default(),
            &mut Vec::new(),
            enemies::ICICLE_FALL + 0.1,
        );
        assert_eq!(world.query::<&FallingRock>().iter().count(), 0);
        assert!(toughness(&world, player) < before);
        assert!(world.get::<&Chilled>(player).is_ok());
        assert_eq!(
            world
                .query::<&crate::components::Enemy>()
                .iter()
                .filter(|enemy| enemy.kind == EnemyKind::MiniGolem)
                .count(),
            0,
            "icicles never hatch golems"
        );
    }
}
