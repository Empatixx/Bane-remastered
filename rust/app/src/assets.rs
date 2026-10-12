use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::audio::Audio;
use crate::components::BarKind;
use crate::render::{FontSize, Fonts, Gpu, Pixels, TextureId};
use crate::tilemap::Tileset;

const FONT_FILE: &str = "fonts/PixelFJVerdana12pt.ttf";

pub(crate) const LOADING_BACKGROUND: &str = "loading/loading.tga";
pub(crate) const LOADING_SPINNER: &str = "loading/loading_image.tga";
pub(crate) const LOGO: &str = "menu/logo.tga";
pub(crate) const BUTTON: &str = "menu/Menu_bar2.tga";
pub(crate) const MENU_PLATE: &str = "menu/Menu_plate.tga";
pub(crate) const CURSOR: &str = "ingame/cursor.tga";
pub(crate) const TILESET: &str = "ingame/tileset_2.png";
pub(crate) const BULLET: &str = "ingame/bullet.tga";
pub(crate) const RAT: &str = "ingame/rat.tga";
pub(crate) const PLAYER: &str = "ingame/player.tga";
pub(crate) const PLAYER_DODGE: &str = "ingame/player_dodge.tga";
pub(crate) const PET: &str = "ingame/wolf_pet.tga";

const GAME_TEXTURES: &[&str] = &[
    LOGO,
    BUTTON,
    MENU_PLATE,
    CURSOR,
    TILESET,
    BULLET,
    RAT,
    PLAYER,
    PLAYER_DODGE,
    PET,
    "ingame/portal.tga",
    "ingame/torch.tga",
    "ingame/bones.tga",
    "ingame/cobweb.tga",
    "ingame/armorstand.tga",
    "ingame/swords.tga",
    "ingame/taskboard.tga",
    "ingame/craftingtable.tga",
    "ingame/boxes.tga",
    "ingame/chains.tga",
    "ingame/rock.tga",
    "ingame/bookshelf.tga",
    "ingame/maptable.tga",
    "ingame/pot.tga",
    "ingame/scientist.tga",
    "ingame/pathgate.tga",
    "ingame/banner.tga",
    "ingame/talent_frame.tga",
    "ingame/talent_path.tga",
    "ingame/utility_icon.tga",
    "ingame/offense_icon.tga",
    "ingame/defense_icon.tga",
    "ingame/talents/boot.tga",
    "ingame/talents/discount.tga",
    "ingame/talents/trap.tga",
    "ingame/talents/extragun.tga",
    "ingame/talents/teleportation.tga",
    "ingame/talents/health_cross.tga",
    "ingame/talents/armor.tga",
    "ingame/talents/armorregen.tga",
    "ingame/talents/health_cross2.tga",
    "ingame/talents/kevlar.tga",
    "ingame/talents/bloodtear.tga",
    "ingame/talents/ammo.tga",
    "ingame/talents/criticalhit.tga",
    "ingame/talents/bloodtear2.tga",
    "ingame/talents/pet.tga",
    "ingame/skull.tga",
    "ingame/accuracy_icon.tga",
    "ingame/kills_icon.tga",
    "ingame/loot_icon.tga",
    "ingame/floor_icon.tga",
    "ingame/artefacthud.tga",
    "ingame/artefacts/ringofdeath.tga",
    "ingame/artefacts/armorrefill.tga",
    "ingame/artefacts/shieldhorn.tga",
    "ingame/artefacts/berserkpot.tga",
    "ingame/artefacts/reveal_map.tga",
    "ingame/artefacts/magicshield.tga",
    "ingame/artefacts/berserkpot-particle.tga",
    "ingame/barrel.tga",
    "ingame/explosive_barrel.tga",
    "ingame/barrel_explosion.tga",
    "ingame/poison_barrel.tga",
    "ingame/frost_barrel.tga",
    "ingame/poison_puddle.tga",
    "ingame/frost_burst.tga",
    "ingame/flamethrower.tga",
    "ingame/shop_price_hud.tga",
    "ingame/sacrifice_price_hud.tga",
    "ingame/shopkeeper_saybox.tga",
    "ingame/banecoin_anim.tga",
    "ingame/ammobox_pistol.tga",
    "ingame/ammobox_shotgun.tga",
    "ingame/ammobox_universal.tga",
    "ingame/healing_pot.tga",
    "ingame/large_healing_pot.tga",
    "ingame/guns/shotgun.tga",
    "ingame/guns/luger.tga",
    "ingame/guns/revolver.tga",
    "ingame/guns/M4.tga",
    "ingame/guns/uzi.tga",
    "ingame/guns/thompson.tga",
    "ingame/shotgun_ammo_hud.tga",
    "ingame/spider.tga",
    "ingame/kingslime.tga",
    "ingame/cobweb_projectile.tga",
    "ingame/purple_slimebullet.tga",
    "ingame/ladder.tga",
    "ingame/snake.tga",
    "ingame/bat.tga",
    "ingame/ghost.tga",
    "ingame/demoneye.tga",
    "ingame/spiderling.tga",
    "ingame/slime.tga",
    "ingame/redslime.tga",
    "ingame/green_spiderling.tga",
    "ingame/purple_spiderling.tga",
    "ingame/eye_watcher.tga",
    "ingame/eye_watcher_projectile.tga",
    "ingame/green_slimebullet.tga",
    "ingame/red_slimebullet.tga",
    "ingame/shopkeeper.tga",
    "ingame/shoptable.tga",
    "ingame/slotmachine.tga",
    "ingame/carpet_tilemap.tga",
    "ingame/chest.tga",
    "ingame/bricks.tga",
    "ingame/angel_statue.tga",
    "ingame/candlestick.tga",
    "ingame/pentagram.tga",
    "ingame/laying_book.tga",
    "ingame/keyboard/q.tga",
    "ingame/keyboard/e.tga",
    "ingame/keyboard/f.tga",
    "ingame/keyboard/1.tga",
    "ingame/keyboard/2.tga",
    "ingame/keyboard/w.tga",
    "ingame/keyboard/a.tga",
    "ingame/keyboard/s.tga",
    "ingame/keyboard/d.tga",
    "ingame/spike.tga",
    "ingame/minimap_icons.tga",
    "ingame/minimap_layout.tga",
    "ingame/gazewind_particle.tga",
    "ingame/weaponslot.tga",
    "ingame/guns/pistol.tga",
    "ingame/ammo_hud.tga",
    "ingame/banecoin.tga",
    "ingame/green_banecoin.tga",
    "ingame/crosshair.tga",
    "ingame/reload_cursor.tga",
    "ingame/pausebg.tga",
    "menu/settings_background.tga",
    "menu/settings_menubar.tga",
    "menu/checkbox.tga",
    "menu/slider.tga",
    "menu/slider-button.tga",
    "menu/button_arrow.tga",
    "menu/button_arrow_rotated.tga",
];

#[derive(Debug, Clone, Default)]
pub(crate) struct Textures {
    ids: HashMap<&'static str, TextureId>,
}

impl Textures {
    pub(crate) fn get(&self, name: &str) -> TextureId {
        self.ids.get(name).copied().unwrap_or_else(|| {
            tracing::warn!(name, "texture was not loaded");
            TextureId::WHITE
        })
    }

    fn insert(&mut self, name: &'static str, id: TextureId) {
        self.ids.insert(name, id);
    }
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
    Texture(&'static str),
    Bar(BarKind),
    Atlas,
    Sound(&'static str),
    Font(FontSize),
}

#[derive(Debug)]
pub(crate) struct Loader {
    pending: VecDeque<Step>,
    decoded: Vec<(&'static str, Pixels)>,
}

impl Loader {
    pub(crate) fn start(gpu: &mut Gpu, assets: &mut Assets) -> Result<Self, Box<dyn Error>> {
        load_texture(gpu, &mut assets.textures, LOADING_BACKGROUND)?;
        load_texture(gpu, &mut assets.textures, LOADING_SPINNER)?;
        let mut pending: VecDeque<Step> =
            GAME_TEXTURES.iter().copied().map(Step::Texture).collect();
        pending.extend(BarKind::ALL.map(Step::Bar));
        pending.push_back(Step::Atlas);
        pending.extend(Audio::files().into_iter().map(Step::Sound));
        pending.extend(FontSize::ALL.map(Step::Font));
        Ok(Self {
            pending,
            decoded: Vec::new(),
        })
    }

    pub(crate) fn is_done(&self) -> bool {
        self.pending.is_empty()
    }

    pub(crate) fn step(
        &mut self,
        gpu: &mut Gpu,
        assets: &mut Assets,
        audio: &mut Audio,
    ) -> Result<(), Box<dyn Error>> {
        let Some(step) = self.pending.pop_front() else {
            return Ok(());
        };
        match step {
            Step::Texture(name) => {
                let pixels = Pixels::load(&root().join("textures").join(name))?;
                self.decoded.push((name, pixels));
            }
            Step::Bar(kind) => {
                let spec = kind.spec();
                let pixels =
                    Pixels::load_with_hole(&root().join("textures").join(spec.texture), spec.hole)?;
                self.decoded.push((spec.texture, pixels));
            }
            Step::Atlas => {
                for (name, id) in gpu.upload_atlas(std::mem::take(&mut self.decoded))? {
                    assets.textures.insert(name, id);
                }
            }
            Step::Sound(file) => audio.preload(file),
            Step::Font(size) => assets.fonts.load(gpu, &root().join(FONT_FILE), size)?,
        }
        if self.pending.is_empty() {
            assets.mark_loaded();
        }
        Ok(())
    }
}

fn load_texture(
    gpu: &mut Gpu,
    textures: &mut Textures,
    name: &'static str,
) -> Result<(), Box<dyn Error>> {
    let id = gpu.load_texture(&root().join("textures").join(name))?;
    textures.insert(name, id);
    Ok(())
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_listed_texture_exists_and_decodes() {
        for name in GAME_TEXTURES {
            let path = root().join("textures").join(name);
            let decoded = crate::render::Pixels::load(&path);
            assert!(decoded.is_ok(), "{name}: {:?}", decoded.err());
        }
    }

    #[test]
    fn every_game_texture_fits_into_one_atlas_without_overlaps() {
        let mut sizes = Vec::new();
        for name in GAME_TEXTURES {
            let pixels = crate::render::Pixels::load(&root().join("textures").join(name))
                .expect("a decodable texture");
            sizes.push(pixels.dimensions());
        }
        let packed = crate::render::pack_atlas(&sizes, 16384).expect("the textures fit");
        assert!(packed.0 <= 8192, "{}", packed.0);
        for (index, (width, height)) in sizes.iter().enumerate() {
            let (x, y) = packed.1[index];
            for other in 0..index {
                let (ox, oy) = packed.1[other];
                let (ow, oh) = sizes[other];
                let apart = x + width <= ox || ox + ow <= x || y + height <= oy || oy + oh <= y;
                assert!(
                    apart,
                    "{} overlaps {}",
                    GAME_TEXTURES[index], GAME_TEXTURES[other]
                );
            }
        }
    }

    #[test]
    fn texture_names_are_unique() {
        let mut seen = std::collections::HashSet::new();
        for name in GAME_TEXTURES {
            assert!(seen.insert(name), "{name} is listed twice");
        }
    }
}
