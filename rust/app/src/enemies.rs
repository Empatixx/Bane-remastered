use glam::{Vec2, Vec4};

use crate::assets::{Tex, tex};
use crate::clip::Clip;
use crate::dungeon::Theme;
use crate::rng::Rng;

const DAMPING: f32 = 5.0;
const STEP_RATE: f32 = 60.0;
const HEALTH_GROWTH_PER_FLOOR: f32 = 0.12;
const DAMAGE_GROWTH_PER_FLOOR: f32 = 0.08;

pub(crate) const DASH_FORCE: f32 = 24_500_000.0;
pub(crate) const HEAL_RADIUS: f32 = 200.0;
pub(crate) const SMASH_COOLDOWN: f32 = 2.0;
pub(crate) const SMASH_BULLETS: usize = 5;
pub(crate) const SHOOT_COOLDOWN: f32 = 3.0;
pub(crate) const HATCH_COOLDOWN: f32 = 2.5;
pub(crate) const WEB_COOLDOWN: f32 = 5.0;
pub(crate) const ARMAGEDDON_COOLDOWN: f32 = 0.15;
pub(crate) const ARMAGEDDON_TURN: f32 = 0.25;
pub(crate) const KING_SMASH_BULLETS: usize = 25;
pub(crate) const COMMON_SPAWN_SECONDS: f32 = 2.0;
pub(crate) const BOSS_SPAWN_SECONDS: f32 = 1.5;
pub(crate) const FINAL_FLOOR: u32 = 5;
/// The themes that keep the plain rat, bat and slime: ice and lava have their own.
const PLAIN_THEMES: &[Theme] = &[
    Theme::Stone,
    Theme::Sewer,
    Theme::Moss,
    Theme::Spider,
    Theme::Temple,
];
/// An ember rat hit once burns brighter and runs this many times faster.
pub(crate) const EMBER_RUSH: f32 = 2.4;
/// A flame bat circles for a while, flares up, then swoops through the player.
pub(crate) const SWOOP_EVERY: f32 = 3.2;
pub(crate) const SWOOP_WINDUP: f32 = 0.5;
pub(crate) const SWOOP_DIVE: f32 = 0.55;
pub(crate) const SWOOP_RUSH: f32 = 3.4;
/// What a flame bat leaves: ash that rises again unless shot within the time.
pub(crate) const ASH_REBIRTH: f32 = 2.0;
pub(crate) const ASH_HEALTH: i32 = 3;
/// A magma slime takes turns between molten and crusted.
pub(crate) const MAGMA_TURN: f32 = 3.0;
pub(crate) const CRUST_SLOWDOWN: f32 = 0.4;
/// The magma slime's lobbed ball: how often, how far, how long in the air.
pub(crate) const MORTAR_EVERY: f32 = 2.8;
pub(crate) const MORTAR_RANGE: f32 = 520.0;
pub(crate) const MORTAR_FLIGHT: f32 = 1.0;
pub(crate) const MORTAR_DAMAGE: i32 = 1;
pub(crate) const MORTAR_RADIUS: f32 = 40.0;
/// The lava worm: it shows its hole, rises, spits a few magma shots at the
/// player, sinks and comes up somewhere else near the player. It can only be
/// hurt while it is up.
pub(crate) const WORM_HIDDEN: f32 = 1.0;
pub(crate) const WORM_SHOW: f32 = 0.7;
pub(crate) const WORM_RISE: f32 = 0.32;
pub(crate) const WORM_SINK: f32 = 0.32;
pub(crate) const WORM_SHOTS: u32 = 3;
pub(crate) const WORM_SHOT_EVERY: f32 = 0.7;
/// How far from the player the worm comes up.
pub(crate) const WORM_EMERGE_NEAR: f32 = 180.0;
pub(crate) const WORM_EMERGE_FAR: f32 = 320.0;
/// The worm's first row: rising out of the floor.
pub(crate) const WORM_RISE_CLIP: crate::clip::Clip = crate::clip::Clip {
    row: 0,
    frames: 4,
    frame_time: 0.08,
    looping: false,
    flipped: false,
};

/// Seconds between two icicles from one icicle bat, and how close the player must be.
pub(crate) const ICICLE_COOLDOWN: f32 = 3.5;
pub(crate) const ICICLE_RANGE: f32 = 420.0;
/// Seconds from the shadow showing to the icicle landing.
pub(crate) const ICICLE_FALL: f32 = 0.9;
pub(crate) const ICICLE_DAMAGE: i32 = 1;
pub(crate) const ICICLE_RADIUS: f32 = 34.0;
/// How many slimelings a snow slime breaks into.
pub(crate) const SNOW_SPLIT: u32 = 2;
/// Chill hits on the player in a row that freeze them, and for how long.
pub(crate) const PLAYER_FREEZE_STACKS: u32 = 3;
pub(crate) const PLAYER_FREEZE_SECONDS: f32 = 1.0;
pub(crate) const GOLEM_ROCK_RING: usize = 16;
pub(crate) const GOLEM_BOULDER_DAMAGE: i32 = 2;
pub(crate) const GOLEM_ROCK_DAMAGE: i32 = 2;
pub(crate) const GOLEM_FIRST_COOLDOWN: f32 = 2.0;
pub(crate) const GOLEM_COOLDOWN: f32 = 2.2;
pub(crate) const GOLEM_RAGE_COOLDOWN: f32 = 1.3;
pub(crate) const GOLEM_RAGE_AT: f32 = 0.5;
pub(crate) const GOLEM_RAGE_SPEED: f32 = 1.35;
/// The colour the golem turns once it is enraged.
pub(crate) const GOLEM_RAGE_TINT: Vec4 = Vec4::new(1.0, 0.62, 0.55, 1.0);
pub(crate) const GOLEM_SLAM_WINDUP: f32 = 0.45;
pub(crate) const GOLEM_SLAM_RECOVER: f32 = 0.45;
pub(crate) const GOLEM_SECOND_RING: f32 = 0.35;
pub(crate) const GOLEM_THROW_WINDUP: f32 = 0.45;
pub(crate) const GOLEM_THROW_RECOVER: f32 = 0.25;
pub(crate) const GOLEM_RAIN_WINDUP: f32 = 0.4;
pub(crate) const GOLEM_RAIN_RECOVER: f32 = 0.5;
pub(crate) const GOLEM_LASER_RECOVER: f32 = 0.6;
pub(crate) const GOLEM_RAIN_SECONDS: f32 = 2.8;
pub(crate) const GOLEM_RAIN_EVERY: f32 = 0.12;
pub(crate) const GOLEM_MINION_CHANCE: f32 = 0.16;
pub(crate) const GOLEM_MINION_LIMIT: usize = 4;
pub(crate) const GOLEM_RAIN_DELAY: f32 = 0.7;
pub(crate) const GOLEM_RAIN_DAMAGE: i32 = 2;
pub(crate) const GOLEM_RAIN_RADIUS: f32 = 74.0;
pub(crate) const GOLEM_RAIN_SPREAD: f32 = 300.0;
pub(crate) const GOLEM_LASER_CHARGE: f32 = 1.0;
pub(crate) const GOLEM_LASER_SECONDS: f32 = 1.9;
pub(crate) const GOLEM_LASER_TURN: f32 = 0.7;
pub(crate) const GOLEM_LASER_RAGE_TURN: f32 = 1.1;
pub(crate) const GOLEM_LASER_REACH: f32 = 1100.0;
pub(crate) const GOLEM_LASER_WIDTH: f32 = 26.0;
pub(crate) const GOLEM_LASER_DAMAGE: i32 = 2;
pub(crate) const GOLEM_NEAR: f32 = 340.0;
pub(crate) const HOP_SECONDS: f32 = 0.42;
pub(crate) const HOP_REST: f32 = 0.6;
pub(crate) const MIMIC_LURE_WINDUP: f32 = 0.36;
pub(crate) const MIMIC_LURE_RECOVER: f32 = 0.5;
pub(crate) const MIMIC_BAITS: usize = 5;
pub(crate) const MIMIC_BAIT_LIMIT: usize = 8;
pub(crate) const MIMIC_BAIT_SECONDS: f32 = 9.0;
pub(crate) const MIMIC_BAIT_REACH: f32 = 34.0;
pub(crate) const MIMIC_BAIT_DAMAGE: i32 = 2;
pub(crate) const MIMIC_FIRST_COOLDOWN: f32 = 2.2;
pub(crate) const MIMIC_COOLDOWN: f32 = 2.6;
pub(crate) const MIMIC_RAGE_COOLDOWN: f32 = 1.5;
pub(crate) const MIMIC_RAGE_AT: f32 = 0.5;
pub(crate) const MIMIC_RAGE_SPEED: f32 = 1.3;
/// The colour the mimic turns once it is enraged.
pub(crate) const MIMIC_RAGE_TINT: Vec4 = Vec4::new(1.0, 0.72, 0.66, 1.0);
pub(crate) const MIMIC_NEAR: f32 = 330.0;
pub(crate) const MIMIC_HOP_WINDUP: f32 = 0.2;
pub(crate) const MIMIC_HOP_SECONDS: f32 = 0.45;
pub(crate) const MIMIC_HOP_SPEED: f32 = 6.0;
pub(crate) const MIMIC_HOP_RECOVER: f32 = 0.3;
pub(crate) const MIMIC_HOP_RING: usize = 14;
pub(crate) const MIMIC_BITE_WINDUP: f32 = 0.28;
pub(crate) const MIMIC_BITE_SECONDS: f32 = 0.13;
pub(crate) const MIMIC_BITE_SPEED: f32 = 4.0;
pub(crate) const MIMIC_BITE_RECOVER: f32 = 0.4;
pub(crate) const MIMIC_BITE_FAN: usize = 7;
pub(crate) const MIMIC_BITE_SPREAD: f32 = 0.5;
pub(crate) const MIMIC_SPIT_WINDUP: f32 = 0.32;
pub(crate) const MIMIC_SPIT_RECOVER: f32 = 0.55;
pub(crate) const MIMIC_MOB_LIMIT: usize = 4;
pub(crate) const MIMIC_REST_WINDUP: f32 = 0.3;
pub(crate) const MIMIC_REST_SECONDS: f32 = 2.2;
pub(crate) const MIMIC_REST_RECOVER: f32 = 0.3;
pub(crate) const MIMIC_SPLINTER_DAMAGE: i32 = 2;
pub(crate) const HOUND_FIRST_COOLDOWN: f32 = 2.2;
pub(crate) const HOUND_COOLDOWN: f32 = 2.4;
pub(crate) const HOUND_RAGE_COOLDOWN: f32 = 1.3;
pub(crate) const HOUND_RAGE_AT: f32 = 0.5;
pub(crate) const HOUND_RAGE_SPEED: f32 = 1.25;
/// The colour the hound turns once it is enraged.
pub(crate) const HOUND_RAGE_TINT: Vec4 = Vec4::new(1.0, 0.7, 0.62, 1.0);
pub(crate) const HOUND_NEAR: f32 = 360.0;
pub(crate) const HOUND_LUNGE_WINDUP: f32 = 0.32;
pub(crate) const HOUND_LUNGE_SECONDS: f32 = 1.4;
pub(crate) const HOUND_LUNGE_SPEED: f32 = 6.5;
pub(crate) const HOUND_LUNGE_RECOVER: f32 = 0.6;
pub(crate) const HOUND_CRASH_STUN: f32 = 1.7;
pub(crate) const HOUND_CRASH_REACH: f32 = 140.0;
pub(crate) const HOUND_MAGMA_START: f32 = 3.0;
pub(crate) const HOUND_MAGMA_EVERY: f32 = 0.45;
pub(crate) const HOUND_MAGMA_RAGE_EVERY: f32 = 0.28;
pub(crate) const HOUND_MAGMA_DEPTH: usize = 3;
pub(crate) const HOUND_MAGMA_WARM: f32 = 1.0;
pub(crate) const HOUND_MAGMA_LIFE: f32 = 16.0;
pub(crate) const HOUND_MAGMA_DAMAGE: i32 = 2;
pub(crate) const HOUND_BREATH_WINDUP: f32 = 0.45;
pub(crate) const HOUND_BREATH_SECONDS: f32 = 1.8;
pub(crate) const HOUND_BEAM_TURN: f32 = 0.45;
pub(crate) const HOUND_BEAM_RAGE_TURN: f32 = 0.7;
pub(crate) const HOUND_BREATH_RECOVER: f32 = 0.6;
pub(crate) const HOUND_BEAM_DAMAGE: i32 = 1;
pub(crate) const HOUND_HOWL_WINDUP: f32 = 0.3;
pub(crate) const HOUND_HOWL_RECOVER: f32 = 0.55;
pub(crate) const HOUND_HOWL_RING: usize = 14;
pub(crate) const HOUND_HOWL_RAGE_RING: usize = 20;
pub(crate) const HOUND_FIRE_DAMAGE: i32 = 1;
pub(crate) const OGRE_FIRST_COOLDOWN: f32 = 2.4;
pub(crate) const OGRE_COOLDOWN: f32 = 3.0;
pub(crate) const OGRE_RAGE_COOLDOWN: f32 = 1.8;
pub(crate) const OGRE_RAGE_AT: f32 = 0.5;
pub(crate) const OGRE_RAGE_SPEED: f32 = 1.35;
/// The colour the ogre turns once it is enraged.
pub(crate) const OGRE_RAGE_TINT: Vec4 = Vec4::new(1.0, 0.7, 0.62, 1.0);
pub(crate) const OGRE_NEAR: f32 = 380.0;
pub(crate) const OGRE_JUMP_WINDUP: f32 = 0.5;
pub(crate) const OGRE_JUMP_RECOVER: f32 = 0.6;
pub(crate) const OGRE_JUMP_RING: usize = 16;
pub(crate) const OGRE_JUMP_RINGS: usize = 3;
pub(crate) const OGRE_JUMP_GAP: f32 = 55.0;
pub(crate) const OGRE_STOMP_DAMAGE: i32 = 2;
pub(crate) const OGRE_THROW_WINDUP: f32 = 0.38;
pub(crate) const OGRE_THROW_RECOVER: f32 = 0.45;
pub(crate) const OGRE_THROW_RATS: usize = 2;
pub(crate) const OGRE_SLAM_WINDUP: f32 = 0.45;
pub(crate) const OGRE_SLAM_RECOVER: f32 = 0.45;
pub(crate) const OGRE_SLAM_FAN: usize = 11;
pub(crate) const OGRE_SLAM_SPREAD: f32 = 0.9;
pub(crate) const OGRE_ROAR_WINDUP: f32 = 0.32;
pub(crate) const OGRE_ROAR_RECOVER: f32 = 0.55;
pub(crate) const OGRE_ROAR_RATS: usize = 3;
pub(crate) const OGRE_RAT_LIMIT: usize = 6;
pub(crate) const ARGUS_FIRST_COOLDOWN: f32 = 2.0;
pub(crate) const ARGUS_COOLDOWN: f32 = 1.7;
pub(crate) const ARGUS_RAGE_COOLDOWN: f32 = 0.9;
pub(crate) const ARGUS_RAGE_AT: f32 = 0.5;
pub(crate) const ARGUS_RAGE_SPEED: f32 = 1.3;
/// The colour the argus turns once it is enraged.
pub(crate) const ARGUS_RAGE_TINT: Vec4 = Vec4::new(1.0, 0.72, 0.66, 1.0);
pub(crate) const ARGUS_VOLLEY_WINDUP: f32 = 0.5;
pub(crate) const ARGUS_VOLLEY_PULSES: u32 = 5;
pub(crate) const ARGUS_VOLLEY_RAGE_PULSES: u32 = 7;
pub(crate) const ARGUS_VOLLEY_EVERY: f32 = 0.4;
pub(crate) const ARGUS_VOLLEY_RING: usize = 10;
pub(crate) const ARGUS_VOLLEY_RAGE_RING: usize = 14;
pub(crate) const ARGUS_SHOT_DAMAGE: i32 = 1;
pub(crate) const ARGUS_BARRAGE_WINDUP: f32 = 0.6;
pub(crate) const ARGUS_BARRAGE_SHOTS: u32 = 14;
pub(crate) const ARGUS_BARRAGE_RAGE_SHOTS: u32 = 22;
pub(crate) const ARGUS_BARRAGE_EVERY: f32 = 0.13;
pub(crate) const ARGUS_BARRAGE_SPREAD: f32 = 0.16;
pub(crate) const ARGUS_SHIELD_WINDUP: f32 = 0.6;
pub(crate) const ARGUS_ORBIT_COUNT: usize = 4;
pub(crate) const ARGUS_ORBIT_RAGE_COUNT: usize = 6;
pub(crate) const ARGUS_ORBIT_RADIUS: f32 = 185.0;
pub(crate) const ARGUS_ORBIT_SPEED: f32 = 1.4;
pub(crate) const ARGUS_RECOVER: f32 = 0.5;
pub(crate) const ARGUS_TELEPORT_EVERY: u32 = 3;
pub(crate) const ARGUS_FADE: f32 = 0.35;

pub(crate) fn speed_from_force(force: f32, size: Vec2) -> f32 {
    force / (DAMPING * size.x * size.y)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ProjectileKind {
    Green,
    Red,
    Purple,
    EyeWatcher,
    GolemRock,
    Boulder,
    ArgusShot,
    Ember,
    MimicCoin,
    MimicTooth,
    /// The snow slime's snowball: it chills, and enough of them freeze.
    Snow,
    /// The lava worm's spit and the magma slime's lobbed ball.
    Magma,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ProjectileSpec {
    pub(crate) texture: Tex,
    pub(crate) columns: usize,
    pub(crate) rows: usize,
    pub(crate) frame: Vec2,
    pub(crate) scale: f32,
    pub(crate) idle: (usize, f32),
    pub(crate) impact: (usize, f32),
    pub(crate) light: [f32; 5],
    pub(crate) force: f32,
    pub(crate) homing: bool,
    pub(crate) lifetime: Option<f32>,
}

impl ProjectileSpec {
    pub(crate) fn size(&self) -> Vec2 {
        (self.frame * self.scale).floor()
    }

    pub(crate) fn speed(&self) -> f32 {
        let size = self.size();
        self.force / (size.x * size.y) / STEP_RATE
    }

    pub(crate) fn idle_clip(&self) -> Clip {
        Clip {
            row: 0,
            frames: self.idle.0,
            frame_time: self.idle.1,
            looping: true,
            flipped: false,
        }
    }

    pub(crate) fn impact_clip(&self) -> Clip {
        Clip {
            row: 1,
            frames: self.impact.0,
            frame_time: self.impact.1,
            looping: false,
            flipped: false,
        }
    }
}

impl ProjectileKind {
    pub(crate) fn spec(self) -> ProjectileSpec {
        match self {
            Self::Green => slime_bullet(tex::GREEN_SLIMEBULLET, [0.0, 200.0, 0.0]),
            Self::Snow => slime_bullet(tex::SNOWBALL, [150.0, 220.0, 255.0]),
            Self::Magma => slime_bullet(tex::MAGMA_BALL, [255.0, 130.0, 30.0]),
            Self::Red => slime_bullet(tex::RED_SLIMEBULLET, [255.0, 215.0, 0.0]),
            Self::Purple => slime_bullet(tex::PURPLE_SLIMEBULLET, [255.0, 0.0, 255.0]),
            Self::EyeWatcher => ProjectileSpec {
                texture: tex::EYE_WATCHER_PROJECTILE,
                columns: 3,
                rows: 2,
                frame: Vec2::new(16.0, 16.0),
                scale: 3.0,
                idle: (3, 0.07),
                impact: (3, 0.085),
                light: [250.0, 240.0, 0.0, 30.0, 1.0],
                force: 35_000_000.0,
                homing: true,
                lifetime: Some(5.0),
            },
            Self::MimicCoin => ProjectileSpec {
                texture: tex::MIMIC_COIN,
                columns: 4,
                rows: 2,
                frame: Vec2::new(12.0, 12.0),
                scale: 3.2,
                idle: (4, 0.07),
                impact: (4, 0.07),
                light: [255.0, 210.0, 90.0, 22.0, 0.5],
                force: 30_000_000.0,
                homing: false,
                lifetime: Some(1.8),
            },
            Self::MimicTooth => ProjectileSpec {
                texture: tex::MIMIC_TOOTH,
                columns: 4,
                rows: 2,
                frame: Vec2::new(12.0, 12.0),
                scale: 3.2,
                idle: (4, 0.05),
                impact: (4, 0.07),
                light: [255.0, 240.0, 210.0, 16.0, 0.3],
                force: 42_000_000.0,
                homing: false,
                lifetime: Some(1.5),
            },
            Self::Ember => ProjectileSpec {
                texture: tex::HOUND_MAGMA,
                columns: 4,
                rows: 2,
                frame: Vec2::new(16.0, 16.0),
                scale: 3.4,
                idle: (4, 0.09),
                impact: (4, 0.07),
                light: [255.0, 110.0, 30.0, 26.0, 0.6],
                force: 34_000_000.0,
                homing: false,
                lifetime: Some(3.0),
            },
            Self::ArgusShot => ProjectileSpec {
                texture: tex::ARGUS_SHOT,
                columns: 3,
                rows: 2,
                frame: Vec2::new(12.0, 12.0),
                scale: 3.0,
                idle: (3, 0.08),
                impact: (3, 0.07),
                light: [255.0, 140.0, 50.0, 22.0, 0.5],
                force: 26_000_000.0,
                homing: false,
                lifetime: Some(4.0),
            },
            Self::GolemRock => ProjectileSpec {
                texture: tex::GOLEM_ROCK,
                columns: 4,
                rows: 2,
                frame: Vec2::new(6.0, 6.0),
                scale: 6.0,
                idle: (4, 0.08),
                impact: (4, 0.07),
                light: [255.0, 190.0, 80.0, 18.0, 0.4],
                force: 24_000_000.0,
                homing: false,
                lifetime: Some(2.4),
            },
            Self::Boulder => ProjectileSpec {
                texture: tex::GOLEM_BOULDER,
                columns: 4,
                rows: 2,
                frame: Vec2::new(16.0, 16.0),
                scale: 4.0,
                idle: (4, 0.09),
                impact: (4, 0.08),
                light: [255.0, 190.0, 80.0, 28.0, 0.5],
                force: 62_000_000.0,
                homing: false,
                lifetime: Some(3.2),
            },
        }
    }
}

const fn slime_bullet(texture: Tex, color: [f32; 3]) -> ProjectileSpec {
    ProjectileSpec {
        texture,
        columns: 4,
        rows: 2,
        frame: Vec2::new(16.0, 16.0),
        scale: 1.0,
        idle: (4, 0.07),
        impact: (4, 0.07),
        light: [color[0], color[1], color[2], 25.0, 0.75],
        force: 5_000_000.0,
        homing: false,
        lifetime: None,
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EnemyKind {
    Rat,
    Snake,
    Bat,
    Ghost,
    Slime,
    RedSlime,
    DemonEye,
    RedSpiderling,
    GreenSpiderling,
    PurpleSpiderling,
    EyeWatcher,
    BloatedSpider,
    StiltSpider,
    Spider,
    KingSlime,
    Golem,
    MiniGolem,
    Argus,
    ArgusEye,
    Ogre,
    Hellhound,
    Mimic,
    SalivaSlime,
    FrostRat,
    IcicleBat,
    SnowSlime,
    SnowSlimeling,
    EmberRat,
    FlameBat,
    MagmaSlime,
    LavaWorm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Role {
    Common,
    Summoned,
    Boss(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Behavior {
    Plain,
    Dash,
    PoisonTrail,
    Smash(ProjectileKind),
    Burst(ProjectileKind),
    Shoots(ProjectileKind),
    HealsKin,
    ToxicBurst,
    Leaps,
    SpiderBoss,
    KingSlimeBoss,
    GolemBoss,
    ArgusBoss,
    OgreBoss,
    HoundBoss,
    MimicBoss,
    Hops,
    /// Touching it (a bite, say) chills.
    Chills,
    /// Now and then it drops an icicle on the player, shadow first.
    DropsIcicles,
    /// Hit once, it flares up and rushes the player.
    Kindles,
    /// Circles, flares up and swoops through the player; leaves ash that rises again.
    Swoops,
    /// Takes turns between molten and an armoured crust, lobbing magma at the player.
    Molten,
    /// Comes up out of the floor near the player, spits magma, sinks, and comes up elsewhere.
    Burrows,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct EnemySpec {
    pub(crate) texture: Tex,
    pub(crate) columns: usize,
    pub(crate) rows: usize,
    pub(crate) frame: Vec2,
    pub(crate) scale: f32,
    pub(crate) idle_frames: usize,
    pub(crate) idle_time: f32,
    pub(crate) death: Option<(usize, f32)>,
    pub(crate) death_row: usize,
    pub(crate) health: i32,
    pub(crate) damage: i32,
    pub(crate) force: f32,
    pub(crate) min_floor: u32,
    pub(crate) role: Role,
    pub(crate) behavior: Behavior,
    pub(crate) kin: bool,
    pub(crate) themes: &'static [Theme],
    /// The sheet row of the puddle pulling itself back together; enemies with one survive
    /// their first death as a puddle and re-form.
    pub(crate) reform_row: Option<usize>,
}

impl EnemySpec {
    pub(crate) fn size(&self) -> Vec2 {
        (self.frame * self.scale).floor()
    }

    pub(crate) fn speed(&self) -> f32 {
        speed_from_force(self.force, self.size())
    }

    pub(crate) fn idle_clip(&self) -> Clip {
        Clip {
            row: 0,
            frames: self.idle_frames,
            frame_time: self.idle_time,
            looping: true,
            flipped: false,
        }
    }

    pub(crate) fn death_clip(&self) -> Clip {
        match self.death {
            Some((frames, frame_time)) => Clip {
                row: self.death_row,
                frames,
                frame_time,
                looping: false,
                flipped: false,
            },
            None => Clip {
                looping: false,
                ..self.idle_clip()
            },
        }
    }
}

const fn spec(
    texture: Tex,
    grid: (usize, usize),
    frame: (f32, f32),
    scale: f32,
    idle: (usize, f32),
    death: Option<(usize, f32)>,
    stats: (i32, i32, f32),
) -> EnemySpec {
    EnemySpec {
        texture,
        columns: grid.0,
        rows: grid.1,
        frame: Vec2::new(frame.0, frame.1),
        scale,
        idle_frames: idle.0,
        idle_time: idle.1,
        death,
        death_row: 1,
        health: stats.0,
        damage: stats.1,
        force: stats.2,
        min_floor: 0,
        role: Role::Common,
        behavior: Behavior::Plain,
        kin: false,
        themes: &[],
        reform_row: None,
    }
}

const fn spiderling(texture: Tex, rows: usize, stats: (i32, i32, f32)) -> EnemySpec {
    EnemySpec {
        min_floor: 1,
        kin: true,
        themes: &[Theme::Spider],
        ..spec(
            texture,
            (5, rows),
            (32.0, 20.0),
            3.0,
            (5, 0.12),
            Some((3, 0.15)),
            stats,
        )
    }
}

impl EnemyKind {
    pub(crate) fn index(self) -> usize {
        Self::ALL.iter().position(|kind| *kind == self).unwrap_or(0)
    }

    pub(crate) const ALL: [Self; 31] = [
        Self::Rat,
        Self::Snake,
        Self::Bat,
        Self::Ghost,
        Self::Slime,
        Self::RedSlime,
        Self::DemonEye,
        Self::RedSpiderling,
        Self::GreenSpiderling,
        Self::PurpleSpiderling,
        Self::BloatedSpider,
        Self::StiltSpider,
        Self::EyeWatcher,
        Self::Golem,
        Self::MiniGolem,
        Self::Argus,
        Self::ArgusEye,
        Self::Ogre,
        Self::Hellhound,
        Self::Mimic,
        Self::SalivaSlime,
        Self::Spider,
        Self::KingSlime,
        Self::FrostRat,
        Self::IcicleBat,
        Self::SnowSlime,
        Self::SnowSlimeling,
        Self::EmberRat,
        Self::FlameBat,
        Self::MagmaSlime,
        Self::LavaWorm,
    ];

    pub(crate) fn spec(self) -> EnemySpec {
        match self {
            Self::Rat => EnemySpec {
                themes: PLAIN_THEMES,
                ..spec(
                    tex::RAT,
                    (4, 2),
                    (85.0, 37.0),
                    1.5,
                    (4, 0.12),
                    Some((3, 0.15)),
                    (12, 2, 6_500_000.0),
                )
            },
            Self::FrostRat => EnemySpec {
                behavior: Behavior::Chills,
                themes: &[Theme::Ice],
                ..spec(
                    tex::FROST_RAT,
                    (4, 2),
                    (85.0, 37.0),
                    1.5,
                    (4, 0.12),
                    Some((3, 0.15)),
                    (14, 2, 6_500_000.0),
                )
            },
            Self::Snake => EnemySpec {
                behavior: Behavior::PoisonTrail,
                ..spec(
                    tex::SNAKE,
                    (9, 2),
                    (64.0, 64.0),
                    1.5,
                    (9, 0.125),
                    Some((5, 0.1)),
                    (8, 3, 8_000_000.0),
                )
            },
            Self::Bat => EnemySpec {
                themes: PLAIN_THEMES,
                ..spec(
                    tex::BAT,
                    (4, 2),
                    (64.0, 64.0),
                    1.5,
                    (4, 0.125),
                    Some((4, 0.1)),
                    (9, 2, 14_500_000.0),
                )
            },
            Self::IcicleBat => EnemySpec {
                behavior: Behavior::DropsIcicles,
                themes: &[Theme::Ice],
                ..spec(
                    tex::ICICLE_BAT,
                    (4, 2),
                    (64.0, 64.0),
                    1.5,
                    (4, 0.125),
                    Some((4, 0.1)),
                    (10, 2, 14_500_000.0),
                )
            },
            Self::Ghost => EnemySpec {
                behavior: Behavior::Dash,
                ..spec(
                    tex::GHOST,
                    (5, 2),
                    (22.0, 28.0),
                    4.0,
                    (4, 0.1),
                    Some((5, 0.085)),
                    (7, 2, 6_500_000.0),
                )
            },
            Self::Slime => EnemySpec {
                behavior: Behavior::Smash(ProjectileKind::Green),
                themes: PLAIN_THEMES,
                ..spec(
                    tex::SLIME,
                    (6, 2),
                    (32.0, 24.0),
                    3.0,
                    (4, 0.175),
                    Some((6, 0.065)),
                    (9, 1, 1_000_000.0),
                )
            },
            Self::SnowSlime => EnemySpec {
                behavior: Behavior::Smash(ProjectileKind::Snow),
                themes: &[Theme::Ice],
                ..spec(
                    tex::SNOW_SLIME,
                    (6, 2),
                    (32.0, 24.0),
                    3.0,
                    (4, 0.175),
                    Some((6, 0.065)),
                    (11, 1, 1_000_000.0),
                )
            },
            Self::EmberRat => EnemySpec {
                behavior: Behavior::Kindles,
                themes: &[Theme::Lava],
                ..spec(
                    tex::EMBER_RAT,
                    (4, 2),
                    (85.0, 37.0),
                    1.5,
                    (4, 0.12),
                    Some((3, 0.15)),
                    (12, 2, 6_000_000.0),
                )
            },
            Self::FlameBat => EnemySpec {
                behavior: Behavior::Swoops,
                themes: &[Theme::Lava],
                ..spec(
                    tex::FLAME_BAT,
                    (4, 2),
                    (64.0, 64.0),
                    1.5,
                    (4, 0.125),
                    Some((4, 0.1)),
                    (9, 2, 12_000_000.0),
                )
            },
            Self::MagmaSlime => EnemySpec {
                behavior: Behavior::Molten,
                themes: &[Theme::Lava],
                ..spec(
                    tex::MAGMA_SLIME,
                    (6, 2),
                    (32.0, 24.0),
                    3.0,
                    (4, 0.175),
                    Some((6, 0.065)),
                    (12, 1, 1_000_000.0),
                )
            },
            Self::LavaWorm => EnemySpec {
                behavior: Behavior::Burrows,
                themes: &[Theme::Lava],
                min_floor: 1,
                death_row: 2,
                ..spec(
                    tex::LAVA_WORM,
                    (4, 4),
                    (32.0, 52.0),
                    2.0,
                    (4, 0.1),
                    Some((4, 0.14)),
                    (20, 2, 7_000_000.0),
                )
            },
            // What a snow slime breaks into: small, quick to die, and it chills on touch.
            Self::SnowSlimeling => EnemySpec {
                behavior: Behavior::Chills,
                role: Role::Summoned,
                themes: &[Theme::Ice],
                ..spec(
                    tex::SNOW_SLIME,
                    (6, 2),
                    (32.0, 24.0),
                    1.8,
                    (4, 0.15),
                    Some((6, 0.065)),
                    (4, 1, 1_000_000.0),
                )
            },
            Self::RedSlime => EnemySpec {
                behavior: Behavior::Burst(ProjectileKind::Red),
                ..spec(
                    tex::REDSLIME,
                    (6, 2),
                    (64.0, 48.0),
                    1.5,
                    (4, 0.175),
                    Some((6, 0.065)),
                    (9, 1, 1_000_000.0),
                )
            },
            Self::DemonEye => EnemySpec {
                min_floor: 3,
                ..spec(
                    tex::DEMONEYE,
                    (5, 2),
                    (76.0, 64.0),
                    1.5,
                    (4, 0.1),
                    Some((5, 0.1)),
                    (11, 3, 14_500_000.0),
                )
            },
            Self::RedSpiderling => spiderling(tex::SPIDERLING, 2, (3, 2, 5_500_000.0)),
            Self::GreenSpiderling => EnemySpec {
                behavior: Behavior::HealsKin,
                ..spiderling(tex::GREEN_SPIDERLING, 2, (5, 2, 5_500_000.0))
            },
            Self::PurpleSpiderling => spiderling(tex::PURPLE_SPIDERLING, 3, (3, 1, 6_500_000.0)),
            Self::BloatedSpider => EnemySpec {
                behavior: Behavior::ToxicBurst,
                kin: false,
                ..spiderling(tex::BLOATED_SPIDER, 2, (5, 1, 4_800_000.0))
            },
            Self::StiltSpider => EnemySpec {
                behavior: Behavior::Leaps,
                death_row: 1,
                kin: false,
                min_floor: 1,
                themes: &[Theme::Spider],
                ..spec(
                    tex::STILT_SPIDER,
                    (5, 5),
                    (40.0, 32.0),
                    3.0,
                    (5, 0.14),
                    Some((3, 0.18)),
                    (7, 1, 3_600_000.0),
                )
            },
            Self::Spider => EnemySpec {
                death_row: 2,
                themes: &[Theme::Spider],
                role: Role::Boss("Spider king"),
                behavior: Behavior::SpiderBoss,
                ..spec(
                    tex::SPIDER,
                    (5, 3),
                    (64.0, 40.0),
                    3.0,
                    (5, 0.12),
                    Some((3, 0.15)),
                    (75, 2, 32_500_000.0),
                )
            },
            Self::KingSlime => EnemySpec {
                themes: &[Theme::Moss],
                role: Role::Boss("King slime"),
                behavior: Behavior::KingSlimeBoss,
                ..spec(
                    tex::KINGSLIME,
                    (6, 2),
                    (64.0, 48.0),
                    4.0,
                    (4, 0.175),
                    Some((6, 0.065)),
                    (90, 2, 10_000_000.0),
                )
            },
            Self::Golem => EnemySpec {
                death_row: 7,
                themes: &[Theme::Stone],
                role: Role::Boss("Stone golem"),
                behavior: Behavior::GolemBoss,
                ..spec(
                    tex::GOLEM,
                    (14, 8),
                    (100.0, 100.0),
                    3.0,
                    (4, 0.175),
                    Some((14, 0.09)),
                    (110, 2, 17_000_000.0),
                )
            },
            Self::Argus => EnemySpec {
                death_row: 3,
                themes: &[Theme::Ice],
                role: Role::Boss("Argus"),
                behavior: Behavior::ArgusBoss,
                ..spec(
                    tex::ARGUS,
                    (8, 4),
                    (64.0, 64.0),
                    4.0,
                    (6, 0.14),
                    Some((8, 0.1)),
                    (120, 2, 2_500_000.0),
                )
            },
            Self::SalivaSlime => EnemySpec {
                death_row: 1,
                reform_row: Some(3),
                role: Role::Summoned,
                behavior: Behavior::Hops,
                ..spec(
                    tex::SALIVA_SLIME,
                    (6, 4),
                    (32.0, 32.0),
                    2.8,
                    (4, 0.16),
                    Some((6, 0.07)),
                    (5, 1, 5_500_000.0),
                )
            },
            Self::Mimic => EnemySpec {
                death_row: 5,
                themes: &[Theme::Temple],
                role: Role::Boss("Mimic"),
                behavior: Behavior::MimicBoss,
                ..spec(
                    tex::MIMIC,
                    (8, 6),
                    (92.0, 102.0),
                    2.2,
                    (8, 0.12),
                    Some((8, 0.12)),
                    (115, 2, 30_000_000.0),
                )
            },
            Self::Hellhound => EnemySpec {
                death_row: 4,
                themes: &[Theme::Lava],
                role: Role::Boss("Hellhound"),
                behavior: Behavior::HoundBoss,
                ..spec(
                    tex::HELLHOUND,
                    (8, 5),
                    (128.0, 84.0),
                    2.4,
                    (8, 0.1),
                    Some((8, 0.1)),
                    (110, 2, 75_000_000.0),
                )
            },
            Self::Ogre => EnemySpec {
                death_row: 5,
                themes: &[Theme::Sewer],
                role: Role::Boss("Sewer ogre"),
                behavior: Behavior::OgreBoss,
                ..spec(
                    tex::OGRE,
                    (8, 7),
                    (80.0, 80.0),
                    3.5,
                    (8, 0.11),
                    Some((8, 0.12)),
                    (130, 2, 15_000_000.0),
                )
            },
            Self::ArgusEye => EnemySpec {
                death_row: 1,
                role: Role::Summoned,
                ..spec(
                    tex::ARGUS_ORBIT,
                    (4, 2),
                    (24.0, 24.0),
                    3.0,
                    (4, 0.2),
                    Some((4, 0.1)),
                    (4, 1, 0.0),
                )
            },
            Self::MiniGolem => EnemySpec {
                role: Role::Summoned,
                ..spec(
                    tex::MINI_GOLEM,
                    (6, 2),
                    (32.0, 32.0),
                    3.0,
                    (4, 0.16),
                    Some((6, 0.11)),
                    (6, 1, 9_000_000.0),
                )
            },
            Self::EyeWatcher => EnemySpec {
                min_floor: 3,
                death_row: 2,
                behavior: Behavior::Shoots(ProjectileKind::EyeWatcher),
                ..spec(
                    tex::EYE_WATCHER,
                    (5, 3),
                    (32.0, 32.0),
                    3.0,
                    (4, 0.1),
                    Some((5, 0.085)),
                    (4, 1, 0.0),
                )
            },
        }
    }

    pub(crate) fn available_on(self, floor: u32) -> bool {
        floor >= self.spec().min_floor
    }

    pub(crate) fn available_in(self, floor: u32, theme: Theme) -> bool {
        self.available_on(floor) && {
            let themes = self.spec().themes;
            themes.is_empty() || themes.contains(&theme)
        }
    }

    pub(crate) fn is_boss(self) -> bool {
        matches!(self.spec().role, Role::Boss(_))
    }
}

pub(crate) const LEAP_FIRST_COOLDOWN: f32 = 1.4;
pub(crate) const LEAP_COOLDOWN: f32 = 2.4;
pub(crate) const LEAP_MIN_DISTANCE: f32 = 150.0;
pub(crate) const LEAP_MAX_DISTANCE: f32 = 420.0;
pub(crate) const LEAP_CROUCH_SECONDS: f32 = 0.5;
pub(crate) const LEAP_AIR_SECONDS: f32 = 0.5;
pub(crate) const LEAP_REST_SECONDS: f32 = 0.8;
pub(crate) const LEAP_LANDING_RADIUS: f32 = 112.0;

pub(crate) fn crouch_clip() -> Clip {
    Clip {
        row: 2,
        frames: 2,
        frame_time: LEAP_CROUCH_SECONDS / 2.0,
        looping: false,
        flipped: false,
    }
}

pub(crate) fn air_clip() -> Clip {
    Clip {
        row: 3,
        frames: 1,
        frame_time: LEAP_AIR_SECONDS,
        looping: true,
        flipped: false,
    }
}

pub(crate) fn landing_clip() -> Clip {
    Clip {
        row: 4,
        frames: 2,
        frame_time: LEAP_REST_SECONDS / 2.0,
        looping: false,
        flipped: false,
    }
}

pub(crate) fn spell_clip() -> Clip {
    Clip {
        row: 1,
        frames: 3,
        frame_time: 0.15,
        looping: false,
        flipped: false,
    }
}

pub(crate) fn attack_clip() -> Clip {
    Clip {
        row: 1,
        frames: 5,
        frame_time: 0.075,
        looping: false,
        flipped: false,
    }
}

pub(crate) fn pick(floor: u32, theme: Theme, rng: &mut Rng) -> EnemyKind {
    pick_where(floor, theme, rng, |kind| {
        !kind.is_boss() && kind.spec().role != Role::Summoned
    })
}

const BOSS_VAR: &str = "BANE_BOSS";

fn boss_named(name: &str) -> Option<EnemyKind> {
    match name.trim().to_ascii_lowercase().as_str() {
        "golem" => Some(EnemyKind::Golem),
        "argus" => Some(EnemyKind::Argus),
        "ogre" => Some(EnemyKind::Ogre),
        "hound" | "hellhound" => Some(EnemyKind::Hellhound),
        "mimic" => Some(EnemyKind::Mimic),
        "spider" => Some(EnemyKind::Spider),
        "slime" | "kingslime" => Some(EnemyKind::KingSlime),
        _ => None,
    }
}

pub(crate) fn pick_boss(floor: u32, theme: Theme, rng: &mut Rng) -> EnemyKind {
    if let Some(kind) = std::env::var(BOSS_VAR)
        .ok()
        .and_then(|name| boss_named(&name))
    {
        return kind;
    }
    pick_where(floor, theme, rng, EnemyKind::is_boss)
}

fn pick_where(
    floor: u32,
    theme: Theme,
    rng: &mut Rng,
    wanted: impl Fn(EnemyKind) -> bool,
) -> EnemyKind {
    let pool: Vec<EnemyKind> = EnemyKind::ALL
        .into_iter()
        .filter(|kind| kind.available_in(floor, theme) && wanted(*kind))
        .collect();
    pool[rng.below(pool.len())]
}

pub(crate) fn health_multiplier(floor: u32) -> f32 {
    1.0 + HEALTH_GROWTH_PER_FLOOR * floor as f32
}

pub(crate) fn damage_multiplier(floor: u32) -> f32 {
    1.0 + DAMAGE_GROWTH_PER_FLOOR * floor as f32
}

pub(crate) fn scale_stat(base: i32, multiplier: f32) -> i32 {
    ((base as f32 * multiplier).round() as i32).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_zero_is_neutral_and_later_floors_grow() {
        assert_eq!(scale_stat(12, health_multiplier(0)), 12);
        assert_eq!(scale_stat(12, health_multiplier(5)), 19);
        assert_eq!(scale_stat(2, damage_multiplier(5)), 3);
        assert_eq!(scale_stat(1, 0.1), 1);
    }

    #[test]
    fn tougher_enemies_appear_only_on_later_floors() {
        assert!(EnemyKind::Rat.available_on(0));
        assert!(EnemyKind::RedSpiderling.available_on(1));
        assert!(EnemyKind::RedSpiderling.available_in(1, Theme::Spider));
        assert!(!EnemyKind::RedSpiderling.available_in(1, Theme::Stone));
        assert!(!EnemyKind::BloatedSpider.available_in(3, Theme::Lava));
        assert!(EnemyKind::EmberRat.available_in(1, Theme::Lava));
        assert!(EnemyKind::Rat.available_in(1, Theme::Stone));
        assert!(!EnemyKind::DemonEye.available_on(2));
        assert!(EnemyKind::DemonEye.available_on(3));
        assert!(!EnemyKind::EyeWatcher.available_on(2));
        assert!(EnemyKind::Slime.available_on(0));
    }

    #[test]
    fn picking_respects_the_floor() {
        let mut rng = Rng::new(7);
        for _ in 0..200 {
            let kind = pick(0, Theme::Stone, &mut rng);
            assert!(kind.available_in(0, Theme::Stone), "{kind:?}");
        }
        let mut spider_floor = std::collections::HashSet::new();
        let mut stone_floor = std::collections::HashSet::new();
        for _ in 0..1500 {
            spider_floor.insert(pick(5, Theme::Spider, &mut rng));
            stone_floor.insert(pick(5, Theme::Stone, &mut rng));
        }
        // The ice (frost rat, icicle bat, snow slime, slimeling) and lava (ember
        // rat, flame bat, magma slime, lava worm) enemies live only in their biome.
        let biome_only = 8;
        assert_eq!(spider_floor.len(), EnemyKind::ALL.len() - 10 - biome_only);
        assert_eq!(stone_floor.len(), EnemyKind::ALL.len() - 10 - 5 - biome_only);
        assert!(!stone_floor.contains(&EnemyKind::BloatedSpider));
        assert!(!stone_floor.contains(&EnemyKind::StiltSpider));
        assert!(spider_floor.contains(&EnemyKind::StiltSpider));
        assert!(!stone_floor.contains(&EnemyKind::MiniGolem));
    }

    #[test]
    fn speed_follows_force_over_body_area() {
        assert!((EnemyKind::Rat.spec().speed() - 186.0).abs() < 1.0);
        assert!((EnemyKind::Bat.spec().speed() - 314.0).abs() < 2.0);
        assert!((EnemyKind::Slime.spec().speed() - 29.0).abs() < 1.0);
        assert_eq!(EnemyKind::EyeWatcher.spec().speed(), 0.0);
    }

    #[test]
    fn projectile_speeds_follow_the_single_step_impulse() {
        let slime = ProjectileKind::Green.spec().speed();
        let eye = ProjectileKind::EyeWatcher.spec().speed();
        assert!((slime - 325.0).abs() < 2.0, "{slime}");
        assert!((eye - 253.0).abs() < 2.0, "{eye}");
    }

    #[test]
    fn every_enemy_has_a_death_animation_that_fits_inside_its_sheet() {
        for kind in EnemyKind::ALL {
            let spec = kind.spec();
            let (frames, _) = spec
                .death
                .unwrap_or_else(|| panic!("{kind:?} has no death"));
            assert!(spec.death_row < spec.rows, "{kind:?}");
            assert!(frames <= spec.columns, "{kind:?}");
            let clip = spec.death_clip();
            assert!(!clip.looping, "{kind:?}");
            assert_ne!(clip.row, 0, "{kind:?} would play its idle row");
        }
    }

    const THEMES: [Theme; 7] = [
        Theme::Stone,
        Theme::Sewer,
        Theme::Moss,
        Theme::Spider,
        Theme::Ice,
        Theme::Lava,
        Theme::Temple,
    ];

    fn own_boss(theme: Theme) -> EnemyKind {
        match theme {
            Theme::Spider => EnemyKind::Spider,
            Theme::Stone => EnemyKind::Golem,
            Theme::Sewer => EnemyKind::Ogre,
            Theme::Lava => EnemyKind::Hellhound,
            Theme::Moss => EnemyKind::KingSlime,
            Theme::Ice => EnemyKind::Argus,
            Theme::Temple => EnemyKind::Mimic,
        }
    }

    #[test]
    fn every_theme_always_brings_its_own_boss_on_any_floor() {
        let mut rng = Rng::new(3);
        for theme in THEMES {
            for floor in 1..=9 {
                for _ in 0..10 {
                    assert_eq!(pick_boss(floor, theme, &mut rng), own_boss(theme), "{theme:?}");
                }
            }
        }
    }

    #[test]
    fn no_boss_is_shared_between_themes() {
        let bosses: std::collections::HashSet<EnemyKind> =
            THEMES.into_iter().map(own_boss).collect();
        assert_eq!(bosses.len(), THEMES.len());
        for kind in EnemyKind::ALL.into_iter().filter(|kind| kind.is_boss()) {
            assert_eq!(kind.spec().themes.len(), 1, "{kind:?}");
        }
    }

    #[test]
    fn every_theme_has_common_enemies_already_on_the_first_floor() {
        for theme in THEMES {
            assert!(
                EnemyKind::ALL.into_iter().any(|kind| !kind.is_boss()
                    && kind.spec().role != Role::Summoned
                    && kind.available_in(1, theme)),
                "{theme:?}"
            );
        }
    }

    #[test]
    fn boss_names_for_previews_are_parsed() {
        assert_eq!(boss_named(" Golem "), Some(EnemyKind::Golem));
        assert_eq!(boss_named("slime"), Some(EnemyKind::KingSlime));
        assert_eq!(boss_named("argus"), Some(EnemyKind::Argus));
        assert_eq!(boss_named("ogre"), Some(EnemyKind::Ogre));
        assert_eq!(boss_named("hound"), Some(EnemyKind::Hellhound));
        assert_eq!(boss_named("mimic"), Some(EnemyKind::Mimic));
        assert_eq!(boss_named("nothing"), None);
    }

    #[test]
    fn bosses_never_appear_among_common_enemies() {
        let mut rng = Rng::new(11);
        for _ in 0..500 {
            assert!(!pick(1, Theme::Spider, &mut rng).is_boss());
        }
    }

    #[test]
    fn the_bloated_spider_bursts_into_a_cloud_when_it_dies() {
        assert_eq!(
            EnemyKind::BloatedSpider.spec().behavior,
            Behavior::ToxicBurst
        );
        assert_eq!(EnemyKind::BloatedSpider.spec().themes, &[Theme::Spider]);
    }

    #[test]
    fn the_spider_dies_on_its_third_row() {
        assert_eq!(EnemyKind::Spider.spec().death_clip().row, 2);
        assert!((EnemyKind::Spider.spec().speed() - 282.0).abs() < 1.0);
        assert!((EnemyKind::KingSlime.spec().speed() - 40.6).abs() < 0.5);
    }

    #[test]
    fn only_spiderlings_count_as_kin() {
        let kin: Vec<EnemyKind> = EnemyKind::ALL
            .into_iter()
            .filter(|kind| kind.spec().kin)
            .collect();
        assert_eq!(
            kin,
            vec![
                EnemyKind::RedSpiderling,
                EnemyKind::GreenSpiderling,
                EnemyKind::PurpleSpiderling
            ]
        );
    }
}
