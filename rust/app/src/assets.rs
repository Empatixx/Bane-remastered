use std::collections::VecDeque;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::render::{FontSize, Fonts, Gpu, TextureId};
use crate::tilemap::Tileset;

const FONT_FILE: &str = "fonts/PixelFJVerdana12pt.ttf";

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Textures {
    pub(crate) loading_background: TextureId,
    pub(crate) loading_spinner: TextureId,
    pub(crate) menu_background: TextureId,
    pub(crate) logo: TextureId,
    pub(crate) button: TextureId,
    pub(crate) cursor: TextureId,
    pub(crate) tileset: TextureId,
    pub(crate) bullet: TextureId,
    pub(crate) rat: TextureId,
}

#[derive(Debug, Clone)]
pub(crate) struct Assets {
    pub(crate) textures: Textures,
    pub(crate) tileset: Arc<Tileset>,
    pub(crate) fonts: Fonts,
    loaded: bool,
}

impl Assets {
    pub(crate) fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            textures: Textures::default(),
            tileset: Arc::new(Tileset::dungeon()?),
            fonts: Fonts::default(),
            loaded: false,
        })
    }

    pub(crate) fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub(crate) fn mark_loaded(&mut self) {
        self.loaded = true;
    }
}

#[derive(Debug, Clone, Copy)]
enum Step {
    MenuBackground,
    Logo,
    Button,
    Cursor,
    Tileset,
    Bullet,
    Rat,
    Font(FontSize),
}

#[derive(Debug)]
pub(crate) struct Loader {
    pending: VecDeque<Step>,
}

impl Loader {
    pub(crate) fn start(gpu: &mut Gpu, assets: &mut Assets) -> Result<Self, Box<dyn Error>> {
        let root = root();
        assets.textures.loading_background =
            gpu.load_texture(&root.join("textures/loading/loading.tga"))?;
        assets.textures.loading_spinner =
            gpu.load_texture(&root.join("textures/loading/loading_image.tga"))?;
        let mut pending = VecDeque::from([
            Step::MenuBackground,
            Step::Logo,
            Step::Button,
            Step::Cursor,
            Step::Tileset,
            Step::Bullet,
            Step::Rat,
        ]);
        pending.extend(FontSize::ALL.map(Step::Font));
        Ok(Self { pending })
    }

    pub(crate) fn is_done(&self) -> bool {
        self.pending.is_empty()
    }

    pub(crate) fn step(
        &mut self,
        gpu: &mut Gpu,
        assets: &mut Assets,
    ) -> Result<(), Box<dyn Error>> {
        let Some(step) = self.pending.pop_front() else {
            return Ok(());
        };
        let root = root();
        let mut texture = |path: &str| gpu.load_texture(&root.join(path));
        match step {
            Step::MenuBackground => {
                assets.textures.menu_background = texture("textures/menu/bg.png")?
            }
            Step::Logo => assets.textures.logo = texture("textures/menu/logo.tga")?,
            Step::Button => assets.textures.button = texture("textures/menu/Menu_bar2.tga")?,
            Step::Cursor => assets.textures.cursor = texture("textures/ingame/cursor.tga")?,
            Step::Tileset => assets.textures.tileset = texture("textures/ingame/tileset_2.png")?,
            Step::Bullet => assets.textures.bullet = texture("textures/ingame/bullet.tga")?,
            Step::Rat => assets.textures.rat = texture("textures/ingame/rat.tga")?,
            Step::Font(size) => assets.fonts.load(gpu, &root.join(FONT_FILE), size)?,
        }
        if self.pending.is_empty() {
            assets.mark_loaded();
        }
        Ok(())
    }
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets")
}
