use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

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

/// The walls that block light, one byte per tile: 255 for a wall, 0 for open floor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Occluders {
    id: u64,
    columns: u32,
    rows: u32,
    tile_size: u32,
    cells: Vec<u8>,
}

impl Occluders {
    pub(crate) fn new(
        columns: usize,
        rows: usize,
        tile_size: f32,
        wall: impl Fn(usize, usize) -> bool,
    ) -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        let cells = (0..rows)
            .flat_map(|row| (0..columns).map(move |column| (column, row)))
            .map(|(column, row)| if wall(column, row) { u8::MAX } else { 0 })
            .collect();
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            columns: columns as u32,
            rows: rows as u32,
            tile_size: tile_size as u32,
            cells,
        }
    }

    pub(super) fn id(&self) -> u64 {
        self.id
    }

    pub(super) fn size(&self) -> (u32, u32) {
        (self.columns, self.rows)
    }

    pub(super) fn cells(&self) -> &[u8] {
        &self.cells
    }
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
    grid_columns: u32,
    grid_rows: u32,
    tile_size: f32,
    lights: [GpuLight; MAX_LIGHTS],
}

impl Globals {
    pub(super) fn new(camera: Vec2, lights: &[LightSource], occluders: Option<&Occluders>) -> Self {
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
            grid_columns: occluders.map_or(0, |grid| grid.columns),
            grid_rows: occluders.map_or(0, |grid| grid.rows),
            tile_size: occluders.map_or(1.0, |grid| grid.tile_size as f32),
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
    pub(crate) occluders: Option<Arc<Occluders>>,
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
        let globals = Globals::new(Vec2::ZERO, &lights, None);
        assert_eq!(globals.light_count as usize, MAX_LIGHTS);
    }

    #[test]
    fn occluders_store_walls_row_by_row_and_each_grid_gets_its_own_id() {
        let grid = Occluders::new(3, 2, 64.0, |column, row| column == 0 || row == 1);
        assert_eq!(grid.size(), (3, 2));
        assert_eq!(grid.cells(), &[255, 0, 0, 255, 255, 255]);
        assert_ne!(grid.id(), Occluders::new(3, 2, 64.0, |_, _| false).id());

        let globals = Globals::new(Vec2::ZERO, &[], Some(&grid));
        assert_eq!((globals.grid_columns, globals.grid_rows), (3, 2));
        assert_eq!(globals.tile_size, 64.0);
    }
}
