use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Animation, Boss, Charge, Chase, Clip, Color, Dash, Detour, Enemy, Expiring, Frozen, Health,
    HitFlash, Layer, Outline, Player, Position, Puddle, Revives, Rooted, Size, Smash, Spawning,
    Slot, Sprite, Velocity,
};
use crate::flow::FlowField;
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::systems::crowd::{SHOOTER_FAR, SHOOTER_NEAR};

pub(crate) const FLASH_SECONDS: f32 = 0.3;
const HIT_OUTLINE: Vec4 = Vec4::new(1.0, 0.0, 0.0, 1.0);
const PLAYER_HIT_OUTLINE: Vec4 = Vec4::new(1.0, 0.3, 0.25, 1.0);
const DASH_PERIOD: f32 = 3.0;
const DASH_TRAIL_INTERVAL: f32 = 0.04;
const DASH_DURATION: f32 = 1.0;

fn player_placement(world: &World) -> Option<(Vec2, Vec2)> {
    world
        .query::<(&Position, &Size)>()
        .with::<&Player>()
        .without::<&crate::components::Phantom>()
        .iter()
        .next()
        .map(|(position, size)| (position.0, size.0))
}

/// Every chasing enemy's size, for the flow fields to be kept for.
fn chasing_sizes(world: &World) -> Vec<Vec2> {
    let mut classes: Vec<Vec2> = world
        .query::<&Size>()
        .with::<&Chase>()
        .without::<&Expiring>()
        .iter()
        .map(|size| crate::flow::size_class(size.0))
        .collect();
    classes.sort_by(|a, b| (a.x, a.y).partial_cmp(&(b.x, b.y)).unwrap_or(std::cmp::Ordering::Equal));
    classes.dedup();
    classes
}

pub(crate) fn update_flow(
    flow: &mut FlowField,
    world: &World,
    physics: &Physics,
    navigation: &Navigation,
    dt: f32,
) {
    if let Some((target, _)) = player_placement(world) {
        flow.update(target, &chasing_sizes(world), navigation, physics, dt);
    }
}

/// The ways an enemy can choose between each moment.
const DIRECTIONS: usize = 16;
/// How far ahead an enemy looks for walls: a third of a second's walk, within these bounds.
const LOOK_AHEAD_SECONDS: f32 = 0.35;
const LOOK_AHEAD: (f32, f32) = (40.0, 140.0);
/// Enemies keep this much room between their bodies, and start easing off this much further out.
const ELBOW_ROOM: f32 = 4.0;
const CROWD_REACH: f32 = 36.0;
/// An enemy whose body overlaps the player's by this share of its size has reached them: it
/// stands its ground there, hurting them, rather than pushing on into the others.
const REACHED: f32 = 0.85;
/// The places round the player enemies close in to, one each, so they come at the player from
/// all sides instead of shoving for the same spot. An enemy takes one when it comes within
/// `ENGAGE` of the player's reach and keeps it until it falls back beyond `RELEASE`; one that
/// finds them all taken waits within `WAITING_ROOM` for one to come free.
const SLOTS: usize = 16;
const ENGAGE: f32 = 220.0;
const RELEASE: f32 = 360.0;
const WAITING_ROOM: f32 = 150.0;
/// How far into the player an enemy's place is, as a share of the two bodies touching.
const SLOT_DEPTH: f32 = 0.92;
/// Two places are too close together when the bodies on them, at this share of their size,
/// would overlap.
const SLOT_ROOM: f32 = 0.9;
/// One waiting for a place prowls round the player at this share of its speed, keeping its
/// distance.
const PROWL: f32 = 0.35;
/// Close enough to its place to stand there.
const AT_SLOT: f32 = 12.0;
/// An enemy only gives way to one that is ahead of it, and not to one already going its way at
/// this share of its speed or more.
const AHEAD: f32 = 0.3;
const SAME_WAY: f32 = 0.6;
/// How quickly an enemy turns and gets up to speed towards where its steering points: most of
/// the way in a tenth of a second, so it bends its way round rather than jerking.
const TURN_RATE: f32 = 14.0;
/// Slower than this, an easing enemy has come to a stop.
const STANDING: f32 = 1.0;
/// A little of the way it was already going, so an enemy doesn't twitch between two choices.
const MOMENTUM: f32 = 0.25;
/// Stuck, an enemy shuns the way it was trying to go and favours one side of it.
const DETOUR_SHUN: f32 = 0.9;
const DETOUR_FAVOUR: f32 = 0.6;

/// One chasing enemy, as the others see it.
#[derive(Debug, Clone, Copy)]
struct Walker {
    entity: Entity,
    at: Vec2,
    size: Vec2,
    velocity: Vec2,
}

/// Context steering: each of the ways round an enemy is scored for how much it leads where the
/// enemy wants to go, and marked down for how soon it runs into a wall or how close it brings
/// the enemy to another; the best one wins, eased towards its neighbours.
fn steer(
    flow: &FlowField,
    me: &Walker,
    desired: Vec2,
    speed: f32,
    crowd: &[Walker],
    detour: Option<&Detour>,
) -> Vec2 {
    if desired == Vec2::ZERO {
        return Vec2::ZERO;
    }
    // How fast it means to go, as a share of its speed, and which way.
    let pace = desired.length().min(1.0);
    let desired = desired.normalize();
    let speed = speed * pace;
    let reach = (speed * LOOK_AHEAD_SECONDS).clamp(LOOK_AHEAD.0, LOOK_AHEAD.1);
    let heading = me.velocity.normalize_or_zero();
    let mut score = [0.0f32; DIRECTIONS];
    let mut danger = [0.0f32; DIRECTIONS];
    let ways: [Vec2; DIRECTIONS] =
        std::array::from_fn(|i| Vec2::from_angle(i as f32 / DIRECTIONS as f32 * std::f32::consts::TAU));
    for (i, way) in ways.iter().enumerate() {
        // Most for straight on, some for sideways (the way round something in front), none for
        // straight back.
        let lean = 0.5 + 0.5 * way.dot(desired);
        let mut interest = lean * lean + MOMENTUM * way.dot(heading).max(0.0);
        if let Some(detour) = detour {
            if way.dot(desired) > 0.7 {
                danger[i] = danger[i].max(DETOUR_SHUN);
            }
            interest += DETOUR_FAVOUR * way.dot(desired.perp() * detour.turn).max(0.0);
        }
        score[i] = interest;
        danger[i] = danger[i].max(1.0 - flow.free_along(me.at, *way, reach, me.size));
    }
    for other in crowd {
        if other.entity == me.entity {
            continue;
        }
        let offset = other.at - me.at;
        let distance = offset.length();
        // The gap between the two bodies, box to box, as the physics sees them.
        let gap = (offset.abs() - (me.size + other.size) / 2.0).max_element();
        if gap > ELBOW_ROOM + CROWD_REACH || distance < 0.001 {
            continue;
        }
        let toward = offset / distance;
        // Only one in front, on the way it wants to go, and not already going that way itself.
        if toward.dot(desired) < AHEAD || other.velocity.dot(desired) > speed * SAME_WAY {
            continue;
        }
        let closeness = (1.0 - (gap - ELBOW_ROOM) / CROWD_REACH).clamp(0.0, 1.0);
        for (i, way) in ways.iter().enumerate() {
            let facing = (way.dot(toward) - 0.5) * 2.0;
            if facing > 0.0 {
                danger[i] = danger[i].max(closeness * facing);
            }
        }
    }
    // Nothing in the way where it wants to go: it goes exactly there.
    let wanted = (desired.to_angle() / std::f32::consts::TAU * DIRECTIONS as f32).rem_euclid(DIRECTIONS as f32);
    let near = [wanted.floor() as usize % DIRECTIONS, wanted.ceil() as usize % DIRECTIONS];
    if detour.is_none() && near.iter().all(|&i| danger[i] < 0.05) {
        return desired * speed;
    }
    let mut best = None;
    let mut best_value = 0.0;
    for i in 0..DIRECTIONS {
        let value = score[i] * (1.0 - danger[i]);
        if value > best_value {
            best_value = value;
            best = Some(i);
        }
    }
    let Some(best) = best else {
        // Hemmed in every way that leads anywhere: follow the way with the least in front.
        return desired * speed * 0.3;
    };
    // The peak between the best way and its neighbours, so the turn isn't in sixteenths.
    let value = |i: usize| score[i] * (1.0 - danger[i]);
    let before = value((best + DIRECTIONS - 1) % DIRECTIONS);
    let after = value((best + 1) % DIRECTIONS);
    let curve = before - 2.0 * best_value + after;
    let shift = if curve < -1e-6 { (0.5 * (before - after) / curve).clamp(-0.5, 0.5) } else { 0.0 };
    let way = Vec2::from_angle((best as f32 + shift) / DIRECTIONS as f32 * std::f32::consts::TAU);
    // Slow down with something close ahead rather than shove into it.
    way * speed * (1.0 - danger[best] * 0.6)
}

/// Where the place `slot` round the player is for an enemy of `size`: just inside touching.
fn slot_point(player: Vec2, player_size: Vec2, size: Vec2, slot: usize) -> Vec2 {
    let way = Vec2::from_angle(slot as f32 / SLOTS as f32 * std::f32::consts::TAU);
    player + way * (player_size + size) / 2.0 * SLOT_DEPTH
}

/// Hands out the places round the player: those already holding one keep it until they fall
/// back; the rest coming close, nearest first, take the free place nearest the side they come
/// from where their body fits beside those already there.
fn assign_slots(world: &mut World, player: Vec2, player_size: Vec2) {
    let mut closing: Vec<(Entity, Vec2, Vec2, Option<usize>)> = world
        .query::<(Entity, &Position, &Size, Option<&Slot>)>()
        .with::<&Chase>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .without::<&Smash>()
        .iter()
        .map(|(entity, position, size, slot)| (entity, position.0, size.0, slot.map(|slot| slot.0)))
        .collect();
    let reach = |at: Vec2, size: Vec2| at.distance(player) - size.max_element() / 2.0;
    let mut taken: Vec<(Vec2, Vec2)> = Vec::new();
    let fits = |taken: &[(Vec2, Vec2)], at: Vec2, size: Vec2| {
        taken
            .iter()
            .all(|(other, other_size)| !crate::geometry::overlaps(at, size * SLOT_ROOM, *other, *other_size * SLOT_ROOM))
    };
    let mut released = Vec::new();
    for (entity, at, size, slot) in &mut closing {
        if let Some(index) = *slot {
            let place = slot_point(player, player_size, *size, index);
            if reach(*at, *size) > RELEASE || !fits(&taken, place, *size) {
                released.push(*entity);
                *slot = None;
            } else {
                taken.push((place, *size));
            }
        }
    }
    for entity in released {
        let _ = world.remove_one::<Slot>(entity);
    }
    closing.retain(|(_, at, size, slot)| slot.is_none() && reach(*at, *size) < ENGAGE);
    closing.sort_by(|a, b| a.1.distance(player).total_cmp(&b.1.distance(player)));
    for (entity, at, size, _) in closing {
        let side = (at - player).to_angle() / std::f32::consts::TAU * SLOTS as f32;
        let off = |index: usize| {
            let d = (index as f32 - side).rem_euclid(SLOTS as f32);
            d.min(SLOTS as f32 - d)
        };
        let free = (0..SLOTS)
            .filter(|&index| fits(&taken, slot_point(player, player_size, size, index), size))
            .min_by(|&a, &b| off(a).total_cmp(&off(b)));
        if let Some(index) = free {
            taken.push((slot_point(player, player_size, size, index), size));
            let _ = world.insert_one(entity, Slot(index));
        }
    }
}

pub(crate) fn chase_player(world: &mut World, flow: &FlowField, dt: f32) {
    let ease = 1.0 - (-TURN_RATE * dt).exp();
    let player = player_placement(world);
    let target = player.map(|(position, _)| position);
    match player {
        Some((at, body)) => assign_slots(world, at, body),
        None => {
            let holders: Vec<Entity> = world.query::<Entity>().with::<&Slot>().iter().collect();
            for entity in holders {
                let _ = world.remove_one::<Slot>(entity);
            }
        }
    }
    let crowd: Vec<Walker> = world
        .query::<(Entity, &Position, &Size, &Velocity)>()
        .with::<&Chase>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .map(|(entity, position, size, velocity)| Walker {
            entity,
            at: position.0,
            size: size.0,
            velocity: velocity.0,
        })
        .collect();
    for (entity, position, size, chase, velocity, rooted, shooter, detour, frozen, charge, slot) in world
        .query_mut::<(
            Entity,
            &Position,
            &Size,
            &Chase,
            &mut Velocity,
            Option<&Rooted>,
            Option<(&Smash, Option<&Boss>)>,
            Option<&Detour>,
            Option<&Frozen>,
            Option<&Charge>,
            Option<&Slot>,
        )>()
        .without::<&Expiring>()
        .without::<&Spawning>()
    {
        if rooted.is_some() || frozen.is_some() {
            velocity.0 = Vec2::ZERO;
            continue;
        }
        if let Some(charge) = charge {
            velocity.0 = charge.direction * chase.speed;
            continue;
        }
        let (Some(target), Some((_, body))) = (target, player) else {
            velocity.0 = Vec2::ZERO;
            continue;
        };
        let distance = position.0.distance(target);
        let reach = distance - size.0.max_element() / 2.0;
        let desired = if shooter.is_some_and(|(_, boss)| boss.is_none()) && distance < SHOOTER_FAR {
            // A shooter in range holds its ground, backing off when pressed.
            if distance < SHOOTER_NEAR {
                (position.0 - target).normalize_or_zero()
            } else {
                Vec2::ZERO
            }
        } else if let Some(slot) = slot {
            // Its own place at the player: there it stands, hurting them.
            let place = slot_point(target, body, size.0, slot.0);
            let touching = crate::geometry::overlaps(position.0, size.0 * REACHED, target, body * REACHED);
            if (touching && position.0.distance(place) < AT_SLOT * 4.0) || position.0.distance(place) < AT_SLOT {
                Vec2::ZERO
            } else {
                flow.heading(position.0, place, size.0)
            }
        } else if reach < WAITING_ROOM {
            // Every place taken: it prowls round at a distance, waiting its turn.
            let out = (position.0 - target).normalize_or_zero();
            let side = if entity.id() % 2 == 0 { out.perp() } else { -out.perp() };
            let hold = (WAITING_ROOM * 0.8 - reach) / WAITING_ROOM;
            (side + out * hold * 2.0).normalize_or_zero() * PROWL
        } else {
            flow.heading(position.0, target, size.0)
        };
        let me = Walker {
            entity,
            at: position.0,
            size: size.0,
            velocity: velocity.0,
        };
        let steered = steer(flow, &me, desired, chase.speed, &crowd, detour);
        velocity.0 = velocity.0.lerp(steered, ease);
        if velocity.0.length() < STANDING {
            velocity.0 = Vec2::ZERO;
        }
    }
}

pub(crate) fn face_player(world: &mut World) {
    let Some((target, _)) = player_placement(world) else {
        return;
    };
    for (position, animation) in world
        .query_mut::<(&Position, &mut Animation)>()
        .with::<&Enemy>()
        .without::<&Expiring>()
        .without::<&Charge>()
    {
        animation.face(target.x < position.0.x);
    }
}

pub(crate) fn tick_spawning(world: &mut World, physics: &mut Physics, dt: f32) {
    let mut finished = Vec::new();
    for (entity, spawning, color, puddle, animation, enemy) in world.query_mut::<(
        Entity,
        &mut Spawning,
        &mut Color,
        Option<&Puddle>,
        Option<&mut Animation>,
        Option<&Enemy>,
    )>() {
        let before = spawning.remaining;
        spawning.remaining -= dt;
        if puddle.is_some() {
            let reform = enemy.and_then(|enemy| reform_clip(enemy.kind));
            if let (Some(reform), Some(animation)) = (reform, animation) {
                let lead = reform.frames as f32 * reform.frame_time;
                if before > lead && spawning.remaining <= lead {
                    let facing = animation.flipped();
                    animation.play(Clip {
                        flipped: facing,
                        ..reform
                    });
                }
            }
        } else {
            color.0.w = (1.0 - spawning.remaining / spawning.total).clamp(0.0, 1.0);
        }
        if spawning.remaining <= 0.0 {
            finished.push(entity);
        }
    }
    for entity in finished {
        crate::prefabs::finish_spawning(world, physics, entity);
        if world.remove_one::<Puddle>(entity).is_ok() {
            stand_up(world, entity);
        }
    }
}

/// Seconds a puddle lies on the floor before it is a slime again.
pub(crate) const REVIVE_SECONDS: f32 = 10.0;
const REFORM_FRAME_TIME: f32 = 0.08;
const REFORM_FRAMES: usize = 6;

fn reform_clip(kind: crate::enemies::EnemyKind) -> Option<Clip> {
    kind.spec().reform_row.map(|row| Clip {
        row,
        frames: REFORM_FRAMES,
        frame_time: REFORM_FRAME_TIME,
        looping: false,
        flipped: false,
    })
}

fn stand_up(world: &mut World, entity: Entity) {
    let Ok(kind) = world.get::<&Enemy>(entity).map(|enemy| enemy.kind) else {
        return;
    };
    if let Ok(mut health) = world.get::<&mut Health>(entity) {
        health.current = health.max;
    }
    if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
        let facing = animation.flipped();
        animation.play(Clip {
            flipped: facing,
            ..kind.spec().idle_clip()
        });
    }
}

/// The first death of an enemy that revives: it melts into a puddle that cannot be hit and
/// lies still until it re-forms, the second death is for good.
fn collapse_into_puddles(world: &mut World, physics: &mut Physics) {
    let melting: Vec<(Entity, Enemy)> = world
        .query::<(Entity, &Health, &Enemy)>()
        .with::<&Revives>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .filter(|(_, health, _)| health.current <= 0)
        .map(|(entity, _, enemy)| (entity, *enemy))
        .collect();
    for (entity, enemy) in melting {
        physics.detach(world, entity);
        let _ = world.remove_one::<Revives>(entity);
        let _ = world.remove_one::<Frozen>(entity);
        let _ = world.insert(
            entity,
            (
                Puddle,
                Spawning {
                    remaining: REVIVE_SECONDS,
                    total: REVIVE_SECONDS,
                },
            ),
        );
        if let Ok(mut velocity) = world.get::<&mut Velocity>(entity) {
            velocity.0 = Vec2::ZERO;
        }
        if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
            let facing = animation.flipped();
            animation.play(Clip {
                flipped: facing,
                ..enemy.death
            });
        }
    }
}

pub(crate) fn dash(world: &mut World, sounds: &mut Vec<AudioCommand>, dt: f32) {
    let mut trail = Vec::new();
    for (entity, position, dash, chase) in world
        .query_mut::<(Entity, &Position, &mut Dash, &mut Chase)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
    {
        let before = dash.cycle;
        dash.cycle = (dash.cycle + dt) % DASH_PERIOD;
        let dashing = dash.cycle < DASH_DURATION;
        if dashing && (before == 0.0 || before >= DASH_DURATION || dash.cycle < before) {
            sounds.push(AudioCommand::At(SoundEffect::GhostDash, position.0));
        }
        chase.speed = if dashing {
            dash.base_speed + dash.bonus_speed
        } else {
            dash.base_speed
        };
        dash.trail -= dt;
        if dashing && dash.trail <= 0.0 {
            dash.trail = DASH_TRAIL_INTERVAL;
            trail.push(entity);
        }
    }
    for entity in trail {
        let copy = world
            .query_one_mut::<(&Position, &Size, &Sprite, Option<&Layer>)>(entity)
            .map(|(position, size, sprite, layer)| {
                (
                    *position,
                    *size,
                    *sprite,
                    layer.copied().unwrap_or(Layer(0)),
                )
            });
        if let Ok((position, size, sprite, layer)) = copy {
            super::spawn_afterimage(world, position, size, sprite, layer);
        }
    }
}

pub(crate) fn flash_hits(world: &mut World, dt: f32) {
    let mut fading = Vec::new();
    let mut done = Vec::new();
    for (entity, flash) in world.query_mut::<(Entity, &mut HitFlash)>() {
        flash.remaining -= dt;
        if flash.remaining <= 0.0 {
            done.push(entity);
        } else {
            let strength = (flash.remaining / FLASH_SECONDS).clamp(0.0, 1.0);
            fading.push((entity, strength));
        }
    }
    for (entity, strength) in fading {
        // The player's outline is a brighter red, to stand out against their own dark one.
        let red = if world.get::<&Player>(entity).is_ok() { PLAYER_HIT_OUTLINE } else { HIT_OUTLINE };
        let _ = world.insert_one(entity, Outline(red * Vec4::new(1.0, 1.0, 1.0, strength)));
    }
    for entity in done {
        let _ = world.remove_one::<HitFlash>(entity);
        let _ = world.remove_one::<Outline>(entity);
    }
}

/// The one place where enemies die: every death effect sees each kill before it is reaped,
/// whichever system dealt the killing blow.
pub(crate) fn resolve_deaths(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
) {
    collapse_into_puddles(world, physics);
    super::skills::death_bursts(world, physics, textures, sounds);
    super::skills::death_clouds(world, textures, sounds);
    super::skills::death_heals(world, sounds);
    super::frost::death_splits(world, physics, textures);
    super::lava::death_ashes(world, physics, textures);
    super::bosses::clean_up_after_boss(world);
    reap_dead(world, physics);
}

pub(super) fn reap_dead(world: &mut World, physics: &mut Physics) {
    let dead: Vec<(Entity, Enemy)> = world
        .query::<(Entity, &Health, &Enemy)>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .filter(|(_, health, _)| health.current <= 0)
        .map(|(entity, _, enemy)| (entity, *enemy))
        .collect();
    for (entity, enemy) in dead {
        physics.detach(world, entity);
        let _ = world.insert_one(entity, Expiring);
        let _ = world.remove_one::<Frozen>(entity);
        if let Ok(mut velocity) = world.get::<&mut Velocity>(entity) {
            velocity.0 = Vec2::ZERO;
        }
        if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
            let facing = animation.flipped();
            animation.play(Clip {
                flipped: facing,
                ..enemy.death
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec4;

    use crate::assets::Textures;
    use crate::components::Body;
    use crate::enemies::{COMMON_SPAWN_SECONDS, EnemyKind};
    use crate::navigation::Navigation;
    use crate::prefabs;

    use super::*;

    fn arena() -> (World, Physics) {
        let mut world = World::new();
        let mut physics = Physics::new();
        prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(0.0, 0.0),
            &Textures::default(),
        );
        (world, physics)
    }

    #[test]
    fn enemies_lose_track_of_a_cloaked_player_and_stand_still() {
        let (mut world, mut physics) = arena();
        let player = world
            .query::<Entity>()
            .with::<&Player>()
            .iter()
            .next()
            .expect("player");
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );
        let _ = world.insert_one(player, crate::components::Phantom { remaining: 5.0 });

        chase_player(&mut world, &FlowField::default(), 1.0);

        assert!(player_placement(&world).is_none());
        assert!(crate::systems::golem::player_at(&world).is_none());
        assert!(crate::systems::golem::player_body(&world).is_some());
        let velocity = world.get::<&Velocity>(rat).map(|v| v.0).ok();
        assert_eq!(velocity, Some(Vec2::ZERO));
    }

    #[test]
    fn enemy_runs_towards_the_player() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );

        chase_player(&mut world, &FlowField::default(), 1.0);

        let velocity = world.get::<&Velocity>(rat).map(|v| v.0).ok();
        assert!(
            velocity.is_some_and(|v| v.x < 0.0 && v.y.abs() < 0.001),
            "{velocity:?}"
        );
    }

    #[test]
    fn enemy_without_health_dies_and_loses_its_body() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );
        if let Ok(mut health) = world.get::<&mut Health>(rat) {
            health.current = 0;
        }

        reap_dead(&mut world, &mut physics);

        assert!(world.get::<&Expiring>(rat).is_ok());
        assert!(world.get::<&Body>(rat).is_err());
    }

    #[test]
    fn living_enemy_is_left_alone() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );

        reap_dead(&mut world, &mut physics);

        assert!(world.get::<&Expiring>(rat).is_err());
        assert!(world.get::<&Body>(rat).is_ok());
    }

    #[test]
    fn a_spawning_enemy_is_harmless_until_the_spawn_ends() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::enemy(
            &mut world,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
            0,
            &Textures::default(),
        );

        chase_player(&mut world, &FlowField::default(), 1.0);
        let velocity = world.get::<&Velocity>(rat).map(|v| v.0).ok();
        assert_eq!(velocity, Some(Vec2::ZERO));
        assert!(world.get::<&Body>(rat).is_err());

        tick_spawning(&mut world, &mut physics, COMMON_SPAWN_SECONDS / 2.0);
        let alpha = world.get::<&Color>(rat).map(|c| c.0.w).unwrap_or(-1.0);
        assert!((alpha - 0.5).abs() < 0.01, "{alpha}");
        assert!(world.get::<&Spawning>(rat).is_ok());

        tick_spawning(&mut world, &mut physics, COMMON_SPAWN_SECONDS);
        assert!(world.get::<&Spawning>(rat).is_err());
        assert!(world.get::<&Body>(rat).is_ok());
        assert_eq!(world.get::<&Color>(rat).map(|c| c.0).ok(), Some(Vec4::ONE));
    }

    #[test]
    fn ghosts_dash_for_a_second_out_of_every_three() {
        let (mut world, mut physics) = arena();
        let ghost = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Ghost,
            Vec2::new(300.0, 0.0),
        );
        let base = EnemyKind::Ghost.spec().speed();
        let speed = |world: &World| world.get::<&Chase>(ghost).map(|c| c.speed).unwrap_or(0.0);

        dash(&mut world, &mut Vec::new(), 0.5);
        assert!(speed(&world) > base * 3.0);
        dash(&mut world, &mut Vec::new(), 1.0);
        assert!((speed(&world) - base).abs() < 0.01);
        dash(&mut world, &mut Vec::new(), 1.6);
        assert!(speed(&world) > base * 3.0);
    }

    #[test]
    fn a_ghost_whooshes_once_at_the_start_of_every_dash() {
        let (mut world, mut physics) = arena();
        prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Ghost,
            Vec2::new(300.0, 0.0),
        );
        let mut sounds = Vec::new();

        for _ in 0..5 * 60 {
            dash(&mut world, &mut sounds, 1.0 / 60.0);
        }

        assert_eq!(sounds.len(), 2);
        assert!(matches!(
            sounds[0],
            AudioCommand::At(SoundEffect::GhostDash, _)
        ));
    }

    #[test]
    fn a_hit_outlines_the_enemy_in_red_and_fades_back() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );
        let _ = world.insert_one(
            rat,
            HitFlash {
                remaining: FLASH_SECONDS,
            },
        );

        flash_hits(&mut world, 0.01);
        let outline = world
            .get::<&Outline>(rat)
            .map(|o| o.0)
            .unwrap_or(Vec4::ZERO);
        assert!(
            outline.x == 1.0 && outline.y == 0.0 && outline.w > 0.9,
            "{outline:?}"
        );
        assert_eq!(world.get::<&Color>(rat).map(|c| c.0).ok(), Some(Vec4::ONE));

        flash_hits(&mut world, FLASH_SECONDS);
        assert!(world.get::<&HitFlash>(rat).is_err());
        assert!(world.get::<&Outline>(rat).is_err());
    }

    #[test]
    fn an_enemy_steers_around_a_wall_instead_of_into_it() {
        let mut world = World::new();
        let mut physics = Physics::new();
        prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(600.0, 160.0),
            &Textures::default(),
        );
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(100.0, 160.0),
        );
        let wall: Vec<(usize, usize)> = (0..5).map(|row| (4, row)).collect();
        let navigation = Navigation::new(&wall);
        let mut flow = FlowField::default();

        update_flow(&mut flow, &world, &physics, &navigation, 0.016);
        chase_player(&mut world, &flow, 1.0);

        let velocity = world.get::<&Velocity>(rat).map(|v| v.0).unwrap_or_default();
        assert!(velocity.x > 0.0 && velocity.y > 0.0, "{velocity:?}");
    }

    #[test]
    fn a_slime_closes_in_to_shooting_range_then_holds_and_backs_off_when_pressed() {
        let (mut world, mut physics) = arena();
        let slime = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Slime,
            Vec2::new(600.0, 0.0),
        );
        let flow = FlowField::default();
        let velocity = |world: &World| {
            world
                .get::<&Velocity>(slime)
                .map(|v| v.0)
                .unwrap_or_default()
        };
        let place = |world: &mut World, x: f32| {
            if let Ok(mut position) = world.get::<&mut Position>(slime) {
                position.0 = Vec2::new(x, 0.0);
            }
        };

        chase_player(&mut world, &flow, 1.0);
        assert!(velocity(&world).x < 0.0);

        place(&mut world, SHOOTER_NEAR + 40.0);
        chase_player(&mut world, &flow, 1.0);
        assert_eq!(velocity(&world), Vec2::ZERO);

        place(&mut world, SHOOTER_NEAR - 40.0);
        chase_player(&mut world, &flow, 1.0);
        assert!(velocity(&world).x > 0.0);
    }

    #[test]
    fn an_enemy_on_a_detour_turns_away_from_its_heading() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );
        let _ = world.insert_one(
            rat,
            Detour {
                remaining: 0.5,
                turn: 1.0,
            },
        );

        chase_player(&mut world, &FlowField::default(), 1.0);

        let velocity = world.get::<&Velocity>(rat).map(|v| v.0).unwrap_or_default();
        assert!(velocity.y.abs() > velocity.x.abs() * 0.5, "{velocity:?}");
        assert!(velocity.x < 0.0, "{velocity:?}");
    }

    #[test]
    fn enemies_closing_in_take_places_of_their_own_round_the_player() {
        let (mut world, mut physics) = arena();
        let first = prefabs::ready_enemy(&mut world, &mut physics, EnemyKind::Snake, Vec2::new(150.0, 0.0));
        let second = prefabs::ready_enemy(&mut world, &mut physics, EnemyKind::Snake, Vec2::new(170.0, 20.0));
        chase_player(&mut world, &FlowField::default(), 1.0);
        let slot = |world: &World, entity| world.get::<&Slot>(entity).map(|slot| slot.0).ok();
        assert!(slot(&world, first).is_some() && slot(&world, second).is_some());
        assert_ne!(slot(&world, first), slot(&world, second));
        // Far off, nobody has a place.
        let far = prefabs::ready_enemy(&mut world, &mut physics, EnemyKind::Snake, Vec2::new(900.0, 0.0));
        chase_player(&mut world, &FlowField::default(), 1.0);
        assert!(slot(&world, far).is_none());
    }

    #[test]
    fn far_from_the_player_an_enemy_goes_round_one_standing_in_its_way() {
        let (mut world, mut physics) = arena();
        let standing = prefabs::ready_enemy(&mut world, &mut physics, EnemyKind::Snake, Vec2::new(500.0, 0.0));
        let _ = world.insert_one(standing, Rooted);
        let walker = prefabs::ready_enemy(&mut world, &mut physics, EnemyKind::Snake, Vec2::new(600.0, 0.0));
        let flow = FlowField::default();
        chase_player(&mut world, &flow, 1.0);
        let around = world.get::<&Velocity>(walker).map(|v| v.0).unwrap_or_default();
        assert!(around.x < 0.0 && around.y.abs() > around.x.abs() * 0.3, "{around:?}");
    }

    #[test]
    fn enemies_face_the_player() {
        let (mut world, mut physics) = arena();
        let right = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );
        let left = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(-300.0, 0.0),
        );

        face_player(&mut world);

        let flipped = |entity| world.get::<&Animation>(entity).map(|a| a.flipped());
        assert_eq!(flipped(right).ok(), Some(true));
        assert_eq!(flipped(left).ok(), Some(false));
    }

    #[test]
    fn a_dashing_ghost_leaves_fading_afterimages_like_the_player_and_a_calm_one_does_not() {
        let (mut world, mut physics) = arena();
        let ghost = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Ghost,
            Vec2::new(300.0, 0.0),
        );
        let images = |world: &World| {
            world
                .query::<&crate::components::Afterimage>()
                .iter()
                .count()
        };

        for _ in 0..30 {
            dash(&mut world, &mut Vec::new(), 1.0 / 60.0);
        }
        let during = images(&world);
        assert!(during >= 8, "{during}");
        let copy = world
            .query::<(&crate::components::Afterimage, &Position, &Sprite)>()
            .iter()
            .next()
            .map(|(_, position, _)| position.0);
        assert_eq!(copy, world.get::<&Position>(ghost).map(|p| p.0).ok());

        for _ in 0..30 {
            dash(&mut world, &mut Vec::new(), 1.0 / 60.0);
        }
        let after_burst = images(&world);
        for _ in 0..60 {
            dash(&mut world, &mut Vec::new(), 1.0 / 60.0);
        }
        assert_eq!(images(&world), after_burst);

        for _ in 0..40 {
            crate::systems::fade_afterimages(&mut world, 1.0 / 60.0);
        }
        assert_eq!(images(&world), 0);
    }

    #[test]
    fn other_enemies_do_not_leave_afterimages() {
        let (mut world, mut physics) = arena();
        prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );

        for _ in 0..60 {
            dash(&mut world, &mut Vec::new(), 1.0 / 60.0);
        }

        assert_eq!(
            world
                .query::<&crate::components::Afterimage>()
                .iter()
                .count(),
            0
        );
    }

    #[test]
    fn a_killed_bloated_spider_bursts_into_a_poison_cloud_exactly_once() {
        let (mut world, mut physics) = arena();
        let spider = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::BloatedSpider,
            Vec2::new(400.0, 400.0),
        );
        resolve_deaths(&mut world, &mut physics, &Textures::default(), &mut Vec::new());
        assert_eq!(
            world
                .query::<&crate::components::PoisonPool>()
                .iter()
                .count(),
            0
        );

        if let Ok(mut health) = world.get::<&mut crate::components::Health>(spider) {
            health.current = 0;
        }
        resolve_deaths(&mut world, &mut physics, &Textures::default(), &mut Vec::new());
        resolve_deaths(&mut world, &mut physics, &Textures::default(), &mut Vec::new());

        assert_eq!(
            world
                .query::<&crate::components::PoisonPool>()
                .iter()
                .count(),
            1
        );
    }

    fn kill(world: &mut World, physics: &mut Physics, entity: Entity) {
        if let Ok(mut health) = world.get::<&mut Health>(entity) {
            health.current = 0;
        }
        resolve_deaths(world, physics, &Textures::default(), &mut Vec::new());
    }

    fn wait(world: &mut World, physics: &mut Physics, seconds: f32) {
        let steps = (seconds * 60.0).ceil() as usize;
        for _ in 0..steps {
            tick_spawning(world, physics, 1.0 / 60.0);
        }
    }

    #[test]
    fn a_saliva_slime_melts_into_a_puddle_and_re_forms_with_full_health() {
        let (mut world, mut physics) = arena();
        let slime = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::SalivaSlime,
            Vec2::new(300.0, 0.0),
        );

        kill(&mut world, &mut physics, slime);

        assert!(world.get::<&Puddle>(slime).is_ok());
        assert!(world.get::<&Expiring>(slime).is_err());
        assert!(world.get::<&Body>(slime).is_err());
        wait(&mut world, &mut physics, REVIVE_SECONDS - 0.4);
        assert!(world.get::<&Puddle>(slime).is_ok());
        assert_eq!(
            world.get::<&Animation>(slime).map(|a| a.row()).ok(),
            Some(3),
            "the puddle pulls itself together before it stands up"
        );
        wait(&mut world, &mut physics, 0.5);
        assert!(world.get::<&Puddle>(slime).is_err());
        assert!(world.get::<&Body>(slime).is_ok());
        let health = world.get::<&Health>(slime).map(|h| (h.current, h.max)).ok();
        assert!(health.is_some_and(|(current, max)| current == max && max > 0));
    }

    #[test]
    fn the_second_death_of_a_saliva_slime_is_for_good() {
        let (mut world, mut physics) = arena();
        let slime = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::SalivaSlime,
            Vec2::new(300.0, 0.0),
        );
        kill(&mut world, &mut physics, slime);
        wait(&mut world, &mut physics, REVIVE_SECONDS + 0.1);

        kill(&mut world, &mut physics, slime);

        assert!(world.get::<&Expiring>(slime).is_ok());
        assert!(world.get::<&Puddle>(slime).is_err());
    }

    #[test]
    fn a_rat_does_not_revive() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );
        kill(&mut world, &mut physics, rat);
        assert!(world.get::<&Expiring>(rat).is_ok());
        assert!(world.get::<&Puddle>(rat).is_err());
    }

    #[test]
    fn a_fallen_boss_takes_the_puddles_of_its_slimes_with_it() {
        let (mut world, mut physics) = arena();
        let slime = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::SalivaSlime,
            Vec2::new(300.0, 0.0),
        );
        let _ = world.insert_one(slime, crate::components::Minion);
        kill(&mut world, &mut physics, slime);
        assert!(world.get::<&Puddle>(slime).is_ok());
        let boss = world.spawn((
            crate::components::Boss,
            Health {
                current: 0,
                max: 10,
            },
        ));

        resolve_deaths(&mut world, &mut physics, &Textures::default(), &mut Vec::new());

        assert!(world.get::<&Expiring>(slime).is_ok());
        let _ = world.despawn(boss);
    }
}
