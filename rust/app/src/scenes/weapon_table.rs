use glam::{Vec2, Vec4};
use hecs::World;

use crate::assets::{Assets, tex};
use crate::guns::GunKind;
use crate::input::Input;
use crate::render::{Align, FontSize};

use super::panel::{self, BLUE_INK, BLUE_SOFT, ButtonStyle, CREAM, INK, INK_SOFT, YELLOW};
use super::upgrade_menu::{Element, clear, rect, spawn};

const SCALE: f32 = panel::SCALE;
const LIST_LEFT: f32 = 204.0;
const SEARCH_TOP: f32 = 52.0;
const SEARCH_SIZE: Vec2 = Vec2::new(44.0 * SCALE, 36.0);
const ROW_SIZE: Vec2 = Vec2::new(44.0 * SCALE, 12.0 * SCALE);
const ROW_TOP: f32 = 100.0;
const ROW_STEP: f32 = 54.0;
const SHEET_TOP_LEFT: Vec2 = Vec2::new(404.0, 66.0);
const SHEET_SIZE: Vec2 = Vec2::new(92.0 * SCALE, 98.0 * SCALE);
const SHEET_INSET: f32 = SCALE;
const CONTENT_LEFT: f32 = 44.0;
const NODE: f32 = 48.0;
const GLYPH: f32 = 33.0;
const TREE_TOP: f32 = 152.0;
const TIER_STEP: f32 = 66.0;
const BRANCH_CENTERS: [f32; 3] = [92.0, 192.0, 292.0];
const LABEL_BASELINE: f32 = 142.0;
const DETAIL_BASELINE: f32 = 354.0;
const BUY_SIZE: Vec2 = Vec2::new(88.0, 28.0);
const BUY_OFFSET: Vec2 = Vec2::new(256.0, 338.0);
const LINK_WIDTH: f32 = 4.0;
const LINK_LAYER: i32 = 32;
const ICON_LAYER: i32 = 33;
const TIERS: usize = 3;
const SAMPLE_GOLD: u32 = 1240;
const SEARCH_FILL: Vec4 = Vec4::new(58.0 / 255.0, 40.0 / 255.0, 23.0 / 255.0, 1.0);
const LINK_ON: Vec4 = Vec4::new(71.0 / 255.0, 71.0 / 255.0, 120.0 / 255.0, 1.0);
const LINK_OFF: Vec4 = Vec4::new(113.0 / 255.0, 113.0 / 255.0, 199.0 / 255.0, 1.0);

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

fn row_top_left(index: usize) -> Vec2 {
    Vec2::new(LIST_LEFT, ROW_TOP + ROW_STEP * index as f32)
}

fn search_top_left() -> Vec2 {
    Vec2::new(LIST_LEFT, SEARCH_TOP)
}

fn origin() -> Vec2 {
    SHEET_TOP_LEFT + Vec2::splat(SHEET_INSET)
}

fn node_top_left(branch: usize, tier: usize) -> Vec2 {
    origin()
        + Vec2::new(
            BRANCH_CENTERS[branch] - NODE / 2.0,
            TREE_TOP + TIER_STEP * tier as f32,
        )
}

fn buy_top_left() -> Vec2 {
    origin() + BUY_OFFSET
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Target {
    Search,
    Weapon(usize),
    Node(usize, usize),
    Buy,
}

pub(crate) fn target_at(point: Vec2) -> Option<Target> {
    if panel::contains(search_top_left(), SEARCH_SIZE, point) {
        return Some(Target::Search);
    }
    if let Some(index) =
        (0..GunKind::ALL.len()).find(|index| panel::contains(row_top_left(*index), ROW_SIZE, point))
    {
        return Some(Target::Weapon(index));
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

fn glyph(branch: usize, top_left: Vec2) -> Element {
    Element::Image {
        texture: tex::UPGRADE_ICONS,
        top_left: top_left + Vec2::splat((NODE - GLYPH) / 2.0),
        size: Vec2::splat(GLYPH),
        cell: Some((branch, Branch::ALL.len())),
        layer: ICON_LAYER,
        alpha: 1.0,
    }
}

pub(crate) fn layout(
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
    let search = search_top_left();
    let searching = hover == Some(Target::Search);
    elements.push(rect(
        search,
        SEARCH_SIZE,
        if searching { YELLOW } else { SEARCH_FILL },
        31,
    ));
    elements.push(rect(
        search + Vec2::splat(2.0),
        SEARCH_SIZE - Vec2::splat(4.0),
        SEARCH_FILL,
        32,
    ));
    elements.push(panel::label(
        "Search weapons",
        search + Vec2::new(12.0, 24.0),
        if searching { YELLOW } else { CREAM },
        FontSize::Size16,
        Align::Left,
    ));
    for (index, other) in GunKind::ALL.iter().enumerate() {
        let top_left = row_top_left(index);
        let chosen = index == selected;
        let hovered = hover == Some(Target::Weapon(index));
        let row = if chosen { 2 } else { usize::from(hovered) };
        elements.push(panel::note(tex::NOTE_TAG, top_left, ROW_SIZE, row, 3));
        elements.push(panel::label(
            name(*other),
            top_left + Vec2::new(28.0, 32.0),
            if chosen { INK } else { INK_SOFT },
            FontSize::Size22,
            Align::Left,
        ));
    }
    elements.push(panel::sheet(SHEET_TOP_LEFT, SHEET_SIZE));
    let base = origin();
    elements.push(panel::label(
        name(kind),
        base + Vec2::new(CONTENT_LEFT, 56.0),
        BLUE_INK,
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
            base + Vec2::new(
                CONTENT_LEFT + (index % 2) as f32 * 150.0,
                100.0 + (index / 2) as f32 * 18.0,
            ),
            BLUE_SOFT,
            FontSize::Size16,
            Align::Left,
        ));
    }
    for (branch, kind_of) in Branch::ALL.iter().enumerate() {
        elements.push(panel::label(
            kind_of.name(),
            base + Vec2::new(BRANCH_CENTERS[branch], LABEL_BASELINE),
            BLUE_INK,
            FontSize::Size16,
            Align::Center,
        ));
        for tier in 0..TIERS {
            let top_left = node_top_left(branch, tier);
            let state = node_state(kind, *kind_of, tier);
            let chosen = node == Some((branch, tier));
            let hovered = hover == Some(Target::Node(branch, tier));
            let row = if chosen || hovered {
                3
            } else {
                match state {
                    NodeState::Owned => 0,
                    NodeState::Available => 1,
                    NodeState::Locked => 2,
                }
            };
            elements.push(panel::note(
                tex::UPGRADE_NODE,
                top_left,
                Vec2::splat(NODE),
                row,
                4,
            ));
            elements.push(glyph(branch, top_left));
            if tier + 1 < TIERS {
                let lit = state == NodeState::Owned;
                elements.push(rect(
                    top_left + Vec2::new((NODE - LINK_WIDTH) / 2.0, NODE),
                    Vec2::new(LINK_WIDTH, TIER_STEP - NODE),
                    if lit { LINK_ON } else { LINK_OFF },
                    LINK_LAYER,
                ));
            }
        }
    }
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
        base + Vec2::new(CONTENT_LEFT, DETAIL_BASELINE),
        BLUE_INK,
        FontSize::Size22,
        Align::Left,
    ));
    elements.push(panel::label(
        effect,
        base + Vec2::new(CONTENT_LEFT, DETAIL_BASELINE + 18.0),
        BLUE_SOFT,
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

    pub(crate) fn update(&mut self, world: &mut World, input: &Input, assets: &Assets) {
        if !self.open {
            return;
        }
        let hover = target_at(input.mouse());
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
        for element in layout(self.selected, self.node, self.hover) {
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
    fn the_list_always_names_every_weapon_next_to_the_search() {
        let shown = labels(&layout(0, None, None));

        for kind in GunKind::ALL {
            assert!(shown.contains(&name(kind).to_string()), "{kind:?}");
        }
        assert!(shown.contains(&"Search weapons".to_string()));
    }

    #[test]
    fn the_tree_has_three_branches_of_three_tiers_for_every_weapon() {
        for selected in 0..GunKind::ALL.len() {
            let elements = layout(selected, None, None);

            assert_eq!(images(&elements, tex::UPGRADE_NODE), 9);
            assert_eq!(images(&elements, tex::UPGRADE_ICONS), 9);
        }
    }

    #[test]
    fn the_header_shows_the_real_stats_of_the_selected_weapon() {
        let spec = GunKind::Shotgun.spec();
        let shown = labels(&layout(1, None, None));

        assert!(shown.contains(&format!("Damage {}-{}", spec.min_damage, spec.max_damage)));
        assert!(shown.contains(&format!("Clip {}", spec.clip_size)));
    }

    #[test]
    fn a_chosen_node_fills_the_detail_line_and_the_buy_plaque() {
        let kind = GunKind::ALL[0];
        let tier = sample_level(kind, Branch::Damage);
        let shown = labels(&layout(0, Some((0, tier.min(TIERS - 1))), None));

        assert!(shown.contains(&format!("Damage {}", roman(tier.min(TIERS - 1)))));
        assert!(labels(&layout(0, None, None)).contains(&"Pick an upgrade".to_string()));
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
    fn the_pointer_finds_search_weapons_nodes_and_buy() {
        assert_eq!(
            target_at(search_top_left() + Vec2::splat(4.0)),
            Some(Target::Search)
        );
        assert_eq!(
            target_at(row_top_left(3) + Vec2::splat(4.0)),
            Some(Target::Weapon(3))
        );
        assert_eq!(
            target_at(node_top_left(1, 2) + Vec2::splat(4.0)),
            Some(Target::Node(1, 2))
        );
        assert_eq!(
            target_at(buy_top_left() + Vec2::splat(4.0)),
            Some(Target::Buy)
        );
        assert_eq!(target_at(Vec2::ZERO), None);
    }

    #[test]
    fn everything_fits_inside_the_board() {
        let board = panel::BOARD_TOP_LEFT + panel::BOARD_SIZE;
        let list = row_top_left(GunKind::ALL.len() - 1) + ROW_SIZE;
        let sheet = SHEET_TOP_LEFT + SHEET_SIZE;
        let tree = node_top_left(BRANCH_CENTERS.len() - 1, TIERS - 1) + Vec2::splat(NODE);

        assert!(list.y < board.y - 20.0);
        assert!(sheet.y < board.y - 20.0);
        assert!(sheet.x < board.x);
        assert!(tree.y < buy_top_left().y);
        assert!(buy_top_left().y + BUY_SIZE.y < sheet.y);
        assert!(row_top_left(0).y > search_top_left().y + SEARCH_SIZE.y);
    }

    #[test]
    fn upgrade_costs_grow_with_the_tier() {
        assert!(cost(1) > cost(0));
        assert!(cost(2) > cost(1));
    }
}
