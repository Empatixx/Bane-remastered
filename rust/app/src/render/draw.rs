use super::{Quad, TextureId};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Draw {
    pub(crate) texture: TextureId,
    pub(crate) quad: Quad,
}
