//! Mine carts on their rails, laid out with the floor; a rail may turn by a right angle once or
//! twice. Walking into a cart or shooting it sends it rolling along the rail, away from the push,
//! round the turns all the way to the buffer at the end, where it stops; on the way it throws
//! sparks off its wheels and runs over enemies for heavy damage. Only the player can set it
//! going.

use std::f32::consts::{FRAC_PI_2, PI};

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::components::{
    Body, BodyOffset, Bullet, Color, DropShadow, Enemy, Heading, Layer, Lit, Player, Position, Size, Spawning, Sprite,
    Velocity, YSort,
};
use crate::dungeon::{Dungeon, Track};
use crate::physics::Physics;
use crate::render::TextureId;
use crate::rng::cosmetic;
use crate::tilemap::TILE_SIZE;

use super::damage::{Damage, DamageSource, deal};

const COLUMNS: usize = 5;
/// One row of pictures per floor's look (see `Theme::style`).
const ROWS: usize = 7;
const RAIL: usize = 0;
const BUFFER: usize = 1;
const CART: usize = 2;
const TURN: usize = 4;
/// Points along a turn's curve, for a smooth ride round it.
const ARC_STEPS: usize = 6;
const RAIL_LAYER: i32 = -2;
const CART_SIZE: Vec2 = Vec2::splat(TILE_SIZE);
const CART_BODY: Vec2 = Vec2::splat(48.0);
/// The cart's picture sits a little up on the rails.
const CART_LIFT: f32 = 6.0;

/// How fast a push sends the cart, in world units a second.
const PUSH_SPEED: f32 = 620.0;
/// How fast the player has to walk into the cart to shove it.
const SHOVE_SPEED: f32 = 40.0;
/// How close the player has to be to count as walking into it.
const SHOVE_REACH: f32 = 8.0;
/// A push this far across the rail does not move the cart.
const PUSH_ALONG: f32 = 0.4;
/// How far short of the middle of the last tiles the cart stops, against the buffers.
const STOP_SHORT: f32 = 6.0;
const HURT_SPEED: f32 = 180.0;
pub(crate) const CART_DAMAGE: i32 = 20;
/// An enemy run over is safe from the same cart for this long.
const HURT_COOLDOWN: f32 = 0.6;
const SPARK_SPEED: f32 = 140.0;
/// Sparks a second per world unit a second of speed.
const SPARKS_PER_SPEED: f32 = 0.06;
const SPARK_SIZE: f32 = 32.0;
const SPARK_LIFE: f32 = 0.45;
const SPARK_LAYER: i32 = 3;
const MOTE_FRAMES: usize = 8;
const SPARK_FRAMES: [usize; 2] = [1, 2];
/// A roll of this length turns the wheels by one picture.
const WHEEL_STEP: f32 = 16.0;

/// The line a cart rolls along: through the middles of the rail tiles and round each turn on a
/// quarter circle, with the distance along it to every point.
#[derive(Debug, Clone)]
pub(crate) struct Rail {
    points: Vec<Vec2>,
    lengths: Vec<f32>,
}

impl Rail {
    fn new(points: Vec<Vec2>) -> Self {
        let mut lengths = vec![0.0];
        for pair in points.windows(2) {
            let last = lengths.last().copied().unwrap_or(0.0);
            lengths.push(last + pair[0].distance(pair[1]));
        }
        Self { points, lengths }
    }

    fn through(path: &[(usize, usize)]) -> Self {
        let centre = tile_centre;
        let mut points = vec![centre(path[0])];
        for index in 0..path.len() - 1 {
            let edge = (centre(path[index]) + centre(path[index + 1])) / 2.0;
            points.push(edge);
            let Some(&after) = path.get(index + 2) else {
                continue;
            };
            let next_edge = (centre(path[index + 1]) + centre(after)) / 2.0;
            let turning = (edge - centre(path[index + 1])).dot(next_edge - centre(path[index + 1]))
                .abs()
                < 1.0;
            if turning {
                // Round the turn about the tile corner both edges touch.
                let pivot = edge + next_edge - centre(path[index + 1]);
                let from = (edge - pivot).to_angle();
                let mut sweep = (next_edge - pivot).to_angle() - from;
                if sweep > PI {
                    sweep -= 2.0 * PI;
                } else if sweep < -PI {
                    sweep += 2.0 * PI;
                }
                for step in 1..ARC_STEPS {
                    let angle = from + sweep * step as f32 / ARC_STEPS as f32;
                    points.push(pivot + Vec2::from_angle(angle) * TILE_SIZE / 2.0);
                }
            }
        }
        points.push(centre(path[path.len() - 1]));
        Self::new(points)
    }

    /// How far along the line the point of it nearest to `point` is.
    fn nearest(&self, point: Vec2) -> f32 {
        let steps = self.length().ceil() as usize;
        (0..=steps)
            .map(|step| step as f32)
            .min_by(|a, b| {
                let (a, b) = (self.at(*a).0.distance(point), self.at(*b).0.distance(point));
                a.total_cmp(&b)
            })
            .unwrap_or(0.0)
    }

    fn length(&self) -> f32 {
        self.lengths.last().copied().unwrap_or(0.0)
    }

    /// Where on the line a distance along it is, and which way the line runs there.
    fn at(&self, distance: f32) -> (Vec2, Vec2) {
        let distance = distance.clamp(0.0, self.length());
        let segment = self
            .lengths
            .windows(2)
            .position(|pair| distance <= pair[1])
            .unwrap_or(self.lengths.len().saturating_sub(2));
        let (from, to) = (self.points[segment], self.points[segment + 1]);
        let span = self.lengths[segment + 1] - self.lengths[segment];
        let t = if span > 0.0 {
            (distance - self.lengths[segment]) / span
        } else {
            0.0
        };
        (from.lerp(to, t), (to - from).normalize_or_zero())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Cart {
    rail: Rail,
    distance: f32,
    min: f32,
    max: f32,
    speed: f32,
    rolled: f32,
    hurt: Vec<(Entity, f32)>,
    sparks_due: f32,
    sparks: TextureId,
    /// Its floor's look: its row in the sheet.
    style: usize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct CartSpark {
    velocity: Vec2,
    age: f32,
}

fn tile_centre(tile: (usize, usize)) -> Vec2 {
    (Vec2::new(tile.0 as f32, tile.1 as f32) + 0.5) * TILE_SIZE
}

/// A quarter turn clockwise on the screen, the way a sprite turns for a positive heading.
fn quarter(direction: (isize, isize)) -> (isize, isize) {
    (-direction.1, direction.0)
}

/// How many quarter turns bring the picture's own directions onto the wanted ones.
fn quarters(own: &[(isize, isize)], wanted: &[(isize, isize)]) -> usize {
    (0..4)
        .find(|&turns| {
            own.iter().all(|&direction| {
                let turned = (0..turns).fold(direction, |d, _| quarter(d));
                wanted.contains(&turned)
            })
        })
        .unwrap_or(0)
}

fn step(from: (usize, usize), to: (usize, usize)) -> (isize, isize) {
    (to.0 as isize - from.0 as isize, to.1 as isize - from.1 as isize)
}

/// Which picture each rail tile gets and how far it is turned.
fn rail_pieces(track: &Track) -> Vec<(usize, f32)> {
    const WEST: (isize, isize) = (-1, 0);
    const EAST: (isize, isize) = (1, 0);
    const SOUTH: (isize, isize) = (0, 1);
    let last = track.path.len() - 1;
    (0..=last)
        .map(|index| {
            let here = track.path[index];
            let neighbours: Vec<(isize, isize)> = [index.checked_sub(1), Some(index + 1)]
                .into_iter()
                .flatten()
                .filter(|&other| other <= last)
                .map(|other| step(here, track.path[other]))
                .collect();
            let (cell, own): (usize, &[(isize, isize)]) = if neighbours.len() == 1 {
                (BUFFER, &[WEST])
            } else if neighbours[0].0 == -neighbours[1].0 && neighbours[0].1 == -neighbours[1].1 {
                (RAIL, &[WEST, EAST])
            } else {
                (TURN, &[WEST, SOUTH])
            };
            (cell, quarters(own, &neighbours) as f32 * FRAC_PI_2)
        })
        .collect()
}

/// A cart lies along the rail, but never upside down: going left it still looks like going right.
fn lying(direction: Vec2) -> f32 {
    let angle = direction.to_angle();
    if angle > FRAC_PI_2 + 0.01 {
        angle - PI
    } else if angle <= -FRAC_PI_2 + 0.01 {
        angle + PI
    } else {
        angle
    }
}

pub(crate) fn spawn_tracks(
    world: &mut World,
    physics: &mut Physics,
    dungeon: &Dungeon,
    textures: &Textures,
) {
    let texture = textures.get(tex::RAILS);
    let sparks = textures.get(tex::MOTES);
    let style = dungeon.theme.style();
    for track in dungeon.rooms.iter().filter_map(|room| room.track.as_ref()) {
        for (&tile, (cell, angle)) in track.path.iter().zip(rail_pieces(track)) {
            let at = (Vec2::new(tile.0 as f32, tile.1 as f32) + 0.5) * TILE_SIZE;
            world.spawn((
                Position(at),
                Size(Vec2::splat(TILE_SIZE)),
                Color(Vec4::ONE),
                Sprite::cell(texture, cell, style, COLUMNS, ROWS),
                Heading(angle),
                Layer(RAIL_LAYER),
                Lit,
            ));
        }
        let rail = Rail::through(&track.path);
        let min = TILE_SIZE - STOP_SHORT;
        let max = rail.length() - TILE_SIZE + STOP_SHORT;
        let distance = rail.nearest(tile_centre(track.path[track.cart])).clamp(min, max);
        let (start, direction) = rail.at(distance);
        let cart = world.spawn((
            Cart {
                rail,
                distance,
                min,
                max,
                speed: 0.0,
                rolled: 0.0,
                hurt: Vec::new(),
                sparks_due: 0.0,
                sparks,
                style,
            },
            Position(start - Vec2::new(0.0, CART_LIFT)),
            Size(CART_SIZE),
            Color(Vec4::ONE),
            Sprite::cell(texture, CART, style, COLUMNS, ROWS),
            Heading(lying(direction)),
            Layer(Layer::ACTORS.0),
            Lit,
            YSort {
                foot: CART_SIZE.y / 2.0 - CART_LIFT,
            },
            DropShadow::default(),
            BodyOffset(Vec2::new(0.0, -CART_LIFT)),
        ));
        let body = physics.add_moving_object(cart, start, CART_BODY);
        let _ = world.insert_one(cart, body);
    }
}

fn overlaps(a: Vec2, a_size: Vec2, b: Vec2, b_size: Vec2) -> bool {
    let reach = (a_size + b_size) / 2.0;
    (a - b).abs().cmplt(reach).all()
}

/// Which way along the rail, if any, the player or a bullet is pushing a cart at `at`.
fn push(world: &World, at: Vec2, along: Vec2) -> Option<f32> {
    let way = |velocity: Vec2| {
        let share = velocity.normalize_or_zero().dot(along);
        (share.abs() >= PUSH_ALONG).then_some(share.signum())
    };
    let shove = world
        .query::<(&Position, &Size, &Velocity)>()
        .with::<&Player>()
        .iter()
        .find_map(|(position, size, velocity)| {
            let gap = at - position.0;
            let reach = (CART_BODY + size.0) / 2.0 + Vec2::splat(SHOVE_REACH);
            let beside = gap.abs().cmple(reach).all();
            let towards = velocity.0.dot(gap) > 0.0 && velocity.0.length() >= SHOVE_SPEED;
            if beside && towards { way(velocity.0) } else { None }
        });
    shove.or_else(|| {
        world
            .query::<(&Position, &Size, &Velocity)>()
            .with::<&Bullet>()
            .iter()
            .find_map(|(position, size, velocity)| {
                let hit = overlaps(position.0, size.0 + Vec2::splat(SHOVE_REACH), at, CART_BODY);
                if hit { way(velocity.0) } else { None }
            })
    })
}

pub(crate) fn tick_carts(world: &mut World, physics: &mut Physics, dt: f32) {
    let carts: Vec<Entity> = world.query::<Entity>().with::<&Cart>().iter().collect();
    for entity in carts {
        let Ok(mut cart) = world.get::<&Cart>(entity).map(|cart| (*cart).clone()) else {
            continue;
        };
        let Some(&body) = world.get::<&Body>(entity).ok().as_deref() else {
            continue;
        };
        let (at, along) = cart.rail.at(cart.distance);
        if let Some(direction) = push(world, at, along) {
            let at_the_end = (direction < 0.0 && cart.distance <= cart.min)
                || (direction > 0.0 && cart.distance >= cart.max);
            if !at_the_end {
                cart.speed = PUSH_SPEED * direction;
            }
        }
        let before = cart.distance;
        cart.distance += cart.speed * dt;
        if cart.distance <= cart.min || cart.distance >= cart.max {
            cart.distance = cart.distance.clamp(cart.min, cart.max);
            cart.speed = 0.0;
        }
        cart.rolled += (cart.distance - before).abs();
        let (to, along) = cart.rail.at(cart.distance);
        // The picture follows the body through the simulation, lifted by its BodyOffset.
        physics.move_to(body, to);
        if let Ok(mut heading) = world.get::<&mut Heading>(entity) {
            heading.0 = lying(along);
        }
        if let Ok(mut sprite) = world.get::<&mut Sprite>(entity) {
            let frame = CART + ((cart.rolled / WHEEL_STEP) as usize % 2);
            sprite.uv = Sprite::cell_uv(frame, cart.style, COLUMNS, ROWS);
        }
        run_over(world, &mut cart, to, dt);
        throw_sparks(world, &mut cart, to, along, dt);
        if let Ok(mut held) = world.get::<&mut Cart>(entity) {
            *held = cart;
        }
    }
    fade_sparks(world, dt);
}

fn run_over(world: &mut World, cart: &mut Cart, at: Vec2, dt: f32) {
    cart.hurt.retain_mut(|(_, left)| {
        *left -= dt;
        *left > 0.0
    });
    if cart.speed.abs() < HURT_SPEED {
        return;
    }
    let victims: Vec<Entity> = world
        .query::<(Entity, &Position, &Size)>()
        .with::<&Enemy>()
        .without::<&Spawning>()
        .iter()
        .filter(|(entity, position, size)| {
            overlaps(position.0, size.0 * 0.6, at, CART_BODY)
                && cart.hurt.iter().all(|(hurt, _)| hurt != entity)
        })
        .map(|(entity, ..)| entity)
        .collect();
    for target in victims {
        deal(
            world,
            Damage {
                target,
                amount: CART_DAMAGE,
                source: DamageSource::Trap,
            },
        );
        cart.hurt.push((target, HURT_COOLDOWN));
    }
}

fn throw_sparks(world: &mut World, cart: &mut Cart, at: Vec2, along: Vec2, dt: f32) {
    let speed = cart.speed.abs();
    if speed < SPARK_SPEED {
        cart.sparks_due = 0.0;
        return;
    }
    cart.sparks_due += speed * SPARKS_PER_SPEED * dt;
    let behind = -along * cart.speed.signum();
    let side = along.perp();
    while cart.sparks_due >= 1.0 {
        cart.sparks_due -= 1.0;
        let wheel = if cosmetic() < 0.5 { -18.0 } else { 18.0 };
        let from = at + side * wheel + behind * 18.0 + Vec2::new(0.0, 12.0);
        let velocity = behind * (60.0 + 140.0 * cosmetic()) - Vec2::new(0.0, 40.0 + 120.0 * cosmetic());
        let frame = SPARK_FRAMES[usize::from(cosmetic() < 0.5)];
        world.spawn((
            CartSpark { velocity, age: 0.0 },
            Position(from),
            Size(Vec2::splat(SPARK_SIZE)),
            Color(Vec4::ONE),
            Sprite::cell(cart.sparks, frame, 0, MOTE_FRAMES, 1),
            Layer(SPARK_LAYER),
        ));
    }
}

fn fade_sparks(world: &mut World, dt: f32) {
    let mut gone = Vec::new();
    for (entity, spark, position, color) in
        world.query_mut::<(Entity, &mut CartSpark, &mut Position, &mut Color)>()
    {
        spark.age += dt;
        spark.velocity.y += 360.0 * dt;
        position.0 += spark.velocity * dt;
        color.0.w = (1.0 - spark.age / SPARK_LIFE).clamp(0.0, 1.0);
        if spark.age >= SPARK_LIFE {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::Health;

    fn rat() -> Enemy {
        Enemy {
            kind: crate::enemies::EnemyKind::Rat,
            death: crate::components::Clip {
                row: 0,
                frames: 1,
                frame_time: 1.0,
                looping: false,
                flipped: false,
            },
        }
    }

    /// A straight rail of ten tiles along row 7, or one turning down after six.
    fn track(turning: bool) -> Track {
        let mut path: Vec<(usize, usize)> = (2..8).map(|column| (column, 7)).collect();
        if turning {
            path.extend((8..12).map(|row| (7, row)));
        } else {
            path.extend((8..12).map(|column| (column, 7)));
        }
        Track { path, cart: 3 }
    }

    fn yard(turning: bool) -> (World, Physics, Entity) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let rail = Rail::through(&track(turning).path);
        let min = TILE_SIZE - STOP_SHORT;
        let max = rail.length() - TILE_SIZE + STOP_SHORT;
        let distance = 3.0 * TILE_SIZE;
        let (start, direction) = rail.at(distance);
        let cart = world.spawn((
            Cart {
                rail,
                distance,
                min,
                max,
                speed: 0.0,
                rolled: 0.0,
                hurt: Vec::new(),
                sparks_due: 0.0,
                sparks: TextureId::WHITE,
                style: 0,
            },
            Position(start),
            Size(CART_SIZE),
            Color(Vec4::ONE),
            Sprite::whole(TextureId::WHITE),
            Heading(lying(direction)),
        ));
        let body = physics.add_moving_object(cart, start, CART_BODY);
        let _ = world.insert_one(cart, body);
        (world, physics, cart)
    }

    fn cart_at(world: &World, physics: &Physics, cart: Entity) -> Vec2 {
        let body = *world.get::<&Body>(cart).expect("a body");
        physics.position(body).unwrap_or_default()
    }

    fn run(world: &mut World, physics: &mut Physics, seconds: f32) {
        for _ in 0..(seconds * 60.0) as usize {
            tick_carts(world, physics, 1.0 / 60.0);
            physics.step(1.0 / 60.0);
        }
    }

    fn speed(world: &World, cart: Entity) -> f32 {
        world.get::<&Cart>(cart).map(|cart| cart.speed).unwrap_or(0.0)
    }

    fn set_speed(world: &mut World, cart: Entity, speed: f32) {
        if let Ok(mut held) = world.get::<&mut Cart>(cart) {
            held.speed = speed;
        }
    }

    #[test]
    fn the_rail_runs_through_the_tile_middles_and_rounds_the_turn() {
        let rail = Rail::through(&track(true).path);
        let (start, along) = rail.at(0.0);
        assert_eq!(start, Vec2::new(2.5, 7.5) * TILE_SIZE);
        assert_eq!(along, Vec2::X);
        let (end, along) = rail.at(rail.length());
        assert_eq!(end, Vec2::new(7.5, 11.5) * TILE_SIZE);
        assert!(along.distance(Vec2::Y) < 1e-4);
        // Rounding the corner is shorter than going through its middle.
        assert!(rail.length() < 9.0 * TILE_SIZE);
        assert!(rail.length() > 8.5 * TILE_SIZE);
    }

    #[test]
    fn each_tile_gets_the_right_piece_turned_the_right_way() {
        let pieces = rail_pieces(&track(true));
        assert_eq!(pieces[0], (BUFFER, PI));
        assert_eq!(pieces[1], (RAIL, 0.0));
        // The turn from the left round to below needs no turning of its own picture.
        assert_eq!(pieces[5], (TURN, 0.0));
        assert_eq!(pieces[6], (RAIL, FRAC_PI_2));
        assert_eq!(pieces[9], (BUFFER, FRAC_PI_2));
    }

    #[test]
    fn walking_into_the_cart_sends_it_rolling_away_from_the_player() {
        let (mut world, mut physics, cart) = yard(false);
        let at = cart_at(&world, &physics, cart);
        world.spawn((
            Player,
            Position(at - Vec2::new(60.0, 0.0)),
            Size(Vec2::splat(56.0)),
            Velocity(Vec2::new(200.0, 0.0)),
        ));
        tick_carts(&mut world, &mut physics, 1.0 / 60.0);
        assert!(speed(&world, cart) > PUSH_SPEED * 0.9);
    }

    #[test]
    fn a_bullet_pushes_it_the_way_it_flies_and_enemies_never_do() {
        let (mut world, mut physics, cart) = yard(false);
        let at = cart_at(&world, &physics, cart);
        world.spawn((
            rat(),
            Position(at - Vec2::new(40.0, 0.0)),
            Size(Vec2::splat(48.0)),
            Velocity(Vec2::new(300.0, 0.0)),
        ));
        tick_carts(&mut world, &mut physics, 1.0 / 60.0);
        assert_eq!(speed(&world, cart), 0.0);

        world.spawn((
            Bullet::plain(1),
            Position(at + Vec2::new(30.0, 0.0)),
            Size(Vec2::splat(8.0)),
            Velocity(Vec2::new(-900.0, 0.0)),
        ));
        tick_carts(&mut world, &mut physics, 1.0 / 60.0);
        assert!(speed(&world, cart) < -PUSH_SPEED * 0.9);
    }

    #[test]
    fn a_push_across_the_rail_does_not_move_it() {
        let (mut world, mut physics, cart) = yard(false);
        let at = cart_at(&world, &physics, cart);
        world.spawn((
            Bullet::plain(1),
            Position(at),
            Size(Vec2::splat(8.0)),
            Velocity(Vec2::new(0.0, 900.0)),
        ));
        tick_carts(&mut world, &mut physics, 1.0 / 60.0);
        assert_eq!(speed(&world, cart), 0.0);
    }

    #[test]
    fn it_rolls_round_the_turn_to_the_buffer_stops_there_and_goes_back_when_pushed() {
        let (mut world, mut physics, cart) = yard(true);
        set_speed(&mut world, cart, PUSH_SPEED);
        let mut turned = false;
        for _ in 0..(4 * 60) {
            tick_carts(&mut world, &mut physics, 1.0 / 60.0);
            physics.step(1.0 / 60.0);
            assert!(speed(&world, cart) >= 0.0, "it never comes back by itself");
            let heading = world.get::<&Heading>(cart).map(|h| h.0).unwrap_or(0.0);
            turned |= (heading - FRAC_PI_2).abs() < 0.01;
        }
        assert!(turned, "it faces down the rail after the turn");
        let end = cart_at(&world, &physics, cart);
        assert!(end.distance(Vec2::new(7.5, 10.5) * TILE_SIZE) <= STOP_SHORT + 1.0, "{end}");

        set_speed(&mut world, cart, -PUSH_SPEED);
        run(&mut world, &mut physics, 4.0);
        let back = cart_at(&world, &physics, cart);
        assert!(back.distance(Vec2::new(3.5, 7.5) * TILE_SIZE) <= STOP_SHORT + 1.0, "{back}");
    }

    #[test]
    fn a_rolling_cart_runs_over_an_enemy_once_per_pass() {
        let (mut world, mut physics, cart) = yard(false);
        let at = cart_at(&world, &physics, cart);
        let rat = world.spawn((
            rat(),
            Health {
                current: 100,
                max: 100,
            },
            Position(at + Vec2::new(100.0, 0.0)),
            Size(Vec2::splat(48.0)),
        ));
        set_speed(&mut world, cart, PUSH_SPEED);
        run(&mut world, &mut physics, 0.3);
        let health = world.get::<&Health>(rat).map(|h| h.current).unwrap_or(0);
        assert_eq!(health, 100 - CART_DAMAGE);
    }

    #[test]
    fn a_standing_cart_hurts_nobody_and_throws_no_sparks() {
        let (mut world, mut physics, cart) = yard(false);
        let at = cart_at(&world, &physics, cart);
        let rat = world.spawn((
            rat(),
            Health {
                current: 100,
                max: 100,
            },
            Position(at),
            Size(Vec2::splat(48.0)),
        ));
        run(&mut world, &mut physics, 1.0);
        assert_eq!(world.get::<&Health>(rat).map(|h| h.current).unwrap_or(0), 100);
        assert_eq!(world.query::<&CartSpark>().iter().count(), 0);
    }
}
