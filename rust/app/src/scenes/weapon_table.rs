use glam::{Vec2, Vec4};
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::{Assets, Tex, tex};
use crate::guns::GunKind;
use crate::input::Input;
use crate::render::{Align, FontSize};
use crate::upgrades::Quality;

use super::upgrade_menu::{
    BLOCKED_COLOR, BOARD_LAYER, BuyState, COST_BASELINE, COST_COLOR, COUNTER_BASELINE,
    COUNTER_CENTER, COUNTER_ROWS, COUNTER_SIZE, DESCRIPTION_STEP, DESCRIPTION_TOP, DIM_COLOR,
    DIM_LAYER, Element, FRAME_INSET, FRAME_LAYER, FRAME_ROWS, HOVER_COLOR, LINE_COLOR, LINE_OFF,
    LOCKED_ICON_ALPHA, MENU_COLOR, PANEL_LAYER, PANEL_SIZE, PANEL_TALENT_Y, PANEL_TOP_LEFT,
    PLATE_BASELINE, PLATE_CENTER, PLATE_HIT_HALF, PLATE_ROWS, PLATE_SIZE, TALENT, TITLE_BASELINE,
    clear, draw_path, frame_row, rect, spawn,
};
use super::weapon_tree::{self, Node, Tree};

const LIST_TOP_LEFT: Vec2 = Vec2::new(16.0, 14.0);
const LIST_SIZE: Vec2 = Vec2::new(72.0 * 3.0, 171.0 * 3.0);
const TREE_TOP_LEFT: Vec2 = Vec2::new(240.0, 14.0);
const TREE_SIZE: Vec2 = Vec2::new(142.0 * 3.0, 171.0 * 3.0);
const ROW_LEFT: f32 = 36.0;
const ROW_SIZE: Vec2 = Vec2::new(164.0, 70.0);
const ROW_TOP: f32 = 36.0;
const ROW_STEP: f32 = 78.0;
const VISIBLE_ROWS: usize = 6;
const GUN_BOX: Vec2 = Vec2::new(80.0, 34.0);
const GUN_TOP: f32 = 6.0;
const NAME_BASELINE: f32 = 62.0;
const BAR_LEFT: f32 = 204.0;
const BAR_WIDTH: f32 = 4.0;
const CENTER_X: f32 = TREE_TOP_LEFT.x + TREE_SIZE.x / 2.0;
const TITLE_Y: f32 = 60.0;
const FACTS_Y: f32 = 82.0;
const VIEW_TOP_LEFT: Vec2 = Vec2::new(TREE_TOP_LEFT.x + 18.0, 96.0);
const VIEW_SIZE: Vec2 = Vec2::new(TREE_SIZE.x - 36.0, 360.0);
const ROOT_TOP: f32 = VIEW_TOP_LEFT.y + 8.0;
const FADE: f32 = 20.0;
const DRAG_THRESHOLD: f32 = 5.0;
const PAN_SPEED: f32 = 380.0;
const PAN_MARGIN: f32 = 120.0;
const RARE_FROM_ROW: i32 = 3;
const STEP: Vec2 = Vec2::new(80.0, 74.0);
const REROLL_LEFT: f32 = CENTER_X - 54.0;
const REROLL_TOP: f32 = 462.0;
const REROLL_SIZE: Vec2 = Vec2::new(108.0, 43.0);
const REROLL_BASELINE: f32 = 27.0;
const GLYPH: f32 = 36.0;
const MUTATION_COST: u32 = 800;
const REROLL_COST: u32 = 100;
const SAMPLE_GOLD: u32 = 1240;
const NUMERALS: [&str; 3] = ["I", "II", "III"];
const ROW_FILL: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.5);
const ROW_BACK: Vec4 = Vec4::new(0.28, 0.2, 0.2, 0.6);
const ROW_RIM: Vec4 = Vec4::new(225.0 / 255.0, 152.0 / 255.0, 116.0 / 255.0, 1.0);
const ROW_LAYER: i32 = 31;
const GUN_LAYER: i32 = 33;
const BAR_LAYER: i32 = 32;

struct Stat {
    name: &'static str,
    amount: u32,
    unit: &'static str,
    noun: &'static str,
    second: &'static str,
}

const STATS: [Stat; 6] = [
    Stat {
        name: "Damage",
        amount: 5,
        unit: "%",
        noun: "damage",
        second: "on every shot",
    },
    Stat {
        name: "Fire rate",
        amount: 4,
        unit: "%",
        noun: "fire rate",
        second: "less time between shots",
    },
    Stat {
        name: "Magazine",
        amount: 1,
        unit: "",
        noun: "magazine size",
        second: "more rounds before reload",
    },
    Stat {
        name: "Reload",
        amount: 6,
        unit: "%",
        noun: "reload speed",
        second: "back in the fight sooner",
    },
    Stat {
        name: "Accuracy",
        amount: 8,
        unit: "%",
        noun: "less spread",
        second: "bullets fly straighter",
    },
    Stat {
        name: "Crit",
        amount: 4,
        unit: "%",
        noun: "crit chance",
        second: "crits deal double damage",
    },
];

struct Mutation {
    name: &'static str,
    description: [&'static str; 2],
}

const MUTATIONS: [Mutation; 6] = [
    Mutation {
        name: "Piercing Rounds",
        description: ["Bullets pierce", "the first enemy"],
    },
    Mutation {
        name: "Explosive Rounds",
        description: ["Hits explode and hurt", "enemies nearby"],
    },
    Mutation {
        name: "Venom Rounds",
        description: ["Hits poison and leave", "a toxic pool"],
    },
    Mutation {
        name: "Frost Rounds",
        description: ["Hits chill and", "slow enemies"],
    },
    Mutation {
        name: "Ricochet",
        description: ["Bullets bounce", "off walls once"],
    },
    Mutation {
        name: "Deadeye",
        description: ["Every fifth shot", "is a critical hit"],
    },
];

fn tree(kind: GunKind, rerolls: u32) -> Tree {
    weapon_tree::generate(kind, rerolls)
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

fn numeral(tree: &Tree, index: usize) -> &'static str {
    let stat = tree.nodes[index].stat;
    let before = tree.nodes[..index]
        .iter()
        .filter(|node| node.mutation.is_none() && node.stat == stat)
        .count();
    NUMERALS[before.min(NUMERALS.len() - 1)]
}

fn reach(kind: GunKind, rerolls: u32, branch: usize) -> i32 {
    let gun = GunKind::ALL
        .iter()
        .position(|other| *other == kind)
        .unwrap_or(0);
    ((gun * 3 + branch * 5 + rerolls as usize * 2) % (weapon_tree::DEPTH as usize + 1)) as i32
}

fn cost(node: &Node) -> u32 {
    if node.mutation.is_some() {
        MUTATION_COST
    } else {
        100 + 100 * (node.row as u32 - 1)
    }
}

fn node_state(kind: GunKind, rerolls: u32, grown: &Tree, index: usize) -> BuyState {
    let node = &grown.nodes[index];
    let affordable = cost(node) <= SAMPLE_GOLD;
    if node.mutation.is_some() {
        return if reach(kind, rerolls, 0) >= node.row - 1 && affordable {
            BuyState::Available
        } else {
            BuyState::Locked
        };
    }
    let level = reach(kind, rerolls, node.branch);
    if node.row <= level {
        BuyState::Owned
    } else if node.row == level + 1 && affordable {
        BuyState::Available
    } else {
        BuyState::Locked
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Hover {
    buy: bool,
    reroll: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Icon {
    Stat(usize),
    Mutation(usize),
}

fn max_first() -> usize {
    GunKind::ALL.len().saturating_sub(VISIBLE_ROWS)
}

fn row_top_left(slot: usize) -> Vec2 {
    Vec2::new(ROW_LEFT, ROW_TOP + ROW_STEP * slot as f32)
}

fn cell_top_left(column: i32, row: i32, pan: Vec2) -> Vec2 {
    Vec2::new(
        CENTER_X + column as f32 * STEP.x - TALENT / 2.0,
        ROOT_TOP + row as f32 * STEP.y,
    ) + pan
}

fn visibility(top_left: Vec2, size: Vec2) -> f32 {
    let inside = (top_left - VIEW_TOP_LEFT)
        .min(VIEW_TOP_LEFT + VIEW_SIZE - (top_left + size))
        .min_element();
    (inside / FADE).clamp(0.0, 1.0)
}

fn in_view(point: Vec2) -> bool {
    inside(point, VIEW_TOP_LEFT, VIEW_SIZE)
}

fn pan_limits(grown: &Tree) -> (Vec2, Vec2) {
    let (mut low, mut high) = ((0, 0), (0, 0));
    for node in &grown.nodes {
        low = (low.0.min(node.column), low.1.min(node.row));
        high = (high.0.max(node.column), high.1.max(node.row));
    }
    let tree_low = cell_top_left(low.0, low.1, Vec2::ZERO);
    let tree_high = cell_top_left(high.0, high.1, Vec2::ZERO) + TALENT;
    let minimum = VIEW_TOP_LEFT + PAN_MARGIN - tree_high;
    let maximum = VIEW_TOP_LEFT + VIEW_SIZE - PAN_MARGIN - tree_low;
    (minimum.min(maximum), minimum.max(maximum))
}

fn clamp_pan(grown: &Tree, pan: Vec2) -> Vec2 {
    let (low, high) = pan_limits(grown);
    pan.clamp(low, high)
}

fn root_top_left(pan: Vec2) -> Vec2 {
    cell_top_left(0, 0, pan)
}

fn reroll_top_left() -> Vec2 {
    Vec2::new(REROLL_LEFT, REROLL_TOP)
}

fn inside(point: Vec2, top_left: Vec2, size: Vec2) -> bool {
    point.cmpge(top_left).all() && point.cmple(top_left + size).all()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Target {
    Weapon(usize),
    Node(usize),
    Reroll,
}

fn target_at(first: usize, grown: &Tree, pan: Vec2, point: Vec2) -> Option<Target> {
    let shown = GunKind::ALL.len().min(VISIBLE_ROWS);
    if let Some(slot) = (0..shown).find(|slot| inside(point, row_top_left(*slot), ROW_SIZE)) {
        return Some(Target::Weapon(first + slot));
    }
    if in_view(point) {
        return grown
            .nodes
            .iter()
            .position(|node| {
                inside(
                    point,
                    cell_top_left(node.column, node.row, pan),
                    Vec2::splat(TALENT),
                )
            })
            .map(Target::Node);
    }
    inside(point, reroll_top_left(), REROLL_SIZE).then_some(Target::Reroll)
}

fn on_buy_plate(point: Vec2) -> bool {
    (point - PLATE_CENTER).abs().cmple(PLATE_HIT_HALF).all()
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
            + Vec2::new(
                (ROW_SIZE.x - size.x) / 2.0,
                GUN_TOP + (GUN_BOX.y - size.y) / 2.0,
            ),
        size,
        cell: None,
        layer: GUN_LAYER,
        alpha: 1.0,
    }
}

fn node_elements(
    icon: Icon,
    quality: Quality,
    top_left: Vec2,
    state: BuyState,
    selected: bool,
    alpha: f32,
    layer: i32,
) -> Vec<Element> {
    if alpha <= 0.0 {
        return Vec::new();
    }
    let shown = if state == BuyState::Available && !selected {
        BuyState::Locked
    } else {
        state
    };
    let (texture, row, rows) = match icon {
        Icon::Stat(index) => (tex::UPGRADE_ICONS, index, STATS.len()),
        Icon::Mutation(index) => (tex::MUTATION_ICONS, index, MUTATIONS.len()),
    };
    let mut elements = vec![
        Element::Image {
            texture: tex::TALENT_FRAME,
            top_left,
            size: Vec2::splat(TALENT),
            cell: Some((frame_row(quality, shown), FRAME_ROWS)),
            layer,
            alpha,
        },
        Element::Image {
            texture,
            top_left: top_left + Vec2::splat((TALENT - GLYPH) / 2.0),
            size: Vec2::splat(GLYPH),
            cell: Some((row, rows)),
            layer: layer + 1,
            alpha: if state == BuyState::Locked {
                alpha * LOCKED_ICON_ALPHA
            } else {
                alpha
            },
        },
    ];
    if selected {
        elements.push(Element::Image {
            texture: tex::TALENT_SELECT,
            top_left,
            size: Vec2::splat(TALENT),
            cell: None,
            layer: layer + 2,
            alpha,
        });
    }
    elements
}

fn board(texture: Tex, top_left: Vec2, size: Vec2) -> Element {
    Element::Image {
        texture,
        top_left,
        size,
        cell: None,
        layer: BOARD_LAYER,
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
    let at = (track - thumb) * first as f32 / max_first() as f32;
    vec![
        rect(
            Vec2::new(BAR_LEFT, ROW_TOP),
            Vec2::new(BAR_WIDTH, track),
            ROW_FILL,
            BAR_LAYER,
        ),
        rect(
            Vec2::new(BAR_LEFT, ROW_TOP + at.floor()),
            Vec2::new(BAR_WIDTH, thumb),
            ROW_RIM,
            BAR_LAYER + 1,
        ),
    ]
}

fn links(kind: GunKind, rerolls: u32, grown: &Tree, pan: Vec2) -> Vec<Element> {
    let mut elements = Vec::new();
    let near = FRAME_INSET;
    let far = TALENT - FRAME_INSET;
    for (index, node) in grown.nodes.iter().enumerate() {
        let (from_column, from_row) = node.parent.map_or((0, 0), |parent| {
            (grown.nodes[parent].column, grown.nodes[parent].row)
        });
        let start = cell_top_left(from_column, from_row, pan);
        let end = cell_top_left(node.column, node.row, pan);
        let alpha =
            visibility(start, Vec2::splat(TALENT)).min(visibility(end, Vec2::splat(TALENT)));
        let bottom = start.y + far;
        let top = end.y + near;
        let from_x = start.x + TALENT / 2.0;
        let to_x = end.x + TALENT / 2.0;
        let mut points = vec![Vec2::new(from_x, bottom)];
        if (from_x - to_x).abs() > 1.0 {
            let middle = (bottom + (top - bottom) / 2.0).floor();
            points.push(Vec2::new(from_x, middle));
            points.push(Vec2::new(to_x, middle));
        }
        points.push(Vec2::new(to_x, top));
        let lit = node_state(kind, rerolls, grown, index) == BuyState::Owned;
        draw_path(
            &mut elements,
            &points,
            if lit { LINE_COLOR } else { LINE_OFF },
            alpha,
        );
    }
    elements
}

fn reroll_plate(hover: Hover) -> Vec<Element> {
    let top_left = reroll_top_left();
    vec![
        Element::Image {
            texture: tex::MENU_PLATE_SMALL,
            top_left,
            size: REROLL_SIZE,
            cell: Some((usize::from(hover.reroll), PLATE_ROWS)),
            layer: FRAME_LAYER,
            alpha: 1.0,
        },
        Element::Label {
            content: format!("Reroll {REROLL_COST} g"),
            position: Vec2::new(
                top_left.x + REROLL_SIZE.x / 2.0,
                top_left.y + REROLL_BASELINE,
            ),
            color: if hover.reroll {
                HOVER_COLOR
            } else {
                MENU_COLOR
            },
            size: FontSize::Size16,
            align: Align::Center,
        },
    ]
}

fn icon_of(node: &Node) -> Icon {
    node.mutation.map_or(Icon::Stat(node.stat), Icon::Mutation)
}

fn quality_of(node: &Node) -> Quality {
    if node.mutation.is_some() {
        Quality::Epic
    } else if node.row >= RARE_FROM_ROW {
        Quality::Rare
    } else {
        Quality::Common
    }
}

fn multiplier(node: &Node) -> u32 {
    match quality_of(node) {
        Quality::Common => 1,
        Quality::Rare | Quality::Epic => 2,
    }
}

fn describe(node: &Node) -> (String, [String; 2]) {
    match node.mutation {
        Some(id) => (
            MUTATIONS[id].name.to_owned(),
            MUTATIONS[id].description.map(str::to_owned),
        ),
        None => {
            let stat = &STATS[node.stat];
            (
                stat.name.to_owned(),
                [
                    format!(
                        "+{}{} {}",
                        stat.amount * multiplier(node),
                        stat.unit,
                        stat.noun
                    ),
                    stat.second.to_owned(),
                ],
            )
        }
    }
}

fn detail(kind: GunKind, rerolls: u32, grown: &Tree, index: usize, hover: bool) -> Vec<Element> {
    let centre_x = PANEL_TOP_LEFT.x + PANEL_SIZE.x / 2.0;
    let node = &grown.nodes[index];
    let state = node_state(kind, rerolls, grown, index);
    let (name_of, lines) = describe(node);
    let title = if node.mutation.is_some() {
        name_of
    } else {
        format!("{} {}", name_of, numeral(grown, index))
    };
    let mut elements = vec![Element::Image {
        texture: tex::PAUSEBG,
        top_left: PANEL_TOP_LEFT,
        size: PANEL_SIZE,
        cell: None,
        layer: PANEL_LAYER,
        alpha: 1.0,
    }];
    elements.extend(node_elements(
        icon_of(node),
        quality_of(node),
        Vec2::new(centre_x - TALENT / 2.0, PANEL_TALENT_Y),
        state,
        true,
        1.0,
        PANEL_LAYER + 2,
    ));
    elements.push(Element::Label {
        content: title,
        position: Vec2::new(centre_x, TITLE_BASELINE),
        color: MENU_COLOR,
        size: FontSize::Size22,
        align: Align::Center,
    });
    for (line_index, line) in lines.iter().enumerate() {
        elements.push(Element::Label {
            content: line.clone(),
            position: Vec2::new(
                centre_x,
                DESCRIPTION_TOP + DESCRIPTION_STEP * line_index as f32,
            ),
            color: MENU_COLOR,
            size: FontSize::Size16,
            align: Align::Center,
        });
    }
    let row = match state {
        BuyState::Available if hover => 1,
        BuyState::Available => 0,
        BuyState::Owned | BuyState::Locked => 2,
    };
    elements.push(Element::Image {
        texture: tex::MENU_PLATE_SMALL,
        top_left: PLATE_CENTER - PLATE_SIZE / 2.0,
        size: PLATE_SIZE,
        cell: Some((row, PLATE_ROWS)),
        layer: PANEL_LAYER + 2,
        alpha: 1.0,
    });
    let (text, color) = match state {
        BuyState::Available if hover => ("Buy", HOVER_COLOR),
        BuyState::Available => ("Buy", MENU_COLOR),
        BuyState::Owned => ("Bought", BLOCKED_COLOR),
        BuyState::Locked => ("Can't buy", BLOCKED_COLOR),
    };
    elements.push(Element::Label {
        content: text.to_owned(),
        position: Vec2::new(PLATE_CENTER.x, PLATE_BASELINE),
        color,
        size: FontSize::Size22,
        align: Align::Center,
    });
    if state != BuyState::Owned {
        elements.push(Element::Label {
            content: format!("Cost: {} gold", cost(node)),
            position: Vec2::new(centre_x, COST_BASELINE),
            color: if state == BuyState::Locked {
                BLOCKED_COLOR
            } else {
                COST_COLOR
            },
            size: FontSize::Size16,
            align: Align::Center,
        });
    }
    elements
}

pub(crate) fn layout(
    first: usize,
    selected: usize,
    choice: Option<usize>,
    rerolls: u32,
    pan: Vec2,
    hover: Hover,
) -> Vec<Element> {
    let kind = GunKind::ALL[selected.min(GunKind::ALL.len() - 1)];
    let spec = kind.spec();
    let grown = tree(kind, rerolls);
    let mut elements = vec![
        rect(Vec2::ZERO, crate::viewport::TARGET, DIM_COLOR, DIM_LAYER),
        board(tex::WEAPON_LIST, LIST_TOP_LEFT, LIST_SIZE),
        board(tex::WEAPON_TREE, TREE_TOP_LEFT, TREE_SIZE),
    ];
    for slot in 0..GunKind::ALL.len().min(VISIBLE_ROWS) {
        let index = first + slot;
        let other = GunKind::ALL[index];
        let top_left = row_top_left(slot);
        let chosen = index == selected;
        elements.push(rect(
            top_left,
            ROW_SIZE,
            if chosen { HOVER_COLOR } else { ROW_RIM },
            ROW_LAYER,
        ));
        elements.push(rect(
            top_left + Vec2::splat(2.0),
            ROW_SIZE - Vec2::splat(4.0),
            ROW_BACK,
            ROW_LAYER + 1,
        ));
        elements.push(gun_picture(other, top_left));
        elements.push(Element::Label {
            content: name(other).to_owned(),
            position: Vec2::new(top_left.x + ROW_SIZE.x / 2.0, top_left.y + NAME_BASELINE),
            color: if chosen { HOVER_COLOR } else { MENU_COLOR },
            size: FontSize::Size22,
            align: Align::Center,
        });
    }
    elements.extend(scrollbar(first));
    elements.push(Element::Label {
        content: name(kind).to_owned(),
        position: Vec2::new(CENTER_X, TITLE_Y),
        color: MENU_COLOR,
        size: FontSize::Size27,
        align: Align::Center,
    });
    elements.push(Element::Label {
        content: format!(
            "Damage {}-{}   Rate {}   Clip {}   Reload {:.1}s",
            spec.min_damage,
            spec.max_damage,
            per_second(spec.fire_interval),
            spec.clip_size,
            spec.reload_time
        ),
        position: Vec2::new(CENTER_X, FACTS_Y),
        color: ROW_RIM,
        size: FontSize::Size16,
        align: Align::Center,
    });
    let root = root_top_left(pan);
    let root_alpha = visibility(root, Vec2::splat(TALENT));
    if root_alpha > 0.0 {
        elements.push(Element::Image {
            texture: tex::TALENT_ROOT,
            top_left: root,
            size: Vec2::splat(TALENT),
            cell: None,
            layer: FRAME_LAYER,
            alpha: root_alpha,
        });
    }
    elements.extend(links(kind, rerolls, &grown, pan));
    for (index, node) in grown.nodes.iter().enumerate() {
        let top_left = cell_top_left(node.column, node.row, pan);
        elements.extend(node_elements(
            icon_of(node),
            quality_of(node),
            top_left,
            node_state(kind, rerolls, &grown, index),
            choice == Some(index),
            visibility(top_left, Vec2::splat(TALENT)),
            FRAME_LAYER,
        ));
    }
    elements.extend(reroll_plate(hover));
    elements.push(Element::Image {
        texture: tex::MENU_PLATE,
        top_left: COUNTER_CENTER - COUNTER_SIZE / 2.0,
        size: COUNTER_SIZE,
        cell: Some((0, COUNTER_ROWS)),
        layer: PANEL_LAYER,
        alpha: 1.0,
    });
    elements.push(Element::Label {
        content: format!("Gold: {SAMPLE_GOLD}"),
        position: Vec2::new(COUNTER_CENTER.x, COUNTER_BASELINE),
        color: MENU_COLOR,
        size: FontSize::Size27,
        align: Align::Center,
    });
    if let Some(index) = choice.filter(|index| *index < grown.nodes.len()) {
        elements.extend(detail(kind, rerolls, &grown, index, hover.buy));
    }
    elements
}

#[derive(Debug, Clone, Copy)]
struct Press {
    from: Vec2,
    pan: Vec2,
    moved: bool,
}

#[derive(Debug, Default)]
pub(crate) struct WeaponTable {
    open: bool,
    first: usize,
    selected: usize,
    choice: Option<usize>,
    rerolls: u32,
    pan: Vec2,
    press: Option<Press>,
    was_down: bool,
    hover: Hover,
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
            *self = Self {
                open: true,
                ..Self::default()
            };
            self.rebuild(world, assets);
        }
    }

    pub(crate) fn close(&mut self, world: &mut World) {
        self.open = false;
        self.press = None;
        self.hover = Hover::default();
        clear(world);
    }

    fn scroll(&mut self, rows: i32) {
        let moved = (self.first as i32 + rows).clamp(0, max_first() as i32) as usize;
        if moved != self.first {
            self.first = moved;
            self.dirty = true;
        }
    }

    fn regrow(&mut self) {
        self.choice = None;
        self.pan = Vec2::ZERO;
        self.press = None;
        self.dirty = true;
    }

    fn can_buy(&self, grown: &Tree) -> bool {
        self.choice.is_some_and(|index| {
            index < grown.nodes.len()
                && node_state(GunKind::ALL[self.selected], self.rerolls, grown, index)
                    == BuyState::Available
        })
    }

    pub(crate) fn update(&mut self, world: &mut World, input: &Input, assets: &Assets, dt: f32) {
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
        let grown = tree(GunKind::ALL[self.selected], self.rerolls);
        let point = input.mouse();
        let direction = input.direction();
        if direction != Vec2::ZERO {
            self.pan = clamp_pan(&grown, self.pan - direction * PAN_SPEED * dt);
            self.dirty = true;
        }
        let target = target_at(self.first, &grown, self.pan, point);
        let down = input.mouse_down();
        if input.clicked() {
            match target {
                Some(Target::Weapon(index)) if index != self.selected => {
                    self.selected = index;
                    self.rerolls = 0;
                    self.regrow();
                }
                Some(Target::Reroll) => {
                    self.rerolls += 1;
                    self.regrow();
                }
                _ if in_view(point) => {
                    self.press = Some(Press {
                        from: point,
                        pan: self.pan,
                        moved: false,
                    });
                }
                _ => {}
            }
        }
        if let (true, Some(press)) = (down, self.press.as_mut()) {
            if (point - press.from).length() > DRAG_THRESHOLD {
                press.moved = true;
            }
            if press.moved {
                self.pan = clamp_pan(&grown, press.pan + (point - press.from));
                self.dirty = true;
            }
        }
        if self.was_down && !down {
            if let Some(press) = self.press.take() {
                if !press.moved {
                    self.choice = match target {
                        Some(Target::Node(index)) => Some(index),
                        _ => None,
                    };
                    self.dirty = true;
                }
            }
        }
        self.was_down = down;
        let grown = tree(GunKind::ALL[self.selected], self.rerolls);
        let hover = Hover {
            buy: self.can_buy(&grown) && on_buy_plate(point),
            reroll: target == Some(Target::Reroll),
        };
        if hover != self.hover {
            self.hover = hover;
            self.dirty = true;
        }
        if self.dirty {
            self.dirty = false;
            self.rebuild(world, assets);
        }
    }

    fn rebuild(&self, world: &mut World, assets: &Assets) {
        clear(world);
        for element in layout(
            self.first,
            self.selected,
            self.choice,
            self.rerolls,
            self.pan,
            self.hover,
        ) {
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

    fn images(elements: &[Element], wanted: Tex) -> usize {
        elements
            .iter()
            .filter(
                |element| matches!(element, Element::Image { texture, .. } if *texture == wanted),
            )
            .count()
    }

    fn plain(first: usize, selected: usize) -> Vec<Element> {
        layout(first, selected, None, 0, Vec2::ZERO, Hover::default())
    }

    #[test]
    fn the_icon_tables_match_the_textures() {
        assert_eq!(STATS.len(), weapon_tree::STAT_KINDS);
        assert_eq!(MUTATIONS.len(), weapon_tree::MUTATION_KINDS);
    }

    #[test]
    fn each_visible_row_shows_the_picture_of_its_weapon() {
        let elements = plain(0, 0);

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
        let elements = plain(max_first(), 0);

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
    fn only_the_nodes_inside_the_view_are_drawn() {
        for kind in GunKind::ALL {
            for rerolls in 0..10 {
                let grown = tree(kind, rerolls);
                let selected = GunKind::ALL
                    .iter()
                    .position(|other| *other == kind)
                    .unwrap_or(0);
                let elements = layout(0, selected, None, rerolls, Vec2::ZERO, Hover::default());
                let visible = grown
                    .nodes
                    .iter()
                    .filter(|node| {
                        visibility(
                            cell_top_left(node.column, node.row, Vec2::ZERO),
                            Vec2::splat(TALENT),
                        ) > 0.0
                    })
                    .count();

                assert_eq!(images(&elements, tex::TALENT_FRAME), visible);
            }
        }
    }

    #[test]
    fn panning_brings_the_far_nodes_into_view() {
        let grown = tree(GunKind::ALL[0], 0);
        let (low, high) = pan_limits(&grown);
        let deepest = grown.nodes.iter().map(|node| node.row).max().unwrap_or(0);

        assert!(low.y < 0.0 || high.y < 0.0 || high.y > 0.0);
        let down = Vec2::new(0.0, low.y);
        let elements = layout(0, 0, None, 0, down, Hover::default());
        let seen = grown
            .nodes
            .iter()
            .filter(|node| {
                visibility(
                    cell_top_left(node.column, node.row, down),
                    Vec2::splat(TALENT),
                ) > 0.0
            })
            .count();

        assert_eq!(images(&elements, tex::TALENT_FRAME), seen);
        assert!(deepest == weapon_tree::DEPTH);
        assert!(grown.nodes.iter().any(|node| node.row == deepest
            && visibility(
                cell_top_left(node.column, node.row, down),
                Vec2::splat(TALENT)
            ) > 0.0));
    }

    #[test]
    fn the_pan_is_clamped_to_the_tree() {
        let grown = tree(GunKind::ALL[0], 0);
        let (low, high) = pan_limits(&grown);

        assert_eq!(clamp_pan(&grown, Vec2::splat(1e6)), high);
        assert_eq!(clamp_pan(&grown, Vec2::splat(-1e6)), low);
        assert_eq!(
            clamp_pan(&grown, Vec2::ZERO).clamp(low, high),
            clamp_pan(&grown, Vec2::ZERO)
        );
    }

    #[test]
    fn deeper_nodes_are_rare_and_the_mutation_is_epic() {
        for kind in GunKind::ALL {
            for rerolls in 0..20 {
                for node in &tree(kind, rerolls).nodes {
                    let expected = if node.mutation.is_some() {
                        Quality::Epic
                    } else if node.row >= RARE_FROM_ROW {
                        Quality::Rare
                    } else {
                        Quality::Common
                    };

                    assert_eq!(quality_of(node), expected);
                }
            }
        }
    }

    #[test]
    fn a_rare_upgrade_is_worth_twice_a_common_one() {
        let grown = tree(GunKind::ALL[0], 0);
        let common = grown
            .nodes
            .iter()
            .find(|node| node.mutation.is_none() && quality_of(node) == Quality::Common);
        let rare = grown
            .nodes
            .iter()
            .find(|node| node.mutation.is_none() && quality_of(node) == Quality::Rare);

        let (Some(common), Some(rare)) = (common, rare) else {
            panic!("a tree has both");
        };
        assert_eq!(multiplier(rare), 2 * multiplier(common));
        assert!(describe(rare).1[0] != describe(common).1[0] || rare.stat != common.stat);
    }

    #[test]
    fn rerolling_changes_the_tree_that_is_drawn() {
        let before = plain(0, 0);
        let after = layout(0, 0, None, 1, Vec2::ZERO, Hover::default());

        assert_ne!(before, after);
        assert!(labels(&after).contains(&format!("Reroll {REROLL_COST} g")));
    }

    #[test]
    fn a_chosen_node_opens_the_detail_panel_with_its_description() {
        let grown = tree(GunKind::ALL[0], 0);
        let with = layout(0, 0, Some(0), 0, Vec2::ZERO, Hover::default());
        let shown = labels(&with);

        assert_eq!(images(&with, tex::PAUSEBG), 1);
        assert!(shown.contains(&format!("{} I", STATS[grown.nodes[0].stat].name)));
        assert!(shown.iter().any(|text| text.starts_with("Cost: ")));
        assert_eq!(images(&plain(0, 0), tex::PAUSEBG), 0);
    }

    #[test]
    fn the_mutation_detail_names_the_mutation() {
        let grown = tree(GunKind::ALL[0], 0);
        let Some(index) = grown.nodes.iter().position(|node| node.mutation.is_some()) else {
            panic!("no mutation");
        };
        let id = grown.nodes[index].mutation.unwrap_or(0);
        let shown = labels(&layout(0, 0, Some(index), 0, Vec2::ZERO, Hover::default()));

        assert!(shown.contains(&MUTATIONS[id].name.to_string()));
        assert!(shown.contains(&format!("Cost: {MUTATION_COST} gold")));
    }

    #[test]
    fn a_node_can_only_be_bought_after_its_parent_is_owned() {
        for kind in GunKind::ALL {
            for rerolls in 0..20 {
                let grown = tree(kind, rerolls);
                for (index, node) in grown.nodes.iter().enumerate() {
                    if node_state(kind, rerolls, &grown, index) != BuyState::Available {
                        continue;
                    }
                    if let Some(parent) = node.parent {
                        assert_eq!(
                            node_state(kind, rerolls, &grown, parent),
                            BuyState::Owned,
                            "{kind:?} {rerolls} {index}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn the_header_shows_the_real_stats_of_the_selected_weapon() {
        let spec = GunKind::Shotgun.spec();
        let shown = labels(&plain(0, 1)).join(" ");

        assert!(shown.contains(&format!("Damage {}-{}", spec.min_damage, spec.max_damage)));
        assert!(shown.contains(&format!("Clip {}", spec.clip_size)));
    }

    #[test]
    fn the_pointer_finds_weapons_nodes_and_reroll() {
        let near = Vec2::splat(4.0);
        let grown = tree(GunKind::ALL[0], 0);
        let node = grown.nodes[0];

        assert_eq!(
            target_at(0, &grown, Vec2::ZERO, row_top_left(3) + near),
            Some(Target::Weapon(3))
        );
        assert_eq!(
            target_at(1, &grown, Vec2::ZERO, row_top_left(3) + near),
            Some(Target::Weapon(4))
        );
        assert_eq!(
            target_at(
                0,
                &grown,
                Vec2::ZERO,
                cell_top_left(node.column, node.row, Vec2::ZERO) + near
            ),
            Some(Target::Node(0))
        );
        assert_eq!(
            target_at(0, &grown, Vec2::ZERO, reroll_top_left() + near),
            Some(Target::Reroll)
        );
        assert_eq!(target_at(0, &grown, Vec2::ZERO, Vec2::ZERO), None);
        assert!(on_buy_plate(PLATE_CENTER));
    }

    #[test]
    fn everything_fits_inside_the_panels() {
        let list_end = LIST_TOP_LEFT + LIST_SIZE;
        let tree_end = TREE_TOP_LEFT + TREE_SIZE;
        let last_row = row_top_left(VISIBLE_ROWS - 1) + ROW_SIZE;
        let view_end = VIEW_TOP_LEFT + VIEW_SIZE;
        let reroll_end = reroll_top_left() + REROLL_SIZE;
        let widest = (2 * weapon_tree::HALF_COLUMNS) as f32 * STEP.x + TALENT;
        let tallest = weapon_tree::DEPTH as f32 * STEP.y + TALENT;

        assert!(last_row.y < list_end.y - 12.0);
        assert!(BAR_LEFT + BAR_WIDTH < list_end.x - 12.0);
        const { assert!(VIEW_TOP_LEFT.x >= TREE_TOP_LEFT.x + 18.0) };
        assert!(view_end.x <= tree_end.x - 18.0);
        assert!(view_end.y < reroll_top_left().y);
        assert!(reroll_end.y < tree_end.y - 18.0);
        assert!(widest > VIEW_SIZE.x && tallest > VIEW_SIZE.y);
        assert!(list_end.x < TREE_TOP_LEFT.x);
        assert!(tree_end.x < PANEL_TOP_LEFT.x);
    }

    #[test]
    fn costs_grow_with_the_row() {
        let grown = tree(GunKind::ALL[0], 0);
        let cheapest = grown
            .nodes
            .iter()
            .filter(|node| node.mutation.is_none())
            .map(cost)
            .min()
            .unwrap_or(0);

        assert_eq!(cheapest, 100);
        assert!(
            grown
                .nodes
                .iter()
                .filter(|node| node.mutation.is_none())
                .all(|node| cost(node) < MUTATION_COST)
        );
    }
}
