use glam::{Vec2, Vec3};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Lit;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Light {
    pub(crate) offset: Vec2,
    pub(crate) color: Vec3,
    pub(crate) radius: f32,
    pub(crate) intensity: f32,
}

impl Light {
    pub(crate) fn rgb(
        offset: Vec2,
        red: f32,
        green: f32,
        blue: f32,
        radius: f32,
        intensity: f32,
    ) -> Self {
        Self {
            offset,
            color: Vec3::new(red, green, blue) / 255.0,
            radius,
            intensity,
        }
    }
}
