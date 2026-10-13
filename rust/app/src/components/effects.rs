pub(crate) const PLAYER_GAZE_SPEED: f32 = 350.0;

#[derive(Debug, Clone, Copy)]
pub(crate) struct GazeWind {
    pub(crate) cooldown: f32,
    pub(crate) speed_limit: f32,
}

impl GazeWind {
    pub(crate) fn above(speed_limit: f32) -> Self {
        Self {
            cooldown: 0.0,
            speed_limit,
        }
    }
}

impl Default for GazeWind {
    fn default() -> Self {
        Self::above(PLAYER_GAZE_SPEED)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Particle;

#[derive(Debug, Clone, Copy)]
pub(crate) struct LightFade {
    pub(crate) base: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ThenPlay(pub(crate) super::animation::Clip);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Gate(pub(crate) super::animation::Clip);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Opening(pub(crate) f32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RoomId(pub(crate) usize);

pub(crate) const HOVER_AMPLITUDE: f32 = 2.5;
pub(crate) const HOVER_CYCLE: f32 = 0.5;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Hover {
    pub(crate) cycle: f32,
}

impl Hover {
    pub(crate) fn offset(&self) -> f32 {
        let phase = self.cycle.rem_euclid(1.0);
        if phase < 0.5 {
            HOVER_AMPLITUDE * (1.0 - 4.0 * phase)
        } else {
            HOVER_AMPLITUDE * (4.0 * phase - 3.0)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Floating {
    pub(crate) remaining: f32,
    pub(crate) velocity: glam::Vec2,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TrapCooldown {
    pub(crate) remaining: f32,
}

#[cfg(test)]
mod hover_tests {
    use super::*;

    #[test]
    fn hovering_swings_linearly_between_plus_and_minus_two_and_a_half_pixels() {
        assert_eq!(Hover { cycle: 0.0 }.offset(), 2.5);
        assert_eq!(Hover { cycle: 0.25 }.offset(), 0.0);
        assert_eq!(Hover { cycle: 0.5 }.offset(), -2.5);
        assert_eq!(Hover { cycle: 0.75 }.offset(), 0.0);
        assert_eq!(Hover { cycle: 1.0 }.offset(), 2.5);
    }
}
