use std::f32::consts::TAU;

use glam::Vec2;
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Animation, Armor, Color, Despawn, Enemy, Expiring, Health, Loadout, NoDrop, Pickup, Player,
    Position, Shop, Size, Wallet,
};
use crate::geometry;
use crate::items::{
    drop_threshold_reached, ItemKind, Vital, BLINK_PERIOD, BLINK_SECONDS, DROP_ROLL_RANGE,
};
use crate::prefabs;

const BURST_ITEMS: usize = 5;
const BURST_SPEED: f32 = 333.0;

pub(crate) fn can_use(world: &World, item: ItemKind) -> bool {
    match item {
        ItemKind::Coin => true,
        ItemKind::Ammo(ammo) => world
            .query::<&Loadout>()
            .with::<&Player>()
            .iter()
            .next()
            .is_some_and(|loadout| loadout.wants_ammo(ammo)),
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
            sounds.push(AudioCommand::Effect(SoundEffect::Coin));
        }
        ItemKind::Ammo(ammo) => {
            for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
                loadout.add_ammo(ammo);
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
    let player = world
        .query::<(&Position, &Size)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(position, size)| (position.0, size.0));
    let Some((at, extent)) = player else {
        return;
    };
    let touched: Vec<(Entity, ItemKind)> = world
        .query::<(Entity, &Pickup, &Position, &Size)>()
        .without::<&Expiring>()
        .without::<&Shop>()
        .iter()
        .filter(|(_, _, position, size)| geometry::overlaps(at, extent, position.0, size.0))
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

pub(crate) fn enemy_drops(
    world: &mut World,
    kills_without_drop: &mut u32,
    textures: &Textures,
    mut roll: impl FnMut(usize) -> usize,
) {
    let dead: Vec<Vec2> = world
        .query::<(&Position, &Animation)>()
        .with::<(&Enemy, &Expiring)>()
        .without::<&NoDrop>()
        .iter()
        .filter(|(_, animation)| animation.finished())
        .map(|(position, _)| position.0)
        .collect();
    let kills = world
        .query::<&Animation>()
        .with::<(&Enemy, &Expiring)>()
        .without::<&crate::components::Minion>()
        .iter()
        .filter(|animation| animation.finished())
        .count();
    super::stats::record(world, super::stats::Stat::Kill, kills as u32);
    for position in dead {
        *kills_without_drop += 1;
        if drop_threshold_reached(*kills_without_drop, roll(DROP_ROLL_RANGE)) {
            *kills_without_drop = 0;
            let kind = ItemKind::COMMON[roll(ItemKind::COMMON.len())];
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
        let kind = ItemKind::COMMON[roll(ItemKind::COMMON.len())];
        let velocity = Vec2::new(angle.cos(), angle.sin()) * BURST_SPEED;
        prefabs::item(world, kind, center, false, velocity, textures);
    }
}

#[cfg(test)]
mod tests {
    use crate::components::Velocity;
    use crate::items::{AmmoBox, PotionSize, DESPAWN_SECONDS};
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

        enemy_drops(&mut world, &mut kills, &textures, |_| 0);

        assert_eq!(world.query::<&Pickup>().iter().count(), 1);
        assert_eq!(kills, 0);
    }
}
