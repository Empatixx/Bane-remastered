use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Chase, Clip, Color, Expiring, Frozen, Health, Minion, OgreAttack, OgreBoss, OgreStage,
    Position, RoomId, Rooted, Spawning,
};
use crate::enemies::{self, EnemyKind, ProjectileKind};
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::prefabs;
use crate::systems::golem::{play, player_at, spawn_rock, unit};

const RAGE_TINT: Vec4 = Vec4::new(1.0, 0.7, 0.62, 1.0);
const RING_DROP: f32 = 110.0;
const HAND: Vec2 = Vec2::new(-110.0, -10.0);
const RAT_RADIUS: f32 = 70.0;
const ATTACK_VAR: &str = "BANE_OGRE_ATTACK";

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Cmd {
    Play(Clip),
    Root,
    Free,
    Ring,
    Fan,
    Rats(usize),
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

fn forced_attack() -> Option<OgreAttack> {
    match std::env::var(ATTACK_VAR)
        .ok()?
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "stomp" => Some(OgreAttack::Stomp),
        "throw" => Some(OgreAttack::Throw),
        "slam" => Some(OgreAttack::Slam),
        "roar" => Some(OgreAttack::Roar),
        _ => None,
    }
}

pub(crate) fn choose_attack(ogre: &mut OgreBoss, distance: f32, rats: usize) -> OgreAttack {
    if let Some(attack) = forced_attack() {
        return attack;
    }
    let close = distance < enemies::OGRE_NEAR;
    let crowded = rats >= enemies::OGRE_RAT_LIMIT;
    let weights = [
        (OgreAttack::Stomp, if close { 4.0 } else { 1.0 }),
        (OgreAttack::Slam, if close { 3.0 } else { 1.0 }),
        (
            OgreAttack::Throw,
            if crowded {
                0.0
            } else if close {
                1.5
            } else {
                3.0
            },
        ),
        (OgreAttack::Roar, if crowded { 0.0 } else { 2.0 }),
    ];
    let allowed = |attack: OgreAttack, weight: f32| weight > 0.0 && Some(attack) != ogre.last;
    let total: f32 = weights
        .iter()
        .filter(|(attack, weight)| allowed(*attack, *weight))
        .map(|(_, weight)| weight)
        .sum();
    let mut roll = unit(&mut ogre.seed) * total;
    for (attack, weight) in weights {
        if !allowed(attack, weight) {
            continue;
        }
        if roll < weight {
            return attack;
        }
        roll -= weight;
    }
    OgreAttack::Stomp
}

pub(crate) fn step(ogre: &mut OgreBoss, distance: f32, rats: usize, dt: f32) -> Vec<Cmd> {
    let mut commands = Vec::new();
    match ogre.stage {
        OgreStage::Walk => {
            ogre.cooldown -= dt;
            if ogre.cooldown <= 0.0 {
                let attack = choose_attack(ogre, distance, rats);
                ogre.attack = attack;
                ogre.last = Some(attack);
                ogre.stage = OgreStage::Winding;
                commands.push(Cmd::Root);
                let (row, windup) = match attack {
                    OgreAttack::Stomp => (1, enemies::OGRE_STOMP_WINDUP),
                    OgreAttack::Throw => (2, enemies::OGRE_THROW_WINDUP),
                    OgreAttack::Slam => (3, enemies::OGRE_SLAM_WINDUP),
                    OgreAttack::Roar => (4, enemies::OGRE_ROAR_WINDUP),
                };
                ogre.timer = windup;
                commands.push(Cmd::Play(clip(row)));
            }
        }
        OgreStage::Winding => {
            ogre.timer -= dt;
            if ogre.timer <= 0.0 {
                let (command, recover) = match ogre.attack {
                    OgreAttack::Stomp => (Cmd::Ring, enemies::OGRE_STOMP_RECOVER),
                    OgreAttack::Throw => (
                        Cmd::Rats(enemies::OGRE_THROW_RATS),
                        enemies::OGRE_THROW_RECOVER,
                    ),
                    OgreAttack::Slam => (Cmd::Fan, enemies::OGRE_SLAM_RECOVER),
                    OgreAttack::Roar => (
                        Cmd::Rats(enemies::OGRE_ROAR_RATS),
                        enemies::OGRE_ROAR_RECOVER,
                    ),
                };
                commands.push(command);
                ogre.stage = OgreStage::Recovering;
                ogre.timer = recover;
            }
        }
        OgreStage::Recovering => {
            ogre.timer -= dt;
            if ogre.timer <= 0.0 {
                commands.push(Cmd::Play(ogre.idle));
                commands.push(Cmd::Free);
                ogre.stage = OgreStage::Walk;
                ogre.cooldown = if ogre.enraged {
                    enemies::OGRE_RAGE_COOLDOWN
                } else {
                    enemies::OGRE_COOLDOWN
                };
            }
        }
    }
    commands
}

pub(crate) fn tick_ogre(
    world: &mut World,
    physics: &mut Physics,
    navigation: &Navigation,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    let target = player_at(world).map(|(_, position, _)| position);
    let rats = world.query::<&Minion>().iter().count();
    let bosses: Vec<(Entity, Vec2, i32, i32)> = world
        .query::<(Entity, &Position, &Health, &OgreBoss)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .without::<&Frozen>()
        .iter()
        .map(|(entity, position, health, _)| (entity, position.0, health.current, health.max))
        .collect();
    for (entity, at, current, max) in bosses {
        let calm = world
            .get::<&OgreBoss>(entity)
            .is_ok_and(|ogre| !ogre.enraged);
        if calm && current as f32 <= max as f32 * enemies::OGRE_RAGE_AT {
            if let Ok(mut ogre) = world.get::<&mut OgreBoss>(entity) {
                ogre.enraged = true;
            }
            if let Ok(mut chase) = world.get::<&mut Chase>(entity) {
                chase.speed *= enemies::OGRE_RAGE_SPEED;
            }
            if let Ok(mut color) = world.get::<&mut Color>(entity) {
                color.0 = RAGE_TINT;
            }
        }
        let distance = target.map_or(f32::MAX, |player| player.distance(at));
        let commands = {
            let Ok(mut ogre) = world.get::<&mut OgreBoss>(entity) else {
                continue;
            };
            step(&mut ogre, distance, rats, dt)
        };
        for command in commands {
            execute(
                world, physics, navigation, textures, sounds, entity, at, target, command,
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
        Cmd::Ring => {
            sounds.push(AudioCommand::Effect(SoundEffect::Explosion));
            let origin = at + Vec2::new(0.0, RING_DROP);
            for index in 0..enemies::OGRE_STOMP_RING {
                let angle = index as f32 / enemies::OGRE_STOMP_RING as f32 * std::f32::consts::TAU;
                spawn_rock(
                    world,
                    physics,
                    ProjectileKind::GolemRock,
                    origin,
                    Vec2::from_angle(angle),
                    enemies::OGRE_STOMP_DAMAGE,
                    room,
                    textures,
                );
            }
        }
        Cmd::Fan => {
            sounds.push(AudioCommand::Effect(SoundEffect::Explosion));
            let aim = player.map_or(Vec2::Y, |player| player - at).to_angle();
            let origin = at + Vec2::new(0.0, RING_DROP * 0.6);
            for index in 0..enemies::OGRE_SLAM_FAN {
                let t = index as f32 / (enemies::OGRE_SLAM_FAN - 1) as f32 - 0.5;
                spawn_rock(
                    world,
                    physics,
                    ProjectileKind::GolemRock,
                    origin,
                    Vec2::from_angle(aim + t * enemies::OGRE_SLAM_SPREAD * 2.0),
                    enemies::OGRE_STOMP_DAMAGE,
                    room,
                    textures,
                );
            }
        }
        Cmd::Rats(count) => {
            let mut seed = (at.x as u32).wrapping_mul(2_654_435_761) ^ (at.y as u32);
            let toward = player.map_or(Vec2::X, |player| (player - at).normalize_or_zero());
            for index in 0..count {
                let angle = toward.to_angle()
                    + (index as f32 - (count as f32 - 1.0) / 2.0) * 0.7
                    + (unit(&mut seed) - 0.5) * 0.3;
                let spot = at + HAND + Vec2::from_angle(angle) * RAT_RADIUS;
                if navigation.is_blocked(spot, Vec2::splat(40.0)) {
                    continue;
                }
                prefabs::hatchling(world, physics, EnemyKind::Rat, spot, room, textures);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ogre() -> OgreBoss {
        OgreBoss {
            stage: OgreStage::Walk,
            attack: OgreAttack::Stomp,
            last: None,
            timer: 0.0,
            cooldown: 0.0,
            enraged: false,
            seed: 4242,
            idle: clip(0),
        }
    }

    fn run(ogre: &mut OgreBoss, seconds: f32) -> Vec<Cmd> {
        let mut all = Vec::new();
        let mut time = 0.0;
        while time < seconds {
            all.extend(step(ogre, 200.0, 0, 0.02));
            time += 0.02;
        }
        all
    }

    #[test]
    fn attacks_never_repeat_and_rats_stop_at_the_limit() {
        let mut boss = ogre();
        let mut last = None;
        for _ in 0..200 {
            boss.last = last;
            let attack = choose_attack(&mut boss, 200.0, 0);
            assert_ne!(Some(attack), last);
            last = Some(attack);
        }
        boss.last = None;
        for _ in 0..200 {
            let attack = choose_attack(&mut boss, 600.0, enemies::OGRE_RAT_LIMIT);
            assert!(matches!(attack, OgreAttack::Stomp | OgreAttack::Slam));
        }
    }

    #[test]
    fn a_stomp_roots_rings_and_frees_the_ogre_again() {
        let mut boss = ogre();
        boss.stage = OgreStage::Winding;
        boss.attack = OgreAttack::Stomp;
        boss.last = Some(OgreAttack::Stomp);
        boss.timer = 0.01;
        let commands = run(&mut boss, 1.0);
        assert!(commands.contains(&Cmd::Ring));
        assert!(commands.contains(&Cmd::Free));
        assert_eq!(boss.stage, OgreStage::Walk);
    }

    #[test]
    fn a_roar_and_a_throw_call_the_rats() {
        for (attack, count) in [
            (OgreAttack::Roar, enemies::OGRE_ROAR_RATS),
            (OgreAttack::Throw, enemies::OGRE_THROW_RATS),
        ] {
            let mut boss = ogre();
            boss.stage = OgreStage::Winding;
            boss.attack = attack;
            boss.last = Some(attack);
            boss.timer = 0.01;
            assert!(run(&mut boss, 0.5).contains(&Cmd::Rats(count)));
        }
    }

    #[test]
    fn the_club_slam_fires_a_fan() {
        let mut boss = ogre();
        boss.stage = OgreStage::Winding;
        boss.attack = OgreAttack::Slam;
        boss.last = Some(OgreAttack::Slam);
        boss.timer = 0.01;
        assert!(run(&mut boss, 0.5).contains(&Cmd::Fan));
    }
}
