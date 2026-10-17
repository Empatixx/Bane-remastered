//! How far a shooting enemy keeps from the player: in range it holds its ground, and closer
//! than that it backs off. (Enemies keeping their distance from one another is part of their
//! steering, see `enemies::steer`.)

pub(crate) const SHOOTER_FAR: f32 = 300.0;
pub(crate) const SHOOTER_NEAR: f32 = 200.0;
