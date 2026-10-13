use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::audio::AudioCommand;
use crate::components::{
    ArrowTrap, Color, Health, Layer, Lit, Player, Position, Size, Sprite, TrapArrow,
};
use crate::geometry;
use crate::navigation::Navigation;
use crate::systems::damage::hurt_player;

const ARROW_PERIOD: f32 = 2.4;
const ARROW_SPEED: f32 = 520.0;
const ARROW_SIZE: Vec2 = Vec2::new(20.0, 56.0);
const ARROW_HITBOX: Vec2 = Vec2::new(12.0, 40.0);
const ARROW_DAMAGE: i32 = 1;
const ARROW_REACH: f32 = 640.0;
const ARROW_RECHECK: f32 = 0.25;

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
        shots.push(position.0 + Vec2::new(0.0, size.0.y / 2.0 + ARROW_SIZE.y / 2.0));
    }
    for at in shots {
        world.spawn((
            TrapArrow,
            Position(at),
            Size(ARROW_SIZE),
            Color(Vec4::ONE),
            Sprite::whole(textures.get(tex::TRAP_ARROW)),
            Layer(2),
            Lit,
        ));
    }
    let flying: Vec<(Entity, Vec2)> = world
        .query::<(Entity, &Position)>()
        .with::<&TrapArrow>()
        .iter()
        .map(|(entity, position)| (entity, position.0))
        .collect();
    for (entity, at) in flying {
        let next = at + Vec2::new(0.0, ARROW_SPEED * dt);
        let tip = next + Vec2::new(0.0, ARROW_SIZE.y / 2.0 - ARROW_HITBOX.y / 2.0);
        if navigation.is_blocked(tip, ARROW_HITBOX) {
            let _ = world.despawn(entity);
            continue;
        }
        if geometry::overlaps(next, ARROW_HITBOX, player_at, player_size) {
            hurt_player(world, player, ARROW_DAMAGE, sounds);
            let _ = world.despawn(entity);
            continue;
        }
        if let Ok(mut position) = world.get::<&mut Position>(entity) {
            position.0 = next;
        }
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
        let (mut world, _) = world_with_player(Vec2::new(900.0, 900.0));
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
}
