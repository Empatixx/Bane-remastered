use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Dodge {
    pub(crate) remaining: f32,
    pub(crate) direction: Vec2,
    pub(crate) trail: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Afterimage {
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DodgeCooldown {
    pub(crate) remaining: f32,
}
