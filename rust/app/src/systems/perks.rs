use hecs::{Entity, World};

use crate::components::{
    Animation, Armor, ContactDamage, Flame, Health, Loadout, Player, PlayerPerks, Trap,
};
use crate::upgrades::Perks;
use crate::weapon_upgrades::WeaponMods;

const TRAP_DISABLE_CHANCE: usize = 30;
const SPIKE_RETRACTED_FRAME: usize = 2;
const FLAME_OFF_FRAME: usize = 0;

pub(crate) fn apply_floor_perks(world: &mut World, perks: Perks) {
    let player = world.query::<Entity>().with::<&Player>().iter().next();
    let Some(player) = player else {
        return;
    };
    let _ = world.insert_one(player, PlayerPerks(perks));
    if let Ok(mut loadout) = world.get::<&mut Loadout>(player) {
        loadout.apply_perks(perks.extra_slots, perks.ammo_scale);
        loadout.restrict(perks.only_gun);
    }
}

pub(crate) fn apply_weapon_mods(world: &mut World, mods: WeaponMods) {
    for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
        loadout.apply_weapon_mods(mods);
    }
}

pub(crate) fn apply_run_perks(world: &mut World, perks: Perks) {
    for (health, armor) in world
        .query_mut::<(&mut Health, &mut Armor)>()
        .with::<&Player>()
    {
        let max_health = perks.max_health(health.max);
        health.current = (health.current + (max_health - health.max).max(0)).min(max_health);
        health.max = max_health;
        let max_armor = perks.max_armor(armor.max);
        armor.current = (armor.current + (max_armor - armor.max).max(0)).min(max_armor);
        armor.max = max_armor;
    }
    apply_floor_perks(world, perks);
}

pub(crate) fn immobilize_traps(
    world: &mut World,
    perks: Perks,
    mut roll: impl FnMut(usize) -> usize,
) {
    if !perks.trap_master {
        return;
    }
    let traps: Vec<(Entity, bool)> = world
        .query::<(Entity, Option<&Flame>)>()
        .with::<&Trap>()
        .iter()
        .map(|(entity, flame)| (entity, flame.is_some()))
        .collect();
    for (entity, is_flame) in traps {
        if roll(101) > TRAP_DISABLE_CHANCE {
            continue;
        }
        let _ = world.remove_one::<Trap>(entity);
        let _ = world.remove_one::<ContactDamage>(entity);
        if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
            animation.hold(if is_flame {
                FLAME_OFF_FRAME
            } else {
                SPIKE_RETRACTED_FRAME
            });
        }
    }
}

pub(crate) fn regenerate_armor(world: &mut World) {
    let enabled = world
        .query::<&PlayerPerks>()
        .with::<&Player>()
        .iter()
        .next()
        .is_some_and(|perks| perks.0.armor_regeneration);
    if !enabled {
        return;
    }
    for armor in world.query_mut::<&mut Armor>().with::<&Player>() {
        armor.current = (armor.current + 1).min(armor.max);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use glam::Vec2;

    use crate::assets::Textures;
    use crate::physics::Physics;
    use crate::prefabs;
    use crate::upgrades::Upgrade;

    use super::*;

    fn perks(upgrades: &[Upgrade]) -> Perks {
        Perks::from_unlocked(&upgrades.iter().copied().collect::<BTreeSet<_>>())
    }

    fn player() -> World {
        let mut world = World::new();
        prefabs::player(
            &mut world,
            &mut Physics::new(),
            Vec2::ZERO,
            &Textures::default(),
        );
        world
    }

    #[test]
    fn a_health_cap_keeps_the_player_at_one_heart() {
        let mut world = player();
        let capped = Perks {
            max_health_cap: Some(1),
            ..Perks::default()
        };
        apply_run_perks(&mut world, capped);
        let (current, max, _, _) = vitals(&world);
        assert_eq!((current, max), (1, 1));
    }

    #[test]
    fn pistol_only_takes_the_starting_shotgun_away() {
        let mut world = player();
        let only = Perks {
            only_gun: Some(crate::guns::GunKind::Pistol),
            ..Perks::default()
        };
        apply_floor_perks(&mut world, only);
        let loadout = world
            .query::<&Loadout>()
            .with::<&Player>()
            .iter()
            .next()
            .cloned()
            .expect("a loadout");
        assert!(loadout.has(crate::guns::GunKind::Pistol));
        assert!(!loadout.has(crate::guns::GunKind::Shotgun));
        assert!(!loadout.allows(crate::guns::GunKind::Uzi));
    }

    fn vitals(world: &World) -> (i32, i32, i32, i32) {
        world
            .query::<(&Health, &Armor)>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|(h, a)| (h.current, h.max, a.current, a.max))
            .unwrap_or_default()
    }

    #[test]
    fn health_and_armor_upgrades_raise_the_maximum_and_fill_the_difference() {
        let mut world = player();
        apply_run_perks(
            &mut world,
            perks(&[
                Upgrade::ImprovedHealth,
                Upgrade::EnhancedHealth,
                Upgrade::ImprovedArmor,
            ]),
        );
        assert_eq!(vitals(&world), (15, 15, 6, 6));
    }

    #[test]
    fn without_upgrades_only_the_free_ones_apply() {
        let mut world = player();
        apply_run_perks(&mut world, Perks::default());
        assert_eq!(vitals(&world), (12, 12, 5, 5));
    }

    #[test]
    fn the_extra_gun_slot_and_ammo_capacity_reach_the_loadout() {
        let mut world = player();
        apply_run_perks(
            &mut world,
            perks(&[Upgrade::AdditionalGun, Upgrade::AdditionalAmmo]),
        );
        let mut loadout_query = world.query::<&Loadout>();
        let loadout = loadout_query.iter().next().expect("a loadout");
        let mut copy = loadout.clone();
        assert!(copy.switch_to(2));
        assert_eq!(copy.fresh(crate::guns::GunKind::Uzi).max_ammo(), 220);
    }

    #[test]
    fn trap_master_immobilizes_some_traps_for_good() {
        let mut world = World::new();
        let textures = Textures::default();
        let mut physics = Physics::new();
        let spikes: Vec<Entity> = (0..4)
            .map(|index| {
                let decor = crate::dungeon::Decor {
                    kind: crate::dungeon::DecorKind::Spike,
                    tile: (index, 0),
                };
                prefabs::decor(&mut world, &mut physics, decor, &textures)
            })
            .collect();
        let mut index = 0;
        immobilize_traps(&mut world, perks(&[Upgrade::TrapMaster]), |_| {
            index += 1;
            if index % 2 == 0 { 100 } else { 0 }
        });

        let harmless = spikes
            .iter()
            .filter(|s| world.get::<&Trap>(**s).is_err())
            .count();
        assert_eq!(harmless, 2);
        let frozen = spikes
            .iter()
            .filter_map(|s| world.get::<&Animation>(*s).ok().map(|a| a.frame()))
            .filter(|frame| *frame == 2)
            .count();
        assert!(frozen >= 2);
    }

    #[test]
    fn without_trap_master_every_trap_stays_dangerous() {
        let mut world = World::new();
        let decor = crate::dungeon::Decor {
            kind: crate::dungeon::DecorKind::Spike,
            tile: (0, 0),
        };
        let spike = prefabs::decor(&mut world, &mut Physics::new(), decor, &Textures::default());
        immobilize_traps(&mut world, Perks::default(), |_| 0);
        assert!(world.get::<&Trap>(spike).is_ok());
    }

    #[test]
    fn armor_regeneration_gives_one_point_per_cleared_room_up_to_the_maximum() {
        let mut world = player();
        apply_floor_perks(&mut world, perks(&[Upgrade::ArmorRegeneration]));
        for armor in world.query_mut::<&mut Armor>().with::<&Player>() {
            armor.current = 3;
        }
        regenerate_armor(&mut world);
        assert_eq!(vitals(&world).2, 4);
        regenerate_armor(&mut world);
        regenerate_armor(&mut world);
        assert_eq!(vitals(&world).2, 5);

        let mut plain = player();
        for armor in plain.query_mut::<&mut Armor>().with::<&Player>() {
            armor.current = 1;
        }
        regenerate_armor(&mut plain);
        assert_eq!(vitals(&plain).2, 1);
    }

    #[test]
    fn speed_damage_and_kevlar_change_how_the_player_behaves() {
        use crate::input::Input;
        use winit::keyboard::KeyCode;

        let mut world = player();
        let mut input = Input::default();
        input.press(KeyCode::KeyD);
        let top_speed = |world: &mut World| {
            let mut speed = 0.0;
            for _ in 0..240 {
                crate::systems::steer_player(world, &input, 1.0 / 60.0);
                speed = world
                    .query::<&crate::components::Velocity>()
                    .with::<&Player>()
                    .iter()
                    .next()
                    .map_or(0.0, |v| v.0.x);
            }
            speed
        };
        let base = top_speed(&mut world);
        let mut boosted_world = player();
        apply_floor_perks(&mut boosted_world, perks(&[Upgrade::ReducedDamageTaken]));
        let boosted = top_speed(&mut boosted_world);
        assert!((base - 411.0).abs() < 2.0, "{base}");
        assert!((boosted - 411.0 * 1.1).abs() < 2.0, "{boosted}");

        let player_entity = boosted_world
            .query::<Entity>()
            .with::<&Player>()
            .iter()
            .next()
            .expect("player");
        let before = vitals(&boosted_world);
        crate::systems::damage::hurt_player(&mut boosted_world, player_entity, 1, &mut Vec::new());
        assert_ne!(vitals(&boosted_world), before);
    }

    #[test]
    fn sales_follow_the_chances_of_bane() {
        use crate::systems::shop::sale_for;
        assert_eq!(sale_for(0), Some(0.75));
        assert_eq!(sale_for(7), Some(0.5));
        assert_eq!(sale_for(20), Some(0.25));
        assert_eq!(sale_for(33), None);
    }
}
