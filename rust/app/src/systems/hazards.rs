use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::audio::AudioCommand;
use crate::components::{
    Animation, ArrowTrap, Body, Bullet, Clip, Color, EnemyBullet, Expiring, FloorPatch, Health,
    Layer, Lit, PatchKind, Player, Position, Size, Sprite, TrapArrow,
};
use crate::geometry;
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::systems::damage::hurt_player;

const ARROW_PERIOD: f32 = 2.4;
const ARROW_SPEED: f32 = 520.0;
const ARROW_SIZE: Vec2 = Vec2::new(20.0, 56.0);
const ARROW_HITBOX: Vec2 = Vec2::new(12.0, 40.0);
const ARROW_DAMAGE: i32 = 1;
const ARROW_REACH: f32 = 640.0;
const ARROW_RECHECK: f32 = 0.25;
const SLIT_OFFSET: f32 = 4.0;
const IMPACT_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const IMPACT_FRAMES: usize = 4;
const WEB_BULLET_DRAG: f32 = 30.0;
const WEB_CATCHES_BELOW: f32 = 150.0;

fn impact(world: &mut World, at: Vec2, textures: &Textures) {
    let clip = Clip {
        row: 0,
        frames: IMPACT_FRAMES,
        frame_time: 0.09,
        looping: false,
        flipped: false,
    };
    world.spawn((
        Expiring,
        Position(at),
        Size(IMPACT_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(tex::ARROW_HIT), 0, 0, IMPACT_FRAMES, 1),
        Animation::new(IMPACT_FRAMES, 1, clip),
        Layer(3),
        Lit,
    ));
}

pub(crate) fn tick_arrow_traps(
    world: &mut World,
    navigation: &Navigation,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    let Some((player, player_at, player_size)) = world
        .query::<(Entity, &Position, &Size)>()
        .with::<(&Player, &Health)>()
        .iter()
        .next()
        .map(|(entity, position, size)| (entity, position.0, size.0))
    else {
        return;
    };
    let mut shots = Vec::new();
    for (position, size, trap) in world.query_mut::<(&Position, &Size, &mut ArrowTrap)>() {
        trap.timer -= dt;
        if trap.timer > 0.0 {
            continue;
        }
        if position.0.distance(player_at) > ARROW_REACH {
            trap.timer = ARROW_RECHECK;
            continue;
        }
        trap.timer = ARROW_PERIOD;
        let slit = position.0 + Vec2::new(0.0, SLIT_OFFSET);
        shots.push((slit, size.0.y / 2.0 - SLIT_OFFSET + 2.0));
    }
    for (slit, free_after) in shots {
        world.spawn((
            TrapArrow {
                origin_y: slit.y,
                travelled: 0.0,
                free_after,
            },
            Position(slit),
            Size(Vec2::new(ARROW_SIZE.x, 2.0)),
            Color(Vec4::ONE),
            Sprite::whole(textures.get(tex::TRAP_ARROW)),
            Layer(2),
            Lit,
        ));
    }
    let flying: Vec<(Entity, f32, f32, f32, f32)> = world
        .query::<(Entity, &Position, &TrapArrow)>()
        .iter()
        .map(|(entity, position, arrow)| {
            (
                entity,
                position.0.x,
                arrow.origin_y,
                arrow.travelled,
                arrow.free_after,
            )
        })
        .collect();
    for (entity, x, origin_y, travelled, free_after) in flying {
        let travelled = travelled + ARROW_SPEED * dt;
        let tip_y = origin_y + travelled;
        let hitbox_at = Vec2::new(x, tip_y - ARROW_HITBOX.y / 2.0);
        let tip = Vec2::new(x, tip_y);
        if travelled > free_after && navigation.is_blocked(hitbox_at, ARROW_HITBOX) {
            let _ = world.despawn(entity);
            impact(world, tip, textures);
            continue;
        }
        if travelled > free_after
            && geometry::overlaps(hitbox_at, ARROW_HITBOX, player_at, player_size)
        {
            hurt_player(world, player, ARROW_DAMAGE, sounds);
            let _ = world.despawn(entity);
            impact(world, tip, textures);
            continue;
        }
        let visible = travelled.clamp(2.0, ARROW_SIZE.y);
        if let Ok(mut arrow) = world.get::<&mut TrapArrow>(entity) {
            arrow.travelled = travelled;
        }
        if let Ok(mut position) = world.get::<&mut Position>(entity) {
            position.0 = Vec2::new(x, tip_y - visible / 2.0);
        }
        if let Ok(mut size) = world.get::<&mut Size>(entity) {
            size.0 = Vec2::new(ARROW_SIZE.x, visible);
        }
        if let Ok(mut sprite) = world.get::<&mut Sprite>(entity) {
            sprite.uv[1] = 1.0 - visible / ARROW_SIZE.y;
        }
    }
}

pub(crate) fn slow_bullets_in_webs(world: &mut World, physics: &mut Physics, dt: f32) {
    let webs: Vec<(glam::Vec2, glam::Vec2)> = world
        .query::<(&Position, &Size, &FloorPatch)>()
        .iter()
        .filter(|(_, _, patch)| patch.0 == PatchKind::Web)
        .map(|(position, size, _)| (position.0, size.0))
        .collect();
    if webs.is_empty() {
        return;
    }
    let keep = (-WEB_BULLET_DRAG * dt).exp();
    let mut caught = Vec::new();
    let bullets: Vec<(Entity, Body, glam::Vec2)> = world
        .query::<(Entity, &Body, &Position)>()
        .without::<&Expiring>()
        .iter()
        .filter(|(entity, _, _)| {
            world.get::<&Bullet>(*entity).is_ok() || world.get::<&EnemyBullet>(*entity).is_ok()
        })
        .map(|(entity, body, position)| (entity, *body, position.0))
        .collect();
    for (entity, body, at) in bullets {
        if !webs
            .iter()
            .any(|(web, size)| geometry::contains(*web, *size, at))
        {
            continue;
        }
        let Some(velocity) = physics.velocity(body) else {
            continue;
        };
        let slowed = velocity * keep;
        if slowed.length() < WEB_CATCHES_BELOW {
            caught.push(entity);
        } else {
            physics.set_velocity(body, slowed);
        }
    }
    for entity in caught {
        physics.detach(world, entity);
        let _ = world.insert_one(entity, Expiring);
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{Armor, Invulnerable};
    use crate::physics::Physics;
    use crate::prefabs;

    use super::*;

    fn world_with_player(at: Vec2) -> (World, Entity) {
        let mut world = World::new();
        let player = prefabs::player(&mut world, &mut Physics::new(), at, &Textures::default());
        if let Ok(mut armor) = world.get::<&mut Armor>(player) {
            armor.current = 0;
        }
        (world, player)
    }

    fn trap_at(world: &mut World, at: Vec2, timer: f32) {
        world.spawn((Position(at), Size(Vec2::splat(64.0)), ArrowTrap { timer }));
    }

    fn arrows(world: &World) -> usize {
        world.query::<&TrapArrow>().iter().count()
    }

    #[test]
    fn a_trap_in_reach_fires_an_arrow_downwards() {
        let (mut world, _) = world_with_player(Vec2::new(200.0, 500.0));
        trap_at(&mut world, Vec2::new(200.0, 100.0), 0.0);
        let navigation = Navigation::new(&[]);

        tick_arrow_traps(
            &mut world,
            &navigation,
            &Textures::default(),
            &mut Vec::new(),
            0.016,
        );

        assert_eq!(arrows(&world), 1);
        let y = world
            .query::<&Position>()
            .with::<&TrapArrow>()
            .iter()
            .next()
            .map(|p| p.0.y);
        assert!(y.is_some_and(|y| y > 100.0));
    }

    #[test]
    fn a_trap_far_away_stays_quiet() {
        let (mut world, _) = world_with_player(Vec2::new(200.0, 3000.0));
        trap_at(&mut world, Vec2::new(200.0, 100.0), 0.0);
        let navigation = Navigation::new(&[]);

        tick_arrow_traps(
            &mut world,
            &navigation,
            &Textures::default(),
            &mut Vec::new(),
            0.016,
        );

        assert_eq!(arrows(&world), 0);
    }

    #[test]
    fn an_arrow_that_reaches_the_player_hurts_once_and_disappears() {
        let (mut world, player) = world_with_player(Vec2::new(200.0, 300.0));
        trap_at(&mut world, Vec2::new(200.0, 100.0), 0.0);
        let navigation = Navigation::new(&[]);
        let before = world.get::<&Health>(player).map(|h| h.current).unwrap_or(0);

        for _ in 0..90 {
            tick_arrow_traps(
                &mut world,
                &navigation,
                &Textures::default(),
                &mut Vec::new(),
                0.016,
            );
        }

        let after = world.get::<&Health>(player).map(|h| h.current).unwrap_or(0);
        assert_eq!(after, before - ARROW_DAMAGE);
        assert!(world.get::<&Invulnerable>(player).is_ok());
    }

    #[test]
    fn an_arrow_stops_at_a_wall() {
        let (mut world, _) = world_with_player(Vec2::new(600.0, 400.0));
        trap_at(&mut world, Vec2::new(200.0, 100.0), 0.0);
        let navigation = Navigation::new(&[(3, 3), (3, 4)]);

        for _ in 0..60 {
            tick_arrow_traps(
                &mut world,
                &navigation,
                &Textures::default(),
                &mut Vec::new(),
                0.016,
            );
        }

        assert!(world.query::<&TrapArrow>().iter().all(|_| false) || arrows(&world) <= 1);
        let max_y = world
            .query::<&Position>()
            .with::<&TrapArrow>()
            .iter()
            .map(|p| p.0.y)
            .fold(0.0, f32::max);
        assert!(max_y < 3.0 * 64.0 + 40.0, "{max_y}");
    }

    fn web_at(world: &mut World, at: Vec2) {
        world.spawn((
            Position(at),
            Size(Vec2::splat(96.0)),
            FloorPatch(PatchKind::Web),
        ));
    }

    fn bullet_at(world: &mut World, physics: &mut Physics, at: Vec2, enemy: bool) -> Entity {
        let entity = world.spawn((Position(at),));
        let body = if enemy {
            let _ = world.insert_one(entity, EnemyBullet);
            physics.add_enemy_bullet(entity, at, Vec2::new(700.0, 0.0), Vec2::splat(8.0))
        } else {
            let _ = world.insert_one(entity, Bullet { damage: 1 });
            physics.add_bullet(entity, at, Vec2::new(700.0, 0.0), Vec2::splat(8.0))
        };
        let _ = world.insert_one(entity, body);
        entity
    }

    fn speed(world: &World, physics: &Physics, entity: Entity) -> f32 {
        world
            .get::<&Body>(entity)
            .ok()
            .and_then(|body| physics.velocity(*body))
            .map_or(0.0, |velocity| velocity.length())
    }

    #[test]
    fn bullets_of_both_sides_slow_down_inside_a_web_and_not_outside() {
        let mut world = World::new();
        let mut physics = Physics::new();
        web_at(&mut world, Vec2::new(300.0, 300.0));
        let player_shot = bullet_at(&mut world, &mut physics, Vec2::new(300.0, 300.0), false);
        let enemy_shot = bullet_at(&mut world, &mut physics, Vec2::new(300.0, 310.0), true);
        let free_shot = bullet_at(&mut world, &mut physics, Vec2::new(900.0, 900.0), false);

        slow_bullets_in_webs(&mut world, &mut physics, 1.0 / 60.0);

        assert!(speed(&world, &physics, player_shot) < 700.0 * 0.9);
        assert!(speed(&world, &physics, enemy_shot) < 700.0 * 0.9);
        assert!((speed(&world, &physics, free_shot) - 700.0).abs() < 0.5);
    }

    #[test]
    fn a_bullet_that_stays_in_the_web_gets_caught() {
        let mut world = World::new();
        let mut physics = Physics::new();
        web_at(&mut world, Vec2::new(300.0, 300.0));
        let shot = bullet_at(&mut world, &mut physics, Vec2::new(300.0, 300.0), false);

        for _ in 0..120 {
            slow_bullets_in_webs(&mut world, &mut physics, 1.0 / 60.0);
        }

        assert!(world.get::<&Expiring>(shot).is_ok());
    }

    #[test]
    fn a_bullet_fired_into_a_web_stops_within_a_fraction_of_its_width() {
        let mut world = World::new();
        let mut physics = Physics::new();
        web_at(&mut world, Vec2::new(300.0, 300.0));
        let start = Vec2::new(252.0, 300.0);
        let shot = bullet_at(&mut world, &mut physics, start, false);

        for _ in 0..60 {
            if let Ok(body) = world.get::<&Body>(shot).map(|body| *body) {
                if let Some(at) = physics.position(body) {
                    if let Ok(mut position) = world.get::<&mut Position>(shot) {
                        position.0 = at;
                    }
                }
            }
            slow_bullets_in_webs(&mut world, &mut physics, 1.0 / 60.0);
            physics.step(1.0 / 60.0);
        }

        assert!(world.get::<&Expiring>(shot).is_ok());
    }

    fn impacts(world: &World) -> usize {
        world.query::<&Expiring>().iter().count()
    }

    #[test]
    fn an_arrow_that_hits_a_wall_leaves_a_hit_animation() {
        let (mut world, _) = world_with_player(Vec2::new(600.0, 400.0));
        trap_at(&mut world, Vec2::new(200.0, 100.0), 0.0);
        let navigation = Navigation::new(&[(3, 3), (3, 4)]);

        for _ in 0..80 {
            tick_arrow_traps(
                &mut world,
                &navigation,
                &Textures::default(),
                &mut Vec::new(),
                0.016,
            );
        }

        assert!(impacts(&world) >= 1);
    }

    #[test]
    fn an_arrow_that_hits_the_player_leaves_a_hit_animation_too() {
        let (mut world, _) = world_with_player(Vec2::new(200.0, 300.0));
        trap_at(&mut world, Vec2::new(200.0, 100.0), 0.0);
        let navigation = Navigation::new(&[]);

        for _ in 0..40 {
            tick_arrow_traps(
                &mut world,
                &navigation,
                &Textures::default(),
                &mut Vec::new(),
                0.016,
            );
        }

        assert!(impacts(&world) >= 1);
    }

    #[test]
    fn the_arrow_grows_out_of_the_slit_inside_its_own_wall_tile() {
        let (mut world, _) = world_with_player(Vec2::new(600.0, 400.0));
        trap_at(&mut world, Vec2::new(224.0, 96.0), 0.0);
        let navigation = Navigation::new(&[(3, 1), (9, 15)]);

        tick_arrow_traps(
            &mut world,
            &navigation,
            &Textures::default(),
            &mut Vec::new(),
            0.016,
        );
        let first = world
            .query::<&Size>()
            .with::<&TrapArrow>()
            .iter()
            .next()
            .map(|size| size.0.y);
        assert!(first.is_some_and(|height| height < ARROW_SIZE.y / 2.0));

        for _ in 0..30 {
            tick_arrow_traps(
                &mut world,
                &navigation,
                &Textures::default(),
                &mut Vec::new(),
                0.016,
            );
        }
        let grown = world
            .query::<&Size>()
            .with::<&TrapArrow>()
            .iter()
            .next()
            .map(|size| size.0.y);
        assert_eq!(grown, Some(ARROW_SIZE.y));
    }
}
