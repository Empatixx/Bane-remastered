use glam::Vec2;

use crate::viewport::TARGET;

const TWEEN: f32 = 5.0;
const MAX_SHAKE: f32 = 12.0;
pub(crate) const TRAUMA_DECAY: f32 = 1.5;
const SHAKE_SPEED: f32 = 30.0;

#[derive(Debug, Default)]
pub(crate) struct Camera {
    position: Vec2,
    min: Vec2,
    max: Vec2,
    target_min: Vec2,
    target_max: Vec2,
    trauma: f32,
    shake_time: f32,
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

    /// Where the camera looks for gameplay: aiming, mouse picking. Never shakes.
    pub(crate) fn position(&self) -> Vec2 {
        self.position.round()
    }

    /// Where the camera draws from: the position plus the screen shake.
    pub(crate) fn view(&self) -> Vec2 {
        (self.position + self.shake_offset()).round()
    }

    /// Adds trauma between 0 and 1; the shake grows with its square, so small hits stay subtle.
    pub(crate) fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).clamp(0.0, 1.0);
    }

    pub(crate) fn update_shake(&mut self, dt: f32) {
        self.shake_time += dt * SHAKE_SPEED;
        self.trauma = (self.trauma - TRAUMA_DECAY * dt).max(0.0);
    }

    fn shake_offset(&self) -> Vec2 {
        let strength = self.trauma * self.trauma * MAX_SHAKE;
        Vec2::new(wave(self.shake_time, 0.0), wave(self.shake_time, 7.3)) * strength
    }
}

/// Smooth noise between -1 and 1 built from three detuned sines.
fn wave(time: f32, seed: f32) -> f32 {
    let first = (time + seed).sin();
    let second = (time * 2.17 + seed * 1.9).sin() * 0.5;
    let third = (time * 4.31 + seed * 3.7).sin() * 0.25;
    (first + second + third) / 1.75
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
    fn a_still_camera_views_from_its_position() {
        let mut camera = Camera::new(Vec2::new(5000.0, 5000.0));
        camera.snap_to(Vec2::new(300.0, 200.0));
        camera.update_shake(0.3);
        assert_eq!(camera.view(), camera.position());
    }

    #[test]
    fn trauma_shakes_only_the_view_within_the_limit_and_fades_out() {
        let mut camera = Camera::new(Vec2::new(5000.0, 5000.0));
        camera.snap_to(Vec2::new(300.0, 200.0));
        camera.add_trauma(5.0);
        let mut moved = false;
        for _ in 0..20 {
            camera.update_shake(1.0 / 60.0);
            let offset = camera.view() - camera.position();
            assert!(offset.x.abs() <= MAX_SHAKE + 1.0 && offset.y.abs() <= MAX_SHAKE + 1.0);
            moved |= offset != Vec2::ZERO;
        }
        assert!(moved);
        assert_eq!(camera.position(), Vec2::new(300.0, 200.0));

        camera.update_shake(1.0);
        assert_eq!(camera.view(), camera.position());
    }

    #[test]
    fn bounds_glide_towards_a_new_area() {
        let mut camera = Camera::new(Vec2::new(1280.0, 960.0));
        camera.glide_bounds(Vec2::new(1000.0, 0.0), Vec2::new(2280.0, 960.0), 0.1);
        camera.snap_to(Vec2::new(0.0, 0.0));
        assert_eq!(camera.position(), Vec2::new(500.0, 0.0));
    }
}
