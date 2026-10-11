use glam::Vec2;

use crate::viewport::TARGET;

const TWEEN: f32 = 5.0;

#[derive(Debug, Default)]
pub(crate) struct Camera {
    position: Vec2,
    max: Vec2,
}

impl Camera {
    pub(crate) fn new(bounds: Vec2) -> Self {
        Self {
            position: Vec2::ZERO,
            max: (bounds - TARGET).max(Vec2::ZERO),
        }
    }

    pub(crate) fn snap_to(&mut self, target: Vec2) {
        self.position = target.clamp(Vec2::ZERO, self.max);
    }

    pub(crate) fn follow(&mut self, target: Vec2, dt: f32) {
        let tween = (TWEEN * dt).min(1.0);
        let next = self.position + (target - self.position) * tween;
        self.position = next.clamp(Vec2::ZERO, self.max);
    }

    pub(crate) fn position(&self) -> Vec2 {
        self.position.round()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follow_moves_towards_the_target_and_stays_inside_the_map() {
        let mut camera = Camera::new(Vec2::new(2000.0, 1000.0));
        camera.follow(Vec2::new(500.0, 100.0), 0.1);
        assert_eq!(camera.position(), Vec2::new(250.0, 50.0));

        camera.snap_to(Vec2::new(5000.0, -300.0));
        assert_eq!(camera.position(), Vec2::new(2000.0 - TARGET.x, 0.0));
    }

    #[test]
    fn map_smaller_than_the_screen_pins_the_camera_to_the_origin() {
        let mut camera = Camera::new(Vec2::new(500.0, 300.0));
        camera.snap_to(Vec2::new(100.0, 100.0));
        assert_eq!(camera.position(), Vec2::ZERO);
    }
}
