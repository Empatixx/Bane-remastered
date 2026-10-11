use hecs::World;

use crate::components::{Armor, Health, Loadout, Player, Wallet};

#[derive(Debug, Clone)]
pub(crate) struct Carried {
    health: Health,
    armor: Armor,
    wallet: Wallet,
    loadout: Loadout,
}

pub(crate) fn carried_stats(world: &World) -> Option<Carried> {
    let mut query = world
        .query::<(&Health, &Armor, &Wallet, &Loadout)>()
        .with::<&Player>();
    let (health, armor, wallet, loadout) = query.iter().next()?;
    Some(Carried {
        health: *health,
        armor: *armor,
        wallet: *wallet,
        loadout: loadout.clone(),
    })
}

pub(crate) fn restore_stats(world: &mut World, carried: Option<Carried>) {
    let Some(carried) = carried else {
        return;
    };
    for (health, armor, wallet, loadout) in world
        .query_mut::<(&mut Health, &mut Armor, &mut Wallet, &mut Loadout)>()
        .with::<&Player>()
    {
        *health = carried.health;
        *armor = carried.armor;
        *wallet = carried.wallet;
        *loadout = carried.loadout.clone();
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::assets::Textures;
    use crate::physics::Physics;
    use crate::prefabs;

    use super::*;

    fn player(world: &mut World) {
        let mut physics = Physics::new();
        prefabs::player(world, &mut physics, Vec2::ZERO, &Textures::default());
    }

    #[test]
    fn the_player_keeps_health_armor_coins_and_ammo_between_floors() {
        let mut first = World::new();
        player(&mut first);
        for (health, armor, wallet, loadout) in
            first.query_mut::<(&mut Health, &mut Armor, &mut Wallet, &mut Loadout)>()
        {
            health.current = 3;
            armor.current = 1;
            wallet.coins = 42;
            if let Some(weapon) = loadout.current_mut() {
                weapon.ammo = 17;
            }
        }

        let carried = carried_stats(&first);
        let mut second = World::new();
        player(&mut second);
        restore_stats(&mut second, carried);

        let stats = second
            .query::<(&Health, &Armor, &Wallet, &Loadout)>()
            .with::<&Player>()
            .iter()
            .map(|(h, a, w, g)| {
                (
                    h.current,
                    a.current,
                    w.coins,
                    g.current().map_or(0, |x| x.ammo),
                )
            })
            .next();
        assert_eq!(stats, Some((3, 1, 42, 17)));
    }

    #[test]
    fn nothing_is_restored_without_a_saved_player() {
        let mut world = World::new();
        player(&mut world);
        restore_stats(&mut world, None);
        let health = world
            .query::<&Health>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|h| h.current == h.max);
        assert_eq!(health, Some(true));
    }
}
