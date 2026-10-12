use glam::{Vec2, Vec4};

use crate::assets::tex;
use crate::palette;
use crate::render::{Align, FontSize};

use super::upgrade_menu::{Element, rect};

pub(super) const BOARD_TOP_LEFT: Vec2 = Vec2::new(180.0, 20.0);
pub(super) const BOARD_SIZE: Vec2 = Vec2::new(600.0, 501.0);
pub(super) const BOARD_CENTER_X: f32 = BOARD_TOP_LEFT.x + BOARD_SIZE.x / 2.0;
const BOARD_MARGIN: f32 = 18.0;
const DIM_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.55);
const FILL_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.5);
const TRACK_COLOR: Vec4 = Vec4::new(0.1, 0.07, 0.1, 0.9);
const DIM_LAYER: i32 = 29;
const BOARD_LAYER: i32 = 30;
const CARD_LAYER: i32 = 31;
const BAR_LAYER: i32 = 33;

pub(super) const ORANGE: Vec4 = palette::ORANGE;
pub(super) const SALMON: Vec4 = palette::SALMON;
pub(super) const YELLOW: Vec4 = Vec4::new(1.0, 0.91, 0.43, 1.0);
pub(super) const GRAY: Vec4 = Vec4::new(112.0 / 255.0, 100.0 / 255.0, 100.0 / 255.0, 1.0);
pub(super) const GOLD: Vec4 = Vec4::new(1.0, 0.84, 0.38, 1.0);
pub(super) const ACCENT: Vec4 = Vec4::new(244.0 / 255.0, 110.0 / 255.0, 110.0 / 255.0, 1.0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ButtonStyle {
    Normal,
    Hover,
    Disabled,
}

pub(super) fn frame() -> Vec<Element> {
    vec![
        rect(Vec2::ZERO, crate::viewport::TARGET, DIM_COLOR, DIM_LAYER),
        Element::Image {
            texture: tex::TALENT_BOARD,
            top_left: BOARD_TOP_LEFT - Vec2::splat(BOARD_MARGIN),
            size: BOARD_SIZE + Vec2::splat(BOARD_MARGIN * 2.0),
            cell: None,
            layer: BOARD_LAYER,
            alpha: 1.0,
        },
    ]
}

pub(super) fn label(
    content: impl Into<String>,
    position: Vec2,
    color: Vec4,
    size: FontSize,
    align: Align,
) -> Element {
    Element::Label {
        content: content.into(),
        position,
        color,
        size,
        align,
    }
}

pub(super) fn card(top_left: Vec2, size: Vec2, rim: Vec4, filled: bool) -> Vec<Element> {
    let inner = if filled {
        Vec4::new(rim.x * 0.25, rim.y * 0.18, rim.z * 0.12, 0.7)
    } else {
        FILL_COLOR
    };
    vec![
        rect(top_left, size, rim, CARD_LAYER),
        rect(
            top_left + Vec2::splat(2.0),
            size - Vec2::splat(4.0),
            inner,
            CARD_LAYER + 1,
        ),
    ]
}

pub(super) fn bar(top_left: Vec2, size: Vec2, fraction: f32, fill: Vec4) -> Vec<Element> {
    let width = (size.x * fraction.clamp(0.0, 1.0)).floor();
    let mut elements = vec![rect(top_left, size, TRACK_COLOR, BAR_LAYER)];
    if width > 0.0 {
        elements.push(rect(
            top_left,
            Vec2::new(width, size.y),
            fill,
            BAR_LAYER + 1,
        ));
    }
    elements
}

pub(super) fn button(
    top_left: Vec2,
    size: Vec2,
    text: &str,
    style: ButtonStyle,
    font: FontSize,
) -> Vec<Element> {
    let (rim, ink) = match style {
        ButtonStyle::Normal => (SALMON, ORANGE),
        ButtonStyle::Hover => (YELLOW, YELLOW),
        ButtonStyle::Disabled => (GRAY, GRAY),
    };
    let mut elements = card(top_left, size, rim, false);
    let baseline = top_left.y
        + size.y / 2.0
        + match font {
            FontSize::Size16 => 5.0,
            FontSize::Size22 => 7.0,
            _ => 9.0,
        };
    elements.push(label(
        text,
        Vec2::new(top_left.x + size.x / 2.0, baseline),
        ink,
        font,
        Align::Center,
    ));
    elements
}

pub(super) fn contains(top_left: Vec2, size: Vec2, point: Vec2) -> bool {
    point.cmpge(top_left).all() && point.cmple(top_left + size).all()
}

pub(super) fn footer() -> Element {
    label(
        "Esc  close",
        Vec2::new(BOARD_CENTER_X, BOARD_TOP_LEFT.y + BOARD_SIZE.y - 10.0),
        GRAY,
        FontSize::Size16,
        Align::Center,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bar_fills_by_its_fraction_and_stays_inside_its_track() {
        let size = Vec2::new(100.0, 8.0);
        let widths = |fraction: f32| -> Vec<f32> {
            bar(Vec2::ZERO, size, fraction, ORANGE)
                .iter()
                .filter_map(|element| match element {
                    Element::Rect { size, layer, .. } if *layer == BAR_LAYER + 1 => Some(size.x),
                    _ => None,
                })
                .collect()
        };

        assert!(widths(0.0).is_empty());
        assert_eq!(widths(0.4), vec![40.0]);
        assert_eq!(widths(7.0), vec![100.0]);
    }

    #[test]
    fn buttons_change_their_rim_and_text_with_the_style() {
        let colour_of = |style: ButtonStyle| {
            button(
                Vec2::ZERO,
                Vec2::new(100.0, 30.0),
                "Go",
                style,
                FontSize::Size16,
            )
            .iter()
            .find_map(|element| match element {
                Element::Label { color, .. } => Some(*color),
                _ => None,
            })
        };

        assert_eq!(colour_of(ButtonStyle::Normal), Some(ORANGE));
        assert_eq!(colour_of(ButtonStyle::Hover), Some(YELLOW));
        assert_eq!(colour_of(ButtonStyle::Disabled), Some(GRAY));
    }

    #[test]
    fn the_board_covers_the_screen_with_a_dim_layer_and_one_frame_image() {
        let elements = frame();
        assert_eq!(elements.len(), 2);
        assert!(
            matches!(&elements[1], Element::Image { texture, .. } if *texture == tex::TALENT_BOARD)
        );
    }

    #[test]
    fn contains_includes_the_edges() {
        let at = Vec2::new(10.0, 10.0);
        let size = Vec2::new(20.0, 10.0);
        assert!(contains(at, size, Vec2::new(30.0, 20.0)));
        assert!(!contains(at, size, Vec2::new(31.0, 20.0)));
    }
}
