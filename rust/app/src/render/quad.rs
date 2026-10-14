use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};

pub(crate) const MODE_MAP: f32 = 1.0;
pub(crate) const MODE_FOG: f32 = 2.0;
pub(crate) const MODE_SKY: f32 = 3.0;
pub(crate) const MODE_LIQUID: f32 = 4.0;
// fx.w mode 5 is the shadow mode, so the next mode must use 6 or more.
pub(crate) const MODE_SHADOW: f32 = 5.0;

const SHADOW_RIM: [f32; 4] = [0.55, 0.62, 1.0, 0.35];

#[derive(Debug, Clone, Copy)]
pub(crate) struct MapRoomLook {
    pub(crate) time: f32,
    pub(crate) seed: f32,
    pub(crate) links: u8,
    pub(crate) fogged: bool,
    pub(crate) local: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub(crate) struct Quad {
    center: [f32; 2],
    size: [f32; 2],
    color: [f32; 4],
    uv: [f32; 4],
    lit: f32,
    angle: f32,
    fx: [f32; 4],
    local: [f32; 4],
    outline: [f32; 4],
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
            fx: [0.0; 4],
            local: [0.0, 0.0, 1.0, 1.0],
            outline: [0.0; 4],
        }
    }

    pub(super) fn remap_uv(&mut self, region: [f32; 4]) {
        let [x, y, width, height] = region;
        let [u0, v0, u1, v1] = self.uv;
        self.uv = [
            x + u0 * width,
            y + v0 * height,
            x + u1 * width,
            y + v1 * height,
        ];
    }

    pub(crate) fn map_room(mut self, look: MapRoomLook) -> Self {
        self.fx = [
            look.time,
            look.seed,
            f32::from(look.links),
            if look.fogged { MODE_FOG } else { MODE_MAP },
        ];
        self.local = look.local;
        self
    }

    pub(crate) fn outlined(mut self, color: Vec4) -> Self {
        self.outline = color.to_array();
        self
    }

    pub(crate) fn underworld(mut self, time: f32) -> Self {
        self.fx = [time, 0.0, 0.0, MODE_SKY];
        self
    }

    pub(crate) fn liquid(mut self, kind: f32, time: f32) -> Self {
        self.fx = [time, kind, 0.0, MODE_LIQUID];
        self
    }

    pub(crate) fn shadowed(mut self, time: f32) -> Self {
        self.fx = [time, 0.0, 0.0, MODE_SHADOW];
        self.outline = SHADOW_RIM;
        self
    }

    pub(crate) fn shined(mut self, progress: f32) -> Self {
        self.fx[1] = progress;
        self
    }

    pub(crate) fn frosted(mut self, strength: f32) -> Self {
        self.fx[0] = strength;
        self
    }

    pub(crate) fn rotated(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }

    pub(crate) fn lit(mut self, lit: bool) -> Self {
        self.set_lit(lit);
        self
    }

    #[cfg(test)]
    pub(crate) fn is_shadowed(&self) -> bool {
        self.fx[3] == MODE_SHADOW
    }

    pub(crate) fn set_lit(&mut self, lit: bool) {
        self.lit = if lit { 1.0 } else { 0.0 };
    }
}

#[cfg(test)]
mod tests {
    use glam::{Vec2, Vec4};

    use super::*;

    #[test]
    fn liquid_quads_carry_their_kind_and_the_clock() {
        let quad = Quad::new(Vec2::ZERO, Vec2::ONE, Vec4::ONE).liquid(2.0, 7.5);
        assert_eq!(quad.fx, [7.5, 2.0, 0.0, MODE_LIQUID]);
    }

    #[test]
    fn a_shadowed_quad_uses_the_shadow_mode_the_clock_and_a_cool_rim() {
        let quad = Quad::new(Vec2::ZERO, Vec2::ONE, Vec4::ONE).shadowed(1.25);
        assert_eq!(quad.fx, [1.25, 0.0, 0.0, MODE_SHADOW]);
        assert!(quad.outline[3] > 0.0);
        assert!(quad.outline[2] > quad.outline[0]);
    }

    #[test]
    fn the_underworld_sky_uses_its_own_mode_and_the_clock() {
        let quad = Quad::new(Vec2::ZERO, Vec2::ONE, Vec4::ONE).underworld(2.5);
        assert_eq!(quad.fx[0], 2.5);
        assert_eq!(quad.fx[3], MODE_SKY);
    }

    #[test]
    fn the_shine_goes_into_the_second_effect_slot_and_leaves_frost_alone() {
        let quad = Quad::new(Vec2::ZERO, Vec2::ONE, Vec4::ONE).shined(0.4);
        assert_eq!(quad.fx[1], 0.4);
        assert_eq!(quad.fx[0], 0.0);
        assert_eq!(quad.fx[3], 0.0);
    }

    #[test]
    fn frost_goes_into_the_first_effect_slot_and_leaves_the_map_mode_alone() {
        let quad = Quad::new(Vec2::ZERO, Vec2::ONE, Vec4::ONE).frosted(0.7);
        assert_eq!(quad.fx[0], 0.7);
        assert_eq!(quad.fx[3], 0.0);
    }
}
