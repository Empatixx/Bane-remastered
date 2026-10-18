use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    ArgusAttack, ArgusBoss, ArgusStage, Body, Chase, Clip, Color, ContactDamage, Expiring, Frozen,
    Health, Orbiter, Position, RoomId, Rooted, Spawning,
};
use crate::enemies::{self, EnemyKind, ProjectileKind};
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::prefabs;
use crate::systems::boss_kit::{self, play};
use crate::systems::player::living_player_at;
use crate::systems::hostile::Hostile;

const SPRITE_SCALE: f32 = 4.0;
const EYES: [(f32, f32); 7] = [
    (35.0, 23.0),
    (17.0, 30.0),
    (49.0, 29.0),
    (21.0, 44.0),
    (44.0, 45.0),
    (30.0, 48.0),
    (36.0, 40.0),
];
const RAGE_TINT: Vec4 = Vec4::new(1.0, 0.72, 0.66, 1.0);
const HOP_NEAR: f32 = 260.0;
const HOP_FAR: f32 = 480.0;
const HOP_ROOM: f32 = 140.0;
const ORBIT_SQUASH: f32 = 0.8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Cmd {
    Play(Clip),
    Root,
    Free,
    Ring(f32),
    Shot(u32),
    Shield,
    Teleport,
    Sound(SoundEffect),
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

pub(crate) fn aim_clip() -> Clip {
    clip(1, 0.1, false)
}

pub(crate) fn choose_attack(argus: &mut ArgusBoss, orbiters: usize) -> ArgusAttack {
    let wanted = orbit_count(argus.enraged);
    let shield = if orbiters == 0 {
        6.0
    } else if orbiters < wanted {
        2.0
    } else {
        0.0
    };
    let weights = [
        (ArgusAttack::Volley, 3.0),
        (ArgusAttack::Barrage, 3.0),
        (ArgusAttack::Shield, shield),
    ];
    boss_kit::pick_attack(&mut argus.brain, &weights, ArgusAttack::Volley)
}

pub(crate) fn orbit_count(enraged: bool) -> usize {
    if enraged {
        enemies::ARGUS_ORBIT_RAGE_COUNT
    } else {
        enemies::ARGUS_ORBIT_COUNT
    }
}

pub(crate) fn step(argus: &mut ArgusBoss, orbiters: usize, dt: f32) -> Vec<Cmd> {
    let mut commands = Vec::new();
    match argus.brain.stage {
        ArgusStage::Float => {
            argus.brain.cooldown -= dt;
            if argus.brain.cooldown > 0.0 {
                return commands;
            }
            commands.push(Cmd::Root);
            if argus.attacks >= enemies::ARGUS_TELEPORT_EVERY {
                argus.attacks = 0;
                argus.brain.stage = ArgusStage::FadingOut;
                argus.brain.timer = enemies::ARGUS_FADE;
                commands.push(Cmd::Sound(SoundEffect::ArgusTeleport));
                return commands;
            }
            let attack = choose_attack(argus, orbiters);
            argus.brain.attack = attack;
            argus.brain.last = Some(attack);
            argus.brain.stage = ArgusStage::Winding;
            match attack {
                ArgusAttack::Volley => {
                    argus.brain.timer = enemies::ARGUS_VOLLEY_WINDUP;
                    commands.push(Cmd::Play(volley_clip()));
                    commands.push(Cmd::Sound(SoundEffect::ArgusCharge));
                }
                ArgusAttack::Barrage => {
                    argus.brain.timer = enemies::ARGUS_BARRAGE_WINDUP;
                    commands.push(Cmd::Play(aim_clip()));
                    commands.push(Cmd::Sound(SoundEffect::ArgusCharge));
                }
                ArgusAttack::Shield => {
                    argus.brain.timer = enemies::ARGUS_SHIELD_WINDUP;
                    commands.push(Cmd::Play(volley_clip()));
                    commands.push(Cmd::Sound(SoundEffect::ArgusSummon));
                }
            }
        }
        ArgusStage::Winding => {
            argus.brain.timer -= dt;
            if argus.brain.timer > 0.0 {
                return commands;
            }
            argus.pulse = 0.0;
            argus.pulses = 0;
            match argus.brain.attack {
                ArgusAttack::Volley => {
                    argus.brain.stage = ArgusStage::Acting;
                    let pulses = if argus.enraged {
                        enemies::ARGUS_VOLLEY_RAGE_PULSES
                    } else {
                        enemies::ARGUS_VOLLEY_PULSES
                    };
                    argus.brain.timer = pulses as f32 * enemies::ARGUS_VOLLEY_EVERY - 0.05;
                }
                ArgusAttack::Barrage => {
                    argus.brain.stage = ArgusStage::Acting;
                    let shots = if argus.enraged {
                        enemies::ARGUS_BARRAGE_RAGE_SHOTS
                    } else {
                        enemies::ARGUS_BARRAGE_SHOTS
                    };
                    argus.brain.timer = shots as f32 * enemies::ARGUS_BARRAGE_EVERY - 0.05;
                }
                ArgusAttack::Shield => {
                    commands.push(Cmd::Shield);
                    argus.brain.stage = ArgusStage::Recovering;
                    argus.brain.timer = enemies::ARGUS_RECOVER;
                }
            }
        }
        ArgusStage::Acting => {
            argus.brain.timer -= dt;
            argus.pulse -= dt;
            while argus.pulse <= 0.0 && argus.brain.timer > 0.0 {
                match argus.brain.attack {
                    ArgusAttack::Volley => {
                        argus.angle += 0.37;
                        commands.push(Cmd::Ring(argus.angle));
                        argus.pulse += enemies::ARGUS_VOLLEY_EVERY;
                    }
                    ArgusAttack::Barrage => {
                        commands.push(Cmd::Shot(argus.pulses));
                        argus.pulse += enemies::ARGUS_BARRAGE_EVERY;
                    }
                    ArgusAttack::Shield => break,
                }
                argus.pulses += 1;
            }
            if argus.brain.timer <= 0.0 {
                argus.brain.stage = ArgusStage::Recovering;
                argus.brain.timer = enemies::ARGUS_RECOVER;
            }
        }
        ArgusStage::Recovering => {
            argus.brain.timer -= dt;
            if argus.brain.timer <= 0.0 {
                argus.attacks += 1;
                commands.push(Cmd::Play(argus.brain.idle));
                commands.push(Cmd::Free);
                argus.brain.stage = ArgusStage::Float;
                argus.brain.cooldown = if argus.enraged {
                    enemies::ARGUS_RAGE_COOLDOWN
                } else {
                    enemies::ARGUS_COOLDOWN
                };
            }
        }
        ArgusStage::FadingOut => {
            argus.brain.timer -= dt;
            if argus.brain.timer <= 0.0 {
                commands.push(Cmd::Teleport);
                argus.brain.stage = ArgusStage::FadingIn;
                argus.brain.timer = enemies::ARGUS_FADE;
            }
        }
        ArgusStage::FadingIn => {
            argus.brain.timer -= dt;
            if argus.brain.timer <= 0.0 {
                commands.push(Cmd::Free);
                argus.brain.stage = ArgusStage::Float;
                argus.brain.cooldown = enemies::ARGUS_FIRST_COOLDOWN * 0.4;
            }
        }
    }
    commands
}

pub(crate) fn fade(argus: &ArgusBoss) -> f32 {
    match argus.brain.stage {
        ArgusStage::FadingOut => (argus.brain.timer / enemies::ARGUS_FADE).clamp(0.0, 1.0),
        ArgusStage::FadingIn => 1.0 - (argus.brain.timer / enemies::ARGUS_FADE).clamp(0.0, 1.0),
        _ => 1.0,
    }
}

pub(crate) fn eye_offset(index: u32) -> Vec2 {
    let (x, y) = EYES[index as usize % EYES.len()];
    Vec2::new(x - 32.0, y - 32.0) * SPRITE_SCALE
}

fn orbiters_of(world: &World, owner: Entity) -> Vec<f32> {
    world
        .query::<&Orbiter>()
        .iter()
        .filter(|orbiter| orbiter.owner == owner)
        .map(|orbiter| orbiter.angle)
        .collect()
}

pub(crate) fn tick_argus(world: &mut World, hostile: &mut Hostile<'_>) {
    let physics = &mut *hostile.physics;
    let navigation = hostile.navigation;
    let textures = hostile.textures;
    let sounds = &mut *hostile.sounds;
    let dt = hostile.dt;
    let target = living_player_at(world).map(|(_, position, _)| position);
    let bosses: Vec<(Entity, Vec2, i32, i32)> = world
        .query::<(Entity, &Position, &Health, &ArgusBoss)>()
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
            sounds.push(AudioCommand::At(SoundEffect::ArgusRage, at));
            if let Ok(mut chase) = world.get::<&mut Chase>(entity) {
                chase.speed *= enemies::ARGUS_RAGE_SPEED;
            }
        }
        let orbiters = orbiters_of(world, entity).len();
        let (commands, alpha, enraged) = {
            let Ok(mut argus) = world.get::<&mut ArgusBoss>(entity) else {
                continue;
            };
            let commands = step(&mut argus, orbiters, dt);
            (commands, fade(&argus), argus.enraged)
        };
        if let Ok(mut color) = world.get::<&mut Color>(entity) {
            let tint = if enraged { RAGE_TINT } else { Vec4::ONE };
            color.0 = Vec4::new(tint.x, tint.y, tint.z, alpha);
        }
        for command in commands {
            execute(
                world, physics, navigation, textures, sounds, entity, at, target, command,
            );
        }
    }
}

fn shoot(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    room: Option<RoomId>,
    origin: Vec2,
    direction: Vec2,
) {
    let bullet = prefabs::enemy_bullet(
        world,
        physics,
        ProjectileKind::ArgusShot,
        origin,
        direction,
        textures,
    );
    let _ = world.insert_one(bullet, ContactDamage(enemies::ARGUS_SHOT_DAMAGE));
    if let Some(room) = room {
        let _ = world.insert_one(bullet, room);
    }
}

#[allow(clippy::too_many_arguments)]
fn execute(
    world: &mut World,
    physics: &mut Physics,
    navigation: &Navigation,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    entity: Entity,
    at: Vec2,
    player: Option<Vec2>,
    command: Cmd,
) {
    let room = world.get::<&RoomId>(entity).map(|room| *room).ok();
    let enraged = world
        .get::<&ArgusBoss>(entity)
        .is_ok_and(|argus| argus.enraged);
    match command {
        Cmd::Play(clip) => play(world, entity, clip),
        Cmd::Sound(effect) => sounds.push(AudioCommand::At(effect, at)),
        Cmd::Root => {
            let _ = world.insert_one(entity, Rooted);
        }
        Cmd::Free => {
            let _ = world.remove_one::<Rooted>(entity);
        }
        Cmd::Ring(offset) => {
            sounds.push(AudioCommand::At(SoundEffect::ArgusPulse, at));
            let count = if enraged {
                enemies::ARGUS_VOLLEY_RAGE_RING
            } else {
                enemies::ARGUS_VOLLEY_RING
            };
            for index in 0..count {
                let angle = offset + index as f32 / count as f32 * std::f32::consts::TAU;
                shoot(world, physics, textures, room, at, Vec2::from_angle(angle));
            }
        }
        Cmd::Shot(index) => {
            let Some(player) = player else {
                return;
            };
            sounds.push(AudioCommand::At(SoundEffect::ArgusShot, at));
            let shots = if enraged { 2 } else { 1 };
            let mut seed = index.wrapping_mul(0x9E37_79B1) ^ (at.x as u32);
            for shot in 0..shots {
                let origin = at + eye_offset(index + shot);
                let aimed = (player - origin).to_angle();
                let jitter =
                    (crate::rng::roll(&mut seed) - 0.5) * 2.0 * enemies::ARGUS_BARRAGE_SPREAD;
                shoot(
                    world,
                    physics,
                    textures,
                    room,
                    origin,
                    Vec2::from_angle(aimed + jitter),
                );
            }
        }
        Cmd::Shield => {
            let wanted = orbit_count(enraged);
            let existing = orbiters_of(world, entity);
            let missing = wanted.saturating_sub(existing.len());
            let base = existing.first().copied().unwrap_or(0.0);
            for index in 0..missing {
                let slot = existing.len() + index;
                let angle = base + slot as f32 / wanted as f32 * std::f32::consts::TAU;
                let spot = at + Vec2::from_angle(angle) * enemies::ARGUS_ORBIT_RADIUS;
                let eye =
                    prefabs::hatchling(world, physics, EnemyKind::ArgusEye, spot, room, textures);
                let _ = world.insert_one(
                    eye,
                    Orbiter {
                        owner: entity,
                        angle,
                    },
                );
            }
        }
        Cmd::Teleport => {
            let mut seed = (at.x as u32).wrapping_mul(2_246_822_519) ^ (at.y as u32);
            for _ in 0..16 {
                let angle = crate::rng::roll(&mut seed) * std::f32::consts::TAU;
                let distance = HOP_NEAR + crate::rng::roll(&mut seed) * (HOP_FAR - HOP_NEAR);
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
    }
}

pub(crate) fn tick_orbiters(world: &mut World, physics: &mut Physics, dt: f32) {
    let orbiters: Vec<(Entity, Orbiter)> = world
        .query::<(Entity, &Orbiter)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .iter()
        .map(|(entity, orbiter)| (entity, *orbiter))
        .collect();
    for (entity, orbiter) in orbiters {
        let owner = world
            .get::<&Position>(orbiter.owner)
            .ok()
            .zip(world.get::<&Health>(orbiter.owner).ok())
            .filter(|(_, health)| health.current > 0)
            .map(|(position, _)| position.0);
        let Some(center) = owner else {
            if let Ok(mut health) = world.get::<&mut Health>(entity) {
                health.current = 0;
            }
            continue;
        };
        let angle = orbiter.angle + enemies::ARGUS_ORBIT_SPEED * dt;
        if let Ok(mut stored) = world.get::<&mut Orbiter>(entity) {
            stored.angle = angle;
        }
        let spot = center
            + Vec2::new(angle.cos(), angle.sin() * ORBIT_SQUASH) * enemies::ARGUS_ORBIT_RADIUS;
        if let Ok(mut position) = world.get::<&mut Position>(entity) {
            position.0 = spot;
        }
        if let Ok(body) = world.get::<&Body>(entity) {
            physics.teleport(*body, spot);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::BossBrain;

    fn argus() -> ArgusBoss {
        ArgusBoss {
            brain: BossBrain::new(
                ArgusStage::Float,
                ArgusAttack::Volley,
                0.0,
                777,
                volley_clip(),
            ),
            pulse: 0.0,
            pulses: 0,
            angle: 0.0,
            attacks: 0,
            enraged: false,
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
    fn attacks_never_repeat_and_the_shield_is_only_rebuilt_when_eyes_are_missing() {
        let mut boss = argus();
        let mut last = None;
        for _ in 0..200 {
            boss.brain.last = last;
            let attack = choose_attack(&mut boss, 0);
            assert_ne!(Some(attack), last);
            last = Some(attack);
        }
        boss.brain.last = None;
        for _ in 0..200 {
            assert_ne!(
                choose_attack(&mut boss, enemies::ARGUS_ORBIT_COUNT),
                ArgusAttack::Shield
            );
        }
    }

    #[test]
    fn argus_charges_audibly_before_shooting_and_chimes_before_calling_its_eyes() {
        for seed in 0..40 {
            let mut boss = argus();
            boss.brain.seed = seed * 7919 + 1;
            let commands = step(&mut boss, 0, 0.02);
            let expected = match boss.brain.attack {
                ArgusAttack::Volley | ArgusAttack::Barrage => SoundEffect::ArgusCharge,
                ArgusAttack::Shield => SoundEffect::ArgusSummon,
            };
            assert!(commands.contains(&Cmd::Sound(expected)), "{:?}", boss.brain.attack);
        }
    }

    #[test]
    fn fading_out_to_teleport_makes_a_sound() {
        let mut boss = argus();
        boss.attacks = enemies::ARGUS_TELEPORT_EVERY;
        let commands = step(&mut boss, 0, 0.02);
        assert_eq!(boss.brain.stage, ArgusStage::FadingOut);
        assert!(commands.contains(&Cmd::Sound(SoundEffect::ArgusTeleport)));
    }

    #[test]
    fn a_volley_fires_rotating_rings_and_returns_to_floating() {
        let mut boss = argus();
        boss.brain.stage = ArgusStage::Winding;
        boss.brain.attack = ArgusAttack::Volley;
        boss.brain.last = Some(ArgusAttack::Volley);
        boss.brain.timer = 0.01;
        let commands = run(&mut boss, 4.0);
        let rings = commands
            .iter()
            .filter(|command| matches!(command, Cmd::Ring(_)))
            .count();
        assert_eq!(rings, enemies::ARGUS_VOLLEY_PULSES as usize);
        assert!(commands.contains(&Cmd::Free));
        assert_eq!(boss.brain.stage, ArgusStage::Float);
    }

    #[test]
    fn a_shield_attack_raises_the_orbiting_eyes_once() {
        let mut boss = argus();
        boss.brain.stage = ArgusStage::Winding;
        boss.brain.attack = ArgusAttack::Shield;
        boss.brain.timer = 0.01;
        let summons = run(&mut boss, 1.0)
            .iter()
            .filter(|command| **command == Cmd::Shield)
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
        boss.brain.stage = ArgusStage::FadingOut;
        boss.brain.timer = enemies::ARGUS_FADE / 2.0;
        assert!((fade(&boss) - 0.5).abs() < 1e-4);
        boss.brain.stage = ArgusStage::FadingIn;
        assert!((fade(&boss) - 0.5).abs() < 1e-4);
    }

    #[test]
    fn a_barrage_fires_one_aimed_shot_per_pulse() {
        let mut boss = argus();
        boss.brain.stage = ArgusStage::Winding;
        boss.brain.attack = ArgusAttack::Barrage;
        boss.brain.last = Some(ArgusAttack::Barrage);
        boss.brain.timer = 0.01;
        let shots = run(&mut boss, 3.0)
            .iter()
            .filter(|command| matches!(command, Cmd::Shot(_)))
            .count();
        assert_eq!(shots, enemies::ARGUS_BARRAGE_SHOTS as usize);
    }

    #[test]
    fn rage_fires_more_rings() {
        let mut boss = argus();
        boss.enraged = true;
        boss.brain.stage = ArgusStage::Winding;
        boss.brain.attack = ArgusAttack::Volley;
        boss.brain.last = Some(ArgusAttack::Volley);
        boss.brain.timer = 0.01;
        let rings = run(&mut boss, 5.0)
            .iter()
            .filter(|command| matches!(command, Cmd::Ring(_)))
            .count();
        assert_eq!(rings, enemies::ARGUS_VOLLEY_RAGE_PULSES as usize);
    }
}
