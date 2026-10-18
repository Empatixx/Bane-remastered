use glam::{Vec2, Vec4};

use crate::audio::SoundEffect;
use crate::components::Clip;
use crate::enemies::ProjectileKind;

#[derive(Debug, Clone, Copy)]
pub(crate) struct EnemyBullet;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Impact(pub(crate) Clip);

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Hits(pub(crate) u32);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Smash {
    pub(crate) kind: ProjectileKind,
    pub(crate) count: usize,
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Boss;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Minion;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Rooted;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GolemAttack {
    Slam,
    Throw,
    Rain,
    Laser,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GolemStage {
    Walk,
    Winding,
    Acting,
    Recovering,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArgusAttack {
    Volley,
    Barrage,
    Shield,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArgusStage {
    Float,
    Winding,
    Acting,
    Recovering,
    FadingOut,
    FadingIn,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Charge {
    pub(crate) direction: glam::Vec2,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Magma {
    pub(crate) age: f32,
    pub(crate) tile: (usize, usize),
}

#[derive(Debug, Clone, Default)]
pub(crate) struct MagmaSpreader {
    pub(crate) order: Vec<(usize, usize)>,
    pub(crate) cursor: usize,
    pub(crate) timer: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Hopper {
    pub(crate) timer: f32,
    pub(crate) hopping: bool,
    pub(crate) idle: Clip,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Bait {
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Sealed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MimicAttack {
    Hop,
    Bite,
    Spit,
    Lure,
    Rest,
}

/// The part of a boss's mind every boss has: where it is in its attack cycle, the attack it is
/// on and the one it used last (it never repeats an attack twice in a row), the clocks of the
/// current stage and of the wait before the next attack, its own dice, and the clip it idles in.
#[derive(Debug, Clone, Copy)]
pub(crate) struct BossBrain<Stage, Attack> {
    pub(crate) stage: Stage,
    pub(crate) attack: Attack,
    pub(crate) last: Option<Attack>,
    pub(crate) timer: f32,
    pub(crate) cooldown: f32,
    pub(crate) seed: u32,
    pub(crate) idle: Clip,
}

impl<Stage, Attack> BossBrain<Stage, Attack> {
    /// A boss that has not attacked yet: in `stage` with `attack` as a placeholder, waiting
    /// `cooldown` seconds before its first attack.
    pub(crate) fn new(stage: Stage, attack: Attack, cooldown: f32, seed: u32, idle: Clip) -> Self {
        Self {
            stage,
            attack,
            last: None,
            timer: 0.0,
            cooldown,
            seed,
            idle,
        }
    }
}

/// When and how a boss loses its temper: at or below `at` of its max health it roars `sound`,
/// its chase speed is multiplied by `speed`, and it turns `tint` — none for a boss that colours
/// itself every tick. `tick_enrage` reads it once per tick until the boss is `Enraged`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Enrage {
    pub(crate) at: f32,
    pub(crate) speed: f32,
    pub(crate) sound: SoundEffect,
    pub(crate) tint: Option<Vec4>,
}

/// A boss that has lost its temper. Set by `tick_enrage`, it stays for the rest of the boss's
/// life — a state, not a one-tick event, so nothing clears it.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Enraged;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MimicStage {
    Walk,
    Winding,
    Acting,
    Recovering,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct MimicBoss {
    pub(crate) brain: BossBrain<MimicStage, MimicAttack>,
    pub(crate) charging: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HoundAttack {
    Lunge,
    Breath,
    Howl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HoundStage {
    Prowl,
    Winding,
    Acting,
    Recovering,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct HoundBoss {
    pub(crate) brain: BossBrain<HoundStage, HoundAttack>,
    pub(crate) angles: [f32; 3],
    pub(crate) charging: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OgreAttack {
    Jump,
    Throw,
    Slam,
    Roar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OgreStage {
    Walk,
    Winding,
    Recovering,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OgreBoss {
    pub(crate) brain: BossBrain<OgreStage, OgreAttack>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Orbiter {
    pub(crate) owner: hecs::Entity,
    pub(crate) angle: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ArgusBoss {
    pub(crate) brain: BossBrain<ArgusStage, ArgusAttack>,
    pub(crate) pulse: f32,
    pub(crate) pulses: u32,
    pub(crate) angle: f32,
    pub(crate) attacks: u32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct GolemBoss {
    pub(crate) brain: BossBrain<GolemStage, GolemAttack>,
    pub(crate) pulse: f32,
    pub(crate) rain_dropped: u32,
    pub(crate) second_ring: f32,
    pub(crate) beam_angle: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct RockFall {
    pub(crate) remaining: f32,
    /// The whole fall, for placing the rock on its way down.
    pub(crate) duration: f32,
    pub(crate) rock: hecs::Entity,
    pub(crate) room: Option<crate::components::RoomId>,
    pub(crate) kind: FallKind,
    /// Where a lobbed shot starts its arc; a dropped one falls straight down.
    pub(crate) from: Option<glam::Vec2>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FallKind {
    Boulder,
    Icicle,
    Magma,
}

/// An ember rat: once hit it is lit and rushes the player.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Kindling {
    pub(crate) lit: bool,
    pub(crate) base_speed: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SwoopStage {
    Circle,
    Windup,
    Dive,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Swooper {
    pub(crate) stage: SwoopStage,
    pub(crate) timer: f32,
    pub(crate) base_speed: f32,
}

/// Leaves a heap of ash that rises again as a new flame bat.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DeathAsh;

/// The heap of ash a flame bat leaves; shoot it before it rises.
#[derive(Debug, Clone, Copy)]
pub(crate) struct AshPile {
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct MagmaShell {
    pub(crate) crusted: bool,
    pub(crate) timer: f32,
    pub(crate) mortar: f32,
    pub(crate) base_speed: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WormStage {
    /// Gone below; nothing to see or hit.
    Hidden,
    /// Its hole glows where it is about to come up.
    Showing,
    Rising,
    /// Up and spitting magma; the only time it can be hurt.
    Up,
    Sinking,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Burrower {
    pub(crate) stage: WormStage,
    pub(crate) timer: f32,
    pub(crate) shot: f32,
    pub(crate) shots_left: u32,
    pub(crate) damage: i32,
    /// How many times it came up, to vary where it comes up next.
    pub(crate) emerged: u32,
}

/// Damage from this (a bite, a snowball) also chills the player.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Chills;

/// Breaks into smaller enemies when it dies.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DeathSplit {
    pub(crate) kind: crate::enemies::EnemyKind,
    pub(crate) count: u32,
}

/// An icicle bat's wait until it drops the next icicle.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IcicleDropper {
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FallingRock;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BeamPart;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LeapStage {
    Stalk,
    Crouch,
    Airborne,
    Rest,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Leaper {
    pub(crate) stage: LeapStage,
    pub(crate) timer: f32,
    pub(crate) target: Vec2,
    pub(crate) damage: i32,
    pub(crate) idle: Clip,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Hatcher {
    pub(crate) remaining: f32,
    pub(crate) next: usize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct WebSpitter {
    pub(crate) remaining: f32,
    pub(crate) casting: bool,
    pub(crate) idle: Clip,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Cobweb;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Drag {
    pub(crate) rate: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Armageddon {
    pub(crate) kind: ProjectileKind,
    pub(crate) remaining: f32,
    pub(crate) angle: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DeathCloud;

#[derive(Debug, Clone, Copy)]
pub(crate) struct DeathBurst {
    pub(crate) kind: ProjectileKind,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct EyeShooter {
    pub(crate) kind: ProjectileKind,
    pub(crate) idle: Clip,
    pub(crate) remaining: f32,
    pub(crate) attacking: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Homing {
    pub(crate) aim: Vec2,
    pub(crate) speed: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Fuse {
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DeathHeal {
    pub(crate) radius: f32,
    pub(crate) amount: i32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Kin;
