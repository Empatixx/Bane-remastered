use std::collections::HashSet;

use glam::Vec2;
use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Debug, Default)]
pub(crate) struct Input {
    held: HashSet<KeyCode>,
    pressed: HashSet<KeyCode>,
    mouse: Vec2,
    clicked: bool,
    mouse_held: bool,
    scrolled: i32,
}

impl Input {
    pub(crate) fn handle_key(&mut self, event: &KeyEvent) {
        let PhysicalKey::Code(code) = event.physical_key else {
            return;
        };
        match event.state {
            ElementState::Pressed => {
                if self.held.insert(code) {
                    self.pressed.insert(code);
                }
            }
            ElementState::Released => {
                self.held.remove(&code);
            }
        }
    }

    pub(crate) fn handle_click(&mut self, button: MouseButton, state: ElementState) {
        if button != MouseButton::Left {
            return;
        }
        match state {
            ElementState::Pressed => {
                self.clicked = true;
                self.mouse_held = true;
            }
            ElementState::Released => self.mouse_held = false,
        }
    }

    pub(crate) fn handle_scroll(&mut self, lines: f32) {
        if lines > 0.0 {
            self.scrolled += 1;
        } else if lines < 0.0 {
            self.scrolled -= 1;
        }
    }

    pub(crate) fn any_key(&self) -> bool {
        !self.pressed.is_empty()
    }

    pub(crate) fn scrolled(&self) -> i32 {
        self.scrolled
    }

    pub(crate) fn set_mouse(&mut self, position: Vec2) {
        self.mouse = position;
    }

    pub(crate) fn end_frame(&mut self) {
        self.pressed.clear();
        self.clicked = false;
        self.scrolled = 0;
    }

    #[cfg(test)]
    pub(crate) fn press(&mut self, code: KeyCode) {
        self.held.insert(code);
        self.pressed.insert(code);
    }

    pub(crate) fn just_pressed(&self, code: KeyCode) -> bool {
        self.pressed.contains(&code)
    }

    pub(crate) fn mouse(&self) -> Vec2 {
        self.mouse
    }

    pub(crate) fn mouse_down(&self) -> bool {
        self.mouse_held
    }

    pub(crate) fn clicked(&self) -> bool {
        self.clicked
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
