use glam::Vec2;
use hecs::{Entity, World};

use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Animation, Armor, Breakable, ContactDamage, Enemy, Expiring, Health, HitFlash, Invulnerable,
    Player, Position, Shielded, Size, Spawning, Trap, TrapCooldown,
};
use crate::geometry;
use crate::systems::enemies::FLASH_SECONDS;
use crate::systems::stats::{self, Stat};

const INVULNERABLE_SECONDS: f32 = 1.0;
const HURT_FLASH_SECONDS: f32 = 0.5;
/// Without a cast die a fraction of a hit rounds to the nearest point.
const UNROLLED_HIT: f32 = 0.5;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DamageSource {
    Bullet,
    Summoned,
    Trap,
    Bite,
    Explosion,
    Poison,
}

impl DamageSource {
    fn counts_as_hit(self) -> bool {
        matches!(self, Self::Bullet | Self::Summoned)
    }

    fn flashes(self) -> bool {
        self != Self::Poison
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Damage {
    pub(crate) target: Entity,
    pub(crate) amount: i32,
    pub(crate) source: DamageSource,
}

/// The one way to hurt an enemy or a breakable object, the player is hurt through `hurt_player`.
/// Returns whether the target could take damage at all.
pub(crate) fn deal(world: &mut World, damage: Damage) -> bool {
    let Damage {
        target,
        amount,
        source,
    } = damage;
    debug_assert!(world.get::<&Player>(target).is_err(), "use hurt_player");
    if world.get::<&crate::components::Sealed>(target).is_ok() {
        return false;
    }
    match world.get::<&mut Health>(target) {
        Ok(mut health) => health.current = (health.current - amount).max(0),
        Err(_) => return false,
    }
    if source.counts_as_hit() {
        super::skills::count_hits(world, target);
        if source == DamageSource::Bullet && world.get::<&Enemy>(target).is_ok() {
            stats::record(world, Stat::Hit, 1);
        }
    }
    if source.flashes() {
        let _ = world.insert_one(
            target,
            HitFlash {
                remaining: FLASH_SECONDS,
            },
        );
    }
    if world.get::<&Breakable>(target).is_err() {
        super::effects::show_change(world, target, -amount, crate::rng::cosmetic());
    }
    true
}

fn trap_is_armed(trap: Option<&Trap>, animation: Option<&Animation>) -> bool {
    match (trap, animation) {
        (Some(trap), Some(animation)) => {
            (trap.from_frame..trap.until_frame).contains(&animation.frame())
        }
        _ => true,
    }
}

const TRAP_COOLDOWN: f32 = 1.0;

pub(crate) fn trap_hits_enemies(world: &mut World, dt: f32) {
    for cooldown in world.query_mut::<&mut TrapCooldown>() {
        cooldown.remaining -= dt;
    }
    let ready: Vec<Entity> = world
        .query::<(Entity, &TrapCooldown)>()
        .iter()
        .filter(|(_, cooldown)| cooldown.remaining <= 0.0)
        .map(|(entity, _)| entity)
        .collect();
    for entity in ready {
        let _ = world.remove_one::<TrapCooldown>(entity);
    }
    let traps: Vec<(Vec2, Vec2, i32)> = world
        .query::<(&Position, &Size, &ContactDamage, &Trap, &Animation)>()
        .iter()
        .filter(|(_, _, _, trap, animation)| trap_is_armed(Some(*trap), Some(*animation)))
        .map(|(position, size, damage, _, _)| (position.0, size.0, damage.0))
        .collect();
    if traps.is_empty() {
        return;
    }
    let victims: Vec<(Entity, i32)> = world
        .query::<(Entity, &Position, &Size)>()
        .with::<(&Enemy, &Health)>()
        .without::<&TrapCooldown>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .filter_map(|(entity, position, size)| {
            traps
                .iter()
                .find(|(trap, trap_size, _)| {
                    geometry::overlaps(position.0, size.0, *trap, *trap_size)
                })
                .map(|(_, _, damage)| (entity, *damage))
        })
        .collect();
    for (entity, amount) in victims {
        deal(
            world,
            Damage {
                target: entity,
                amount,
                source: DamageSource::Trap,
            },
        );
        let _ = world.insert_one(
            entity,
            TrapCooldown {
                remaining: TRAP_COOLDOWN,
            },
        );
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
    let cloaked = world.get::<&crate::components::Phantom>(player).is_ok();
    let damage = world
        .query::<(
            &Position,
            &Size,
            &ContactDamage,
            Option<&Trap>,
            Option<&Animation>,
            Option<&Enemy>,
        )>()
        .without::<&Expiring>()
        .without::<&crate::components::Frozen>()
        .without::<&Spawning>()
        .iter()
        .find(
            |(source_position, source_size, _, trap, animation, enemy)| {
                !(cloaked && enemy.is_some())
                    && trap_is_armed(*trap, *animation)
                    && geometry::overlaps(position, size, source_position.0, source_size.0)
            },
        )
        .map(|(_, _, damage, _, _, _)| damage.0);
    let Some(damage) = damage else {
        return false;
    };
    hurt_player(world, player, damage, sounds)
}

pub(crate) fn hurt_player(
    world: &mut World,
    player: Entity,
    damage: i32,
    sounds: &mut Vec<AudioCommand>,
) -> bool {
    hurt(world, player, damage, false, sounds)
}

pub(crate) fn hurt_player_through_armor(
    world: &mut World,
    player: Entity,
    damage: i32,
    sounds: &mut Vec<AudioCommand>,
) -> bool {
    hurt(world, player, damage, true, sounds)
}

fn hurt(
    world: &mut World,
    player: Entity,
    damage: i32,
    through_armor: bool,
    sounds: &mut Vec<AudioCommand>,
) -> bool {
    if world.get::<&Invulnerable>(player).is_ok() {
        return false;
    }
    let roll = world
        .get::<&crate::components::HitRoll>(player)
        .map_or(UNROLLED_HIT, |roll| roll.0);
    let damage = world
        .get::<&crate::components::PlayerPerks>(player)
        .map_or(damage, |perks| perks.0.damage_taken(damage, roll));
    let damage = if world.remove_one::<Shielded>(player).is_ok() {
        0
    } else {
        damage
    };
    if let Ok((health, armor)) = world.query_one_mut::<(&mut Health, &mut Armor)>(player) {
        if health.current <= 0 {
            return false;
        }
        let was_low = (health.current as f32) < health.max as f32 * LOW_HEALTH;
        if through_armor {
            health.current = (health.current - damage).max(0);
        } else {
            absorb(health, armor, damage);
        }
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
        let _ = world.insert_one(
            player,
            crate::components::HitFlash {
                remaining: HURT_FLASH_SECONDS,
            },
        );
        stats::record(world, Stat::Hurt, 1);
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
    use crate::components::Hits;
    use crate::enemies::EnemyKind;
    use crate::physics::Physics;
    use crate::prefabs;

    use super::*;

    fn enemy(world: &mut World, physics: &mut Physics, kind: EnemyKind) -> Entity {
        prefabs::ready_enemy(world, physics, kind, Vec2::new(300.0, 0.0))
    }

    fn floating_numbers(world: &World) -> usize {
        world.query::<&crate::components::Floating>().iter().count()
    }

    fn hit(world: &mut World, target: Entity, amount: i32, source: DamageSource) -> bool {
        deal(
            world,
            Damage {
                target,
                amount,
                source,
            },
        )
    }

    #[test]
    fn every_source_clamps_at_zero_and_shows_the_number() {
        for source in [
            DamageSource::Bullet,
            DamageSource::Trap,
            DamageSource::Bite,
            DamageSource::Explosion,
            DamageSource::Poison,
        ] {
            let mut world = World::new();
            let mut physics = Physics::new();
            let rat = enemy(&mut world, &mut physics, EnemyKind::Rat);

            assert!(hit(&mut world, rat, 1000, source));

            let health = world.get::<&Health>(rat).map(|h| h.current).ok();
            assert_eq!(health, Some(0), "{source:?}");
            assert_eq!(floating_numbers(&world), 1, "{source:?}");
        }
    }

    #[test]
    fn only_bullets_count_as_hits_and_poison_does_not_flash() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let slime = enemy(&mut world, &mut physics, EnemyKind::RedSlime);

        hit(&mut world, slime, 1, DamageSource::Explosion);
        assert_eq!(world.get::<&Hits>(slime).map(|h| h.0).ok(), Some(0));
        assert!(world.get::<&HitFlash>(slime).is_ok());

        hit(&mut world, slime, 1, DamageSource::Bullet);
        assert_eq!(world.get::<&Hits>(slime).map(|h| h.0).ok(), Some(1));

        let _ = world.remove_one::<HitFlash>(slime);
        hit(&mut world, slime, 1, DamageSource::Poison);
        assert!(world.get::<&HitFlash>(slime).is_err());
    }

    #[test]
    fn a_broken_object_shows_no_number_and_a_target_without_health_takes_nothing() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let barrel = prefabs::breakable(
            &mut world,
            &mut physics,
            crate::components::BreakKind::Explosive,
            Vec2::ZERO,
            &Textures::default(),
        );
        assert!(hit(&mut world, barrel, 1, DamageSource::Bullet));
        assert_eq!(floating_numbers(&world), 0);

        let nothing = world.spawn((Position(Vec2::ZERO),));
        assert!(!hit(&mut world, nothing, 1, DamageSource::Bullet));
    }

    #[test]
    fn an_enemy_killed_by_an_explosion_bursts_and_is_reaped() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let slime = enemy(&mut world, &mut physics, EnemyKind::RedSlime);
        hit(&mut world, slime, 1, DamageSource::Bullet);
        hit(&mut world, slime, 1000, DamageSource::Explosion);

        crate::systems::resolve_deaths(&mut world, &mut physics, &Textures::default(), &mut Vec::new());

        assert!(world.get::<&Expiring>(slime).is_ok());
        assert!(world.get::<&crate::components::DeathBurst>(slime).is_err());
        assert!(
            world
                .query::<&crate::components::EnemyBullet>()
                .iter()
                .count()
                > 0
        );
    }

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
    fn touching_an_enemy_does_not_hurt_a_cloaked_player_but_spikes_still_do() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let player = prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(32.0, 32.0),
            &Textures::default(),
        );
        let _ = world.insert_one(player, crate::components::Phantom { remaining: 5.0 });
        prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(32.0, 32.0),
        );

        assert!(!contact_damage(&mut world, &mut Vec::new()));

        let decor = crate::dungeon::Decor {
            kind: crate::dungeon::DecorKind::Spike,
            tile: (0, 0),
        };
        let spike = prefabs::decor(&mut world, &mut Physics::new(), decor, &Textures::default());
        world
            .get::<&mut Animation>(spike)
            .expect("the spike animates")
            .advance(0.8);
        assert!(contact_damage(&mut world, &mut Vec::new()));
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
    fn a_fire_geyser_hurts_only_while_it_erupts() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let player = prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(96.0, 96.0),
            &Textures::default(),
        );
        if let Ok(mut armor) = world.get::<&mut Armor>(player) {
            armor.current = 0;
        }
        let decor = crate::dungeon::Decor {
            kind: crate::dungeon::DecorKind::FireGeyser,
            tile: (1, 2),
        };
        let geyser = prefabs::decor(&mut world, &mut Physics::new(), decor, &Textures::default());

        world
            .get::<&mut Animation>(geyser)
            .expect("the geyser animates")
            .advance(0.8);
        assert!(!contact_damage(&mut world, &mut Vec::new()));

        world
            .get::<&mut Animation>(geyser)
            .expect("the geyser animates")
            .advance(0.2);
        let before = world.get::<&Health>(player).map(|h| h.current).unwrap_or(0);
        assert!(contact_damage(&mut world, &mut Vec::new()));
        let after = world.get::<&Health>(player).map(|h| h.current).unwrap_or(0);
        assert_eq!(after, before - 1);
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
        let spike = prefabs::decor(&mut world, &mut Physics::new(), decor, &Textures::default());

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

    #[test]
    fn an_extended_spike_hurts_an_enemy_only_once_a_second() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        let decor = crate::dungeon::Decor {
            kind: crate::dungeon::DecorKind::Spike,
            tile: (10, 10),
        };
        let spike = prefabs::decor(&mut world, &mut Physics::new(), decor, &textures);
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(10.5 * 64.0, 10.5 * 64.0),
        );
        let health = |world: &World| world.get::<&Health>(rat).map(|h| h.current).unwrap_or(-1);

        trap_hits_enemies(&mut world, 0.016);
        assert_eq!(health(&world), 11);
        trap_hits_enemies(&mut world, 0.5);
        assert_eq!(health(&world), 11);
        trap_hits_enemies(&mut world, 0.6);
        assert_eq!(health(&world), 10);

        world
            .get::<&mut Animation>(spike)
            .expect("the spike animates")
            .advance(0.4);
        trap_hits_enemies(&mut world, 1.1);
        assert_eq!(health(&world), 10);
    }

    #[test]
    fn hurting_the_player_is_recorded_for_the_floor() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let player = prefabs::player(&mut world, &mut physics, Vec2::ZERO, &Textures::default());
        assert!(hurt_player(&mut world, player, 1, &mut Vec::new()));
        assert_eq!(stats::current(&world).hurt_this_floor, 1);
    }

    fn player_health(world: &World, player: Entity) -> i32 {
        let armor = world.get::<&Armor>(player).map_or(0, |a| a.current);
        world.get::<&Health>(player).map_or(-1, |h| h.current) + armor
    }

    #[test]
    fn a_cloaked_player_is_hurt_by_enemy_bullets_but_not_by_touching_enemies() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        let at = Vec2::new(500.0, 500.0);
        let player = prefabs::player(&mut world, &mut physics, at, &textures);
        let _ = world.insert_one(player, crate::components::Phantom { remaining: 5.0 });
        let before = player_health(&world, player);

        let rat = prefabs::ready_enemy(&mut world, &mut physics, EnemyKind::Rat, at);
        assert!(!contact_damage(&mut world, &mut Vec::new()));
        assert_eq!(player_health(&world, player), before);

        let _ = world.despawn(rat);
        prefabs::enemy_bullet(
            &mut world,
            &mut physics,
            crate::enemies::ProjectileKind::Red,
            at,
            Vec2::X,
            &textures,
        );
        assert!(contact_damage(&mut world, &mut Vec::new()));
        assert!(player_health(&world, player) < before);
    }

    #[test]
    fn summoned_rounds_count_as_hits_on_the_enemy_but_stay_out_of_the_stats() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(&mut world, &mut physics, Vec2::ZERO, &textures);
        let slime = enemy(&mut world, &mut physics, EnemyKind::RedSlime);

        hit(&mut world, slime, 1, DamageSource::Summoned);
        assert_eq!(stats::current(&world).hits, 0);
        assert_eq!(world.get::<&Hits>(slime).map(|h| h.0).ok(), Some(1));

        hit(&mut world, slime, 1, DamageSource::Bullet);
        assert_eq!(stats::current(&world).hits, 1);
    }
}
