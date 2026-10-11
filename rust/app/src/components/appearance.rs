use glam::Vec4;

use crate::render::TextureId;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Color(pub(crate) Vec4);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Sprite(pub(crate) TextureId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Layer(pub(crate) i32);
