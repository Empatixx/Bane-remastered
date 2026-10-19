use std::f32::consts::TAU;

use glam::Vec2;
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Armor, ArtefactDrop, Color, Despawn, Enemy, Expiring, Gone, GunDrop, HOVER_CYCLE, Health,
    Hover, InRange, Loadout, NoDrop, Outline, Pet, Pickup, Player, Position, Shop, Size, Wallet,
};
use crate::geometry;
use crate::items::{
    AmmoBox, BLINK_PERIOD, BLINK_SECONDS, DROP_ROLL_RANGE, ItemKind, Vital, drop_threshold_reached,
};
use crate::prefabs;
use crate::systems::loadout;

const BURST_ITEMS: usize = 5;
const DROP_OUTLINE: glam::Vec4 = glam::Vec4::new(1.0, 1.0, 0.0, 1.0);
const BURST_SPEED: f32 = 333.0;

pub(crate) fn hover_drops(world: &mut World, dt: f32) {
    let fresh: Vec<Entity> = world
        .query::<Entity>()
        .with::<hecs::Or<hecs::Or<&Pickup, &GunDrop>, &ArtefactDrop>>()
        .without::<&Hover>()
        .iter()
        .collect();
    for entity in fresh {
        let cycle = crate::rng::cosmetic();
        let _ = world.insert_one(entity, Hover { cycle });
    }
    for hover in world.query_mut::<&mut Hover>() {
        hover.cycle = (hover.cycle + dt / HOVER_CYCLE).rem_euclid(1.0);
    }
}

pub(crate) fn outline_drops(world: &mut World) {
    let drops: Vec<(Entity, bool)> = world
        .query::<(Entity, Option<&InRange>)>()
        .with::<hecs::Or<&GunDrop, &ArtefactDrop>>()
        .iter()
        .map(|(entity, in_range)| (entity, in_range.is_some()))
        .collect();
    for (entity, near) in drops {
        if near {
            let _ = world.insert_one(entity, Outline(DROP_OUTLINE));
        } else {
            let _ = world.remove_one::<Outline>(entity);
        }
    }
}

/// Turns a rolled ammo box that no held gun could use into the box the
/// emptiest held gun needs, so rifle rounds, rockets and batteries drop for
/// the guns that take them.
pub(crate) fn common_drop(world: &World, kind: ItemKind) -> ItemKind {
    let ItemKind::Ammo(ammo) = kind else {
        return kind;
    };
    let mut loadouts = world.query::<&Loadout>().with::<&Player>();
    let Some(loadout) = loadouts.iter().next() else {
        return kind;
    };
    if loadout.accepts_ammo(ammo) && loadout.feeds_a_held_gun(ammo) {
        return kind;
    }
    if let Some(class) = loadout::neediest_ammo(loadout) {
        return ItemKind::Ammo(AmmoBox::feeding(class));
    }
    if loadout.accepts_ammo(ammo) {
        kind
    } else if loadout.accepts_ammo(AmmoBox::Pistol) {
        ItemKind::Ammo(AmmoBox::Pistol)
    } else {
        ItemKind::Ammo(AmmoBox::Shotgun)
    }
}

pub(crate) fn can_use(world: &World, item: ItemKind) -> bool {
    match item {
        ItemKind::Coin => true,
        ItemKind::Ammo(ammo) => world
            .query::<&Loadout>()
            .with::<&Player>()
            .iter()
            .next()
            .is_some_and(|loadout| loadout::wants_ammo(loadout, ammo)),
        ItemKind::Potion(Vital::Health, _) => world
            .query::<&Health>()
            .with::<&Player>()
            .iter()
            .next()
            .is_some_and(|health| health.current < health.max),
        ItemKind::Potion(Vital::Armor, _) => world
            .query::<&Armor>()
            .with::<&Player>()
            .iter()
            .next()
            .is_some_and(|armor| armor.current < armor.max),
    }
}

pub(crate) fn use_item(world: &mut World, item: ItemKind, sounds: &mut Vec<AudioCommand>) {
    super::stats::record(world, super::stats::Stat::Item, 1);
    match item {
        ItemKind::Coin => {
            for wallet in world.query_mut::<&mut Wallet>().with::<&Player>() {
                wallet.coins += 1;
            }
            super::stats::record(world, super::stats::Stat::Gold, 1);
            sounds.push(AudioCommand::Effect(SoundEffect::Coin));
        }
        ItemKind::Ammo(ammo) => {
            for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
                loadout::add_ammo(loadout, ammo);
            }
            sounds.push(AudioCommand::Effect(SoundEffect::Pickup));
        }
        ItemKind::Potion(vital, _) => {
            let amount = item.recover_amount();
            match vital {
                Vital::Health => {
                    for health in world.query_mut::<&mut Health>().with::<&Player>() {
                        health.current = (health.current + amount).min(health.max);
                    }
                }
                Vital::Armor => {
                    for armor in world.query_mut::<&mut Armor>().with::<&Player>() {
                        armor.current = (armor.current + amount).min(armor.max);
                    }
                }
            }
            sounds.push(AudioCommand::Effect(SoundEffect::Heal));
            sounds.push(AudioCommand::Effect(SoundEffect::Pickup));
        }
    }
}

pub(crate) fn collect_items(world: &mut World, sounds: &mut Vec<AudioCommand>) {
    let collectors: Vec<(Vec2, Vec2)> = world
        .query::<(&Position, &Size)>()
        .with::<&Player>()
        .iter()
        .chain(world.query::<(&Position, &Size)>().with::<&Pet>().iter())
        .map(|(position, size)| (position.0, size.0))
        .collect();
    if collectors.is_empty() {
        return;
    }
    let touched: Vec<(Entity, ItemKind)> = world
        .query::<(Entity, &Pickup, &Position, &Size)>()
        .without::<&Expiring>()
        .without::<&Shop>()
        .iter()
        .filter(|(_, _, position, size)| {
            collectors
                .iter()
                .any(|(at, extent)| geometry::overlaps(*at, *extent, position.0, size.0))
        })
        .map(|(entity, pickup, _, _)| (entity, pickup.0))
        .collect();
    for (entity, item) in touched {
        if can_use(world, item) {
            use_item(world, item, sounds);
            let _ = world.despawn(entity);
        }
    }
}

pub(crate) fn age_items(world: &mut World, dt: f32) {
    let mut gone = Vec::new();
    for (entity, despawn, color) in world.query_mut::<(Entity, &mut Despawn, &mut Color)>() {
        despawn.remaining -= dt;
        if despawn.remaining <= 0.0 {
            gone.push(entity);
        } else if despawn.remaining < BLINK_SECONDS {
            let phase = (despawn.remaining / BLINK_PERIOD) as usize;
            color.0.w = if phase.is_multiple_of(2) { 1.0 } else { 0.25 };
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

/// Pays out the enemy corpses that finished this frame (`Gone`): a kill in the run stats and,
/// now and then, a common drop where the corpse lay.
pub(crate) fn enemy_drops(
    world: &mut World,
    kills_without_drop: &mut u32,
    textures: &Textures,
    mut roll: impl FnMut(usize) -> usize,
) {
    let dead: Vec<Vec2> = world
        .query::<&Position>()
        .with::<(&Enemy, &Gone)>()
        .without::<&NoDrop>()
        .iter()
        .map(|position| position.0)
        .collect();
    let killed: Vec<crate::enemies::EnemyKind> = world
        .query::<&Enemy>()
        .with::<&Gone>()
        .without::<&crate::components::Minion>()
        .iter()
        .map(|enemy| enemy.kind)
        .collect();
    for kind in killed {
        super::stats::record(world, super::stats::Stat::Kill(kind), 1);
    }
    for position in dead {
        *kills_without_drop += 1;
        if drop_threshold_reached(*kills_without_drop, roll(DROP_ROLL_RANGE)) {
            *kills_without_drop = 0;
            let kind = common_drop(world, ItemKind::COMMON[roll(ItemKind::COMMON.len())]);
            prefabs::item(world, kind, position, true, Vec2::ZERO, textures);
        }
    }
}

pub(crate) fn burst_items(
    world: &mut World,
    center: Vec2,
    textures: &Textures,
    mut roll: impl FnMut(usize) -> usize,
) {
    for index in 0..BURST_ITEMS {
        let angle = TAU * (index + 1) as f32 / BURST_ITEMS as f32;
        let kind = common_drop(world, ItemKind::COMMON[roll(ItemKind::COMMON.len())]);
        let velocity = Vec2::new(angle.cos(), angle.sin()) * BURST_SPEED;
        prefabs::item(world, kind, center, false, velocity, textures);
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{Animation, Velocity};
    use crate::items::{AmmoBox, DESPAWN_SECONDS, PotionSize};
    use crate::physics::Physics;

    use super::*;

    fn arena() -> (World, Textures) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(&mut world, &mut physics, Vec2::new(100.0, 100.0), &textures);
        (world, textures)
    }

    fn coins(world: &World) -> i32 {
        world
            .query::<&Wallet>()
            .with::<&Player>()
            .iter()
            .next()
            .map_or(-1, |wallet| wallet.coins)
    }

    #[test]
    fn walking_over_a_coin_picks_it_up() {
        let (mut world, textures) = arena();
        prefabs::item(
            &mut world,
            ItemKind::Coin,
            Vec2::new(100.0, 100.0),
            true,
            Vec2::ZERO,
            &textures,
        );
        prefabs::item(
            &mut world,
            ItemKind::Coin,
            Vec2::new(900.0, 900.0),
            true,
            Vec2::ZERO,
            &textures,
        );
        let mut sounds = Vec::new();

        collect_items(&mut world, &mut sounds);

        assert_eq!(coins(&world), 1);
        assert_eq!(world.query::<&Pickup>().iter().count(), 1);
        assert_eq!(sounds, vec![AudioCommand::Effect(SoundEffect::Coin)]);
    }

    #[test]
    fn a_potion_stays_on_the_floor_while_you_are_at_full_health() {
        let (mut world, textures) = arena();
        let potion = ItemKind::Potion(Vital::Health, PotionSize::Small);
        prefabs::item(
            &mut world,
            potion,
            Vec2::new(100.0, 100.0),
            true,
            Vec2::ZERO,
            &textures,
        );
        collect_items(&mut world, &mut Vec::new());
        assert_eq!(world.query::<&Pickup>().iter().count(), 1);

        for health in world.query_mut::<&mut Health>().with::<&Player>() {
            health.current = 3;
        }
        collect_items(&mut world, &mut Vec::new());
        assert_eq!(world.query::<&Pickup>().iter().count(), 0);
        let health = world
            .query::<&Health>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|h| h.current);
        assert_eq!(health, Some(4));
    }

    #[test]
    fn a_large_armor_potion_restores_four_but_never_above_the_maximum() {
        let (mut world, textures) = arena();
        for armor in world.query_mut::<&mut Armor>().with::<&Player>() {
            armor.current = 0;
        }
        let potion = ItemKind::Potion(Vital::Armor, PotionSize::Large);
        prefabs::item(
            &mut world,
            potion,
            Vec2::new(100.0, 100.0),
            true,
            Vec2::ZERO,
            &textures,
        );
        collect_items(&mut world, &mut Vec::new());
        let armor = world
            .query::<&Armor>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|a| (a.current, a.max));
        assert_eq!(
            armor.map(|(current, max)| current.min(max)),
            Some(4.min(armor.map_or(0, |a| a.1)))
        );
    }

    #[test]
    fn ammo_boxes_are_only_taken_when_a_gun_needs_them() {
        let (mut world, textures) = arena();
        let ammo = ItemKind::Ammo(AmmoBox::Shotgun);
        prefabs::item(
            &mut world,
            ammo,
            Vec2::new(100.0, 100.0),
            true,
            Vec2::ZERO,
            &textures,
        );
        collect_items(&mut world, &mut Vec::new());
        assert_eq!(world.query::<&Pickup>().iter().count(), 1);

        loadout::give_a_shotgun(&mut world);
        for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
            loadout.switch_to(1);
            if let Some(weapon) = loadout.current_mut() {
                weapon.ammo = 10;
            }
        }
        collect_items(&mut world, &mut Vec::new());
        assert_eq!(world.query::<&Pickup>().iter().count(), 0);
        let ammo = world
            .query::<&Loadout>()
            .with::<&Player>()
            .iter()
            .next()
            .and_then(|loadout| loadout.current().map(|w| w.ammo));
        assert_eq!(ammo, Some(22));
    }

    #[test]
    fn dropped_items_blink_and_vanish_after_ten_seconds() {
        let (mut world, textures) = arena();
        let coin = prefabs::item(
            &mut world,
            ItemKind::Coin,
            Vec2::new(900.0, 900.0),
            true,
            Vec2::ZERO,
            &textures,
        );

        age_items(&mut world, DESPAWN_SECONDS - 1.0);
        let faded: Vec<f32> = (0..10)
            .map(|_| {
                age_items(&mut world, 0.1);
                world.get::<&Color>(coin).map_or(0.0, |c| c.0.w)
            })
            .collect();
        assert!(faded.contains(&1.0) && faded.contains(&0.25));

        age_items(&mut world, 0.5);
        assert!(world.get::<&Pickup>(coin).is_err());
    }

    #[test]
    fn a_pistol_only_loadout_never_gets_a_shotgun_box_from_drops_or_chests() {
        let (mut world, textures) = arena();
        let shotgun_roll = |bound: usize| {
            assert_eq!(bound, ItemKind::COMMON.len());
            2
        };
        burst_items(&mut world, Vec2::new(500.0, 500.0), &textures, shotgun_roll);
        let before = world.query::<&Pickup>().iter().count();
        assert!(
            world
                .query::<&Pickup>()
                .iter()
                .any(|pickup| pickup.0 == ItemKind::Ammo(AmmoBox::Shotgun))
        );

        for loadout in world.query_mut::<&mut Loadout>() {
            loadout::restrict_to(loadout, Some(crate::guns::GunKind::Pistol));
        }
        burst_items(&mut world, Vec2::new(500.0, 500.0), &textures, shotgun_roll);

        let kinds: Vec<ItemKind> = world
            .query::<&Pickup>()
            .iter()
            .map(|pickup| pickup.0)
            .collect();
        assert_eq!(kinds.len(), before + 5);
        assert_eq!(
            kinds
                .iter()
                .filter(|k| **k == ItemKind::Ammo(AmmoBox::Shotgun))
                .count(),
            before
        );
        assert_eq!(
            kinds
                .iter()
                .filter(|k| **k == ItemKind::Ammo(AmmoBox::Pistol))
                .count(),
            5
        );
    }

    #[test]
    fn items_from_chests_never_despawn() {
        let (mut world, textures) = arena();
        burst_items(&mut world, Vec2::new(500.0, 500.0), &textures, |bound| {
            bound - 1
        });
        assert_eq!(world.query::<&Pickup>().iter().count(), 5);
        assert_eq!(world.query::<&Despawn>().iter().count(), 0);
        let moving = world
            .query::<&Velocity>()
            .with::<&Pickup>()
            .iter()
            .filter(|v| v.0.length() > 300.0)
            .count();
        assert_eq!(moving, 5);
    }

    #[test]
    fn every_fifth_kill_drops_something_and_minions_never_do() {
        let (mut world, mut physics) = (World::new(), Physics::new());
        let textures = Textures::default();
        let mut kills = 0;
        let mut dying = Vec::new();
        for index in 0..6 {
            let rat = prefabs::ready_enemy(
                &mut world,
                &mut physics,
                crate::enemies::EnemyKind::Rat,
                Vec2::new(100.0 * index as f32, 0.0),
            );
            let _ = world.insert_one(rat, Expiring);
            if let Ok(mut animation) = world.get::<&mut Animation>(rat) {
                animation.play(crate::enemies::EnemyKind::Rat.spec().death_clip());
                animation.advance(5.0);
            }
            dying.push(rat);
        }
        let _ = world.insert_one(dying[0], NoDrop);
        crate::systems::mark_gone(&mut world);

        enemy_drops(&mut world, &mut kills, &textures, |_| 0);

        assert_eq!(world.query::<&Pickup>().iter().count(), 1);
        assert_eq!(kills, 0);
    }

    #[test]
    fn a_boss_that_finishes_dying_counts_as_a_kill_of_its_kind() {
        let (mut world, textures) = arena();
        let mut physics = Physics::new();
        let golem = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Golem,
            Vec2::new(400.0, 0.0),
        );
        let _ = world.insert_one(golem, Expiring);
        if let Ok(mut animation) = world.get::<&mut Animation>(golem) {
            animation.play(crate::enemies::EnemyKind::Golem.spec().death_clip());
            animation.advance(5.0);
        }
        crate::systems::mark_gone(&mut world);

        enemy_drops(&mut world, &mut 0, &textures, |_| 0);

        let stats = super::super::stats::current(&world);
        assert_eq!(stats.kills_of(crate::enemies::EnemyKind::Golem), 1);
        assert_eq!(stats.kills, 1);
    }

    #[test]
    fn picking_up_a_coin_adds_to_gold() {
        let (mut world, _) = arena();
        use_item(&mut world, ItemKind::Coin, &mut Vec::new());
        assert_eq!(super::super::stats::current(&world).gold, 1);
    }

    #[test]
    fn a_drop_is_outlined_in_yellow_only_while_the_player_is_close() {
        let mut world = World::new();
        let near = world.spawn((
            ArtefactDrop {
                kind: crate::artefacts::ArtefactKind::RingOfDeath,
            },
            InRange,
        ));
        let far = world.spawn((ArtefactDrop {
            kind: crate::artefacts::ArtefactKind::RingOfDeath,
        },));

        outline_drops(&mut world);
        assert!(world.get::<&Outline>(near).is_ok());
        assert!(world.get::<&Outline>(far).is_err());

        let _ = world.remove_one::<InRange>(near);
        outline_drops(&mut world);
        assert!(world.get::<&Outline>(near).is_err());
    }

    #[test]
    fn drops_start_at_a_random_phase_and_keep_bobbing() {
        let mut world = World::new();
        let drop = world.spawn((ArtefactDrop {
            kind: crate::artefacts::ArtefactKind::RingOfDeath,
        },));
        let other = world.spawn((Position(Vec2::ZERO),));

        hover_drops(&mut world, 0.0);
        assert!(world.get::<&Hover>(drop).is_ok());
        assert!(world.get::<&Hover>(other).is_err());

        let before = world.get::<&Hover>(drop).map(|h| h.cycle).unwrap_or(0.0);
        hover_drops(&mut world, 0.125);
        let after = world.get::<&Hover>(drop).map(|h| h.cycle).unwrap_or(0.0);
        let moved = (after - before).rem_euclid(1.0);
        assert!((moved - 0.25).abs() < 0.001, "{moved}");
    }

    #[test]
    fn a_laser_gun_that_runs_low_gets_battery_cells_from_drops() {
        let (mut world, _textures) = arena();
        for loadout in world.query_mut::<&mut Loadout>() {
            loadout.clear();
            let mut tesla = crate::components::Weapon::new(crate::guns::GunKind::TeslaGun);
            tesla.clip = 0;
            loadout::add_gun(loadout, tesla);
        }
        assert_eq!(
            common_drop(&world, ItemKind::Ammo(AmmoBox::Shotgun)),
            ItemKind::Ammo(AmmoBox::Energy)
        );
        assert_eq!(common_drop(&world, ItemKind::Coin), ItemKind::Coin);
    }
}
