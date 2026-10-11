use std::ops::ControlFlow;

use glam::Vec2;
use winit::event::{ElementState, KeyEvent, MouseButton};

use crate::assets::Textures;
use crate::input::Input;
use crate::render::Draw;
use crate::scenes::Scenes;

#[derive(Debug)]
pub(crate) struct Game {
    input: Input,
    scenes: Scenes,
}

impl Game {
    pub(crate) fn new(textures: Textures) -> Self {
        Self {
            input: Input::default(),
            scenes: Scenes::new(textures),
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
        self.scenes.draws()
    }
}
