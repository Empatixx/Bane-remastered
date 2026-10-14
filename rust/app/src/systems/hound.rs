use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Animation, BeamPart, Charge, Chase, Clip, Color, ContactDamage, Expiring, Frozen, Health,
    HoundAttack, HoundBoss, HoundStage, Layer, Position, RoomId, Rooted, Size, Spawning,
};
use crate::enemies::{self, ProjectileKind};
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::prefabs;
use crate::systems::golem::{
    BeamStyle, Ray, aim, beam_end, draw_beam, play, player_at, turn_towards, unit,
};

const SPRITE_SCALE: f32 = 2.4;
const CELL: Vec2 = Vec2::new(64.0, 42.0);
const HEADS: [Vec2; 3] = [
    Vec2::new(109.0, 33.0),
    Vec2::new(93.0, 4.0),
    Vec2::new(100.0, 55.0),
];
const RAGE_TINT: Vec4 = Vec4::new(1.0, 0.7, 0.62, 1.0);
const ATTACK_VAR: &str = "BANE_HOUND_ATTACK";
const FLAME_BEAM: BeamStyle = BeamStyle {
    glow: Vec4::new(1.0, 0.4, 0.1, 0.55),
    core: Vec4::new(1.0, 0.92, 0.55, 1.0),
    light: [255.0, 110.0, 30.0],
    spark: tex::HOUND_SPARK,
    damage: enemies::HOUND_BEAM_DAMAGE,
    wave: 9.0,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Cmd {
    Play(Clip),
    Root,
    Free,
    Charge(bool),
    AimBreath,
    Ring,
    Crash,
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
        (HoundAttack::Breath, if close { 4.0 } else { 2.0 }),
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

pub(crate) fn step(hound: &mut HoundBoss, distance: f32, crashed: bool, dt: f32) -> Vec<Cmd> {
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
            if attack == HoundAttack::Breath {
                commands.push(Cmd::AimBreath);
            }
        }
        HoundStage::Winding => {
            hound.timer -= dt;
            if hound.timer > 0.0 {
                return commands;
            }
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
            if crashed && hound.attack == HoundAttack::Lunge {
                commands.push(Cmd::Crash);
                hound.stage = HoundStage::Recovering;
                hound.timer = enemies::HOUND_CRASH_STUN;
                return commands;
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
    navigation: &Navigation,
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
        let crashed = world.get::<&Charge>(entity).is_ok_and(|charge| {
            navigation.is_blocked(
                at + charge.direction * enemies::HOUND_CRASH_REACH,
                Vec2::splat(48.0),
            )
        });
        let (commands, enraged) = {
            let Ok(mut hound) = world.get::<&mut HoundBoss>(entity) else {
                continue;
            };
            (step(&mut hound, distance, crashed, dt), hound.enraged)
        };
        for command in commands {
            execute(
                world, physics, textures, sounds, entity, at, target, enraged, command,
            );
        }
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
            set_charge(world, entity, on, at, player);
            if on {
                sounds.push(AudioCommand::Effect(SoundEffect::Explosion));
            }
        }
        Cmd::Crash => {
            set_charge(world, entity, false, at, player);
            let _ = world.insert_one(entity, Rooted);
            sounds.push(AudioCommand::Effect(SoundEffect::Explosion));
            sounds.push(AudioCommand::Effect(SoundEffect::WallHit));
        }
        Cmd::AimBreath => {
            let origin = at + head_offset(0, mirrored);
            if let Ok(mut hound) = world.get::<&mut HoundBoss>(entity) {
                hound.aim_at = player.unwrap_or(origin + Vec2::X * 200.0);
                hound.angle = aim(origin, hound.aim_at);
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
                    ProjectileKind::Ember,
                    origin,
                    Vec2::from_angle(angle),
                    textures,
                );
                let _ = world.insert_one(bullet, ContactDamage(enemies::HOUND_FIRE_DAMAGE));
                if let Some(room) = room {
                    let _ = world.insert_one(bullet, room);
                }
            }
        }
    }
}

fn set_charge(world: &mut World, entity: Entity, on: bool, at: Vec2, player: Option<Vec2>) {
    let charging = world
        .get::<&HoundBoss>(entity)
        .is_ok_and(|hound| hound.charging);
    if on == charging {
        return;
    }
    if let Ok(mut hound) = world.get::<&mut HoundBoss>(entity) {
        hound.charging = on;
    }
    if let Ok(mut chase) = world.get::<&mut Chase>(entity) {
        if on {
            chase.speed *= enemies::HOUND_LUNGE_SPEED;
        } else {
            chase.speed /= enemies::HOUND_LUNGE_SPEED;
        }
    }
    if on {
        let direction = player.map_or(Vec2::X, |player| (player - at).normalize_or_zero());
        let _ = world.insert_one(entity, Charge { direction });
    } else {
        let _ = world.remove_one::<Charge>(entity);
    }
}

pub(crate) fn beam_angle(base: f32, wobble: f32, head: usize, heads: usize) -> f32 {
    let t = if heads <= 1 {
        0.0
    } else {
        head as f32 / (heads - 1) as f32 - 0.5
    };
    base + wobble * enemies::HOUND_BREATH_WOBBLE + t * enemies::HOUND_BREATH_FAN * 2.0
}

fn warn(world: &mut World, navigation: &Navigation, ray: Ray) {
    let end = beam_end(navigation, ray.eye, ray.angle);
    let direction = Vec2::from_angle(ray.angle);
    let mut distance = 0.0;
    while distance <= ray.eye.distance(end) {
        world.spawn((
            BeamPart,
            Position(ray.eye + direction * distance),
            Size(Vec2::splat(6.0)),
            Color(Vec4::new(1.0, 0.5, 0.15, 0.7)),
            Layer(4),
        ));
        distance += 22.0;
    }
}

pub(crate) fn tick_flames(
    world: &mut World,
    navigation: &Navigation,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    let target = player_at(world).map(|(_, position, _)| position);
    let active: Vec<(Entity, Vec2, bool, bool, f32)> = world
        .query::<(Entity, &Position, &HoundBoss)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .iter()
        .filter(|(_, _, hound)| {
            hound.attack == HoundAttack::Breath
                && matches!(hound.stage, HoundStage::Winding | HoundStage::Acting)
        })
        .map(|(entity, position, hound)| {
            (
                entity,
                position.0,
                hound.stage == HoundStage::Acting,
                hound.enraged,
                hound.timer,
            )
        })
        .collect();
    for (entity, at, firing, enraged, timer) in active {
        let mirrored = world
            .get::<&Animation>(entity)
            .is_ok_and(|animation| animation.flipped());
        let main = at + head_offset(0, mirrored);
        let angle = {
            let Ok(mut hound) = world.get::<&mut HoundBoss>(entity) else {
                continue;
            };
            if let Some(target) = target {
                hound.aim_at = target;
            }
            let rate = if enraged {
                enemies::HOUND_BEAM_RAGE_TURN
            } else {
                enemies::HOUND_BEAM_TURN
            };
            let wanted = aim(main, hound.aim_at);
            let smooth = 1.0 - (-rate * 2.2 * dt).exp();
            let step = turn_towards(hound.angle, wanted, f32::MAX) - hound.angle;
            hound.angle += (step * smooth).clamp(-rate * dt, rate * dt);
            hound.angle
        };
        let heads = if enraged { 3 } else { 2 };
        let wobble = (timer * 4.0).sin();
        for head in 0..heads {
            let ray = Ray {
                eye: at + head_offset(head, mirrored),
                angle: beam_angle(angle, wobble, head, heads),
                phase: timer * 16.0 + head as f32,
            };
            if firing {
                draw_beam(world, navigation, textures, sounds, ray, FLAME_BEAM);
            } else {
                warn(world, navigation, ray);
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
            angle: 0.0,
            aim_at: Vec2::ZERO,
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
            all.extend(step(hound, 200.0, false, 0.02));
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
    fn crashing_into_a_wall_ends_the_lunge_and_stuns_the_hound_longer() {
        let mut boss = hound();
        boss.stage = HoundStage::Acting;
        boss.attack = HoundAttack::Lunge;
        boss.timer = 1.0;
        boss.charging = true;
        let commands = step(&mut boss, 200.0, true, 0.02);
        assert!(commands.contains(&Cmd::Crash));
        assert_eq!(boss.stage, HoundStage::Recovering);
        assert!(boss.timer > enemies::HOUND_LUNGE_RECOVER);
    }

    #[test]
    fn a_breath_holds_its_stage_for_the_whole_beam_and_a_howl_rings_once() {
        let mut boss = hound();
        boss.stage = HoundStage::Winding;
        boss.attack = HoundAttack::Breath;
        boss.last = Some(HoundAttack::Breath);
        boss.timer = 0.01;
        run(&mut boss, 1.0);
        assert_eq!(boss.stage, HoundStage::Acting);
        run(&mut boss, 3.0);
        assert_eq!(boss.stage, HoundStage::Prowl);
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
    fn the_flame_beams_wobble_and_fan_out() {
        let start = beam_angle(0.0, -1.0, 1, 3);
        let end = beam_angle(0.0, 1.0, 1, 3);
        assert!(end - start > 0.2);
        let left = beam_angle(0.0, 0.0, 0, 3);
        let right = beam_angle(0.0, 0.0, 2, 3);
        assert!(right - left > 0.5);
    }
}
