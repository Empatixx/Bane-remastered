use glam::{Vec2, Vec4};
use hecs::{Entity, Satisfies, World};

use crate::assets::{Textures, tex};
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Animation, Clip, Color, Despawn, Dodge, Enraged, Expiring, FallingRock, Frozen, GolemAttack,
    GolemBoss, GolemStage, Layer, Minion, Position, RockFall, RoomId, Rooted, Size, Spawning,
    Sprite,
};
use crate::enemies::{self, EnemyKind, ProjectileKind};
use crate::geometry;
use crate::physics::Physics;
use crate::prefabs;
use crate::systems::boss_kit::{
    self, BeamStyle, Ray, aim, clear_beam, draw_beam, play, spawn_rock, turn_towards,
};
use crate::systems::damage::hurt_player;
use crate::systems::fx::{self, Fx};
use crate::systems::hostile::Hostile;
use crate::systems::player::{living_player_at, player_body};

const ROCK_RING_ORIGIN_DROP: f32 = 60.0;
const EYE_HEIGHT: f32 = 45.0;
const FALL_HEIGHT: f32 = 380.0;
/// How high a lobbed shot rises at the top of its arc.
const LOB_HEIGHT: f32 = 140.0;
const SHADOW_SIZE: Vec2 = Vec2::new(150.0, 75.0);
const ROCK_SIZE: Vec2 = Vec2::new(72.0, 72.0);
const IMPACT_SIZE: Vec2 = Vec2::new(150.0, 150.0);

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Cmd {
    Play(Clip),
    Root,
    Free,
    Ring(f32),
    Boulder,
    Rain(u32),
    AimBeam,
    Sound(SoundEffect),
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

const ATTACK_VAR: &str = "BANE_GOLEM_ATTACK";

fn forced_attack() -> Option<GolemAttack> {
    static FORCED: std::sync::OnceLock<Option<GolemAttack>> = std::sync::OnceLock::new();
    *FORCED.get_or_init(|| {
        boss_kit::forced_attack(
            ATTACK_VAR,
            &[
                ("slam", GolemAttack::Slam),
                ("throw", GolemAttack::Throw),
                ("rain", GolemAttack::Rain),
                ("laser", GolemAttack::Laser),
            ],
        )
    })
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
    boss_kit::pick_attack(&mut golem.brain, &weights, GolemAttack::Slam)
}

pub(crate) fn step(golem: &mut GolemBoss, enraged: bool, distance: f32, dt: f32) -> Vec<Cmd> {
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
    match golem.brain.stage {
        GolemStage::Walk => {
            golem.brain.cooldown -= dt;
            if golem.brain.cooldown <= 0.0 {
                let attack = choose_attack(golem, distance);
                golem.brain.attack = attack;
                golem.brain.last = Some(attack);
                golem.brain.stage = GolemStage::Winding;
                commands.push(Cmd::Root);
                match attack {
                    GolemAttack::Slam => {
                        golem.brain.timer = enemies::GOLEM_SLAM_WINDUP;
                        commands.push(Cmd::Play(slam_clip()));
                        commands.push(Cmd::Sound(SoundEffect::GolemSlam));
                    }
                    GolemAttack::Throw => {
                        golem.brain.timer = enemies::GOLEM_THROW_WINDUP;
                        commands.push(Cmd::Play(throw_clip()));
                        commands.push(Cmd::Sound(SoundEffect::GolemThrow));
                    }
                    GolemAttack::Rain => {
                        golem.brain.timer = enemies::GOLEM_RAIN_WINDUP;
                        commands.push(Cmd::Play(rain_clip()));
                        commands.push(Cmd::Sound(SoundEffect::GolemRain));
                    }
                    GolemAttack::Laser => {
                        golem.brain.timer = enemies::GOLEM_LASER_CHARGE;
                        commands.push(Cmd::Play(laser_clip()));
                        commands.push(Cmd::AimBeam);
                        commands.push(Cmd::Sound(SoundEffect::LaserCharge));
                    }
                }
            }
        }
        GolemStage::Winding => {
            golem.brain.timer -= dt;
            if golem.brain.timer <= 0.0 {
                match golem.brain.attack {
                    GolemAttack::Slam => {
                        commands.push(Cmd::Ring(0.0));
                        if enraged {
                            golem.second_ring = enemies::GOLEM_SECOND_RING;
                        }
                        golem.brain.stage = GolemStage::Recovering;
                        golem.brain.timer = enemies::GOLEM_SLAM_RECOVER;
                    }
                    GolemAttack::Throw => {
                        commands.push(Cmd::Boulder);
                        golem.brain.stage = GolemStage::Recovering;
                        golem.brain.timer = enemies::GOLEM_THROW_RECOVER;
                    }
                    GolemAttack::Rain => {
                        golem.brain.stage = GolemStage::Acting;
                        golem.brain.timer = enemies::GOLEM_RAIN_SECONDS;
                        golem.pulse = 0.0;
                        golem.rain_dropped = 0;
                    }
                    GolemAttack::Laser => {
                        golem.brain.stage = GolemStage::Acting;
                        golem.brain.timer = enemies::GOLEM_LASER_SECONDS;
                        commands.push(Cmd::Sound(SoundEffect::LaserFire));
                    }
                }
            }
        }
        GolemStage::Acting => {
            golem.brain.timer -= dt;
            if golem.brain.attack == GolemAttack::Rain {
                golem.pulse -= dt;
                while golem.pulse <= 0.0 && golem.brain.timer > 0.0 {
                    commands.push(Cmd::Rain(golem.rain_dropped));
                    golem.rain_dropped += 1;
                    golem.pulse += enemies::GOLEM_RAIN_EVERY;
                }
            }
            if golem.brain.timer <= 0.0 {
                golem.brain.stage = GolemStage::Recovering;
                golem.brain.timer = match golem.brain.attack {
                    GolemAttack::Laser => enemies::GOLEM_LASER_RECOVER,
                    _ => enemies::GOLEM_RAIN_RECOVER,
                };
            }
        }
        GolemStage::Recovering => {
            golem.brain.timer -= dt;
            if golem.brain.timer <= 0.0 {
                commands.push(Cmd::Play(golem.brain.idle));
                commands.push(Cmd::Free);
                golem.brain.stage = GolemStage::Walk;
                golem.brain.cooldown = if enraged {
                    enemies::GOLEM_RAGE_COOLDOWN
                } else {
                    enemies::GOLEM_COOLDOWN
                };
            }
        }
    }
    commands
}

pub(crate) fn tick_golem(world: &mut World, hostile: &mut Hostile<'_>) {
    let physics = &mut *hostile.physics;
    let textures = hostile.textures;
    let sounds = &mut *hostile.sounds;
    let fx = &mut *hostile.fx;
    let dt = hostile.dt;
    let target = living_player_at(world);
    let golems: Vec<(Entity, Vec2, bool)> = world
        .query::<(Entity, &Position, &GolemBoss, Satisfies<&Enraged>)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .without::<&Frozen>()
        .iter()
        .map(|(entity, position, _, enraged)| (entity, position.0, enraged))
        .collect();
    for (entity, at, enraged) in golems {
        let distance = target.map_or(f32::MAX, |(_, player, _)| player.distance(at));
        let commands = {
            let Ok(mut golem) = world.get::<&mut GolemBoss>(entity) else {
                continue;
            };
            step(&mut golem, enraged, distance, dt)
        };
        for command in commands {
            execute(
                world,
                physics,
                textures,
                sounds,
                fx,
                entity,
                at,
                target.map(|(_, p, _)| p),
                command,
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
        Cmd::Ring(offset) => {
            fx::explosion(sounds, fx);
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
            let jitter = Vec2::new(
                crate::rng::roll(&mut seed) - 0.5,
                crate::rng::roll(&mut seed) - 0.5,
            ) * 2.0
                * spread;
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
            duration: enemies::GOLEM_RAIN_DELAY,
            rock,
            room,
            kind: crate::components::FallKind::Boulder,
            from: None,
        },
        Position(ground),
        Size(SHADOW_SIZE),
        Color(Vec4::ONE),
        Sprite::whole(textures.get(tex::ROCK_SHADOW)),
        Layer(-1),
    ));
}

pub(crate) fn tick_rock_falls(world: &mut World, hostile: &mut Hostile<'_>) {
    let physics = &mut *hostile.physics;
    let navigation = hostile.navigation;
    let textures = hostile.textures;
    let sounds = &mut *hostile.sounds;
    let fx = &mut *hostile.fx;
    let dt = hostile.dt;
    let mut landed = Vec::new();
    for (entity, fall, position) in world.query_mut::<(Entity, &mut RockFall, &Position)>() {
        fall.remaining -= dt;
        if fall.remaining <= 0.0 {
            landed.push((entity, fall.rock, position.0, fall.room, fall.kind));
        }
    }
    let falling: Vec<(Entity, Entity, f32, Option<Vec2>)> = world
        .query::<(Entity, &RockFall)>()
        .iter()
        .map(|(entity, fall)| {
            let progress = (fall.remaining / fall.duration).clamp(0.0, 1.0);
            (entity, fall.rock, progress, fall.from)
        })
        .collect();
    for (shadow, rock, progress, from) in falling {
        let ground = world.get::<&Position>(shadow).map(|p| p.0).ok();
        if let (Some(ground), Ok(mut position)) = (ground, world.get::<&mut Position>(rock)) {
            position.0 = match from {
                // A lobbed shot: along the line from where it was thrown, up in an arc.
                Some(from) => {
                    let flown = 1.0 - progress;
                    from.lerp(ground, flown) - Vec2::new(0.0, LOB_HEIGHT * 4.0 * flown * (1.0 - flown))
                }
                None => ground - Vec2::new(0.0, FALL_HEIGHT * (2.0 * progress - progress * progress)),
            };
        }
    }
    let player = player_body(world);
    for (shadow, rock, ground, room, kind) in landed {
        let _ = world.despawn(shadow);
        let _ = world.despawn(rock);
        match kind {
            crate::components::FallKind::Icicle => {
                icicle_lands(world, textures, ground, player, sounds, fx);
                continue;
            }
            crate::components::FallKind::Magma => {
                super::lava::magma_lands(world, textures, ground, player, sounds, fx);
                continue;
            }
            crate::components::FallKind::Boulder => {}
        }
        sounds.push(AudioCommand::At(SoundEffect::WallHit, ground));
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
                hurt_player(world, player, enemies::GOLEM_RAIN_DAMAGE, sounds, fx);
            }
        }
        let mut seed =
            (ground.x as u32).wrapping_mul(2_654_435_761) ^ (ground.y as u32).rotate_left(11);
        let minions = world.query::<&Minion>().iter().count();
        if crate::rng::roll(&mut seed) < enemies::GOLEM_MINION_CHANCE
            && minions < enemies::GOLEM_MINION_LIMIT
            && !navigation.is_blocked(ground, Vec2::splat(60.0))
        {
            prefabs::hatchling(world, physics, EnemyKind::MiniGolem, ground, room, textures);
        }
    }
}

/// An icicle shatters where its shadow was; standing there hurts and chills.
fn icicle_lands(
    world: &mut World,
    textures: &Textures,
    ground: Vec2,
    player: Option<(Entity, Vec2, Vec2)>,
    sounds: &mut Vec<AudioCommand>,
    fx: &mut Fx,
) {
    sounds.push(AudioCommand::At(SoundEffect::WallHit, ground));
    prefabs::ice_shatter(world, ground, textures);
    let Some((player, at, size)) = player else {
        return;
    };
    let reach = Vec2::splat(enemies::ICICLE_RADIUS * 2.0);
    if world.get::<&Dodge>(player).is_err()
        && geometry::overlaps(ground, reach, at, size)
        && hurt_player(world, player, enemies::ICICLE_DAMAGE, sounds, fx)
    {
        super::frost::chill_player(world, player);
    }
}

pub(crate) const BLUE_BEAM: BeamStyle = BeamStyle {
    glow: Vec4::new(0.2, 0.5, 1.0, 0.5),
    core: Vec4::new(0.85, 0.96, 1.0, 1.0),
    light: [60.0, 140.0, 255.0],
    spark: tex::SPARK,
    damage: enemies::GOLEM_LASER_DAMAGE,
    wave: 0.0,
};

pub(crate) fn tick_laser(world: &mut World, hostile: &mut Hostile<'_>) {
    let navigation = hostile.navigation;
    let textures = hostile.textures;
    let sounds = &mut *hostile.sounds;
    let fx = &mut *hostile.fx;
    let dt = hostile.dt;
    clear_beam(world);
    let player = living_player_at(world);
    let firing: Vec<(Entity, Vec2, bool)> = world
        .query::<(Entity, &Position, &GolemBoss, Satisfies<&Enraged>)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .iter()
        .filter(|(_, _, golem, _)| {
            golem.brain.attack == GolemAttack::Laser && golem.brain.stage == GolemStage::Acting
        })
        .map(|(entity, position, _, enraged)| (entity, position.0, enraged))
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
        draw_beam(world, navigation, textures, sounds, fx, ray, BLUE_BEAM);
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
    use crate::components::BossBrain;

    fn golem() -> GolemBoss {
        GolemBoss {
            brain: BossBrain::new(
                GolemStage::Walk,
                GolemAttack::Slam,
                0.0,
                12345,
                slam_clip(),
            ),
            pulse: 0.0,
            rain_dropped: 0,
            second_ring: 0.0,
            beam_angle: 0.0,
        }
    }

    fn run(golem: &mut GolemBoss, enraged: bool, seconds: f32) -> Vec<Cmd> {
        let mut all = Vec::new();
        let mut time = 0.0;
        while time < seconds {
            all.extend(step(golem, enraged, 200.0, 0.02));
            time += 0.02;
        }
        all
    }

    #[test]
    fn an_attack_never_repeats_the_previous_one() {
        let mut boss = golem();
        let mut last = None;
        for _ in 0..200 {
            boss.brain.last = last;
            let attack = choose_attack(&mut boss, 200.0);
            assert_ne!(Some(attack), last);
            last = Some(attack);
        }
    }

    #[test]
    fn every_wind_up_announces_its_attack_with_a_sound() {
        for seed in 0..40 {
            let mut boss = golem();
            boss.brain.seed = seed * 7919 + 1;
            let commands = step(&mut boss, false, 200.0, 0.02);
            let expected = match boss.brain.attack {
                GolemAttack::Slam => SoundEffect::GolemSlam,
                GolemAttack::Throw => SoundEffect::GolemThrow,
                GolemAttack::Rain => SoundEffect::GolemRain,
                GolemAttack::Laser => SoundEffect::LaserCharge,
            };
            assert!(commands.contains(&Cmd::Sound(expected)), "{:?}", boss.brain.attack);
        }
    }

    #[test]
    fn the_laser_hums_once_it_starts_firing() {
        let mut boss = golem();
        boss.brain.attack = GolemAttack::Laser;
        boss.brain.last = Some(GolemAttack::Laser);
        boss.brain.stage = GolemStage::Winding;
        boss.brain.timer = 0.01;
        let commands = step(&mut boss, false, 200.0, 0.02);
        assert!(commands.contains(&Cmd::Sound(SoundEffect::LaserFire)));
        assert_eq!(boss.brain.stage, GolemStage::Acting);
    }

    #[test]
    fn a_slam_roots_the_golem_fires_a_ring_and_frees_it_again() {
        let mut boss = golem();
        boss.brain.attack = GolemAttack::Slam;
        boss.brain.last = Some(GolemAttack::Slam);
        boss.brain.stage = GolemStage::Winding;
        boss.brain.timer = enemies::GOLEM_SLAM_WINDUP;
        let commands = run(&mut boss, false, 3.0);
        assert!(commands.contains(&Cmd::Ring(0.0)));
        assert!(commands.contains(&Cmd::Free));
        assert_eq!(boss.brain.stage, GolemStage::Walk);
    }

    #[test]
    fn rage_adds_a_second_ring() {
        let mut boss = golem();
        boss.brain.stage = GolemStage::Winding;
        boss.brain.attack = GolemAttack::Slam;
        boss.brain.timer = 0.01;
        let rings = run(&mut boss, true, 2.0)
            .iter()
            .filter(|command| matches!(command, Cmd::Ring(_)))
            .count();
        assert_eq!(rings, 2);
    }

    #[test]
    fn rain_drops_rocks_over_its_whole_duration() {
        let mut boss = golem();
        boss.brain.stage = GolemStage::Winding;
        boss.brain.attack = GolemAttack::Rain;
        boss.brain.timer = 0.01;
        let rocks = run(&mut boss, false, 4.0)
            .iter()
            .filter(|command| matches!(command, Cmd::Rain(_)))
            .count();
        assert!(rocks >= 8, "{rocks}");
    }
}
