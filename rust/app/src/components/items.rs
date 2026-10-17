use crate::items::ItemKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChestKind {
    Weapon,
    Artefact,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Chest {
    pub(crate) kind: ChestKind,
    pub(crate) opened: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Pickup(pub(crate) ItemKind);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Despawn {
    pub(crate) remaining: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct NoDrop;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Currency {
    Coins,
    Health,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Shop {
    pub(crate) price: i32,
    pub(crate) currency: Currency,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PriceBox(pub(crate) hecs::Entity);

#[derive(Debug, Clone, Copy)]
pub(crate) struct PriceDigits(pub(crate) hecs::Entity);

#[derive(Debug, Clone, Copy)]
pub(crate) struct PriceTag {
    pub(crate) owner: hecs::Entity,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Candle;

#[derive(Debug, Clone)]
pub(crate) struct SpeechBubble {
    pub(crate) remaining: f32,
    pub(crate) message: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SpeechText;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpeechPart {
    Frame,
    Tail,
}

/// Where a talker's words show: the speech bubble moves over this point when it speaks.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Speaker {
    pub(crate) anchor: glam::Vec2,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Chatty {
    pub(crate) cooldown: f32,
    pub(crate) line: usize,
    pub(crate) greeted: bool,
}
