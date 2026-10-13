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

pub(crate) struct Pixels {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) rgba: Vec<u8>,
}

impl std::fmt::Debug for Pixels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pixels")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl Pixels {
    #[cfg(test)]
    pub(crate) fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub(super) fn white() -> Self {
        Self {
            width: 1,
            height: 1,
            rgba: vec![255; 4],
        }
    }

    pub(crate) fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let image = image::ImageReader::open(path)?
            .with_guessed_format()?
            .decode()?
            .into_rgba8();
        Ok(Self {
            width: image.width(),
            height: image.height(),
            rgba: image.into_raw(),
        })
    }

    pub(crate) fn load_with_hole(path: &Path, hole: [u32; 4]) -> Result<Self, Box<dyn Error>> {
        let mut pixels = Self::load(path)?;
        pixels.clear_rect(hole[0], hole[1], hole[2], hole[3]);
        Ok(pixels)
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
