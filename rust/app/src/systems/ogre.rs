use glam::Vec2;
use hecs::{Entity, Satisfies, World};

use crate::assets::Textures;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Animation, Clip, Enraged, Minion, OgreAttack, OgreBoss, OgreStage, Position, RoomId, Rooted,
};
use crate::enemies::{self, EnemyKind, ProjectileKind};
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::prefabs;
use crate::systems::boss_kit::{self, play, spawn_rock};
use crate::systems::player::living_player_at;
use crate::systems::fx::{self, Fx};
use crate::systems::hostile::{Hostile, Inactive};

const RING_DROP: f32 = 110.0;
const CLUB_REACH: Vec2 = Vec2::new(91.0, 133.0);
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
    Sound(SoundEffect),
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
    static FORCED: std::sync::OnceLock<Option<OgreAttack>> = std::sync::OnceLock::new();
    *FORCED.get_or_init(|| {
        boss_kit::forced_attack(
            ATTACK_VAR,
            &[
                ("jump", OgreAttack::Jump),
                ("stomp", OgreAttack::Jump),
                ("throw", OgreAttack::Throw),
                ("slam", OgreAttack::Slam),
                ("roar", OgreAttack::Roar),
            ],
        )
    })
}

pub(crate) fn choose_attack(ogre: &mut OgreBoss, distance: f32, rats: usize) -> OgreAttack {
    if let Some(attack) = forced_attack() {
        return attack;
    }
    let close = distance < enemies::OGRE_NEAR;
    let crowded = rats >= enemies::OGRE_RAT_LIMIT;
    let weights = [
        (OgreAttack::Jump, if close { 4.0 } else { 1.0 }),
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
    boss_kit::pick_attack(&mut ogre.brain, &weights, OgreAttack::Jump)
}

pub(crate) fn step(
    ogre: &mut OgreBoss,
    enraged: bool,
    distance: f32,
    rats: usize,
    dt: f32,
) -> Vec<Cmd> {
    let mut commands = Vec::new();
    match ogre.brain.stage {
        OgreStage::Walk => {
            ogre.brain.cooldown -= dt;
            if ogre.brain.cooldown <= 0.0 {
                let attack = choose_attack(ogre, distance, rats);
                ogre.brain.attack = attack;
                ogre.brain.last = Some(attack);
                ogre.brain.stage = OgreStage::Winding;
                commands.push(Cmd::Root);
                let (row, windup, sound) = match attack {
                    OgreAttack::Jump => (6, enemies::OGRE_JUMP_WINDUP, SoundEffect::OgreJump),
                    OgreAttack::Throw => (2, enemies::OGRE_THROW_WINDUP, SoundEffect::OgreThrow),
                    OgreAttack::Slam => (3, enemies::OGRE_SLAM_WINDUP, SoundEffect::OgreSwing),
                    OgreAttack::Roar => (4, enemies::OGRE_ROAR_WINDUP, SoundEffect::OgreRoar),
                };
                ogre.brain.timer = windup;
                commands.push(Cmd::Play(clip(row)));
                commands.push(Cmd::Sound(sound));
            }
        }
        OgreStage::Winding => {
            ogre.brain.timer -= dt;
            if ogre.brain.timer <= 0.0 {
                let (command, recover) = match ogre.brain.attack {
                    OgreAttack::Jump => (Cmd::Ring, enemies::OGRE_JUMP_RECOVER),
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
                ogre.brain.stage = OgreStage::Recovering;
                ogre.brain.timer = recover;
            }
        }
        OgreStage::Recovering => {
            ogre.brain.timer -= dt;
            if ogre.brain.timer <= 0.0 {
                commands.push(Cmd::Play(ogre.brain.idle));
                commands.push(Cmd::Free);
                ogre.brain.stage = OgreStage::Walk;
                ogre.brain.cooldown = if enraged {
                    enemies::OGRE_RAGE_COOLDOWN
                } else {
                    enemies::OGRE_COOLDOWN
                };
            }
        }
    }
    commands
}

pub(crate) fn tick_ogre(world: &mut World, hostile: &mut Hostile<'_>) {
    let physics = &mut *hostile.physics;
    let navigation = hostile.navigation;
    let textures = hostile.textures;
    let sounds = &mut *hostile.sounds;
    let fx = &mut *hostile.fx;
    let dt = hostile.dt;
    let target = living_player_at(world).map(|(_, position, _)| position);
    let rats = world.query::<&Minion>().iter().count();
    let bosses: Vec<(Entity, Vec2, bool)> = world
        .query::<(Entity, &Position, &OgreBoss, Satisfies<&Enraged>)>()
        .without::<Inactive>()
        .iter()
        .map(|(entity, position, _, enraged)| (entity, position.0, enraged))
        .collect();
    for (entity, at, enraged) in bosses {
        let distance = target.map_or(f32::MAX, |player| player.distance(at));
        let commands = {
            let Ok(mut ogre) = world.get::<&mut OgreBoss>(entity) else {
                continue;
            };
            step(&mut ogre, enraged, distance, rats, dt)
        };
        for command in commands {
            execute(
                world, physics, navigation, textures, sounds, fx, entity, at, target, command,
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
    fx: &mut Fx,
    entity: Entity,
    at: Vec2,
    player: Option<Vec2>,
    command: Cmd,
) {
    let room = world.get::<&RoomId>(entity).map(|room| *room).ok();
    match command {
        Cmd::Play(clip) => play(world, entity, clip),
        Cmd::Sound(effect) => sounds.push(AudioCommand::At(effect, at)),
        Cmd::Root => {
            let _ = world.insert_one(entity, Rooted);
        }
        Cmd::Free => {
            let _ = world.remove_one::<Rooted>(entity);
        }
        Cmd::Ring => {
            fx::explosion(sounds, fx);
            let origin = at + Vec2::new(0.0, RING_DROP);
            for ring in 0..enemies::OGRE_JUMP_RINGS {
                let count = enemies::OGRE_JUMP_RING;
                let reach = ring as f32 * enemies::OGRE_JUMP_GAP;
                let turn = ring as f32 * 0.5 / count as f32 * std::f32::consts::TAU;
                for index in 0..count {
                    let angle = turn + index as f32 / count as f32 * std::f32::consts::TAU;
                    let direction = Vec2::from_angle(angle);
                    spawn_rock(
                        world,
                        physics,
                        ProjectileKind::GolemRock,
                        origin + direction * reach,
                        direction,
                        enemies::OGRE_STOMP_DAMAGE,
                        room,
                        textures,
                    );
                }
            }
        }
        Cmd::Fan => {
            fx::explosion(sounds, fx);
            let mirrored = world
                .get::<&Animation>(entity)
                .is_ok_and(|animation| animation.flipped());
            let side = if mirrored { -1.0 } else { 1.0 };
            let origin = at + Vec2::new(side * CLUB_REACH.x, CLUB_REACH.y);
            let aim = player.map_or(Vec2::Y, |player| player - origin).to_angle();
            for layer in 0..2 {
                let count = enemies::OGRE_SLAM_FAN;
                for index in 0..count {
                    let t = (index as f32 + layer as f32 * 0.5) / (count - 1) as f32 - 0.5;
                    let direction = Vec2::from_angle(aim + t * enemies::OGRE_SLAM_SPREAD * 2.0);
                    spawn_rock(
                        world,
                        physics,
                        ProjectileKind::GolemRock,
                        origin + direction * layer as f32 * 45.0,
                        direction,
                        enemies::OGRE_STOMP_DAMAGE,
                        room,
                        textures,
                    );
                }
            }
        }
        Cmd::Rats(count) => {
            let mut seed = (at.x as u32).wrapping_mul(2_654_435_761) ^ (at.y as u32);
            let toward = player.map_or(Vec2::X, |player| (player - at).normalize_or_zero());
            for index in 0..count {
                let angle = toward.to_angle()
                    + (index as f32 - (count as f32 - 1.0) / 2.0) * 0.7
                    + (crate::rng::roll(&mut seed) - 0.5) * 0.3;
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
    use crate::components::BossBrain;

    fn ogre() -> OgreBoss {
        OgreBoss {
            brain: BossBrain::new(
                OgreStage::Walk,
                OgreAttack::Jump,
                0.0,
                4242,
                clip(0),
            ),
        }
    }

    fn run(ogre: &mut OgreBoss, seconds: f32) -> Vec<Cmd> {
        let mut all = Vec::new();
        let mut time = 0.0;
        while time < seconds {
            all.extend(step(ogre, false, 200.0, 0, 0.02));
            time += 0.02;
        }
        all
    }

    #[test]
    fn attacks_never_repeat_and_rats_stop_at_the_limit() {
        let mut boss = ogre();
        let mut last = None;
        for _ in 0..200 {
            boss.brain.last = last;
            let attack = choose_attack(&mut boss, 200.0, 0);
            assert_ne!(Some(attack), last);
            last = Some(attack);
        }
        boss.brain.last = None;
        for _ in 0..200 {
            let attack = choose_attack(&mut boss, 600.0, enemies::OGRE_RAT_LIMIT);
            assert!(matches!(attack, OgreAttack::Jump | OgreAttack::Slam));
        }
    }

    #[test]
    fn a_jump_roots_rings_and_frees_the_ogre_again() {
        let mut boss = ogre();
        boss.brain.stage = OgreStage::Winding;
        boss.brain.attack = OgreAttack::Jump;
        boss.brain.last = Some(OgreAttack::Jump);
        boss.brain.timer = 0.01;
        let commands = run(&mut boss, 1.0);
        assert!(commands.contains(&Cmd::Ring));
        assert!(commands.contains(&Cmd::Free));
        assert_eq!(boss.brain.stage, OgreStage::Walk);
    }

    #[test]
    fn a_roar_and_a_throw_call_the_rats() {
        for (attack, count) in [
            (OgreAttack::Roar, enemies::OGRE_ROAR_RATS),
            (OgreAttack::Throw, enemies::OGRE_THROW_RATS),
        ] {
            let mut boss = ogre();
            boss.brain.stage = OgreStage::Winding;
            boss.brain.attack = attack;
            boss.brain.last = Some(attack);
            boss.brain.timer = 0.01;
            assert!(run(&mut boss, 0.5).contains(&Cmd::Rats(count)));
        }
    }

    #[test]
    fn every_wind_up_announces_its_attack_with_a_sound() {
        for seed in 0..40 {
            let mut boss = ogre();
            boss.brain.seed = seed * 7919 + 1;
            let commands = step(&mut boss, false, 200.0, 0, 0.02);
            let expected = match boss.brain.attack {
                OgreAttack::Jump => SoundEffect::OgreJump,
                OgreAttack::Throw => SoundEffect::OgreThrow,
                OgreAttack::Slam => SoundEffect::OgreSwing,
                OgreAttack::Roar => SoundEffect::OgreRoar,
            };
            assert!(commands.contains(&Cmd::Sound(expected)), "{:?}", boss.brain.attack);
        }
    }

    #[test]
    fn the_club_slam_fires_a_fan() {
        let mut boss = ogre();
        boss.brain.stage = OgreStage::Winding;
        boss.brain.attack = OgreAttack::Slam;
        boss.brain.last = Some(OgreAttack::Slam);
        boss.brain.timer = 0.01;
        assert!(run(&mut boss, 0.5).contains(&Cmd::Fan));
    }
}
