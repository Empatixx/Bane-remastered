use hecs::{Entity, World};

use crate::assets::Textures;
use crate::audio::AudioCommand;
use crate::components::Died;
use crate::physics::Physics;

/// The one place where enemies die, once per fixed tick after every system that deals damage.
/// Whichever system dealt the killing blow, the enemy is reaped and tagged `Died` here, every
/// death effect reads that tag in turn, and the tag is cleared before the tick ends.
pub(crate) fn death_phase(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
) {
    super::enemies::collapse_into_puddles(world, physics);
    super::enemies::reap_dead(world, physics);
    super::skills::death_bursts(world, physics, textures, sounds);
    super::skills::death_clouds(world, textures, sounds);
    super::skills::death_heals(world, sounds);
    super::frost::death_splits(world, physics, textures);
    super::lava::death_ashes(world, physics, textures);
    super::bosses::clean_up_after_boss(world);
    // The brood a fallen boss took with it is reaped on the same tick, after the death effects
    // ran, so it leaves no bursts, clouds or splits behind.
    super::enemies::reap_dead(world, physics);
    clear_died(world);
}

/// The one cleanup point of `Died`: the end of the death phase, after its last reader. Nothing
/// outside the death phase reads it, so it never survives into another tick or frame.
fn clear_died(world: &mut World) {
    let died: Vec<Entity> = world.query::<Entity>().with::<&Died>().iter().collect();
    for entity in died {
        let _ = world.remove_one::<Died>(entity);
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::components::{Expiring, Health, Minion};
    use crate::enemies::EnemyKind;
    use crate::prefabs;

    use super::*;

    fn kill(world: &mut World, entity: Entity) {
        if let Ok(mut health) = world.get::<&mut Health>(entity) {
            health.current = 0;
        }
    }

    #[test]
    fn a_kill_is_tagged_died_only_inside_the_death_phase() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let rat = prefabs::ready_enemy(&mut world, &mut physics, EnemyKind::Rat, Vec2::ZERO);
        kill(&mut world, rat);

        crate::systems::enemies::reap_dead(&mut world, &mut physics);
        assert!(world.get::<&Died>(rat).is_ok());
        clear_died(&mut world);
        assert!(world.get::<&Died>(rat).is_err());

        death_phase(&mut world, &mut physics, &Textures::default(), &mut Vec::new());
        assert!(world.get::<&Died>(rat).is_err());
        assert!(world.get::<&Expiring>(rat).is_ok());
    }

    #[test]
    fn a_boss_takes_its_brood_on_the_same_tick_without_their_death_effects() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        let boss = prefabs::ready_enemy(&mut world, &mut physics, EnemyKind::Spider, Vec2::ZERO);
        let _ = world.insert_one(boss, crate::components::Boss);
        let brood = prefabs::hatchling(
            &mut world,
            &mut physics,
            EnemyKind::SnowSlime,
            Vec2::new(300.0, 0.0),
            None,
            &textures,
        );
        let _ = world.insert_one(brood, Minion);
        kill(&mut world, boss);

        death_phase(&mut world, &mut physics, &textures, &mut Vec::new());

        assert!(world.get::<&Expiring>(boss).is_ok());
        assert!(world.get::<&Expiring>(brood).is_ok());
        let slimelings = world
            .query::<&crate::components::Enemy>()
            .iter()
            .filter(|enemy| enemy.kind == EnemyKind::SnowSlimeling)
            .count();
        assert_eq!(slimelings, 0, "the brood dies without splitting");
        assert_eq!(world.query::<&Died>().iter().count(), 0);
    }
}
