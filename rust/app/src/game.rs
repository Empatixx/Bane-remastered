use std::ops::ControlFlow;

use glam::Vec2;
use winit::event::{ElementState, KeyEvent, MouseButton};

use crate::assets::{Assets, CURSOR};
use crate::input::Input;
use crate::render::{Draw, Quad};
use crate::scenes::Scenes;

const CURSOR_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const CURSOR_HOTSPOT: Vec2 = Vec2::new(6.0, 3.5);

#[derive(Debug)]
pub(crate) struct Game {
    input: Input,
    scenes: Scenes,
    assets: Assets,
}

impl Game {
    pub(crate) fn new(assets: Assets) -> Self {
        Self {
            input: Input::default(),
            scenes: Scenes::new(&assets),
            assets,
        }
    }

    pub(crate) fn assets_mut(&mut self) -> &mut Assets {
        &mut self.assets
    }

    pub(crate) fn handle_key(&mut self, event: &KeyEvent) {
        self.input.handle_key(event);
    }

    pub(crate) fn handle_click(&mut self, button: MouseButton, state: ElementState) {
        self.input.handle_click(button, state);
    }

    pub(crate) fn set_mouse(&mut self, position: Vec2) {
        self.input.set_mouse(position);
    }

    pub(crate) fn fixed_update(&mut self, dt: f32) -> ControlFlow<()> {
        self.scenes.fixed_update(&self.input, dt, &self.assets)
    }

    pub(crate) fn update(&mut self, dt: f32) -> ControlFlow<()> {
        let flow = self.scenes.update(&self.input, dt, &self.assets);
        self.input.end_frame();
        flow
    }

    pub(crate) fn draws(&self) -> Vec<Draw> {
        let mut draws = self.scenes.draws(&self.assets.fonts);
        if self.assets.is_loaded() {
            let top_left = self.input.mouse() - CURSOR_HOTSPOT;
            draws.push(Draw {
                texture: self.assets.textures.get(CURSOR),
                quad: Quad::new(top_left + CURSOR_SIZE / 2.0, CURSOR_SIZE, glam::Vec4::ONE),
            });
        }
        draws
    }
}
