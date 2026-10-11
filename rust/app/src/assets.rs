use std::error::Error;
use std::path::{Path, PathBuf};

use crate::render::{Gpu, TextureId};

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Textures {
    pub(crate) menu_background: TextureId,
    pub(crate) logo: TextureId,
}

impl Textures {
    pub(crate) fn load(gpu: &mut Gpu) -> Result<Self, Box<dyn Error>> {
        let root = root();
        Ok(Self {
            menu_background: gpu.load_texture(&root.join("textures/menu/bg.png"))?,
            logo: gpu.load_texture(&root.join("textures/menu/logo.tga"))?,
        })
    }
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets")
}
