use glam::{Vec2, Vec4};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Position(pub(crate) Vec2);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Size(pub(crate) Vec2);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Color(pub(crate) Vec4);
