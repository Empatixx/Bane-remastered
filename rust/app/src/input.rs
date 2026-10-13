use std::collections::HashSet;

use glam::Vec2;
use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::controls::{Action, Bindings};

#[derive(Debug, Default)]
pub(crate) struct Input {
    held: HashSet<KeyCode>,
    pressed: HashSet<KeyCode>,
    mouse: Vec2,
    clicked: bool,
    mouse_held: bool,
    scrolled: i32,
    bindings: Bindings,
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

    pub(crate) fn set_bindings(&mut self, bindings: Bindings) {
        self.bindings = bindings;
    }

    pub(crate) fn bindings(&self) -> &Bindings {
        &self.bindings
    }

    pub(crate) fn action_pressed(&self, action: Action) -> bool {
        self.just_pressed(self.bindings.key(action))
    }

    pub(crate) fn first_pressed(&self) -> Option<KeyCode> {
        self.pressed.iter().next().copied()
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
        let axis = |negative: Action, positive: Action| {
            f32::from(u8::from(self.held.contains(&self.bindings.key(positive))))
                - f32::from(u8::from(self.held.contains(&self.bindings.key(negative))))
        };
        Vec2::new(
            axis(Action::MoveLeft, Action::MoveRight),
            axis(Action::MoveUp, Action::MoveDown),
        )
        .normalize_or_zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_and_actions_follow_the_current_bindings() {
        let mut input = Input::default();
        let mut bindings = Bindings::default();
        bindings.rebind(Action::MoveRight, KeyCode::ArrowRight);
        bindings.rebind(Action::Interact, KeyCode::KeyX);
        input.set_bindings(bindings);

        input.press(KeyCode::KeyD);
        assert_eq!(input.direction(), Vec2::ZERO);
        input.press(KeyCode::ArrowRight);
        assert_eq!(input.direction(), Vec2::X);

        assert!(!input.action_pressed(Action::Interact));
        input.press(KeyCode::KeyX);
        assert!(input.action_pressed(Action::Interact));
    }
}
