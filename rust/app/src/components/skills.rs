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
pub(crate) struct Scorch {
    pub(crate) remaining: f32,
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
    pub(crate) pulse: f32,
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
    pub(crate) rock: hecs::Entity,
    pub(crate) room: Option<crate::components::RoomId>,
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
