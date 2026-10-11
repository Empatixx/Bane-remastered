use std::error::Error;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct TextureId(usize);

impl TextureId {
    pub(crate) const WHITE: Self = Self(0);

    pub(super) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(super) fn index(self) -> usize {
        self.0
    }
}

pub(super) struct Pixels {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) rgba: Vec<u8>,
}

impl Pixels {
    pub(super) fn white() -> Self {
        Self {
            width: 1,
            height: 1,
            rgba: vec![255; 4],
        }
    }

    pub(super) fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let image = image::open(path)?.into_rgba8();
        Ok(Self {
            width: image.width(),
            height: image.height(),
            rgba: image.into_raw(),
        })
    }

    pub(super) fn clear_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        for row in y..(y + height).min(self.height) {
            for column in x..(x + width).min(self.width) {
                let index = ((row * self.width + column) * 4 + 3) as usize;
                self.rgba[index] = 0;
            }
        }
    }
}
