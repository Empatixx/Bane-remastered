use glam::{Vec2, Vec4};
use hecs::World;

use crate::components::{Layer, Position, Text};
use crate::palette;
use crate::render::{Align, FontSize};

mod actors;
mod fixtures;
mod fx;
mod hud;
mod ui;

pub(crate) use actors::*;
pub(crate) use fixtures::*;
pub(crate) use fx::*;
pub(crate) use hud::*;
pub(crate) use ui::*;

pub(super) const PRICE_TAG_SIZE: f32 = 32.0;

pub(super) const PRICE_COLOR: Vec4 = Vec4::new(212.0 / 255.0, 168.0 / 255.0, 8.0 / 255.0, 1.0);

pub(crate) fn label(world: &mut World, content: &str, position: Vec2, centered: bool) {
    let align = if centered { Align::Center } else { Align::Left };
    world.spawn((
        Position(position),
        Text::new(content.to_owned(), Vec4::ONE, FontSize::Size22).aligned(align),
        Layer(-1),
    ));
}

pub(super) const PROMPT_COLOR: Vec4 = palette::ORANGE;
