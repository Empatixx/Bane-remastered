use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};

#[derive(Debug, Clone, Copy)]
pub(crate) struct BodyOffset(pub(crate) glam::Vec2);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Body {
    pub(crate) body: RigidBodyHandle,
    pub(crate) collider: ColliderHandle,
}

/// A request to put the entity's body at a spot (a blink, an orbit, a bounce off a wall),
/// carried out by `simulate` before it steps. Whoever asks moves `Position` itself, at once.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Teleport(pub(crate) glam::Vec2);

/// A request for where a body moved only by the game (a mine cart on its rail) is next,
/// handed to the physics by `simulate` before it steps; the body gets there over that step.
#[derive(Debug, Clone, Copy)]
pub(crate) struct MoveTo(pub(crate) glam::Vec2);

/// A request to set the velocity of a body whose entity has no `Velocity` (a floating
/// barrel), carried out by `simulate` before it steps. An entity with `Velocity` is driven by
/// that instead, every step.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SetVelocity(pub(crate) glam::Vec2);
