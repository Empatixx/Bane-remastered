use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Position(pub(crate) Vec2);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Previous(pub(crate) Vec2);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Blend(pub(crate) f32);

impl Default for Blend {
    fn default() -> Self {
        Self(1.0)
    }
}

const TELEPORT_DISTANCE: f32 = 128.0;

pub(crate) fn blended(position: Vec2, previous: Option<&Previous>, blend: f32) -> Vec2 {
    match previous {
        Some(previous) if previous.0.distance(position) <= TELEPORT_DISTANCE => {
            previous.0.lerp(position, blend.clamp(0.0, 1.0))
        }
        _ => position,
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Size(pub(crate) Vec2);
