use glam::Vec4;

use crate::render::{Align, Anchor, FontSize, TextureId};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Color(pub(crate) Vec4);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Outline(pub(crate) Vec4);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Shine(pub(crate) f32);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Sky(pub(crate) f32);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Liquid {
    pub(crate) kind: f32,
    pub(crate) time: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Sprite {
    pub(crate) texture: TextureId,
    pub(crate) uv: [f32; 4],
}

const CELL_INSET: f32 = 0.0015;

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
            (column as f32 + CELL_INSET) / columns,
            (row as f32 + CELL_INSET) / rows,
            ((column + 1) as f32 - CELL_INSET) / columns,
            ((row + 1) as f32 - CELL_INSET) / rows,
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
    pub(crate) fn new(content: impl Into<String>, color: Vec4, size: FontSize) -> Self {
        Self {
            content: content.into(),
            color,
            size,
            align: Align::Left,
            anchor: Anchor::Baseline,
        }
    }

    pub(crate) fn centered(content: impl Into<String>, color: Vec4, size: FontSize) -> Self {
        Self::new(content, color, size)
            .aligned(Align::Center)
            .anchored(Anchor::Middle)
    }

    pub(crate) fn inked(content: impl Into<String>, color: Vec4, size: FontSize) -> Self {
        Self::new(content, color, size)
            .aligned(Align::Center)
            .anchored(Anchor::Ink)
    }

    pub(crate) fn aligned(self, align: Align) -> Self {
        Self { align, ..self }
    }

    pub(crate) fn anchored(self, anchor: Anchor) -> Self {
        Self { anchor, ..self }
    }
}
