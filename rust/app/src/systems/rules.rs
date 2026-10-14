use hecs::{Entity, World};

use crate::components::{ContactDamage, Enemy, EnemyBullet, Health, Leaper, Toughened};
use crate::enemies::scale_stat;

pub(crate) fn toughen_enemies(world: &mut World, health: f32, damage: f32) {
    if is_neutral(health) && is_neutral(damage) {
        return;
    }
    let fresh: Vec<Entity> = world
        .query::<Entity>()
        .with::<&Enemy>()
        .without::<&Toughened>()
        .iter()
        .collect();
    for entity in fresh {
        if let Ok(mut hp) = world.get::<&mut Health>(entity) {
            hp.max = scale_stat(hp.max, health);
            hp.current = scale_stat(hp.current, health);
        }
        scale_damage(world, entity, damage);
        if let Ok(mut leaper) = world.get::<&mut Leaper>(entity) {
            leaper.damage = scale_stat(leaper.damage, damage);
        }
        let _ = world.insert_one(entity, Toughened);
    }
    let bullets: Vec<Entity> = world
        .query::<Entity>()
        .with::<&EnemyBullet>()
        .without::<&Toughened>()
        .iter()
        .collect();
    for entity in bullets {
        scale_damage(world, entity, damage);
        let _ = world.insert_one(entity, Toughened);
    }
}

fn is_neutral(factor: f32) -> bool {
    (factor - 1.0).abs() < f32::EPSILON
}

fn scale_damage(world: &World, entity: Entity, factor: f32) {
    if let Ok(mut contact) = world.get::<&mut ContactDamage>(entity) {
        contact.0 = scale_stat(contact.0, factor);
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;
    use hecs::World;

    use crate::assets::Textures;
    use crate::components::Health;
    use crate::enemies::EnemyKind;
    use crate::prefabs;

    use super::*;

    #[test]
    fn new_enemies_are_toughened_once_and_neutral_rules_touch_nothing() {
        let mut world = World::new();
        let rat = prefabs::enemy(
            &mut world,
            EnemyKind::Rat,
            Vec2::ZERO,
            1,
            &Textures::default(),
        );
        let base = world.get::<&Health>(rat).expect("health").max;

        toughen_enemies(&mut world, 1.0, 1.0);
        assert_eq!(world.get::<&Health>(rat).expect("health").max, base);

        toughen_enemies(&mut world, 2.0, 1.0);
        toughen_enemies(&mut world, 2.0, 1.0);
        let health = world.get::<&Health>(rat).expect("health");
        assert_eq!((health.current, health.max), (base * 2, base * 2));
    }

    #[test]
    fn a_horde_damage_factor_rounds_a_two_damage_enemy_up_to_three() {
        let mut world = World::new();
        let enemy = prefabs::enemy(
            &mut world,
            EnemyKind::Rat,
            Vec2::ZERO,
            1,
            &Textures::default(),
        );
        world
            .insert_one(enemy, ContactDamage(2))
            .expect("contact damage");
        let bullet = world.spawn((EnemyBullet, ContactDamage(2)));

        toughen_enemies(&mut world, 1.0, 1.3);
        toughen_enemies(&mut world, 1.0, 1.3);

        assert_eq!(world.get::<&ContactDamage>(enemy).expect("damage").0, 3);
        assert_eq!(world.get::<&ContactDamage>(bullet).expect("damage").0, 3);
    }
}
