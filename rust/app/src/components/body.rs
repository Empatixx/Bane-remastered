use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};

#[derive(Debug, Clone, Copy)]
pub(crate) struct BodyOffset(pub(crate) glam::Vec2);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Body {
    pub(crate) body: RigidBodyHandle,
    pub(crate) collider: ColliderHandle,
}
