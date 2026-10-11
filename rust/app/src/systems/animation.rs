use hecs::{Entity, World};

use crate::components::{Animation, Impact, Sprite};

pub(crate) fn animate(world: &mut World, dt: f32) {
    for (animation, sprite) in world.query_mut::<(&mut Animation, &mut Sprite)>() {
        animation.advance(dt);
        sprite.uv = Sprite::cell_uv(
            animation.frame(),
            animation.row(),
            animation.columns,
            animation.rows,
        );
    }
}

pub(crate) fn finish_impacts(world: &mut World) {
    let finished: Vec<Entity> = world
        .query::<(Entity, &Animation)>()
        .with::<&Impact>()
        .iter()
        .filter(|(_, animation)| animation.finished())
        .map(|(entity, _)| entity)
        .collect();
    for entity in finished {
        let _ = world.despawn(entity);
    }
}
