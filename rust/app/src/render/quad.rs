use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub(crate) struct Quad {
    center: [f32; 2],
    size: [f32; 2],
    color: [f32; 4],
    uv: [f32; 4],
}

impl Quad {
    pub(crate) fn new(center: Vec2, size: Vec2, color: Vec4) -> Self {
        Self::with_uv(center, size, color, [0.0, 0.0, 1.0, 1.0])
    }

    pub(crate) fn with_uv(center: Vec2, size: Vec2, color: Vec4, uv: [f32; 4]) -> Self {
        Self {
            center: center.to_array(),
            size: size.to_array(),
            color: color.to_array(),
            uv,
        }
    }
}
