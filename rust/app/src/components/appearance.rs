use glam::Vec4;

use crate::render::TextureId;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Color(pub(crate) Vec4);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Sprite {
    pub(crate) texture: TextureId,
    pub(crate) uv: [f32; 4],
}

impl Sprite {
    pub(crate) fn whole(texture: TextureId) -> Self {
        Self {
            texture,
            uv: [0.0, 0.0, 1.0, 1.0],
        }
    }

    pub(crate) fn cell(
        texture: TextureId,
        column: usize,
        row: usize,
        columns: usize,
        rows: usize,
    ) -> Self {
        let (columns, rows) = (columns as f32, rows as f32);
        Self {
            texture,
            uv: [
                column as f32 / columns,
                row as f32 / rows,
                (column + 1) as f32 / columns,
                (row + 1) as f32 / rows,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Layer(pub(crate) i32);

#[derive(Debug, Clone)]
pub(crate) struct Text {
    pub(crate) content: String,
    pub(crate) color: Vec4,
}
