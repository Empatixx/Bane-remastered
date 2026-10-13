use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::sync::Arc;

use crate::audio::Audio;
use crate::components::BarKind;
use crate::render::{FontSize, Fonts, Gpu, Pixels, TextureId};
use crate::tilemap::Tileset;

const FONT_FILE: &str = "fonts/PixelFJVerdana12pt.ttf";

/// A texture key. Keys only come from `tex`, so a typo is a compile error, never a white quad.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Tex(&'static str);

impl Tex {
    pub(crate) fn path(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for Tex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl AsRef<Path> for Tex {
    fn as_ref(&self) -> &Path {
        Path::new(self.0)
    }
}

macro_rules! texture_keys {
    ($($name:ident = $path:literal,)*) => {
        $(pub(crate) const $name: Tex = Tex($path);)*
        #[cfg(test)]
        pub(crate) const ALL: &[Tex] = &[$($name),*];
    };
}

pub(crate) mod tex {
    use super::Tex;

    texture_keys! {
        LOGO = "menu/logo.tga",
        LOADING_TITLE = "loading/bane_title.tga",
        LOADING_SPINNER = "loading/loading_image.tga",
        MENU_PLATE = "menu/Menu_plate.tga",
        MENU_PLATE_SMALL = "menu/Menu_plate_small.tga",
        CURSOR = "ingame/cursor.tga",
        TILESET = "ingame/tileset_2.png",
        BULLET = "ingame/bullet.tga",
        RAT = "ingame/rat.tga",
        PLAYER = "ingame/player.tga",
        PLAYER_DODGE = "ingame/player_dodge.tga",
        PET = "ingame/wolf_pet.tga",
        PORTAL = "ingame/portal.tga",
        TORCH = "ingame/torch.tga",
        BONES = "ingame/bones.tga",
        COBWEB = "ingame/cobweb.tga",
        ARMORSTAND = "ingame/armorstand.tga",
        SWORDS = "ingame/swords.tga",
        TASKBOARD = "ingame/taskboard.tga",
        CRAFTINGTABLE = "ingame/craftingtable.tga",
        BOXES = "ingame/boxes.tga",
        CHAINS = "ingame/chains.tga",
        ROCK = "ingame/rock.tga",
        BOOKSHELF = "ingame/bookshelf.tga",
        MAPTABLE = "ingame/maptable.tga",
        POT = "ingame/pot.tga",
        SCIENTIST = "ingame/scientist.tga",
        PATHGATE = "ingame/pathgate.tga",
        BANNER = "ingame/banner.tga",
        TALENT_FRAME = "ingame/talent_frame.tga",
        TALENT_ROOT = "ingame/talent_root.tga",
        TALENT_BOARD = "ingame/talent_board.tga",
        TALENT_SELECT = "ingame/talent_select.tga",
        BOARD_NOTICE = "ingame/board_notice.tga",
        NOTE_PAPER = "ingame/note_paper.tga",
        TALENT_ICON = "ingame/talent_icon.tga",
        UPGRADE_ICONS = "ingame/upgrade_icons.tga",
        MUTATION_ICONS = "ingame/mutation_icons.tga",
        REROLL_DICE = "ingame/reroll_dice.tga",
        ENEMY_ARROW = "ingame/enemy_arrow.tga",
        CASING = "ingame/casing.tga",
        SNAKE_TRAIL = "ingame/snake_trail.tga",
        WEAPON_LIST = "ingame/weapon_list.tga",
        WEAPON_TREE = "ingame/weapon_tree.tga",
        TALENTS_BOOT = "ingame/talents/boot.tga",
        TALENTS_DISCOUNT = "ingame/talents/discount.tga",
        TALENTS_TRAP = "ingame/talents/trap.tga",
        TALENTS_EXTRAGUN = "ingame/talents/extragun.tga",
        TALENTS_TELEPORTATION = "ingame/talents/teleportation.tga",
        TALENTS_HEALTH_CROSS = "ingame/talents/health_cross.tga",
        TALENTS_ARMOR = "ingame/talents/armor.tga",
        TALENTS_ARMORREGEN = "ingame/talents/armorregen.tga",
        TALENTS_HEALTH_CROSS2 = "ingame/talents/health_cross2.tga",
        TALENTS_KEVLAR = "ingame/talents/kevlar.tga",
        TALENTS_BLOODTEAR = "ingame/talents/bloodtear.tga",
        TALENTS_AMMO = "ingame/talents/ammo.tga",
        TALENTS_CRITICALHIT = "ingame/talents/criticalhit.tga",
        TALENTS_BLOODTEAR2 = "ingame/talents/bloodtear2.tga",
        TALENTS_PET = "ingame/talents/pet.tga",
        SKULL = "ingame/skull.tga",
        ACCURACY_ICON = "ingame/accuracy_icon.tga",
        KILLS_ICON = "ingame/kills_icon.tga",
        LOOT_ICON = "ingame/loot_icon.tga",
        FLOOR_ICON = "ingame/floor_icon.tga",
        ARTEFACTHUD = "ingame/artefacthud.tga",
        ARTEFACTS_RINGOFDEATH = "ingame/artefacts/ringofdeath.tga",
        ARTEFACTS_ARMORREFILL = "ingame/artefacts/armorrefill.tga",
        ARTEFACTS_SHIELDHORN = "ingame/artefacts/shieldhorn.tga",
        ARTEFACTS_BERSERKPOT = "ingame/artefacts/berserkpot.tga",
        ARTEFACTS_REVEAL_MAP = "ingame/artefacts/reveal_map.tga",
        ARTEFACTS_MAGICSHIELD = "ingame/artefacts/magicshield.tga",
        ARTEFACTS_BERSERKPOT_PARTICLE = "ingame/artefacts/berserkpot-particle.tga",
        BARREL = "ingame/barrel.tga",
        EXPLOSIVE_BARREL = "ingame/explosive_barrel.tga",
        BARREL_EXPLOSION = "ingame/barrel_explosion.tga",
        POISON_BARREL = "ingame/poison_barrel.tga",
        FROST_BARREL = "ingame/frost_barrel.tga",
        POISON_PUDDLE = "ingame/poison_puddle.tga",
        FROST_BURST = "ingame/frost_burst.tga",
        FLAMETHROWER = "ingame/flamethrower.tga",
        SHOP_PRICE_HUD = "ingame/shop_price_hud.tga",
        SACRIFICE_PRICE_HUD = "ingame/sacrifice_price_hud.tga",
        SHOPKEEPER_SAYBOX = "ingame/shopkeeper_saybox.tga",
        BANECOIN_ANIM = "ingame/banecoin_anim.tga",
        AMMOBOX_PISTOL = "ingame/ammobox_pistol.tga",
        AMMOBOX_SHOTGUN = "ingame/ammobox_shotgun.tga",
        AMMOBOX_UNIVERSAL = "ingame/ammobox_universal.tga",
        HEALING_POT = "ingame/healing_pot.tga",
        LARGE_HEALING_POT = "ingame/large_healing_pot.tga",
        GUNS_SHOTGUN = "ingame/guns/shotgun.tga",
        GUNS_LUGER = "ingame/guns/luger.tga",
        GUNS_REVOLVER = "ingame/guns/revolver.tga",
        GUNS_M4 = "ingame/guns/M4.tga",
        GUNS_UZI = "ingame/guns/uzi.tga",
        GUNS_THOMPSON = "ingame/guns/thompson.tga",
        SHOTGUN_AMMO_HUD = "ingame/shotgun_ammo_hud.tga",
        SPIDER = "ingame/spider.tga",
        KINGSLIME = "ingame/kingslime.tga",
        COBWEB_PROJECTILE = "ingame/cobweb_projectile.tga",
        PURPLE_SLIMEBULLET = "ingame/purple_slimebullet.tga",
        LADDER = "ingame/ladder.tga",
        SNAKE = "ingame/snake.tga",
        BAT = "ingame/bat.tga",
        GHOST = "ingame/ghost.tga",
        DEMONEYE = "ingame/demoneye.tga",
        SPIDERLING = "ingame/spiderling.tga",
        SLIME = "ingame/slime.tga",
        REDSLIME = "ingame/redslime.tga",
        GREEN_SPIDERLING = "ingame/green_spiderling.tga",
        PURPLE_SPIDERLING = "ingame/purple_spiderling.tga",
        EYE_WATCHER = "ingame/eye_watcher.tga",
        EYE_WATCHER_PROJECTILE = "ingame/eye_watcher_projectile.tga",
        GREEN_SLIMEBULLET = "ingame/green_slimebullet.tga",
        RED_SLIMEBULLET = "ingame/red_slimebullet.tga",
        SHOPKEEPER = "ingame/shopkeeper.tga",
        SHOPTABLE = "ingame/shoptable.tga",
        SLOTMACHINE = "ingame/slotmachine.tga",
        CARPET_TILEMAP = "ingame/carpet_tilemap.tga",
        CHEST = "ingame/chest.tga",
        BRICKS = "ingame/bricks.tga",
        ANGEL_STATUE = "ingame/angel_statue.tga",
        CANDLESTICK = "ingame/candlestick.tga",
        PENTAGRAM = "ingame/pentagram.tga",
        LAYING_BOOK = "ingame/laying_book.tga",
        KEYBOARD_Q = "ingame/keyboard/q.tga",
        KEYBOARD_E = "ingame/keyboard/e.tga",
        KEYBOARD_F = "ingame/keyboard/f.tga",
        KEYBOARD_1 = "ingame/keyboard/1.tga",
        KEYBOARD_2 = "ingame/keyboard/2.tga",
        KEYBOARD_W = "ingame/keyboard/w.tga",
        KEYBOARD_A = "ingame/keyboard/a.tga",
        KEYBOARD_S = "ingame/keyboard/s.tga",
        KEYBOARD_D = "ingame/keyboard/d.tga",
        SPIKE = "ingame/spike.tga",
        MINIMAP_ICONS = "ingame/minimap_icons.tga",
        MINIMAP_LAYOUT = "ingame/minimap_layout.tga",
        GAZEWIND_PARTICLE = "ingame/gazewind_particle.tga",
        WEAPONSLOT = "ingame/weaponslot.tga",
        GUNS_PISTOL = "ingame/guns/pistol.tga",
        AMMO_HUD = "ingame/ammo_hud.tga",
        BANECOIN = "ingame/banecoin.tga",
        GREEN_BANECOIN = "ingame/green_banecoin.tga",
        CROSSHAIR = "ingame/crosshair.tga",
        RELOAD_CURSOR = "ingame/reload_cursor.tga",
        PAUSEBG = "ingame/pausebg.tga",
        MENU_SETTINGS_BACKGROUND = "menu/settings_background.tga",
        MENU_SETTINGS_MENUBAR = "menu/settings_menubar.tga",
        MENU_CHECKBOX = "menu/checkbox.tga",
        MENU_SLIDER = "menu/slider.tga",
        MENU_SLIDER_BUTTON = "menu/slider-button.tga",
        MENU_BUTTON_ARROW = "menu/button_arrow.tga",
        MENU_BUTTON_ARROW_ROTATED = "menu/button_arrow_rotated.tga",
        HEALTH_BAR = "ingame/healthBar.tga",
        ARMOR_BAR = "ingame/armorBar.tga",
        BOSSHEALTHBAR = "ingame/bosshealthbar.tga",
    }
}

pub(crate) use tex::{
    BULLET, LOADING_SPINNER, LOADING_TITLE, LOGO, MENU_PLATE, PET, PLAYER, PLAYER_DODGE, TILESET,
};

const EARLY_TEXTURES: &[Tex] = &[tex::LOGO, tex::LOADING_TITLE, tex::LOADING_SPINNER];

const GAME_TEXTURES: &[Tex] = &[
    tex::MENU_PLATE,
    tex::MENU_PLATE_SMALL,
    tex::CURSOR,
    tex::TILESET,
    tex::BULLET,
    tex::RAT,
    tex::PLAYER,
    tex::PLAYER_DODGE,
    tex::PET,
    tex::PORTAL,
    tex::TORCH,
    tex::BONES,
    tex::COBWEB,
    tex::ARMORSTAND,
    tex::SWORDS,
    tex::TASKBOARD,
    tex::CRAFTINGTABLE,
    tex::BOXES,
    tex::CHAINS,
    tex::ROCK,
    tex::BOOKSHELF,
    tex::MAPTABLE,
    tex::POT,
    tex::SCIENTIST,
    tex::PATHGATE,
    tex::BANNER,
    tex::TALENT_FRAME,
    tex::TALENT_ROOT,
    tex::TALENT_BOARD,
    tex::TALENT_SELECT,
    tex::BOARD_NOTICE,
    tex::NOTE_PAPER,
    tex::TALENT_ICON,
    tex::UPGRADE_ICONS,
    tex::MUTATION_ICONS,
    tex::REROLL_DICE,
    tex::ENEMY_ARROW,
    tex::CASING,
    tex::SNAKE_TRAIL,
    tex::WEAPON_LIST,
    tex::WEAPON_TREE,
    tex::TALENTS_BOOT,
    tex::TALENTS_DISCOUNT,
    tex::TALENTS_TRAP,
    tex::TALENTS_EXTRAGUN,
    tex::TALENTS_TELEPORTATION,
    tex::TALENTS_HEALTH_CROSS,
    tex::TALENTS_ARMOR,
    tex::TALENTS_ARMORREGEN,
    tex::TALENTS_HEALTH_CROSS2,
    tex::TALENTS_KEVLAR,
    tex::TALENTS_BLOODTEAR,
    tex::TALENTS_AMMO,
    tex::TALENTS_CRITICALHIT,
    tex::TALENTS_BLOODTEAR2,
    tex::TALENTS_PET,
    tex::SKULL,
    tex::ACCURACY_ICON,
    tex::KILLS_ICON,
    tex::LOOT_ICON,
    tex::FLOOR_ICON,
    tex::ARTEFACTHUD,
    tex::ARTEFACTS_RINGOFDEATH,
    tex::ARTEFACTS_ARMORREFILL,
    tex::ARTEFACTS_SHIELDHORN,
    tex::ARTEFACTS_BERSERKPOT,
    tex::ARTEFACTS_REVEAL_MAP,
    tex::ARTEFACTS_MAGICSHIELD,
    tex::ARTEFACTS_BERSERKPOT_PARTICLE,
    tex::BARREL,
    tex::EXPLOSIVE_BARREL,
    tex::BARREL_EXPLOSION,
    tex::POISON_BARREL,
    tex::FROST_BARREL,
    tex::POISON_PUDDLE,
    tex::FROST_BURST,
    tex::FLAMETHROWER,
    tex::SHOP_PRICE_HUD,
    tex::SACRIFICE_PRICE_HUD,
    tex::SHOPKEEPER_SAYBOX,
    tex::BANECOIN_ANIM,
    tex::AMMOBOX_PISTOL,
    tex::AMMOBOX_SHOTGUN,
    tex::AMMOBOX_UNIVERSAL,
    tex::HEALING_POT,
    tex::LARGE_HEALING_POT,
    tex::GUNS_SHOTGUN,
    tex::GUNS_LUGER,
    tex::GUNS_REVOLVER,
    tex::GUNS_M4,
    tex::GUNS_UZI,
    tex::GUNS_THOMPSON,
    tex::SHOTGUN_AMMO_HUD,
    tex::SPIDER,
    tex::KINGSLIME,
    tex::COBWEB_PROJECTILE,
    tex::PURPLE_SLIMEBULLET,
    tex::LADDER,
    tex::SNAKE,
    tex::BAT,
    tex::GHOST,
    tex::DEMONEYE,
    tex::SPIDERLING,
    tex::SLIME,
    tex::REDSLIME,
    tex::GREEN_SPIDERLING,
    tex::PURPLE_SPIDERLING,
    tex::EYE_WATCHER,
    tex::EYE_WATCHER_PROJECTILE,
    tex::GREEN_SLIMEBULLET,
    tex::RED_SLIMEBULLET,
    tex::SHOPKEEPER,
    tex::SHOPTABLE,
    tex::SLOTMACHINE,
    tex::CARPET_TILEMAP,
    tex::CHEST,
    tex::BRICKS,
    tex::ANGEL_STATUE,
    tex::CANDLESTICK,
    tex::PENTAGRAM,
    tex::LAYING_BOOK,
    tex::KEYBOARD_Q,
    tex::KEYBOARD_E,
    tex::KEYBOARD_F,
    tex::KEYBOARD_1,
    tex::KEYBOARD_2,
    tex::KEYBOARD_W,
    tex::KEYBOARD_A,
    tex::KEYBOARD_S,
    tex::KEYBOARD_D,
    tex::SPIKE,
    tex::MINIMAP_ICONS,
    tex::MINIMAP_LAYOUT,
    tex::GAZEWIND_PARTICLE,
    tex::WEAPONSLOT,
    tex::GUNS_PISTOL,
    tex::AMMO_HUD,
    tex::BANECOIN,
    tex::GREEN_BANECOIN,
    tex::CROSSHAIR,
    tex::RELOAD_CURSOR,
    tex::PAUSEBG,
    tex::MENU_SETTINGS_BACKGROUND,
    tex::MENU_SETTINGS_MENUBAR,
    tex::MENU_CHECKBOX,
    tex::MENU_SLIDER,
    tex::MENU_SLIDER_BUTTON,
    tex::MENU_BUTTON_ARROW,
    tex::MENU_BUTTON_ARROW_ROTATED,
];

#[derive(Debug, Clone, Default)]
pub(crate) struct Textures {
    ids: HashMap<&'static str, TextureId>,
}

impl Textures {
    pub(crate) fn get(&self, key: Tex) -> TextureId {
        self.ids.get(key.path()).copied().unwrap_or_else(|| {
            tracing::warn!(name = key.path(), "texture was not loaded");
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
    Texture(Tex),
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
        for key in EARLY_TEXTURES {
            load_texture(gpu, &mut assets.textures, *key)?;
        }
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
            Step::Texture(key) => {
                let pixels = Pixels::load(&root().join("textures").join(key.path()))?;
                self.decoded.push((key.path(), pixels));
            }
            Step::Bar(kind) => {
                let spec = kind.spec();
                let path = spec.texture.path();
                let pixels =
                    Pixels::load_with_hole(&root().join("textures").join(path), spec.hole)?;
                self.decoded.push((path, pixels));
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

fn load_texture(gpu: &mut Gpu, textures: &mut Textures, key: Tex) -> Result<(), Box<dyn Error>> {
    let id = gpu.load_texture(&root().join("textures").join(key.path()))?;
    textures.insert(key.path(), id);
    Ok(())
}

fn root() -> &'static Path {
    crate::storage::assets_root()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_texture_key_is_loaded_by_one_of_the_lists() {
        let bars: Vec<Tex> = BarKind::ALL
            .iter()
            .map(|kind| kind.spec().texture)
            .collect();
        for key in tex::ALL {
            let loaded =
                EARLY_TEXTURES.contains(key) || GAME_TEXTURES.contains(key) || bars.contains(key);
            assert!(loaded, "{key} has a key but is never loaded");
        }
        for key in EARLY_TEXTURES.iter().chain(&bars) {
            assert!(root().join("textures").join(key).is_file(), "missing {key}");
        }
    }

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
