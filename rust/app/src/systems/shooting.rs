use glam::Vec2;
use hecs::{Entity, World};

use crate::audio::{AudioCommand, SoundEffect};
use crate::components::BULLET_IMPACT;
use crate::components::{
    Animation, Body, Bullet, Burst, Expiring, Health, HitFlash, Loadout, Player, PlayerPerks,
    Position, Velocity,
};
use crate::controls::Action;
use crate::guns::{GunSpec, BURST_DELAY, MAX_PELLETS};
use crate::input::Input;
use crate::physics::{Owner, Physics};
use crate::prefabs;
use crate::render::TextureId;
use crate::systems::enemies::FLASH_SECONDS;
use crate::upgrades::Perks;

const BULLET_SPEED: f32 = 700.0;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ShotRoll {
    pub(crate) spread: f32,
    pub(crate) damages: [f32; MAX_PELLETS],
    pub(crate) critical: f32,
}

impl ShotRoll {
    pub(crate) fn random() -> Self {
        Self {
            spread: rand::random_range(-1.0..=1.0),
            damages: std::array::from_fn(|_| rand::random_range(0.0..1.0)),
            critical: rand::random_range(0.0..1.0),
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

struct Shot {
    origin: Vec2,
    direction: Vec2,
    spec: GunSpec,
    roll: ShotRoll,
    pellets: usize,
    perks: Perks,
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
    for (position, loadout, perks) in world
        .query_mut::<(&Position, &mut Loadout, Option<&PlayerPerks>)>()
        .with::<&Player>()
        .without::<&crate::components::Dodge>()
    {
        let perks = perks.map_or_else(Perks::default, |perks| perks.0);
        let Some(weapon) = loadout.current_mut() else {
            continue;
        };
        let spec = weapon.spec();
        weapon.advance(dt);
        if input.action_pressed(Action::Reload) && weapon.start_reload() {
            sounds.push(AudioCommand::Effect(spec.reload));
        }
        if let Some(mut burst) = weapon.burst {
            burst.timer -= dt;
            if burst.timer <= 0.0 {
                if weapon.clip > 0 {
                    weapon.clip -= 1;
                    weapon.cooldown = spec.fire_interval;
                    shots.push(Shot {
                        origin: position.0,
                        direction: burst.aim,
                        spec,
                        roll,
                        pellets: 1,
                        perks,
                    });
                    burst.left -= 1;
                    burst.timer = BURST_DELAY;
                }
                weapon.burst = (burst.left > 0 && weapon.clip > 0).then_some(burst);
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
        if weapon.clip == 0 {
            sounds.push(AudioCommand::Effect(SoundEffect::EmptyGunShoot));
            if spec.auto {
                weapon.cooldown = spec.fire_interval;
            }
            continue;
        }
        let to_target = target - position.0;
        if to_target == Vec2::ZERO {
            continue;
        }
        weapon.cooldown = spec.fire_interval;
        weapon.clip -= 1;
        sounds.push(AudioCommand::Effect(spec.shoot));
        if let Some(count) = spec.burst {
            weapon.burst = (count > 1 && weapon.clip > 0).then_some(Burst {
                left: count - 1,
                timer: BURST_DELAY,
                aim: to_target,
            });
        }
        shots.push(Shot {
            origin: position.0,
            direction: to_target,
            spec,
            roll,
            pellets: spec.pellets,
            perks,
        });
    }
    let fired: u32 = shots.iter().map(|shot| shot.pellets as u32).sum();
    super::stats::record(world, super::stats::Stat::Fired, fired);
    for shot in shots {
        let aim = shot.direction.to_angle();
        for pellet in 0..shot.pellets {
            let angle = aim + shot.spec.angle_offset(pellet, shot.roll.spread);
            let damage = shot.perks.bullet_damage(
                shot.spec.damage_for(shot.roll.damages[pellet]),
                shot.roll.critical,
            );
            prefabs::bullet(
                world,
                physics,
                shot.origin,
                Vec2::from_angle(angle) * BULLET_SPEED,
                damage,
                bullet_texture,
            );
        }
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
        if let (true, Some(entity)) = (hit_enemy, victim) {
            super::skills::count_hits(world, entity);
            if world.get::<&crate::components::Enemy>(entity).is_ok() {
                super::stats::record(world, super::stats::Stat::Hit, 1);
            }
            if world.get::<&crate::components::Breakable>(entity).is_err() {
                super::effects::show_change(world, entity, -damage, rand::random_range(0.0..1.0));
            }
            let _ = world.insert_one(
                entity,
                HitFlash {
                    remaining: FLASH_SECONDS,
                },
            );
        }
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
    use winit::keyboard::KeyCode;

    use crate::assets::Textures;
    use crate::components::{Player, Weapon};
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
                    damages: [0.0; MAX_PELLETS],
                    critical: 1.0,
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

    fn give(world: &mut World, kind: crate::guns::GunKind) {
        for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
            loadout.drop_current();
            loadout.add(Weapon::new(kind));
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
                bullet_texture: TextureId::WHITE,
                dt,
                roll: ShotRoll {
                    spread: 0.0,
                    damages: [0.0; MAX_PELLETS],
                    critical: 1.0,
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
}
