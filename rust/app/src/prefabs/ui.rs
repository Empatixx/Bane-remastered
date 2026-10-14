use crate::assets::tex;
use crate::palette;
use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::components::{
    Button, Color, DeathPart, Hidden, Hitbox, Layer, OnClick, Overlay, PauseUi, Position,
    ScreenSpace, Size, SpeechBubble, SpeechPart, SpeechText, Sprite, Text, TextButton, UiAction,
};
use crate::render::{Align, FontSize, Fonts, TextureId};
use crate::viewport::TARGET;

const DEATH_ICON: f32 = 32.0;
const SKULL_SIZE: Vec2 = Vec2::new(102.0, 96.0);
const TITLE_BASELINE: f32 = 175.0;
const NOTE_BASELINE: f32 = 450.0;
const PROMPT_POSITION: Vec2 = Vec2::new(700.0, 500.0);

const BUTTON_SIZE: Vec2 = Vec2::new(126.0, 63.0);
const BUTTON_TEXT_COLOR: Vec4 = palette::RUST;

pub(crate) fn button(
    world: &mut World,
    position: Vec2,
    label: &str,
    action: UiAction,
    texture: TextureId,
) -> (Entity, Entity) {
    let text = world.spawn((
        Text::centered(label, BUTTON_TEXT_COLOR, FontSize::Size27),
        Position(position),
        Layer(2),
    ));
    let button = world.spawn((
        Button {
            idle_uv: Sprite::cell_uv(0, 0, 1, 2),
            hover_uv: Sprite::cell_uv(0, 1, 1, 2),
        },
        OnClick(action),
        Position(position),
        Size(BUTTON_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(texture, 0, 0, 1, 2),
        Layer(1),
    ));
    (button, text)
}

const MINIMAL_BACK: Vec4 = Vec4::new(0.07, 0.06, 0.09, 0.85);

const MINIMAL_EDGE: Vec4 = Vec4::new(0.78, 0.36, 0.07, 0.9);

const MINIMAL_TEXT: Vec4 = Vec4::new(0.95, 0.9, 0.8, 1.0);
const MINIMAL_RISE: f32 = 26.0;

pub(crate) fn minimal_speech(world: &mut World, head: Vec2) {
    let anchor = head - Vec2::new(0.0, MINIMAL_RISE);
    world.spawn((
        SpeechBubble {
            remaining: 0.0,
            message: "",
        },
        Position(anchor),
        Size(Vec2::ZERO),
        Color(MINIMAL_BACK),
        Layer(7),
        Hidden,
    ));
    for part in [SpeechPart::Frame, SpeechPart::Tail] {
        world.spawn((
            part,
            Position(anchor),
            Size(Vec2::ZERO),
            Color(MINIMAL_EDGE),
            Layer(6),
            Hidden,
        ));
    }
    world.spawn((
        SpeechText,
        Position(anchor),
        Text::centered(String::new(), MINIMAL_TEXT, FontSize::Size22),
        Layer(8),
        Hidden,
    ));
}

pub(crate) fn death_ui(world: &mut World, victory: bool, textures: &Textures) {
    let overlay = |part, position: Vec2, size: Vec2, texture, layer| {
        (
            part,
            Position(position),
            Size(size),
            Color(Vec4::ONE),
            Sprite::whole(texture),
            Layer(layer),
            Overlay,
            ScreenSpace,
            Hidden,
        )
    };
    world.spawn((
        DeathPart::Dim,
        Position(TARGET / 2.0),
        Size(TARGET),
        Color(Vec4::new(0.0, 0.0, 0.0, 0.0)),
        Sprite::whole(TextureId::WHITE),
        Layer(20),
        Overlay,
        ScreenSpace,
    ));
    world.spawn(overlay(
        DeathPart::Skull,
        Vec2::ZERO,
        SKULL_SIZE,
        textures.get(tex::SKULL),
        21,
    ));
    let icons = [
        tex::ACCURACY_ICON,
        tex::KILLS_ICON,
        tex::LOOT_ICON,
        tex::FLOOR_ICON,
    ];
    for (index, icon) in icons.into_iter().enumerate() {
        world.spawn(overlay(
            DeathPart::Icon(index),
            Vec2::ZERO,
            Vec2::splat(DEATH_ICON),
            textures.get(icon),
            21,
        ));
        world.spawn((
            DeathPart::Label(index),
            Position(Vec2::ZERO),
            Size(Vec2::ONE),
            Color(Vec4::ONE),
            Text::new(String::new(), Vec4::ONE, FontSize::Size22),
            Layer(22),
            Overlay,
            ScreenSpace,
            Hidden,
        ));
        world.spawn((
            DeathPart::Reward(index),
            Position(Vec2::ZERO),
            Size(Vec2::ONE),
            Color(Vec4::ONE),
            Text::new(
                String::new(),
                Vec4::new(1.0, 216.0 / 255.0, 0.0, 1.0),
                FontSize::Size22,
            ),
            Layer(22),
            Overlay,
            ScreenSpace,
            Hidden,
        ));
    }
    world.spawn((
        DeathPart::Title,
        Position(Vec2::new(TARGET.x / 2.0, TITLE_BASELINE)),
        Size(Vec2::ONE),
        Color(Vec4::ONE),
        Text::new(
            if victory { "Victory!" } else { "Game Over" }.to_owned(),
            Vec4::ONE,
            FontSize::Size32,
        )
        .aligned(Align::Center),
        Layer(22),
        Overlay,
        ScreenSpace,
        Hidden,
    ));
    world.spawn((
        DeathPart::Note,
        Position(Vec2::new(TARGET.x / 2.0, NOTE_BASELINE)),
        Size(Vec2::ONE),
        Color(Vec4::ONE),
        Text::new(
            String::new(),
            Vec4::new(1.0, 216.0 / 255.0, 0.0, 1.0),
            FontSize::Size22,
        )
        .aligned(Align::Center),
        Layer(22),
        Overlay,
        ScreenSpace,
        Hidden,
    ));
    world.spawn((
        DeathPart::Prompt,
        Position(PROMPT_POSITION),
        Size(Vec2::ONE),
        Color(Vec4::ONE),
        Text::new(
            "Press any key to continue...".to_owned(),
            Vec4::ONE,
            FontSize::Size22,
        ),
        Layer(22),
        Overlay,
        ScreenSpace,
        Hidden,
    ));
}

const PAUSE_BACKGROUND_SIZE: Vec2 = Vec2::new(240.0, 300.0);
const PAUSE_TEXT_COLOR: Vec4 = palette::ORANGE;

const PAUSE_HOVER_COLOR: Vec4 = Vec4::new(1.0, 1.0, 0.0, 1.0);

pub(crate) const PAUSE_GAME: [(&str, UiAction); 3] = [
    ("Resume", UiAction::Resume),
    ("Die", UiAction::Die),
    ("Exit Game", UiAction::ExitGame),
];

pub(crate) const PAUSE_LOBBY: [(&str, UiAction); 3] = [
    ("Resume", UiAction::Resume),
    ("Main Menu", UiAction::LeaveToMenu),
    ("Exit Game", UiAction::ExitGame),
];

const PAUSE_CENTER: f32 = 275.0;
const PAUSE_STEP: f32 = 50.0;

pub(crate) fn pause_ui(
    world: &mut World,
    textures: &Textures,
    fonts: &Fonts,
    buttons: &[(&str, UiAction)],
) {
    world.spawn((
        PauseUi,
        Overlay,
        Position(Vec2::new(480.0, 270.0)),
        Size(PAUSE_BACKGROUND_SIZE),
        Color(Vec4::ONE),
        Sprite::whole(textures.get(tex::PAUSEBG)),
        Layer(8),
        ScreenSpace,
    ));
    let line = fonts.get(FontSize::Size22);
    let middle = (buttons.len() as f32 - 1.0) / 2.0;
    for (index, (label, action)) in buttons.iter().copied().enumerate() {
        let baseline = PAUSE_CENTER + PAUSE_STEP * (index as f32 - middle);
        let width = line.width(label);
        let height = line.line_height();
        world.spawn((
            PauseUi,
            Overlay,
            OnClick(action),
            TextButton {
                idle: PAUSE_TEXT_COLOR,
                hover: PAUSE_HOVER_COLOR,
            },
            Hitbox {
                offset: Vec2::new(-5.0, -height / 2.0),
                size: Vec2::new(width + 10.0, height + 20.0),
            },
            Text::new(label.to_owned(), PAUSE_TEXT_COLOR, FontSize::Size22).aligned(Align::Center),
            Position(Vec2::new(480.0, baseline)),
            Layer(9),
            ScreenSpace,
        ));
    }
}
