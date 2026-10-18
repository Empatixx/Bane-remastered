use glam::Vec2;
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::BULLET_IMPACT;
use crate::components::{
    Animation, Body, Boss, Breakable, Bullet, Burst, CHILL_SECONDS, CHILL_STACKS_TO_FREEZE,
    Chilled, Enemy, Expiring, Friendly, Frozen, Heading, Health, InfiniteAmmo, Loadout, Phantom,
    Player, PlayerPerks, PoisonPool, Position, Size, Spawning, Summoned, TrainingDummy, Velocity,
};
use crate::controls::Action;
use crate::geometry;
use crate::guns::Mutation;
use crate::guns::{BURST_DELAY, GunSpec, MAX_PELLETS, Round};
use crate::input::Input;
use crate::physics::{Owner, Physics};
use crate::prefabs;
use crate::render::TextureId;
use crate::rng::Rng;
use crate::systems::damage::{Damage, DamageSource};
use crate::systems::fx::{self, Fx};
use crate::systems::loadout;
use crate::upgrades::Perks;

const BOUNCE_CLEARANCE: f32 = 2.0;
const BLAST_LOOK: Vec2 = Vec2::new(72.0, 72.0);
const BLAST_REACH: Vec2 = Vec2::new(96.0, 96.0);
const ROCKET_LOOK: Vec2 = Vec2::new(144.0, 144.0);
const ROCKET_REACH: Vec2 = Vec2::new(176.0, 176.0);
const SPLASH_REACH: Vec2 = Vec2::new(80.0, 80.0);
const CHAIN_REACH: f32 = 220.0;
const VENOM_POOL_SECONDS: f32 = 3.0;
const VENOM_SPACING: f32 = 64.0;
const FROST_ROUND_FREEZE: f32 = 1.2;
const AMBUSH_MULTIPLIER: i32 = 2;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ShotRoll {
    pub(crate) spread: f32,
    pub(crate) damages: [f32; MAX_PELLETS],
    pub(crate) fractions: [f32; MAX_PELLETS],
    /// Whether the fraction the damage talents add becomes a point, per pellet.
    pub(crate) perk_fractions: [f32; MAX_PELLETS],
    /// Every pellet rolls its own critical hit.
    pub(crate) criticals: [f32; MAX_PELLETS],
    pub(crate) eject: [f32; 4],
}

impl ShotRoll {
    pub(crate) fn from(rng: &mut Rng) -> Self {
        Self {
            spread: rng.signed(),
            damages: std::array::from_fn(|_| rng.float()),
            fractions: std::array::from_fn(|_| rng.float()),
            perk_fractions: std::array::from_fn(|_| rng.float()),
            criticals: std::array::from_fn(|_| rng.float()),
            eject: std::array::from_fn(|_| rng.float()),
        }
    }
}

/// The sheet of every kind of round, looked up once a frame instead of per shot.
#[derive(Debug, Clone, Copy)]
pub(crate) struct RoundTextures([TextureId; Round::ALL.len()]);

impl RoundTextures {
    pub(crate) fn load(textures: &Textures) -> Self {
        Self(Round::ALL.map(|round| textures.get(round.spec().texture)))
    }

    #[cfg(test)]
    pub(crate) fn white() -> Self {
        Self([TextureId::WHITE; Round::ALL.len()])
    }

    fn get(&self, round: Round) -> TextureId {
        self.0[round as usize]
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ShotContext {
    pub(crate) camera: Vec2,
    pub(crate) rounds: RoundTextures,
    pub(crate) casing_texture: TextureId,
    pub(crate) dt: f32,
    pub(crate) roll: ShotRoll,
}

struct Shot {
    origin: Vec2,
    direction: Vec2,
    spec: GunSpec,
    roll: ShotRoll,
    pellets: usize,
    perks: Perks,
    deadeye: bool,
    ambush: bool,
}

pub(crate) fn shoot(
    world: &mut World,
    physics: &mut Physics,
    input: &Input,
    context: ShotContext,
    sounds: &mut Vec<AudioCommand>,
) -> f32 {
    let ShotContext {
        camera,
        rounds,
        casing_texture,
        dt,
        roll,
    } = context;
    let target = input.mouse() + camera;
    let mut shots = Vec::new();
    let mut uncloaked = Vec::new();
    for (entity, position, loadout, perks, infinite, phantom) in world
        .query_mut::<(
            Entity,
            &Position,
            &mut Loadout,
            Option<&PlayerPerks>,
            Option<&InfiniteAmmo>,
            Option<&Phantom>,
        )>()
        .with::<&Player>()
        .without::<&crate::components::Dodge>()
        .without::<&crate::components::Frozen>()
    {
        let perks = perks.map_or_else(Perks::default, |perks| perks.0);
        let Some(weapon) = loadout.current_mut() else {
            continue;
        };
        let spec = weapon.spec();
        let infinite = infinite.is_some();
        if infinite {
            weapon.reloading = None;
        }
        loadout::advance_weapon(weapon, dt);
        if !infinite && input.action_pressed(Action::Reload) && loadout::start_reload(weapon) {
            sounds.push(AudioCommand::Effect(spec.reload));
        }
        if let Some(mut burst) = weapon.burst {
            burst.timer -= dt;
            if burst.timer <= 0.0 {
                if infinite || weapon.clip > 0 {
                    if !infinite {
                        weapon.clip -= 1;
                    }
                    weapon.cooldown = spec.fire_interval;
                    let deadeye = loadout::count_shot(weapon);
                    shots.push(Shot {
                        origin: position.0,
                        direction: burst.aim,
                        spec,
                        roll,
                        pellets: 1,
                        perks,
                        deadeye,
                        ambush: false,
                    });
                    burst.left -= 1;
                    burst.timer = BURST_DELAY;
                }
                weapon.burst = (burst.left > 0 && (infinite || weapon.clip > 0)).then_some(burst);
            } else {
                weapon.burst = Some(burst);
            }
            continue;
        }
        let pulled = if spec.auto {
            input.mouse_down()
        } else {
            input.clicked()
        };
        if !pulled || weapon.reloading.is_some() || weapon.cooldown > 0.0 {
            continue;
        }
        if !infinite && weapon.clip == 0 {
            // One dry sound per pull, not one per fire interval while held.
            if input.clicked() {
                sounds.push(AudioCommand::Effect(spec.empty));
            }
            continue;
        }
        let to_target = target - position.0;
        if to_target == Vec2::ZERO {
            continue;
        }
        weapon.cooldown = spec.fire_interval;
        if !infinite {
            weapon.clip -= 1;
        }
        sounds.push(AudioCommand::Effect(spec.shoot));
        if let Some(count) = spec.burst {
            weapon.burst = (count > 1 && (infinite || weapon.clip > 0)).then_some(Burst {
                left: count - 1,
                timer: BURST_DELAY,
                aim: to_target,
            });
        }
        let deadeye = loadout::count_shot(weapon);
        shots.push(Shot {
            origin: position.0,
            direction: to_target,
            spec,
            roll,
            pellets: spec.pellets,
            perks,
            deadeye,
            ambush: phantom.is_some(),
        });
        if phantom.is_some() {
            uncloaked.push(entity);
        }
    }
    for entity in uncloaked {
        let _ = world.remove_one::<Phantom>(entity);
    }
    let recoil: f32 = shots.iter().map(|shot| shot.spec.shake).sum();
    let fired: u32 = shots.iter().map(|shot| shot.pellets as u32).sum();
    super::stats::record(world, super::stats::Stat::Fired, fired);
    for shot in shots {
        let round = shot.spec.round;
        if matches!(round, Round::Bullet | Round::Tracer | Round::Slug) {
            prefabs::casing(
                world,
                shot.origin,
                shot.direction,
                shot.spec.pellets > 1,
                shot.roll.eject,
                casing_texture,
            );
        }
        let aim = shot.direction.to_angle();
        for pellet in 0..shot.pellets {
            let angle = aim + shot.spec.angle_offset(pellet, shot.roll.spread);
            let critical = shot.deadeye
                || shot.roll.criticals[pellet] < shot.perks.crit_chance() + shot.spec.crit_chance;
            let damage = shot.perks.bullet_damage(
                shot.spec
                    .scaled_damage(shot.roll.damages[pellet], shot.roll.fractions[pellet]),
                critical,
                shot.roll.perk_fractions[pellet],
            ) * if shot.ambush { AMBUSH_MULTIPLIER } else { 1 };
            prefabs::bullet(
                world,
                physics,
                shot.origin,
                Vec2::from_angle(angle) * round.spec().speed,
                Bullet::mutated(damage, shot.spec.mutation).as_round(round),
                rounds.get(round),
            );
        }
    }
    recoil
}

pub(crate) fn bullet_hits(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    fx: &mut Fx,
) {
    let hits: Vec<(Entity, Bullet, Vec<Owner>, Body)> = world
        .query::<(Entity, &Body, &Bullet)>()
        .without::<&Expiring>()
        .iter()
        .map(|(entity, body, bullet)| (entity, *bullet, physics.touching(*body), *body))
        .filter(|(_, _, touched, _)| !touched.is_empty())
        .collect();
    for (entity, mut bullet, touched, body) in hits {
        let at = world
            .get::<&Position>(entity)
            .map_or(Vec2::ZERO, |position| position.0);
        let victim = touched.iter().find_map(|owner| match owner {
            Owner::Entity(target) if Some(*target) != bullet.last_hit => Some(*target),
            _ => None,
        });
        if victim.is_none() && touched.iter().all(|owner| *owner != Owner::Wall) {
            continue;
        }
        if bullet.round.spec().blast {
            rocket_blast(world, bullet.damage, at, textures);
            fx::explosion(sounds, fx);
            stop_round(world, physics, entity);
            continue;
        }
        let source = if world.get::<&Summoned>(entity).is_ok() {
            DamageSource::Summoned
        } else {
            DamageSource::Bullet
        };
        let hit_enemy = victim.is_some_and(|target| {
            super::damage::deal(
                world,
                Damage {
                    target,
                    amount: bullet.damage,
                    source,
                },
            )
        });
        sounds.push(AudioCommand::Effect(if hit_enemy {
            SoundEffect::EnemyHit
        } else {
            SoundEffect::WallHit
        }));
        let round = bullet.round.spec();
        let struck = victim.filter(|_| hit_enemy);
        if round.splash {
            splash(world, struck, bullet.damage, at);
        }
        if let Some(target) = struck.filter(|_| round.chains > 0) {
            let jumps = (round.chains, round.chain_falloff);
            chain_lightning(
                world,
                target,
                bullet.damage,
                jumps,
                round.bolt,
                textures,
                sounds,
            );
        }
        if let Some(target) = victim.filter(|_| hit_enemy) {
            mutate_hit(world, target, bullet, at, textures);
            if bullet.pierce > 0 {
                bullet.pierce -= 1;
                bullet.last_hit = Some(target);
                let _ = world.insert_one(entity, bullet);
                continue;
            }
        }
        if victim.is_none() && bullet.bounces > 0 {
            let velocity = world
                .get::<&Velocity>(entity)
                .map_or(Vec2::ZERO, |velocity| velocity.0);
            if let Some((normal, depth)) = physics.wall_push(body, velocity) {
                bullet.bounces -= 1;
                bullet.last_hit = None;
                let _ = world.insert_one(entity, bullet);
                let pushed = at + normal * (depth + BOUNCE_CLEARANCE);
                physics.teleport(body, pushed);
                if let Ok(mut position) = world.get::<&mut Position>(entity) {
                    position.0 = pushed;
                }
                let bounced = reflect(velocity, normal);
                if let Ok(mut moving) = world.get::<&mut Velocity>(entity) {
                    moving.0 = bounced;
                }
                if let Ok(mut heading) = world.get::<&mut Heading>(entity) {
                    heading.0 = bounced.to_angle();
                }
                continue;
            }
        }
        if bullet.mutation == Some(Mutation::Explosive) && victim.is_none() {
            explode_round(world, None, bullet.damage, at, textures);
        }
        stop_round(world, physics, entity);
    }
}

/// Short-lived rounds such as flames fizzle out in the air once their time is up.
pub(crate) fn burn_out_rounds(world: &mut World, physics: &mut Physics, dt: f32) {
    let mut spent = Vec::new();
    for (entity, bullet) in world
        .query_mut::<(Entity, &mut Bullet)>()
        .without::<&Expiring>()
    {
        if let Some(left) = &mut bullet.lifetime {
            *left -= dt;
            if *left <= 0.0 {
                spent.push(entity);
            }
        }
    }
    for entity in spent {
        stop_round(world, physics, entity);
    }
}

fn stop_round(world: &mut World, physics: &mut Physics, entity: Entity) {
    physics.detach(world, entity);
    let _ = world.insert_one(entity, Expiring);
    if let Ok(mut velocity) = world.get::<&mut Velocity>(entity) {
        velocity.0 = Vec2::ZERO;
    }
    if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
        animation.play(BULLET_IMPACT);
    }
}

/// A rocket bursts on whatever it touches and hits everything around with its full damage.
fn rocket_blast(world: &mut World, damage: i32, at: Vec2, textures: &Textures) {
    prefabs::blast(world, at, ROCKET_LOOK, textures);
    for target in blast_victims(world, None, at, ROCKET_REACH) {
        super::damage::deal(
            world,
            Damage {
                target,
                amount: damage,
                source: DamageSource::Explosion,
            },
        );
    }
}

fn reflect(velocity: Vec2, normal: Vec2) -> Vec2 {
    if velocity.dot(normal) >= 0.0 {
        return velocity;
    }
    velocity - 2.0 * velocity.dot(normal) * normal
}

fn mutate_hit(world: &mut World, target: Entity, bullet: Bullet, at: Vec2, textures: &Textures) {
    let is_enemy = world.get::<&Enemy>(target).is_ok();
    match bullet.mutation {
        Some(Mutation::Explosive) => {
            explode_round(world, Some(target), bullet.damage, at, textures)
        }
        Some(Mutation::Venom) if is_enemy => {
            let near_pool = world
                .query::<&Position>()
                .with::<(&PoisonPool, &Friendly)>()
                .iter()
                .any(|position| position.0.distance(at) < VENOM_SPACING);
            if !near_pool {
                prefabs::venom_pool(world, at, VENOM_POOL_SECONDS, textures);
            }
        }
        Some(Mutation::Frost) if is_enemy => chill(world, target),
        _ => {}
    }
}

fn chill(world: &mut World, target: Entity) {
    let boss = world.get::<&Boss>(target).is_ok();
    let stacks = world
        .get::<&Chilled>(target)
        .map_or(0, |chilled| chilled.stacks)
        + 1;
    if stacks >= CHILL_STACKS_TO_FREEZE && !boss && world.get::<&Frozen>(target).is_err() {
        let _ = world.insert_one(target, Frozen::new(FROST_ROUND_FREEZE));
        let _ = world.insert_one(
            target,
            Chilled {
                remaining: CHILL_SECONDS,
                stacks: 0,
            },
        );
        return;
    }
    let _ = world.insert_one(
        target,
        Chilled {
            remaining: CHILL_SECONDS,
            stacks: stacks.min(CHILL_STACKS_TO_FREEZE),
        },
    );
}

/// The small blast of an explosive round: it hurts what stands around the hit but never the
/// player who fired it, and the enemy that was hit already took the bullet.
fn explode_round(
    world: &mut World,
    struck: Option<Entity>,
    damage: i32,
    at: Vec2,
    textures: &Textures,
) {
    prefabs::blast(world, at, BLAST_LOOK, textures);
    let amount = (damage / 2).max(1);
    for target in blast_victims(world, struck, at, BLAST_REACH) {
        super::damage::deal(
            world,
            Damage {
                target,
                amount,
                source: DamageSource::Explosion,
            },
        );
    }
}

/// Plasma spatters what stands around the hit with half the damage.
fn splash(world: &mut World, struck: Option<Entity>, damage: i32, at: Vec2) {
    let amount = (damage / 2).max(1);
    for target in blast_victims(world, struck, at, SPLASH_REACH) {
        super::damage::deal(
            world,
            Damage {
                target,
                amount,
                source: DamageSource::Explosion,
            },
        );
    }
}

/// The hit jumps as lightning to the nearest enemy it has not struck yet, then on from there.
/// Lightning jumps from the hit to the nearest enemy (or practice dummy) not struck yet,
/// `jumps.0` times, each jump keeping `jumps.1` of the damage before it.
fn chain_lightning(
    world: &mut World,
    first: Entity,
    damage: i32,
    jumps: (u32, f32),
    bolt: crate::guns::Bolt,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
) {
    let (jumps, falloff) = jumps;
    let mut amount = damage as f32;
    let Ok(start) = world.get::<&Position>(first).map(|position| position.0) else {
        return;
    };
    let mut struck = vec![first];
    let mut from = start;
    for _ in 0..jumps {
        let next = world
            .query::<(Entity, &Position, Option<&Enemy>, Option<&TrainingDummy>)>()
            .with::<&Health>()
            .without::<&Expiring>()
            .without::<&Spawning>()
            .iter()
            .filter(|(entity, position, enemy, dummy)| {
                (enemy.is_some() || dummy.is_some())
                    && !struck.contains(entity)
                    && position.0.distance(from) <= CHAIN_REACH
            })
            .map(|(entity, position, ..)| (entity, position.0))
            .min_by(|a, b| a.1.distance(from).total_cmp(&b.1.distance(from)));
        let Some((target, to)) = next else {
            break;
        };
        prefabs::arc(world, from, to, bolt, textures);
        super::trails::arc_motes(
            world,
            textures.get(bolt.sparks.texture),
            bolt.sparks,
            from,
            to,
        );
        amount *= falloff;
        super::damage::deal(
            world,
            Damage {
                target,
                amount: (amount.round() as i32).max(1),
                source: DamageSource::Bullet,
            },
        );
        struck.push(target);
        from = to;
    }
    if struck.len() > 1 {
        sounds.push(AudioCommand::Effect(SoundEffect::TeslaZap));
    }
}

/// Enemies and breakables inside the blast; the player who fired never is.
fn blast_victims(world: &World, struck: Option<Entity>, at: Vec2, reach: Vec2) -> Vec<Entity> {
    world
        .query::<(Entity, &Position, &Size)>()
        .with::<&Health>()
        .without::<&Player>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .filter(|(entity, position, size)| {
            Some(*entity) != struck && geometry::overlaps(at, reach, position.0, size.0)
        })
        .filter(|(entity, ..)| {
            world.get::<&Enemy>(*entity).is_ok() || world.get::<&Breakable>(*entity).is_ok()
        })
        .map(|(entity, ..)| entity)
        .collect()
}

#[cfg(test)]
mod tests {
    use winit::event::{ElementState, MouseButton};
    use winit::keyboard::KeyCode;

    use crate::assets::Textures;
    use crate::components::{Health, Player, Weapon};
    use crate::systems::simulate;

    use super::*;

    fn clicked_at(target: Vec2) -> Input {
        let mut input = Input::default();
        input.set_mouse(target);
        input.handle_click(MouseButton::Left, ElementState::Pressed);
        input
    }

    fn setup() -> (World, Physics) {
        let mut world = World::new();
        let mut physics = Physics::new();
        prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(100.0, 100.0),
            &Textures::default(),
        );
        (world, physics)
    }

    fn fire(world: &mut World, physics: &mut Physics, input: &Input, camera: Vec2, dt: f32) {
        shoot(
            world,
            physics,
            input,
            ShotContext {
                camera,
                rounds: RoundTextures::white(),
                casing_texture: TextureId::WHITE,
                dt,
                roll: ShotRoll {
                    spread: 0.0,
                    damages: [0.0; MAX_PELLETS],
                    fractions: [1.0; MAX_PELLETS],
                    perk_fractions: [1.0; MAX_PELLETS],
                    criticals: [1.0; MAX_PELLETS],
                    eject: [0.5; 4],
                },
            },
            &mut Vec::new(),
        );
    }

    fn bullet_velocities(world: &World) -> Vec<Vec2> {
        world
            .query::<&Velocity>()
            .with::<&Bullet>()
            .iter()
            .map(|velocity| velocity.0)
            .collect()
    }

    fn bullet_damages(world: &World) -> Vec<i32> {
        world
            .query::<&Bullet>()
            .iter()
            .map(|bullet| bullet.damage)
            .collect()
    }

    #[test]
    fn the_first_shot_from_the_cloak_ends_it_and_hits_twice_as_hard() {
        let (mut world, mut physics) = setup();
        let input = clicked_at(Vec2::new(300.0, 100.0));
        fire(&mut world, &mut physics, &input, Vec2::ZERO, 0.0);
        let plain = bullet_damages(&world);
        assert_eq!(plain.len(), 1);

        let (mut world, mut physics) = setup();
        let player = world
            .query::<Entity>()
            .with::<&Player>()
            .iter()
            .next()
            .expect("player");
        let _ = world.insert_one(player, Phantom { remaining: 5.0 });
        fire(&mut world, &mut physics, &input, Vec2::ZERO, 0.0);

        assert_eq!(bullet_damages(&world), vec![plain[0] * 2]);
        assert!(world.get::<&Phantom>(player).is_err());
    }

    #[test]
    fn click_fires_a_bullet_towards_the_mouse_in_world_space() {
        let (mut world, mut physics) = setup();
        let input = clicked_at(Vec2::new(50.0, 0.0));

        fire(
            &mut world,
            &mut physics,
            &input,
            Vec2::new(100.0, 100.0),
            0.0,
        );

        let velocities = bullet_velocities(&world);
        assert_eq!(velocities.len(), 1);
        assert!((velocities[0] - Vec2::new(700.0, 0.0)).length() < 0.01);
    }

    #[test]
    fn cooldown_blocks_the_next_shot_until_the_fire_interval_passes() {
        let (mut world, mut physics) = setup();
        let input = clicked_at(Vec2::new(300.0, 100.0));

        fire(&mut world, &mut physics, &input, Vec2::ZERO, 0.0);
        fire(&mut world, &mut physics, &input, Vec2::ZERO, 0.1);
        assert_eq!(bullet_velocities(&world).len(), 1);

        fire(&mut world, &mut physics, &input, Vec2::ZERO, 0.2);
        assert_eq!(bullet_velocities(&world).len(), 2);
    }

    #[test]
    fn bullet_hitting_a_wall_loses_its_body_and_starts_the_impact_animation() {
        let mut world = World::new();
        let mut physics = Physics::new();
        physics.add_walls(&[(3, 0), (3, 1)], 64.0);
        let bullet = prefabs::bullet(
            &mut world,
            &mut physics,
            Vec2::new(100.0, 64.0),
            Vec2::new(700.0, 0.0),
            crate::components::Bullet::plain(1),
            TextureId::WHITE,
        );

        for _ in 0..30 {
            simulate(&mut world, &mut physics, 1.0 / 60.0);
            bullet_hits(
                &mut world,
                &mut physics,
                &Textures::default(),
                &mut Vec::new(), &mut Fx::default(),
            );
        }

        assert!(world.get::<&Expiring>(bullet).is_ok());
        assert!(world.get::<&Body>(bullet).is_err());
        let position = world.get::<&Position>(bullet).map(|p| p.0).ok();
        assert!(
            position.is_some_and(|p| p.x > 184.0 && p.x < 216.0),
            "{position:?}"
        );
    }

    #[test]
    fn bullet_damages_the_enemy_it_hits() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Rat,
            Vec2::new(300.0, 100.0),
        );
        let before = world.get::<&Health>(rat).map(|h| h.current).unwrap_or(0);
        let bullet = prefabs::bullet(
            &mut world,
            &mut physics,
            Vec2::new(200.0, 100.0),
            Vec2::new(700.0, 0.0),
            crate::components::Bullet::plain(3),
            TextureId::WHITE,
        );

        for _ in 0..30 {
            simulate(&mut world, &mut physics, 1.0 / 60.0);
            bullet_hits(
                &mut world,
                &mut physics,
                &Textures::default(),
                &mut Vec::new(), &mut Fx::default(),
            );
        }

        let after = world.get::<&Health>(rat).map(|h| h.current).unwrap_or(0);
        assert_eq!(before - after, 3);
        assert!(world.get::<&Expiring>(bullet).is_ok());
    }

    fn give(world: &mut World, kind: crate::guns::GunKind) {
        for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
            loadout.drop_current();
            loadout::add_gun(loadout, Weapon::new(kind));
        }
    }

    fn held_at(target: Vec2) -> Input {
        let mut input = Input::default();
        input.set_mouse(target);
        input.handle_click(MouseButton::Left, ElementState::Pressed);
        input.end_frame();
        input
    }

    fn current(world: &World) -> Weapon {
        world
            .query::<&Loadout>()
            .with::<&Player>()
            .iter()
            .next()
            .and_then(|loadout| loadout.current().copied())
            .expect("the player holds a gun")
    }

    fn fire_with_sounds(
        world: &mut World,
        physics: &mut Physics,
        input: &Input,
        dt: f32,
        sounds: &mut Vec<AudioCommand>,
    ) {
        shoot(
            world,
            physics,
            input,
            ShotContext {
                camera: Vec2::ZERO,
                rounds: RoundTextures::white(),
                casing_texture: TextureId::WHITE,
                dt,
                roll: ShotRoll {
                    spread: 0.0,
                    damages: [0.0; MAX_PELLETS],
                    fractions: [1.0; MAX_PELLETS],
                    perk_fractions: [1.0; MAX_PELLETS],
                    criticals: [1.0; MAX_PELLETS],
                    eject: [0.5; 4],
                },
            },
            sounds,
        );
    }

    #[test]
    fn the_shotgun_fires_four_pellets_for_a_single_round() {
        let (mut world, mut physics) = setup();
        give(&mut world, crate::guns::GunKind::Shotgun);
        let input = clicked_at(Vec2::new(500.0, 100.0));

        fire(&mut world, &mut physics, &input, Vec2::ZERO, 0.0);

        let velocities = bullet_velocities(&world);
        assert_eq!(velocities.len(), 4);
        let angles: Vec<f32> = velocities.iter().map(|v| v.to_angle()).collect();
        assert!(angles.iter().any(|a| a.abs() < 1e-4));
        assert!(angles.iter().any(|a| *a > 0.05) && angles.iter().any(|a| *a < -0.04));
        assert_eq!(current(&world).clip, 5);
    }

    #[test]
    fn every_shotgun_pellet_rolls_its_own_critical_hit() {
        let (mut world, mut physics) = setup();
        give(&mut world, crate::guns::GunKind::Shotgun);
        let player = world
            .query::<Entity>()
            .with::<&Player>()
            .iter()
            .next()
            .expect("player");
        let unlocked = std::collections::BTreeSet::from([crate::upgrades::Upgrade::CriticalHit]);
        let _ = world.insert_one(player, PlayerPerks(Perks::from_unlocked(&unlocked)));
        let input = clicked_at(Vec2::new(500.0, 100.0));

        shoot(
            &mut world,
            &mut physics,
            &input,
            ShotContext {
                camera: Vec2::ZERO,
                rounds: RoundTextures::white(),
                casing_texture: TextureId::WHITE,
                dt: 0.0,
                roll: ShotRoll {
                    spread: 0.0,
                    damages: [0.0; MAX_PELLETS],
                    fractions: [1.0; MAX_PELLETS],
                    perk_fractions: [1.0; MAX_PELLETS],
                    criticals: [1.0, 0.05, 1.0, 1.0],
                    eject: [0.5; 4],
                },
            },
            &mut Vec::new(),
        );

        let mut damages = bullet_damages(&world);
        damages.sort_unstable();
        assert_eq!(damages, vec![2, 2, 2, 4]);
    }

    #[test]
    fn holding_the_trigger_keeps_an_uzi_firing_but_not_a_pistol() {
        let (mut world, mut physics) = setup();
        give(&mut world, crate::guns::GunKind::Uzi);
        let input = held_at(Vec2::new(500.0, 100.0));
        for _ in 0..6 {
            fire(&mut world, &mut physics, &input, Vec2::ZERO, 0.1);
        }
        assert!(bullet_velocities(&world).len() >= 5);

        let (mut world, mut physics) = setup();
        for _ in 0..6 {
            fire(&mut world, &mut physics, &input, Vec2::ZERO, 0.3);
        }
        assert_eq!(bullet_velocities(&world).len(), 0);
    }

    #[test]
    fn the_m4_fires_a_four_round_burst_with_its_aim_locked() {
        let (mut world, mut physics) = setup();
        give(&mut world, crate::guns::GunKind::M4);
        let input = clicked_at(Vec2::new(500.0, 100.0));

        fire(&mut world, &mut physics, &input, Vec2::ZERO, 0.0);
        assert_eq!(bullet_velocities(&world).len(), 1);

        let idle = Input::default();
        for _ in 0..3 {
            fire(&mut world, &mut physics, &idle, Vec2::ZERO, 0.051);
        }
        assert_eq!(bullet_velocities(&world).len(), 4);
        assert_eq!(current(&world).clip, 28);
        assert!(current(&world).burst.is_none());
        for velocity in bullet_velocities(&world) {
            assert!(velocity.y.abs() < 0.01, "{velocity:?}");
        }
    }

    #[test]
    fn r_starts_a_reload_with_the_gun_specific_sound() {
        let (mut world, mut physics) = setup();
        give(&mut world, crate::guns::GunKind::Shotgun);
        for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
            if let Some(weapon) = loadout.current_mut() {
                weapon.clip = 1;
            }
        }
        let mut input = Input::default();
        input.press(KeyCode::KeyR);
        let mut sounds = Vec::new();

        fire_with_sounds(&mut world, &mut physics, &input, 0.016, &mut sounds);

        assert_eq!(
            sounds,
            vec![AudioCommand::Effect(SoundEffect::ShotgunReload)]
        );
        assert!(current(&world).reloading.is_some());
    }

    fn empty_clip(world: &mut World) {
        for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
            if let Some(weapon) = loadout.current_mut() {
                weapon.clip = 0;
            }
        }
    }

    #[test]
    fn holding_the_trigger_on_an_empty_auto_gun_clicks_only_once() {
        let (mut world, mut physics) = setup();
        give(&mut world, crate::guns::GunKind::PlasmaRifle);
        empty_clip(&mut world);
        let mut sounds = Vec::new();
        let target = Vec2::new(500.0, 100.0);
        fire_with_sounds(
            &mut world,
            &mut physics,
            &clicked_at(target),
            0.016,
            &mut sounds,
        );
        for _ in 0..30 {
            fire_with_sounds(
                &mut world,
                &mut physics,
                &held_at(target),
                0.016,
                &mut sounds,
            );
        }
        assert_eq!(sounds, vec![AudioCommand::Effect(SoundEffect::EnergyEmpty)]);
    }

    #[test]
    fn energy_guns_fizzle_and_powder_guns_click_when_empty() {
        use crate::guns::GunKind;
        for kind in GunKind::ALL {
            let energy = matches!(
                kind,
                GunKind::Blaster
                    | GunKind::PlasmaRifle
                    | GunKind::TeslaGun
                    | GunKind::Railgun
                    | GunKind::ArcCaster
            );
            let expected = if energy {
                SoundEffect::EnergyEmpty
            } else {
                SoundEffect::EmptyGunShoot
            };
            assert_eq!(kind.spec().empty, expected, "{kind:?}");
        }
    }

    #[test]
    fn an_empty_gun_clicks_and_an_empty_slot_stays_silent() {
        let (mut world, mut physics) = setup();
        for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
            if let Some(weapon) = loadout.current_mut() {
                weapon.clip = 0;
            }
        }
        let input = clicked_at(Vec2::new(500.0, 100.0));
        let mut sounds = Vec::new();
        fire_with_sounds(&mut world, &mut physics, &input, 0.016, &mut sounds);
        assert_eq!(
            sounds,
            vec![AudioCommand::Effect(SoundEffect::EmptyGunShoot)]
        );

        for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
            loadout.drop_current();
        }
        let mut sounds = Vec::new();
        fire_with_sounds(&mut world, &mut physics, &input, 0.016, &mut sounds);
        assert!(sounds.is_empty());
        assert_eq!(bullet_velocities(&world).len(), 0);
    }

    #[test]
    fn a_gun_keeps_its_own_clip_when_you_switch_away_and_back() {
        let (mut world, mut physics) = setup();
        let input = clicked_at(Vec2::new(500.0, 100.0));
        fire(&mut world, &mut physics, &input, Vec2::ZERO, 0.0);
        for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
            loadout.switch_to(1);
        }
        assert_eq!(current(&world).clip, 6);
        for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
            loadout.switch_to(0);
        }
        assert_eq!(current(&world).clip, 6);
    }

    #[test]
    fn a_shot_throws_out_one_casing() {
        let (mut world, mut physics) = setup();
        let input = clicked_at(Vec2::new(300.0, 100.0));

        fire(&mut world, &mut physics, &input, Vec2::ZERO, 1.0 / 60.0);

        let casings: Vec<crate::components::Casing> = world
            .query::<&crate::components::Casing>()
            .iter()
            .copied()
            .collect();
        assert_eq!(casings.len(), 1);
        assert!(!casings[0].settled && casings[0].height > 0.0);
    }

    #[test]
    fn no_shot_no_casing() {
        let (mut world, mut physics) = setup();
        let input = Input::default();

        fire(&mut world, &mut physics, &input, Vec2::ZERO, 1.0 / 60.0);

        assert_eq!(
            world.query::<&crate::components::Casing>().iter().count(),
            0
        );
    }

    fn rat(world: &mut World, physics: &mut Physics, at: Vec2) -> Entity {
        prefabs::ready_enemy(world, physics, crate::enemies::EnemyKind::Rat, at)
    }

    fn health(world: &World, entity: Entity) -> i32 {
        world
            .get::<&Health>(entity)
            .map_or(0, |health| health.current)
    }

    fn fly(world: &mut World, physics: &mut Physics, steps: usize) {
        for _ in 0..steps {
            simulate(world, physics, 1.0 / 60.0);
            bullet_hits(world, physics, &Textures::default(), &mut Vec::new(), &mut Fx::default());
        }
    }

    fn round(world: &mut World, physics: &mut Physics, at: Vec2, mutation: Mutation) -> Entity {
        prefabs::bullet(
            world,
            physics,
            at,
            Vec2::new(700.0, 0.0),
            Bullet::mutated(1, Some(mutation)),
            TextureId::WHITE,
        )
    }

    #[test]
    fn a_piercing_round_goes_through_the_first_enemy_and_stops_in_the_second() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let first = rat(&mut world, &mut physics, Vec2::new(300.0, 100.0));
        let second = rat(&mut world, &mut physics, Vec2::new(450.0, 100.0));
        let third = rat(&mut world, &mut physics, Vec2::new(600.0, 100.0));
        let full = health(&world, first);
        round(
            &mut world,
            &mut physics,
            Vec2::new(200.0, 100.0),
            Mutation::Piercing,
        );

        fly(&mut world, &mut physics, 40);

        assert_eq!(health(&world, first), full - 1);
        assert_eq!(health(&world, second), full - 1);
        assert_eq!(health(&world, third), full);
    }

    #[test]
    fn a_ricochet_round_bounces_off_the_wall_once() {
        let mut world = World::new();
        let mut physics = Physics::new();
        physics.add_walls(&[(5, 0), (5, 1), (5, 2), (5, 3)], 64.0);
        let bullet = round(
            &mut world,
            &mut physics,
            Vec2::new(200.0, 100.0),
            Mutation::Ricochet,
        );

        fly(&mut world, &mut physics, 20);

        assert!(world.get::<&Expiring>(bullet).is_err());
        let velocity = world.get::<&Velocity>(bullet).map_or(Vec2::ZERO, |v| v.0);
        assert!(velocity.x < -600.0, "{velocity:?}");
        assert_eq!(world.get::<&Bullet>(bullet).map_or(9, |b| b.bounces), 0);
    }

    #[test]
    fn a_plain_round_does_not_bounce() {
        let mut world = World::new();
        let mut physics = Physics::new();
        physics.add_walls(&[(5, 0), (5, 1), (5, 2), (5, 3)], 64.0);
        let bullet = prefabs::bullet(
            &mut world,
            &mut physics,
            Vec2::new(200.0, 100.0),
            Vec2::new(700.0, 0.0),
            Bullet::plain(1),
            TextureId::WHITE,
        );

        fly(&mut world, &mut physics, 20);

        assert!(world.get::<&Expiring>(bullet).is_ok());
    }

    #[test]
    fn an_explosive_round_hurts_the_enemies_around_the_hit_but_not_the_player() {
        let (mut world, mut physics) = setup();
        let player = world
            .query::<Entity>()
            .with::<&Player>()
            .iter()
            .next()
            .expect("player");
        let player_health = health(&world, player);
        let struck = rat(&mut world, &mut physics, Vec2::new(300.0, 160.0));
        let beside = rat(&mut world, &mut physics, Vec2::new(300.0, 220.0));
        let far = rat(&mut world, &mut physics, Vec2::new(300.0, 600.0));
        let full = health(&world, beside);
        round(
            &mut world,
            &mut physics,
            Vec2::new(150.0, 160.0),
            Mutation::Explosive,
        );

        fly(&mut world, &mut physics, 30);

        assert_eq!(health(&world, struck), full - 1);
        assert_eq!(health(&world, beside), full - 1);
        assert_eq!(health(&world, far), full);
        assert_eq!(health(&world, player), player_health);
    }

    #[test]
    fn a_venom_round_leaves_a_pool_that_spares_the_player() {
        let mut world = World::new();
        let mut physics = Physics::new();
        rat(&mut world, &mut physics, Vec2::new(300.0, 100.0));
        round(
            &mut world,
            &mut physics,
            Vec2::new(200.0, 100.0),
            Mutation::Venom,
        );

        fly(&mut world, &mut physics, 30);

        let pools: Vec<Entity> = world
            .query::<Entity>()
            .with::<(&PoisonPool, &Friendly)>()
            .iter()
            .collect();
        assert_eq!(pools.len(), 1);
    }

    #[test]
    fn frost_rounds_chill_an_enemy_and_enough_of_them_freeze_it() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let target = rat(&mut world, &mut physics, Vec2::new(300.0, 100.0));

        for hit in 1..=CHILL_STACKS_TO_FREEZE {
            chill(&mut world, target);
            if hit < CHILL_STACKS_TO_FREEZE {
                assert!(world.get::<&Chilled>(target).is_ok());
                assert!(world.get::<&Frozen>(target).is_err());
            }
        }

        assert!(world.get::<&Frozen>(target).is_ok());
    }

    #[test]
    fn a_chilled_enemy_moves_at_half_speed() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let slow = rat(&mut world, &mut physics, Vec2::new(100.0, 100.0));
        let quick = rat(&mut world, &mut physics, Vec2::new(100.0, 400.0));
        let _ = world.insert_one(
            slow,
            Chilled {
                remaining: 10.0,
                stacks: 1,
            },
        );
        for entity in [slow, quick] {
            if let Ok(mut velocity) = world.get::<&mut Velocity>(entity) {
                velocity.0 = Vec2::new(120.0, 0.0);
            }
        }

        for _ in 0..30 {
            simulate(&mut world, &mut physics, 1.0 / 60.0);
        }

        let moved = |entity| {
            world
                .get::<&Position>(entity)
                .map_or(0.0, |p| p.0.x - 100.0)
        };
        assert!(
            (moved(slow) * 2.0 - moved(quick)).abs() < 3.0,
            "{} {}",
            moved(slow),
            moved(quick)
        );
    }

    #[test]
    fn deadeye_makes_every_fifth_shot_a_critical_hit() {
        let mut weapon = Weapon::new(crate::guns::GunKind::Pistol);
        weapon.upgrade(crate::weapon_upgrades::GunMods {
            mutation: Some(Mutation::Deadeye),
            ..Default::default()
        });

        let crits: Vec<bool> = (0..10).map(|_| loadout::count_shot(&mut weapon)).collect();

        assert_eq!(
            crits,
            [
                false, false, false, false, true, false, false, false, false, true
            ]
        );
        let mut plain = Weapon::new(crate::guns::GunKind::Pistol);
        assert!((0..10).all(|_| !loadout::count_shot(&mut plain)));
    }

    #[test]
    fn the_loadout_hands_every_gun_its_own_tree_bonus() {
        let kind = crate::guns::GunKind::Pistol;
        let mut progress = crate::weapon_upgrades::WeaponProgress::default();
        progress.owned = (0..progress.tree(kind).nodes.len()).collect();
        let mods = crate::weapon_upgrades::WeaponMods::from_progress(
            &std::iter::once((kind, progress)).collect(),
        );
        let (mut world, _) = setup();
        for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
            loadout.apply_weapon_mods(mods);
        }

        let pistol = current(&world);
        assert_eq!(pistol.mods, mods.of(kind));
        assert_eq!(pistol.clip, pistol.clip_size());
        assert_eq!(
            pistol.spec().clip_size,
            mods.of(kind).apply(kind.spec()).clip_size
        );
        assert!(pistol.spec().damage_scale > 1.0 || mods.of(kind).damage == 0.0);
        let fresh = super::super::loadout::fresh_weapon(&world, crate::guns::GunKind::Uzi);
        assert_eq!(fresh.mods, crate::weapon_upgrades::GunMods::default());
        let mut dropped = Weapon::new(kind);
        dropped.clip = 1;
        for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
            loadout.drop_current();
            loadout::add_gun(loadout, dropped);
        }
        assert_eq!(current(&world).mods, mods.of(kind));
        assert_eq!(current(&world).clip, 1);
    }

    #[test]
    fn the_clip_does_not_drop_while_infinite_ammo_lasts_and_drops_after_it_expires() {
        let (mut world, mut physics) = setup();
        let player = world
            .query::<Entity>()
            .with::<&Player>()
            .iter()
            .next()
            .expect("the player exists");
        let clip = current(&world).clip;
        let input = clicked_at(Vec2::new(50.0, 0.0));
        world
            .insert_one(player, crate::components::InfiniteAmmo { remaining: 8.0 })
            .expect("the player exists");

        for _ in 0..(clip + 3) {
            fire(&mut world, &mut physics, &input, Vec2::ZERO, 1.0);
        }
        assert_eq!(current(&world).clip, clip);
        assert!(current(&world).reloading.is_none());

        world
            .remove_one::<crate::components::InfiniteAmmo>(player)
            .expect("the effect was present");
        fire(&mut world, &mut physics, &input, Vec2::ZERO, 1.0);
        assert_eq!(current(&world).clip, clip - 1);
    }

    #[test]
    fn infinite_ammo_cancels_a_reload_and_fires_from_an_empty_clip() {
        let (mut world, mut physics) = setup();
        for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
            if let Some(weapon) = loadout.current_mut() {
                weapon.clip = 0;
                weapon.reloading = Some(1.0);
            }
        }
        let player = world
            .query::<Entity>()
            .with::<&Player>()
            .iter()
            .next()
            .expect("the player exists");
        world
            .insert_one(player, crate::components::InfiniteAmmo { remaining: 8.0 })
            .expect("the player exists");
        let input = clicked_at(Vec2::new(50.0, 0.0));

        fire(&mut world, &mut physics, &input, Vec2::ZERO, 0.1);

        assert!(current(&world).reloading.is_none());
        assert_eq!(current(&world).clip, 0);
        assert_eq!(bullet_velocities(&world).len(), 1);
    }

    fn run_rounds(world: &mut World, physics: &mut Physics, frames: usize) {
        for _ in 0..frames {
            simulate(world, physics, 1.0 / 60.0);
            burn_out_rounds(world, physics, 1.0 / 60.0);
            bullet_hits(world, physics, &Textures::default(), &mut Vec::new(), &mut Fx::default());
        }
    }

    #[test]
    fn every_new_gun_fires_its_own_round_at_its_own_speed() {
        use crate::guns::GunKind;
        for (kind, round) in [
            (GunKind::Sniper, Round::Slug),
            (GunKind::Rpg, Round::Rocket),
            (GunKind::Minigun, Round::Tracer),
            (GunKind::Crossbow, Round::Bolt),
            (GunKind::Flamethrower, Round::Flame),
        ] {
            let (mut world, mut physics) = setup();
            give(&mut world, kind);
            fire(
                &mut world,
                &mut physics,
                &clicked_at(Vec2::new(500.0, 100.0)),
                Vec2::ZERO,
                0.0,
            );
            let fired: Vec<(Bullet, Vec2, bool)> = world
                .query::<(&Bullet, &Velocity, Option<&Heading>)>()
                .iter()
                .map(|(bullet, velocity, heading)| (*bullet, velocity.0, heading.is_some()))
                .collect();
            assert_eq!(fired.len(), 1, "{kind:?}");
            let (bullet, velocity, turned) = fired[0];
            assert_eq!(bullet.round, round, "{kind:?}");
            assert!(
                (velocity.length() - round.spec().speed).abs() < 0.5,
                "{kind:?}"
            );
            assert_eq!(turned, round.turns(), "{kind:?}");
        }
    }

    #[test]
    fn a_rocket_bursts_on_the_wall_and_hurts_every_enemy_around_with_full_damage() {
        let mut world = World::new();
        let mut physics = Physics::new();
        physics.add_walls(&[(5, 0), (5, 1), (5, 2)], 64.0);
        let near = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Rat,
            Vec2::new(250.0, 40.0),
        );
        let before = health(&world, near);
        prefabs::bullet(
            &mut world,
            &mut physics,
            Vec2::new(150.0, 96.0),
            Vec2::new(480.0, 0.0),
            Bullet::plain(2).as_round(Round::Rocket),
            TextureId::WHITE,
        );

        run_rounds(&mut world, &mut physics, 40);

        assert!(before - health(&world, near) >= 2 || world.get::<&Expiring>(near).is_ok());
        assert_eq!(
            world
                .query::<&Bullet>()
                .without::<&Expiring>()
                .iter()
                .count(),
            0
        );
    }

    #[test]
    fn a_flame_burns_out_in_the_air_after_its_short_life() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let flame = prefabs::bullet(
            &mut world,
            &mut physics,
            Vec2::new(100.0, 100.0),
            Vec2::new(380.0, 0.0),
            Bullet::plain(1).as_round(Round::Flame),
            TextureId::WHITE,
        );

        run_rounds(&mut world, &mut physics, 20);
        assert!(world.get::<&Expiring>(flame).is_err());
        run_rounds(&mut world, &mut physics, 10);
        assert!(world.get::<&Expiring>(flame).is_ok());
        assert!(world.get::<&Body>(flame).is_err());
    }

    #[test]
    fn a_sniper_slug_passes_through_two_enemies_and_stops_in_the_third() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let rats: Vec<Entity> = [300.0, 400.0, 500.0, 600.0]
            .into_iter()
            .map(|x| {
                prefabs::ready_enemy(
                    &mut world,
                    &mut physics,
                    crate::enemies::EnemyKind::Rat,
                    Vec2::new(x, 100.0),
                )
            })
            .collect();
        let before: Vec<i32> = rats.iter().map(|rat| health(&world, *rat)).collect();
        prefabs::bullet(
            &mut world,
            &mut physics,
            Vec2::new(200.0, 100.0),
            Vec2::new(1500.0, 0.0),
            Bullet::plain(1).as_round(Round::Slug),
            TextureId::WHITE,
        );

        run_rounds(&mut world, &mut physics, 30);

        let hurt: Vec<bool> = rats
            .iter()
            .zip(before)
            .map(|(rat, before)| health(&world, *rat) < before)
            .collect();
        assert_eq!(hurt, vec![true, true, true, false]);
    }

    #[test]
    fn a_tesla_spark_jumps_to_the_two_nearest_enemies_but_not_to_a_far_one() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let rats: Vec<Entity> = [
            Vec2::new(300.0, 100.0),
            Vec2::new(300.0, 250.0),
            Vec2::new(300.0, 400.0),
            Vec2::new(300.0, 900.0),
        ]
        .into_iter()
        .map(|at| {
            prefabs::ready_enemy(&mut world, &mut physics, crate::enemies::EnemyKind::Rat, at)
        })
        .collect();
        let before: Vec<i32> = rats.iter().map(|rat| health(&world, *rat)).collect();
        prefabs::bullet(
            &mut world,
            &mut physics,
            Vec2::new(200.0, 100.0),
            Vec2::new(600.0, 0.0),
            Bullet::plain(1).as_round(Round::Spark),
            TextureId::WHITE,
        );

        run_rounds(&mut world, &mut physics, 20);

        let hurt: Vec<bool> = rats
            .iter()
            .zip(before)
            .map(|(rat, before)| health(&world, *rat) < before)
            .collect();
        assert_eq!(hurt, vec![true, true, true, false]);
    }

    #[test]
    fn an_arc_jumps_along_four_more_targets_losing_a_quarter_each_time() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let dummies: Vec<Entity> = (0..6)
            .map(|index| {
                prefabs::training_dummy(
                    &mut world,
                    &mut physics,
                    Vec2::new(300.0, 100.0 + 120.0 * index as f32),
                    &crate::assets::Textures::default(),
                )
            })
            .collect();
        let first = world.get::<&Position>(dummies[0]).expect("dummy").0;
        prefabs::bullet(
            &mut world,
            &mut physics,
            Vec2::new(200.0, first.y),
            Vec2::new(600.0, 0.0),
            Bullet::plain(100).as_round(Round::Arc),
            TextureId::WHITE,
        );

        run_rounds(&mut world, &mut physics, 20);

        let taken: Vec<i32> = dummies
            .iter()
            .map(|dummy| prefabs::DUMMY_HEALTH - health(&world, *dummy))
            .collect();
        assert_eq!(taken, vec![100, 75, 56, 42, 32, 0]);
    }

    #[test]
    fn plasma_spatters_the_enemy_next_to_the_one_it_hits() {
        let mut world = World::new();
        let mut physics = Physics::new();
        let hit = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Rat,
            Vec2::new(300.0, 100.0),
        );
        let beside = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Rat,
            Vec2::new(300.0, 150.0),
        );
        let far = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            crate::enemies::EnemyKind::Rat,
            Vec2::new(300.0, 400.0),
        );
        let before = [hit, beside, far].map(|rat| health(&world, rat));
        prefabs::bullet(
            &mut world,
            &mut physics,
            Vec2::new(200.0, 100.0),
            Vec2::new(650.0, 0.0),
            Bullet::plain(2).as_round(Round::Plasma),
            TextureId::WHITE,
        );

        run_rounds(&mut world, &mut physics, 20);

        let after = [hit, beside, far].map(|rat| health(&world, rat));
        assert_eq!(before[0] - after[0], 2);
        assert_eq!(before[1] - after[1], 1);
        assert_eq!(before[2], after[2]);
    }
}
