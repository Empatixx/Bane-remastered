use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::audio::AudioCommand;
use crate::components::{
    ArgusAttack, ArgusBoss, ArgusStage, Body, Chase, Clip, Color, ContactDamage, Expiring, Frozen,
    Minion, Position, RoomId, Rooted, Spawning,
};
use crate::enemies::{self, EnemyKind, ProjectileKind};
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::prefabs;
use crate::systems::golem::{BeamStyle, aim, draw_beam, play, player_at, turn_towards, unit};

const EYE_LIFT: f32 = 36.0;
const RAGE_TINT: Vec4 = Vec4::new(1.0, 0.72, 0.66, 1.0);
const SUMMON_RADIUS: f32 = 150.0;
const HOP_NEAR: f32 = 260.0;
const HOP_FAR: f32 = 480.0;
const HOP_ROOM: f32 = 140.0;
const ARGUS_BEAM: BeamStyle = BeamStyle {
    glow: Vec4::new(1.0, 0.45, 0.15, 0.5),
    core: Vec4::new(1.0, 0.94, 0.72, 1.0),
    light: [255.0, 140.0, 50.0],
    spark: tex::SPARK_WARM,
    damage: enemies::ARGUS_GAZE_DAMAGE,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Cmd {
    Play(Clip),
    Root,
    Free,
    Ring(f32),
    Summon,
    Teleport,
    AimBeam,
}

fn clip(row: usize, frame_time: f32, looping: bool) -> Clip {
    Clip {
        row,
        frames: 6,
        frame_time,
        looping,
        flipped: false,
    }
}

pub(crate) fn volley_clip() -> Clip {
    clip(2, 0.11, true)
}

pub(crate) fn gaze_clip() -> Clip {
    clip(1, 0.1, false)
}

pub(crate) fn choose_attack(argus: &mut ArgusBoss, minions: usize) -> ArgusAttack {
    let summon = if minions >= enemies::ARGUS_SUMMON_LIMIT {
        0.0
    } else {
        1.5
    };
    let weights = [
        (ArgusAttack::Volley, 3.0),
        (ArgusAttack::Gaze, 2.5),
        (ArgusAttack::Summon, summon),
    ];
    let total: f32 = weights
        .iter()
        .filter(|(attack, _)| Some(*attack) != argus.last)
        .map(|(_, weight)| weight)
        .sum();
    let mut roll = unit(&mut argus.seed) * total;
    for (attack, weight) in weights {
        if Some(attack) == argus.last || weight == 0.0 {
            continue;
        }
        if roll < weight {
            return attack;
        }
        roll -= weight;
    }
    ArgusAttack::Volley
}

pub(crate) fn step(argus: &mut ArgusBoss, minions: usize, dt: f32) -> Vec<Cmd> {
    let mut commands = Vec::new();
    match argus.stage {
        ArgusStage::Float => {
            argus.cooldown -= dt;
            if argus.cooldown > 0.0 {
                return commands;
            }
            commands.push(Cmd::Root);
            if argus.attacks >= enemies::ARGUS_TELEPORT_EVERY {
                argus.attacks = 0;
                argus.stage = ArgusStage::FadingOut;
                argus.timer = enemies::ARGUS_FADE;
                return commands;
            }
            let attack = choose_attack(argus, minions);
            argus.attack = attack;
            argus.last = Some(attack);
            argus.stage = ArgusStage::Winding;
            match attack {
                ArgusAttack::Volley => {
                    argus.timer = enemies::ARGUS_VOLLEY_WINDUP;
                    commands.push(Cmd::Play(volley_clip()));
                }
                ArgusAttack::Gaze => {
                    argus.timer = enemies::ARGUS_GAZE_CHARGE;
                    commands.push(Cmd::Play(gaze_clip()));
                    commands.push(Cmd::AimBeam);
                }
                ArgusAttack::Summon => {
                    argus.timer = enemies::ARGUS_SUMMON_WINDUP;
                    commands.push(Cmd::Play(volley_clip()));
                }
            }
        }
        ArgusStage::Winding => {
            argus.timer -= dt;
            if argus.timer > 0.0 {
                return commands;
            }
            match argus.attack {
                ArgusAttack::Volley => {
                    argus.stage = ArgusStage::Acting;
                    let pulses = if argus.enraged {
                        enemies::ARGUS_VOLLEY_RAGE_PULSES
                    } else {
                        enemies::ARGUS_VOLLEY_PULSES
                    };
                    argus.timer = pulses as f32 * enemies::ARGUS_VOLLEY_EVERY - 0.05;
                    argus.pulse = 0.0;
                    argus.pulses = 0;
                }
                ArgusAttack::Gaze => {
                    argus.stage = ArgusStage::Acting;
                    argus.timer = enemies::ARGUS_GAZE_SECONDS;
                }
                ArgusAttack::Summon => {
                    commands.push(Cmd::Summon);
                    argus.stage = ArgusStage::Recovering;
                    argus.timer = enemies::ARGUS_RECOVER;
                }
            }
        }
        ArgusStage::Acting => {
            argus.timer -= dt;
            if argus.attack == ArgusAttack::Volley {
                argus.pulse -= dt;
                while argus.pulse <= 0.0 && argus.timer > 0.0 {
                    argus.angle += 0.37;
                    commands.push(Cmd::Ring(argus.angle));
                    argus.pulses += 1;
                    argus.pulse += enemies::ARGUS_VOLLEY_EVERY;
                }
            }
            if argus.timer <= 0.0 {
                argus.stage = ArgusStage::Recovering;
                argus.timer = enemies::ARGUS_RECOVER;
            }
        }
        ArgusStage::Recovering => {
            argus.timer -= dt;
            if argus.timer <= 0.0 {
                argus.attacks += 1;
                commands.push(Cmd::Play(argus.idle));
                commands.push(Cmd::Free);
                argus.stage = ArgusStage::Float;
                argus.cooldown = if argus.enraged {
                    enemies::ARGUS_RAGE_COOLDOWN
                } else {
                    enemies::ARGUS_COOLDOWN
                };
            }
        }
        ArgusStage::FadingOut => {
            argus.timer -= dt;
            if argus.timer <= 0.0 {
                commands.push(Cmd::Teleport);
                argus.stage = ArgusStage::FadingIn;
                argus.timer = enemies::ARGUS_FADE;
            }
        }
        ArgusStage::FadingIn => {
            argus.timer -= dt;
            if argus.timer <= 0.0 {
                commands.push(Cmd::Free);
                argus.stage = ArgusStage::Float;
                argus.cooldown = enemies::ARGUS_FIRST_COOLDOWN * 0.4;
            }
        }
    }
    commands
}

pub(crate) fn fade(argus: &ArgusBoss) -> f32 {
    match argus.stage {
        ArgusStage::FadingOut => (argus.timer / enemies::ARGUS_FADE).clamp(0.0, 1.0),
        ArgusStage::FadingIn => 1.0 - (argus.timer / enemies::ARGUS_FADE).clamp(0.0, 1.0),
        _ => 1.0,
    }
}

pub(crate) fn tick_argus(
    world: &mut World,
    physics: &mut Physics,
    navigation: &Navigation,
    textures: &Textures,
    _sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    let target = player_at(world).map(|(_, position, _)| position);
    let minions = world.query::<&Minion>().iter().count();
    let bosses: Vec<(Entity, Vec2, i32, i32)> = world
        .query::<(Entity, &Position, &crate::components::Health, &ArgusBoss)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .without::<&Frozen>()
        .iter()
        .map(|(entity, position, health, _)| (entity, position.0, health.current, health.max))
        .collect();
    for (entity, at, current, max) in bosses {
        let calm = world
            .get::<&ArgusBoss>(entity)
            .is_ok_and(|argus| !argus.enraged);
        if calm && current as f32 <= max as f32 * enemies::ARGUS_RAGE_AT {
            if let Ok(mut argus) = world.get::<&mut ArgusBoss>(entity) {
                argus.enraged = true;
            }
            if let Ok(mut chase) = world.get::<&mut Chase>(entity) {
                chase.speed *= enemies::ARGUS_RAGE_SPEED;
            }
        }
        let (commands, alpha, enraged) = {
            let Ok(mut argus) = world.get::<&mut ArgusBoss>(entity) else {
                continue;
            };
            let commands = step(&mut argus, minions, dt);
            (commands, fade(&argus), argus.enraged)
        };
        if let Ok(mut color) = world.get::<&mut Color>(entity) {
            let tint = if enraged { RAGE_TINT } else { Vec4::ONE };
            color.0 = Vec4::new(tint.x, tint.y, tint.z, alpha);
        }
        for command in commands {
            execute(
                world, physics, navigation, textures, entity, at, target, command,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn execute(
    world: &mut World,
    physics: &mut Physics,
    navigation: &Navigation,
    textures: &Textures,
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
            let enraged = world
                .get::<&ArgusBoss>(entity)
                .is_ok_and(|argus| argus.enraged);
            let count = if enraged {
                enemies::ARGUS_VOLLEY_RAGE_RING
            } else {
                enemies::ARGUS_VOLLEY_RING
            };
            for index in 0..count {
                let angle = offset + index as f32 / count as f32 * std::f32::consts::TAU;
                let bullet = prefabs::enemy_bullet(
                    world,
                    physics,
                    ProjectileKind::ArgusShot,
                    at,
                    Vec2::from_angle(angle),
                    textures,
                );
                let _ = world.insert_one(bullet, ContactDamage(enemies::ARGUS_SHOT_DAMAGE));
                if let Some(room) = room {
                    let _ = world.insert_one(bullet, room);
                }
            }
        }
        Cmd::Summon => {
            let mut seed = (at.x as u32) ^ ((at.y as u32) << 9) ^ 0x51ED_270B;
            let turn = unit(&mut seed) * std::f32::consts::TAU;
            for index in 0..enemies::ARGUS_SUMMON_COUNT {
                let angle = turn
                    + index as f32 / enemies::ARGUS_SUMMON_COUNT as f32 * std::f32::consts::TAU;
                let spot = at + Vec2::from_angle(angle) * SUMMON_RADIUS;
                if !navigation.is_blocked(spot, Vec2::splat(60.0)) {
                    prefabs::hatchling(world, physics, EnemyKind::DemonEye, spot, room, textures);
                }
            }
        }
        Cmd::Teleport => {
            let mut seed = (at.x as u32).wrapping_mul(2_246_822_519) ^ (at.y as u32);
            for _ in 0..16 {
                let angle = unit(&mut seed) * std::f32::consts::TAU;
                let distance = HOP_NEAR + unit(&mut seed) * (HOP_FAR - HOP_NEAR);
                let spot = at + Vec2::from_angle(angle) * distance;
                if player.is_some_and(|player| player.distance(spot) < HOP_ROOM) {
                    continue;
                }
                if navigation.is_blocked(spot, Vec2::splat(HOP_ROOM)) {
                    continue;
                }
                if let Ok(mut position) = world.get::<&mut Position>(entity) {
                    position.0 = spot;
                }
                if let Ok(body) = world.get::<&Body>(entity) {
                    physics.teleport(*body, spot);
                }
                break;
            }
        }
        Cmd::AimBeam => {
            if let Some(player) = player {
                if let Ok(mut argus) = world.get::<&mut ArgusBoss>(entity) {
                    argus.angle = aim(at - Vec2::new(0.0, EYE_LIFT), player);
                }
            }
        }
    }
}

pub(crate) fn tick_gaze(
    world: &mut World,
    navigation: &Navigation,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    let player = player_at(world).map(|(_, position, _)| position);
    let firing: Vec<(Entity, Vec2, bool)> = world
        .query::<(Entity, &Position, &ArgusBoss)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .iter()
        .filter(|(_, _, argus)| {
            argus.attack == ArgusAttack::Gaze && argus.stage == ArgusStage::Acting
        })
        .map(|(entity, position, argus)| (entity, position.0, argus.enraged))
        .collect();
    for (entity, at, enraged) in firing {
        let eye = at - Vec2::new(0.0, EYE_LIFT);
        let angle = {
            let Ok(mut argus) = world.get::<&mut ArgusBoss>(entity) else {
                continue;
            };
            if let Some(player) = player {
                let rate = if enraged {
                    enemies::ARGUS_GAZE_RAGE_TURN
                } else {
                    enemies::ARGUS_GAZE_TURN
                };
                argus.angle = turn_towards(argus.angle, aim(eye, player), rate * dt);
            }
            argus.angle
        };
        draw_beam(world, navigation, textures, sounds, eye, angle, ARGUS_BEAM);
        if enraged {
            draw_beam(
                world,
                navigation,
                textures,
                sounds,
                eye,
                angle + std::f32::consts::PI,
                ARGUS_BEAM,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argus() -> ArgusBoss {
        ArgusBoss {
            stage: ArgusStage::Float,
            attack: ArgusAttack::Volley,
            last: None,
            timer: 0.0,
            cooldown: 0.0,
            pulse: 0.0,
            pulses: 0,
            angle: 0.0,
            attacks: 0,
            enraged: false,
            seed: 777,
            idle: volley_clip(),
        }
    }

    fn run(argus: &mut ArgusBoss, seconds: f32) -> Vec<Cmd> {
        let mut all = Vec::new();
        let mut time = 0.0;
        while time < seconds {
            all.extend(step(argus, 0, 0.02));
            time += 0.02;
        }
        all
    }

    #[test]
    fn attacks_never_repeat_and_summoning_stops_at_the_limit() {
        let mut boss = argus();
        let mut last = None;
        for _ in 0..200 {
            boss.last = last;
            let attack = choose_attack(&mut boss, 0);
            assert_ne!(Some(attack), last);
            last = Some(attack);
        }
        boss.last = None;
        for _ in 0..200 {
            assert_ne!(
                choose_attack(&mut boss, enemies::ARGUS_SUMMON_LIMIT),
                ArgusAttack::Summon
            );
        }
    }

    #[test]
    fn a_volley_fires_rotating_rings_and_returns_to_floating() {
        let mut boss = argus();
        boss.stage = ArgusStage::Winding;
        boss.attack = ArgusAttack::Volley;
        boss.last = Some(ArgusAttack::Volley);
        boss.timer = 0.01;
        let commands = run(&mut boss, 4.0);
        let rings = commands
            .iter()
            .filter(|command| matches!(command, Cmd::Ring(_)))
            .count();
        assert_eq!(rings, enemies::ARGUS_VOLLEY_PULSES as usize);
        assert!(commands.contains(&Cmd::Free));
        assert_eq!(boss.stage, ArgusStage::Float);
    }

    #[test]
    fn a_summon_calls_the_minions_once() {
        let mut boss = argus();
        boss.stage = ArgusStage::Winding;
        boss.attack = ArgusAttack::Summon;
        boss.timer = 0.01;
        let summons = run(&mut boss, 1.0)
            .iter()
            .filter(|command| **command == Cmd::Summon)
            .count();
        assert_eq!(summons, 1);
    }

    #[test]
    fn every_third_attack_ends_in_a_teleport_with_a_fade() {
        let mut boss = argus();
        boss.attacks = enemies::ARGUS_TELEPORT_EVERY;
        let commands = run(&mut boss, 1.5);
        assert!(commands.contains(&Cmd::Teleport));
        assert_eq!(boss.attacks, 0);
        boss.stage = ArgusStage::FadingOut;
        boss.timer = enemies::ARGUS_FADE / 2.0;
        assert!((fade(&boss) - 0.5).abs() < 1e-4);
        boss.stage = ArgusStage::FadingIn;
        assert!((fade(&boss) - 0.5).abs() < 1e-4);
    }

    #[test]
    fn rage_fires_more_rings() {
        let mut boss = argus();
        boss.enraged = true;
        boss.stage = ArgusStage::Winding;
        boss.attack = ArgusAttack::Volley;
        boss.last = Some(ArgusAttack::Volley);
        boss.timer = 0.01;
        let rings = run(&mut boss, 5.0)
            .iter()
            .filter(|command| matches!(command, Cmd::Ring(_)))
            .count();
        assert_eq!(rings, enemies::ARGUS_VOLLEY_RAGE_PULSES as usize);
    }
}
