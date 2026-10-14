use hecs::{Entity, World};

use crate::components::{Health, Text, TrainingDummy};

/// How far back the damage-per-second readout looks.
pub(crate) const DPS_WINDOW: f32 = 3.0;

/// Refills every training dummy, remembers what it took and writes the
/// damage per second over the last few seconds above it.
pub(crate) fn tick_dummies(world: &mut World, dt: f32) {
    let mut readouts: Vec<(Entity, String)> = Vec::new();
    for (dummy, health) in world.query_mut::<(&mut TrainingDummy, &mut Health)>() {
        for hit in &mut dummy.recent {
            hit.0 += dt;
        }
        dummy.recent.retain(|(age, _)| *age < DPS_WINDOW);
        let taken = health.max - health.current;
        if taken > 0 {
            dummy.recent.push((0.0, taken));
            health.current = health.max;
        }
        let shown = damage_per_second(&dummy.recent)
            .map_or_else(String::new, |dps| format!("{dps:.0} DPS"));
        readouts.push((dummy.readout, shown));
    }
    for (readout, shown) in readouts {
        if let Ok(mut text) = world.get::<&mut Text>(readout) {
            text.content = shown;
        }
    }
}

/// The damage over the time since the oldest remembered hit, but at least a
/// second, so a lone first shot does not read as a huge number.
fn damage_per_second(recent: &[(f32, i32)]) -> Option<f32> {
    let oldest = recent
        .iter()
        .map(|(age, _)| *age)
        .fold(None, |most: Option<f32>, age| {
            Some(most.map_or(age, |most| most.max(age)))
        })?;
    let total: i32 = recent.iter().map(|(_, amount)| amount).sum();
    Some(total as f32 / oldest.max(1.0))
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::assets::Textures;
    use crate::physics::Physics;
    use crate::prefabs;
    use crate::systems::damage::{Damage, DamageSource, deal};

    use super::*;

    fn dummy() -> (World, Entity) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let entity =
            prefabs::training_dummy(&mut world, &mut physics, Vec2::ZERO, &Textures::default());
        (world, entity)
    }

    fn hit(world: &mut World, target: Entity, amount: i32) {
        assert!(deal(
            world,
            Damage {
                target,
                amount,
                source: DamageSource::Bullet,
            },
        ));
    }

    fn readout(world: &World, entity: Entity) -> String {
        let readout = world.get::<&TrainingDummy>(entity).expect("dummy").readout;
        world
            .get::<&Text>(readout)
            .expect("readout")
            .content
            .clone()
    }

    #[test]
    fn the_dummy_takes_any_hit_and_is_whole_again_next_tick() {
        let (mut world, entity) = dummy();
        hit(&mut world, entity, prefabs::DUMMY_HEALTH * 2);
        tick_dummies(&mut world, 1.0 / 60.0);
        let health = *world.get::<&Health>(entity).expect("health");
        assert_eq!(health.current, health.max);
    }

    #[test]
    fn the_readout_shows_damage_per_second_and_clears_when_the_shooting_stops() {
        let (mut world, entity) = dummy();
        assert_eq!(readout(&world, entity), "");
        for _ in 0..4 {
            hit(&mut world, entity, 10);
            tick_dummies(&mut world, 0.5);
        }
        // 40 damage, the oldest hit 1.5 s ago.
        assert_eq!(readout(&world, entity), "27 DPS");

        tick_dummies(&mut world, DPS_WINDOW);
        assert_eq!(readout(&world, entity), "");
    }

    #[test]
    fn a_lone_hit_counts_over_at_least_a_second() {
        assert_eq!(damage_per_second(&[(0.0, 50)]), Some(50.0));
        assert_eq!(damage_per_second(&[]), None);
    }
}
