use glam::Vec2;
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Animation, BreakKind, Breakable, Broken, Clip, Color, Enemy, Expiring, Flame, Health, HitFlash,
    Invulnerable, Light, Outline, Player, PoisonPool, Poisoned, Position, Size, Solid, Spawning,
};
use crate::geometry;
use crate::items::ItemKind;
use crate::objects::{
    EXPLOSION_DAMAGE, EXPLOSION_REACH, POISON_DAMAGE, POISON_TICK_SECONDS, POOL_FADE_SECONDS,
    POOL_HITBOX,
};
use crate::physics::Physics;
use crate::prefabs;
use crate::systems::damage::hurt_player;
use crate::systems::enemies::FLASH_SECONDS;

const DEATH_CLIP: Clip = Clip {
    row: 1,
    frames: 4,
    frame_time: 0.1,
    looping: false,
    flipped: false,
};
const POISON_OUTLINE: glam::Vec4 = glam::Vec4::new(0.35, 1.0, 0.25, 0.9);
const POOL_DROP: f32 = 24.0;
const FLAME_PEAK_INTENSITY: f32 = 3.0;
const FLAME_LIGHT_DROP: f32 = 20.0;

pub(crate) fn break_objects(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    mut roll: impl FnMut(usize) -> usize,
) {
    let fallen: Vec<(Entity, BreakKind, Vec2, Solid)> = world
        .query::<(Entity, &Breakable, &Health, &Position, &Solid)>()
        .without::<&Broken>()
        .iter()
        .filter(|(_, _, health, _, _)| health.current <= 0)
        .map(|(entity, breakable, _, position, solid)| (entity, breakable.kind, position.0, *solid))
        .collect();
    for (entity, kind, center, solid) in fallen {
        physics.remove_collider(solid.0);
        let _ = world.remove_one::<Solid>(entity);
        let _ = world.insert_one(entity, Broken);
        if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
            animation.play(DEATH_CLIP);
        }
        let item = ItemKind::COMMON[roll(ItemKind::COMMON.len())];
        prefabs::item(world, item, center, true, Vec2::ZERO, textures);
        match kind {
            BreakKind::Explosive => explode(world, entity, center, textures, sounds),
            BreakKind::Poison => {
                prefabs::poison_pool(world, center + Vec2::new(0.0, POOL_DROP), textures);
            }
            _ => {}
        }
    }
}

fn explode(
    world: &mut World,
    source: Entity,
    center: Vec2,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
) {
    sounds.push(AudioCommand::Effect(SoundEffect::Explosion));
    prefabs::explosion(world, center - Vec2::new(0.0, 20.0), textures);
    let reach = BreakKind::Explosive.collider().size + EXPLOSION_REACH * 2.0;
    let victims: Vec<Entity> = world
        .query::<(Entity, &Position, &Size)>()
        .with::<&Health>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .filter(|(entity, position, size)| {
            *entity != source && geometry::overlaps(center, reach, position.0, size.0)
        })
        .map(|(entity, _, _)| entity)
        .collect();
    for entity in victims {
        if world.get::<&Player>(entity).is_ok() {
            if world.get::<&Invulnerable>(entity).is_err() {
                hurt_player(world, entity, EXPLOSION_DAMAGE, sounds);
            }
            continue;
        }
        let hurt = world.get::<&Enemy>(entity).is_ok() || world.get::<&Breakable>(entity).is_ok();
        if !hurt {
            continue;
        }
        if let Ok(mut health) = world.get::<&mut Health>(entity) {
            health.current = (health.current - EXPLOSION_DAMAGE).max(0);
        }
        let _ = world.insert_one(
            entity,
            HitFlash {
                remaining: FLASH_SECONDS,
            },
        );
    }
}

pub(crate) fn tick_poison(world: &mut World, dt: f32, sounds: &mut Vec<AudioCommand>) {
    let mut pools = Vec::new();
    let mut gone = Vec::new();
    for (entity, pool, position, color) in
        world.query_mut::<(Entity, &mut PoisonPool, &Position, &mut Color)>()
    {
        pool.remaining -= dt;
        pool.tick -= dt;
        color.0.w = (pool.remaining / POOL_FADE_SECONDS).clamp(0.0, 1.0);
        if pool.remaining <= 0.0 {
            gone.push(entity);
            continue;
        }
        let due = pool.tick <= 0.0;
        if due {
            pool.tick = POISON_TICK_SECONDS;
        }
        pools.push((position.0, due));
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
    let standing: Vec<(Entity, bool, bool)> = world
        .query::<(Entity, &Position, &Size, Option<&Player>)>()
        .with::<&Health>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .filter(|(entity, _, _, player)| player.is_some() || world.get::<&Enemy>(*entity).is_ok())
        .filter_map(|(entity, position, size, player)| {
            let mut inside = false;
            let mut hurt = false;
            for (center, due) in &pools {
                if geometry::overlaps(*center, POOL_HITBOX, position.0, size.0) {
                    inside = true;
                    hurt |= *due;
                }
            }
            inside.then_some((entity, hurt, player.is_some()))
        })
        .collect();
    let inside: Vec<Entity> = standing.iter().map(|(entity, _, _)| *entity).collect();
    for (entity, hurt, is_player) in standing {
        if hurt {
            if is_player {
                hurt_player(world, entity, POISON_DAMAGE, sounds);
            } else if let Ok(mut health) = world.get::<&mut Health>(entity) {
                health.current = (health.current - POISON_DAMAGE).max(0);
            }
        }
        let _ = world.insert_one(entity, Poisoned);
        if world.get::<&HitFlash>(entity).is_err() {
            let _ = world.insert_one(entity, Outline(POISON_OUTLINE));
        }
    }
    let leaving: Vec<Entity> = world
        .query::<Entity>()
        .with::<&Poisoned>()
        .iter()
        .filter(|entity| !inside.contains(entity))
        .collect();
    for entity in leaving {
        let _ = world.remove_one::<Poisoned>(entity);
        if world.get::<&HitFlash>(entity).is_err() {
            let _ = world.remove_one::<Outline>(entity);
        }
    }
}

pub(crate) fn flicker_flames(world: &mut World) {
    for (animation, light) in world
        .query_mut::<(&Animation, &mut Light)>()
        .with::<&Flame>()
    {
        let frame = animation.frame();
        let frames = animation.clip_frames().max(1) as f32;
        light.intensity = FLAME_PEAK_INTENSITY * frame.min(1) as f32 / frames;
        light.offset.y = FLAME_LIGHT_DROP * frame as f32 / frames;
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{Armor, Pickup};

    use super::*;

    fn arena() -> (World, Physics, Textures) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(&mut world, &mut physics, Vec2::new(0.0, 0.0), &textures);
        (world, physics, textures)
    }

    fn kill(world: &mut World, entity: Entity) {
        if let Ok(mut health) = world.get::<&mut Health>(entity) {
            health.current = 0;
        }
    }

    #[test]
    fn a_pot_takes_four_hits_then_breaks_and_drops_an_item() {
        let (mut world, mut physics, textures) = arena();
        let pot = prefabs::breakable(
            &mut world,
            &mut physics,
            BreakKind::Pot,
            Vec2::new(500.0, 500.0),
            &textures,
        );
        break_objects(&mut world, &mut physics, &textures, &mut Vec::new(), |_| 0);
        assert!(world.get::<&Solid>(pot).is_ok());
        assert_eq!(world.query::<&Pickup>().iter().count(), 0);

        kill(&mut world, pot);
        break_objects(&mut world, &mut physics, &textures, &mut Vec::new(), |_| 0);

        assert!(world.get::<&Broken>(pot).is_ok());
        assert!(world.get::<&Solid>(pot).is_err());
        assert_eq!(world.query::<&Pickup>().iter().count(), 1);

        break_objects(&mut world, &mut physics, &textures, &mut Vec::new(), |_| 0);
        assert_eq!(world.query::<&Pickup>().iter().count(), 1);
    }

    #[test]
    fn a_player_bullet_chips_an_object_and_stops_at_it() {
        let (mut world, mut physics, textures) = arena();
        let barrel = prefabs::breakable(
            &mut world,
            &mut physics,
            BreakKind::Barrel,
            Vec2::new(300.0 - 96.0, 100.0 - 96.0),
            &textures,
        );
        let bullet = prefabs::bullet(
            &mut world,
            &mut physics,
            Vec2::new(150.0, 100.0),
            Vec2::new(700.0, 0.0),
            1,
            crate::render::TextureId::WHITE,
        );
        let mut sounds = Vec::new();
        for _ in 0..40 {
            crate::systems::simulate(&mut world, &mut physics, 1.0 / 60.0);
            crate::systems::bullet_hits(&mut world, &mut physics, &mut sounds);
        }
        let health = world.get::<&Health>(barrel).map(|h| h.current).unwrap_or(0);
        assert_eq!(health, 3);
        assert!(world.get::<&Expiring>(bullet).is_ok());
    }

    #[test]
    fn an_explosive_barrel_hurts_what_is_near_and_nothing_far() {
        let (mut world, mut physics, textures) = arena();
        let barrel_center = Vec2::new(500.0, 500.0);
        let barrel = prefabs::breakable(
            &mut world,
            &mut physics,
            BreakKind::Explosive,
            barrel_center - Vec2::splat(96.0),
            &textures,
        );
        let near = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Rat,
            barrel_center + Vec2::new(100.0, 0.0),
        );
        let far = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Rat,
            barrel_center + Vec2::new(600.0, 0.0),
        );
        let next_barrel = prefabs::breakable(
            &mut world,
            &mut physics,
            BreakKind::Barrel,
            barrel_center + Vec2::new(120.0, 60.0) - Vec2::splat(96.0),
            &textures,
        );
        let mut sounds = Vec::new();

        kill(&mut world, barrel);
        break_objects(&mut world, &mut physics, &textures, &mut sounds, |_| 0);

        let hp = |entity| {
            world
                .get::<&Health>(entity)
                .map(|h| h.current)
                .unwrap_or(-1)
        };
        assert_eq!((hp(near), hp(far)), (9, 12));
        assert_eq!(hp(next_barrel), 1);
        assert!(sounds.contains(&AudioCommand::Effect(SoundEffect::Explosion)));
        assert_eq!(
            world.query::<&Light>().with::<&Expiring>().iter().count(),
            1
        );
    }

    #[test]
    fn an_explosion_next_to_the_player_hurts_through_the_armor() {
        let (mut world, mut physics, textures) = arena();
        let barrel = prefabs::breakable(
            &mut world,
            &mut physics,
            BreakKind::Explosive,
            Vec2::new(40.0, -96.0),
            &textures,
        );
        kill(&mut world, barrel);
        let mut sounds = Vec::new();

        break_objects(&mut world, &mut physics, &textures, &mut sounds, |_| 0);

        let armor = world
            .query::<&Armor>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|a| a.current);
        assert_eq!(armor, Some(2));
        assert_eq!(
            world
                .query::<&Invulnerable>()
                .with::<&Player>()
                .iter()
                .count(),
            1
        );
    }

    #[test]
    fn the_flamethrower_burns_only_on_the_last_frame_and_lights_up() {
        let (mut world, mut physics, textures) = arena();
        let flame = prefabs::flamethrower(&mut world, Vec2::new(-32.0, -64.0), &textures);
        let _ = &mut physics;

        assert!(!crate::systems::contact_damage(&mut world, &mut Vec::new()));
        flicker_flames(&mut world);
        let dark = world
            .get::<&Light>(flame)
            .map(|l| l.intensity)
            .unwrap_or(-1.0);
        assert_eq!(dark, 0.0);

        if let Ok(mut animation) = world.get::<&mut Animation>(flame) {
            animation.advance(0.6);
        }
        flicker_flames(&mut world);
        let lit = world
            .get::<&Light>(flame)
            .map(|l| l.intensity)
            .unwrap_or(-1.0);
        assert!(lit > 0.5, "{lit}");
        assert!(crate::systems::contact_damage(&mut world, &mut Vec::new()));
    }

    #[test]
    fn breaking_a_poison_barrel_leaves_a_puddle_but_a_plain_barrel_does_not() {
        let (mut world, mut physics, textures) = arena();
        let plain = prefabs::breakable(
            &mut world,
            &mut physics,
            BreakKind::Barrel,
            Vec2::new(500.0, 500.0),
            &textures,
        );
        let poison = prefabs::breakable(
            &mut world,
            &mut physics,
            BreakKind::Poison,
            Vec2::new(900.0, 500.0),
            &textures,
        );
        kill(&mut world, plain);
        break_objects(&mut world, &mut physics, &textures, &mut Vec::new(), |_| 0);
        assert_eq!(world.query::<&PoisonPool>().iter().count(), 0);

        kill(&mut world, poison);
        break_objects(&mut world, &mut physics, &textures, &mut Vec::new(), |_| 0);
        assert_eq!(world.query::<&PoisonPool>().iter().count(), 1);
    }

    #[test]
    fn standing_in_the_puddle_hurts_once_a_second_and_marks_green() {
        let (mut world, mut physics, textures) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Slime,
            Vec2::new(700.0, 700.0),
        );
        let far = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Slime,
            Vec2::new(2000.0, 700.0),
        );
        prefabs::poison_pool(&mut world, Vec2::new(700.0, 700.0), &textures);
        let health =
            |world: &World, entity| world.get::<&Health>(entity).map(|h| h.current).unwrap_or(0);
        let start = health(&world, rat);
        let mut sounds = Vec::new();

        tick_poison(&mut world, 0.016, &mut sounds);
        assert_eq!(health(&world, rat), start - POISON_DAMAGE);
        assert!(world.get::<&Poisoned>(rat).is_ok());
        assert!(world.get::<&Outline>(rat).is_ok());

        tick_poison(&mut world, 0.5, &mut sounds);
        assert_eq!(health(&world, rat), start - POISON_DAMAGE);

        tick_poison(&mut world, 0.6, &mut sounds);
        assert_eq!(health(&world, rat), start - POISON_DAMAGE * 2);
        assert_eq!(health(&world, far), start);
        assert!(world.get::<&Poisoned>(far).is_err());
    }

    #[test]
    fn the_puddle_hurts_the_player_too_and_dries_up_after_a_while() {
        let (mut world, mut physics, textures) = arena();
        let _ = &mut physics;
        let player = world
            .query::<Entity>()
            .with::<&Player>()
            .iter()
            .next()
            .expect("player");
        let at = world
            .get::<&Position>(player)
            .map(|p| p.0)
            .unwrap_or_default();
        if let Ok(mut armor) = world.get::<&mut crate::components::Armor>(player) {
            armor.current = 0;
        }
        let before = world.get::<&Health>(player).map(|h| h.current).unwrap_or(0);
        prefabs::poison_pool(&mut world, at, &textures);
        let mut sounds = Vec::new();

        tick_poison(&mut world, 0.016, &mut sounds);
        let after = world.get::<&Health>(player).map(|h| h.current).unwrap_or(0);
        assert_eq!(after, before - POISON_DAMAGE);

        for _ in 0..20 {
            tick_poison(&mut world, 0.5, &mut sounds);
        }
        assert_eq!(world.query::<&PoisonPool>().iter().count(), 0);
        assert!(world.get::<&Poisoned>(player).is_err());
    }
}
