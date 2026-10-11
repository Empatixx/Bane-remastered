use std::error::Error;
use std::path::{Path, PathBuf};

use crate::render::{Font, Gpu, TextureId};

const FONT_SIZE: f32 = 20.0;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Textures {
    pub(crate) menu_background: TextureId,
    pub(crate) logo: TextureId,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Assets {
    pub(crate) textures: Textures,
    pub(crate) font: Font,
}

impl Assets {
    pub(crate) fn load(gpu: &mut Gpu) -> Result<Self, Box<dyn Error>> {
        let root = root();
        Ok(Self {
            textures: Textures {
                menu_background: gpu.load_texture(&root.join("textures/menu/bg.png"))?,
                logo: gpu.load_texture(&root.join("textures/menu/logo.tga"))?,
            },
            font: Font::load(gpu, &root.join("fonts/PixelFJVerdana12pt.ttf"), FONT_SIZE)?,
        })
    }
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets")
}
