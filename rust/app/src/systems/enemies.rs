use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::components::{
    Animation, Boss, Charge, Chase, Clip, Color, Dash, Detour, Enemy, Expiring, Frozen, Health,
    HitFlash, Layer, Outline, Player, Position, Puddle, Revives, Rooted, Size, Smash, Spawning,
    Sprite, Velocity,
};
use crate::flow::FlowField;
use crate::navigation::Navigation;
use crate::physics::Physics;
use crate::systems::crowd::{SHOOTER_FAR, SHOOTER_NEAR};
use crate::systems::unstick::DETOUR_ANGLE;

pub(crate) const FLASH_SECONDS: f32 = 0.3;
const DASH_PERIOD: f32 = 3.0;
const DASH_TRAIL_INTERVAL: f32 = 0.04;
const DASH_DURATION: f32 = 1.0;

fn player_placement(world: &World) -> Option<(Vec2, Vec2)> {
    world
        .query::<(&Position, &Size)>()
        .with::<&Player>()
        .iter()
        .next()
        .map(|(position, size)| (position.0, size.0))
}

pub(crate) fn update_flow(
    flow: &mut FlowField,
    world: &World,
    physics: &Physics,
    navigation: &Navigation,
    dt: f32,
) {
    if let Some((target, _)) = player_placement(world) {
        flow.update(target, navigation, physics, dt);
    }
}

pub(crate) fn chase_player(world: &mut World, flow: &FlowField) {
    let target = player_placement(world).map(|(position, _)| position);
    for (position, chase, velocity, rooted, shooter, detour, frozen, charge) in world
        .query_mut::<(
            &Position,
            &Chase,
            &mut Velocity,
            Option<&Rooted>,
            Option<(&Smash, Option<&Boss>)>,
            Option<&Detour>,
            Option<&Frozen>,
            Option<&Charge>,
        )>()
        .without::<&Expiring>()
        .without::<&Spawning>()
    {
        if rooted.is_some() || frozen.is_some() {
            velocity.0 = Vec2::ZERO;
            continue;
        }
        if let Some(charge) = charge {
            velocity.0 = charge.direction * chase.speed;
            continue;
        }
        let Some(target) = target else {
            velocity.0 = Vec2::ZERO;
            continue;
        };
        let distance = position.0.distance(target);
        if shooter.is_some_and(|(_, boss)| boss.is_none()) && distance < SHOOTER_FAR {
            velocity.0 = if distance < SHOOTER_NEAR {
                (position.0 - target).normalize_or_zero() * chase.speed
            } else {
                Vec2::ZERO
            };
            continue;
        }
        velocity.0 = flow.heading(position.0, target) * chase.speed;
        if let Some(detour) = detour {
            velocity.0 = Vec2::from_angle(detour.turn * DETOUR_ANGLE).rotate(velocity.0);
        }
    }
}

pub(crate) fn face_player(world: &mut World) {
    let Some((target, _)) = player_placement(world) else {
        return;
    };
    for (position, animation) in world
        .query_mut::<(&Position, &mut Animation)>()
        .with::<&Enemy>()
        .without::<&Expiring>()
        .without::<&Charge>()
    {
        animation.face(target.x < position.0.x);
    }
}

pub(crate) fn tick_spawning(world: &mut World, physics: &mut Physics, dt: f32) {
    let mut finished = Vec::new();
    for (entity, spawning, color, puddle, animation, enemy) in world.query_mut::<(
        Entity,
        &mut Spawning,
        &mut Color,
        Option<&Puddle>,
        Option<&mut Animation>,
        Option<&Enemy>,
    )>() {
        let before = spawning.remaining;
        spawning.remaining -= dt;
        if puddle.is_some() {
            let reform = enemy.and_then(|enemy| reform_clip(enemy.kind));
            if let (Some(reform), Some(animation)) = (reform, animation) {
                let lead = reform.frames as f32 * reform.frame_time;
                if before > lead && spawning.remaining <= lead {
                    let facing = animation.flipped();
                    animation.play(Clip {
                        flipped: facing,
                        ..reform
                    });
                }
            }
        } else {
            color.0.w = (1.0 - spawning.remaining / spawning.total).clamp(0.0, 1.0);
        }
        if spawning.remaining <= 0.0 {
            finished.push(entity);
        }
    }
    for entity in finished {
        crate::prefabs::finish_spawning(world, physics, entity);
        if world.remove_one::<Puddle>(entity).is_ok() {
            stand_up(world, entity);
        }
    }
}

/// Seconds a puddle lies on the floor before it is a slime again.
pub(crate) const REVIVE_SECONDS: f32 = 10.0;
const REFORM_FRAME_TIME: f32 = 0.08;
const REFORM_FRAMES: usize = 6;

fn reform_clip(kind: crate::enemies::EnemyKind) -> Option<Clip> {
    kind.spec().reform_row.map(|row| Clip {
        row,
        frames: REFORM_FRAMES,
        frame_time: REFORM_FRAME_TIME,
        looping: false,
        flipped: false,
    })
}

fn stand_up(world: &mut World, entity: Entity) {
    let Ok(kind) = world.get::<&Enemy>(entity).map(|enemy| enemy.kind) else {
        return;
    };
    if let Ok(mut health) = world.get::<&mut Health>(entity) {
        health.current = health.max;
    }
    if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
        let facing = animation.flipped();
        animation.play(Clip {
            flipped: facing,
            ..kind.spec().idle_clip()
        });
    }
}

/// The first death of an enemy that revives: it melts into a puddle that cannot be hit and
/// lies still until it re-forms, the second death is for good.
fn collapse_into_puddles(world: &mut World, physics: &mut Physics) {
    let melting: Vec<(Entity, Enemy)> = world
        .query::<(Entity, &Health, &Enemy)>()
        .with::<&Revives>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .filter(|(_, health, _)| health.current <= 0)
        .map(|(entity, _, enemy)| (entity, *enemy))
        .collect();
    for (entity, enemy) in melting {
        physics.detach(world, entity);
        let _ = world.remove_one::<Revives>(entity);
        let _ = world.remove_one::<Frozen>(entity);
        let _ = world.insert(
            entity,
            (
                Puddle,
                Spawning {
                    remaining: REVIVE_SECONDS,
                    total: REVIVE_SECONDS,
                },
            ),
        );
        if let Ok(mut velocity) = world.get::<&mut Velocity>(entity) {
            velocity.0 = Vec2::ZERO;
        }
        if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
            let facing = animation.flipped();
            animation.play(Clip {
                flipped: facing,
                ..enemy.death
            });
        }
    }
}

pub(crate) fn dash(world: &mut World, dt: f32) {
    let mut trail = Vec::new();
    for (entity, dash, chase) in world
        .query_mut::<(Entity, &mut Dash, &mut Chase)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
    {
        dash.cycle = (dash.cycle + dt) % DASH_PERIOD;
        let dashing = dash.cycle < DASH_DURATION;
        chase.speed = if dashing {
            dash.base_speed + dash.bonus_speed
        } else {
            dash.base_speed
        };
        dash.trail -= dt;
        if dashing && dash.trail <= 0.0 {
            dash.trail = DASH_TRAIL_INTERVAL;
            trail.push(entity);
        }
    }
    for entity in trail {
        let copy = world
            .query_one_mut::<(&Position, &Size, &Sprite, Option<&Layer>)>(entity)
            .map(|(position, size, sprite, layer)| {
                (
                    *position,
                    *size,
                    *sprite,
                    layer.copied().unwrap_or(Layer(0)),
                )
            });
        if let Ok((position, size, sprite, layer)) = copy {
            super::spawn_afterimage(world, position, size, sprite, layer);
        }
    }
}

pub(crate) fn flash_hits(world: &mut World, dt: f32) {
    let mut fading = Vec::new();
    let mut done = Vec::new();
    for (entity, flash) in world.query_mut::<(Entity, &mut HitFlash)>() {
        flash.remaining -= dt;
        if flash.remaining <= 0.0 {
            done.push(entity);
        } else {
            let strength = (flash.remaining / FLASH_SECONDS).clamp(0.0, 1.0);
            fading.push((entity, strength));
        }
    }
    for (entity, strength) in fading {
        let _ = world.insert_one(entity, Outline(Vec4::new(1.0, 0.0, 0.0, strength)));
    }
    for entity in done {
        let _ = world.remove_one::<HitFlash>(entity);
        let _ = world.remove_one::<Outline>(entity);
    }
}

/// The one place where enemies die: every death effect sees each kill before it is reaped,
/// whichever system dealt the killing blow.
pub(crate) fn resolve_deaths(world: &mut World, physics: &mut Physics, textures: &Textures) {
    collapse_into_puddles(world, physics);
    super::skills::death_bursts(world, physics, textures);
    super::skills::death_clouds(world, textures);
    super::skills::death_heals(world);
    super::bosses::clean_up_after_boss(world);
    reap_dead(world, physics);
}

pub(super) fn reap_dead(world: &mut World, physics: &mut Physics) {
    let dead: Vec<(Entity, Enemy)> = world
        .query::<(Entity, &Health, &Enemy)>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .filter(|(_, health, _)| health.current <= 0)
        .map(|(entity, _, enemy)| (entity, *enemy))
        .collect();
    for (entity, enemy) in dead {
        physics.detach(world, entity);
        let _ = world.insert_one(entity, Expiring);
        let _ = world.remove_one::<Frozen>(entity);
        if let Ok(mut velocity) = world.get::<&mut Velocity>(entity) {
            velocity.0 = Vec2::ZERO;
        }
        if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
            let facing = animation.flipped();
            animation.play(Clip {
                flipped: facing,
                ..enemy.death
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec4;

    use crate::assets::Textures;
    use crate::components::Body;
    use crate::enemies::{COMMON_SPAWN_SECONDS, EnemyKind};
    use crate::navigation::Navigation;
    use crate::prefabs;

    use super::*;

    fn arena() -> (World, Physics) {
        let mut world = World::new();
        let mut physics = Physics::new();
        prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(0.0, 0.0),
            &Textures::default(),
        );
        (world, physics)
    }

    #[test]
    fn enemy_runs_towards_the_player() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );

        chase_player(&mut world, &FlowField::default());

        let velocity = world.get::<&Velocity>(rat).map(|v| v.0).ok();
        assert!(
            velocity.is_some_and(|v| v.x < 0.0 && v.y.abs() < 0.001),
            "{velocity:?}"
        );
    }

    #[test]
    fn enemy_without_health_dies_and_loses_its_body() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );
        if let Ok(mut health) = world.get::<&mut Health>(rat) {
            health.current = 0;
        }

        reap_dead(&mut world, &mut physics);

        assert!(world.get::<&Expiring>(rat).is_ok());
        assert!(world.get::<&Body>(rat).is_err());
    }

    #[test]
    fn living_enemy_is_left_alone() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );

        reap_dead(&mut world, &mut physics);

        assert!(world.get::<&Expiring>(rat).is_err());
        assert!(world.get::<&Body>(rat).is_ok());
    }

    #[test]
    fn a_spawning_enemy_is_harmless_until_the_spawn_ends() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::enemy(
            &mut world,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
            0,
            &Textures::default(),
        );

        chase_player(&mut world, &FlowField::default());
        let velocity = world.get::<&Velocity>(rat).map(|v| v.0).ok();
        assert_eq!(velocity, Some(Vec2::ZERO));
        assert!(world.get::<&Body>(rat).is_err());

        tick_spawning(&mut world, &mut physics, COMMON_SPAWN_SECONDS / 2.0);
        let alpha = world.get::<&Color>(rat).map(|c| c.0.w).unwrap_or(-1.0);
        assert!((alpha - 0.5).abs() < 0.01, "{alpha}");
        assert!(world.get::<&Spawning>(rat).is_ok());

        tick_spawning(&mut world, &mut physics, COMMON_SPAWN_SECONDS);
        assert!(world.get::<&Spawning>(rat).is_err());
        assert!(world.get::<&Body>(rat).is_ok());
        assert_eq!(world.get::<&Color>(rat).map(|c| c.0).ok(), Some(Vec4::ONE));
    }

    #[test]
    fn ghosts_dash_for_a_second_out_of_every_three() {
        let (mut world, mut physics) = arena();
        let ghost = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Ghost,
            Vec2::new(300.0, 0.0),
        );
        let base = EnemyKind::Ghost.spec().speed();
        let speed = |world: &World| world.get::<&Chase>(ghost).map(|c| c.speed).unwrap_or(0.0);

        dash(&mut world, 0.5);
        assert!(speed(&world) > base * 3.0);
        dash(&mut world, 1.0);
        assert!((speed(&world) - base).abs() < 0.01);
        dash(&mut world, 1.6);
        assert!(speed(&world) > base * 3.0);
    }

    #[test]
    fn a_hit_outlines_the_enemy_in_red_and_fades_back() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );
        let _ = world.insert_one(
            rat,
            HitFlash {
                remaining: FLASH_SECONDS,
            },
        );

        flash_hits(&mut world, 0.01);
        let outline = world
            .get::<&Outline>(rat)
            .map(|o| o.0)
            .unwrap_or(Vec4::ZERO);
        assert!(
            outline.x == 1.0 && outline.y == 0.0 && outline.w > 0.9,
            "{outline:?}"
        );
        assert_eq!(world.get::<&Color>(rat).map(|c| c.0).ok(), Some(Vec4::ONE));

        flash_hits(&mut world, FLASH_SECONDS);
        assert!(world.get::<&HitFlash>(rat).is_err());
        assert!(world.get::<&Outline>(rat).is_err());
    }

    #[test]
    fn an_enemy_steers_around_a_wall_instead_of_into_it() {
        let mut world = World::new();
        let mut physics = Physics::new();
        prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(600.0, 160.0),
            &Textures::default(),
        );
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(100.0, 160.0),
        );
        let wall: Vec<(usize, usize)> = (0..5).map(|row| (4, row)).collect();
        let navigation = Navigation::new(&wall);
        let mut flow = FlowField::default();

        update_flow(&mut flow, &world, &physics, &navigation, 0.016);
        chase_player(&mut world, &flow);

        let velocity = world.get::<&Velocity>(rat).map(|v| v.0).unwrap_or_default();
        assert!(velocity.x > 0.0 && velocity.y > 0.0, "{velocity:?}");
    }

    #[test]
    fn a_slime_closes_in_to_shooting_range_then_holds_and_backs_off_when_pressed() {
        let (mut world, mut physics) = arena();
        let slime = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Slime,
            Vec2::new(600.0, 0.0),
        );
        let flow = FlowField::default();
        let velocity = |world: &World| {
            world
                .get::<&Velocity>(slime)
                .map(|v| v.0)
                .unwrap_or_default()
        };
        let place = |world: &mut World, x: f32| {
            if let Ok(mut position) = world.get::<&mut Position>(slime) {
                position.0 = Vec2::new(x, 0.0);
            }
        };

        chase_player(&mut world, &flow);
        assert!(velocity(&world).x < 0.0);

        place(&mut world, SHOOTER_NEAR + 40.0);
        chase_player(&mut world, &flow);
        assert_eq!(velocity(&world), Vec2::ZERO);

        place(&mut world, SHOOTER_NEAR - 40.0);
        chase_player(&mut world, &flow);
        assert!(velocity(&world).x > 0.0);
    }

    #[test]
    fn an_enemy_on_a_detour_turns_away_from_its_heading() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );
        let _ = world.insert_one(
            rat,
            Detour {
                remaining: 0.5,
                turn: 1.0,
            },
        );

        chase_player(&mut world, &FlowField::default());

        let velocity = world.get::<&Velocity>(rat).map(|v| v.0).unwrap_or_default();
        assert!(velocity.y.abs() > velocity.x.abs() * 0.5, "{velocity:?}");
        assert!(velocity.x < 0.0, "{velocity:?}");
    }

    #[test]
    fn enemies_face_the_player() {
        let (mut world, mut physics) = arena();
        let right = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );
        let left = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(-300.0, 0.0),
        );

        face_player(&mut world);

        let flipped = |entity| world.get::<&Animation>(entity).map(|a| a.flipped());
        assert_eq!(flipped(right).ok(), Some(true));
        assert_eq!(flipped(left).ok(), Some(false));
    }

    #[test]
    fn a_dashing_ghost_leaves_fading_afterimages_like_the_player_and_a_calm_one_does_not() {
        let (mut world, mut physics) = arena();
        let ghost = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Ghost,
            Vec2::new(300.0, 0.0),
        );
        let images = |world: &World| {
            world
                .query::<&crate::components::Afterimage>()
                .iter()
                .count()
        };

        for _ in 0..30 {
            dash(&mut world, 1.0 / 60.0);
        }
        let during = images(&world);
        assert!(during >= 8, "{during}");
        let copy = world
            .query::<(&crate::components::Afterimage, &Position, &Sprite)>()
            .iter()
            .next()
            .map(|(_, position, _)| position.0);
        assert_eq!(copy, world.get::<&Position>(ghost).map(|p| p.0).ok());

        for _ in 0..30 {
            dash(&mut world, 1.0 / 60.0);
        }
        let after_burst = images(&world);
        for _ in 0..60 {
            dash(&mut world, 1.0 / 60.0);
        }
        assert_eq!(images(&world), after_burst);

        for _ in 0..40 {
            crate::systems::fade_afterimages(&mut world, 1.0 / 60.0);
        }
        assert_eq!(images(&world), 0);
    }

    #[test]
    fn other_enemies_do_not_leave_afterimages() {
        let (mut world, mut physics) = arena();
        prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );

        for _ in 0..60 {
            dash(&mut world, 1.0 / 60.0);
        }

        assert_eq!(
            world
                .query::<&crate::components::Afterimage>()
                .iter()
                .count(),
            0
        );
    }

    #[test]
    fn a_killed_bloated_spider_bursts_into_a_poison_cloud_exactly_once() {
        let (mut world, mut physics) = arena();
        let spider = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::BloatedSpider,
            Vec2::new(400.0, 400.0),
        );
        resolve_deaths(&mut world, &mut physics, &Textures::default());
        assert_eq!(
            world
                .query::<&crate::components::PoisonPool>()
                .iter()
                .count(),
            0
        );

        if let Ok(mut health) = world.get::<&mut crate::components::Health>(spider) {
            health.current = 0;
        }
        resolve_deaths(&mut world, &mut physics, &Textures::default());
        resolve_deaths(&mut world, &mut physics, &Textures::default());

        assert_eq!(
            world
                .query::<&crate::components::PoisonPool>()
                .iter()
                .count(),
            1
        );
    }

    fn kill(world: &mut World, physics: &mut Physics, entity: Entity) {
        if let Ok(mut health) = world.get::<&mut Health>(entity) {
            health.current = 0;
        }
        resolve_deaths(world, physics, &Textures::default());
    }

    fn wait(world: &mut World, physics: &mut Physics, seconds: f32) {
        let steps = (seconds * 60.0).ceil() as usize;
        for _ in 0..steps {
            tick_spawning(world, physics, 1.0 / 60.0);
        }
    }

    #[test]
    fn a_saliva_slime_melts_into_a_puddle_and_re_forms_with_full_health() {
        let (mut world, mut physics) = arena();
        let slime = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::SalivaSlime,
            Vec2::new(300.0, 0.0),
        );

        kill(&mut world, &mut physics, slime);

        assert!(world.get::<&Puddle>(slime).is_ok());
        assert!(world.get::<&Expiring>(slime).is_err());
        assert!(world.get::<&Body>(slime).is_err());
        wait(&mut world, &mut physics, REVIVE_SECONDS - 0.4);
        assert!(world.get::<&Puddle>(slime).is_ok());
        assert_eq!(
            world.get::<&Animation>(slime).map(|a| a.row()).ok(),
            Some(3),
            "the puddle pulls itself together before it stands up"
        );
        wait(&mut world, &mut physics, 0.5);
        assert!(world.get::<&Puddle>(slime).is_err());
        assert!(world.get::<&Body>(slime).is_ok());
        let health = world.get::<&Health>(slime).map(|h| (h.current, h.max)).ok();
        assert!(health.is_some_and(|(current, max)| current == max && max > 0));
    }

    #[test]
    fn the_second_death_of_a_saliva_slime_is_for_good() {
        let (mut world, mut physics) = arena();
        let slime = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::SalivaSlime,
            Vec2::new(300.0, 0.0),
        );
        kill(&mut world, &mut physics, slime);
        wait(&mut world, &mut physics, REVIVE_SECONDS + 0.1);

        kill(&mut world, &mut physics, slime);

        assert!(world.get::<&Expiring>(slime).is_ok());
        assert!(world.get::<&Puddle>(slime).is_err());
    }

    #[test]
    fn a_rat_does_not_revive() {
        let (mut world, mut physics) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            Vec2::new(300.0, 0.0),
        );
        kill(&mut world, &mut physics, rat);
        assert!(world.get::<&Expiring>(rat).is_ok());
        assert!(world.get::<&Puddle>(rat).is_err());
    }

    #[test]
    fn a_fallen_boss_takes_the_puddles_of_its_slimes_with_it() {
        let (mut world, mut physics) = arena();
        let slime = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::SalivaSlime,
            Vec2::new(300.0, 0.0),
        );
        let _ = world.insert_one(slime, crate::components::Minion);
        kill(&mut world, &mut physics, slime);
        assert!(world.get::<&Puddle>(slime).is_ok());
        let boss = world.spawn((
            crate::components::Boss,
            Health {
                current: 0,
                max: 10,
            },
        ));

        resolve_deaths(&mut world, &mut physics, &Textures::default());

        assert!(world.get::<&Expiring>(slime).is_ok());
        let _ = world.despawn(boss);
    }
}
