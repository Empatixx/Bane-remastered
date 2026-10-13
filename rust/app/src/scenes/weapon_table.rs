use glam::{Vec2, Vec4};
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::{Assets, tex};
use crate::guns::GunKind;
use crate::input::Input;
use crate::render::{Align, FontSize};

use super::panel::{self, ButtonStyle, CREAM, INK, INK_SOFT, SAND};
use super::upgrade_menu::{Element, clear, rect, spawn};

const SCALE: f32 = panel::SCALE;
const LIST_LEFT: f32 = 204.0;
const ROW_SIZE: Vec2 = Vec2::new(52.0 * SCALE, 14.0 * SCALE);
const ROW_TOP: f32 = 74.0;
const ROW_STEP: f32 = 64.0;
const VISIBLE_ROWS: usize = 6;
const GUN_BOX: Vec2 = Vec2::new(80.0, 40.0);
const BAR_LEFT: f32 = 424.0;
const BAR_WIDTH: f32 = 6.0;
const PANEL_TOP_LEFT: Vec2 = Vec2::new(444.0, 66.0);
const PANEL_SIZE: Vec2 = Vec2::new(330.0, 392.0);
const CONTENT_LEFT: f32 = 20.0;
const NODE: f32 = 48.0;
const GLYPH: f32 = 33.0;
const TREE_TOP: f32 = 124.0;
const TIER_STEP: f32 = 64.0;
const BRANCH_CENTERS: [f32; 3] = [62.0, 168.0, 274.0];
const LABEL_BASELINE: f32 = 114.0;
const RULE_TOP: f32 = 316.0;
const DETAIL_BASELINE: f32 = 346.0;
const BUY_SIZE: Vec2 = Vec2::new(88.0, 28.0);
const BUY_OFFSET: Vec2 = Vec2::new(232.0, 336.0);
const LINK_WIDTH: f32 = 4.0;
const PANEL_LAYER: i32 = 31;
const FILL_LAYER: i32 = 32;
const NODE_LAYER: i32 = 33;
const LINK_LAYER: i32 = 33;
const ICON_LAYER: i32 = 34;
const TIERS: usize = 3;
const SAMPLE_GOLD: u32 = 1240;
const PANEL_EDGE: Vec4 = Vec4::new(37.0 / 255.0, 19.0 / 255.0, 26.0 / 255.0, 1.0);
const PANEL_FILL: Vec4 = Vec4::new(58.0 / 255.0, 40.0 / 255.0, 23.0 / 255.0, 1.0);
const LINK_ON: Vec4 = Vec4::new(232.0 / 255.0, 214.0 / 255.0, 170.0 / 255.0, 1.0);
const LINK_OFF: Vec4 = Vec4::new(97.0 / 255.0, 67.0 / 255.0, 38.0 / 255.0, 1.0);
const TRACK: Vec4 = Vec4::new(122.0 / 255.0, 83.0 / 255.0, 51.0 / 255.0, 1.0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Branch {
    Damage,
    FireRate,
    Magazine,
}

impl Branch {
    const ALL: [Self; 3] = [Self::Damage, Self::FireRate, Self::Magazine];

    fn name(self) -> &'static str {
        match self {
            Self::Damage => "Damage",
            Self::FireRate => "Fire rate",
            Self::Magazine => "Magazine",
        }
    }

    fn effect(self) -> &'static str {
        match self {
            Self::Damage => "+5% damage",
            Self::FireRate => "+4% fire rate",
            Self::Magazine => "+1 round",
        }
    }
}

fn name(kind: GunKind) -> &'static str {
    match kind {
        GunKind::Pistol => "Pistol",
        GunKind::Shotgun => "Shotgun",
        GunKind::Luger => "Luger",
        GunKind::Revolver => "Revolver",
        GunKind::M4 => "M4",
        GunKind::Uzi => "Uzi",
        GunKind::Thompson => "Thompson",
    }
}

fn roman(tier: usize) -> &'static str {
    ["I", "II", "III"][tier.min(TIERS - 1)]
}

fn sample_level(kind: GunKind, branch: Branch) -> usize {
    let gun = GunKind::ALL
        .iter()
        .position(|other| *other == kind)
        .unwrap_or(0);
    let branch = Branch::ALL
        .iter()
        .position(|other| *other == branch)
        .unwrap_or(0);
    (gun + branch * 2) % (TIERS + 1)
}

fn cost(tier: usize) -> u32 {
    100 + 150 * tier as u32
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeState {
    Owned,
    Available,
    Locked,
}

fn node_state(kind: GunKind, branch: Branch, tier: usize) -> NodeState {
    let level = sample_level(kind, branch);
    if tier < level {
        NodeState::Owned
    } else if tier == level {
        NodeState::Available
    } else {
        NodeState::Locked
    }
}

fn max_first() -> usize {
    GunKind::ALL.len().saturating_sub(VISIBLE_ROWS)
}

fn row_top_left(slot: usize) -> Vec2 {
    Vec2::new(LIST_LEFT, ROW_TOP + ROW_STEP * slot as f32)
}

fn node_top_left(branch: usize, tier: usize) -> Vec2 {
    PANEL_TOP_LEFT
        + Vec2::new(
            BRANCH_CENTERS[branch] - NODE / 2.0,
            TREE_TOP + TIER_STEP * tier as f32,
        )
}

fn buy_top_left() -> Vec2 {
    PANEL_TOP_LEFT + BUY_OFFSET
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Target {
    Weapon(usize),
    Node(usize, usize),
    Buy,
}

pub(crate) fn target_at(first: usize, point: Vec2) -> Option<Target> {
    let shown = GunKind::ALL.len().min(VISIBLE_ROWS);
    if let Some(slot) =
        (0..shown).find(|slot| panel::contains(row_top_left(*slot), ROW_SIZE, point))
    {
        return Some(Target::Weapon(first + slot));
    }
    for branch in 0..Branch::ALL.len() {
        for tier in 0..TIERS {
            if panel::contains(node_top_left(branch, tier), Vec2::splat(NODE), point) {
                return Some(Target::Node(branch, tier));
            }
        }
    }
    panel::contains(buy_top_left(), BUY_SIZE, point).then_some(Target::Buy)
}

fn per_second(interval: f32) -> String {
    format!("{:.1}/s", 1.0 / interval)
}

fn gun_picture(kind: GunKind, row: Vec2) -> Element {
    let spec = kind.spec();
    let scale = (GUN_BOX.x / spec.icon.x).min(GUN_BOX.y / spec.icon.y);
    let size = (spec.icon * scale).floor();
    Element::Image {
        texture: spec.texture,
        top_left: row
            + Vec2::new(8.0, (ROW_SIZE.y - size.y) / 2.0)
            + Vec2::new((GUN_BOX.x - size.x) / 2.0, 0.0),
        size,
        cell: None,
        layer: ICON_LAYER,
        alpha: 1.0,
    }
}

fn glyph(branch: usize, top_left: Vec2, owned: bool) -> Element {
    let row = branch + if owned { Branch::ALL.len() } else { 0 };
    Element::Image {
        texture: tex::UPGRADE_ICONS,
        top_left: top_left + Vec2::splat((NODE - GLYPH) / 2.0),
        size: Vec2::splat(GLYPH),
        cell: Some((row, Branch::ALL.len() * 2)),
        layer: ICON_LAYER,
        alpha: 1.0,
    }
}

fn scrollbar(first: usize) -> Vec<Element> {
    let total = GunKind::ALL.len();
    if total <= VISIBLE_ROWS {
        return Vec::new();
    }
    let track = ROW_STEP * VISIBLE_ROWS as f32 - (ROW_STEP - ROW_SIZE.y);
    let thumb = (track * VISIBLE_ROWS as f32 / total as f32).floor();
    let travel = track - thumb;
    let at = travel * first as f32 / max_first() as f32;
    vec![
        rect(
            Vec2::new(BAR_LEFT, ROW_TOP),
            Vec2::new(BAR_WIDTH, track),
            PANEL_EDGE,
            PANEL_LAYER,
        ),
        rect(
            Vec2::new(BAR_LEFT + 1.0, ROW_TOP + at.floor() + 1.0),
            Vec2::new(BAR_WIDTH - 2.0, thumb - 2.0),
            TRACK,
            FILL_LAYER,
        ),
    ]
}

pub(crate) fn layout(
    first: usize,
    selected: usize,
    node: Option<(usize, usize)>,
    hover: Option<Target>,
) -> Vec<Element> {
    let kind = GunKind::ALL[selected.min(GunKind::ALL.len() - 1)];
    let spec = kind.spec();
    let mut elements = panel::frame(tex::BOARD_WOOD);
    elements.push(panel::label(
        format!("Gold: {SAMPLE_GOLD}"),
        Vec2::new(
            panel::BOARD_TOP_LEFT.x + panel::BOARD_SIZE.x - 24.0,
            panel::BOARD_TOP_LEFT.y + 34.0,
        ),
        INK,
        FontSize::Size22,
        Align::Right,
    ));
    for slot in 0..GunKind::ALL.len().min(VISIBLE_ROWS) {
        let index = first + slot;
        let other = GunKind::ALL[index];
        let top_left = row_top_left(slot);
        let chosen = index == selected;
        let hovered = hover == Some(Target::Weapon(index));
        let row = if chosen { 2 } else { usize::from(hovered) };
        elements.push(panel::note(tex::NOTE_TAG, top_left, ROW_SIZE, row, 3));
        elements.push(gun_picture(other, top_left));
        elements.push(panel::label(
            name(other),
            top_left + Vec2::new(100.0, 36.0),
            if chosen { INK } else { INK_SOFT },
            FontSize::Size22,
            Align::Left,
        ));
    }
    elements.extend(scrollbar(first));
    elements.push(rect(PANEL_TOP_LEFT, PANEL_SIZE, PANEL_EDGE, PANEL_LAYER));
    elements.push(rect(
        PANEL_TOP_LEFT + Vec2::splat(3.0),
        PANEL_SIZE - Vec2::splat(6.0),
        PANEL_FILL,
        FILL_LAYER,
    ));
    elements.push(panel::label(
        name(kind),
        PANEL_TOP_LEFT + Vec2::new(CONTENT_LEFT, 44.0),
        CREAM,
        FontSize::Size27,
        Align::Left,
    ));
    let facts = [
        format!("Damage {}-{}", spec.min_damage, spec.max_damage),
        format!("Rate {}", per_second(spec.fire_interval)),
        format!("Clip {}", spec.clip_size),
        format!("Reload {:.1}s", spec.reload_time),
    ];
    for (index, fact) in facts.into_iter().enumerate() {
        elements.push(panel::label(
            fact,
            PANEL_TOP_LEFT
                + Vec2::new(
                    CONTENT_LEFT + (index % 2) as f32 * 150.0,
                    70.0 + (index / 2) as f32 * 18.0,
                ),
            SAND,
            FontSize::Size16,
            Align::Left,
        ));
    }
    for (branch, kind_of) in Branch::ALL.iter().enumerate() {
        elements.push(panel::label(
            kind_of.name(),
            PANEL_TOP_LEFT + Vec2::new(BRANCH_CENTERS[branch], LABEL_BASELINE),
            SAND,
            FontSize::Size16,
            Align::Center,
        ));
        for tier in 0..TIERS {
            let top_left = node_top_left(branch, tier);
            let state = node_state(kind, *kind_of, tier);
            let chosen = node == Some((branch, tier)) || hover == Some(Target::Node(branch, tier));
            let row = if chosen {
                3
            } else {
                match state {
                    NodeState::Owned => 0,
                    NodeState::Available => 1,
                    NodeState::Locked => 2,
                }
            };
            elements.push(Element::Image {
                texture: tex::UPGRADE_NODE,
                top_left,
                size: Vec2::splat(NODE),
                cell: Some((row, 4)),
                layer: NODE_LAYER,
                alpha: 1.0,
            });
            elements.push(glyph(
                branch,
                top_left,
                state == NodeState::Owned && !chosen,
            ));
            if tier + 1 < TIERS {
                elements.push(rect(
                    top_left + Vec2::new((NODE - LINK_WIDTH) / 2.0, NODE),
                    Vec2::new(LINK_WIDTH, TIER_STEP - NODE),
                    if state == NodeState::Owned {
                        LINK_ON
                    } else {
                        LINK_OFF
                    },
                    LINK_LAYER,
                ));
            }
        }
    }
    elements.push(rect(
        PANEL_TOP_LEFT + Vec2::new(CONTENT_LEFT, RULE_TOP),
        Vec2::new(PANEL_SIZE.x - CONTENT_LEFT * 2.0, 2.0),
        LINK_OFF,
        NODE_LAYER,
    ));
    let (title, effect, text, style) = match node {
        Some((branch, tier)) => {
            let kind_of = Branch::ALL[branch];
            let (text, style) = match node_state(kind, kind_of, tier) {
                NodeState::Owned => ("Owned".to_string(), ButtonStyle::Disabled),
                NodeState::Locked => ("Locked".to_string(), ButtonStyle::Disabled),
                NodeState::Available if cost(tier) > SAMPLE_GOLD => {
                    (format!("{} g", cost(tier)), ButtonStyle::Disabled)
                }
                NodeState::Available if hover == Some(Target::Buy) => {
                    (format!("{} g", cost(tier)), ButtonStyle::Hover)
                }
                NodeState::Available => (format!("{} g", cost(tier)), ButtonStyle::Normal),
            };
            (
                format!("{} {}", kind_of.name(), roman(tier)),
                kind_of.effect().to_string(),
                text,
                style,
            )
        }
        None => (
            "Pick an upgrade".to_string(),
            String::new(),
            "Select".to_string(),
            ButtonStyle::Disabled,
        ),
    };
    elements.push(panel::label(
        title,
        PANEL_TOP_LEFT + Vec2::new(CONTENT_LEFT, DETAIL_BASELINE),
        CREAM,
        FontSize::Size22,
        Align::Left,
    ));
    elements.push(panel::label(
        effect,
        PANEL_TOP_LEFT + Vec2::new(CONTENT_LEFT, DETAIL_BASELINE + 20.0),
        SAND,
        FontSize::Size16,
        Align::Left,
    ));
    elements.extend(panel::plaque(
        buy_top_left(),
        BUY_SIZE,
        &text,
        style,
        FontSize::Size16,
    ));
    elements
}

#[derive(Debug, Default)]
pub(crate) struct WeaponTable {
    open: bool,
    first: usize,
    selected: usize,
    node: Option<(usize, usize)>,
    hover: Option<Target>,
    dirty: bool,
}

impl WeaponTable {
    pub(crate) fn is_open(&self) -> bool {
        self.open
    }

    pub(crate) fn toggle(&mut self, world: &mut World, assets: &Assets) {
        if self.open {
            self.close(world);
        } else {
            self.open = true;
            self.first = 0;
            self.selected = 0;
            self.node = None;
            self.hover = None;
            self.rebuild(world, assets);
        }
    }

    pub(crate) fn close(&mut self, world: &mut World) {
        self.open = false;
        self.hover = None;
        clear(world);
    }

    fn scroll(&mut self, rows: i32) {
        let moved = (self.first as i32 + rows).clamp(0, max_first() as i32) as usize;
        if moved != self.first {
            self.first = moved;
            self.dirty = true;
        }
    }

    pub(crate) fn update(&mut self, world: &mut World, input: &Input, assets: &Assets) {
        if !self.open {
            return;
        }
        self.scroll(-input.scrolled());
        if input.just_pressed(KeyCode::ArrowDown) {
            self.scroll(1);
        }
        if input.just_pressed(KeyCode::ArrowUp) {
            self.scroll(-1);
        }
        let hover = target_at(self.first, input.mouse());
        if hover != self.hover {
            self.hover = hover;
            self.dirty = true;
        }
        if input.clicked() {
            match hover {
                Some(Target::Weapon(index)) if index != self.selected => {
                    self.selected = index;
                    self.node = None;
                    self.dirty = true;
                }
                Some(Target::Node(branch, tier)) => {
                    self.node = Some((branch, tier));
                    self.dirty = true;
                }
                _ => {}
            }
        }
        if self.dirty {
            self.dirty = false;
            self.rebuild(world, assets);
        }
    }

    fn rebuild(&self, world: &mut World, assets: &Assets) {
        clear(world);
        for element in layout(self.first, self.selected, self.node, self.hover) {
            spawn(world, element, assets);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels(elements: &[Element]) -> Vec<String> {
        elements
            .iter()
            .filter_map(|element| match element {
                Element::Label { content, .. } => Some(content.clone()),
                _ => None,
            })
            .collect()
    }

    fn images(elements: &[Element], wanted: crate::assets::Tex) -> usize {
        elements
            .iter()
            .filter(
                |element| matches!(element, Element::Image { texture, .. } if *texture == wanted),
            )
            .count()
    }

    #[test]
    fn each_visible_row_shows_the_picture_of_its_weapon() {
        let elements = layout(0, 0, None, None);

        for kind in &GunKind::ALL[..VISIBLE_ROWS] {
            assert_eq!(images(&elements, kind.spec().texture), 1, "{kind:?}");
            assert!(labels(&elements).contains(&name(*kind).to_string()));
        }
        assert_eq!(
            images(&elements, GunKind::ALL[VISIBLE_ROWS].spec().texture),
            0
        );
    }

    #[test]
    fn scrolling_brings_the_hidden_weapons_into_view() {
        let last = GunKind::ALL[GunKind::ALL.len() - 1];
        let elements = layout(max_first(), 0, None, None);

        assert_eq!(images(&elements, last.spec().texture), 1);
        assert!(labels(&elements).contains(&name(last).to_string()));
    }

    #[test]
    fn the_scroll_position_is_clamped_to_the_list() {
        let mut table = WeaponTable::default();

        table.scroll(-3);
        assert_eq!(table.first, 0);
        table.scroll(99);
        assert_eq!(table.first, max_first());
        table.scroll(-1);
        assert_eq!(table.first, max_first().saturating_sub(1));
    }

    #[test]
    fn a_gun_picture_never_leaves_its_box() {
        for kind in GunKind::ALL {
            let Element::Image { size, .. } = gun_picture(kind, Vec2::ZERO) else {
                panic!("not an image");
            };

            assert!(
                size.x <= GUN_BOX.x && size.y <= GUN_BOX.y,
                "{kind:?}: {size:?}"
            );
        }
    }

    #[test]
    fn the_tree_has_three_branches_of_three_tiers_for_every_weapon() {
        for selected in 0..GunKind::ALL.len() {
            let elements = layout(0, selected, None, None);

            assert_eq!(images(&elements, tex::UPGRADE_NODE), 9);
            assert_eq!(images(&elements, tex::UPGRADE_ICONS), 9);
        }
    }

    #[test]
    fn the_header_shows_the_real_stats_of_the_selected_weapon() {
        let spec = GunKind::Shotgun.spec();
        let shown = labels(&layout(0, 1, None, None));

        assert!(shown.contains(&format!("Damage {}-{}", spec.min_damage, spec.max_damage)));
        assert!(shown.contains(&format!("Clip {}", spec.clip_size)));
    }

    #[test]
    fn a_chosen_node_fills_the_detail_line() {
        let shown = labels(&layout(0, 0, Some((0, 0)), None));

        assert!(shown.contains(&"Damage I".to_string()));
        assert!(labels(&layout(0, 0, None, None)).contains(&"Pick an upgrade".to_string()));
    }

    #[test]
    fn tiers_unlock_in_order() {
        let kind = GunKind::ALL[0];
        let level = sample_level(kind, Branch::Damage);

        for tier in 0..TIERS {
            let expected = if tier < level {
                NodeState::Owned
            } else if tier == level {
                NodeState::Available
            } else {
                NodeState::Locked
            };
            assert_eq!(node_state(kind, Branch::Damage, tier), expected);
        }
    }

    #[test]
    fn the_pointer_follows_the_scrolled_list() {
        assert_eq!(
            target_at(0, row_top_left(3) + Vec2::splat(4.0)),
            Some(Target::Weapon(3))
        );
        assert_eq!(
            target_at(1, row_top_left(3) + Vec2::splat(4.0)),
            Some(Target::Weapon(4))
        );
        assert_eq!(
            target_at(0, node_top_left(1, 2) + Vec2::splat(4.0)),
            Some(Target::Node(1, 2))
        );
        assert_eq!(
            target_at(0, buy_top_left() + Vec2::splat(4.0)),
            Some(Target::Buy)
        );
        assert_eq!(target_at(0, Vec2::ZERO), None);
    }

    #[test]
    fn everything_fits_inside_the_board() {
        let board = panel::BOARD_TOP_LEFT + panel::BOARD_SIZE;
        let list = row_top_left(VISIBLE_ROWS - 1) + ROW_SIZE;
        let panel_end = PANEL_TOP_LEFT + PANEL_SIZE;
        let tree = node_top_left(BRANCH_CENTERS.len() - 1, TIERS - 1) + Vec2::splat(NODE);

        assert!(list.y < board.y - 20.0);
        assert!(panel_end.y < board.y - 20.0);
        assert!(panel_end.x < board.x);
        assert!(list.x < BAR_LEFT && BAR_LEFT + BAR_WIDTH < PANEL_TOP_LEFT.x);
        assert!(tree.y < PANEL_TOP_LEFT.y + RULE_TOP);
        assert!(buy_top_left().y + BUY_SIZE.y < panel_end.y);
    }

    #[test]
    fn upgrade_costs_grow_with_the_tier() {
        assert!(cost(1) > cost(0));
        assert!(cost(2) > cost(1));
    }
}
