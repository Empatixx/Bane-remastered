use glam::Vec2;
use hecs::{Entity, World};

use crate::components::{
    Animation, Clip, Enemy, Expiring, Health, Path, Pet, Pickup, Player, Position, Shop, Size,
    Spawning, Velocity,
};
use crate::geometry;
use crate::navigation::{CELL, Navigation, corridor};
use crate::physics::Physics;
use crate::prefabs::PET_SIZE;
use crate::sheets::pet::{BOTTOM, IDLE as PET_IDLE, LEFT, RIGHT, TOP};
use crate::systems::damage::{Damage, DamageSource};

const SPEED: f32 = 520.0;
const AREA: f32 = 500.0;
const TELEPORT_DISTANCE: f32 = 550.0;
const STOP_DISTANCE: f32 = 100.0;
const FOLLOW_SPEED: f32 = 430.0;
const FOLLOW_RAMP: f32 = 120.0;
const IDLE_SPEED: f32 = 40.0;
const TURN_RATIO: f32 = 1.4;
const BITE_DAMAGE: i32 = 1;
/// One bite a second at most.
const BITE_COOLDOWN: f32 = 1.0;

fn owner(world: &World) -> Option<Entity> {
    world.query::<Entity>().with::<&Player>().iter().next()
}

fn placement(world: &World, entity: Entity) -> Option<(Vec2, Vec2)> {
    let position = world.get::<&Position>(entity).ok()?.0;
    let size = world.get::<&Size>(entity).ok()?.0;
    Some((position, size))
}

fn alive_enemy(world: &World, entity: Entity) -> bool {
    world.satisfies::<&Enemy>(entity)
        && !world.satisfies::<&Expiring>(entity)
        && world
            .get::<&Health>(entity)
            .is_ok_and(|health| health.current > 0)
}

fn lootable(world: &World, entity: Entity, center: Vec2) -> bool {
    let Ok(pickup) = world.get::<&Pickup>(entity).map(|pickup| pickup.0) else {
        return false;
    };
    let Some((position, _)) = placement(world, entity) else {
        return false;
    };
    !world.satisfies::<&Expiring>(entity)
        && position.distance(center) <= AREA
        && super::items::can_use(world, pickup)
}

fn in_area(center: Vec2, point: Vec2) -> bool {
    (point - center).abs().max_element() <= AREA / 2.0
}

fn nearest_loot(world: &World, center: Vec2, skip: &[Entity]) -> Option<Entity> {
    world
        .query::<(Entity, &Pickup, &Position)>()
        .without::<&Expiring>()
        .without::<&Shop>()
        .iter()
        .filter(|(entity, pickup, position)| {
            !skip.contains(entity)
                && in_area(center, position.0)
                && super::items::can_use(world, pickup.0)
        })
        .min_by(|a, b| {
            a.2.0
                .distance_squared(center)
                .total_cmp(&b.2.0.distance_squared(center))
        })
        .map(|(entity, _, _)| entity)
}

fn nearest_enemy(world: &World, center: Vec2, skip: &[Entity]) -> Option<Entity> {
    world
        .query::<(Entity, &Position, &Health)>()
        .with::<&Enemy>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .filter(|(entity, position, health)| {
            !skip.contains(entity) && health.current > 0 && in_area(center, position.0)
        })
        .min_by(|a, b| {
            a.1.0
                .distance_squared(center)
                .total_cmp(&b.1.0.distance_squared(center))
        })
        .map(|(entity, _, _)| entity)
}

pub(crate) fn pick_pet_targets(world: &mut World) {
    let pets: Vec<(Entity, Vec2, Pet)> = world
        .query::<(Entity, &Position, &Pet)>()
        .iter()
        .map(|(entity, position, pet)| (entity, position.0, pet.clone()))
        .collect();
    for (entity, center, pet) in pets {
        let loot = pet
            .loot
            .filter(|&item| lootable(world, item, center))
            .or_else(|| nearest_loot(world, center, &pet.unreachable));
        let enemy = pet
            .enemy
            .filter(|&enemy| alive_enemy(world, enemy))
            .or_else(|| nearest_enemy(world, center, &pet.unreachable));
        if let Ok(mut pet) = world.get::<&mut Pet>(entity) {
            pet.loot = loot;
            pet.enemy = enemy;
        }
    }
}

fn goal_of(pet: &Pet, owner: Entity) -> Entity {
    pet.loot.or(pet.enemy).unwrap_or(owner)
}

fn blocked_by_enemy(world: &World, center: Vec2) -> bool {
    world
        .query::<(&Position, &Size)>()
        .with::<&Enemy>()
        .without::<&Expiring>()
        .without::<&Spawning>()
        .iter()
        .any(|(position, size)| geometry::overlaps(center, PET_SIZE, position.0, size.0))
}

fn teleport_to_owner(world: &mut World, physics: &mut Physics, pet: Entity, destination: Vec2) {
    if blocked_by_enemy(world, destination) {
        tracing::warn!("the pet cannot teleport, an enemy stands next to the owner");
        return;
    }
    if let Ok((position, body, path, state)) =
        world.query_one_mut::<(&mut Position, &crate::components::Body, &mut Path, &mut Pet)>(pet)
    {
        position.0 = destination;
        physics.teleport(*body, destination);
        path.cells.clear();
        path.planned_for = None;
        state.stranded = false;
        state.heading = None;
        tracing::info!(
            x = destination.x,
            y = destination.y,
            "the pet teleports to its owner"
        );
    }
}

pub(crate) fn plan_pet_paths(world: &mut World, physics: &mut Physics, navigation: &Navigation) {
    let Some(owner) = owner(world) else {
        return;
    };
    let pets: Vec<Entity> = world.query::<Entity>().with::<&Pet>().iter().collect();
    for entity in pets {
        let Some(goal) = world
            .get::<&Pet>(entity)
            .ok()
            .map(|pet| goal_of(&pet, owner))
        else {
            continue;
        };
        let (Some((target, target_size)), Some((position, size))) =
            (placement(world, goal), placement(world, entity))
        else {
            continue;
        };
        if let Ok((path, pet)) = world.query_one_mut::<(&mut Path, &mut Pet)>(entity) {
            if pet.heading != Some(goal) {
                path.cells.clear();
                path.planned_for = None;
                pet.heading = Some(goal);
                pet.stranded = false;
            }
            let moved = path
                .planned_for
                .is_none_or(|planned| (target - planned).abs().max_element() > CELL);
            if moved {
                let (min, max) = corridor(position, target);
                let solids = physics.solids_within(min, max);
                path.cells = navigation
                    .find_path_around(position, size, target, target_size, &solids)
                    .into();
                path.planned_for = Some(target);
                pet.stranded = path.cells.is_empty();
            }
            if pet.stranded && goal != owner {
                pet.unreachable.push(goal);
                if pet.loot == Some(goal) {
                    pet.loot = None;
                } else {
                    pet.enemy = None;
                }
                pet.heading = None;
                pet.stranded = false;
            }
        }
        let far = position.distance(target) > TELEPORT_DISTANCE;
        let lost = world.get::<&Pet>(entity).is_ok_and(|pet| pet.stranded);
        if goal == owner && (far || lost) {
            teleport_to_owner(world, physics, entity, target);
        }
    }
}

fn follow_speed(distance: f32) -> f32 {
    let beyond = distance - STOP_DISTANCE;
    if beyond <= 0.0 {
        return 0.0;
    }
    if beyond <= FOLLOW_RAMP {
        return FOLLOW_SPEED * beyond / FOLLOW_RAMP;
    }
    let catch_up = ((beyond - FOLLOW_RAMP) / (TELEPORT_DISTANCE - STOP_DISTANCE - FOLLOW_RAMP))
        .clamp(0.0, 1.0);
    FOLLOW_SPEED + (SPEED - FOLLOW_SPEED) * catch_up
}

fn direction(position: Vec2, path: &mut Path, target: Vec2) -> Vec2 {
    while path
        .cells
        .front()
        .is_some_and(|next| next.distance(position) <= CELL)
    {
        path.cells.pop_front();
    }
    let waypoint = path.cells.front().copied().unwrap_or(target);
    (waypoint - position).normalize_or_zero()
}

fn clip_for(direction: Vec2, current: Clip) -> Clip {
    let sideways = current == RIGHT || current == LEFT;
    let horizontal = if sideways {
        direction.x.abs() * TURN_RATIO > direction.y.abs()
    } else {
        direction.x.abs() > direction.y.abs() * TURN_RATIO
    };
    if horizontal {
        if direction.x > 0.0 { RIGHT } else { LEFT }
    } else if direction.y > 0.0 {
        BOTTOM
    } else {
        TOP
    }
}

pub(crate) fn steer_pets(world: &mut World) {
    let Some(owner) = owner(world) else {
        return;
    };
    let pets: Vec<(Entity, Entity)> = world
        .query::<(Entity, &Pet)>()
        .iter()
        .map(|(entity, pet)| (entity, goal_of(pet, owner)))
        .collect();
    for (entity, goal) in pets {
        let target = placement(world, goal).map(|(position, _)| position);
        let Ok((position, velocity, path, animation)) =
            world.query_one_mut::<(&Position, &mut Velocity, &mut Path, &mut Animation)>(entity)
        else {
            continue;
        };
        let (direction, speed) = target.map_or((Vec2::ZERO, 0.0), |target| {
            let speed = if goal == owner {
                follow_speed(position.0.distance(target))
            } else {
                SPEED
            };
            (direction(position.0, path, target), speed)
        });
        velocity.0 = direction * speed;
        if speed < IDLE_SPEED {
            animation.switch_to(PET_IDLE);
        } else {
            animation.switch_to(clip_for(direction, animation.clip()));
        }
    }
}

pub(crate) fn pet_bites(world: &mut World, dt: f32) {
    let pets: Vec<(Entity, Vec2, Vec2)> = world
        .query_mut::<(Entity, &Position, &Size, &mut Pet)>()
        .into_iter()
        .map(|(entity, position, size, pet)| {
            pet.bite_cooldown = (pet.bite_cooldown - dt).max(0.0);
            (entity, position.0, size.0)
        })
        .collect();
    for (pet, center, size) in pets {
        if world
            .get::<&Pet>(pet)
            .is_ok_and(|pet| pet.bite_cooldown > 0.0)
        {
            continue;
        }
        let victim = world
            .query::<(Entity, &Position, &Size)>()
            .with::<(&Enemy, &Health)>()
            .without::<&Expiring>()
            .without::<&Spawning>()
            .iter()
            .find(|(_, position, enemy_size)| {
                geometry::overlaps(center, size, position.0, enemy_size.0)
            })
            .map(|(entity, _, _)| entity);
        let Some(victim) = victim else {
            continue;
        };
        super::damage::deal(
            world,
            Damage {
                target: victim,
                amount: BITE_DAMAGE,
                source: DamageSource::Bite,
            },
        );
        if let Ok(mut pet) = world.get::<&mut Pet>(pet) {
            pet.bite_cooldown = BITE_COOLDOWN;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assets::Textures;
    use crate::components::HitFlash;
    use crate::enemies::EnemyKind;
    use crate::items::{ItemKind, PotionSize, Vital};
    use crate::prefabs;

    use super::*;

    const OWNER_AT: Vec2 = Vec2::new(1000.0, 1000.0);

    fn arena() -> (World, Physics, Entity) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let textures = Textures::default();
        prefabs::player(&mut world, &mut physics, OWNER_AT, &textures);
        let pet = prefabs::pet(
            &mut world,
            &mut physics,
            OWNER_AT + Vec2::new(150.0, 0.0),
            &textures,
        );
        (world, physics, pet)
    }

    fn drop_item(world: &mut World, kind: ItemKind, at: Vec2) {
        prefabs::item(world, kind, at, true, Vec2::ZERO, &Textures::default());
    }

    fn think(world: &mut World, physics: &mut Physics, navigation: &Navigation) {
        pick_pet_targets(world);
        plan_pet_paths(world, physics, navigation);
        steer_pets(world);
    }

    fn velocity(world: &World, pet: Entity) -> Vec2 {
        world.get::<&Velocity>(pet).map_or(Vec2::NAN, |v| v.0)
    }

    #[test]
    fn the_pet_runs_for_a_coin_nearby() {
        let (mut world, mut physics, pet) = arena();
        drop_item(
            &mut world,
            ItemKind::Coin,
            OWNER_AT + Vec2::new(150.0, 200.0),
        );

        think(&mut world, &mut physics, &Navigation::default());

        let velocity = velocity(&world, pet);
        assert!(
            velocity.y > 0.0 && velocity.length() > 400.0,
            "{velocity:?}"
        );
        assert!(world.get::<&Pet>(pet).is_ok_and(|pet| pet.loot.is_some()));
    }

    #[test]
    fn the_pet_ignores_a_potion_the_owner_cannot_use_and_shop_goods() {
        let (mut world, mut physics, pet) = arena();
        let potion = ItemKind::Potion(Vital::Health, PotionSize::Small);
        drop_item(&mut world, potion, OWNER_AT + Vec2::new(150.0, 200.0));
        let wares = world.spawn((
            Pickup(ItemKind::Coin),
            Shop {
                price: 3,
                currency: crate::components::Currency::Coins,
            },
            Position(OWNER_AT + Vec2::new(200.0, 0.0)),
            Size(Vec2::splat(24.0)),
        ));

        pick_pet_targets(&mut world);

        assert!(world.get::<&Pet>(pet).is_ok_and(|pet| pet.loot.is_none()));
        assert!(world.contains(wares));
        let _ = &mut physics;
    }

    #[test]
    fn a_coin_far_away_is_left_alone() {
        let (mut world, _, pet) = arena();
        drop_item(
            &mut world,
            ItemKind::Coin,
            OWNER_AT + Vec2::new(150.0, 400.0),
        );

        pick_pet_targets(&mut world);

        assert!(world.get::<&Pet>(pet).is_ok_and(|pet| pet.loot.is_none()));
    }

    #[test]
    fn loot_comes_before_an_enemy_and_an_enemy_before_the_owner() {
        let (mut world, mut physics, pet) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            OWNER_AT + Vec2::new(150.0, -200.0),
        );
        think(&mut world, &mut physics, &Navigation::default());
        assert!(velocity(&world, pet).y < 0.0);
        assert!(
            world
                .get::<&Pet>(pet)
                .is_ok_and(|pet| pet.enemy == Some(rat))
        );

        drop_item(
            &mut world,
            ItemKind::Coin,
            OWNER_AT + Vec2::new(150.0, 200.0),
        );
        think(&mut world, &mut physics, &Navigation::default());
        assert!(velocity(&world, pet).y > 0.0);
    }

    #[test]
    fn a_dead_enemy_is_forgotten() {
        let (mut world, mut physics, pet) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            OWNER_AT + Vec2::new(150.0, -200.0),
        );
        pick_pet_targets(&mut world);
        if let Ok(mut health) = world.get::<&mut Health>(rat) {
            health.current = 0;
        }

        pick_pet_targets(&mut world);

        assert!(world.get::<&Pet>(pet).is_ok_and(|pet| pet.enemy.is_none()));
    }

    #[test]
    fn the_pet_stops_next_to_its_owner() {
        let (mut world, mut physics, pet) = arena();
        if let Ok(mut position) = world.get::<&mut Position>(pet) {
            position.0 = OWNER_AT + Vec2::new(60.0, 0.0);
        }

        think(&mut world, &mut physics, &Navigation::default());

        assert_eq!(velocity(&world, pet), Vec2::ZERO);
    }

    #[test]
    fn a_pet_far_from_its_owner_teleports_next_to_it() {
        let (mut world, mut physics, pet) = arena();
        if let Ok(mut position) = world.get::<&mut Position>(pet) {
            position.0 = OWNER_AT + Vec2::new(900.0, 0.0);
        }

        plan_pet_paths(&mut world, &mut physics, &Navigation::default());

        let at = world.get::<&Position>(pet).map_or(Vec2::NAN, |p| p.0);
        assert_eq!(at, OWNER_AT);
    }

    #[test]
    fn the_pet_does_not_teleport_onto_an_enemy() {
        let (mut world, mut physics, pet) = arena();
        prefabs::ready_enemy(&mut world, &mut physics, EnemyKind::Rat, OWNER_AT);
        let far = OWNER_AT + Vec2::new(900.0, 0.0);
        if let Ok(mut position) = world.get::<&mut Position>(pet) {
            position.0 = far;
        }

        plan_pet_paths(&mut world, &mut physics, &Navigation::default());

        let at = world.get::<&Position>(pet).map_or(Vec2::NAN, |p| p.0);
        assert_eq!(at, far);
    }

    #[test]
    fn an_unreachable_coin_is_given_up_on() {
        let (mut world, mut physics, pet) = arena();
        let around = [
            (16, 17),
            (17, 17),
            (18, 17),
            (16, 16),
            (18, 16),
            (16, 15),
            (17, 15),
            (18, 15),
        ];
        let navigation = Navigation::new(&around);
        let coin_at = Vec2::new(17.5 * 64.0, 16.5 * 64.0);
        if let Ok(mut position) = world.get::<&mut Position>(pet) {
            position.0 = coin_at - Vec2::new(0.0, 200.0);
        }
        drop_item(&mut world, ItemKind::Coin, coin_at);

        think(&mut world, &mut physics, &navigation);

        let given_up = world
            .get::<&Pet>(pet)
            .is_ok_and(|pet| pet.loot.is_none() && pet.unreachable.len() == 1);
        assert!(given_up);
    }

    #[test]
    fn the_pet_bites_once_per_cooldown() {
        let (mut world, mut physics, pet) = arena();
        let rat = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::Rat,
            OWNER_AT + Vec2::new(150.0, 0.0),
        );
        let before = world.get::<&Health>(rat).map_or(0, |h| h.current);

        pet_bites(&mut world, 0.0);
        pet_bites(&mut world, 0.5);
        pet_bites(&mut world, 0.4);
        assert_eq!(
            world.get::<&Health>(rat).map_or(0, |h| h.current),
            before - 1
        );

        pet_bites(&mut world, 0.15);
        assert_eq!(
            world.get::<&Health>(rat).map_or(0, |h| h.current),
            before - 2
        );
        crate::systems::hit_phase(&mut world);
        assert!(world.get::<&HitFlash>(rat).is_ok());
        let _ = pet;
    }

    #[test]
    fn the_pet_picks_up_a_coin_for_its_owner() {
        let (mut world, _, pet) = arena();
        let at = world.get::<&Position>(pet).map_or(Vec2::NAN, |p| p.0);
        drop_item(&mut world, ItemKind::Coin, at);
        let mut sounds = Vec::new();

        super::super::items::collect_items(&mut world, &mut sounds);

        let coins = world
            .query::<&crate::components::Wallet>()
            .with::<&Player>()
            .iter()
            .next()
            .map_or(-1, |wallet| wallet.coins);
        assert_eq!(coins, 1);
        assert_eq!(world.query::<&Pickup>().iter().count(), 0);
    }

    #[test]
    fn the_pet_follows_slower_than_its_full_speed_near_the_owner_and_never_outruns_a_walking_player()
     {
        assert_eq!(follow_speed(STOP_DISTANCE - 1.0), 0.0);
        assert_eq!(follow_speed(STOP_DISTANCE), 0.0);
        let mut last = 0.0;
        for step in 0..=60 {
            let speed = follow_speed(STOP_DISTANCE + step as f32 * 8.0);
            assert!(speed >= last, "{step}: {speed} < {last}");
            last = speed;
        }
        assert!(follow_speed(STOP_DISTANCE + FOLLOW_RAMP) <= FOLLOW_SPEED);
        assert!(follow_speed(STOP_DISTANCE + FOLLOW_RAMP / 4.0) < 150.0);
        assert!((follow_speed(TELEPORT_DISTANCE) - SPEED).abs() < 0.01);
    }

    #[test]
    fn the_walk_animation_does_not_flip_while_the_direction_wobbles_around_a_diagonal() {
        let mut clip = RIGHT;
        for wobble in [0.9, 1.1, 0.95, 1.05, 1.0, 0.92] {
            let direction = Vec2::new(wobble, 1.0).normalize();
            clip = clip_for(direction, clip);
            assert_eq!(clip, RIGHT, "{wobble}");
        }
        let mut clip = BOTTOM;
        for wobble in [0.9, 1.1, 0.95, 1.05, 1.0, 0.92] {
            let direction = Vec2::new(wobble, 1.0).normalize();
            clip = clip_for(direction, clip);
            assert_eq!(clip, BOTTOM, "{wobble}");
        }
    }

    #[test]
    fn a_clear_turn_changes_the_walk_animation() {
        assert_eq!(clip_for(Vec2::new(1.0, 0.1).normalize(), BOTTOM), RIGHT);
        assert_eq!(clip_for(Vec2::new(-1.0, 0.1).normalize(), TOP), LEFT);
        assert_eq!(clip_for(Vec2::new(0.1, 1.0).normalize(), RIGHT), BOTTOM);
        assert_eq!(clip_for(Vec2::new(0.1, -1.0).normalize(), LEFT), TOP);
    }
}
