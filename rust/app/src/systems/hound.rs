use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Animation, Chase, Clip, Color, Dodge, Expiring, Frozen, Health, HoundAttack, HoundBoss,
    HoundStage, Layer, Light, Position, RoomId, Rooted, Scorch, Size, Spawning, Sprite,
};
use crate::enemies::{self, ProjectileKind};
use crate::physics::Physics;
use crate::prefabs;
use crate::systems::damage::hurt_player;
use crate::systems::golem::{play, player_at, unit};

const SPRITE_SCALE: f32 = 2.4;
const CELL: Vec2 = Vec2::new(64.0, 42.0);
const HEADS: [Vec2; 3] = [
    Vec2::new(98.0, 31.0),
    Vec2::new(87.0, 11.0),
    Vec2::new(94.0, 46.0),
];
const FEET: Vec2 = Vec2::new(0.0, 66.0);
const SCORCH_SIZE: Vec2 = Vec2::new(58.0, 68.0);
const SCORCH_REACH: f32 = 44.0;
const RAGE_TINT: Vec4 = Vec4::new(1.0, 0.7, 0.62, 1.0);
const ATTACK_VAR: &str = "BANE_HOUND_ATTACK";

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Cmd {
    Play(Clip),
    Root,
    Free,
    Charge(bool),
    Breathe,
    Ring,
}

fn clip(row: usize, frame_time: f32) -> Clip {
    Clip {
        row,
        frames: 8,
        frame_time,
        looping: false,
        flipped: false,
    }
}

fn forced_attack() -> Option<HoundAttack> {
    match std::env::var(ATTACK_VAR)
        .ok()?
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "lunge" | "charge" => Some(HoundAttack::Lunge),
        "breath" => Some(HoundAttack::Breath),
        "howl" => Some(HoundAttack::Howl),
        _ => None,
    }
}

pub(crate) fn choose_attack(hound: &mut HoundBoss, distance: f32) -> HoundAttack {
    if let Some(attack) = forced_attack() {
        return attack;
    }
    let close = distance < enemies::HOUND_NEAR;
    let weights = [
        (HoundAttack::Lunge, if close { 2.0 } else { 4.0 }),
        (HoundAttack::Breath, if close { 4.0 } else { 1.5 }),
        (HoundAttack::Howl, 1.5),
    ];
    let total: f32 = weights
        .iter()
        .filter(|(attack, _)| Some(*attack) != hound.last)
        .map(|(_, weight)| weight)
        .sum();
    let mut roll = unit(&mut hound.seed) * total;
    for (attack, weight) in weights {
        if Some(attack) == hound.last {
            continue;
        }
        if roll < weight {
            return attack;
        }
        roll -= weight;
    }
    HoundAttack::Lunge
}

pub(crate) fn step(hound: &mut HoundBoss, distance: f32, dt: f32) -> Vec<Cmd> {
    let mut commands = Vec::new();
    match hound.stage {
        HoundStage::Prowl => {
            hound.cooldown -= dt;
            if hound.cooldown > 0.0 {
                return commands;
            }
            let attack = choose_attack(hound, distance);
            hound.attack = attack;
            hound.last = Some(attack);
            hound.stage = HoundStage::Winding;
            commands.push(Cmd::Root);
            let (row, frame_time, windup) = match attack {
                HoundAttack::Lunge => (1, 0.08, enemies::HOUND_LUNGE_WINDUP),
                HoundAttack::Breath => (2, 0.1, enemies::HOUND_BREATH_WINDUP),
                HoundAttack::Howl => (3, 0.1, enemies::HOUND_HOWL_WINDUP),
            };
            hound.timer = windup;
            commands.push(Cmd::Play(clip(row, frame_time)));
        }
        HoundStage::Winding => {
            hound.timer -= dt;
            if hound.timer > 0.0 {
                return commands;
            }
            hound.pulse = 0.0;
            match hound.attack {
                HoundAttack::Lunge => {
                    hound.stage = HoundStage::Acting;
                    hound.timer = enemies::HOUND_LUNGE_SECONDS;
                    commands.push(Cmd::Free);
                    commands.push(Cmd::Charge(true));
                }
                HoundAttack::Breath => {
                    hound.stage = HoundStage::Acting;
                    hound.timer = enemies::HOUND_BREATH_SECONDS;
                }
                HoundAttack::Howl => {
                    commands.push(Cmd::Ring);
                    hound.stage = HoundStage::Recovering;
                    hound.timer = enemies::HOUND_HOWL_RECOVER;
                }
            }
        }
        HoundStage::Acting => {
            hound.timer -= dt;
            if hound.attack == HoundAttack::Breath {
                hound.pulse -= dt;
                while hound.pulse <= 0.0 && hound.timer > 0.0 {
                    commands.push(Cmd::Breathe);
                    hound.pulse += enemies::HOUND_BREATH_EVERY;
                }
            }
            if hound.timer <= 0.0 {
                let recover = match hound.attack {
                    HoundAttack::Lunge => {
                        commands.push(Cmd::Charge(false));
                        commands.push(Cmd::Root);
                        enemies::HOUND_LUNGE_RECOVER
                    }
                    _ => enemies::HOUND_BREATH_RECOVER,
                };
                hound.stage = HoundStage::Recovering;
                hound.timer = recover;
            }
        }
        HoundStage::Recovering => {
            hound.timer -= dt;
            if hound.timer <= 0.0 {
                commands.push(Cmd::Play(hound.idle));
                commands.push(Cmd::Free);
                hound.stage = HoundStage::Prowl;
                hound.cooldown = if hound.enraged {
                    enemies::HOUND_RAGE_COOLDOWN
                } else {
                    enemies::HOUND_COOLDOWN
                };
            }
        }
    }
    commands
}

fn head_offset(index: usize, mirrored: bool) -> Vec2 {
    let offset = (HEADS[index % HEADS.len()] - CELL) * SPRITE_SCALE;
    Vec2::new(if mirrored { -offset.x } else { offset.x }, offset.y)
}

pub(crate) fn tick_hound(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    let target = player_at(world).map(|(_, position, _)| position);
    let bosses: Vec<(Entity, Vec2, i32, i32)> = world
        .query::<(Entity, &Position, &Health, &HoundBoss)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .without::<&Frozen>()
        .iter()
        .map(|(entity, position, health, _)| (entity, position.0, health.current, health.max))
        .collect();
    for (entity, at, current, max) in bosses {
        let calm = world
            .get::<&HoundBoss>(entity)
            .is_ok_and(|hound| !hound.enraged);
        if calm && current as f32 <= max as f32 * enemies::HOUND_RAGE_AT {
            if let Ok(mut hound) = world.get::<&mut HoundBoss>(entity) {
                hound.enraged = true;
            }
            if let Ok(mut chase) = world.get::<&mut Chase>(entity) {
                chase.speed *= enemies::HOUND_RAGE_SPEED;
            }
            if let Ok(mut color) = world.get::<&mut Color>(entity) {
                color.0 = RAGE_TINT;
            }
        }
        let distance = target.map_or(f32::MAX, |player| player.distance(at));
        let (commands, charging, enraged) = {
            let Ok(mut hound) = world.get::<&mut HoundBoss>(entity) else {
                continue;
            };
            let commands = step(&mut hound, distance, dt);
            (commands, hound.charging, hound.enraged)
        };
        for command in commands {
            execute(
                world, physics, textures, sounds, entity, at, target, enraged, command,
            );
        }
        if world
            .get::<&HoundBoss>(entity)
            .is_ok_and(|hound| hound.charging)
            || charging
        {
            leave_fire(world, textures, entity, at, dt);
        }
    }
}

fn leave_fire(world: &mut World, textures: &Textures, entity: Entity, at: Vec2, dt: f32) {
    let ready = {
        let Ok(mut hound) = world.get::<&mut HoundBoss>(entity) else {
            return;
        };
        hound.pulse -= dt;
        if hound.pulse > 0.0 {
            return;
        }
        hound.pulse += enemies::HOUND_TRAIL_EVERY;
        true
    };
    if !ready {
        return;
    }
    let mut seed = (at.x as u32).wrapping_mul(2_654_435_761) ^ (at.y as u32).rotate_left(9);
    let jitter = Vec2::new(unit(&mut seed) - 0.5, unit(&mut seed) - 0.5) * 24.0;
    let clip = Clip {
        row: 0,
        frames: 4,
        frame_time: 0.09,
        looping: true,
        flipped: unit(&mut seed) > 0.5,
    };
    world.spawn((
        Scorch {
            remaining: enemies::HOUND_TRAIL_SECONDS,
            total: enemies::HOUND_TRAIL_SECONDS,
        },
        Position(at + FEET * Vec2::Y + jitter - Vec2::new(0.0, SCORCH_SIZE.y * 0.2)),
        Size(SCORCH_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(tex::HOUND_FIRE), 0, 0, 4, 1),
        Animation::new(4, 1, clip),
        Light::rgb(Vec2::ZERO, 255.0, 120.0, 30.0, 70.0, 0.9),
        Layer(1),
    ));
}

pub(crate) fn tick_scorches(world: &mut World, sounds: &mut Vec<AudioCommand>, dt: f32) {
    let mut gone = Vec::new();
    for (entity, scorch, color) in world.query_mut::<(Entity, &mut Scorch, &mut Color)>() {
        scorch.remaining -= dt;
        let fade = (scorch.remaining / (scorch.total * 0.3)).clamp(0.0, 1.0);
        color.0.w = fade;
        if scorch.remaining <= 0.0 {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
    let Some((player, at, size)) = player_at(world) else {
        return;
    };
    if world.get::<&Dodge>(player).is_ok() {
        return;
    }
    let burning = world
        .query::<(&Scorch, &Position)>()
        .iter()
        .any(|(scorch, position)| {
            scorch.remaining > 0.2
                && (position.0 - at)
                    .abs()
                    .cmple(Vec2::splat(SCORCH_REACH) + size * 0.5)
                    .all()
        });
    if burning {
        hurt_player(world, player, enemies::HOUND_FIRE_DAMAGE, sounds);
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
    enraged: bool,
    command: Cmd,
) {
    let room = world.get::<&RoomId>(entity).map(|room| *room).ok();
    let mirrored = world
        .get::<&Animation>(entity)
        .is_ok_and(|animation| animation.flipped());
    match command {
        Cmd::Play(clip) => play(world, entity, clip),
        Cmd::Root => {
            let _ = world.insert_one(entity, Rooted);
        }
        Cmd::Free => {
            let _ = world.remove_one::<Rooted>(entity);
        }
        Cmd::Charge(on) => {
            if let Ok(mut hound) = world.get::<&mut HoundBoss>(entity) {
                hound.charging = on;
                hound.pulse = 0.0;
            }
            if let Ok(mut chase) = world.get::<&mut Chase>(entity) {
                if on {
                    chase.speed *= enemies::HOUND_LUNGE_SPEED;
                } else {
                    chase.speed /= enemies::HOUND_LUNGE_SPEED;
                }
            }
            if on {
                sounds.push(AudioCommand::Effect(SoundEffect::Explosion));
            }
        }
        Cmd::Breathe => {
            let heads = if enraged { 3 } else { 2 };
            let mut seed = (at.x as u32) ^ (at.y as u32).rotate_left(5) ^ 0xA5A5;
            for index in 0..heads {
                let origin = at + head_offset(index, mirrored);
                let aim = player.map_or(Vec2::X, |player| player - origin).to_angle();
                let jitter = (unit(&mut seed) - 0.5) * 2.0 * enemies::HOUND_BREATH_SPREAD;
                let bullet = prefabs::enemy_bullet(
                    world,
                    physics,
                    ProjectileKind::Fireball,
                    origin,
                    Vec2::from_angle(aim + jitter),
                    textures,
                );
                let _ = world.insert_one(
                    bullet,
                    crate::components::ContactDamage(enemies::HOUND_FIRE_DAMAGE),
                );
                if let Some(room) = room {
                    let _ = world.insert_one(bullet, room);
                }
            }
        }
        Cmd::Ring => {
            sounds.push(AudioCommand::Effect(SoundEffect::Explosion));
            let count = if enraged {
                enemies::HOUND_HOWL_RAGE_RING
            } else {
                enemies::HOUND_HOWL_RING
            };
            let origin = at + head_offset(1, mirrored);
            for index in 0..count {
                let angle = index as f32 / count as f32 * std::f32::consts::TAU;
                let bullet = prefabs::enemy_bullet(
                    world,
                    physics,
                    ProjectileKind::ArgusShot,
                    origin,
                    Vec2::from_angle(angle),
                    textures,
                );
                let _ = world.insert_one(
                    bullet,
                    crate::components::ContactDamage(enemies::HOUND_FIRE_DAMAGE),
                );
                if let Some(room) = room {
                    let _ = world.insert_one(bullet, room);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hound() -> HoundBoss {
        HoundBoss {
            stage: HoundStage::Prowl,
            attack: HoundAttack::Lunge,
            last: None,
            timer: 0.0,
            cooldown: 0.0,
            pulse: 0.0,
            charging: false,
            enraged: false,
            seed: 99,
            idle: clip(0, 0.07),
        }
    }

    fn run(hound: &mut HoundBoss, seconds: f32) -> Vec<Cmd> {
        let mut all = Vec::new();
        let mut time = 0.0;
        while time < seconds {
            all.extend(step(hound, 200.0, 0.02));
            time += 0.02;
        }
        all
    }

    #[test]
    fn attacks_never_repeat() {
        let mut boss = hound();
        let mut last = None;
        for _ in 0..200 {
            boss.last = last;
            let attack = choose_attack(&mut boss, 200.0);
            assert_ne!(Some(attack), last);
            last = Some(attack);
        }
    }

    #[test]
    fn a_lunge_charges_for_a_while_and_then_stops_and_rests() {
        let mut boss = hound();
        boss.stage = HoundStage::Winding;
        boss.attack = HoundAttack::Lunge;
        boss.last = Some(HoundAttack::Lunge);
        boss.timer = 0.01;
        let commands = run(&mut boss, 3.0);
        let on = commands
            .iter()
            .position(|command| *command == Cmd::Charge(true));
        let off = commands
            .iter()
            .position(|command| *command == Cmd::Charge(false));
        assert!(on.is_some() && off.is_some() && on < off);
        assert_eq!(boss.stage, HoundStage::Prowl);
    }

    #[test]
    fn a_breath_fires_many_pulses_and_a_howl_one_ring() {
        let mut boss = hound();
        boss.stage = HoundStage::Winding;
        boss.attack = HoundAttack::Breath;
        boss.last = Some(HoundAttack::Breath);
        boss.timer = 0.01;
        let pulses = run(&mut boss, 2.0)
            .iter()
            .filter(|command| **command == Cmd::Breathe)
            .count();
        assert!(pulses >= 5, "{pulses}");
        let mut boss = hound();
        boss.stage = HoundStage::Winding;
        boss.attack = HoundAttack::Howl;
        boss.last = Some(HoundAttack::Howl);
        boss.timer = 0.01;
        let rings = run(&mut boss, 1.0)
            .iter()
            .filter(|command| **command == Cmd::Ring)
            .count();
        assert_eq!(rings, 1);
    }

    #[test]
    fn scorches_fade_and_vanish() {
        let mut world = World::new();
        world.spawn((
            Scorch {
                remaining: 0.1,
                total: 1.0,
            },
            Color(Vec4::ONE),
        ));
        let mut sounds = Vec::new();
        tick_scorches(&mut world, &mut sounds, 0.2);
        assert_eq!(world.query::<&Scorch>().iter().count(), 0);
    }
}
