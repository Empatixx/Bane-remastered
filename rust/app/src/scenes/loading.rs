use glam::Vec2;
use hecs::World;

use crate::components::{LoadingBar, Position, Size};
use crate::input::Input;
use crate::prefabs;
use crate::viewport::TARGET;

use super::{Scene, SceneId, Transition};

const DURATION: f32 = 1.0;
const BAR_WIDTH: f32 = 400.0;

#[derive(Default)]
pub(crate) struct LoadingScene {
    world: World,
    elapsed: f32,
}

impl Scene for LoadingScene {
    fn world(&self) -> &World {
        &self.world
    }

    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn enter(&mut self) {
        self.elapsed = 0.0;
        let center = Vec2::new(TARGET.x / 2.0, 380.0);
        prefabs::block(
            &mut self.world,
            center,
            Vec2::new(BAR_WIDTH + 8.0, 24.0),
            glam::Vec4::new(0.12, 0.14, 0.2, 1.0),
        );
        prefabs::loading_bar(&mut self.world, center);
        prefabs::label(
            &mut self.world,
            Vec2::new(center.x, center.y - 40.0),
            "Načítání",
        );
    }

    fn update(&mut self, _input: &Input, dt: f32) -> Transition {
        self.elapsed += dt;
        let progress = (self.elapsed / DURATION).min(1.0);
        let left = TARGET.x / 2.0 - BAR_WIDTH / 2.0;
        let width = BAR_WIDTH * progress;
        for (position, size) in self
            .world
            .query_mut::<(&mut Position, &mut Size)>()
            .with::<&LoadingBar>()
        {
            position.0.x = left + width / 2.0;
            size.0.x = width;
        }
        if progress >= 1.0 {
            Transition::Switch(SceneId::Menu)
        } else {
            Transition::None
        }
    }
}
