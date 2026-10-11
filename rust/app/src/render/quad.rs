use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub(crate) struct Quad {
    center: [f32; 2],
    size: [f32; 2],
    color: [f32; 4],
    uv: [f32; 4],
    lit: f32,
    angle: f32,
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
            lit: 0.0,
            angle: 0.0,
        }
    }

    pub(crate) fn rotated(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }

    pub(crate) fn lit(mut self, lit: bool) -> Self {
        self.set_lit(lit);
        self
    }

    pub(crate) fn set_lit(&mut self, lit: bool) {
        self.lit = if lit { 1.0 } else { 0.0 };
    }
}
