use std::ops::ControlFlow;

use glam::Vec2;
use winit::event::{ElementState, KeyEvent, MouseButton};

use crate::assets::Assets;
use crate::input::Input;
use crate::render::{Draw, Font};
use crate::scenes::Scenes;

#[derive(Debug)]
pub(crate) struct Game {
    input: Input,
    scenes: Scenes,
    font: Font,
}

impl Game {
    pub(crate) fn new(assets: Assets) -> Self {
        Self {
            input: Input::default(),
            scenes: Scenes::new(assets.textures),
            font: assets.font,
        }
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
        self.scenes.fixed_update(&self.input, dt)
    }

    pub(crate) fn update(&mut self, dt: f32) -> ControlFlow<()> {
        let flow = self.scenes.update(&self.input, dt);
        self.input.end_frame();
        flow
    }

    pub(crate) fn draws(&self) -> Vec<Draw> {
        self.scenes.draws(&self.font)
    }
}
