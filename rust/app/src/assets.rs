use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::sync::Arc;

use crate::audio::Audio;
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
        TILESET_SEWER = "ingame/tileset_sewer.png",
        TILESET_MOSS = "ingame/tileset_moss.png",
        TILESET_SPIDER = "ingame/tileset_spider.png",
        TILESET_ICE = "ingame/tileset_ice.png",
        TILESET_LAVA = "ingame/tileset_lava.png",
        TILESET_TEMPLE = "ingame/tileset_temple.png",
        BULLET = "ingame/bullet.tga",
        RAT = "ingame/rat.tga",
        EMBER_RAT = "ingame/ember_rat.tga",
        FROST_RAT = "ingame/frost_rat.tga",
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
        GUIDE = "ingame/guide.tga",
        RIFT = "ingame/rift.tga",
        SARCOPHAGUS = "ingame/sarcophagus.tga",
        SARCOPHAGUS_LID = "ingame/sarcophagus_lid.tga",
        PATHGATE = "ingame/pathgate.tga",
        BANNER = "ingame/banner.tga",
        BANNER_MOSSY = "ingame/banner_mossy.tga",
        TALENT_FRAME = "ingame/talent_frame.tga",
        TALENT_ROOT = "ingame/talent_root.tga",
        TALENT_BOARD = "ingame/talent_board.tga",
        TALENT_SELECT = "ingame/talent_select.tga",
        BOARD_NOTICE = "ingame/board_notice.tga",
        NOTE_PAPER = "ingame/note_paper.tga",
        CHALLENGE_PAPER = "ingame/challenge_paper.tga",
        TRAINING_DUMMY = "ingame/training_dummy.tga",
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
        ARTEFACTS_HOURGLASS_OF_ASH = "ingame/artefacts/hourglass_of_ash.tga",
        ARTEFACTS_BLOOD_CHALICE = "ingame/artefacts/blood_chalice.tga",
        ARTEFACTS_PHANTOM_CLOAK = "ingame/artefacts/phantom_cloak.tga",
        ARTEFACTS_GRAVE_LANTERN = "ingame/artefacts/grave_lantern.tga",
        ARTEFACTS_MIDAS_COIN = "ingame/artefacts/midas_coin.tga",
        ARTEFACTS_THUNDER_IDOL = "ingame/artefacts/thunder_idol.tga",
        ARTEFACTS_INFINITY_QUIVER = "ingame/artefacts/infinity_quiver.tga",
        ARTEFACTS_INFINITY_GLINT = "ingame/artefacts/infinity_glint.tga",
        ARTEFACTS_BONE_TURRET = "ingame/artefacts/bone_turret.tga",
        ARTEFACTS_GHOST = "ingame/artefacts/ghost.tga",
        ARTEFACTS_TURRET = "ingame/artefacts/turret.tga",
        ARTEFACTS_BONE_SHARD = "ingame/artefacts/bone_shard.tga",
        ARTEFACTS_LIGHTNING_BOLT = "ingame/artefacts/lightning_bolt.tga",
        ARTEFACTS_LIGHTNING_BURST = "ingame/artefacts/lightning_burst.tga",
        ARTEFACTS_LIGHTNING_SPARK = "ingame/artefacts/lightning_spark.tga",
        ARTEFACTS_MAGICSHIELD = "ingame/artefacts/magicshield.tga",
        ARTEFACTS_BERSERKPOT_PARTICLE = "ingame/artefacts/berserkpot-particle.tga",
        ARTEFACTS_BLOOD_DROPS = "ingame/artefacts/blood_drops.tga",
        ARTEFACTS_MIDAS_SPARKLE = "ingame/artefacts/midas_sparkle.tga",
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
        BANECOIN_ANIM = "ingame/banecoin_anim.tga",
        AMMOBOX_PISTOL = "ingame/ammobox_pistol.tga",
        AMMOBOX_SHOTGUN = "ingame/ammobox_shotgun.tga",
        AMMOBOX_UNIVERSAL = "ingame/ammobox_universal.tga",
        AMMOBOX_RIFLE = "ingame/ammobox_rifle.tga",
        AMMOBOX_ROCKET = "ingame/ammobox_rocket.tga",
        AMMOBOX_ENERGY = "ingame/ammobox_energy.tga",
        HEALING_POT = "ingame/healing_pot.tga",
        LARGE_HEALING_POT = "ingame/large_healing_pot.tga",
        GUNS_SHOTGUN = "ingame/guns/shotgun.tga",
        GUNS_LUGER = "ingame/guns/luger.tga",
        GUNS_REVOLVER = "ingame/guns/revolver.tga",
        GUNS_M4 = "ingame/guns/M4.tga",
        GUNS_UZI = "ingame/guns/uzi.tga",
        GUNS_THOMPSON = "ingame/guns/thompson.tga",
        GUNS_SNIPER = "ingame/guns/sniper.tga",
        GUNS_RPG = "ingame/guns/rpg.tga",
        GUNS_MINIGUN = "ingame/guns/minigun.tga",
        GUNS_CROSSBOW = "ingame/guns/crossbow.tga",
        GUNS_FLAMETHROWER = "ingame/guns/flamethrower.tga",
        ROUND_TRACER = "ingame/rounds/tracer.tga",
        ROUND_SLUG = "ingame/rounds/slug.tga",
        ROUND_ROCKET = "ingame/rounds/rocket.tga",
        ROUND_BOLT = "ingame/rounds/bolt.tga",
        ROUND_FLAME = "ingame/rounds/flame.tga",
        GUNS_BLASTER = "ingame/guns/blaster.tga",
        GUNS_PLASMA_RIFLE = "ingame/guns/plasma_rifle.tga",
        GUNS_TESLA_GUN = "ingame/guns/tesla_gun.tga",
        GUNS_ARC_CASTER = "ingame/guns/arc_caster.tga",
        GUNS_RAILGUN = "ingame/guns/railgun.tga",
        ROUND_LASER = "ingame/rounds/laser.tga",
        ROUND_PLASMA = "ingame/rounds/plasma.tga",
        ROUND_SPARK = "ingame/rounds/spark.tga",
        ROUND_SPARK_VIOLET = "ingame/rounds/spark_violet.tga",
        ROUND_RAIL = "ingame/rounds/rail.tga",
        ROUND_ARC = "ingame/rounds/arc.tga",
        ROUND_ARC_VIOLET = "ingame/rounds/arc_violet.tga",
        ROUND_MOTE_VOLT = "ingame/rounds/mote_volt.tga",
        ROUND_MOTE_LASER = "ingame/rounds/mote_laser.tga",
        SHOTGUN_AMMO_HUD = "ingame/shotgun_ammo_hud.tga",
        RIFLE_AMMO_HUD = "ingame/rifle_ammo_hud.tga",
        ROCKET_AMMO_HUD = "ingame/rocket_ammo_hud.tga",
        BATTERY_HUD = "ingame/battery_hud.tga",
        SPIDER = "ingame/spider.tga",
        KINGSLIME = "ingame/kingslime.tga",
        COBWEB_PROJECTILE = "ingame/cobweb_projectile.tga",
        PURPLE_SLIMEBULLET = "ingame/purple_slimebullet.tga",
        LADDER = "ingame/ladder.tga",
        LADDER_SEWER = "ingame/ladder_sewer.tga",
        LADDER_MOSS = "ingame/ladder_moss.tga",
        LADDER_SPIDER = "ingame/ladder_spider.tga",
        LADDER_ICE = "ingame/ladder_ice.tga",
        LADDER_LAVA = "ingame/ladder_lava.tga",
        LADDER_TEMPLE = "ingame/ladder_temple.tga",
        SNAKE = "ingame/snake.tga",
        BAT = "ingame/bat.tga",
        FLAME_BAT = "ingame/flame_bat.tga",
        ASH_PILE = "ingame/ash_pile.tga",
        ICICLE_BAT = "ingame/icicle_bat.tga",
        FALLING_ICICLE = "ingame/falling_icicle.tga",
        GHOST = "ingame/ghost.tga",
        DEMONEYE = "ingame/demoneye.tga",
        SPIDERLING = "ingame/spiderling.tga",
        SLIME = "ingame/slime.tga",
        MAGMA_SLIME = "ingame/magma_slime.tga",
        MAGMA_SLIME_CRUST = "ingame/magma_slime_crust.tga",
        LAVA_WORM = "ingame/lava_worm.tga",
        SNOW_SLIME = "ingame/snow_slime.tga",
        REDSLIME = "ingame/redslime.tga",
        GREEN_SPIDERLING = "ingame/green_spiderling.tga",
        PURPLE_SPIDERLING = "ingame/purple_spiderling.tga",
        EYE_WATCHER = "ingame/eye_watcher.tga",
        EYE_WATCHER_PROJECTILE = "ingame/eye_watcher_projectile.tga",
        GREEN_SLIMEBULLET = "ingame/green_slimebullet.tga",
        MAGMA_BALL = "ingame/magma_ball.tga",
        SNOWBALL = "ingame/snowball.tga",
        RED_SLIMEBULLET = "ingame/red_slimebullet.tga",
        SHOPKEEPER = "ingame/shopkeeper.tga",
        SHOPTABLE = "ingame/shoptable.tga",
        SLOTMACHINE = "ingame/slotmachine.tga",
        SHOP_COUNTER = "ingame/shop_counter.tga",
        SHOP_SHELF = "ingame/shop_shelf.tga",
        SHOP_GOODS = "ingame/shop_goods.tga",
        SHOP_LAMP = "ingame/shop_lamp.tga",
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
        KEYBOARD_SPACE = "ingame/keyboard/space.tga",
        SPIKE = "ingame/spike.tga",
        ICE_PATCH = "ingame/ice_patch.tga",
        WEB_PATCH = "ingame/web_patch.tga",
        ARROW_TRAP = "ingame/arrow_trap.tga",
        TRAP_ARROW = "ingame/trap_arrow.tga",
        BLOATED_SPIDER = "ingame/bloated_spider.tga",
        GOLEM = "ingame/golem.png",
        MINI_GOLEM = "ingame/mini_golem.tga",
        GOLEM_ROCK = "ingame/golem_rock.tga",
        GOLEM_BOULDER = "ingame/golem_boulder.tga",
        ROCK_SHADOW = "ingame/rock_shadow.tga",
        SPARK = "ingame/spark.tga",
        OGRE = "ingame/ogre.tga",
        HELLHOUND = "ingame/hellhound.tga",
        MIMIC = "ingame/mimic.tga",
        MIMIC_COIN = "ingame/mimic_coin.tga",
        MIMIC_TOOTH = "ingame/mimic_tooth.tga",
        MIMIC_BAIT = "ingame/mimic_bait.tga",
        MIMIC_DROOL = "ingame/mimic_drool.tga",
        SALIVA_SLIME = "ingame/saliva_slime.tga",
        HOUND_SPARK = "ingame/hound_spark.tga",
        HOUND_MAGMA = "ingame/hound_magma.tga",
        ARGUS = "ingame/argus.tga",
        ARGUS_SHOT = "ingame/argus_shot.tga",
        ARGUS_ORBIT = "ingame/argus_orbit.tga",
        STILT_SPIDER = "ingame/stilt_spider.tga",
        TOXIC_CLOUD = "ingame/toxic_cloud.tga",
        ARROW_HIT = "ingame/arrow_hit.tga",
        SURFACE_ICE = "ingame/surface_ice.tga",
        SURFACE_BOG = "ingame/surface_bog.tga",
        SHORE_BOG = "ingame/shore_bog.tga",
        SURFACE_SAND = "ingame/surface_sand.tga",
        SHORE_SAND = "ingame/shore_sand.tga",
        SURFACE_BLOOD = "ingame/surface_blood.tga",
        SHORE_BLOOD = "ingame/shore_blood.tga",
        SHACKLES = "ingame/shackles.tga",
        PENDULUM = "ingame/pendulum.tga",
        MUSHROOMS = "ingame/mushrooms.tga",
        SURFACE_SHADE = "ingame/surface_shade.tga",
        RIPPLE = "ingame/ripple.tga",
        SHORE_ICE = "ingame/shore_ice.tga",
        SHORE_LAVA = "ingame/shore_lava.tga",
        SHORE_WATER = "ingame/shore_water.tga",
        SURFACE_LAVA = "ingame/surface_lava.tga",
        SURFACE_WATER = "ingame/surface_water.tga",
        ICICLES = "ingame/icicles.tga",
        MOSS_DRAPE = "ingame/moss_drape.tga",
        FOG = "ingame/fog.tga",
        OVERHEADS = "ingame/overheads.tga",
        OVERHEADS_UPRIGHT = "ingame/overheads_upright.tga",
        HANGING = "ingame/hanging.tga",
        RAILS = "ingame/rails.tga",
        SEWER_PIPE = "ingame/sewer_pipe.tga",
        SEWER_STREAM = "ingame/sewer_stream.tga",
        SEWER_DEBRIS = "ingame/sewer_debris.tga",
        SEWER_BRIDGE = "ingame/sewer_bridge.tga",
        LAVA_CRACK = "ingame/lava_crack.tga",
        LAVA_FALL = "ingame/lava_fall.tga",
        LAVA_DEBRIS = "ingame/lava_debris.tga",
        LAVA_BRIDGE = "ingame/lava_bridge.tga",
        BOG_SEEP = "ingame/bog_seep.tga",
        BOG_TRICKLE = "ingame/bog_trickle.tga",
        BOG_DEBRIS = "ingame/bog_debris.tga",
        BOG_LOG = "ingame/bog_log.tga",
        ICE_SPOUT = "ingame/ice_spout.tga",
        ICE_FALL = "ingame/ice_fall.tga",
        ICE_DEBRIS = "ingame/ice_debris.tga",
        TABLE = "ingame/table.tga",
        MOTES = "ingame/motes.tga",
        ICE_CRYSTALS = "ingame/ice_crystals.tga",
        FROZEN_SKULL = "ingame/frozen_skull.tga",
        COCOON = "ingame/cocoon.tga",
        SPIDER_EGGS = "ingame/spider_eggs.tga",
        SPIDER_WRAPPED = "ingame/spider_wrapped.tga",
        SAND_CRACK = "ingame/sand_crack.tga",
        SAND_FALL = "ingame/sand_fall.tga",
        FIRE_BOWL = "ingame/fire_bowl.tga",
        STATUE_PIECES = "ingame/statue_pieces.tga",
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
    tex::TILESET_SEWER,
    tex::TILESET_MOSS,
    tex::TILESET_SPIDER,
    tex::TILESET_ICE,
    tex::TILESET_LAVA,
    tex::TILESET_TEMPLE,
    tex::BULLET,
    tex::RAT,
    tex::EMBER_RAT,
    tex::FROST_RAT,
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
    tex::GUIDE,
    tex::RIFT,
    tex::SARCOPHAGUS,
    tex::SARCOPHAGUS_LID,
    tex::PATHGATE,
    tex::BANNER,
    tex::BANNER_MOSSY,
    tex::TALENT_FRAME,
    tex::TALENT_ROOT,
    tex::TALENT_BOARD,
    tex::TALENT_SELECT,
    tex::BOARD_NOTICE,
    tex::NOTE_PAPER,
    tex::CHALLENGE_PAPER,
    tex::TRAINING_DUMMY,
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
    tex::ARTEFACTS_HOURGLASS_OF_ASH,
    tex::ARTEFACTS_BLOOD_CHALICE,
    tex::ARTEFACTS_PHANTOM_CLOAK,
    tex::ARTEFACTS_GRAVE_LANTERN,
    tex::ARTEFACTS_MIDAS_COIN,
    tex::ARTEFACTS_THUNDER_IDOL,
    tex::ARTEFACTS_INFINITY_QUIVER,
    tex::ARTEFACTS_INFINITY_GLINT,
    tex::ARTEFACTS_BONE_TURRET,
    tex::ARTEFACTS_GHOST,
    tex::ARTEFACTS_TURRET,
    tex::ARTEFACTS_BONE_SHARD,
    tex::ARTEFACTS_LIGHTNING_BOLT,
    tex::ARTEFACTS_LIGHTNING_BURST,
    tex::ARTEFACTS_LIGHTNING_SPARK,
    tex::ARTEFACTS_MAGICSHIELD,
    tex::ARTEFACTS_BERSERKPOT_PARTICLE,
    tex::ARTEFACTS_BLOOD_DROPS,
    tex::ARTEFACTS_MIDAS_SPARKLE,
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
    tex::BANECOIN_ANIM,
    tex::AMMOBOX_PISTOL,
    tex::AMMOBOX_SHOTGUN,
    tex::AMMOBOX_UNIVERSAL,
    tex::AMMOBOX_RIFLE,
    tex::AMMOBOX_ROCKET,
    tex::AMMOBOX_ENERGY,
    tex::HEALING_POT,
    tex::LARGE_HEALING_POT,
    tex::GUNS_SHOTGUN,
    tex::GUNS_LUGER,
    tex::GUNS_REVOLVER,
    tex::GUNS_M4,
    tex::GUNS_UZI,
    tex::GUNS_THOMPSON,
    tex::GUNS_SNIPER,
    tex::GUNS_RPG,
    tex::GUNS_MINIGUN,
    tex::GUNS_CROSSBOW,
    tex::GUNS_FLAMETHROWER,
    tex::ROUND_TRACER,
    tex::ROUND_SLUG,
    tex::ROUND_ROCKET,
    tex::ROUND_BOLT,
    tex::ROUND_FLAME,
    tex::GUNS_BLASTER,
    tex::GUNS_PLASMA_RIFLE,
    tex::GUNS_TESLA_GUN,
    tex::GUNS_ARC_CASTER,
    tex::GUNS_RAILGUN,
    tex::ROUND_LASER,
    tex::ROUND_PLASMA,
    tex::ROUND_SPARK,
    tex::ROUND_SPARK_VIOLET,
    tex::ROUND_RAIL,
    tex::ROUND_ARC,
    tex::ROUND_ARC_VIOLET,
    tex::ROUND_MOTE_VOLT,
    tex::ROUND_MOTE_LASER,
    tex::SHOTGUN_AMMO_HUD,
    tex::RIFLE_AMMO_HUD,
    tex::ROCKET_AMMO_HUD,
    tex::BATTERY_HUD,
    tex::SPIDER,
    tex::KINGSLIME,
    tex::COBWEB_PROJECTILE,
    tex::PURPLE_SLIMEBULLET,
    tex::LADDER,
    tex::LADDER_SEWER,
    tex::LADDER_MOSS,
    tex::LADDER_SPIDER,
    tex::LADDER_ICE,
    tex::LADDER_LAVA,
    tex::LADDER_TEMPLE,
    tex::SNAKE,
    tex::BAT,
    tex::FLAME_BAT,
    tex::ASH_PILE,
    tex::ICICLE_BAT,
    tex::FALLING_ICICLE,
    tex::GHOST,
    tex::DEMONEYE,
    tex::SPIDERLING,
    tex::SLIME,
    tex::MAGMA_SLIME,
    tex::MAGMA_SLIME_CRUST,
    tex::LAVA_WORM,
    tex::SNOW_SLIME,
    tex::REDSLIME,
    tex::GREEN_SPIDERLING,
    tex::PURPLE_SPIDERLING,
    tex::EYE_WATCHER,
    tex::EYE_WATCHER_PROJECTILE,
    tex::GREEN_SLIMEBULLET,
    tex::MAGMA_BALL,
    tex::SNOWBALL,
    tex::RED_SLIMEBULLET,
    tex::SHOPKEEPER,
    tex::SHOPTABLE,
    tex::SLOTMACHINE,
    tex::SHOP_COUNTER,
    tex::SHOP_SHELF,
    tex::SHOP_GOODS,
    tex::SHOP_LAMP,
    tex::CARPET_TILEMAP,
    tex::CHEST,
    tex::BRICKS,
    tex::ANGEL_STATUE,
    tex::CANDLESTICK,
    tex::PENTAGRAM,
    tex::LAYING_BOOK,
    tex::KEYBOARD_SPACE,
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
    tex::ICE_PATCH,
    tex::WEB_PATCH,
    tex::ARROW_TRAP,
    tex::TRAP_ARROW,
    tex::BLOATED_SPIDER,
    tex::GOLEM,
    tex::MINI_GOLEM,
    tex::GOLEM_ROCK,
    tex::GOLEM_BOULDER,
    tex::ROCK_SHADOW,
    tex::SPARK,
    tex::OGRE,
    tex::HELLHOUND,
    tex::MIMIC,
    tex::MIMIC_COIN,
    tex::MIMIC_TOOTH,
    tex::MIMIC_BAIT,
    tex::MIMIC_DROOL,
    tex::SALIVA_SLIME,
    tex::HOUND_SPARK,
    tex::HOUND_MAGMA,
    tex::ARGUS,
    tex::ARGUS_SHOT,
    tex::ARGUS_ORBIT,
    tex::STILT_SPIDER,
    tex::TOXIC_CLOUD,
    tex::ARROW_HIT,
    tex::SURFACE_ICE,
    tex::SURFACE_BOG,
    tex::SHORE_BOG,
    tex::SURFACE_SAND,
    tex::SHORE_SAND,
    tex::SURFACE_BLOOD,
    tex::SHORE_BLOOD,
    tex::SHACKLES,
    tex::PENDULUM,
    tex::MUSHROOMS,
    tex::SURFACE_SHADE,
    tex::RIPPLE,
    tex::SHORE_ICE,
    tex::SHORE_LAVA,
    tex::SHORE_WATER,
    tex::SURFACE_LAVA,
    tex::SURFACE_WATER,
    tex::ICICLES,
    tex::MOSS_DRAPE,
    tex::FOG,
    tex::OVERHEADS,
    tex::OVERHEADS_UPRIGHT,
    tex::HANGING,
    tex::RAILS,
    tex::SEWER_PIPE,
    tex::SEWER_STREAM,
    tex::SEWER_DEBRIS,
    tex::SEWER_BRIDGE,
    tex::LAVA_CRACK,
    tex::LAVA_FALL,
    tex::LAVA_DEBRIS,
    tex::LAVA_BRIDGE,
    tex::BOG_SEEP,
    tex::BOG_TRICKLE,
    tex::BOG_DEBRIS,
    tex::BOG_LOG,
    tex::ICE_SPOUT,
    tex::ICE_FALL,
    tex::ICE_DEBRIS,
    tex::TABLE,
    tex::MOTES,
    tex::ICE_CRYSTALS,
    tex::FROZEN_SKULL,
    tex::COCOON,
    tex::SPIDER_EGGS,
    tex::SPIDER_WRAPPED,
    tex::SAND_CRACK,
    tex::SAND_FALL,
    tex::FIRE_BOWL,
    tex::STATUE_PIECES,
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
    wide_tileset: Arc<Tileset>,
    pub(crate) fonts: Fonts,
    loaded: bool,
}

impl Assets {
    pub(crate) fn tileset_of(&self, wide: bool) -> &Tileset {
        if wide {
            &self.wide_tileset
        } else {
            &self.tileset
        }
    }

    pub(crate) fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            textures: Textures::default(),
            tileset: Arc::new(Tileset::dungeon()?),
            wide_tileset: Arc::new(Tileset::dungeon_wide()?),
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
    Holed(Tex, [u32; 4]),
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
    pub(crate) fn start(
        gpu: &mut Gpu,
        assets: &mut Assets,
        holed: &[(Tex, [u32; 4])],
    ) -> Result<Self, Box<dyn Error>> {
        for key in EARLY_TEXTURES {
            load_texture(gpu, &mut assets.textures, *key)?;
        }
        let mut pending: VecDeque<Step> =
            GAME_TEXTURES.iter().copied().map(Step::Texture).collect();
        pending.extend(
            holed
                .iter()
                .map(|(texture, hole)| Step::Holed(*texture, *hole)),
        );
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
            Step::Holed(texture, hole) => {
                let path = texture.path();
                let pixels = Pixels::load_with_hole(&root().join("textures").join(path), hole)?;
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
    use crate::components::BarKind;
    use crate::dungeon::Theme;

    #[test]
    fn every_theme_backdrop_matches_the_darkest_colour_of_its_tileset() {
        for theme in [
            Theme::Stone,
            Theme::Sewer,
            Theme::Moss,
            Theme::Spider,
            Theme::Ice,
            Theme::Lava,
            Theme::Temple,
        ] {
            let path = root().join("textures").join(theme.tileset_texture().path());
            let image = image::open(&path).expect("the tileset opens").to_rgba8();
            let [red, green, blue] = theme.backdrop();
            let found = image
                .pixels()
                .any(|pixel| pixel.0 == [red, green, blue, 255]);
            assert!(found, "{theme:?} backdrop is not in {}", path.display());
        }
    }

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
