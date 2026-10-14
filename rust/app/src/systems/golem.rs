use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Tex, Textures, tex};
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Animation, BeamPart, Chase, Clip, Color, ContactDamage, Despawn, Dodge, Drag, Expiring,
    FallingRock, Frozen, GolemAttack, GolemBoss, GolemStage, Health, Layer, Light, Minion, Player,
    Position, RockFall, RoomId, Rooted, Size, Spawning, Sprite, Velocity,
};
use crate::enemies::{self, EnemyKind, ProjectileKind};
use crate::geometry;
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::prefabs;
use crate::systems::damage::hurt_player;

const ROCK_RING_ORIGIN_DROP: f32 = 60.0;
const EYE_HEIGHT: f32 = 45.0;
const FALL_HEIGHT: f32 = 380.0;
const SHADOW_SIZE: Vec2 = Vec2::new(150.0, 75.0);
const ROCK_SIZE: Vec2 = Vec2::new(72.0, 72.0);
const IMPACT_SIZE: Vec2 = Vec2::new(150.0, 150.0);
const BEAM_STEP: f32 = 12.0;
const BEAM_GLOW: f32 = 34.0;
const BEAM_CORE: f32 = 12.0;
const SPARK_SIZE: f32 = 14.0;
const SPARK_FRAMES: usize = 4;
const RAGE_TINT: Vec4 = Vec4::new(1.0, 0.62, 0.55, 1.0);

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Cmd {
    Play(Clip),
    Root,
    Free,
    Ring(f32),
    Boulder,
    Rain(u32),
    AimBeam,
}

pub(crate) fn slam_clip() -> Clip {
    clip(6, 10, 0.09, false)
}

pub(crate) fn throw_clip() -> Clip {
    clip(4, 7, 0.09, false)
}

pub(crate) fn rain_clip() -> Clip {
    clip(1, 8, 0.09, true)
}

pub(crate) fn laser_clip() -> Clip {
    clip(5, 7, 0.14, false)
}

fn clip(row: usize, frames: usize, frame_time: f32, looping: bool) -> Clip {
    Clip {
        row,
        frames,
        frame_time,
        looping,
        flipped: false,
    }
}

fn next(seed: &mut u32) -> u32 {
    let mut x = (*seed).max(1);
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *seed = x;
    x
}

pub(crate) fn unit(seed: &mut u32) -> f32 {
    (next(seed) >> 8) as f32 / (1u32 << 24) as f32
}

const ATTACK_VAR: &str = "BANE_GOLEM_ATTACK";

fn forced_attack() -> Option<GolemAttack> {
    match std::env::var(ATTACK_VAR)
        .ok()?
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "slam" => Some(GolemAttack::Slam),
        "throw" => Some(GolemAttack::Throw),
        "rain" => Some(GolemAttack::Rain),
        "laser" => Some(GolemAttack::Laser),
        _ => None,
    }
}

pub(crate) fn choose_attack(golem: &mut GolemBoss, distance: f32) -> GolemAttack {
    if let Some(attack) = forced_attack() {
        return attack;
    }
    let close = distance < enemies::GOLEM_NEAR;
    let weights = [
        (GolemAttack::Slam, if close { 4.0 } else { 1.0 }),
        (GolemAttack::Throw, if close { 1.0 } else { 3.0 }),
        (GolemAttack::Rain, 2.0),
        (GolemAttack::Laser, if close { 1.5 } else { 3.0 }),
    ];
    let total: f32 = weights
        .iter()
        .filter(|(attack, _)| Some(*attack) != golem.last)
        .map(|(_, weight)| weight)
        .sum();
    let mut roll = unit(&mut golem.seed) * total;
    for (attack, weight) in weights {
        if Some(attack) == golem.last {
            continue;
        }
        if roll < weight {
            return attack;
        }
        roll -= weight;
    }
    GolemAttack::Slam
}

pub(crate) fn step(golem: &mut GolemBoss, distance: f32, dt: f32) -> Vec<Cmd> {
    let mut commands = Vec::new();
    if golem.second_ring > 0.0 {
        golem.second_ring -= dt;
        if golem.second_ring <= 0.0 {
            golem.second_ring = 0.0;
            commands.push(Cmd::Ring(
                std::f32::consts::PI / enemies::GOLEM_ROCK_RING as f32,
            ));
        }
    }
    match golem.stage {
        GolemStage::Walk => {
            golem.cooldown -= dt;
            if golem.cooldown <= 0.0 {
                let attack = choose_attack(golem, distance);
                golem.attack = attack;
                golem.last = Some(attack);
                golem.stage = GolemStage::Winding;
                commands.push(Cmd::Root);
                match attack {
                    GolemAttack::Slam => {
                        golem.timer = enemies::GOLEM_SLAM_WINDUP;
                        commands.push(Cmd::Play(slam_clip()));
                    }
                    GolemAttack::Throw => {
                        golem.timer = enemies::GOLEM_THROW_WINDUP;
                        commands.push(Cmd::Play(throw_clip()));
                    }
                    GolemAttack::Rain => {
                        golem.timer = enemies::GOLEM_RAIN_WINDUP;
                        commands.push(Cmd::Play(rain_clip()));
                    }
                    GolemAttack::Laser => {
                        golem.timer = enemies::GOLEM_LASER_CHARGE;
                        commands.push(Cmd::Play(laser_clip()));
                        commands.push(Cmd::AimBeam);
                    }
                }
            }
        }
        GolemStage::Winding => {
            golem.timer -= dt;
            if golem.timer <= 0.0 {
                match golem.attack {
                    GolemAttack::Slam => {
                        commands.push(Cmd::Ring(0.0));
                        if golem.enraged {
                            golem.second_ring = enemies::GOLEM_SECOND_RING;
                        }
                        golem.stage = GolemStage::Recovering;
                        golem.timer = enemies::GOLEM_SLAM_RECOVER;
                    }
                    GolemAttack::Throw => {
                        commands.push(Cmd::Boulder);
                        golem.stage = GolemStage::Recovering;
                        golem.timer = enemies::GOLEM_THROW_RECOVER;
                    }
                    GolemAttack::Rain => {
                        golem.stage = GolemStage::Acting;
                        golem.timer = enemies::GOLEM_RAIN_SECONDS;
                        golem.pulse = 0.0;
                        golem.rain_dropped = 0;
                    }
                    GolemAttack::Laser => {
                        golem.stage = GolemStage::Acting;
                        golem.timer = enemies::GOLEM_LASER_SECONDS;
                    }
                }
            }
        }
        GolemStage::Acting => {
            golem.timer -= dt;
            if golem.attack == GolemAttack::Rain {
                golem.pulse -= dt;
                while golem.pulse <= 0.0 && golem.timer > 0.0 {
                    commands.push(Cmd::Rain(golem.rain_dropped));
                    golem.rain_dropped += 1;
                    golem.pulse += enemies::GOLEM_RAIN_EVERY;
                }
            }
            if golem.timer <= 0.0 {
                golem.stage = GolemStage::Recovering;
                golem.timer = match golem.attack {
                    GolemAttack::Laser => enemies::GOLEM_LASER_RECOVER,
                    _ => enemies::GOLEM_RAIN_RECOVER,
                };
            }
        }
        GolemStage::Recovering => {
            golem.timer -= dt;
            if golem.timer <= 0.0 {
                commands.push(Cmd::Play(golem.idle));
                commands.push(Cmd::Free);
                golem.stage = GolemStage::Walk;
                golem.cooldown = if golem.enraged {
                    enemies::GOLEM_RAGE_COOLDOWN
                } else {
                    enemies::GOLEM_COOLDOWN
                };
            }
        }
    }
    commands
}

pub(crate) fn player_at(world: &World) -> Option<(Entity, Vec2, Vec2)> {
    world
        .query::<(Entity, &Position, &Size, &Health)>()
        .with::<&Player>()
        .iter()
        .filter(|(_, _, _, health)| health.current > 0)
        .map(|(entity, position, size, _)| (entity, position.0, size.0))
        .next()
}

pub(crate) fn play(world: &mut World, entity: Entity, clip: Clip) {
    if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
        let facing = animation.flipped();
        animation.play(Clip {
            flipped: facing,
            ..clip
        });
    }
}

pub(crate) fn aim(from: Vec2, to: Vec2) -> f32 {
    let direction = to - from;
    direction.y.atan2(direction.x)
}

pub(crate) fn turn_towards(current: f32, wanted: f32, step: f32) -> f32 {
    let mut difference = wanted - current;
    while difference > std::f32::consts::PI {
        difference -= std::f32::consts::TAU;
    }
    while difference < -std::f32::consts::PI {
        difference += std::f32::consts::TAU;
    }
    current + difference.clamp(-step, step)
}

pub(crate) fn tick_golem(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    let target = player_at(world);
    let golems: Vec<(Entity, Vec2, i32, i32)> = world
        .query::<(Entity, &Position, &Health, &GolemBoss)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .without::<&Frozen>()
        .iter()
        .map(|(entity, position, health, _)| (entity, position.0, health.current, health.max))
        .collect();
    for (entity, at, current, max) in golems {
        if !world
            .get::<&GolemBoss>(entity)
            .is_ok_and(|golem| golem.enraged)
            && current as f32 <= max as f32 * enemies::GOLEM_RAGE_AT
        {
            if let Ok(mut golem) = world.get::<&mut GolemBoss>(entity) {
                golem.enraged = true;
            }
            if let Ok(mut chase) = world.get::<&mut Chase>(entity) {
                chase.speed *= enemies::GOLEM_RAGE_SPEED;
            }
            if let Ok(mut color) = world.get::<&mut Color>(entity) {
                color.0 = RAGE_TINT;
            }
        }
        let distance = target.map_or(f32::MAX, |(_, player, _)| player.distance(at));
        let commands = {
            let Ok(mut golem) = world.get::<&mut GolemBoss>(entity) else {
                continue;
            };
            step(&mut golem, distance, dt)
        };
        for command in commands {
            execute(
                world,
                physics,
                textures,
                sounds,
                entity,
                at,
                target.map(|(_, p, _)| p),
                command,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_rock(
    world: &mut World,
    physics: &mut Physics,
    kind: ProjectileKind,
    origin: Vec2,
    direction: Vec2,
    damage: i32,
    room: Option<RoomId>,
    textures: &Textures,
) {
    let bullet = prefabs::enemy_bullet(world, physics, kind, origin, direction, textures);
    let _ = world.insert_one(bullet, ContactDamage(damage));
    if let Some(room) = room {
        let _ = world.insert_one(bullet, room);
    }
}

#[allow(clippy::too_many_arguments)]
fn execute(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    entity: Entity,
    at: Vec2,
    player: Option<Vec2>,
    command: Cmd,
) {
    let room = world.get::<&RoomId>(entity).map(|room| *room).ok();
    match command {
        Cmd::Play(clip) => play(world, entity, clip),
        Cmd::Root => {
            let _ = world.insert_one(entity, Rooted);
        }
        Cmd::Free => {
            let _ = world.remove_one::<Rooted>(entity);
        }
        Cmd::Ring(offset) => {
            sounds.push(AudioCommand::Effect(SoundEffect::Explosion));
            let origin = at + Vec2::new(0.0, ROCK_RING_ORIGIN_DROP);
            for index in 0..enemies::GOLEM_ROCK_RING {
                let angle =
                    offset + index as f32 / enemies::GOLEM_ROCK_RING as f32 * std::f32::consts::TAU;
                spawn_rock(
                    world,
                    physics,
                    ProjectileKind::GolemRock,
                    origin,
                    Vec2::from_angle(angle),
                    enemies::GOLEM_ROCK_DAMAGE,
                    room,
                    textures,
                );
            }
        }
        Cmd::Boulder => {
            let direction = player.map_or(Vec2::Y, |player| player - at);
            spawn_rock(
                world,
                physics,
                ProjectileKind::Boulder,
                at + Vec2::new(0.0, -20.0),
                direction,
                enemies::GOLEM_BOULDER_DAMAGE,
                room,
                textures,
            );
        }
        Cmd::Rain(index) => {
            let Some(player) = player else {
                return;
            };
            let mut seed =
                (index + 1).wrapping_mul(0x9E37_79B1) ^ (at.x as u32) ^ ((at.y as u32) << 7);
            let spread = if index < 3 {
                40.0
            } else {
                enemies::GOLEM_RAIN_SPREAD
            };
            let jitter = Vec2::new(unit(&mut seed) - 0.5, unit(&mut seed) - 0.5) * 2.0 * spread;
            drop_rock(world, textures, player + jitter, room);
        }
        Cmd::AimBeam => {
            if let Some(player) = player {
                let eye = at + Vec2::new(0.0, -EYE_HEIGHT);
                if let Ok(mut golem) = world.get::<&mut GolemBoss>(entity) {
                    golem.beam_angle = aim(eye, player);
                }
            }
        }
    }
}

pub(crate) fn drop_rock(
    world: &mut World,
    textures: &Textures,
    ground: Vec2,
    room: Option<RoomId>,
) {
    let rock = world.spawn((
        FallingRock,
        Position(ground - Vec2::new(0.0, FALL_HEIGHT)),
        Size(ROCK_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(tex::GOLEM_BOULDER), 0, 0, 4, 2),
        Layer(4),
    ));
    world.spawn((
        RockFall {
            remaining: enemies::GOLEM_RAIN_DELAY,
            rock,
            room,
        },
        Position(ground),
        Size(SHADOW_SIZE),
        Color(Vec4::ONE),
        Sprite::whole(textures.get(tex::ROCK_SHADOW)),
        Layer(-1),
    ));
}

pub(crate) fn tick_rock_falls(
    world: &mut World,
    physics: &mut Physics,
    navigation: &Navigation,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    let mut landed = Vec::new();
    for (entity, fall, position) in world.query_mut::<(Entity, &mut RockFall, &Position)>() {
        fall.remaining -= dt;
        if fall.remaining <= 0.0 {
            landed.push((entity, fall.rock, position.0, fall.room));
        }
    }
    let falling: Vec<(Entity, Entity)> = world
        .query::<(Entity, &RockFall)>()
        .iter()
        .map(|(entity, fall)| (entity, fall.rock))
        .collect();
    for (shadow, rock) in falling {
        let progress = world.get::<&RockFall>(shadow).map_or(0.0, |fall| {
            (fall.remaining / enemies::GOLEM_RAIN_DELAY).clamp(0.0, 1.0)
        });
        let ground = world.get::<&Position>(shadow).map(|p| p.0).ok();
        if let (Some(ground), Ok(mut position)) = (ground, world.get::<&mut Position>(rock)) {
            position.0 =
                ground - Vec2::new(0.0, FALL_HEIGHT * (2.0 * progress - progress * progress));
        }
    }
    let player = player_at(world);
    for (shadow, rock, ground, room) in landed {
        let _ = world.despawn(shadow);
        let _ = world.despawn(rock);
        sounds.push(AudioCommand::Effect(SoundEffect::WallHit));
        let burst = Clip {
            row: 1,
            frames: 4,
            frame_time: 0.07,
            looping: false,
            flipped: false,
        };
        world.spawn((
            Expiring,
            Position(ground),
            Size(IMPACT_SIZE),
            Color(Vec4::ONE),
            Sprite::cell(textures.get(tex::GOLEM_BOULDER), 0, 1, 4, 2),
            Animation::new(4, 2, burst),
            Layer(3),
        ));
        if let Some((player, at, size)) = player {
            let flying = world.get::<&Dodge>(player).is_ok();
            if !flying
                && geometry::overlaps(
                    ground,
                    Vec2::splat(enemies::GOLEM_RAIN_RADIUS * 2.0),
                    at,
                    size,
                )
            {
                hurt_player(world, player, enemies::GOLEM_RAIN_DAMAGE, sounds);
            }
        }
        let mut seed =
            (ground.x as u32).wrapping_mul(2_654_435_761) ^ (ground.y as u32).rotate_left(11);
        let minions = world.query::<&Minion>().iter().count();
        if unit(&mut seed) < enemies::GOLEM_MINION_CHANCE
            && minions < enemies::GOLEM_MINION_LIMIT
            && !navigation.is_blocked(ground, Vec2::splat(60.0))
        {
            prefabs::hatchling(world, physics, EnemyKind::MiniGolem, ground, room, textures);
        }
    }
}

pub(crate) fn beam_end(navigation: &Navigation, origin: Vec2, angle: f32) -> Vec2 {
    let direction = Vec2::from_angle(angle);
    let mut length = 0.0;
    while length < enemies::GOLEM_LASER_REACH {
        let point = origin + direction * length;
        if navigation.is_blocked(point, Vec2::splat(6.0)) {
            return origin + direction * (length - BEAM_STEP).max(0.0);
        }
        length += BEAM_STEP;
    }
    origin + direction * enemies::GOLEM_LASER_REACH
}

fn distance_to_segment(point: Vec2, a: Vec2, b: Vec2) -> f32 {
    let line = b - a;
    let length = line.length_squared();
    if length <= f32::EPSILON {
        return point.distance(a);
    }
    let t = ((point - a).dot(line) / length).clamp(0.0, 1.0);
    point.distance(a + line * t)
}

pub(crate) fn clear_beam(world: &mut World) {
    let parts: Vec<Entity> = world.query::<Entity>().with::<&BeamPart>().iter().collect();
    for part in parts {
        let _ = world.despawn(part);
    }
}

pub(crate) fn spark(world: &mut World, textures: &Textures, sprite: Tex, at: Vec2, velocity: Vec2) {
    let burn = Clip {
        row: 0,
        frames: SPARK_FRAMES,
        frame_time: 0.09,
        looping: false,
        flipped: false,
    };
    world.spawn((
        Expiring,
        Position(at),
        Velocity(velocity),
        Drag { rate: 3.0 },
        Size(Vec2::splat(SPARK_SIZE)),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(sprite), 0, 0, SPARK_FRAMES, 1),
        Animation::new(burn.frames, 1, burn),
        Layer(5),
    ));
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BeamStyle {
    pub(crate) glow: Vec4,
    pub(crate) core: Vec4,
    pub(crate) light: [f32; 3],
    pub(crate) spark: Tex,
    pub(crate) damage: i32,
    pub(crate) wave: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Ray {
    pub(crate) eye: Vec2,
    pub(crate) angle: f32,
    pub(crate) phase: f32,
}

pub(crate) const BLUE_BEAM: BeamStyle = BeamStyle {
    glow: Vec4::new(0.2, 0.5, 1.0, 0.5),
    core: Vec4::new(0.85, 0.96, 1.0, 1.0),
    light: [60.0, 140.0, 255.0],
    spark: tex::SPARK,
    damage: enemies::GOLEM_LASER_DAMAGE,
    wave: 0.0,
};

pub(crate) fn draw_beam(
    world: &mut World,
    navigation: &Navigation,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    ray: Ray,
    style: BeamStyle,
) {
    let Ray { eye, angle, phase } = ray;
    let player = player_at(world);
    let end = beam_end(navigation, eye, angle);
    let direction = Vec2::from_angle(angle);
    let length = eye.distance(end);
    let mut distance = 0.0;
    while distance <= length {
        let at = eye + direction * distance;
        world.spawn((
            BeamPart,
            Position(at),
            Size(Vec2::splat(
                BEAM_GLOW + style.wave * (phase + distance * 0.09).sin(),
            )),
            Color(style.glow),
            Layer(4),
        ));
        world.spawn((
            BeamPart,
            Position(at),
            Size(Vec2::splat(BEAM_CORE)),
            Color(style.core),
            Layer(5),
        ));
        distance += BEAM_STEP;
    }
    let [red, green, blue] = style.light;
    world.spawn((
        BeamPart,
        Position(end),
        Light::rgb(Vec2::ZERO, red, green, blue, 140.0, 1.6),
    ));
    let mut seed = (eye.x as u32).wrapping_mul(2_654_435_761) ^ (end.y as u32) ^ (angle.to_bits());
    for _ in 0..3 {
        let along = unit(&mut seed) * length;
        let side = Vec2::new(-direction.y, direction.x) * (unit(&mut seed) - 0.5) * 2.0;
        spark(
            world,
            textures,
            style.spark,
            eye + direction * along + side * 18.0,
            side * 70.0 + direction * 20.0,
        );
    }
    if length < enemies::GOLEM_LASER_REACH - BEAM_STEP {
        for _ in 0..3 {
            let spread = (unit(&mut seed) - 0.5) * 2.2;
            let away = -direction;
            let back = Vec2::from_angle(away.y.atan2(away.x) + spread);
            spark(
                world,
                textures,
                style.spark,
                end,
                back * (140.0 + unit(&mut seed) * 160.0),
            );
        }
    }
    if let Some((player, player_position, size)) = player {
        let flying = world.get::<&Dodge>(player).is_ok();
        let reach = enemies::GOLEM_LASER_WIDTH / 2.0 + size.x.min(size.y) / 2.0;
        if !flying && distance_to_segment(player_position, eye, end) < reach {
            hurt_player(world, player, style.damage, sounds);
        }
    }
}

pub(crate) fn tick_laser(
    world: &mut World,
    navigation: &Navigation,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    clear_beam(world);
    let player = player_at(world);
    let firing: Vec<(Entity, Vec2, bool)> = world
        .query::<(Entity, &Position, &GolemBoss)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .iter()
        .filter(|(_, _, golem)| {
            golem.attack == GolemAttack::Laser && golem.stage == GolemStage::Acting
        })
        .map(|(entity, position, golem)| (entity, position.0, golem.enraged))
        .collect();
    for (entity, at, enraged) in firing {
        let eye = at + Vec2::new(0.0, -EYE_HEIGHT);
        let angle = {
            let Ok(mut golem) = world.get::<&mut GolemBoss>(entity) else {
                continue;
            };
            if let Some((_, player_at, _)) = player {
                let rate = if enraged {
                    enemies::GOLEM_LASER_RAGE_TURN
                } else {
                    enemies::GOLEM_LASER_TURN
                };
                golem.beam_angle = turn_towards(golem.beam_angle, aim(eye, player_at), rate * dt);
            }
            golem.beam_angle
        };
        let ray = Ray {
            eye,
            angle,
            phase: 0.0,
        };
        draw_beam(world, navigation, textures, sounds, ray, BLUE_BEAM);
    }
}

pub(crate) fn clear_golem_effects(world: &mut World) {
    clear_beam(world);
    let falls: Vec<(Entity, Entity)> = world
        .query::<(Entity, &RockFall)>()
        .iter()
        .map(|(entity, fall)| (entity, fall.rock))
        .collect();
    for (shadow, rock) in falls {
        let _ = world.despawn(shadow);
        let _ = world.despawn(rock);
    }
    let _ = Despawn { remaining: 0.0 };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn golem() -> GolemBoss {
        GolemBoss {
            stage: GolemStage::Walk,
            attack: GolemAttack::Slam,
            last: None,
            timer: 0.0,
            cooldown: 0.0,
            pulse: 0.0,
            rain_dropped: 0,
            second_ring: 0.0,
            beam_angle: 0.0,
            enraged: false,
            seed: 12345,
            idle: slam_clip(),
        }
    }

    fn run(golem: &mut GolemBoss, seconds: f32) -> Vec<Cmd> {
        let mut all = Vec::new();
        let mut time = 0.0;
        while time < seconds {
            all.extend(step(golem, 200.0, 0.02));
            time += 0.02;
        }
        all
    }

    #[test]
    fn an_attack_never_repeats_the_previous_one() {
        let mut boss = golem();
        let mut last = None;
        for _ in 0..200 {
            boss.last = last;
            let attack = choose_attack(&mut boss, 200.0);
            assert_ne!(Some(attack), last);
            last = Some(attack);
        }
    }

    #[test]
    fn a_slam_roots_the_golem_fires_a_ring_and_frees_it_again() {
        let mut boss = golem();
        boss.attack = GolemAttack::Slam;
        boss.last = Some(GolemAttack::Slam);
        boss.stage = GolemStage::Winding;
        boss.timer = enemies::GOLEM_SLAM_WINDUP;
        let commands = run(&mut boss, 3.0);
        assert!(commands.contains(&Cmd::Ring(0.0)));
        assert!(commands.contains(&Cmd::Free));
        assert_eq!(boss.stage, GolemStage::Walk);
    }

    #[test]
    fn rage_adds_a_second_ring() {
        let mut boss = golem();
        boss.enraged = true;
        boss.stage = GolemStage::Winding;
        boss.attack = GolemAttack::Slam;
        boss.timer = 0.01;
        let rings = run(&mut boss, 2.0)
            .iter()
            .filter(|command| matches!(command, Cmd::Ring(_)))
            .count();
        assert_eq!(rings, 2);
    }

    #[test]
    fn rain_drops_rocks_over_its_whole_duration() {
        let mut boss = golem();
        boss.stage = GolemStage::Winding;
        boss.attack = GolemAttack::Rain;
        boss.timer = 0.01;
        let rocks = run(&mut boss, 4.0)
            .iter()
            .filter(|command| matches!(command, Cmd::Rain(_)))
            .count();
        assert!(rocks >= 8, "{rocks}");
    }

    #[test]
    fn the_beam_stops_at_a_wall() {
        let navigation = Navigation::new(&[(10, 5)]);
        let origin = Vec2::new(
            3.0 * crate::tilemap::TILE_SIZE,
            5.5 * crate::tilemap::TILE_SIZE,
        );
        let end = beam_end(&navigation, origin, 0.0);
        assert!(end.x < 10.0 * crate::tilemap::TILE_SIZE);
        assert!(end.x > origin.x);
        let free = beam_end(&Navigation::new(&[]), origin, 0.0);
        assert!(free.distance(origin) > end.distance(origin));
    }

    #[test]
    fn turning_takes_the_short_way_round() {
        let turned = turn_towards(3.0, -3.0, 0.5);
        assert!(turned > 3.0);
    }
}
