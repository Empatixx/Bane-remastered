use hecs::{Entity, World};

use crate::components::{Enemy, Health, Toughened};
use crate::enemies::scale_stat;

pub(crate) fn toughen_enemies(world: &mut World, factor: f32) {
    if (factor - 1.0).abs() < f32::EPSILON {
        return;
    }
    let fresh: Vec<Entity> = world
        .query::<Entity>()
        .with::<(&Enemy, &Health)>()
        .without::<&Toughened>()
        .iter()
        .collect();
    for entity in fresh {
        if let Ok(mut health) = world.get::<&mut Health>(entity) {
            health.max = scale_stat(health.max, factor);
            health.current = scale_stat(health.current, factor);
        }
        let _ = world.insert_one(entity, Toughened);
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

        toughen_enemies(&mut world, 1.0);
        assert_eq!(world.get::<&Health>(rat).expect("health").max, base);

        toughen_enemies(&mut world, 2.0);
        toughen_enemies(&mut world, 2.0);
        let health = world.get::<&Health>(rat).expect("health");
        assert_eq!((health.current, health.max), (base * 2, base * 2));
    }
}
