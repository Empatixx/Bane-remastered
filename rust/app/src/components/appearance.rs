use glam::Vec4;

use crate::render::{Align, Anchor, FontSize, TextureId};

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
        Self {
            texture,
            uv: Self::cell_uv(column, row, columns, rows),
        }
    }

    pub(crate) fn flipped(uv: [f32; 4]) -> [f32; 4] {
        [uv[2], uv[1], uv[0], uv[3]]
    }

    pub(crate) fn cell_uv(column: usize, row: usize, columns: usize, rows: usize) -> [f32; 4] {
        let (columns, rows) = (columns as f32, rows as f32);
        [
            column as f32 / columns,
            row as f32 / rows,
            (column + 1) as f32 / columns,
            (row + 1) as f32 / rows,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Layer(pub(crate) i32);

#[derive(Debug, Clone)]
pub(crate) struct Text {
    pub(crate) content: String,
    pub(crate) color: Vec4,
    pub(crate) size: FontSize,
    pub(crate) align: Align,
    pub(crate) anchor: Anchor,
}

impl Text {
    pub(crate) fn centered(content: &str, color: Vec4, size: FontSize) -> Self {
        Self {
            content: content.to_owned(),
            color,
            size,
            align: Align::Center,
            anchor: Anchor::Middle,
        }
    }
}
