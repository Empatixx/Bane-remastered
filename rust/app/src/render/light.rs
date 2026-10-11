use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};

use super::{Layers, PostParams};
use crate::viewport::TARGET;

pub(crate) const MAX_LIGHTS: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LightSource {
    pub(crate) position: Vec2,
    pub(crate) color: Vec3,
    pub(crate) radius: f32,
    pub(crate) intensity: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct GpuLight {
    position: [f32; 2],
    radius: f32,
    intensity: f32,
    color: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub(super) struct Globals {
    size: [f32; 2],
    camera: [f32; 2],
    light_count: u32,
    padding: [u32; 3],
    lights: [GpuLight; MAX_LIGHTS],
}

impl Globals {
    pub(super) fn new(camera: Vec2, lights: &[LightSource]) -> Self {
        let mut gpu_lights = [GpuLight::zeroed(); MAX_LIGHTS];
        let count = lights.len().min(MAX_LIGHTS);
        for (slot, light) in gpu_lights.iter_mut().zip(lights) {
            *slot = GpuLight {
                position: light.position.to_array(),
                radius: light.radius,
                intensity: light.intensity,
                color: [light.color.x, light.color.y, light.color.z, 1.0],
            };
        }
        Self {
            size: TARGET.to_array(),
            camera: camera.to_array(),
            light_count: count as u32,
            padding: [0; 3],
            lights: gpu_lights,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Frame {
    pub(crate) layers: Layers,
    pub(crate) post: PostParams,
    pub(crate) camera: Vec2,
    pub(crate) lights: Vec<LightSource>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_uniform_matches_the_wgsl_layout() {
        assert_eq!(size_of::<GpuLight>(), 32);
        assert_eq!(size_of::<Globals>(), 32 + 32 * MAX_LIGHTS);
    }

    #[test]
    fn extra_lights_are_ignored_instead_of_overflowing() {
        let lights = vec![
            LightSource {
                position: Vec2::ZERO,
                color: Vec3::ONE,
                radius: 10.0,
                intensity: 1.0,
            };
            MAX_LIGHTS + 5
        ];
        let globals = Globals::new(Vec2::ZERO, &lights);
        assert_eq!(globals.light_count as usize, MAX_LIGHTS);
    }
}
