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
