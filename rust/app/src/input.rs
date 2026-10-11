use std::collections::HashSet;

use glam::Vec2;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Debug, Default)]
pub(crate) struct Input {
    held: HashSet<KeyCode>,
}

impl Input {
    pub(crate) fn handle_key(&mut self, event: &KeyEvent) {
        let PhysicalKey::Code(code) = event.physical_key else {
            return;
        };
        match event.state {
            ElementState::Pressed => self.held.insert(code),
            ElementState::Released => self.held.remove(&code),
        };
    }

    pub(crate) fn direction(&self) -> Vec2 {
        let axis = |negative: KeyCode, positive: KeyCode| {
            f32::from(u8::from(self.held.contains(&positive)))
                - f32::from(u8::from(self.held.contains(&negative)))
        };
        Vec2::new(
            axis(KeyCode::KeyA, KeyCode::KeyD),
            axis(KeyCode::KeyW, KeyCode::KeyS),
        )
        .normalize_or_zero()
    }
}
