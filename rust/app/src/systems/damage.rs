use hecs::{Entity, World};

use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Armor, ContactDamage, Expiring, Health, Invulnerable, Player, Position, Size,
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

pub(crate) fn contact_damage(world: &mut World, sounds: &mut Vec<AudioCommand>) {
    let Some((player, position, size)) = world
        .query::<(Entity, &Position, &Size)>()
        .with::<(&Player, &Health)>()
        .without::<&Invulnerable>()
        .iter()
        .next()
        .map(|(entity, position, size)| (entity, position.0, size.0))
    else {
        return;
    };
    let damage = world
        .query::<(&Position, &Size, &ContactDamage)>()
        .without::<&Expiring>()
        .iter()
        .find(|(enemy_position, enemy_size, _)| {
            geometry::overlaps(position, size, enemy_position.0, enemy_size.0)
        })
        .map(|(_, _, damage)| damage.0);
    let Some(damage) = damage else {
        return;
    };
    if let Ok((health, armor)) = world.query_one_mut::<(&mut Health, &mut Armor)>(player) {
        if health.current <= 0 {
            return;
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
    }
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
    use crate::physics::Physics;
    use crate::prefabs;
    use crate::render::TextureId;

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
        prefabs::rat(
            &mut world,
            &mut physics,
            Vec2::new(100.0, 100.0),
            TextureId::WHITE,
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
}
