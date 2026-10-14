use glam::{Vec2, Vec4};
use hecs::World;

use crate::assets::{Textures, tex};
use crate::components::{Bullet, Color, Expiring, Lit, Mote, Position, Trailing, Velocity};
use crate::guns::Trail;
use crate::render::TextureId;
use crate::systems::dust::{DustSpec, spawn_dust};

const MAX_MOTES: usize = 120;
const PEAK: f32 = 0.9;
const DRAG_BEHIND: f32 = 0.06;
const ARC_MOTES: usize = 4;
const ARC_SCATTER: f32 = 60.0;
const ARC_MOTE_SIZE: f32 = 12.0;
const ARC_MOTE_LIFE: f32 = 0.3;

/// The mote sheets, looked up once a frame instead of per mote.
#[derive(Debug, Clone, Copy)]
pub(crate) struct TrailTextures {
    volt: TextureId,
    laser: TextureId,
}

impl TrailTextures {
    pub(crate) fn load(textures: &Textures) -> Self {
        Self {
            volt: textures.get(tex::ROUND_MOTE_VOLT),
            laser: textures.get(tex::ROUND_MOTE_LASER),
        }
    }

    #[cfg(test)]
    pub(crate) fn white() -> Self {
        Self {
            volt: TextureId::WHITE,
            laser: TextureId::WHITE,
        }
    }

    fn get(&self, trail: Trail) -> TextureId {
        if trail == Trail::VOLT { self.volt } else { self.laser }
    }
}

fn jitter(seed: f32, salt: f32) -> f32 {
    ((seed + salt) * 12.9898).sin().abs()
}

fn shed(world: &mut World, texture: TextureId, trail: Trail, at: Vec2, velocity: Vec2, seed: f32) {
    let sideways = velocity.perp().normalize_or_zero() * (jitter(seed, 1.0) - 0.5) * 2.0;
    let mote = spawn_dust(
        world,
        texture,
        DustSpec {
            at,
            velocity: sideways * trail.spread - velocity * DRAG_BEHIND,
            size: trail.size * (0.7 + 0.6 * jitter(seed, 2.0)),
            life: trail.life,
            peak: PEAK,
            layer: 2,
            seed,
        },
    );
    let [red, green, blue] = trail.tint;
    let _ = world.insert(mote, (Color(Vec4::new(red, green, blue, 0.0)), Mote));
    let _ = world.remove_one::<Lit>(mote);
}

/// Every flying round with a trail leaves a mote behind now and then, up to a cap.
pub(crate) fn shed_trails(world: &mut World, textures: &TrailTextures, dt: f32, clock: f32) {
    let mut motes = world.query::<&Mote>().iter().count();
    let mut due = Vec::new();
    for (bullet, position, velocity, trailing) in world
        .query_mut::<(&Bullet, &Position, &Velocity, &mut Trailing)>()
        .without::<&Expiring>()
    {
        let Some(trail) = bullet.round.spec().trail else {
            continue;
        };
        trailing.cooldown -= dt;
        if trailing.cooldown > 0.0 || motes >= MAX_MOTES {
            continue;
        }
        trailing.cooldown = trail.every;
        motes += 1;
        due.push((trail, position.0, velocity.0));
    }
    for (trail, at, velocity) in due {
        let seed = clock * 9.17 + at.x * 0.011 + at.y * 0.019;
        shed(world, textures.get(trail), trail, at, velocity, seed);
    }
}

/// Sparks scattered along a lightning arc between two points.
pub(crate) fn arc_motes(world: &mut World, texture: TextureId, from: Vec2, to: Vec2) {
    let span = to - from;
    for index in 0..ARC_MOTES {
        let along = (index as f32 + 0.5) / ARC_MOTES as f32;
        let at = from + span * along;
        let seed = at.x * 0.013 + at.y * 0.017 + index as f32;
        let scatter = Trail {
            spread: ARC_SCATTER,
            size: ARC_MOTE_SIZE,
            life: ARC_MOTE_LIFE,
            ..Trail::VOLT
        };
        shed(world, texture, scatter, at, span, seed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guns::Round;

    fn bullet(world: &mut World, round: Round) -> hecs::Entity {
        let entity = world.spawn((
            Bullet::plain(1).as_round(round),
            Position(Vec2::new(100.0, 100.0)),
            Velocity(Vec2::new(600.0, 0.0)),
        ));
        if round.spec().trail.is_some() {
            let _ = world.insert_one(entity, Trailing::default());
        }
        entity
    }

    fn motes(world: &World) -> usize {
        world.query::<&Mote>().iter().count()
    }

    #[test]
    fn a_spark_sheds_motes_as_it_flies_and_a_plain_bullet_does_not() {
        let mut world = World::new();
        bullet(&mut world, Round::Spark);
        bullet(&mut world, Round::Bullet);

        shed_trails(&mut world, &TrailTextures::white(), 1.0 / 60.0, 0.0);
        assert_eq!(motes(&world), 1);

        shed_trails(&mut world, &TrailTextures::white(), 1.0 / 60.0, 0.1);
        assert_eq!(motes(&world), 1, "the next mote waits for the cooldown");

        shed_trails(&mut world, &TrailTextures::white(), 0.1, 0.2);
        assert_eq!(motes(&world), 2);
    }

    #[test]
    fn a_laser_mote_is_tinted_red_and_thrown_sideways_not_forwards() {
        let mut world = World::new();
        bullet(&mut world, Round::Laser);
        shed_trails(&mut world, &TrailTextures::white(), 0.1, 0.0);

        let (color, position) = world
            .query::<(&Color, &Position)>()
            .with::<&Mote>()
            .iter()
            .map(|(color, position)| (color.0, position.0))
            .next()
            .expect("a mote");
        assert!(color.x > color.y && color.x > color.z, "{color:?}");
        assert_eq!(position, Vec2::new(100.0, 100.0));

        crate::systems::dust::tick_dust(&mut world, 0.0, 0.1);
        let moved = world
            .query::<&Position>()
            .with::<&Mote>()
            .iter()
            .map(|position| position.0)
            .next()
            .expect("a mote");
        assert!(moved.x < 100.0, "it drifts behind the round: {moved:?}");
    }

    #[test]
    fn a_round_that_already_hit_something_sheds_nothing_more() {
        let mut world = World::new();
        let spark = bullet(&mut world, Round::Spark);
        let _ = world.insert_one(spark, Expiring);

        shed_trails(&mut world, &TrailTextures::white(), 0.1, 0.0);
        assert_eq!(motes(&world), 0);
    }

    #[test]
    fn motes_are_capped_so_a_minigun_of_sparks_cannot_flood_the_scene() {
        let mut world = World::new();
        for _ in 0..(MAX_MOTES + 20) {
            bullet(&mut world, Round::Spark);
        }
        shed_trails(&mut world, &TrailTextures::white(), 0.1, 0.0);
        assert_eq!(motes(&world), MAX_MOTES);
    }

    #[test]
    fn an_arc_scatters_a_few_sparks_along_its_length() {
        let mut world = World::new();
        arc_motes(&mut world, TextureId::WHITE, Vec2::ZERO, Vec2::new(200.0, 0.0));

        let xs: Vec<f32> = world
            .query::<&Position>()
            .with::<&Mote>()
            .iter()
            .map(|position| position.0.x)
            .collect();
        assert_eq!(xs.len(), ARC_MOTES);
        assert!(xs.iter().all(|x| *x > 0.0 && *x < 200.0), "{xs:?}");
    }
}
