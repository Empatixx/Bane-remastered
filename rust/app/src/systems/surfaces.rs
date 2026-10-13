use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::audio::AudioCommand;
use crate::components::{
    Animation, Clip, Color, Dodge, Expiring, Health, Layer, Liquid, Player, Position, RippleTimer,
    Size, Sprite, Velocity,
};
use crate::dungeon::{Surface, Surfaces};
use crate::systems::damage::hurt_player_through_armor;

const LAVA_DAMAGE: i32 = 1;
const RIPPLE_EVERY: f32 = 0.2;
const RIPPLE_MIN_SPEED: f32 = 40.0;
const RIPPLE_SIZE: Vec2 = Vec2::new(96.0, 96.0);
const RIPPLE_FRAMES: usize = 6;

pub(crate) fn burn_on_lava(world: &mut World, surfaces: &Surfaces, sounds: &mut Vec<AudioCommand>) {
    let Some((player, at)) = world
        .query::<(Entity, &Position, &Health)>()
        .with::<&Player>()
        .without::<&Dodge>()
        .iter()
        .filter(|(_, _, health)| health.current > 0)
        .map(|(entity, position, _)| (entity, position.0))
        .next()
    else {
        return;
    };
    if surfaces.at_point(at) == Some(Surface::Lava) {
        hurt_player_through_armor(world, player, LAVA_DAMAGE, sounds);
    }
}

pub(crate) fn sync_liquids(world: &mut World, clock: f32) {
    for liquid in world.query_mut::<&mut Liquid>() {
        liquid.time = clock;
    }
}

pub(crate) fn ripple_on_water(
    world: &mut World,
    surfaces: &Surfaces,
    textures: &Textures,
    dt: f32,
) {
    let Some((player, feet, moving)) = world
        .query::<(Entity, &Position, &Size, &Velocity, &Health)>()
        .with::<&Player>()
        .iter()
        .filter(|(_, _, _, _, health)| health.current > 0)
        .map(|(entity, position, size, velocity, _)| {
            (
                entity,
                position.0 + Vec2::new(0.0, size.0.y * 0.4),
                velocity.0.length() > RIPPLE_MIN_SPEED,
            )
        })
        .next()
    else {
        return;
    };
    if world.get::<&RippleTimer>(player).is_err() {
        let _ = world.insert_one(player, RippleTimer(0.0));
    }
    let ready = world
        .get::<&mut RippleTimer>(player)
        .is_ok_and(|mut timer| {
            timer.0 -= dt;
            timer.0 <= 0.0
        });
    if !ready || !moving || surfaces.at_point(feet) != Some(Surface::Water) {
        return;
    }
    if let Ok(mut timer) = world.get::<&mut RippleTimer>(player) {
        timer.0 = RIPPLE_EVERY;
    }
    let clip = Clip {
        row: 0,
        frames: RIPPLE_FRAMES,
        frame_time: 0.1,
        looping: false,
        flipped: false,
    };
    world.spawn((
        Expiring,
        Position(feet),
        Size(RIPPLE_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(tex::RIPPLE), 0, 0, RIPPLE_FRAMES, 1),
        Animation::new(RIPPLE_FRAMES, 1, clip),
        Layer(-1),
    ));
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::assets::Textures;
    use crate::components::{Armor, Invulnerable};
    use crate::physics::Physics;
    use crate::prefabs;
    use crate::tilemap::TILE_SIZE;

    use super::*;

    fn standing_at(tile: (usize, usize)) -> (World, Entity) {
        let mut world = World::new();
        let at = Vec2::new(tile.0 as f32 + 0.5, tile.1 as f32 + 0.5) * TILE_SIZE;
        let player = prefabs::player(&mut world, &mut Physics::new(), at, &Textures::default());
        if let Ok(mut armor) = world.get::<&mut Armor>(player) {
            armor.current = 0;
        }
        (world, player)
    }

    fn health(world: &World, player: Entity) -> i32 {
        world
            .get::<&Health>(player)
            .map_or(0, |health| health.current)
    }

    #[test]
    fn standing_on_lava_hurts_and_standing_beside_it_does_not() {
        let mut surfaces = Surfaces::default();
        surfaces.insert((4, 4), Surface::Lava);

        let (mut world, player) = standing_at((5, 4));
        burn_on_lava(&mut world, &surfaces, &mut Vec::new());
        assert_eq!(health(&world, player), 10);

        let (mut world, player) = standing_at((4, 4));
        burn_on_lava(&mut world, &surfaces, &mut Vec::new());
        assert_eq!(health(&world, player), 9);
        assert!(world.get::<&Invulnerable>(player).is_ok());
    }

    #[test]
    fn lava_burns_through_the_armor_straight_into_health() {
        let mut surfaces = Surfaces::default();
        surfaces.insert((4, 4), Surface::Lava);
        let (mut world, player) = standing_at((4, 4));
        if let Ok(mut armor) = world.get::<&mut Armor>(player) {
            armor.current = 3;
        }
        burn_on_lava(&mut world, &surfaces, &mut Vec::new());
        assert_eq!(health(&world, player), 9);
        assert_eq!(world.get::<&Armor>(player).map(|a| a.current).ok(), Some(3));
    }

    #[test]
    fn other_surfaces_do_not_burn() {
        let mut surfaces = Surfaces::default();
        surfaces.insert((4, 4), Surface::Water);
        let (mut world, player) = standing_at((4, 4));
        burn_on_lava(&mut world, &surfaces, &mut Vec::new());
        assert_eq!(health(&world, player), 10);
    }

    #[test]
    fn a_dodging_player_flies_over_the_lava() {
        let mut surfaces = Surfaces::default();
        surfaces.insert((4, 4), Surface::Lava);
        let (mut world, player) = standing_at((4, 4));
        let _ = world.insert_one(
            player,
            Dodge {
                direction: Vec2::X,
                remaining: 0.3,
                trail: 0.0,
            },
        );
        burn_on_lava(&mut world, &surfaces, &mut Vec::new());
        assert_eq!(health(&world, player), 10);
    }

    fn ripples(world: &World) -> usize {
        world.query::<&Expiring>().iter().count()
    }

    fn run(surfaces: &Surfaces, tile: (usize, usize), speed: f32, steps: usize) -> usize {
        let (mut world, player) = standing_at(tile);
        let _ = world.insert_one(player, Velocity(Vec2::new(speed, 0.0)));
        for _ in 0..steps {
            ripple_on_water(&mut world, surfaces, &Textures::default(), 1.0 / 60.0);
        }
        ripples(&world)
    }

    #[test]
    fn walking_through_water_makes_ripples_but_dry_floor_and_standing_still_do_not() {
        let mut surfaces = Surfaces::default();
        surfaces.insert((4, 4), Surface::Water);
        assert!(run(&surfaces, (4, 4), 300.0, 30) >= 1);
        assert_eq!(run(&surfaces, (5, 4), 300.0, 30), 0);
        assert_eq!(run(&surfaces, (4, 4), 0.0, 30), 0);
    }

    #[test]
    fn ripples_come_at_a_steady_pace_not_every_frame() {
        let mut surfaces = Surfaces::default();
        surfaces.insert((4, 4), Surface::Water);
        let count = run(&surfaces, (4, 4), 300.0, 60);
        assert!((3..=6).contains(&count), "{count}");
    }

    #[test]
    fn every_liquid_tile_follows_the_same_clock() {
        let mut world = World::new();
        let a = world.spawn((Liquid {
            kind: 1.0,
            time: 0.0,
        },));
        let b = world.spawn((Liquid {
            kind: 3.0,
            time: 5.0,
        },));
        sync_liquids(&mut world, 12.5);
        for entity in [a, b] {
            assert_eq!(
                world.get::<&Liquid>(entity).map(|l| l.time).ok(),
                Some(12.5)
            );
        }
    }
}
