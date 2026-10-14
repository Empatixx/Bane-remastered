use std::f32::consts::TAU;

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::components::{
    Animation, Clip, Color, Enemy, EnemyBullet, Expiring, Layer, MidasTouch, NoDrop, Player,
    Position, Size, Sprite,
};
use crate::items::ItemKind;
use crate::physics::Physics;
use crate::prefabs;

const SPARKLE_FRAMES: usize = 6;
const SPARKLE_FRAME_TIME: f32 = 0.07;
const SPARKLE_SIZE: f32 = 40.0;
const BONUS_COINS: usize = 2;
const SCATTER_SPEED: f32 = 90.0;

const SPARKLE: Clip = Clip {
    row: 0,
    frames: SPARKLE_FRAMES,
    frame_time: SPARKLE_FRAME_TIME,
    looping: false,
    flipped: false,
};

fn spawn_sparkle(world: &mut World, at: Vec2, textures: &Textures) {
    world.spawn((
        Position(at),
        Size(Vec2::splat(SPARKLE_SIZE)),
        Color(Vec4::ONE),
        Sprite::cell(
            textures.get(tex::ARTEFACTS_MIDAS_SPARKLE),
            0,
            0,
            SPARKLE_FRAMES,
            1,
        ),
        Animation::new(SPARKLE_FRAMES, 1, SPARKLE),
        Layer(2),
        Expiring,
    ));
}

pub(crate) fn convert_bullets(world: &mut World, physics: &mut Physics, textures: &Textures) {
    let Some(player) = world
        .query::<Entity>()
        .with::<(&Player, &crate::components::ConvertBullets)>()
        .iter()
        .next()
    else {
        return;
    };
    let _ = world.remove_one::<crate::components::ConvertBullets>(player);
    let bullets: Vec<(Entity, Vec2)> = world
        .query::<(Entity, &Position)>()
        .with::<&EnemyBullet>()
        .without::<&Expiring>()
        .iter()
        .map(|(entity, position)| (entity, position.0))
        .collect();
    for (bullet, at) in bullets {
        physics.detach(world, bullet);
        let _ = world.despawn(bullet);
        prefabs::item(world, ItemKind::Coin, at, true, Vec2::ZERO, textures);
        spawn_sparkle(world, at, textures);
    }
}

pub(crate) fn midas_kill_coins(world: &mut World, textures: &Textures) {
    if world.query::<&MidasTouch>().iter().next().is_none() {
        return;
    }
    let dead: Vec<Vec2> = world
        .query::<(&Position, &Animation)>()
        .with::<(&Enemy, &Expiring)>()
        .without::<&NoDrop>()
        .iter()
        .filter(|(_, animation)| animation.finished())
        .map(|(position, _)| position.0)
        .collect();
    for at in dead {
        let phase = (at.x * 0.37 + at.y * 0.91).sin().abs() * TAU;
        for index in 0..BONUS_COINS {
            let angle = phase + TAU * index as f32 / BONUS_COINS as f32;
            let velocity = Vec2::from_angle(angle) * SCATTER_SPEED;
            prefabs::item(world, ItemKind::Coin, at, true, velocity, textures);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{ConvertBullets, Pickup};
    use crate::enemies::{EnemyKind, ProjectileKind};
    use crate::systems::tick_artefact_effects;

    use super::*;

    fn setup() -> (World, Physics, Entity) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let player = prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(300.0, 200.0),
            &Textures::default(),
        );
        (world, physics, player)
    }

    fn coins(world: &World) -> usize {
        world
            .query::<&Pickup>()
            .iter()
            .filter(|pickup| pickup.0 == ItemKind::Coin)
            .count()
    }

    fn kill_a_rat(world: &mut World, physics: &mut Physics) {
        let rat = prefabs::ready_enemy(world, physics, EnemyKind::Rat, Vec2::new(900.0, 900.0));
        let _ = world.insert_one(rat, Expiring);
        if let Ok(mut animation) = world.get::<&mut Animation>(rat) {
            animation.play(EnemyKind::Rat.spec().death_clip());
            animation.advance(5.0);
        }
    }

    #[test]
    fn every_enemy_bullet_turns_into_a_coin_and_leaves_physics() {
        let (mut world, mut physics, player) = setup();
        let textures = Textures::default();
        let baseline = physics.body_count();
        for index in 0..3 {
            prefabs::enemy_bullet(
                &mut world,
                &mut physics,
                ProjectileKind::Red,
                Vec2::new(500.0 + 40.0 * index as f32, 300.0),
                Vec2::X,
                &textures,
            );
        }
        assert_eq!(physics.body_count(), baseline + 3);
        let _ = world.insert_one(player, ConvertBullets);

        convert_bullets(&mut world, &mut physics, &textures);

        assert_eq!(world.query::<&EnemyBullet>().iter().count(), 0);
        assert_eq!(physics.body_count(), baseline);
        assert_eq!(coins(&world), 3);
        assert!(world.get::<&ConvertBullets>(player).is_err());
    }

    #[test]
    fn a_sparkle_plays_once_at_each_converted_bullet() {
        let (mut world, mut physics, player) = setup();
        let textures = Textures::default();
        prefabs::enemy_bullet(
            &mut world,
            &mut physics,
            ProjectileKind::Red,
            Vec2::new(500.0, 300.0),
            Vec2::X,
            &textures,
        );
        let _ = world.insert_one(player, ConvertBullets);
        convert_bullets(&mut world, &mut physics, &textures);
        assert_eq!(
            world
                .query::<&Expiring>()
                .with::<&Animation>()
                .iter()
                .count(),
            1
        );

        crate::systems::animate(&mut world, 1.0);
        crate::systems::remove_expired(&mut world);

        assert_eq!(
            world
                .query::<&Expiring>()
                .with::<&Animation>()
                .iter()
                .count(),
            0
        );
    }

    #[test]
    fn midas_touch_adds_two_coins_per_kill_and_nothing_after_it_expires() {
        let (mut world, mut physics, player) = setup();
        let textures = Textures::default();
        let _ = world.insert_one(player, MidasTouch { remaining: 0.5 });
        kill_a_rat(&mut world, &mut physics);

        midas_kill_coins(&mut world, &textures);
        assert_eq!(coins(&world), 2);

        tick_artefact_effects(&mut world, 1.0);
        midas_kill_coins(&mut world, &textures);
        assert_eq!(coins(&world), 2);
    }

    #[test]
    fn spent_enemy_shots_are_not_turned_into_coins() {
        let (mut world, mut physics, player) = setup();
        let textures = Textures::default();
        let spent = prefabs::enemy_bullet(
            &mut world,
            &mut physics,
            ProjectileKind::Red,
            Vec2::new(900.0, 900.0),
            Vec2::X,
            &textures,
        );
        let _ = world.insert_one(spent, Expiring);
        let _ = world.insert_one(player, ConvertBullets);

        convert_bullets(&mut world, &mut physics, &textures);

        assert_eq!(coins(&world), 0);
        assert!(world.contains(spent));
    }
}
