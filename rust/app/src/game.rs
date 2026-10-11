use std::fmt;

use hecs::World;

use crate::input::Input;
use crate::render::Quad;
use crate::{scene, systems};

pub(crate) struct Game {
    world: World,
    input: Input,
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
        scene::populate(&mut world);
        Self {
            world,
            input: Input::default(),
            ticks: 0,
        }
    }

    pub(crate) fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub(crate) fn fixed_update(&mut self, dt: f32) {
        systems::steer_player(&mut self.world, &self.input);
        systems::apply_velocity(&mut self.world, dt);
        self.ticks += 1;
        if self.ticks.is_multiple_of(60) {
            tracing::info!(ticks = self.ticks, "one second passed");
        }
    }

    pub(crate) fn quads(&self) -> Vec<Quad> {
        systems::collect_quads(&self.world)
    }
}
