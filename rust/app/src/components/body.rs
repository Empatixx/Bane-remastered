use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Body {
    pub(crate) body: RigidBodyHandle,
    pub(crate) collider: ColliderHandle,
}
