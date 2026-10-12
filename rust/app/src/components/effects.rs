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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RoomId(pub(crate) usize);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Floating {
    pub(crate) remaining: f32,
    pub(crate) velocity: glam::Vec2,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TrapCooldown {
    pub(crate) remaining: f32,
}
