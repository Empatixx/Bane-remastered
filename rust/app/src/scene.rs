use glam::{Vec2, Vec4};
use hecs::World;

use crate::components::{Color, Position, Size};

pub(crate) fn populate(world: &mut World) {
    world.spawn((
        Position(Vec2::new(200.0, 200.0)),
        Size(Vec2::new(64.0, 64.0)),
        Color(Vec4::new(0.9, 0.3, 0.2, 1.0)),
    ));
    world.spawn((
        Position(Vec2::new(400.0, 300.0)),
        Size(Vec2::new(120.0, 40.0)),
        Color(Vec4::new(0.2, 0.7, 0.4, 1.0)),
    ));
    world.spawn((
        Position(Vec2::new(600.0, 150.0)),
        Size(Vec2::new(80.0, 80.0)),
        Color(Vec4::new(0.3, 0.5, 0.9, 1.0)),
    ));
}
