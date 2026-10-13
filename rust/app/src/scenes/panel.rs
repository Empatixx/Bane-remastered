use glam::{Vec2, Vec4};

use crate::assets::{Tex, tex};
use crate::render::{Align, FontSize};

use super::upgrade_menu::{Element, rect};

pub(super) const BOARD_TOP_LEFT: Vec2 = Vec2::new(180.0, 20.0);
pub(super) const BOARD_SIZE: Vec2 = Vec2::new(600.0, 501.0);
pub(super) const BOARD_CENTER_X: f32 = BOARD_TOP_LEFT.x + BOARD_SIZE.x / 2.0;
const BOARD_MARGIN: f32 = 18.0;
const DIM_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.6);
pub(super) const TRACK_BROWN: Vec4 = Vec4::new(77.0 / 255.0, 53.0 / 255.0, 31.0 / 255.0, 1.0);
const DIM_LAYER: i32 = 29;
const BOARD_LAYER: i32 = 30;
const NOTE_LAYER: i32 = 31;
const BAR_LAYER: i32 = 33;
pub(super) const SCALE: f32 = 4.0;

pub(super) const INK: Vec4 = Vec4::new(58.0 / 255.0, 40.0 / 255.0, 23.0 / 255.0, 1.0);
pub(super) const INK_SOFT: Vec4 = Vec4::new(102.0 / 255.0, 85.0 / 255.0, 63.0 / 255.0, 1.0);
pub(super) const TAB_IDLE: Vec4 = Vec4::new(122.0 / 255.0, 83.0 / 255.0, 51.0 / 255.0, 1.0);
pub(super) const SAND: Vec4 = Vec4::new(189.0 / 255.0, 171.0 / 255.0, 147.0 / 255.0, 1.0);
pub(super) const CREAM: Vec4 = Vec4::new(240.0 / 255.0, 224.0 / 255.0, 180.0 / 255.0, 1.0);
pub(super) const YELLOW: Vec4 = Vec4::new(1.0, 0.91, 0.43, 1.0);
pub(super) const GRAY: Vec4 = Vec4::new(130.0 / 255.0, 112.0 / 255.0, 104.0 / 255.0, 1.0);
pub(super) const RED: Vec4 = Vec4::new(176.0 / 255.0, 58.0 / 255.0, 44.0 / 255.0, 1.0);
pub(super) const GREEN: Vec4 = Vec4::new(56.0 / 255.0, 140.0 / 255.0, 76.0 / 255.0, 1.0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ButtonStyle {
    Normal,
    Hover,
    Disabled,
}

pub(super) fn frame(board: Tex) -> Vec<Element> {
    vec![
        rect(Vec2::ZERO, crate::viewport::TARGET, DIM_COLOR, DIM_LAYER),
        Element::Image {
            texture: board,
            top_left: BOARD_TOP_LEFT - Vec2::splat(BOARD_MARGIN),
            size: BOARD_SIZE + Vec2::splat(BOARD_MARGIN * 2.0),
            cell: None,
            layer: BOARD_LAYER,
            alpha: 1.0,
        },
    ]
}

pub(super) fn note(texture: Tex, top_left: Vec2, size: Vec2, row: usize, rows: usize) -> Element {
    Element::Image {
        texture,
        top_left,
        size,
        cell: Some((row, rows)),
        layer: NOTE_LAYER,
        alpha: 1.0,
    }
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

pub(super) fn bar(
    top_left: Vec2,
    size: Vec2,
    fraction: f32,
    fill: Vec4,
    track: Vec4,
) -> Vec<Element> {
    let width = (size.x * fraction.clamp(0.0, 1.0)).floor();
    let mut elements = vec![rect(top_left, size, track, BAR_LAYER)];
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

pub(super) fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for word in text.split_whitespace() {
        match lines.last_mut() {
            Some(line) if line.len() + 1 + word.len() <= width => {
                line.push(' ');
                line.push_str(word);
            }
            _ => lines.push(word.to_string()),
        }
    }
    lines
}

pub(super) fn plaque(
    top_left: Vec2,
    size: Vec2,
    text: &str,
    style: ButtonStyle,
    font: FontSize,
) -> Vec<Element> {
    let (row, ink) = match style {
        ButtonStyle::Normal => (0, CREAM),
        ButtonStyle::Hover => (1, YELLOW),
        ButtonStyle::Disabled => (2, GRAY),
    };
    let baseline = top_left.y
        + size.y / 2.0
        + match font {
            FontSize::Size16 => 5.0,
            _ => 7.0,
        };
    vec![
        note(tex::NOTE_PLAQUE, top_left, size, row, 3),
        label(
            text,
            Vec2::new(top_left.x + size.x / 2.0, baseline),
            ink,
            font,
            Align::Center,
        ),
    ]
}

pub(super) fn contains(top_left: Vec2, size: Vec2, point: Vec2) -> bool {
    point.cmpge(top_left).all() && point.cmple(top_left + size).all()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bar_fills_by_its_fraction_and_stays_inside_its_track() {
        let size = Vec2::new(100.0, 8.0);
        let widths = |fraction: f32| -> Vec<f32> {
            bar(Vec2::ZERO, size, fraction, RED, TRACK_BROWN)
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
    fn plaques_change_their_ink_with_the_style() {
        let colour_of = |style: ButtonStyle| {
            plaque(
                Vec2::ZERO,
                Vec2::new(88.0, 28.0),
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

        assert_eq!(colour_of(ButtonStyle::Normal), Some(CREAM));
        assert_eq!(colour_of(ButtonStyle::Hover), Some(YELLOW));
        assert_eq!(colour_of(ButtonStyle::Disabled), Some(GRAY));
    }

    #[test]
    fn the_frame_is_a_dim_layer_and_the_chosen_board() {
        let elements = frame(tex::BOARD_NOTICE);

        assert_eq!(elements.len(), 2);
        assert!(
            matches!(&elements[1], Element::Image { texture, .. } if *texture == tex::BOARD_NOTICE)
        );
    }

    #[test]
    fn long_text_wraps_on_word_boundaries() {
        assert_eq!(
            wrap("Clear a floor without taking damage", 18),
            vec!["Clear a floor", "without taking", "damage"]
        );
        assert!(wrap("", 10).is_empty());
    }

    #[test]
    fn contains_includes_the_edges() {
        let at = Vec2::new(10.0, 10.0);
        let size = Vec2::new(20.0, 10.0);

        assert!(contains(at, size, Vec2::new(30.0, 20.0)));
        assert!(!contains(at, size, Vec2::new(31.0, 20.0)));
    }
}
