use glam::Vec2;
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::components::{
    Animation, Armageddon, Boss, BossHud, Clip, Cobweb, Expiring, Hatcher, Health, Minion, Player,
    Position, Puddle, Revives, RoomId, Rooted, Size, Spawning, WebSpitter,
};
use crate::enemies::{self, EnemyKind};
use crate::physics::Physics;
use crate::prefabs;

const HATCH_ORDER: [EnemyKind; 3] = [
    EnemyKind::RedSpiderling,
    EnemyKind::GreenSpiderling,
    EnemyKind::PurpleSpiderling,
];
const WEB_SIDE_OFFSET: f32 = 36.0;
const WEB_BOTTOM_INSET: f32 = 15.0;

fn player_center(world: &World) -> Option<Vec2> {
    world
        .query::<&Position>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|position| position.0)
}

pub(crate) fn tick_hatchers(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    dt: f32,
) {
    let mut hatches = Vec::new();
    for (position, hatcher, room, rooted) in world
        .query_mut::<(&Position, &mut Hatcher, Option<&RoomId>, Option<&Rooted>)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .without::<&crate::components::Frozen>()
    {
        if rooted.is_some() {
            continue;
        }
        hatcher.remaining -= dt;
        if hatcher.remaining <= 0.0 {
            hatcher.remaining = enemies::HATCH_COOLDOWN;
            let kind = HATCH_ORDER[hatcher.next % HATCH_ORDER.len()];
            hatcher.next += 1;
            hatches.push((position.0, kind, room.copied()));
        }
    }
    for (origin, kind, room) in hatches {
        prefabs::hatchling(world, physics, kind, origin, room, textures);
    }
}

pub(crate) fn tick_web_spitters(world: &mut World, textures: &Textures, dt: f32) {
    let target = player_center(world);
    let mut casts = Vec::new();
    let mut spits = Vec::new();
    for (entity, position, size, spitter, animation) in world
        .query_mut::<(Entity, &Position, &Size, &mut WebSpitter, &mut Animation)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .without::<&crate::components::Frozen>()
    {
        if spitter.casting {
            if animation.finished() {
                spitter.casting = false;
                spitter.remaining = enemies::WEB_COOLDOWN;
                animation.play(Clip {
                    flipped: animation.flipped(),
                    ..spitter.idle
                });
                spits.push((entity, position.0, size.0, animation.flipped()));
            }
        } else {
            spitter.remaining -= dt;
            if spitter.remaining <= 0.0 {
                spitter.casting = true;
                animation.play(Clip {
                    flipped: animation.flipped(),
                    ..enemies::spell_clip()
                });
                casts.push(entity);
            }
        }
    }
    for entity in casts {
        let _ = world.insert_one(entity, Rooted);
    }
    for (entity, position, size, facing_left) in spits {
        let _ = world.remove_one::<Rooted>(entity);
        let side = if facing_left {
            -WEB_SIDE_OFFSET
        } else {
            WEB_SIDE_OFFSET
        };
        let origin = Vec2::new(
            position.x + side,
            position.y + size.y / 2.0 - WEB_BOTTOM_INSET,
        );
        let direction = target.map_or(Vec2::X, |target| target - origin);
        prefabs::cobweb(world, origin, direction, textures);
    }
}

pub(crate) fn tick_armageddon(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    dt: f32,
) {
    let mut shots = Vec::new();
    for (position, armageddon, room) in world
        .query_mut::<(&Position, &mut Armageddon, Option<&RoomId>)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .without::<&crate::components::Frozen>()
    {
        armageddon.remaining -= dt;
        if armageddon.remaining <= 0.0 {
            armageddon.remaining = enemies::ARMAGEDDON_COOLDOWN;
            armageddon.angle += enemies::ARMAGEDDON_TURN;
            let direction = Vec2::new(armageddon.angle.cos(), armageddon.angle.sin());
            shots.push((position.0, armageddon.kind, direction, room.copied()));
        }
    }
    for (origin, kind, direction, room) in shots {
        let bullet = prefabs::enemy_bullet(world, physics, kind, origin, direction, textures);
        if let Some(room) = room {
            let _ = world.insert_one(bullet, room);
        }
    }
}

pub(super) fn clean_up_after_boss(world: &mut World) {
    let fallen = world
        .query::<&Health>()
        .with::<&Boss>()
        .iter()
        .any(|health| health.current <= 0);
    if !fallen {
        return;
    }
    let puddles: Vec<Entity> = world
        .query::<Entity>()
        .with::<(&Minion, &Puddle)>()
        .iter()
        .collect();
    for puddle in puddles {
        let _ = world.remove::<(Puddle, Spawning)>(puddle);
    }
    let reviving: Vec<Entity> = world
        .query::<Entity>()
        .with::<(&Minion, &Revives)>()
        .iter()
        .collect();
    for minion in reviving {
        let _ = world.remove_one::<Revives>(minion);
    }
    for health in world
        .query_mut::<&mut Health>()
        .with::<&Minion>()
        .without::<&Expiring>()
    {
        health.current = 0;
    }
    let webs: Vec<Entity> = world.query::<Entity>().with::<&Cobweb>().iter().collect();
    for web in webs {
        let _ = world.despawn(web);
    }
    super::clear_golem_effects(world);
    super::clear_magma(world);
}

pub(crate) fn update_boss_hud(world: &mut World) {
    let alive = world
        .query::<&Health>()
        .with::<&Boss>()
        .iter()
        .any(|health| health.current > 0);
    if alive {
        return;
    }
    let hud: Vec<Entity> = world.query::<Entity>().with::<&BossHud>().iter().collect();
    for entity in hud {
        let _ = world.despawn(entity);
    }
}

#[cfg(test)]
mod tests {
    use crate::assets::Textures;
    use crate::assets::tex;
    use crate::components::{Chase, EnemyBullet, Velocity};
    use crate::prefabs::finish_spawning;
    use crate::systems::enemies::reap_dead;
    use crate::systems::{chase_player, tick_spawning};

    use super::*;

    fn arena() -> (World, Physics, Textures) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(&mut world, &mut physics, Vec2::new(0.0, 0.0), &textures);
        (world, physics, textures)
    }

    fn boss(world: &mut World, physics: &mut Physics, kind: EnemyKind) -> Entity {
        let entity = prefabs::enemy(
            world,
            kind,
            Vec2::new(800.0, 800.0),
            1,
            &Textures::default(),
        );
        finish_spawning(world, physics, entity);
        entity
    }

    fn boss_defeated(world: &World) -> bool {
        let mut bosses = world.query::<&Health>().with::<&Boss>();
        let mut any = false;
        for health in bosses.iter() {
            any = true;
            if health.current > 0 {
                return false;
            }
        }
        any
    }

    fn count<T: hecs::Component>(world: &World) -> usize {
        world.query::<&T>().iter().count()
    }

    #[test]
    fn the_spider_hatches_a_rotating_brood_every_two_and_a_half_seconds() {
        let (mut world, mut physics, textures) = arena();
        boss(&mut world, &mut physics, EnemyKind::Spider);

        tick_hatchers(&mut world, &mut physics, &textures, 2.4);
        assert_eq!(count::<Minion>(&world), 0);
        tick_hatchers(&mut world, &mut physics, &textures, 0.2);
        tick_hatchers(&mut world, &mut physics, &textures, 2.6);
        tick_hatchers(&mut world, &mut physics, &textures, 2.6);

        assert_eq!(count::<Minion>(&world), 3);
        let kin_hp: Vec<i32> = {
            let mut hp: Vec<i32> = world
                .query::<&Health>()
                .with::<&Minion>()
                .iter()
                .map(|health| health.max)
                .collect();
            hp.sort_unstable();
            hp
        };
        assert_eq!(kin_hp, vec![3, 3, 5]);
    }

    #[test]
    fn hatchlings_are_active_immediately() {
        let (mut world, mut physics, textures) = arena();
        boss(&mut world, &mut physics, EnemyKind::Spider);
        tick_hatchers(&mut world, &mut physics, &textures, 2.6);

        assert_eq!(count::<Spawning>(&world), 0);
        let rooted = count::<Rooted>(&world);
        assert_eq!(rooted, 0);
    }

    #[test]
    fn the_spider_freezes_to_cast_and_then_spits_a_web_at_the_player() {
        let (mut world, mut physics, textures) = arena();
        let spider = boss(&mut world, &mut physics, EnemyKind::Spider);

        tick_web_spitters(&mut world, &textures, 4.9);
        assert!(world.get::<&Rooted>(spider).is_err());
        tick_web_spitters(&mut world, &textures, 0.2);
        assert!(world.get::<&Rooted>(spider).is_ok());

        chase_player(&mut world, &crate::flow::FlowField::default());
        let velocity = world
            .get::<&Velocity>(spider)
            .map(|v| v.0)
            .unwrap_or(Vec2::X);
        assert_eq!(velocity, Vec2::ZERO);
        tick_hatchers(&mut world, &mut physics, &textures, 10.0);
        assert_eq!(count::<Minion>(&world), 0);

        if let Ok(mut animation) = world.get::<&mut Animation>(spider) {
            animation.advance(1.0);
        }
        tick_web_spitters(&mut world, &textures, 0.016);

        assert!(world.get::<&Rooted>(spider).is_err());
        assert_eq!(count::<Cobweb>(&world), 1);
        let web_velocity = world
            .query::<&Velocity>()
            .with::<&Cobweb>()
            .iter()
            .next()
            .map(|v| v.0)
            .unwrap_or_default();
        assert!(
            web_velocity.x < 0.0 && web_velocity.y < 0.0,
            "{web_velocity:?}"
        );
    }

    #[test]
    fn the_king_slime_sprays_a_turning_spiral_and_rings() {
        let (mut world, mut physics, textures) = arena();
        boss(&mut world, &mut physics, EnemyKind::KingSlime);

        tick_armageddon(&mut world, &mut physics, &textures, 0.016);
        tick_armageddon(&mut world, &mut physics, &textures, 0.16);
        assert_eq!(count::<EnemyBullet>(&world), 2);

        crate::systems::tick_smashes(&mut world, &mut physics, &textures, 0.016);
        assert_eq!(count::<EnemyBullet>(&world), 27);
    }

    #[test]
    fn a_thrown_web_is_one_whole_picture_and_is_not_animated() {
        let mut world = World::new();
        let textures = Textures::default();

        let web = prefabs::cobweb(&mut world, Vec2::ZERO, Vec2::X, &textures);

        let uv = world
            .get::<&crate::components::Sprite>(web)
            .map(|s| s.uv)
            .ok();
        let whole = crate::components::Sprite::whole(textures.get(tex::COBWEB_PROJECTILE));
        assert_eq!(uv, Some(whole.uv));
        assert!(world.get::<&Animation>(web).is_err());
    }

    #[test]
    fn a_dead_boss_takes_its_brood_and_webs_with_it() {
        let (mut world, mut physics, textures) = arena();
        let spider = boss(&mut world, &mut physics, EnemyKind::Spider);
        tick_hatchers(&mut world, &mut physics, &textures, 2.6);
        prefabs::cobweb(&mut world, Vec2::ZERO, Vec2::X, &textures);

        clean_up_after_boss(&mut world);
        assert_eq!(count::<Cobweb>(&world), 1);

        if let Ok(mut health) = world.get::<&mut Health>(spider) {
            health.current = 0;
        }
        clean_up_after_boss(&mut world);
        reap_dead(&mut world, &mut physics);

        assert_eq!(count::<Cobweb>(&world), 0);
        let brood_alive = world
            .query::<&Health>()
            .with::<&Minion>()
            .without::<&Expiring>()
            .iter()
            .count();
        assert_eq!(brood_alive, 0);
        assert!(boss_defeated(&world));
    }

    #[test]
    fn the_boss_bar_goes_away_with_the_boss() {
        let (mut world, mut physics, textures) = arena();
        let spider = boss(&mut world, &mut physics, EnemyKind::Spider);
        prefabs::boss_hud(&mut world, "Spider king", &textures);

        update_boss_hud(&mut world);
        assert_eq!(count::<BossHud>(&world), 2);

        if let Ok(mut health) = world.get::<&mut Health>(spider) {
            health.current = 0;
        }
        update_boss_hud(&mut world);
        assert_eq!(count::<BossHud>(&world), 0);
    }

    #[test]
    fn bosses_ignore_the_floor_scaling_and_spawn_faster() {
        let mut world = World::new();
        let spider = prefabs::enemy(
            &mut world,
            EnemyKind::Spider,
            Vec2::ZERO,
            4,
            &Textures::default(),
        );
        let health = world.get::<&Health>(spider).map(|h| h.max).unwrap_or(0);
        let spawning = world
            .get::<&Spawning>(spider)
            .map(|s| s.total)
            .unwrap_or(0.0);
        assert_eq!(health, 75);
        assert_eq!(spawning, 1.5);
        let chase = world.get::<&Chase>(spider).map(|c| c.speed).unwrap_or(0.0);
        assert!((chase - 282.0).abs() < 1.0);
        let mut physics = Physics::new();
        tick_spawning(&mut world, &mut physics, 1.6);
        assert!(world.get::<&Spawning>(spider).is_err());
    }
}
