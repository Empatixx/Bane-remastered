use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::audio::AudioCommand;
use crate::components::{
    Animation, Color, ContactDamage, Despawn, Dodge, Expiring, Frozen, Health, Layer, LeapStage,
    Leaper, Player, Position, Rooted, Size, Spawning, Sprite, Velocity,
};
use crate::enemies;
use crate::systems::damage::hurt_player;

const MARK_SIZE: Vec2 = Vec2::new(224.0, 112.0);

struct Event {
    entity: Entity,
    position: Vec2,
    step: Step,
}

enum Step {
    Crouch(Vec2),
    Launch,
    Land,
    Recover,
}

fn player_at(world: &World) -> Option<(Entity, Vec2)> {
    world
        .query::<(Entity, &Position, &Health)>()
        .with::<&Player>()
        .iter()
        .filter(|(_, _, health)| health.current > 0)
        .map(|(entity, position, _)| (entity, position.0))
        .next()
}

pub(crate) fn tick_leapers(
    world: &mut World,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    dt: f32,
) {
    let target = player_at(world);
    let mut events = Vec::new();
    for (entity, position, leaper, velocity) in world
        .query_mut::<(Entity, &Position, &mut Leaper, &mut Velocity)>()
        .without::<&Spawning>()
        .without::<&Expiring>()
        .without::<&Frozen>()
    {
        leaper.timer -= dt;
        match leaper.stage {
            LeapStage::Stalk => {
                if leaper.timer <= 0.0 {
                    if let Some((_, at)) = target {
                        let distance = position.0.distance(at);
                        if (enemies::LEAP_MIN_DISTANCE..=enemies::LEAP_MAX_DISTANCE)
                            .contains(&distance)
                        {
                            events.push(Event {
                                entity,
                                position: position.0,
                                step: Step::Crouch(at),
                            });
                        }
                    }
                }
            }
            LeapStage::Crouch => {
                velocity.0 = Vec2::ZERO;
                if leaper.timer <= 0.0 {
                    events.push(Event {
                        entity,
                        position: position.0,
                        step: Step::Launch,
                    });
                }
            }
            LeapStage::Airborne => {
                let remaining = leaper.timer.max(dt);
                velocity.0 = (leaper.target - position.0) / remaining;
                if leaper.timer <= 0.0 {
                    events.push(Event {
                        entity,
                        position: position.0,
                        step: Step::Land,
                    });
                }
            }
            LeapStage::Rest => {
                velocity.0 = Vec2::ZERO;
                if leaper.timer <= 0.0 {
                    events.push(Event {
                        entity,
                        position: position.0,
                        step: Step::Recover,
                    });
                }
            }
        }
    }
    for event in events {
        apply(world, textures, sounds, target, event);
    }
}

fn play(world: &mut World, entity: Entity, clip: crate::components::Clip) {
    if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
        let facing = animation.flipped();
        animation.play(crate::components::Clip {
            flipped: facing,
            ..clip
        });
    }
}

fn apply(
    world: &mut World,
    textures: &Textures,
    sounds: &mut Vec<AudioCommand>,
    target: Option<(Entity, Vec2)>,
    event: Event,
) {
    let entity = event.entity;
    match event.step {
        Step::Crouch(at) => {
            let _ = world.insert_one(entity, Rooted);
            if let Ok(mut leaper) = world.get::<&mut Leaper>(entity) {
                leaper.stage = LeapStage::Crouch;
                leaper.timer = enemies::LEAP_CROUCH_SECONDS;
                leaper.target = at;
            }
            play(world, entity, enemies::crouch_clip());
            world.spawn((
                Despawn {
                    remaining: enemies::LEAP_CROUCH_SECONDS + enemies::LEAP_AIR_SECONDS + 0.3,
                },
                Position(at),
                Size(MARK_SIZE),
                Color(Vec4::ONE),
                Sprite::whole(textures.get(tex::LEAP_MARK)),
                Layer(-1),
            ));
        }
        Step::Launch => {
            let _ = world.remove_one::<ContactDamage>(entity);
            if let Ok(mut leaper) = world.get::<&mut Leaper>(entity) {
                leaper.stage = LeapStage::Airborne;
                leaper.timer = enemies::LEAP_AIR_SECONDS;
            }
            play(world, entity, enemies::air_clip());
            if let Ok(mut layer) = world.get::<&mut Layer>(entity) {
                layer.0 = 3;
            }
        }
        Step::Land => {
            let (damage, landed) = world
                .get::<&Leaper>(entity)
                .map(|leaper| (leaper.damage, leaper.target))
                .unwrap_or((1, event.position));
            let _ = world.insert_one(entity, ContactDamage(damage));
            if let Ok(mut leaper) = world.get::<&mut Leaper>(entity) {
                leaper.stage = LeapStage::Rest;
                leaper.timer = enemies::LEAP_REST_SECONDS;
            }
            if let Ok(mut velocity) = world.get::<&mut Velocity>(entity) {
                velocity.0 = Vec2::ZERO;
            }
            if let Ok(mut layer) = world.get::<&mut Layer>(entity) {
                layer.0 = 1;
            }
            play(world, entity, enemies::landing_clip());
            if let Some((player, at)) = target {
                let landing_at = if event.position.distance(landed) < enemies::LEAP_LANDING_RADIUS {
                    event.position
                } else {
                    landed
                };
                let flying = world.get::<&Dodge>(player).is_ok();
                if !flying && at.distance(landing_at) < enemies::LEAP_LANDING_RADIUS {
                    hurt_player(world, player, damage, sounds);
                }
            }
        }
        Step::Recover => {
            let idle = world.get::<&Leaper>(entity).map(|leaper| leaper.idle);
            let _ = world.remove_one::<Rooted>(entity);
            if let Ok(mut leaper) = world.get::<&mut Leaper>(entity) {
                leaper.stage = LeapStage::Stalk;
                leaper.timer = enemies::LEAP_COOLDOWN;
            }
            if let Ok(idle) = idle {
                play(world, entity, idle);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::Armor;
    use crate::enemies::EnemyKind;
    use crate::physics::Physics;
    use crate::prefabs;

    use super::*;

    const STEP: f32 = 1.0 / 60.0;

    fn arena(distance: f32) -> (World, Entity, Entity) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let player = prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(600.0, 600.0),
            &Textures::default(),
        );
        if let Ok(mut armor) = world.get::<&mut Armor>(player) {
            armor.current = 0;
        }
        let spider = prefabs::ready_enemy(
            &mut world,
            &mut physics,
            EnemyKind::StiltSpider,
            Vec2::new(600.0 - distance, 600.0),
        );
        (world, player, spider)
    }

    fn run(world: &mut World, seconds: f32) {
        let mut sounds = Vec::new();
        for _ in 0..(seconds / STEP) as usize {
            tick_leapers(world, &Textures::default(), &mut sounds, STEP);
        }
    }

    fn stage(world: &World, spider: Entity) -> LeapStage {
        world
            .get::<&Leaper>(spider)
            .map(|l| l.stage)
            .expect("a leaper")
    }

    #[test]
    fn it_crouches_and_marks_the_landing_when_the_player_is_in_range() {
        let (mut world, _, spider) = arena(300.0);
        run(&mut world, enemies::LEAP_FIRST_COOLDOWN + 0.1);
        assert_eq!(stage(&world, spider), LeapStage::Crouch);
        assert!(world.get::<&Rooted>(spider).is_ok());
        assert_eq!(world.query::<&Despawn>().iter().count(), 1);
    }

    #[test]
    fn it_keeps_stalking_when_the_player_is_too_far_or_too_close() {
        for distance in [700.0, 60.0] {
            let (mut world, _, spider) = arena(distance);
            run(&mut world, 3.0);
            assert_eq!(stage(&world, spider), LeapStage::Stalk, "{distance}");
        }
    }

    #[test]
    fn it_cannot_hurt_by_touch_while_it_is_in_the_air_and_does_again_after_landing() {
        let (mut world, _, spider) = arena(300.0);
        run(
            &mut world,
            enemies::LEAP_FIRST_COOLDOWN + enemies::LEAP_CROUCH_SECONDS + 0.15,
        );
        assert_eq!(stage(&world, spider), LeapStage::Airborne);
        assert!(world.get::<&ContactDamage>(spider).is_err());

        run(&mut world, enemies::LEAP_AIR_SECONDS + 0.1);
        assert_eq!(stage(&world, spider), LeapStage::Rest);
        assert!(world.get::<&ContactDamage>(spider).is_ok());
    }

    #[test]
    fn landing_on_the_player_hurts_but_a_player_who_moved_away_is_safe() {
        let (mut world, player, _) = arena(300.0);
        run(
            &mut world,
            enemies::LEAP_FIRST_COOLDOWN
                + enemies::LEAP_CROUCH_SECONDS
                + enemies::LEAP_AIR_SECONDS
                + 0.2,
        );
        let health = world.get::<&Health>(player).map(|h| h.current).unwrap_or(0);
        assert!(health < 10, "{health}");

        let (mut safe_world, safe_player, _) = arena(300.0);
        run(
            &mut safe_world,
            enemies::LEAP_FIRST_COOLDOWN + enemies::LEAP_CROUCH_SECONDS + 0.05,
        );
        if let Ok(mut position) = safe_world.get::<&mut Position>(safe_player) {
            position.0 += Vec2::new(0.0, 400.0);
        }
        run(&mut safe_world, enemies::LEAP_AIR_SECONDS + 0.3);
        let safe = safe_world
            .get::<&Health>(safe_player)
            .map(|h| h.current)
            .unwrap_or(0);
        assert_eq!(safe, 10);
    }

    #[test]
    fn it_recovers_and_stalks_again() {
        let (mut world, _, spider) = arena(300.0);
        run(
            &mut world,
            enemies::LEAP_FIRST_COOLDOWN
                + enemies::LEAP_CROUCH_SECONDS
                + enemies::LEAP_AIR_SECONDS
                + enemies::LEAP_REST_SECONDS
                + 0.2,
        );
        assert_eq!(stage(&world, spider), LeapStage::Stalk);
        assert!(world.get::<&Rooted>(spider).is_err());
    }
}
