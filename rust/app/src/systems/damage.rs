use hecs::{Entity, World};

use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Animation, Armor, ContactDamage, Expiring, Health, Invulnerable, Player, Position, Size,
    Spawning, Trap,
};
use crate::geometry;

const INVULNERABLE_SECONDS: f32 = 1.0;

pub(crate) fn absorb(health: &mut Health, armor: &mut Armor, damage: i32) {
    if armor.current > 0 {
        armor.current -= damage;
        if armor.current < 0 {
            health.current += armor.current;
            armor.current = 0;
        }
    } else {
        health.current -= damage;
    }
    health.current = health.current.max(0);
}

const LOW_HEALTH: f32 = 0.3;

fn trap_is_armed(trap: Option<&Trap>, animation: Option<&Animation>) -> bool {
    match (trap, animation) {
        (Some(trap), Some(animation)) => animation.frame() < trap.active_frames,
        _ => true,
    }
}

pub(crate) fn contact_damage(world: &mut World, sounds: &mut Vec<AudioCommand>) -> bool {
    let Some((player, position, size)) = world
        .query::<(Entity, &Position, &Size)>()
        .with::<(&Player, &Health)>()
        .without::<&Invulnerable>()
        .iter()
        .next()
        .map(|(entity, position, size)| (entity, position.0, size.0))
    else {
        return false;
    };
    let damage = world
        .query::<(
            &Position,
            &Size,
            &ContactDamage,
            Option<&Trap>,
            Option<&Animation>,
        )>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .find(|(source_position, source_size, _, trap, animation)| {
            trap_is_armed(*trap, *animation)
                && geometry::overlaps(position, size, source_position.0, source_size.0)
        })
        .map(|(_, _, damage, _, _)| damage.0);
    let Some(damage) = damage else {
        return false;
    };
    if let Ok((health, armor)) = world.query_one_mut::<(&mut Health, &mut Armor)>(player) {
        if health.current <= 0 {
            return false;
        }
        let was_low = (health.current as f32) < health.max as f32 * LOW_HEALTH;
        absorb(health, armor, damage);
        let is_low = (health.current as f32) < health.max as f32 * LOW_HEALTH;
        sounds.push(AudioCommand::Effect(if health.current <= 0 {
            SoundEffect::Death
        } else {
            SoundEffect::PlayerHurt
        }));
        if is_low && !was_low && health.current > 0 {
            sounds.push(AudioCommand::Effect(SoundEffect::LowHealth));
        }
        let _ = world.insert_one(
            player,
            Invulnerable {
                remaining: INVULNERABLE_SECONDS,
            },
        );
        return true;
    }
    false
}

pub(crate) fn tick_invulnerability(world: &mut World, dt: f32) {
    let mut finished = Vec::new();
    for (entity, invulnerable) in world.query_mut::<(Entity, &mut Invulnerable)>() {
        invulnerable.remaining -= dt;
        if invulnerable.remaining <= 0.0 {
            finished.push(entity);
        }
    }
    for entity in finished {
        let _ = world.remove_one::<Invulnerable>(entity);
    }
}

pub(crate) fn player_is_low_on_health(world: &World) -> bool {
    world
        .query::<&Health>()
        .with::<&Player>()
        .iter()
        .any(|health| {
            health.current > 0 && (health.current as f32) < health.max as f32 * LOW_HEALTH
        })
}

pub(crate) fn player_is_dead(world: &World) -> bool {
    world
        .query::<&Health>()
        .with::<&Player>()
        .iter()
        .any(|health| health.current <= 0)
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::assets::Textures;
    use crate::enemies::EnemyKind;
    use crate::physics::Physics;
    use crate::prefabs;

    use super::*;

    #[test]
    fn armor_absorbs_first_and_the_rest_hurts() {
        let mut health = Health {
            current: 10,
            max: 10,
        };
        let mut armor = Armor { current: 1, max: 5 };
        absorb(&mut health, &mut armor, 3);
        assert_eq!((armor.current, health.current), (0, 8));

        absorb(&mut health, &mut armor, 2);
        assert_eq!(health.current, 6);
    }

    #[test]
    fn health_never_goes_below_zero() {
        let mut health = Health {
            current: 1,
            max: 10,
        };
        let mut armor = Armor { current: 0, max: 5 };
        absorb(&mut health, &mut armor, 5);
        assert_eq!(health.current, 0);
    }

    #[test]
    fn touching_enemy_hurts_once_then_the_player_is_invulnerable() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let player = prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(100.0, 100.0),
            &Textures::default(),
        );
        prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(100.0, 100.0),
        );

        contact_damage(&mut world, &mut Vec::new());
        contact_damage(&mut world, &mut Vec::new());

        let armor = world.get::<&Armor>(player).map(|a| a.current).ok();
        let health = world.get::<&Health>(player).map(|h| h.current).ok();
        assert_eq!((armor, health), (Some(3), Some(10)));
        assert!(world.get::<&Invulnerable>(player).is_ok());

        tick_invulnerability(&mut world, 1.1);
        assert!(world.get::<&Invulnerable>(player).is_err());
    }

    #[test]
    fn spikes_hurt_only_while_extended() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let player = prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(32.0, 32.0),
            &Textures::default(),
        );
        let decor = crate::dungeon::Decor {
            kind: crate::dungeon::DecorKind::Spike,
            tile: (0, 0),
        };
        let spike = prefabs::decor(&mut world, decor, &Textures::default());

        world
            .get::<&mut Animation>(spike)
            .expect("the spike animates")
            .advance(0.4);
        assert!(!contact_damage(&mut world, &mut Vec::new()));
        assert!(world.get::<&Invulnerable>(player).is_err());

        world
            .get::<&mut Animation>(spike)
            .expect("the spike animates")
            .advance(0.4);
        assert!(contact_damage(&mut world, &mut Vec::new()));
    }
}
