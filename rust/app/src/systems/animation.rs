use hecs::{Entity, World};

use crate::components::{Animation, Enemy, EnemyBullet, Expiring, Gate, Opening, Sprite, ThenPlay};

pub(crate) fn animate(world: &mut World, dt: f32) {
    let hostile_dt = dt * super::time_scale(world);
    for (animation, sprite, enemy, enemy_bullet) in world
        .query_mut::<(
            &mut Animation,
            &mut Sprite,
            Option<&Enemy>,
            Option<&EnemyBullet>,
        )>()
        .without::<&crate::components::Frozen>()
    {
        if enemy.is_some() || enemy_bullet.is_some() {
            animation.advance(hostile_dt);
        } else {
            animation.advance(dt);
        }
        let uv = Sprite::cell_uv(
            animation.frame(),
            animation.row(),
            animation.columns,
            animation.rows,
        );
        sprite.uv = if animation.flipped() {
            Sprite::flipped(uv)
        } else {
            uv
        };
    }
}

pub(crate) fn chain_animations(world: &mut World) {
    let next: Vec<(Entity, ThenPlay)> = world
        .query::<(Entity, &Animation, &ThenPlay)>()
        .iter()
        .filter(|(_, animation, _)| animation.finished())
        .map(|(entity, _, then)| (entity, *then))
        .collect();
    for (entity, then) in next {
        if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
            animation.play(then.0);
        }
        let _ = world.remove_one::<ThenPlay>(entity);
    }
}

pub(crate) fn open_gate(world: &mut World, entity: Entity) {
    let Ok(clip) = world.get::<&Gate>(entity).map(|gate| gate.0) else {
        return;
    };
    if let Ok(mut animation) = world.get::<&mut Animation>(entity) {
        animation.play(clip);
        animation.hold(clip.frames - 1);
    }
    let _ = world.remove_one::<ThenPlay>(entity);
    let _ = world.insert_one(entity, Opening(0.0));
}

pub(crate) fn tick_openings(world: &mut World, dt: f32) {
    let mut done = Vec::new();
    for (entity, gate, opening, animation) in
        world.query_mut::<(Entity, &Gate, &mut Opening, &mut Animation)>()
    {
        opening.0 += dt;
        let step = (opening.0 / gate.0.frame_time) as usize;
        if step >= gate.0.frames {
            done.push(entity);
        } else {
            animation.hold(gate.0.frames - 1 - step);
        }
    }
    for entity in done {
        let _ = world.despawn(entity);
    }
}

pub(crate) fn remove_expired(world: &mut World) {
    let finished: Vec<Entity> = world
        .query::<(Entity, &Animation)>()
        .with::<&Expiring>()
        .iter()
        .filter(|(_, animation)| animation.finished())
        .map(|(entity, _)| entity)
        .collect();
    for entity in finished {
        let _ = world.despawn(entity);
    }
}

#[cfg(test)]
mod tests {
    use crate::assets::Textures;
    use crate::dungeon::Direction;
    use crate::physics::Physics;
    use crate::prefabs;

    use super::*;

    fn gate(direction: Direction) -> (World, Entity) {
        let mut world = World::new();
        let mut physics = Physics::new();
        let (entity, _) = prefabs::gate(
            &mut world,
            &mut physics,
            (3, 4),
            direction,
            &Textures::default(),
        );
        (world, entity)
    }

    fn shown_frame(world: &World, entity: Entity) -> Option<usize> {
        world.get::<&Animation>(entity).ok().map(|a| a.frame())
    }

    #[test]
    fn the_hourglass_slows_enemy_animations_but_not_the_rest() {
        let mut world = World::new();
        let player = prefabs::player(
            &mut world,
            &mut Physics::new(),
            glam::Vec2::ZERO,
            &Textures::default(),
        );
        let _ = world.insert_one(
            player,
            crate::components::TimeSlow {
                remaining: crate::artefacts::TIME_SLOW_SECONDS / 2.0,
            },
        );
        let clip = crate::clip::Clip {
            row: 0,
            frames: 4,
            frame_time: 0.1,
            looping: true,
            flipped: false,
        };
        let sprite = Sprite::whole(crate::render::TextureId::WHITE);
        let shot = world.spawn((EnemyBullet, Animation::new(4, 1, clip), sprite));
        let torch = world.spawn((Animation::new(4, 1, clip), sprite));

        animate(&mut world, 0.25);

        assert_eq!(shown_frame(&world, shot), Some(0));
        assert_eq!(shown_frame(&world, torch), Some(2));
    }

    #[test]
    fn an_opening_gate_plays_its_closing_frames_backwards_and_then_goes_away() {
        let (mut world, entity) = gate(Direction::Up);
        let clip = world
            .get::<&Gate>(entity)
            .map(|gate| gate.0)
            .expect("a gate");
        open_gate(&mut world, entity);

        assert_eq!(shown_frame(&world, entity), Some(clip.frames - 1));
        assert!(!world.satisfies::<&ThenPlay>(entity));
        let mut frames = Vec::new();
        for _ in 0..clip.frames {
            tick_openings(&mut world, clip.frame_time + 0.001);
            if let Some(frame) = shown_frame(&world, entity) {
                frames.push(frame);
            }
        }

        assert!(
            frames.windows(2).all(|pair| pair[0] > pair[1]),
            "{frames:?}"
        );
        assert!(world.get::<&Gate>(entity).is_err());
    }

    #[test]
    fn only_gates_can_be_opened() {
        let mut world = World::new();
        let plain = world.spawn((Sprite::whole(crate::render::TextureId::WHITE),));

        open_gate(&mut world, plain);

        assert!(!world.satisfies::<&Opening>(plain));
    }
}
