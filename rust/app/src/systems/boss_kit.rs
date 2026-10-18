//! What the bosses share: playing a clip without turning round, aiming and turning, firing a
//! rock, sparks, and drawing a damaging beam.

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Tex, Textures};
use crate::audio::AudioCommand;
use crate::components::{
    Animation, BeamPart, BossBrain, Chase, Clip, Color, ContactDamage, Dodge, Drag, Enrage,
    Enraged, Expiring, Frozen, Health, Layer, Light, Position, RoomId, Size, Spawning, Sprite,
    Velocity,
};
use crate::enemies::{self, ProjectileKind};
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::prefabs;
use crate::systems::damage::hurt_player;
use crate::systems::fx::Fx;
use crate::systems::player::player_body;

const BEAM_STEP: f32 = 12.0;
const BEAM_GLOW: f32 = 34.0;
const BEAM_CORE: f32 = 12.0;
const SPARK_SIZE: f32 = 14.0;
const SPARK_FRAMES: usize = 4;

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

pub(crate) fn draw_beam(
    world: &mut World,
    navigation: &Navigation,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    fx: &mut Fx,
    ray: Ray,
    style: BeamStyle,
) {
    let Ray { eye, angle, phase } = ray;
    let player = player_body(world);
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
        let along = crate::rng::roll(&mut seed) * length;
        let side = Vec2::new(-direction.y, direction.x) * (crate::rng::roll(&mut seed) - 0.5) * 2.0;
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
            let spread = (crate::rng::roll(&mut seed) - 0.5) * 2.2;
            let away = -direction;
            let back = Vec2::from_angle(away.y.atan2(away.x) + spread);
            spark(
                world,
                textures,
                style.spark,
                end,
                back * (140.0 + crate::rng::roll(&mut seed) * 160.0),
            );
        }
    }
    if let Some((player, player_position, size)) = player {
        let flying = world.get::<&Dodge>(player).is_ok();
        let reach = enemies::GOLEM_LASER_WIDTH / 2.0 + size.x.min(size.y) / 2.0;
        if !flying && distance_to_segment(player_position, eye, end) < reach {
            hurt_player(world, player, style.damage, sounds, fx);
        }
    }
}

/// Enrages every active boss whose health has dropped to its `Enrage` threshold: tags it
/// `Enraged`, plays its roar, speeds up its chase and tints it. Runs once per tick right before
/// the bosses think, so a boss acts on its rage in the same tick it falls below the line.
pub(crate) fn tick_enrage(world: &mut World, sounds: &mut Vec<AudioCommand>) {
    let due: Vec<(Entity, Vec2, Enrage)> = world
        .query::<(Entity, &Position, &Health, &Enrage)>()
        .without::<&Enraged>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .without::<&Frozen>()
        .iter()
        .filter(|(_, _, health, enrage)| health.current as f32 <= health.max as f32 * enrage.at)
        .map(|(entity, position, _, enrage)| (entity, position.0, *enrage))
        .collect();
    for (entity, at, enrage) in due {
        let _ = world.insert_one(entity, Enraged);
        sounds.push(AudioCommand::At(enrage.sound, at));
        if let Ok(mut chase) = world.get::<&mut Chase>(entity) {
            chase.speed *= enrage.speed;
        }
        if let (Some(tint), Ok(mut color)) = (enrage.tint, world.get::<&mut Color>(entity)) {
            color.0 = tint;
        }
    }
}

/// A weighted roll of the boss's dice among `weights`, never the attack it used last nor one
/// weighted zero; `fallback` if nothing is left to roll.
pub(crate) fn pick_attack<Stage, Attack: Copy + PartialEq>(
    brain: &mut BossBrain<Stage, Attack>,
    weights: &[(Attack, f32)],
    fallback: Attack,
) -> Attack {
    let last = brain.last;
    let allowed = |attack: Attack, weight: f32| weight > 0.0 && Some(attack) != last;
    let total: f32 = weights
        .iter()
        .filter(|(attack, weight)| allowed(*attack, *weight))
        .map(|(_, weight)| weight)
        .sum();
    let mut roll = crate::rng::roll(&mut brain.seed) * total;
    for &(attack, weight) in weights {
        if !allowed(attack, weight) {
            continue;
        }
        if roll < weight {
            return attack;
        }
        roll -= weight;
    }
    fallback
}

/// The attack named by `value` — trimmed, any case — among a boss's `names`.
pub(crate) fn attack_named<A: Copy>(value: &str, names: &[(&str, A)]) -> Option<A> {
    let wanted = value.trim().to_ascii_lowercase();
    names
        .iter()
        .find(|(name, _)| *name == wanted)
        .map(|(_, attack)| *attack)
}

/// The attack a debug environment variable `var` forces a boss to use every time, if it is set
/// to one of the boss's `names`. Each boss caches the answer so the variable is read once.
pub(crate) fn forced_attack<A: Copy>(var: &str, names: &[(&str, A)]) -> Option<A> {
    attack_named(&std::env::var(var).ok()?, names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::SoundEffect;
    use crate::enemies::EnemyKind;

    /// Each boss as it enraged before the shared `tick_enrage`: the sound, the chase speed
    /// multiplier and the tint its own tick used (Argus tints itself every tick instead).
    const BOSSES: [(EnemyKind, SoundEffect, f32, Option<Vec4>); 5] = [
        (EnemyKind::Golem, SoundEffect::GolemRage, 1.35, Some(Vec4::new(1.0, 0.62, 0.55, 1.0))),
        (EnemyKind::Ogre, SoundEffect::OgreRage, 1.35, Some(Vec4::new(1.0, 0.7, 0.62, 1.0))),
        (EnemyKind::Hellhound, SoundEffect::HoundRage, 1.25, Some(Vec4::new(1.0, 0.7, 0.62, 1.0))),
        (EnemyKind::Argus, SoundEffect::ArgusRage, 1.3, None),
        (EnemyKind::Mimic, SoundEffect::MimicRage, 1.3, Some(Vec4::new(1.0, 0.72, 0.66, 1.0))),
    ];

    fn set_health(world: &mut World, boss: Entity, current: i32) {
        world.get::<&mut Health>(boss).expect("health").current = current;
    }

    #[test]
    fn every_boss_enrages_at_half_health_as_it_did_on_its_own() {
        for (kind, sound, speed, tint) in BOSSES {
            let mut world = World::new();
            let mut physics = Physics::new();
            let at = Vec2::new(400.0, 300.0);
            let boss = prefabs::ready_enemy(&mut world, &mut physics, kind, at);
            let max = world.get::<&Health>(boss).expect("health").max;
            let calm_speed = world.get::<&Chase>(boss).expect("chase").speed;
            let mut sounds = Vec::new();
            let half = (max as f32 * 0.5).floor() as i32;
            set_health(&mut world, boss, half + 1);
            tick_enrage(&mut world, &mut sounds);
            assert!(world.get::<&Enraged>(boss).is_err(), "{kind:?} above half");
            assert!(sounds.is_empty(), "{kind:?}");
            set_health(&mut world, boss, half);
            tick_enrage(&mut world, &mut sounds);
            assert!(world.get::<&Enraged>(boss).is_ok(), "{kind:?} at half");
            assert_eq!(sounds, vec![AudioCommand::At(sound, at)], "{kind:?}");
            let raged_speed = world.get::<&Chase>(boss).expect("chase").speed;
            assert_eq!(raged_speed, calm_speed * speed, "{kind:?}");
            let color = world.get::<&Color>(boss).expect("color").0;
            assert_eq!(color, tint.unwrap_or(Vec4::ONE), "{kind:?}");
            set_health(&mut world, boss, 1);
            tick_enrage(&mut world, &mut sounds);
            assert_eq!(sounds.len(), 1, "{kind:?} enrages once");
            let speed_after = world.get::<&Chase>(boss).expect("chase").speed;
            assert_eq!(speed_after, raged_speed, "{kind:?}");
        }
    }

    #[test]
    fn a_frozen_boss_keeps_its_temper_until_it_thaws() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let boss = prefabs::ready_enemy(&mut world, &mut physics, EnemyKind::Golem, Vec2::ZERO);
        set_health(&mut world, boss, 1);
        let _ = world.insert_one(boss, Frozen::new(1.0));
        let mut sounds = Vec::new();
        tick_enrage(&mut world, &mut sounds);
        assert!(world.get::<&Enraged>(boss).is_err());
        let _ = world.remove_one::<Frozen>(boss);
        tick_enrage(&mut world, &mut sounds);
        assert!(world.get::<&Enraged>(boss).is_ok());
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

    #[test]
    fn a_picked_attack_is_never_the_last_one_nor_weighted_zero() {
        let idle = Clip {
            row: 0,
            frames: 1,
            frame_time: 0.1,
            looping: true,
            flipped: false,
        };
        let mut brain = BossBrain::new((), 0, 0.0, 99, idle);
        for _ in 0..200 {
            let attack = pick_attack(&mut brain, &[(0, 1.0), (1, 2.0), (2, 0.0)], 9);
            assert_ne!(Some(attack), brain.last);
            assert_ne!(attack, 2);
            brain.last = Some(attack);
        }
        brain.last = Some(0);
        assert_eq!(pick_attack(&mut brain, &[(0, 1.0), (1, 0.0)], 9), 9);
    }

    #[test]
    fn a_forced_attack_is_matched_by_name_in_any_case() {
        let names = [("jump", 1), ("stomp", 1), ("roar", 2)];
        assert_eq!(attack_named(" Stomp\n", &names), Some(1));
        assert_eq!(attack_named("ROAR", &names), Some(2));
        assert_eq!(attack_named("dance", &names), None);
    }
}
