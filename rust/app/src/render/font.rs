use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

use glam::{Vec2, Vec4};

use super::{Draw, Gpu, Quad, TextureId};

const ATLAS_SIZE: u32 = 1024;
const OVERSAMPLE: f32 = 1.0;
const CHARSET: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~áčďéěíňóřšťúůýžÁČĎÉĚÍŇÓŘŠŤÚŮÝŽ";

#[derive(Debug, Clone, Copy)]
struct Glyph {
    uv: [f32; 4],
    size: Vec2,
    offset: Vec2,
    advance: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FontSize {
    Size16,
    Size22,
    Size27,
    Size32,
}

impl FontSize {
    pub(crate) const ALL: [Self; 4] = [Self::Size16, Self::Size22, Self::Size27, Self::Size32];

    fn pixels(self) -> f32 {
        match self {
            Self::Size16 => 16.0,
            Self::Size22 => 22.0,
            Self::Size27 => 27.0,
            Self::Size32 => 32.0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Fonts {
    size16: Font,
    size22: Font,
    size27: Font,
    size32: Font,
}

impl Fonts {
    pub(crate) fn get(&self, size: FontSize) -> &Font {
        match size {
            FontSize::Size16 => &self.size16,
            FontSize::Size22 => &self.size22,
            FontSize::Size27 => &self.size27,
            FontSize::Size32 => &self.size32,
        }
    }

    pub(crate) fn load(
        &mut self,
        gpu: &mut Gpu,
        path: &Path,
        size: FontSize,
    ) -> Result<(), Box<dyn Error>> {
        let font = Font::load(gpu, path, size.pixels())?;
        match size {
            FontSize::Size16 => self.size16 = font,
            FontSize::Size22 => self.size22 = font,
            FontSize::Size27 => self.size27 = font,
            FontSize::Size32 => self.size32 = font,
        }
        Ok(())
    }
}

fn line_height_to_em(font: &fontdue::Font, line_height: f32) -> f32 {
    const PROBE: f32 = 100.0;
    match font.horizontal_line_metrics(PROBE) {
        Some(metrics) => line_height * PROBE / (metrics.ascent - metrics.descent),
        None => line_height,
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Font {
    texture: TextureId,
    glyphs: HashMap<char, Glyph>,
    ascent: f32,
    descent: f32,
}

impl Font {
    fn load(gpu: &mut Gpu, path: &Path, pixel_size: f32) -> Result<Self, Box<dyn Error>> {
        let bytes = fs::read(path)?;
        let font = fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default())?;
        let pixel_size = line_height_to_em(&font, pixel_size) * OVERSAMPLE;
        let line = font
            .horizontal_line_metrics(pixel_size)
            .ok_or("the font has no horizontal line metrics")?;

        let mut atlas = vec![0u8; (ATLAS_SIZE * ATLAS_SIZE) as usize];
        let mut glyphs = HashMap::new();
        let (mut x, mut y, mut row_height) = (1u32, 1u32, 0u32);
        for character in CHARSET.chars() {
            let (metrics, bitmap) = font.rasterize(character, pixel_size);
            let (width, height) = (metrics.width as u32, metrics.height as u32);
            if x + width + 1 > ATLAS_SIZE {
                x = 1;
                y += row_height + 1;
                row_height = 0;
            }
            if y + height + 1 > ATLAS_SIZE {
                return Err("the font does not fit into the atlas".into());
            }
            for row in 0..height {
                let from = (row * width) as usize;
                let to = ((y + row) * ATLAS_SIZE + x) as usize;
                atlas[to..to + width as usize]
                    .copy_from_slice(&bitmap[from..from + width as usize]);
            }
            let atlas_size = ATLAS_SIZE as f32;
            glyphs.insert(
                character,
                Glyph {
                    uv: [
                        x as f32 / atlas_size,
                        y as f32 / atlas_size,
                        (x + width) as f32 / atlas_size,
                        (y + height) as f32 / atlas_size,
                    ],
                    size: Vec2::new(width as f32, height as f32) / OVERSAMPLE,
                    offset: Vec2::new(metrics.xmin as f32, -(metrics.ymin as f32 + height as f32))
                        / OVERSAMPLE,
                    advance: metrics.advance_width / OVERSAMPLE,
                },
            );
            x += width + 1;
            row_height = row_height.max(height);
        }

        Ok(Self {
            texture: gpu.upload_alpha_mask(ATLAS_SIZE, ATLAS_SIZE, &atlas),
            glyphs,
            ascent: line.ascent / OVERSAMPLE,
            descent: line.descent / OVERSAMPLE,
        })
    }

    pub(crate) fn line_height(&self) -> f32 {
        self.ascent - self.descent
    }

    pub(crate) fn width(&self, text: &str) -> f32 {
        text.chars()
            .filter_map(|character| self.glyphs.get(&character))
            .map(|glyph| glyph.advance)
            .sum()
    }

    pub(crate) fn draw_centered(&self, text: &str, center: Vec2, color: Vec4) -> Vec<Draw> {
        let baseline = center.y + (self.ascent + self.descent) / 2.0;
        let mut pen = center.x - self.width(text) / 2.0;
        let mut draws = Vec::new();
        for character in text.chars() {
            let Some(glyph) = self.glyphs.get(&character) else {
                continue;
            };
            if glyph.size.x > 0.0 && glyph.size.y > 0.0 {
                let top_left = (Vec2::new(pen, baseline) + glyph.offset).round();
                draws.push(Draw {
                    texture: self.texture,
                    quad: Quad::with_uv(top_left + glyph.size / 2.0, glyph.size, color, glyph.uv),
                });
            }
            pen += glyph.advance;
        }
        draws
    }
}
