use glam::Vec2;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MimicStage {
    Walk,
    Winding,
    Acting,
    Recovering,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct MimicBoss {
    pub(crate) stage: MimicStage,
    pub(crate) attack: MimicAttack,
    pub(crate) last: Option<MimicAttack>,
    pub(crate) timer: f32,
    pub(crate) cooldown: f32,
    pub(crate) charging: bool,
    pub(crate) enraged: bool,
    pub(crate) seed: u32,
    pub(crate) idle: Clip,
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
    pub(crate) stage: HoundStage,
    pub(crate) attack: HoundAttack,
    pub(crate) last: Option<HoundAttack>,
    pub(crate) timer: f32,
    pub(crate) cooldown: f32,
    pub(crate) angles: [f32; 3],
    pub(crate) charging: bool,
    pub(crate) enraged: bool,
    pub(crate) seed: u32,
    pub(crate) idle: Clip,
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
    pub(crate) stage: OgreStage,
    pub(crate) attack: OgreAttack,
    pub(crate) last: Option<OgreAttack>,
    pub(crate) timer: f32,
    pub(crate) cooldown: f32,
    pub(crate) enraged: bool,
    pub(crate) seed: u32,
    pub(crate) idle: Clip,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Orbiter {
    pub(crate) owner: hecs::Entity,
    pub(crate) angle: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ArgusBoss {
    pub(crate) stage: ArgusStage,
    pub(crate) attack: ArgusAttack,
    pub(crate) last: Option<ArgusAttack>,
    pub(crate) timer: f32,
    pub(crate) cooldown: f32,
    pub(crate) pulse: f32,
    pub(crate) pulses: u32,
    pub(crate) angle: f32,
    pub(crate) attacks: u32,
    pub(crate) enraged: bool,
    pub(crate) seed: u32,
    pub(crate) idle: Clip,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct GolemBoss {
    pub(crate) stage: GolemStage,
    pub(crate) attack: GolemAttack,
    pub(crate) last: Option<GolemAttack>,
    pub(crate) timer: f32,
    pub(crate) cooldown: f32,
    pub(crate) pulse: f32,
    pub(crate) rain_dropped: u32,
    pub(crate) second_ring: f32,
    pub(crate) beam_angle: f32,
    pub(crate) enraged: bool,
    pub(crate) seed: u32,
    pub(crate) idle: Clip,
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
