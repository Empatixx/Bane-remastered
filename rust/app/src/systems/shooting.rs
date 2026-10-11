use glam::Vec2;
use hecs::{Entity, World};

use crate::components::BULLET_IMPACT;
use crate::components::{Animation, Body, Bullet, Impact, Player, Position, Velocity, Weapon};
use crate::input::Input;
use crate::physics::{Owner, Physics};
use crate::prefabs;
use crate::render::TextureId;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ShotRoll {
    pub(crate) spread: f32,
}

impl ShotRoll {
    pub(crate) fn random() -> Self {
        Self {
            spread: rand::random_range(-1.0..=1.0),
        }
    }
}

pub(crate) fn shoot(
    world: &mut World,
    physics: &mut Physics,
    input: &Input,
    camera: Vec2,
    bullet_texture: TextureId,
    dt: f32,
    roll: ShotRoll,
) {
    let target = input.mouse() + camera;
    let mut shots = Vec::new();
    for (position, weapon) in world
        .query_mut::<(&Position, &mut Weapon)>()
        .with::<&Player>()
    {
        weapon.cooldown = (weapon.cooldown - dt).max(0.0);
        if !input.clicked() || weapon.cooldown > 0.0 {
            continue;
        }
        let to_target = target - position.0;
        if to_target == Vec2::ZERO {
            continue;
        }
        weapon.cooldown = weapon.fire_interval;
        let angle = to_target.to_angle() + roll.spread * weapon.inaccuracy;
        shots.push((position.0, Vec2::from_angle(angle) * weapon.bullet_speed));
    }
    for (position, velocity) in shots {
        prefabs::bullet(world, physics, position, velocity, bullet_texture);
    }
}

pub(crate) fn bullet_hits(world: &mut World, physics: &mut Physics) {
    let hit: Vec<(Entity, Body)> = world
        .query::<(Entity, &Body)>()
        .with::<&Bullet>()
        .without::<&Impact>()
        .iter()
        .filter(|(_, body)| physics.touching(**body).contains(&Owner::Wall))
        .map(|(entity, body)| (entity, *body))
        .collect();
    for (entity, body) in hit {
        physics.remove(body);
        let _ = world.remove_one::<Body>(entity);
        let _ = world.insert_one(entity, Impact);
        if let Ok(mut velocity) = world.get::<&mut Velocity>(entity) {
            velocity.0 = Vec2::ZERO;
        }
        if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
            animation.play(BULLET_IMPACT);
        }
    }
}

#[cfg(test)]
mod tests {
    use winit::event::{ElementState, MouseButton};

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
        prefabs::player(&mut world, &mut physics, Vec2::new(100.0, 100.0));
        (world, physics)
    }

    fn fire(world: &mut World, physics: &mut Physics, input: &Input, camera: Vec2, dt: f32) {
        shoot(
            world,
            physics,
            input,
            camera,
            TextureId::WHITE,
            dt,
            ShotRoll { spread: 0.0 },
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
            TextureId::WHITE,
        );

        for _ in 0..30 {
            simulate(&mut world, &mut physics, 1.0 / 60.0);
            bullet_hits(&mut world, &mut physics);
        }

        assert!(world.get::<&Impact>(bullet).is_ok());
        assert!(world.get::<&Body>(bullet).is_err());
        let position = world.get::<&Position>(bullet).map(|p| p.0).ok();
        assert!(
            position.is_some_and(|p| p.x > 184.0 && p.x < 216.0),
            "{position:?}"
        );
    }
}
