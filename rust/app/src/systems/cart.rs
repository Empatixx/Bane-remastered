//! Mine carts on their rails, laid out with the floor. Walking into a cart or shooting it
//! sends it rolling the other way all along the rail until it stops at the buffer at the end; on
//! the way it throws sparks off its wheels and runs over enemies for heavy damage. Only the
//! player can set it going.

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::components::{
    Body, Bullet, Color, Enemy, Layer, Lit, Player, Position, Size, Spawning, Sprite, Velocity,
    YSort,
};
use crate::dungeon::Dungeon;
use crate::physics::Physics;
use crate::render::TextureId;
use crate::rng::cosmetic;
use crate::tilemap::TILE_SIZE;

use super::damage::{Damage, DamageSource, deal};

const COLUMNS: usize = 4;
const RAIL: usize = 0;
const BUFFER: usize = 1;
const CART: usize = 2;
const RAIL_LAYER: i32 = -2;
const CART_SIZE: Vec2 = Vec2::splat(TILE_SIZE);
const CART_BODY: Vec2 = Vec2::new(52.0, 40.0);
/// The cart's picture sits a little up on the rails.
const CART_LIFT: f32 = 6.0;

/// How fast a push sends the cart, in world units a second.
const PUSH_SPEED: f32 = 620.0;
/// How fast the player has to walk into the cart to shove it.
const SHOVE_SPEED: f32 = 40.0;
/// How close the player has to be to count as walking into it.
const SHOVE_REACH: f32 = 8.0;
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

#[derive(Debug, Clone)]
pub(crate) struct Cart {
    min_x: f32,
    max_x: f32,
    y: f32,
    speed: f32,
    rolled: f32,
    hurt: Vec<(Entity, f32)>,
    sparks_due: f32,
    sparks: TextureId,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct CartSpark {
    velocity: Vec2,
    age: f32,
}

pub(crate) fn spawn_tracks(
    world: &mut World,
    physics: &mut Physics,
    dungeon: &Dungeon,
    textures: &Textures,
) {
    let texture = textures.get(tex::RAILS);
    let sparks = textures.get(tex::MOTES);
    for track in dungeon.rooms.iter().filter_map(|room| room.track) {
        let y = (track.row as f32 + 0.5) * TILE_SIZE;
        for column in track.left..=track.right {
            let at = Vec2::new((column as f32 + 0.5) * TILE_SIZE, y);
            let end = column == track.left || column == track.right;
            let mut sprite = Sprite::cell(texture, if end { BUFFER } else { RAIL }, 0, COLUMNS, 1);
            if column == track.left {
                sprite.uv = Sprite::flipped(sprite.uv);
            }
            world.spawn((
                Position(at),
                Size(Vec2::splat(TILE_SIZE)),
                Color(Vec4::ONE),
                sprite,
                Layer(RAIL_LAYER),
                Lit,
            ));
        }
        let start = Vec2::new((track.cart as f32 + 0.5) * TILE_SIZE, y);
        let cart = world.spawn((
            Cart {
                min_x: (track.left as f32 + 1.0) * TILE_SIZE + CART_BODY.x / 2.0,
                max_x: track.right as f32 * TILE_SIZE - CART_BODY.x / 2.0,
                y,
                speed: 0.0,
                rolled: 0.0,
                hurt: Vec::new(),
                sparks_due: 0.0,
                sparks,
            },
            Position(start - Vec2::new(0.0, CART_LIFT)),
            Size(CART_SIZE),
            Color(Vec4::ONE),
            Sprite::cell(texture, CART, 0, COLUMNS, 1),
            Layer(Layer::ACTORS.0),
            Lit,
            YSort {
                foot: CART_SIZE.y / 2.0 - CART_LIFT,
            },
        ));
        let body = physics.add_moving_object(cart, start, CART_BODY);
        let _ = world.insert_one(cart, body);
    }
}

fn overlaps(a: Vec2, a_size: Vec2, b: Vec2, b_size: Vec2) -> bool {
    let reach = (a_size + b_size) / 2.0;
    (a - b).abs().cmplt(reach).all()
}

/// Which way, if any, the player or a bullet is pushing a cart at `at`.
fn push(world: &World, at: Vec2) -> Option<f32> {
    let shove = world
        .query::<(&Position, &Size, &Velocity)>()
        .with::<&Player>()
        .iter()
        .find_map(|(position, size, velocity)| {
            let gap = at - position.0;
            let reach = (CART_BODY + size.0) / 2.0;
            let beside = gap.x.abs() <= reach.x + SHOVE_REACH && gap.y.abs() < reach.y;
            let towards = velocity.0.x * gap.x > 0.0 && velocity.0.x.abs() >= SHOVE_SPEED;
            (beside && towards).then_some(velocity.0.x.signum())
        });
    shove.or_else(|| {
        world
            .query::<(&Position, &Size, &Velocity)>()
            .with::<&Bullet>()
            .iter()
            .find_map(|(position, size, velocity)| {
                let hit = overlaps(position.0, size.0 + Vec2::splat(SHOVE_REACH), at, CART_BODY);
                (hit && velocity.0.x.abs() > 1.0).then_some(velocity.0.x.signum())
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
        let at = physics.position(body).unwrap_or(Vec2::new(cart.min_x, cart.y));
        if let Some(direction) = push(world, at) {
            let at_the_end = (direction < 0.0 && at.x <= cart.min_x)
                || (direction > 0.0 && at.x >= cart.max_x);
            if !at_the_end {
                cart.speed = PUSH_SPEED * direction;
            }
        }
        let mut next = at.x + cart.speed * dt;
        if next <= cart.min_x || next >= cart.max_x {
            next = next.clamp(cart.min_x, cart.max_x);
            cart.speed = 0.0;
        }
        cart.rolled += (next - at.x).abs();
        let to = Vec2::new(next, cart.y);
        physics.move_to(body, to);
        if let Ok(mut position) = world.get::<&mut Position>(entity) {
            position.0 = to - Vec2::new(0.0, CART_LIFT);
        }
        if let Ok(mut sprite) = world.get::<&mut Sprite>(entity) {
            let frame = CART + ((cart.rolled / WHEEL_STEP) as usize % 2);
            sprite.uv = Sprite::cell_uv(frame, 0, COLUMNS, 1);
        }
        run_over(world, &mut cart, to, dt);
        throw_sparks(world, &mut cart, to, dt);
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

fn throw_sparks(world: &mut World, cart: &mut Cart, at: Vec2, dt: f32) {
    let speed = cart.speed.abs();
    if speed < SPARK_SPEED {
        cart.sparks_due = 0.0;
        return;
    }
    cart.sparks_due += speed * SPARKS_PER_SPEED * dt;
    let behind = -cart.speed.signum();
    while cart.sparks_due >= 1.0 {
        cart.sparks_due -= 1.0;
        let wheel = if cosmetic() < 0.5 { -18.0 } else { 18.0 };
        let from = at + Vec2::new(wheel + behind * 8.0, 20.0);
        let velocity = Vec2::new(
            behind * (60.0 + 140.0 * cosmetic()),
            -(40.0 + 120.0 * cosmetic()),
        );
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

    const TRACK_Y: f32 = 500.0;

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

    fn yard() -> (World, Physics, Entity) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let start = Vec2::new(600.0, TRACK_Y);
        let cart = world.spawn((
            Cart {
                min_x: 300.0,
                max_x: 900.0,
                y: TRACK_Y,
                speed: 0.0,
                rolled: 0.0,
                hurt: Vec::new(),
                sparks_due: 0.0,
                sparks: TextureId::WHITE,
            },
            Position(start),
            Size(CART_SIZE),
            Color(Vec4::ONE),
            Sprite::whole(TextureId::WHITE),
        ));
        let body = physics.add_moving_object(cart, start, CART_BODY);
        let _ = world.insert_one(cart, body);
        (world, physics, cart)
    }

    fn step(world: &mut World, physics: &mut Physics, seconds: f32) {
        for _ in 0..(seconds * 60.0) as usize {
            tick_carts(world, physics, 1.0 / 60.0);
            physics.step(1.0 / 60.0);
        }
    }

    fn speed(world: &World, cart: Entity) -> f32 {
        world.get::<&Cart>(cart).map(|cart| cart.speed).unwrap_or(0.0)
    }

    #[test]
    fn walking_into_the_cart_sends_it_rolling_away_from_the_player() {
        let (mut world, mut physics, cart) = yard();
        world.spawn((
            Player,
            Position(Vec2::new(600.0 - 60.0, TRACK_Y)),
            Size(Vec2::splat(56.0)),
            Velocity(Vec2::new(200.0, 0.0)),
        ));
        tick_carts(&mut world, &mut physics, 1.0 / 60.0);
        assert!(speed(&world, cart) > PUSH_SPEED * 0.9);
    }

    #[test]
    fn a_bullet_pushes_it_the_way_it_flies_and_enemies_never_do() {
        let (mut world, mut physics, cart) = yard();
        world.spawn((
            rat(),
            Position(Vec2::new(560.0, TRACK_Y)),
            Size(Vec2::splat(48.0)),
            Velocity(Vec2::new(300.0, 0.0)),
        ));
        tick_carts(&mut world, &mut physics, 1.0 / 60.0);
        assert_eq!(speed(&world, cart), 0.0);

        world.spawn((
            Bullet::plain(1),
            Position(Vec2::new(630.0, TRACK_Y)),
            Size(Vec2::splat(8.0)),
            Velocity(Vec2::new(-900.0, 0.0)),
        ));
        tick_carts(&mut world, &mut physics, 1.0 / 60.0);
        assert!(speed(&world, cart) < -PUSH_SPEED * 0.9);
    }

    #[test]
    fn it_rolls_all_the_way_to_the_buffer_and_stops_there() {
        let (mut world, mut physics, cart) = yard();
        if let Ok(mut held) = world.get::<&mut Cart>(cart) {
            held.speed = PUSH_SPEED;
        }
        let mut stopped_at = None;
        for _ in 0..(5 * 60) {
            tick_carts(&mut world, &mut physics, 1.0 / 60.0);
            physics.step(1.0 / 60.0);
            let speed = speed(&world, cart);
            assert!(speed >= 0.0, "it never comes back by itself");
            if speed == 0.0 && stopped_at.is_none() {
                stopped_at = physics.position(*world.get::<&Body>(cart).expect("body")).map(|p| p.x);
            }
        }
        assert_eq!(stopped_at, Some(900.0));
        // Pushed again from that end, it rolls back the whole way.
        if let Ok(mut held) = world.get::<&mut Cart>(cart) {
            held.speed = -PUSH_SPEED;
        }
        step(&mut world, &mut physics, 3.0);
        let x = physics.position(*world.get::<&Body>(cart).expect("body")).map(|p| p.x);
        assert_eq!(x, Some(300.0));
    }

    #[test]
    fn a_rolling_cart_runs_over_an_enemy_once_per_pass() {
        let (mut world, mut physics, cart) = yard();
        let rat = world.spawn((
            rat(),
            Health {
                current: 100,
                max: 100,
            },
            Position(Vec2::new(700.0, TRACK_Y)),
            Size(Vec2::splat(48.0)),
        ));
        if let Ok(mut held) = world.get::<&mut Cart>(cart) {
            held.speed = PUSH_SPEED;
        }
        step(&mut world, &mut physics, 0.3);
        let health = world.get::<&Health>(rat).map(|h| h.current).unwrap_or(0);
        assert_eq!(health, 100 - CART_DAMAGE);
    }

    #[test]
    fn a_standing_cart_hurts_nobody_and_throws_no_sparks() {
        let (mut world, mut physics, _) = yard();
        let rat = world.spawn((
            rat(),
            Health {
                current: 100,
                max: 100,
            },
            Position(Vec2::new(600.0, TRACK_Y)),
            Size(Vec2::splat(48.0)),
        ));
        step(&mut world, &mut physics, 1.0);
        assert_eq!(world.get::<&Health>(rat).map(|h| h.current).unwrap_or(0), 100);
        assert_eq!(world.query::<&CartSpark>().iter().count(), 0);
    }
}
