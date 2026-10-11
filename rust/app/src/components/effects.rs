#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct GazeWind {
    pub(crate) cooldown: f32,
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
