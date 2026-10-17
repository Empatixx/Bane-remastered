use hecs::World;

use crate::artefacts::BERSERK_SPEED;
use crate::components::{
    Berserk, Cobweb, Dodge, FloorPatch, Health, PatchKind, Player, PlayerPerks, Position, SandSunk,
    Size, Velocity,
};
use crate::dungeon::{Surface, Surfaces};
use crate::geometry;
use crate::input::Input;

const PLAYER_SPEED: f32 = 411.0;
const COBWEB_SLOWDOWN: f32 = 0.5;
const PLAYER_DAMPING: f32 = 5.0;
const ICE_DAMPING: f32 = 1.1;

#[cfg(test)]
fn accelerate(velocity: glam::Vec2, direction: glam::Vec2, top_speed: f32, dt: f32) -> glam::Vec2 {
    accelerate_with(velocity, direction, top_speed, PLAYER_DAMPING, dt)
}

fn accelerate_with(
    velocity: glam::Vec2,
    direction: glam::Vec2,
    top_speed: f32,
    damping: f32,
    dt: f32,
) -> glam::Vec2 {
    let keep = (1.0 - damping * dt).max(0.0);
    let acceleration = top_speed * damping / keep.max(f32::EPSILON);
    (velocity + direction * acceleration * dt) * keep
}

const WATER_SLOWDOWN: f32 = 0.55;
const BOG_SLOWDOWN: f32 = 0.45;
const BLOOD_SLOWDOWN: f32 = 0.7;
/// Sand slows the player down a little at first and more and more the longer they stay in it,
/// up to the most after this many seconds; out of it they climb free several times as fast.
const SAND_SLOWDOWN: (f32, f32) = (0.01, 0.8);
const SINK_TIME: f32 = 3.0;
const CLIMB_OUT_RATE: f32 = 4.0;

/// How much of their speed the player loses after sinking for `sunk` seconds.
pub(crate) fn sand_slowdown(sunk: f32) -> f32 {
    let (least, most) = SAND_SLOWDOWN;
    least + (most - least) * (sunk / SINK_TIME).clamp(0.0, 1.0)
}

pub(crate) fn steer_player(world: &mut World, input: &Input, dt: f32) {
    steer_player_on(world, input, &Surfaces::default(), dt);
}

pub(crate) fn steer_player_on(world: &mut World, input: &Input, surfaces: &Surfaces, dt: f32) {
    let direction = input.direction();
    let webs: Vec<(glam::Vec2, glam::Vec2)> = world
        .query::<(&Position, &Size)>()
        .with::<&Cobweb>()
        .iter()
        .map(|(position, size)| (position.0, size.0))
        .collect();
    let patches: Vec<(glam::Vec2, glam::Vec2, PatchKind)> = world
        .query::<(&Position, &Size, &FloorPatch)>()
        .iter()
        .map(|(position, size, patch)| (position.0, size.0, patch.0))
        .collect();
    for (velocity, health, position, size, berserk, perks, dodge, frozen, sinking) in world
        .query_mut::<(
            &mut Velocity,
            &Health,
            &Position,
            &Size,
            Option<&Berserk>,
            Option<&PlayerPerks>,
            Option<&Dodge>,
            Option<&crate::components::Frozen>,
            Option<&mut SandSunk>,
        )>()
        .with::<&Player>()
    {
        let on = |wanted: PatchKind| {
            patches.iter().any(|(at, patch_size, kind)| {
                *kind == wanted && geometry::overlaps(position.0, size.0, *at, *patch_size)
            })
        };
        let stuck = on(PatchKind::Web)
            || webs
                .iter()
                .any(|(web, web_size)| geometry::overlaps(position.0, size.0, *web, *web_size));
        let ground = surfaces.at_point(position.0);
        let damping = if on(PatchKind::Ice) || ground == Some(Surface::Ice) {
            ICE_DAMPING
        } else {
            PLAYER_DAMPING
        };
        let boost = if berserk.is_some() {
            BERSERK_SPEED
        } else {
            1.0
        } * perks.map_or(1.0, |perks| perks.0.speed);
        let in_sand = ground == Some(Surface::Sand);
        let sunk = sinking.map_or(0.0, |sinking| {
            sinking.0 = if in_sand {
                (sinking.0 + dt).min(SINK_TIME)
            } else {
                (sinking.0 - CLIMB_OUT_RATE * dt).max(0.0)
            };
            sinking.0
        });
        let speed = if in_sand {
            PLAYER_SPEED * (1.0 - sand_slowdown(sunk))
        } else if ground == Some(Surface::Water) {
            PLAYER_SPEED * WATER_SLOWDOWN
        } else if ground == Some(Surface::Bog) {
            PLAYER_SPEED * BOG_SLOWDOWN
        } else if ground == Some(Surface::Blood) {
            PLAYER_SPEED * BLOOD_SLOWDOWN
        } else if stuck {
            PLAYER_SPEED * COBWEB_SLOWDOWN
        } else {
            PLAYER_SPEED
        } * boost;
        velocity.0 = if let (Some(dodge), true) = (dodge, health.current > 0) {
            dodge.direction * super::DODGE_SPEED
        } else if health.current > 0 {
            let direction = if frozen.is_some() {
                glam::Vec2::ZERO
            } else {
                direction
            };
            accelerate_with(velocity.0, direction, speed, damping, dt)
        } else {
            glam::Vec2::ZERO
        };
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use super::*;

    const STEP: f32 = 1.0 / 60.0;

    #[test]
    fn holding_a_direction_accelerates_towards_the_top_speed() {
        let mut velocity = Vec2::ZERO;
        let first = accelerate(velocity, Vec2::X, 411.0, STEP);
        assert!(first.x > 0.0 && first.x < 411.0 * 0.5, "{first}");
        for _ in 0..300 {
            velocity = accelerate(velocity, Vec2::X, 411.0, STEP);
        }
        assert!((velocity.x - 411.0).abs() < 1.0, "{velocity}");
    }

    #[test]
    fn letting_go_slides_to_a_stop_instead_of_halting_at_once() {
        let mut velocity = Vec2::new(411.0, 0.0);
        velocity = accelerate(velocity, Vec2::ZERO, 411.0, STEP);
        assert!(velocity.x > 300.0 && velocity.x < 411.0, "{velocity}");
        for _ in 0..120 {
            velocity = accelerate(velocity, Vec2::ZERO, 411.0, STEP);
        }
        assert!(velocity.x < 1.0, "{velocity}");
    }

    #[test]
    fn reversing_takes_longer_than_stopping() {
        let mut velocity = Vec2::new(411.0, 0.0);
        velocity = accelerate(velocity, -Vec2::X, 411.0, STEP);
        assert!(velocity.x > 0.0, "{velocity}");
    }

    #[test]
    fn on_ice_the_player_keeps_sliding_much_longer() {
        let stop = |damping: f32| {
            let mut velocity = Vec2::new(411.0, 0.0);
            let mut steps = 0;
            while velocity.x > 5.0 && steps < 1000 {
                velocity = accelerate_with(velocity, Vec2::ZERO, 411.0, damping, STEP);
                steps += 1;
            }
            steps
        };
        assert!(stop(ICE_DAMPING) > 3 * stop(PLAYER_DAMPING));
    }

    fn walker(at: Vec2) -> (hecs::World, hecs::Entity) {
        let mut world = hecs::World::new();
        let player = world.spawn((
            Player,
            Velocity(Vec2::ZERO),
            Health { current: 5, max: 5 },
            Position(at),
            Size(Vec2::splat(40.0)),
        ));
        (world, player)
    }

    fn run_right(world: &mut hecs::World, player: hecs::Entity) -> f32 {
        let mut input = Input::default();
        input.press(winit::keyboard::KeyCode::KeyD);
        for _ in 0..120 {
            steer_player(world, &input, STEP);
        }
        world.get::<&Velocity>(player).map_or(0.0, |v| v.0.x)
    }

    #[test]
    fn a_web_patch_halves_the_top_speed() {
        let (mut free, player) = walker(Vec2::new(100.0, 100.0));
        let (mut stuck, trapped) = walker(Vec2::new(100.0, 100.0));
        stuck.spawn((
            Position(Vec2::new(100.0, 100.0)),
            Size(Vec2::splat(96.0)),
            FloorPatch(PatchKind::Web),
        ));
        let normal = run_right(&mut free, player);
        let slowed = run_right(&mut stuck, trapped);
        assert!(slowed < normal * 0.6, "{slowed} vs {normal}");
    }

    fn run_on(surface: Option<crate::dungeon::Surface>) -> f32 {
        let (mut world, player) = walker(Vec2::new(100.0, 100.0));
        let mut surfaces = Surfaces::default();
        if let Some(surface) = surface {
            surfaces.insert((1, 1), surface);
        }
        let mut input = Input::default();
        input.press(winit::keyboard::KeyCode::KeyD);
        let mut slowest: f32 = f32::MAX;
        for _ in 0..30 {
            world
                .get::<&mut Position>(player)
                .map(|mut p| p.0 = Vec2::new(96.0, 96.0))
                .ok();
            steer_player_on(&mut world, &input, &surfaces, STEP);
            slowest = slowest.min(world.get::<&Velocity>(player).map_or(0.0, |v| v.0.x));
        }
        world
            .get::<&Velocity>(player)
            .map_or(0.0, |v| v.0.x)
            .max(slowest)
    }

    #[test]
    fn water_slows_the_player_and_ice_does_not_change_the_top_speed() {
        let dry = run_on(None);
        let wet = run_on(Some(crate::dungeon::Surface::Water));
        let icy = run_on(Some(crate::dungeon::Surface::Ice));
        assert!(wet < dry * 0.8, "{wet} vs {dry}");
        assert!(icy < dry, "{icy} vs {dry}");
    }

    #[test]
    fn bog_slows_the_player_like_a_heavy_mud() {
        let dry = run_on(None);
        let mud = run_on(Some(crate::dungeon::Surface::Bog));
        assert!(mud < dry * 0.7, "{mud} vs {dry}");
    }

    #[test]
    fn sand_slows_from_a_hundredth_to_four_fifths_the_longer_one_stays_in_it() {
        assert!((sand_slowdown(0.0) - 0.01).abs() < 1e-6);
        assert!((sand_slowdown(SINK_TIME) - 0.8).abs() < 1e-6);
        assert!((sand_slowdown(100.0) - 0.8).abs() < 1e-6);
        assert!(sand_slowdown(SINK_TIME / 2.0) > 0.3 && sand_slowdown(SINK_TIME / 2.0) < 0.5);
    }

    #[test]
    fn the_player_sinks_into_sand_and_climbs_out_of_it() {
        let (mut world, player) = walker(Vec2::new(96.0, 96.0));
        let _ = world.insert_one(player, SandSunk::default());
        let mut sand = Surfaces::default();
        sand.insert((1, 1), crate::dungeon::Surface::Sand);
        let input = Input::default();
        let sunk = |world: &World| world.get::<&SandSunk>(player).map_or(0.0, |sunk| sunk.0);
        for _ in 0..90 {
            steer_player_on(&mut world, &input, &sand, STEP);
        }
        let deep = sunk(&world);
        assert!(deep > 1.4, "{deep}");
        for _ in 0..30 {
            steer_player_on(&mut world, &input, &Surfaces::default(), STEP);
        }
        assert_eq!(sunk(&world), 0.0);
    }
}
