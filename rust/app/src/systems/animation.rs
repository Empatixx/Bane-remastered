use hecs::{Entity, World};

use crate::components::{Animation, Expiring, Sprite, ThenPlay};

pub(crate) fn animate(world: &mut World, dt: f32) {
    for (animation, sprite) in world.query_mut::<(&mut Animation, &mut Sprite)>() {
        animation.advance(dt);
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
