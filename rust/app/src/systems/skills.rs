use std::f32::consts::TAU;

use glam::Vec2;
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::components::{
    Animation, Body, Clip, DeathBurst, DeathHeal, EnemyBullet, Expiring, EyeShooter, Fuse, Health,
    Hits, Homing, Impact, Kin, Player, Position, RoomId, Size, Smash, Spawning, Velocity,
};
use crate::enemies::{self, ProjectileKind};
use crate::geometry;
use crate::physics::{Owner, Physics};
use crate::prefabs;

const HOMING_SMOOTHING: f32 = 1.0 / 16.0;
const STEP_RATE: f32 = 60.0;

fn radial(index: usize, count: usize) -> Vec2 {
    let angle = TAU * (index + 1) as f32 / count as f32;
    Vec2::new(angle.cos(), angle.sin())
}

fn burst_directions(hits: usize) -> Vec<Vec2> {
    let spiral = (0..hits).map(|index| {
        let angle = index as f32;
        Vec2::new(angle.cos(), angle.sin())
    });
    let ring = (0..hits).map(|index| radial(index, hits));
    spiral.chain(ring).collect()
}

fn fire(
    world: &mut World,
    physics: &mut Physics,
    kind: ProjectileKind,
    origin: Vec2,
    direction: Vec2,
    room: Option<RoomId>,
    textures: &Textures,
) {
    let bullet = prefabs::enemy_bullet(world, physics, kind, origin, direction, textures);
    if let Some(room) = room {
        let _ = world.insert_one(bullet, room);
    }
}

pub(crate) fn tick_smashes(world: &mut World, physics: &mut Physics, textures: &Textures, dt: f32) {
    let mut volleys = Vec::new();
    for (position, smash, room) in world
        .query_mut::<(&Position, &mut Smash, Option<&RoomId>)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .without::<&crate::components::Frozen>()
    {
        smash.remaining -= dt;
        if smash.remaining <= 0.0 {
            smash.remaining = enemies::SMASH_COOLDOWN;
            volleys.push((position.0, smash.kind, smash.count, room.copied()));
        }
    }
    for (origin, kind, count, room) in volleys {
        for index in 0..count {
            let direction = radial(index, count);
            fire(world, physics, kind, origin, direction, room, textures);
        }
    }
}

pub(crate) fn death_bursts(world: &mut World, physics: &mut Physics, textures: &Textures) {
    let dying: Vec<(Entity, Vec2, ProjectileKind, u32, Option<RoomId>)> = world
        .query::<(
            Entity,
            &Position,
            &DeathBurst,
            &Hits,
            &Health,
            Option<&RoomId>,
        )>()
        .without::<&Expiring>()
        .iter()
        .filter(|(_, _, _, _, health, _)| health.current <= 0)
        .map(|(entity, position, burst, hits, _, room)| {
            (entity, position.0, burst.kind, hits.0, room.copied())
        })
        .collect();
    for (entity, origin, kind, hits, room) in dying {
        let _ = world.remove_one::<DeathBurst>(entity);
        for direction in burst_directions(hits as usize) {
            fire(world, physics, kind, origin, direction, room, textures);
        }
    }
}

pub(crate) fn death_heals(world: &mut World) {
    let dying: Vec<(Entity, Vec2, DeathHeal)> = world
        .query::<(Entity, &Position, &DeathHeal, &Health)>()
        .without::<&Expiring>()
        .iter()
        .filter(|(_, _, _, health)| health.current <= 0)
        .map(|(entity, position, heal, _)| (entity, position.0, *heal))
        .collect();
    let mut healed: Vec<Entity> = Vec::new();
    for (entity, center, heal) in dying {
        let _ = world.remove_one::<DeathHeal>(entity);
        let box_size = Vec2::splat(heal.radius);
        for (other, position, size, health) in world
            .query_mut::<(Entity, &Position, &Size, &mut Health)>()
            .with::<&Kin>()
            .without::<&Expiring>()
            .without::<&Spawning>()
        {
            let nearby =
                other != entity && geometry::overlaps(center, box_size, position.0, size.0);
            if nearby && health.current > 0 {
                health.current = (health.current + heal.amount).min(health.max);
                healed.push(other);
            }
        }
        for other in healed.drain(..) {
            super::effects::show_change(world, other, heal.amount, rand::random_range(0.0..1.0));
        }
    }
}

pub(crate) fn tick_eye_shooters(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    dt: f32,
) {
    let mut shots = Vec::new();
    for (position, shooter, animation, room) in world
        .query_mut::<(&Position, &mut EyeShooter, &mut Animation, Option<&RoomId>)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .without::<&crate::components::Frozen>()
    {
        if shooter.attacking {
            if animation.finished() {
                shooter.attacking = false;
                shooter.remaining = enemies::SHOOT_COOLDOWN;
                animation.play(Clip {
                    flipped: animation.flipped(),
                    ..shooter.idle
                });
                shots.push((position.0, shooter.kind, room.copied()));
            }
        } else {
            shooter.remaining -= dt;
            if shooter.remaining <= 0.0 {
                shooter.attacking = true;
                animation.play(Clip {
                    flipped: animation.flipped(),
                    ..enemies::attack_clip()
                });
            }
        }
    }
    for (origin, kind, room) in shots {
        let spawn = origin - Vec2::new(0.0, kind.spec().size().y / 2.0);
        fire(world, physics, kind, spawn, Vec2::ZERO, room, textures);
    }
}

pub(crate) fn steer_homing(world: &mut World, dt: f32) {
    let target = world
        .query::<&Position>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|position| position.0);
    let Some(target) = target else {
        return;
    };
    let smoothing = (HOMING_SMOOTHING * dt * STEP_RATE).min(1.0);
    for (position, homing, velocity) in world
        .query_mut::<(&Position, &mut Homing, &mut Velocity)>()
        .without::<&Expiring>()
    {
        homing.aim += (target - homing.aim) * smoothing;
        velocity.0 = (homing.aim - position.0).normalize_or_zero() * homing.speed;
    }
}

pub(crate) fn burn_fuses(world: &mut World, physics: &mut Physics, dt: f32) {
    let mut spent = Vec::new();
    for (entity, fuse) in world
        .query_mut::<(Entity, &mut Fuse)>()
        .without::<&Expiring>()
    {
        fuse.remaining -= dt;
        if fuse.remaining <= 0.0 {
            spent.push(entity);
        }
    }
    for entity in spent {
        impact(world, physics, entity);
    }
}

pub(crate) fn enemy_bullet_hits(world: &mut World, physics: &mut Physics) {
    let hit: Vec<Entity> = world
        .query::<(Entity, &Body)>()
        .with::<&EnemyBullet>()
        .without::<&Expiring>()
        .iter()
        .filter(|(_, body)| {
            physics.touching(**body).iter().any(|owner| {
                matches!(owner, Owner::Wall) || is_player(world, owner) || is_solid(world, owner)
            })
        })
        .map(|(entity, _)| entity)
        .collect();
    for entity in hit {
        impact(world, physics, entity);
    }
}

fn is_solid(world: &World, owner: &Owner) -> bool {
    match owner {
        Owner::Entity(entity) => world.get::<&crate::components::Solid>(*entity).is_ok(),
        Owner::Wall => false,
    }
}

fn is_player(world: &World, owner: &Owner) -> bool {
    match owner {
        Owner::Entity(entity) => world.get::<&Player>(*entity).is_ok(),
        Owner::Wall => false,
    }
}

fn impact(world: &mut World, physics: &mut Physics, entity: Entity) {
    let Ok(clip) = world.get::<&Impact>(entity).map(|impact| impact.0) else {
        return;
    };
    if let Ok(body) = world.get::<&Body>(entity).map(|body| *body) {
        physics.remove(body);
    }
    let _ = world.remove_one::<Body>(entity);
    let _ = world.remove_one::<Homing>(entity);
    let _ = world.remove_one::<Fuse>(entity);
    let _ = world.insert_one(entity, Expiring);
    if let Ok(mut velocity) = world.get::<&mut Velocity>(entity) {
        velocity.0 = Vec2::ZERO;
    }
    if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
        animation.play(clip);
    }
}

pub(crate) fn count_hits(world: &mut World, victim: Entity) {
    if let Ok(mut hits) = world.get::<&mut Hits>(victim) {
        hits.0 += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::components::Color;
    use crate::enemies::EnemyKind;
    use crate::systems::{contact_damage, simulate};

    use super::*;

    fn arena() -> (World, Physics, Textures) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(&mut world, &mut physics, Vec2::new(0.0, 0.0), &textures);
        (world, physics, textures)
    }

    fn bullets(world: &World) -> usize {
        world.query::<&EnemyBullet>().iter().count()
    }

    fn ready(world: &mut World, physics: &mut Physics, kind: EnemyKind, at: Vec2) -> Entity {
        prefabs::ready_enemy(world, physics, kind, at)
    }

    #[test]
    fn a_slime_smashes_a_ring_of_five_every_two_seconds() {
        let (mut world, mut physics, textures) = arena();
        ready(
            &mut world,
            &mut physics,
            EnemyKind::Slime,
            Vec2::new(500.0, 500.0),
        );

        tick_smashes(&mut world, &mut physics, &textures, 1.0 / 60.0);
        assert_eq!(bullets(&world), 5);

        tick_smashes(&mut world, &mut physics, &textures, 1.0);
        assert_eq!(bullets(&world), 5);

        tick_smashes(&mut world, &mut physics, &textures, 1.1);
        assert_eq!(bullets(&world), 10);
    }

    #[test]
    fn the_smash_ring_covers_all_directions() {
        let (mut world, mut physics, textures) = arena();
        ready(
            &mut world,
            &mut physics,
            EnemyKind::Slime,
            Vec2::new(500.0, 500.0),
        );
        tick_smashes(&mut world, &mut physics, &textures, 1.0 / 60.0);

        let sum: Vec2 = world
            .query::<&Velocity>()
            .with::<&EnemyBullet>()
            .iter()
            .map(|velocity| velocity.0)
            .sum();
        assert!(sum.length() < 0.01, "{sum:?}");
    }

    #[test]
    fn a_red_slime_bursts_into_two_bullets_per_hit_when_it_dies() {
        let (mut world, mut physics, textures) = arena();
        let slime = ready(
            &mut world,
            &mut physics,
            EnemyKind::RedSlime,
            Vec2::new(500.0, 500.0),
        );
        if let Ok(mut hits) = world.get::<&mut Hits>(slime) {
            hits.0 = 3;
        }

        death_bursts(&mut world, &mut physics, &textures);
        assert_eq!(bullets(&world), 0);

        if let Ok(mut health) = world.get::<&mut Health>(slime) {
            health.current = 0;
        }
        death_bursts(&mut world, &mut physics, &textures);
        assert_eq!(bullets(&world), 6);

        death_bursts(&mut world, &mut physics, &textures);
        assert_eq!(bullets(&world), 6);
    }

    #[test]
    fn a_dying_green_spiderling_heals_only_nearby_spiderlings() {
        let (mut world, mut physics, _) = arena();
        let green = ready(
            &mut world,
            &mut physics,
            EnemyKind::GreenSpiderling,
            Vec2::new(500.0, 500.0),
        );
        let near = ready(
            &mut world,
            &mut physics,
            EnemyKind::RedSpiderling,
            Vec2::new(560.0, 500.0),
        );
        let far = ready(
            &mut world,
            &mut physics,
            EnemyKind::RedSpiderling,
            Vec2::new(900.0, 500.0),
        );
        let rat = ready(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(540.0, 540.0),
        );
        for entity in [near, far, rat] {
            if let Ok(mut health) = world.get::<&mut Health>(entity) {
                health.current = 1;
            }
        }
        if let Ok(mut health) = world.get::<&mut Health>(green) {
            health.current = 0;
        }

        death_heals(&mut world);

        let health = |entity| {
            world
                .get::<&Health>(entity)
                .map(|h| h.current)
                .unwrap_or(-1)
        };
        assert_eq!((health(near), health(far), health(rat)), (2, 1, 1));
    }

    #[test]
    fn an_eye_watcher_winds_up_for_three_seconds_then_fires_a_homing_orb() {
        let (mut world, mut physics, textures) = arena();
        let eye = ready(
            &mut world,
            &mut physics,
            EnemyKind::EyeWatcher,
            Vec2::new(500.0, 500.0),
        );

        tick_eye_shooters(&mut world, &mut physics, &textures, 2.9);
        assert_eq!(bullets(&world), 0);
        tick_eye_shooters(&mut world, &mut physics, &textures, 0.2);
        let attacking = world.get::<&EyeShooter>(eye).map(|s| s.attacking).ok();
        assert_eq!(attacking, Some(true));
        assert_eq!(bullets(&world), 0);

        if let Ok(mut animation) = world.get::<&mut Animation>(eye) {
            animation.advance(1.0);
        }
        tick_eye_shooters(&mut world, &mut physics, &textures, 0.016);

        assert_eq!(bullets(&world), 1);
        assert_eq!(world.query::<&Homing>().iter().count(), 1);
        assert_eq!(world.query::<&Fuse>().iter().count(), 1);
        let attacking = world.get::<&EyeShooter>(eye).map(|s| s.attacking).ok();
        assert_eq!(attacking, Some(false));
    }

    #[test]
    fn a_homing_orb_turns_towards_a_moved_player_and_burns_out() {
        let (mut world, mut physics, textures) = arena();
        let orb = prefabs::enemy_bullet(
            &mut world,
            &mut physics,
            ProjectileKind::EyeWatcher,
            Vec2::new(500.0, 0.0),
            Vec2::new(0.0, 1.0),
            &textures,
        );
        for _ in 0..30 {
            steer_homing(&mut world, 1.0 / 60.0);
        }
        let velocity = world.get::<&Velocity>(orb).map(|v| v.0).unwrap_or_default();
        assert!(
            velocity.x < 0.0 && velocity.y.abs() < velocity.x.abs(),
            "{velocity:?}"
        );

        burn_fuses(&mut world, &mut physics, 4.9);
        assert!(world.get::<&Expiring>(orb).is_err());
        burn_fuses(&mut world, &mut physics, 0.2);
        assert!(world.get::<&Expiring>(orb).is_ok());
        assert!(world.get::<&Body>(orb).is_err());
    }

    #[test]
    fn a_bullet_hurts_the_player_and_is_used_up() {
        let (mut world, mut physics, textures) = arena();
        let bullet = prefabs::enemy_bullet(
            &mut world,
            &mut physics,
            ProjectileKind::Green,
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            &textures,
        );
        simulate(&mut world, &mut physics, 1.0 / 60.0);

        let hurt = contact_damage(&mut world, &mut Vec::new());
        enemy_bullet_hits(&mut world, &mut physics);

        assert!(hurt);
        assert!(world.get::<&Expiring>(bullet).is_ok());
        assert!(world.get::<&Body>(bullet).is_err());
    }

    #[test]
    fn a_bullet_stops_at_a_wall() {
        let (mut world, mut physics, textures) = arena();
        physics.add_walls(&[(5, 0)], 64.0);
        let bullet = prefabs::enemy_bullet(
            &mut world,
            &mut physics,
            ProjectileKind::Green,
            Vec2::new(200.0, 32.0),
            Vec2::new(1.0, 0.0),
            &textures,
        );
        for _ in 0..30 {
            simulate(&mut world, &mut physics, 1.0 / 60.0);
        }

        enemy_bullet_hits(&mut world, &mut physics);

        assert!(world.get::<&Expiring>(bullet).is_ok());
        let tint = world.get::<&Color>(bullet).map(|c| c.0.w).unwrap_or(0.0);
        assert_eq!(tint, 1.0);
    }
}
