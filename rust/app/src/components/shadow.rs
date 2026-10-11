#[derive(Debug, Clone, Copy)]
pub(crate) struct PlayerShadow {
    pub(crate) angle: f32,
    pub(crate) intensity: f32,
    pub(crate) length: f32,
}

impl Default for PlayerShadow {
    fn default() -> Self {
        Self {
            angle: 0.0,
            intensity: 0.0,
            length: 20.0,
        }
    }
}
