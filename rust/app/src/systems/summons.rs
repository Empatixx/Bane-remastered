use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::components::{
    Animation, Bullet, Color, Enemy, Expiring, Ghost, Health, Layer, Light, PlaceTurret, Player,
    Position, Size, Spawning, Sprite, SummonGhosts, Summoned, ThenPlay, Turret, TurretStage,
};
use crate::geometry;
use crate::physics::Physics;
use crate::prefabs;
use crate::sheets::{ghost, turret};
use crate::systems::damage::{Damage, DamageSource};

const GHOST_COUNT: usize = 3;
const GHOST_SIZE: Vec2 = Vec2::new(72.0, 72.0);
const GHOST_LIFE: f32 = 12.0;
const GHOST_FADE: f32 = 1.0;
const GHOST_SPEED: f32 = 300.0;
const GHOST_STEERING: f32 = 5.0;
const GHOST_DAMAGE: i32 = 2;
const GHOST_COOLDOWN: f32 = 0.5;
const GHOST_BOB: f32 = 5.0;
const GHOST_ORBIT_RADIUS: f32 = 110.0;
const GHOST_ORBIT_SPEED: f32 = 1.6;

const TURRET_SIZE: Vec2 = Vec2::new(120.0, 120.0);
const TURRET_LIFE: f32 = 12.0;
const TURRET_RANGE: f32 = 600.0;
const TURRET_COOLDOWN: f32 = 0.35;
const TURRET_DAMAGE: i32 = 2;
const TURRET_BULLET_SPEED: f32 = 700.0;
const TURRET_FEET: Vec2 = Vec2::new(0.0, 24.0);
const TURRET_MUZZLE: Vec2 = Vec2::new(0.0, -6.0);
const TURRET_MUZZLE_REACH: f32 = 40.0;

const AIM_ANGLES: [f32; turret::AIM_ROWS] = [0.0, 45.0, 90.0, 135.0, 180.0];

pub(crate) fn spawn_summons(world: &mut World, textures: &Textures) {
    let Some((player, origin)) = world
        .query::<(Entity, &Position)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(entity, position)| (entity, position.0))
    else {
        return;
    };
    if world.remove_one::<SummonGhosts>(player).is_ok() {
        for index in 0..GHOST_COUNT {
            let phase = index as f32 / GHOST_COUNT as f32 * std::f32::consts::TAU;
            spawn_ghost(
                world,
                origin + Vec2::from_angle(phase) * 60.0,
                phase,
                textures,
            );
        }
    }
    if world.remove_one::<PlaceTurret>(player).is_ok() {
        spawn_turret(world, origin + TURRET_FEET, textures);
    }
}

fn spawn_ghost(world: &mut World, position: Vec2, phase: f32, textures: &Textures) -> Entity {
    world.spawn((
        Ghost {
            life: GHOST_LIFE,
            cooldown: 0.0,
            phase,
            home: position,
            velocity: Vec2::ZERO,
        },
        Position(position),
        Size(GHOST_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(
            textures.get(tex::ARTEFACTS_GHOST),
            0,
            ghost::FLOAT.row,
            ghost::COLUMNS,
            ghost::ROWS,
        ),
        Animation::new(ghost::COLUMNS, ghost::ROWS, ghost::FLOAT),
        Layer(3),
        Light::rgb(Vec2::ZERO, 90.0, 220.0, 110.0, 130.0, 0.6),
    ))
}

fn spawn_turret(world: &mut World, position: Vec2, textures: &Textures) -> Entity {
    world.spawn((
        Turret {
            stage: TurretStage::Rising,
            stage_time: 0.0,
            life: TURRET_LIFE,
            cooldown: 0.0,
            recoil: 0.0,
            aim: 0,
            flipped: false,
        },
        Position(position),
        Size(TURRET_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(
            textures.get(tex::ARTEFACTS_TURRET),
            0,
            turret::RISE.row,
            turret::COLUMNS,
            turret::ROWS,
        ),
        Animation::new(turret::COLUMNS, turret::ROWS, turret::RISE),
        Layer(1),
        crate::components::YSort::bottom(TURRET_SIZE),
    ))
}

fn nearest_enemy(world: &World, center: Vec2, range: Option<f32>) -> Option<(Entity, Vec2)> {
    world
        .query::<(Entity, &Position, &Health)>()
        .with::<&Enemy>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .filter(|(_, position, health)| {
            health.current > 0 && range.is_none_or(|range| position.0.distance(center) <= range)
        })
        .min_by(|a, b| {
            a.1.0
                .distance_squared(center)
                .total_cmp(&b.1.0.distance_squared(center))
        })
        .map(|(entity, position, _)| (entity, position.0))
}

fn overlapped_enemy(world: &World, center: Vec2, size: Vec2) -> Option<Entity> {
    world
        .query::<(Entity, &Position, &Size, &Health)>()
        .with::<&Enemy>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .find(|(_, position, enemy_size, health)| {
            health.current > 0 && geometry::overlaps(center, size, position.0, enemy_size.0)
        })
        .map(|(entity, _, _, _)| entity)
}

pub(crate) fn tick_ghosts(world: &mut World, dt: f32) {
    let owner = world
        .query::<&Position>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|position| position.0);
    let ghosts: Vec<(Entity, Ghost)> = world
        .query::<(Entity, &Ghost)>()
        .iter()
        .map(|(entity, ghost)| (entity, *ghost))
        .collect();
    let mut gone = Vec::new();
    for (entity, mut state) in ghosts {
        state.life -= dt;
        if state.life <= 0.0 {
            gone.push(entity);
            continue;
        }
        state.cooldown = (state.cooldown - dt).max(0.0);
        let age = GHOST_LIFE - state.life;
        let goal = match nearest_enemy(world, state.home, None) {
            Some((_, position)) => Some(position),
            None => owner.map(|owner| {
                owner + Vec2::from_angle(state.phase + age * GHOST_ORBIT_SPEED) * GHOST_ORBIT_RADIUS
            }),
        };
        if let Some(goal) = goal {
            let wanted = (goal - state.home).normalize_or_zero() * GHOST_SPEED;
            let wanted = wanted * (goal.distance(state.home) / 40.0).min(1.0);
            let blend = (GHOST_STEERING * dt).min(1.0);
            state.velocity = state.velocity.lerp(wanted, blend);
        }
        state.home += state.velocity * dt;
        let bob = Vec2::new(0.0, (age * 4.0 + state.phase).sin() * GHOST_BOB);
        let center = state.home + bob;
        let victim = if state.cooldown <= 0.0 {
            overlapped_enemy(world, center, GHOST_SIZE)
        } else {
            None
        };
        let mut hit = false;
        if let Some(victim) = victim {
            hit = crate::systems::damage::deal(
                world,
                Damage {
                    target: victim,
                    amount: GHOST_DAMAGE,
                    source: DamageSource::Bite,
                },
            );
            if hit {
                state.cooldown = GHOST_COOLDOWN;
            }
        }
        let flipped = state.velocity.x < -1.0;
        let fade = (state.life / GHOST_FADE).clamp(0.0, 1.0);
        if let Ok((position, color, animation, light)) =
            world.query_one_mut::<(&mut Position, &mut Color, &mut Animation, &mut Light)>(entity)
        {
            position.0 = center;
            color.0.w = fade;
            light.intensity = 0.6 * fade;
            if hit {
                animation.play(crate::clip::Clip {
                    flipped,
                    ..ghost::HIT
                });
            } else {
                animation.face(flipped);
            }
        }
        if hit {
            let _ = world.insert_one(
                entity,
                ThenPlay(crate::clip::Clip {
                    flipped,
                    ..ghost::FLOAT
                }),
            );
        }
        if let Ok(mut ghost) = world.get::<&mut Ghost>(entity) {
            *ghost = state;
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

fn aim_row(direction: Vec2) -> usize {
    let from_down = direction.x.abs().atan2(direction.y).to_degrees();
    AIM_ANGLES
        .iter()
        .enumerate()
        .min_by(|a, b| (a.1 - from_down).abs().total_cmp(&(b.1 - from_down).abs()))
        .map_or(0, |(index, _)| index)
}

pub(crate) fn tick_turrets(world: &mut World, physics: &mut Physics, textures: &Textures, dt: f32) {
    let turrets: Vec<(Entity, Turret, Vec2)> = world
        .query::<(Entity, &Turret, &Position)>()
        .iter()
        .map(|(entity, turret, position)| (entity, *turret, position.0))
        .collect();
    let mut gone = Vec::new();
    for (entity, mut state, position) in turrets {
        state.stage_time += dt;
        match state.stage {
            TurretStage::Rising => {
                if state.stage_time >= turret::RISE_SECONDS {
                    state.stage = TurretStage::Active;
                    state.stage_time = 0.0;
                    set_clip(world, entity, |_| {
                        Some(turret::idle(state.aim, state.flipped))
                    });
                }
            }
            TurretStage::Active => {
                state.life -= dt;
                state.cooldown = (state.cooldown - dt).max(0.0);
                state.recoil = (state.recoil - dt).max(0.0);
                if state.life <= 0.0 {
                    state.stage = TurretStage::Crumbling;
                    state.stage_time = 0.0;
                    set_clip(world, entity, |_| Some(turret::CRUMBLE));
                } else {
                    let muzzle = position + TURRET_MUZZLE;
                    let target = nearest_enemy(world, muzzle, Some(TURRET_RANGE));
                    let mut fired = false;
                    if let Some((_, aim_at)) = target {
                        let direction = (aim_at - muzzle).normalize_or_zero();
                        state.aim = aim_row(direction);
                        state.flipped = direction.x < 0.0;
                        if state.cooldown <= 0.0 {
                            fire(world, physics, textures, muzzle, direction);
                            state.cooldown = TURRET_COOLDOWN;
                            state.recoil = turret::RECOIL_SECONDS;
                            fired = true;
                        }
                    }
                    let (aim, flipped, recoil) = (state.aim, state.flipped, state.recoil);
                    set_clip(world, entity, |current| {
                        if fired {
                            Some(turret::recoil(aim, flipped))
                        } else if recoil > 0.0 && current.row == 2 + aim {
                            Some(crate::clip::Clip { flipped, ..current })
                        } else {
                            Some(turret::idle(aim, flipped))
                        }
                    });
                }
            }
            TurretStage::Crumbling => {
                if state.stage_time >= turret::CRUMBLE_SECONDS {
                    gone.push(entity);
                }
            }
        }
        if let Ok(mut turret) = world.get::<&mut Turret>(entity) {
            *turret = state;
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
}

fn set_clip(
    world: &World,
    entity: Entity,
    choose: impl FnOnce(crate::clip::Clip) -> Option<crate::clip::Clip>,
) {
    if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
        if let Some(clip) = choose(animation.clip()) {
            animation.switch_to(clip);
        }
    }
}

fn fire(
    world: &mut World,
    physics: &mut Physics,
    textures: &Textures,
    muzzle: Vec2,
    direction: Vec2,
) {
    let shard = prefabs::bullet(
        world,
        physics,
        muzzle + direction * TURRET_MUZZLE_REACH,
        direction * TURRET_BULLET_SPEED,
        Bullet::plain(TURRET_DAMAGE),
        textures.get(tex::ARTEFACTS_BONE_SHARD),
    );
    let _ = world.insert_one(shard, Summoned);
}

#[cfg(test)]
mod tests {
    use crate::components::HitFlash;
    use crate::enemies::EnemyKind;

    use super::*;

    fn arena() -> (World, Physics, Textures) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(1000.0, 1000.0),
            &textures,
        );
        (world, physics, textures)
    }

    fn player_of(world: &World) -> Entity {
        world
            .query::<Entity>()
            .with::<&Player>()
            .iter()
            .next()
            .expect("a player")
    }

    fn ghosts(world: &World) -> usize {
        world.query::<&Ghost>().iter().count()
    }

    fn turrets(world: &World) -> usize {
        world.query::<&Turret>().iter().count()
    }

    fn bullets(world: &World) -> Vec<(Vec2, Vec2)> {
        world
            .query::<(&Position, &crate::components::Velocity)>()
            .with::<&Bullet>()
            .iter()
            .map(|(position, velocity)| (position.0, velocity.0))
            .collect()
    }

    fn health(world: &World, enemy: Entity) -> i32 {
        world.get::<&Health>(enemy).map_or(-1, |h| h.current)
    }

    #[test]
    fn three_ghosts_spawn_and_despawn_after_twelve_seconds() {
        let (mut world, _, textures) = arena();
        let player = player_of(&world);
        world.insert_one(player, SummonGhosts).expect("player");
        spawn_summons(&mut world, &textures);
        assert_eq!(ghosts(&world), 3);
        assert!(world.get::<&SummonGhosts>(player).is_err());
        for _ in 0..(11 * 60) {
            tick_ghosts(&mut world, 1.0 / 60.0);
        }
        assert_eq!(ghosts(&world), 3);
        for _ in 0..90 {
            tick_ghosts(&mut world, 1.0 / 60.0);
        }
        assert_eq!(ghosts(&world), 0);
    }

    #[test]
    fn ghosts_fade_out_during_the_last_second() {
        let (mut world, _, textures) = arena();
        let player = player_of(&world);
        world.insert_one(player, SummonGhosts).expect("player");
        spawn_summons(&mut world, &textures);
        tick_ghosts(&mut world, 11.5);
        let alpha = world
            .query::<&Color>()
            .with::<&Ghost>()
            .iter()
            .next()
            .map(|color| color.0.w)
            .expect("a ghost");
        assert!((0.0..1.0).contains(&alpha) && alpha < 0.6);
    }

    #[test]
    fn a_ghost_flies_through_to_the_nearest_enemy_and_hits_it_with_a_cooldown() {
        let (mut world, mut physics, textures) = arena();
        let enemy = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Golem,
            Vec2::new(1400.0, 1000.0),
        );
        let before = health(&world, enemy);
        spawn_ghost(&mut world, Vec2::new(1400.0, 1000.0), 0.0, &textures);
        tick_ghosts(&mut world, 1.0 / 60.0);
        assert_eq!(health(&world, enemy), before - 2);
        let _ = world.remove_one::<HitFlash>(enemy);
        tick_ghosts(&mut world, 0.3);
        assert_eq!(health(&world, enemy), before - 2);
        tick_ghosts(&mut world, 0.3);
        assert_eq!(health(&world, enemy), before - 4);
    }

    #[test]
    fn ghosts_move_towards_an_enemy_and_orbit_the_player_without_one() {
        let (mut world, mut physics, textures) = arena();
        let ghost = spawn_ghost(&mut world, Vec2::new(1000.0, 1000.0), 0.0, &textures);
        for _ in 0..120 {
            tick_ghosts(&mut world, 1.0 / 60.0);
        }
        let orbit = world.get::<&Position>(ghost).expect("ghost").0;
        assert!(orbit.distance(Vec2::new(1000.0, 1000.0)) < 200.0);
        prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(1000.0, 1900.0),
        );
        let start = world.get::<&Position>(ghost).expect("ghost").0;
        for _ in 0..30 {
            tick_ghosts(&mut world, 1.0 / 60.0);
        }
        let moved = world.get::<&Position>(ghost).expect("ghost").0;
        assert!(moved.y > start.y + 20.0);
    }

    fn place_turret(world: &mut World, textures: &Textures) {
        let player = player_of(world);
        world.insert_one(player, PlaceTurret).expect("player");
        spawn_summons(world, textures);
    }

    fn run_turret(world: &mut World, physics: &mut Physics, textures: &Textures, seconds: f32) {
        let steps = (seconds * 60.0).round() as usize;
        for _ in 0..steps {
            tick_turrets(world, physics, textures, 1.0 / 60.0);
        }
    }

    #[test]
    fn the_turret_spawns_at_the_players_feet_and_rises_before_it_fires() {
        let (mut world, mut physics, textures) = arena();
        let player = player_of(&world);
        prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(1300.0, 1000.0),
        );
        place_turret(&mut world, &textures);
        assert_eq!(turrets(&world), 1);
        assert!(world.get::<&PlaceTurret>(player).is_err());
        run_turret(&mut world, &mut physics, &textures, 0.3);
        assert!(bullets(&world).is_empty());
        run_turret(&mut world, &mut physics, &textures, 0.3);
        assert!(!bullets(&world).is_empty());
    }

    #[test]
    fn the_turret_fires_a_bone_shard_towards_an_enemy_in_range() {
        let (mut world, mut physics, textures) = arena();
        prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(1400.0, 1024.0),
        );
        place_turret(&mut world, &textures);
        run_turret(&mut world, &mut physics, &textures, 0.6);
        let shots = bullets(&world);
        assert_eq!(shots.len(), 1);
        assert!(shots[0].1.x > 600.0 && shots[0].1.y.abs() < 100.0);
        run_turret(&mut world, &mut physics, &textures, 0.4);
        assert_eq!(bullets(&world).len(), 2);
    }

    #[test]
    fn the_turret_holds_fire_without_an_enemy_in_range() {
        let (mut world, mut physics, textures) = arena();
        place_turret(&mut world, &textures);
        run_turret(&mut world, &mut physics, &textures, 2.0);
        assert!(bullets(&world).is_empty());
        prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(1900.0, 1000.0),
        );
        run_turret(&mut world, &mut physics, &textures, 1.0);
        assert!(bullets(&world).is_empty());
    }

    #[test]
    fn the_turret_crumbles_and_despawns_after_its_lifetime() {
        let (mut world, mut physics, textures) = arena();
        place_turret(&mut world, &textures);
        run_turret(&mut world, &mut physics, &textures, 12.6);
        let stage = world.query::<&Turret>().iter().next().map(|t| t.stage);
        assert_eq!(stage, Some(TurretStage::Crumbling));
        run_turret(&mut world, &mut physics, &textures, 0.6);
        assert_eq!(turrets(&world), 0);
    }

    #[test]
    fn the_aim_row_follows_the_direction_of_the_target() {
        assert_eq!(aim_row(Vec2::new(0.0, 1.0)), 0);
        assert_eq!(aim_row(Vec2::new(1.0, 0.0)), 2);
        assert_eq!(aim_row(Vec2::new(-1.0, 0.0)), 2);
        assert_eq!(aim_row(Vec2::new(0.0, -1.0)), 4);
        assert_eq!(aim_row(Vec2::new(1.0, 1.0).normalize()), 1);
    }

    #[test]
    fn a_ghost_skips_a_dying_enemy_and_bites_the_live_one_it_overlaps() {
        let (mut world, mut physics, textures) = arena();
        let at = Vec2::new(1400.0, 1000.0);
        let dying = prefabs::ready_enemy(&mut world, &mut physics, EnemyKind::Golem, at);
        if let Ok(mut health) = world.get::<&mut Health>(dying) {
            health.current = 0;
        }
        let alive = prefabs::ready_enemy(&mut world, &mut physics, EnemyKind::Golem, at);
        let before = health(&world, alive);
        spawn_ghost(&mut world, at, 0.0, &textures);

        tick_ghosts(&mut world, 1.0 / 60.0);

        assert_eq!(health(&world, alive), before - 2);
        assert_eq!(health(&world, dying), 0);
    }

    #[test]
    fn turret_shards_are_marked_summoned() {
        let (mut world, mut physics, textures) = arena();
        prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(1400.0, 1024.0),
        );
        place_turret(&mut world, &textures);
        run_turret(&mut world, &mut physics, &textures, 0.6);
        assert_eq!(
            world.query::<&Summoned>().with::<&Bullet>().iter().count(),
            1
        );
    }
}
