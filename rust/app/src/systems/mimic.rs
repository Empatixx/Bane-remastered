use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Animation, Bait, Charge, Chase, Clip, Color, Expiring, Frozen, Health, Layer, MimicAttack,
    MimicBoss, MimicStage, Minion, Position, RoomId, Rooted, Sealed, Size, Spawning, Sprite,
};
use crate::enemies::{self, EnemyKind, ProjectileKind};
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::prefabs;
use crate::systems::damage::hurt_player;
use crate::systems::golem::{play, player_at, spark, spawn_rock, unit};

const RAGE_TINT: Vec4 = Vec4::new(1.0, 0.72, 0.66, 1.0);
const MOUTH: Vec2 = Vec2::new(44.0, 29.0);
const FOOT: f32 = 85.0;
const ATTACK_VAR: &str = "BANE_MIMIC_ATTACK";

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Cmd {
    Play(Clip),
    Root,
    Free,
    Charge(bool, f32),
    Ring,
    Fan,
    Spit,
    Lure,
    Shield(bool),
}

fn clip(row: usize) -> Clip {
    Clip {
        row,
        frames: 8,
        frame_time: 0.1,
        looping: false,
        flipped: false,
    }
}

fn forced_attack() -> Option<MimicAttack> {
    match std::env::var(ATTACK_VAR)
        .ok()?
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "hop" => Some(MimicAttack::Hop),
        "bite" => Some(MimicAttack::Bite),
        "spit" => Some(MimicAttack::Spit),
        "lure" => Some(MimicAttack::Lure),
        "rest" => Some(MimicAttack::Rest),
        _ => None,
    }
}

pub(crate) fn choose_attack(
    mimic: &mut MimicBoss,
    distance: f32,
    mobs: usize,
    baits: usize,
) -> MimicAttack {
    if let Some(attack) = forced_attack() {
        return attack;
    }
    let close = distance < enemies::MIMIC_NEAR;
    let crowded = mobs >= enemies::MIMIC_MOB_LIMIT;
    let weights = [
        (MimicAttack::Hop, if close { 2.0 } else { 4.0 }),
        (MimicAttack::Bite, if close { 4.0 } else { 0.5 }),
        (MimicAttack::Spit, if crowded { 0.0 } else { 3.0 }),
        (
            MimicAttack::Lure,
            if baits >= enemies::MIMIC_BAIT_LIMIT {
                0.0
            } else {
                2.5
            },
        ),
        (MimicAttack::Rest, 1.5),
    ];
    let allowed = |attack: MimicAttack, weight: f32| weight > 0.0 && Some(attack) != mimic.last;
    let total: f32 = weights
        .iter()
        .filter(|(attack, weight)| allowed(*attack, *weight))
        .map(|(_, weight)| weight)
        .sum();
    let mut roll = unit(&mut mimic.seed) * total;
    for (attack, weight) in weights {
        if !allowed(attack, weight) {
            continue;
        }
        if roll < weight {
            return attack;
        }
        roll -= weight;
    }
    MimicAttack::Hop
}

fn speed_of(attack: MimicAttack) -> f32 {
    match attack {
        MimicAttack::Hop => enemies::MIMIC_HOP_SPEED,
        _ => enemies::MIMIC_BITE_SPEED,
    }
}

pub(crate) fn step(
    mimic: &mut MimicBoss,
    distance: f32,
    mobs: usize,
    baits: usize,
    dt: f32,
) -> Vec<Cmd> {
    let mut commands = Vec::new();
    match mimic.stage {
        MimicStage::Walk => {
            mimic.cooldown -= dt;
            if mimic.cooldown > 0.0 {
                return commands;
            }
            let attack = choose_attack(mimic, distance, mobs, baits);
            mimic.attack = attack;
            mimic.last = Some(attack);
            mimic.stage = MimicStage::Winding;
            commands.push(Cmd::Root);
            let (row, windup) = match attack {
                MimicAttack::Hop => (1, enemies::MIMIC_HOP_WINDUP),
                MimicAttack::Bite => (2, enemies::MIMIC_BITE_WINDUP),
                MimicAttack::Spit => (3, enemies::MIMIC_SPIT_WINDUP),
                MimicAttack::Lure => (3, enemies::MIMIC_LURE_WINDUP),
                MimicAttack::Rest => (4, enemies::MIMIC_REST_WINDUP),
            };
            mimic.timer = windup;
            commands.push(Cmd::Play(clip(row)));
        }
        MimicStage::Winding => {
            mimic.timer -= dt;
            if mimic.timer > 0.0 {
                return commands;
            }
            match mimic.attack {
                MimicAttack::Hop | MimicAttack::Bite => {
                    mimic.stage = MimicStage::Acting;
                    mimic.timer = if mimic.attack == MimicAttack::Hop {
                        enemies::MIMIC_HOP_SECONDS
                    } else {
                        enemies::MIMIC_BITE_SECONDS
                    };
                    commands.push(Cmd::Free);
                    commands.push(Cmd::Charge(true, speed_of(mimic.attack)));
                }
                MimicAttack::Spit => {
                    commands.push(Cmd::Spit);
                    mimic.stage = MimicStage::Recovering;
                    mimic.timer = enemies::MIMIC_SPIT_RECOVER;
                }
                MimicAttack::Lure => {
                    commands.push(Cmd::Lure);
                    mimic.stage = MimicStage::Recovering;
                    mimic.timer = enemies::MIMIC_LURE_RECOVER;
                }
                MimicAttack::Rest => {
                    commands.push(Cmd::Shield(true));
                    mimic.stage = MimicStage::Acting;
                    mimic.timer = enemies::MIMIC_REST_SECONDS;
                }
            }
        }
        MimicStage::Acting => {
            mimic.timer -= dt;
            if mimic.timer > 0.0 {
                return commands;
            }
            let recover = match mimic.attack {
                MimicAttack::Hop => {
                    commands.push(Cmd::Charge(false, speed_of(mimic.attack)));
                    commands.push(Cmd::Root);
                    commands.push(Cmd::Ring);
                    enemies::MIMIC_HOP_RECOVER
                }
                MimicAttack::Bite => {
                    commands.push(Cmd::Charge(false, speed_of(mimic.attack)));
                    commands.push(Cmd::Root);
                    commands.push(Cmd::Fan);
                    enemies::MIMIC_BITE_RECOVER
                }
                _ => {
                    commands.push(Cmd::Shield(false));
                    commands.push(Cmd::Play(clip(0)));
                    enemies::MIMIC_REST_RECOVER
                }
            };
            mimic.stage = MimicStage::Recovering;
            mimic.timer = recover;
        }
        MimicStage::Recovering => {
            mimic.timer -= dt;
            if mimic.timer <= 0.0 {
                commands.push(Cmd::Play(mimic.idle));
                commands.push(Cmd::Free);
                mimic.stage = MimicStage::Walk;
                mimic.cooldown = if mimic.enraged {
                    enemies::MIMIC_RAGE_COOLDOWN
                } else {
                    enemies::MIMIC_COOLDOWN
                };
            }
        }
    }
    commands
}

pub(crate) fn tick_mimic(
    world: &mut World,
    physics: &mut Physics,
    navigation: &Navigation,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    let target = player_at(world).map(|(_, position, _)| position);
    let mobs = world.query::<&Minion>().iter().count();
    let baits = world.query::<&Bait>().iter().count();
    let bosses: Vec<(Entity, Vec2, i32, i32)> = world
        .query::<(Entity, &Position, &Health, &MimicBoss)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .without::<&Frozen>()
        .iter()
        .map(|(entity, position, health, _)| (entity, position.0, health.current, health.max))
        .collect();
    for (entity, at, current, max) in bosses {
        let calm = world
            .get::<&MimicBoss>(entity)
            .is_ok_and(|mimic| !mimic.enraged);
        if calm && current as f32 <= max as f32 * enemies::MIMIC_RAGE_AT {
            if let Ok(mut mimic) = world.get::<&mut MimicBoss>(entity) {
                mimic.enraged = true;
            }
            if let Ok(mut chase) = world.get::<&mut Chase>(entity) {
                chase.speed *= enemies::MIMIC_RAGE_SPEED;
            }
            if let Ok(mut color) = world.get::<&mut Color>(entity) {
                color.0 = RAGE_TINT;
            }
        }
        let distance = target.map_or(f32::MAX, |player| player.distance(at));
        let commands = {
            let Ok(mut mimic) = world.get::<&mut MimicBoss>(entity) else {
                continue;
            };
            step(&mut mimic, distance, mobs, baits, dt)
        };
        for command in commands {
            execute(
                world, physics, navigation, textures, sounds, entity, at, target, command,
            );
        }
    }
}

fn set_charge(
    world: &mut World,
    entity: Entity,
    on: bool,
    factor: f32,
    at: Vec2,
    player: Option<Vec2>,
) {
    let charging = world
        .get::<&MimicBoss>(entity)
        .is_ok_and(|mimic| mimic.charging);
    if on == charging {
        return;
    }
    if let Ok(mut mimic) = world.get::<&mut MimicBoss>(entity) {
        mimic.charging = on;
    }
    if let Ok(mut chase) = world.get::<&mut Chase>(entity) {
        if on {
            chase.speed *= factor;
        } else {
            chase.speed /= factor;
        }
    }
    if on {
        let direction = player.map_or(Vec2::X, |player| (player - at).normalize_or_zero());
        let _ = world.insert_one(entity, Charge { direction });
    } else {
        let _ = world.remove_one::<Charge>(entity);
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
    let mirrored = world
        .get::<&Animation>(entity)
        .is_ok_and(|animation| animation.flipped());
    let side = if mirrored { -1.0 } else { 1.0 };
    match command {
        Cmd::Play(clip) => play(world, entity, clip),
        Cmd::Root => {
            let _ = world.insert_one(entity, Rooted);
        }
        Cmd::Free => {
            let _ = world.remove_one::<Rooted>(entity);
        }
        Cmd::Charge(on, factor) => {
            set_charge(world, entity, on, factor, at, player);
            if on {
                sounds.push(AudioCommand::Effect(SoundEffect::Explosion));
            }
        }
        Cmd::Shield(on) => {
            if on {
                let _ = world.insert_one(entity, Sealed);
                let _ = world.insert_one(entity, Rooted);
            } else {
                let _ = world.remove_one::<Sealed>(entity);
            }
        }
        Cmd::Ring => {
            sounds.push(AudioCommand::Effect(SoundEffect::Explosion));
            let origin = at + Vec2::new(0.0, FOOT);
            for index in 0..enemies::MIMIC_HOP_RING {
                let angle = index as f32 / enemies::MIMIC_HOP_RING as f32 * std::f32::consts::TAU;
                spawn_rock(
                    world,
                    physics,
                    ProjectileKind::MimicCoin,
                    origin,
                    Vec2::from_angle(angle),
                    enemies::MIMIC_SPLINTER_DAMAGE,
                    room,
                    textures,
                );
            }
        }
        Cmd::Fan => {
            sounds.push(AudioCommand::Effect(SoundEffect::Explosion));
            let origin = at + Vec2::new(side * MOUTH.x, MOUTH.y);
            let aim = player.map_or(Vec2::X, |player| player - origin).to_angle();
            for index in 0..enemies::MIMIC_BITE_FAN {
                let t = index as f32 / (enemies::MIMIC_BITE_FAN - 1) as f32 - 0.5;
                spawn_rock(
                    world,
                    physics,
                    ProjectileKind::MimicTooth,
                    origin,
                    Vec2::from_angle(aim + t * enemies::MIMIC_BITE_SPREAD * 2.0),
                    enemies::MIMIC_SPLINTER_DAMAGE,
                    room,
                    textures,
                );
            }
        }
        Cmd::Spit => {
            let spot = at + Vec2::new(side * (MOUTH.x + 60.0), MOUTH.y);
            prefabs::hatchling(world, physics, EnemyKind::SalivaSlime, spot, room, textures);
            sounds.push(AudioCommand::Effect(SoundEffect::WallHit));
            let mouth = at + Vec2::new(side * MOUTH.x, MOUTH.y);
            let mut seed = (at.x as u32).wrapping_mul(2_246_822_519) ^ (at.y as u32);
            for _ in 0..16 {
                let angle = (unit(&mut seed) - 0.5) * 1.5;
                let speed = 140.0 + unit(&mut seed) * 260.0;
                let velocity = Vec2::new(side * angle.cos(), angle.sin() - 0.35) * speed;
                spark(world, textures, tex::MIMIC_DROOL, mouth, velocity);
            }
        }
        Cmd::Lure => {
            let Some(player) = player else {
                return;
            };
            let mut seed = (at.x as u32).wrapping_mul(2_654_435_761) ^ (player.y as u32);
            for index in 0..enemies::MIMIC_BAITS {
                let angle = (index as f32 + unit(&mut seed) * 0.6) / enemies::MIMIC_BAITS as f32
                    * std::f32::consts::TAU;
                let spot = player + Vec2::from_angle(angle) * (120.0 + unit(&mut seed) * 120.0);
                if navigation.is_blocked(spot, Vec2::splat(48.0)) {
                    continue;
                }
                spawn_bait(world, spot, textures);
            }
        }
    }
}

fn spawn_bait(world: &mut World, at: Vec2, textures: &Textures) -> Entity {
    let idle = Clip {
        row: 0,
        frames: 4,
        frame_time: 0.18,
        looping: true,
        flipped: false,
    };
    world.spawn((
        Bait {
            remaining: enemies::MIMIC_BAIT_SECONDS,
        },
        Position(at),
        Size(Vec2::splat(46.0)),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(tex::MIMIC_BAIT), 0, 0, 4, 2),
        Animation::new(4, 2, idle),
        Layer(0),
    ))
}

pub(crate) fn tick_baits(
    world: &mut World,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    let player = player_at(world);
    let mut gone = Vec::new();
    let mut snapped = Vec::new();
    for (entity, bait, position) in world.query_mut::<(Entity, &mut Bait, &Position)>() {
        bait.remaining -= dt;
        if bait.remaining <= 0.0 {
            gone.push(entity);
        } else if player
            .is_some_and(|(_, at, _)| at.distance(position.0) < enemies::MIMIC_BAIT_REACH)
        {
            snapped.push((entity, position.0));
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
    for (entity, at) in snapped {
        let _ = world.remove_one::<Bait>(entity);
        let snap = Clip {
            row: 1,
            frames: 4,
            frame_time: 0.08,
            looping: false,
            flipped: false,
        };
        let _ = world.insert_one(entity, Expiring);
        if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
            animation.play(snap);
        }
        let _ = (at, textures);
        if let Some((player, _, _)) = player {
            hurt_player(world, player, enemies::MIMIC_BAIT_DAMAGE, sounds);
        }
    }
}

pub(crate) fn clear_baits(world: &mut World) {
    let baits: Vec<Entity> = world.query::<Entity>().with::<&Bait>().iter().collect();
    for bait in baits {
        let _ = world.despawn(bait);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mimic() -> MimicBoss {
        MimicBoss {
            stage: MimicStage::Walk,
            attack: MimicAttack::Hop,
            last: None,
            timer: 0.0,
            cooldown: 0.0,
            charging: false,
            enraged: false,
            seed: 31,
            idle: clip(0),
        }
    }

    fn run(mimic: &mut MimicBoss, seconds: f32) -> Vec<Cmd> {
        let mut all = Vec::new();
        let mut time = 0.0;
        while time < seconds {
            all.extend(step(mimic, 200.0, 0, 0, 0.02));
            time += 0.02;
        }
        all
    }

    fn winding(attack: MimicAttack) -> MimicBoss {
        let mut boss = mimic();
        boss.stage = MimicStage::Winding;
        boss.attack = attack;
        boss.last = Some(attack);
        boss.timer = 0.01;
        boss
    }

    #[test]
    fn attacks_never_repeat_and_a_full_room_stops_the_spitting() {
        let mut boss = mimic();
        let mut last = None;
        for _ in 0..200 {
            boss.last = last;
            let attack = choose_attack(&mut boss, 200.0, 0, 0);
            assert_ne!(Some(attack), last);
            last = Some(attack);
        }
        boss.last = None;
        for _ in 0..200 {
            assert_ne!(
                choose_attack(&mut boss, 200.0, enemies::MIMIC_MOB_LIMIT, 0),
                MimicAttack::Spit
            );
        }
    }

    #[test]
    fn a_hop_charges_then_lands_with_a_ring() {
        let mut boss = winding(MimicAttack::Hop);
        let commands = run(&mut boss, 2.0);
        let on = commands
            .iter()
            .position(|command| matches!(command, Cmd::Charge(true, _)));
        let ring = commands.iter().position(|command| *command == Cmd::Ring);
        assert!(on.is_some() && ring.is_some() && on < ring);
        assert_eq!(boss.stage, MimicStage::Walk);
    }

    #[test]
    fn a_bite_lunges_and_fires_a_fan_and_a_spit_calls_one_mob() {
        let mut boss = winding(MimicAttack::Bite);
        assert!(run(&mut boss, 2.0).contains(&Cmd::Fan));
        let mut boss = winding(MimicAttack::Spit);
        let spits = run(&mut boss, 2.0)
            .iter()
            .filter(|command| **command == Cmd::Spit)
            .count();
        assert_eq!(spits, 1);
    }

    #[test]
    fn a_lure_scatters_baits_once_and_baits_snap_when_touched() {
        let mut boss = winding(MimicAttack::Lure);
        let lures = run(&mut boss, 2.0)
            .iter()
            .filter(|command| **command == Cmd::Lure)
            .count();
        assert_eq!(lures, 1);
        let mut world = World::new();
        let textures = Textures::default();
        let bait = spawn_bait(&mut world, Vec2::new(100.0, 100.0), &textures);
        let mut sounds = Vec::new();
        tick_baits(&mut world, &textures, &mut sounds, 0.1);
        assert!(world.get::<&Bait>(bait).is_ok());
        world.despawn(bait).ok();
        clear_baits(&mut world);
        assert_eq!(world.query::<&Bait>().iter().count(), 0);
    }

    #[test]
    fn resting_shields_the_chest_for_a_while_and_then_drops_it() {
        let mut boss = winding(MimicAttack::Rest);
        let commands = run(&mut boss, 4.0);
        let on = commands
            .iter()
            .position(|command| *command == Cmd::Shield(true));
        let off = commands
            .iter()
            .position(|command| *command == Cmd::Shield(false));
        assert!(on.is_some() && off.is_some() && on < off);
    }
}
