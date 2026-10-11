use std::fmt;

use glam::{Vec2, Vec4};
use hecs::World;

use crate::components::{Color, Position, Size};
use crate::gpu::Quad;

pub(crate) struct Game {
    world: World,
    ticks: u64,
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game")
            .field("entities", &self.world.len())
            .field("ticks", &self.ticks)
            .finish()
    }
}

impl Game {
    pub(crate) fn new() -> Self {
        let mut world = World::new();
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
        Self { world, ticks: 0 }
    }

    pub(crate) fn fixed_update(&mut self) {
        self.ticks += 1;
        if self.ticks.is_multiple_of(60) {
            tracing::info!(ticks = self.ticks, "one second passed");
        }
    }

    pub(crate) fn quads(&self) -> Vec<Quad> {
        self.world
            .query::<(&Position, &Size, &Color)>()
            .iter()
            .map(|(position, size, color)| Quad::new(position.0, size.0, color.0))
            .collect()
    }
}
