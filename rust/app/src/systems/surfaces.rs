use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::audio::AudioCommand;
use crate::components::{
    Animation, Clip, Color, Dodge, Expiring, Health, Layer, Liquid, Player, Position, RippleTimer,
    Size, Sprite, Velocity,
};
use crate::dungeon::{Surface, Surfaces};
use crate::tilemap::TILE_SIZE;
use crate::systems::damage::hurt_player_through_armor;

const LAVA_DAMAGE: i32 = 2;
const RIPPLE_EVERY: f32 = 0.2;
const RIPPLE_MIN_SPEED: f32 = 40.0;
const RIPPLE_SIZE: Vec2 = Vec2::new(96.0, 96.0);
const RIPPLE_FRAMES: usize = 6;
/// Rings in a bog spread slower and come less often than on water, in the colour of the mud.
const BOG_RIPPLE_EVERY: f32 = 0.32;
const BOG_RIPPLE: Vec4 = Vec4::new(0.55, 0.45, 0.3, 0.75);
/// Wading through sand stirs it in pale rings.
const SAND_RIPPLE: Vec4 = Vec4::new(1.0, 0.82, 0.62, 0.6);
/// Wading through blood leaves dark red rings.
const BLOOD_RIPPLE: Vec4 = Vec4::new(0.75, 0.12, 0.1, 0.7);

pub(crate) fn burn_on_lava(
    world: &mut World,
    surfaces: &Surfaces,
    sounds: &mut Vec<AudioCommand>,
    fx: &mut super::Fx,
) -> bool {
    let Some((player, at)) = world
        .query::<(Entity, &Position, &Health)>()
        .with::<&Player>()
        .without::<&Dodge>()
        .iter()
        .filter(|(_, _, health)| health.current > 0)
        .map(|(entity, position, _)| (entity, position.0))
        .next()
    else {
        return false;
    };
    surfaces.at_point(at) == Some(Surface::Lava)
        && hurt_player_through_armor(world, player, LAVA_DAMAGE, sounds, fx)
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
    let (every, tint) = match surfaces.at_point(feet) {
        Some(Surface::Water) => (RIPPLE_EVERY, Vec4::ONE),
        Some(Surface::Bog) => (BOG_RIPPLE_EVERY, BOG_RIPPLE),
        Some(Surface::Sand) => (BOG_RIPPLE_EVERY, SAND_RIPPLE),
        Some(Surface::Blood) => (BOG_RIPPLE_EVERY, BLOOD_RIPPLE),
        _ => return,
    };
    if !ready || !moving {
        return;
    }
    if let Ok(mut timer) = world.get::<&mut RippleTimer>(player) {
        timer.0 = every;
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
        Color(tint),
        Sprite::cell(textures.get(tex::RIPPLE), 0, 0, RIPPLE_FRAMES, 1),
        Animation::new(RIPPLE_FRAMES, 1, clip),
        Layer(-1),
    ));
}

/// How often, while walking, the surface underfoot is kicked up.
const SPRAY_EVERY: f32 = 0.09;
const SPRAY_MIN_SPEED: f32 = 40.0;
/// Kicked-up bits fly round the feet, over the floor and under the overheads.
const SPRAY_LAYER: i32 = 2;

/// What one surface throws up when walked through: its two colours, how many bits at a time,
/// how hard they're thrown up and out, how fast they fall back, how big they are and how long
/// they last; whether they glow (and so ignore the dark) and fade as they go rather than land.
#[derive(Debug, Clone, Copy)]
struct Spray {
    colours: [Vec4; 2],
    count: usize,
    rise: (f32, f32),
    spread: f32,
    gravity: f32,
    size: f32,
    life: f32,
    glows: bool,
}

fn spray(surface: Surface) -> Spray {
    match surface {
        // Drops of water and pale foam.
        Surface::Water => Spray {
            colours: [Vec4::new(0.7, 0.85, 0.62, 1.0), Vec4::new(0.92, 0.98, 0.86, 1.0)],
            count: 3,
            rise: (180.0, 260.0),
            spread: 80.0,
            gravity: 900.0,
            size: 8.0,
            life: 0.7,
            glows: false,
        },
        // Heavy blobs of mud.
        Surface::Bog => Spray {
            colours: [Vec4::new(0.5, 0.4, 0.22, 1.0), Vec4::new(0.66, 0.56, 0.32, 1.0)],
            count: 3,
            rise: (120.0, 190.0),
            spread: 55.0,
            gravity: 900.0,
            size: 10.0,
            life: 0.7,
            glows: false,
        },
        // Black cinders and white-hot sparks, floating up and dying out.
        Surface::Lava => Spray {
            colours: [Vec4::new(0.22, 0.08, 0.06, 1.0), Vec4::new(1.0, 1.0, 0.82, 1.0)],
            count: 3,
            rise: (160.0, 260.0),
            spread: 60.0,
            gravity: 150.0,
            size: 7.0,
            life: 0.9,
            glows: true,
        },
        // Chips of ice and glittering frost.
        Surface::Ice => Spray {
            colours: [Vec4::new(0.85, 0.95, 1.0, 1.0), Vec4::new(1.0, 1.0, 1.0, 1.0)],
            count: 2,
            rise: (110.0, 180.0),
            spread: 90.0,
            gravity: 700.0,
            size: 6.0,
            life: 0.55,
            glows: true,
        },
        // A spray of grains.
        Surface::Sand => Spray {
            colours: [Vec4::new(0.92, 0.8, 0.54, 1.0), Vec4::new(0.66, 0.52, 0.3, 1.0)],
            count: 4,
            rise: (110.0, 190.0),
            spread: 70.0,
            gravity: 800.0,
            size: 6.0,
            life: 0.6,
            glows: false,
        },
        // Drops of blood, the bright red of it fresh.
        Surface::Blood => Spray {
            colours: [Vec4::new(0.78, 0.1, 0.12, 1.0), Vec4::new(0.95, 0.24, 0.22, 1.0)],
            count: 3,
            rise: (150.0, 230.0),
            spread: 65.0,
            gravity: 950.0,
            size: 8.0,
            life: 0.65,
            glows: true,
        },
    }
}

/// When the next spray is due, and a little state for scattering the bits.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SprayTimer {
    timer: f32,
    seed: u32,
}

/// A bit kicked up from the surface underfoot, flying in an arc over the spot below it.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Kicked {
    ground: Vec2,
    drift: Vec2,
    height: f32,
    rise: f32,
    gravity: f32,
    life: f32,
    lasts: f32,
    alpha: f32,
    glows: bool,
}

/// Walking through any liquid kicks bits of it up round the feet: drops of water, blobs of mud,
/// sparks off the lava, chips of ice, grains of sand, drops of blood.
pub(crate) fn kick_up_surfaces(world: &mut World, surfaces: &Surfaces, dt: f32) {
    let walker = world
        .query::<(Entity, &Position, &Size, &Velocity, &Health)>()
        .with::<&Player>()
        .iter()
        .filter(|(_, _, _, _, health)| health.current > 0)
        .map(|(entity, position, size, velocity, _)| {
            (entity, position.0 + Vec2::new(0.0, size.0.y * 0.4), velocity.0)
        })
        .next();
    if let Some((player, feet, velocity)) = walker {
        if world.get::<&SprayTimer>(player).is_err() {
            let _ = world.insert_one(player, SprayTimer { timer: 0.0, seed: 0x2545_F491 });
        }
        let mut thrown = Vec::new();
        if let Ok(mut timer) = world.get::<&mut SprayTimer>(player) {
            timer.timer -= dt;
            let surface = surfaces.at_point(feet);
            if let (Some(surface), true) = (surface, timer.timer <= 0.0 && velocity.length() > SPRAY_MIN_SPEED) {
                timer.timer = SPRAY_EVERY;
                let kind = spray(surface);
                let mut roll = || {
                    timer.seed ^= timer.seed << 13;
                    timer.seed ^= timer.seed >> 17;
                    timer.seed ^= timer.seed << 5;
                    (timer.seed % 10_000) as f32 / 10_000.0
                };
                for _ in 0..kind.count {
                    let out = Vec2::new(roll() * 2.0 - 1.0, (roll() * 2.0 - 1.0) * 0.5) * kind.spread;
                    let rise = kind.rise.0 + (kind.rise.1 - kind.rise.0) * roll();
                    let colour = kind.colours[usize::from(roll() > 0.5)];
                    let start = feet + Vec2::new((roll() - 0.5) * 16.0, (roll() - 0.5) * 8.0);
                    thrown.push((kind, start, out - velocity * 0.15, rise, colour));
                }
            }
        }
        for (kind, start, drift, rise, colour) in thrown {
            let bit = world.spawn((
                Kicked {
                    ground: start,
                    drift,
                    height: 0.0,
                    rise,
                    gravity: kind.gravity,
                    life: kind.life,
                    lasts: kind.life,
                    alpha: colour.w,
                    glows: kind.glows,
                },
                Position(start),
                Size(Vec2::splat(kind.size)),
                Color(colour),
                Sprite::whole(crate::render::TextureId::WHITE),
                Layer(SPRAY_LAYER),
            ));
            if !kind.glows {
                let _ = world.insert_one(bit, crate::components::Lit);
            }
        }
    }
    let mut gone = Vec::new();
    for (entity, bit, position, color) in world.query_mut::<(Entity, &mut Kicked, &mut Position, &mut Color)>() {
        bit.life -= dt;
        bit.ground += bit.drift * dt;
        bit.rise -= bit.gravity * dt;
        bit.height += bit.rise * dt;
        position.0 = bit.ground - Vec2::new(0.0, bit.height.max(0.0));
        if bit.glows {
            color.0.w = bit.alpha * (bit.life / bit.lasts).max(0.0);
        }
        if bit.life <= 0.0 || bit.height < 0.0 {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

/// What a surface gives off on its own, wherever it lies in view: the colours of it, how
/// often a tile gives one off (a second), how it drifts, how big it is and how long it lasts,
/// whether it glows in the dark, and whether it twinkles (fades in and out where it is) rather
/// than drifting off and fading.
#[derive(Debug, Clone, Copy)]
struct Stir {
    colours: [Vec4; 2],
    rate: f32,
    drift: (Vec2, Vec2),
    size: f32,
    life: f32,
    glows: bool,
    twinkles: bool,
}

fn stir(surface: Surface) -> Stir {
    match surface {
        // Black cinders and white-hot sparks floating up off the lava and dying out: the lava's
        // own oranges would not show against it.
        Surface::Lava => Stir {
            colours: [Vec4::new(0.24, 0.08, 0.05, 1.0), Vec4::new(1.0, 1.0, 0.86, 1.0)],
            rate: 0.5,
            drift: (Vec2::new(-12.0, -55.0), Vec2::new(12.0, -30.0)),
            size: 6.0,
            life: 1.4,
            glows: true,
            twinkles: false,
        },
        // Glints of light on the water.
        Surface::Water => Stir {
            colours: [Vec4::new(0.85, 0.97, 0.9, 0.9), Vec4::new(1.0, 1.0, 1.0, 1.0)],
            rate: 0.35,
            drift: (Vec2::new(-3.0, 0.0), Vec2::new(3.0, 0.0)),
            size: 5.0,
            life: 0.8,
            glows: true,
            twinkles: true,
        },
        // Bubbles rising through the mud and bursting.
        Surface::Bog => Stir {
            colours: [Vec4::new(0.3, 0.24, 0.12, 1.0), Vec4::new(0.8, 0.72, 0.46, 1.0)],
            rate: 0.3,
            drift: (Vec2::new(-2.0, -8.0), Vec2::new(2.0, -4.0)),
            size: 6.0,
            life: 0.9,
            glows: false,
            twinkles: false,
        },
        // Glitter on the ice.
        Surface::Ice => Stir {
            colours: [Vec4::new(0.85, 0.95, 1.0, 0.9), Vec4::new(1.0, 1.0, 1.0, 1.0)],
            rate: 0.3,
            drift: (Vec2::ZERO, Vec2::ZERO),
            size: 4.0,
            life: 0.6,
            glows: true,
            twinkles: true,
        },
        // Grains of sand blown along the top of it.
        Surface::Sand => Stir {
            colours: [Vec4::new(1.0, 0.92, 0.7, 1.0), Vec4::new(0.55, 0.42, 0.24, 1.0)],
            rate: 0.35,
            drift: (Vec2::new(18.0, -4.0), Vec2::new(34.0, 2.0)),
            size: 4.0,
            life: 1.2,
            glows: false,
            twinkles: false,
        },
        // A bubble now and then in the blood.
        Surface::Blood => Stir {
            colours: [Vec4::new(0.6, 0.06, 0.08, 1.0), Vec4::new(0.85, 0.2, 0.2, 1.0)],
            rate: 0.12,
            drift: (Vec2::new(-1.0, -6.0), Vec2::new(1.0, -3.0)),
            size: 4.0,
            life: 0.8,
            glows: false,
            twinkles: false,
        },
    }
}

/// Bits rise off the surfaces over the floor and under whoever walks there.
const STIR_LAYER: i32 = 0;

/// A bit a surface gave off: its drift, how long it has left of how long, its full alpha and
/// whether it twinkles.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Stirred {
    drift: Vec2,
    life: f32,
    lasts: f32,
    alpha: f32,
    twinkles: bool,
}

/// The scatter for what the surfaces give off. Not an entity: the scene keeps it with the
/// floor's other resources, and a fresh floor starts it over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StirSeed(pub(crate) u32);

impl Default for StirSeed {
    fn default() -> Self {
        Self(0x9E37_79B9)
    }
}

/// The surfaces in view, alive on their own: embers off the lava, glints on the water, bubbles
/// in the bog and the blood, glitter on the ice, grains blowing over the sand. `view` is the
/// top left of the camera and `size` what it shows.
pub(crate) fn stir_surfaces(world: &mut World, scatter: &mut StirSeed, surfaces: &Surfaces, view: Vec2, size: Vec2, dt: f32) {
    let mut seed = scatter.0;
    let mut roll = || {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        (seed % 10_000) as f32 / 10_000.0
    };
    let first = ((view / TILE_SIZE).floor() - 1.0).max(Vec2::ZERO);
    let last = ((view + size) / TILE_SIZE).ceil() + 1.0;
    let mut given = Vec::new();
    for (tile, surface) in surfaces.iter() {
        let at = Vec2::new(tile.0 as f32, tile.1 as f32);
        if at.x < first.x || at.y < first.y || at.x > last.x || at.y > last.y || surfaces.at_point((at + 0.5) * TILE_SIZE).is_none() {
            continue;
        }
        let kind = stir(surface);
        if roll() >= kind.rate * dt {
            continue;
        }
        let spot = (at + Vec2::new(0.15 + roll() * 0.7, 0.15 + roll() * 0.7)) * TILE_SIZE;
        let drift = kind.drift.0 + (kind.drift.1 - kind.drift.0) * Vec2::new(roll(), roll());
        let colour = kind.colours[usize::from(roll() > 0.5)];
        let life = kind.life * (0.7 + roll() * 0.6);
        given.push((kind, spot, drift, colour, life));
    }
    for (kind, spot, drift, colour, life) in given {
        let bit = world.spawn((
            Stirred { drift, life, lasts: life, alpha: colour.w, twinkles: kind.twinkles },
            Position(spot),
            Size(Vec2::splat(kind.size)),
            Color(Vec4::new(colour.x, colour.y, colour.z, 0.0)),
            Sprite::whole(crate::render::TextureId::WHITE),
            Layer(STIR_LAYER),
        ));
        if !kind.glows {
            let _ = world.insert_one(bit, crate::components::Lit);
        }
    }
    let mut gone = Vec::new();
    for (entity, bit, position, color) in world.query_mut::<(Entity, &mut Stirred, &mut Position, &mut Color)>() {
        bit.life -= dt;
        position.0 += bit.drift * dt;
        let through = 1.0 - (bit.life / bit.lasts).clamp(0.0, 1.0);
        let shown = if bit.twinkles { (through * std::f32::consts::PI).sin() } else { (1.0 - through).min(through * 6.0).min(1.0) };
        color.0.w = bit.alpha * shown;
        if bit.life <= 0.0 {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
    scatter.0 = seed;
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
        burn_on_lava(&mut world, &surfaces, &mut Vec::new(), &mut crate::systems::Fx::default());
        assert_eq!(health(&world, player), 10);

        let (mut world, player) = standing_at((4, 4));
        burn_on_lava(&mut world, &surfaces, &mut Vec::new(), &mut crate::systems::Fx::default());
        assert_eq!(health(&world, player), 8);
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
        burn_on_lava(&mut world, &surfaces, &mut Vec::new(), &mut crate::systems::Fx::default());
        assert_eq!(health(&world, player), 8);
        assert_eq!(world.get::<&Armor>(player).map(|a| a.current).ok(), Some(3));
    }

    #[test]
    fn a_player_standing_on_a_real_lava_tile_of_a_generated_floor_is_burned() {
        use crate::dungeon::{Dungeon, Theme};
        use crate::rng::Rng;

        let mut found = 0;
        for seed in 0..10 {
            let dungeon =
                Dungeon::generate_themed(&mut Rng::new(seed), Theme::Lava).expect("a dungeon");
            // A tile of open lava, not one under a bridge.
            let center = |(c, r): (usize, usize)| {
                (Vec2::new(c as f32, r as f32) + 0.5) * crate::tilemap::TILE_SIZE
            };
            let Some((tile, _)) = dungeon
                .surfaces
                .iter()
                .find(|&(tile, _)| dungeon.surfaces.at_point(center(tile)).is_some())
            else {
                continue;
            };
            let (mut world, player) = standing_at(tile);
            if let Ok(mut armor) = world.get::<&mut Armor>(player) {
                armor.current = 5;
            }
            burn_on_lava(
                &mut world,
                &dungeon.surfaces,
                &mut Vec::new(),
                &mut crate::systems::Fx::default(),
            );
            assert_eq!(health(&world, player), 8, "seed {seed} tile {tile:?}");
            found += 1;
        }
        assert!(found > 0);
    }

    #[test]
    fn a_bridge_over_lava_keeps_the_player_from_burning() {
        let mut surfaces = Surfaces::default();
        surfaces.insert((4, 4), Surface::Lava);
        surfaces.bridge((4, 4));
        let (mut world, player) = standing_at((4, 4));
        assert!(!burn_on_lava(
            &mut world,
            &surfaces,
            &mut Vec::new(),
            &mut crate::systems::Fx::default(),
        ));
        assert_eq!(health(&world, player), 10);
    }

    #[test]
    fn other_surfaces_do_not_burn() {
        let mut surfaces = Surfaces::default();
        surfaces.insert((4, 4), Surface::Water);
        let (mut world, player) = standing_at((4, 4));
        burn_on_lava(&mut world, &surfaces, &mut Vec::new(), &mut crate::systems::Fx::default());
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
        burn_on_lava(&mut world, &surfaces, &mut Vec::new(), &mut crate::systems::Fx::default());
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

    #[test]
    fn walking_through_any_liquid_kicks_bits_of_it_up_that_fall_back_down() {
        for surface in [Surface::Water, Surface::Bog, Surface::Lava, Surface::Ice, Surface::Sand, Surface::Blood] {
            let (mut world, player) = standing_at((4, 4));
            let mut surfaces = Surfaces::default();
            for column in 2..7 {
                for row in 2..7 {
                    surfaces.insert((column, row), surface);
                }
            }
            if let Ok(mut velocity) = world.get::<&mut Velocity>(player) {
                velocity.0 = Vec2::new(200.0, 0.0);
            }
            let mut most = 0;
            for _ in 0..30 {
                kick_up_surfaces(&mut world, &surfaces, 1.0 / 60.0);
                most = most.max(world.query::<&Kicked>().iter().count());
            }
            assert!(most > 0, "{surface:?}");
            if let Ok(mut velocity) = world.get::<&mut Velocity>(player) {
                velocity.0 = Vec2::ZERO;
            }
            for _ in 0..120 {
                kick_up_surfaces(&mut world, &surfaces, 1.0 / 60.0);
            }
            assert_eq!(world.query::<&Kicked>().iter().count(), 0, "{surface:?}");
        }
        let (mut world, player) = standing_at((4, 4));
        if let Ok(mut velocity) = world.get::<&mut Velocity>(player) {
            velocity.0 = Vec2::new(200.0, 0.0);
        }
        for _ in 0..30 {
            kick_up_surfaces(&mut world, &Surfaces::default(), 1.0 / 60.0);
        }
        assert_eq!(world.query::<&Kicked>().iter().count(), 0);
    }
}
