use super::{Quad, TextureId};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Draw {
    pub(crate) texture: TextureId,
    pub(crate) quad: Quad,
}

#[derive(Debug, Default)]
pub(crate) struct Layers {
    pub(crate) scene: Vec<Draw>,
    pub(crate) hud: Vec<Draw>,
    pub(crate) overlay: Vec<Draw>,
}
