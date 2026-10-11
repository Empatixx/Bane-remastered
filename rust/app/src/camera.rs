use glam::Vec2;

use crate::viewport::TARGET;

const TWEEN: f32 = 5.0;

#[derive(Debug, Default)]
pub(crate) struct Camera {
    position: Vec2,
    min: Vec2,
    max: Vec2,
    target_min: Vec2,
    target_max: Vec2,
}

impl Camera {
    pub(crate) fn new(bounds: Vec2) -> Self {
        let mut camera = Self::default();
        camera.set_bounds(Vec2::ZERO, bounds);
        camera
    }

    pub(crate) fn set_bounds(&mut self, area_min: Vec2, area_max: Vec2) {
        self.target_min = area_min;
        self.target_max = (area_max - TARGET).max(area_min);
        self.min = self.target_min;
        self.max = self.target_max;
    }

    pub(crate) fn glide_bounds(&mut self, area_min: Vec2, area_max: Vec2, dt: f32) {
        self.target_min = area_min;
        self.target_max = (area_max - TARGET).max(area_min);
        let tween = (TWEEN * dt).min(1.0);
        self.min += (self.target_min - self.min) * tween;
        self.max += (self.target_max - self.max) * tween;
    }

    pub(crate) fn snap_to(&mut self, target: Vec2) {
        self.position = target.clamp(self.min, self.max);
    }

    pub(crate) fn follow(&mut self, target: Vec2, dt: f32) {
        let tween = (TWEEN * dt).min(1.0);
        let next = self.position + (target - self.position) * tween;
        self.position = next.clamp(self.min, self.max);
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

    #[test]
    fn the_camera_is_confined_to_an_area_that_does_not_start_at_the_origin() {
        let mut camera = Camera::new(Vec2::new(5000.0, 5000.0));
        camera.set_bounds(Vec2::new(1000.0, 800.0), Vec2::new(2280.0, 1760.0));
        camera.snap_to(Vec2::new(0.0, 0.0));
        assert_eq!(camera.position(), Vec2::new(1000.0, 800.0));
        camera.snap_to(Vec2::new(9000.0, 9000.0));
        assert_eq!(
            camera.position(),
            Vec2::new(2280.0 - TARGET.x, 1760.0 - TARGET.y)
        );
    }

    #[test]
    fn bounds_glide_towards_a_new_area() {
        let mut camera = Camera::new(Vec2::new(1280.0, 960.0));
        camera.glide_bounds(Vec2::new(1000.0, 0.0), Vec2::new(2280.0, 960.0), 0.1);
        camera.snap_to(Vec2::new(0.0, 0.0));
        assert_eq!(camera.position(), Vec2::new(500.0, 0.0));
    }
}
