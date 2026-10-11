use glam::Vec2;
use hecs::{Entity, World};
use winit::keyboard::KeyCode;

use crate::audio::{AudioCommand, SoundEffect};
use crate::components::BULLET_IMPACT;
use crate::components::{
    Animation, Body, Bullet, Expiring, Health, Player, Position, Velocity, Weapon,
};
use crate::input::Input;
use crate::physics::{Owner, Physics};
use crate::prefabs;
use crate::render::TextureId;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ShotRoll {
    pub(crate) spread: f32,
    pub(crate) damage: f32,
}

impl ShotRoll {
    pub(crate) fn random() -> Self {
        Self {
            spread: rand::random_range(-1.0..=1.0),
            damage: rand::random_range(0.0..1.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ShotContext {
    pub(crate) camera: Vec2,
    pub(crate) bullet_texture: TextureId,
    pub(crate) dt: f32,
    pub(crate) roll: ShotRoll,
}

pub(crate) fn shoot(
    world: &mut World,
    physics: &mut Physics,
    input: &Input,
    context: ShotContext,
    sounds: &mut Vec<AudioCommand>,
) {
    let ShotContext {
        camera,
        bullet_texture,
        dt,
        roll,
    } = context;
    let target = input.mouse() + camera;
    let mut shots = Vec::new();
    for (position, weapon) in world
        .query_mut::<(&Position, &mut Weapon)>()
        .with::<&Player>()
    {
        weapon.advance(dt);
        if input.just_pressed(KeyCode::KeyR) {
            let was_reloading = weapon.reloading.is_some();
            weapon.start_reload();
            if !was_reloading && weapon.reloading.is_some() {
                sounds.push(AudioCommand::Effect(SoundEffect::PistolReload));
            }
        }
        if input.clicked()
            && weapon.reloading.is_none()
            && weapon.cooldown <= 0.0
            && weapon.clip == 0
        {
            sounds.push(AudioCommand::Effect(SoundEffect::EmptyGunShoot));
        }
        if !input.clicked() || !weapon.can_fire() {
            continue;
        }
        let to_target = target - position.0;
        if to_target == Vec2::ZERO {
            continue;
        }
        weapon.cooldown = weapon.fire_interval;
        weapon.clip -= 1;
        sounds.push(AudioCommand::Effect(SoundEffect::PistolShoot));
        let angle = to_target.to_angle() + roll.spread * weapon.inaccuracy;
        let range = weapon.max_damage - weapon.min_damage + 1;
        let damage = weapon.min_damage + ((range as f32 * roll.damage) as i32).min(range - 1);
        shots.push((
            position.0,
            Vec2::from_angle(angle) * weapon.bullet_speed,
            damage,
        ));
    }
    for (position, velocity, damage) in shots {
        prefabs::bullet(world, physics, position, velocity, damage, bullet_texture);
    }
}

pub(crate) fn bullet_hits(
    world: &mut World,
    physics: &mut Physics,
    sounds: &mut Vec<AudioCommand>,
) {
    let hits: Vec<(Entity, Body, i32, Vec<Owner>)> = world
        .query::<(Entity, &Body, &Bullet)>()
        .without::<&Expiring>()
        .iter()
        .map(|(entity, body, bullet)| (entity, *body, bullet.damage, physics.touching(*body)))
        .filter(|(_, _, _, touched)| !touched.is_empty())
        .collect();
    for (bullet, body, damage, touched) in hits {
        let victim = touched.iter().find_map(|owner| match owner {
            Owner::Entity(entity) => Some(*entity),
            Owner::Wall => None,
        });
        let hit_enemy = victim
            .and_then(|entity| world.get::<&mut Health>(entity).ok())
            .map(|mut health| health.current -= damage)
            .is_some();
        sounds.push(AudioCommand::Effect(if hit_enemy {
            SoundEffect::EnemyHit
        } else {
            SoundEffect::WallHit
        }));
        physics.remove(body);
        let _ = world.remove_one::<Body>(bullet);
        let _ = world.insert_one(bullet, Expiring);
        if let Ok(mut velocity) = world.get::<&mut Velocity>(bullet) {
            velocity.0 = Vec2::ZERO;
        }
        if let Ok(mut animation) = world.get::<&mut Animation>(bullet) {
            animation.play(BULLET_IMPACT);
        }
    }
}

#[cfg(test)]
mod tests {
    use winit::event::{ElementState, MouseButton};

    use crate::assets::Textures;
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
                bullet_texture: TextureId::WHITE,
                dt,
                roll: ShotRoll {
                    spread: 0.0,
                    damage: 0.0,
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
            1,
            TextureId::WHITE,
        );

        for _ in 0..30 {
            simulate(&mut world, &mut physics, 1.0 / 60.0);
            bullet_hits(&mut world, &mut physics, &mut Vec::new());
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
        let rat = prefabs::rat(
            &mut world,
            &mut physics,
            Vec2::new(300.0, 100.0),
            TextureId::WHITE,
        );
        let before = world.get::<&Health>(rat).map(|h| h.current).unwrap_or(0);
        let bullet = prefabs::bullet(
            &mut world,
            &mut physics,
            Vec2::new(200.0, 100.0),
            Vec2::new(700.0, 0.0),
            3,
            TextureId::WHITE,
        );

        for _ in 0..30 {
            simulate(&mut world, &mut physics, 1.0 / 60.0);
            bullet_hits(&mut world, &mut physics, &mut Vec::new());
        }

        let after = world.get::<&Health>(rat).map(|h| h.current).unwrap_or(0);
        assert_eq!(before - after, 3);
        assert!(world.get::<&Expiring>(bullet).is_ok());
    }
}
