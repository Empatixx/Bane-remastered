use glam::{Vec2, Vec4};
use hecs::World;
use winit::keyboard::KeyCode;

use crate::assets::{Assets, Tex, tex};
use crate::controls;
use crate::guns::GunKind;
use crate::input::Input;
use crate::render::{Align, FontSize};
use crate::audio::{AudioCommand, SoundEffect};
use crate::profile::SharedProfile;
use crate::upgrades::{Purchase, Quality};
use crate::weapon_upgrades::{
    REROLL_COST, WeaponProgress, cost, multiplier, quality_of, stat_of,
};

use super::upgrade_menu::{
    BLOCKED_COLOR, BOARD_LAYER, BuyState, COST_BASELINE, COST_COLOR, COUNTER_BASELINE,
    COUNTER_CENTER, COUNTER_ROWS, COUNTER_SIZE, DESCRIPTION_STEP, DESCRIPTION_TOP, DIM_COLOR,
    DIM_LAYER, Element, FRAME_INSET, FRAME_LAYER, FRAME_ROWS, HOVER_COLOR, LINE_COLOR, LINE_OFF,
    LOCKED_ICON_ALPHA, MENU_COLOR, PANEL_LAYER, PANEL_SIZE, PANEL_TALENT_Y, PANEL_TOP_LEFT,
    PLATE_BASELINE, PLATE_CENTER, PLATE_HIT_HALF, PLATE_ROWS, PLATE_SIZE, TALENT, TITLE_BASELINE,
    clear, draw_path, frame_row, rect, spawn,
};
use crate::weapon_tree::{Node, Tree};

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
const VIEW_SIZE: Vec2 = Vec2::new(TREE_SIZE.x - 36.0, 404.0);
const ROOT_TOP: f32 = VIEW_TOP_LEFT.y + 8.0;
const FADE: f32 = 20.0;
const DRAG_THRESHOLD: f32 = 5.0;
const PAN_SPEED: f32 = 380.0;
const PAN_MARGIN: f32 = 120.0;
const STEP: Vec2 = Vec2::new(80.0, 74.0);
const ARROW_INSET: f32 = 10.0;
const ARROW_LAYER: i32 = 36;
const DICE_SIZE: Vec2 = Vec2::new(54.0, 54.0);
const REROLL_CENTER: Vec2 = Vec2::new(PLATE_CENTER.x, 488.0);
const REROLL_COST_BASELINE: f32 = 532.0;
const HINT_BASELINE: f32 = 492.0;
const GLYPH: f32 = 36.0;
const ROLL_TIME: f32 = 0.7;
const ROLL_FRAMES: usize = 6;
const ROLL_FPS: f32 = 14.0;
const ROLL_HOP: f32 = 22.0;
const DICE_ROWS: usize = 2 + ROLL_FRAMES;
const NUMERALS: [&str; 3] = ["I", "II", "III"];
const ROW_FILL: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.5);
const ROW_BACK: Vec4 = Vec4::new(0.28, 0.2, 0.2, 0.6);
const ROW_RIM: Vec4 = Vec4::new(225.0 / 255.0, 152.0 / 255.0, 116.0 / 255.0, 1.0);
const ROW_LAYER: i32 = 31;
const GUN_LAYER: i32 = 33;
const BAR_LAYER: i32 = 32;

struct Stat {
    name: &'static str,
    unit: &'static str,
    noun: &'static str,
    second: &'static str,
}

const STATS: [Stat; 6] = [
    Stat {
        name: "Damage",
        unit: "%",
        noun: "damage",
        second: "on every shot",
    },
    Stat {
        name: "Fire rate",
        unit: "%",
        noun: "fire rate",
        second: "less time between shots",
    },
    Stat {
        name: "Magazine",
        unit: "",
        noun: "magazine size",
        second: "more rounds before reload",
    },
    Stat {
        name: "Reload",
        unit: "%",
        noun: "reload speed",
        second: "back in the fight sooner",
    },
    Stat {
        name: "Accuracy",
        unit: "%",
        noun: "less spread",
        second: "bullets fly straighter",
    },
    Stat {
        name: "Crit",
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

fn numeral(tree: &Tree, index: usize) -> &'static str {
    let stat = tree.nodes[index].stat;
    let before = tree.nodes[..index]
        .iter()
        .filter(|node| node.mutation.is_none() && node.stat == stat)
        .count();
    NUMERALS[before.min(NUMERALS.len() - 1)]
}

fn node_state(progress: &WeaponProgress, gold: u64, grown: &Tree, index: usize) -> BuyState {
    if progress.owns(index) {
        BuyState::Owned
    } else if progress.can_unlock(grown, index) && cost(&grown.nodes[index]) <= gold {
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

pub(crate) fn row_top_left(slot: usize) -> Vec2 {
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
    REROLL_CENTER - DICE_SIZE / 2.0
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct More {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

fn more_beyond(grown: &Tree, pan: Vec2) -> More {
    let mut more = More::default();
    let view_end = VIEW_TOP_LEFT + VIEW_SIZE;
    let cells = grown
        .nodes
        .iter()
        .map(|node| (node.column, node.row))
        .chain(std::iter::once((0, 0)));
    for (column, row) in cells {
        let top_left = cell_top_left(column, row, pan);
        more.up |= top_left.y + TALENT <= VIEW_TOP_LEFT.y;
        more.down |= top_left.y >= view_end.y;
        more.left |= top_left.x + TALENT <= VIEW_TOP_LEFT.x;
        more.right |= top_left.x >= view_end.x;
    }
    more
}

fn arrow(center: Vec2, toward: Vec2) -> Vec<Element> {
    (0..5)
        .map(|step| {
            let reach = (4 - step) as f32 * 2.0 + 1.0;
            let depth = step as f32 * 2.0;
            let (top_left, size) = if toward.y != 0.0 {
                (
                    Vec2::new(center.x - reach, center.y + toward.y * (depth - 4.0) - 1.0),
                    Vec2::new(reach * 2.0 + 1.0, 2.0),
                )
            } else {
                (
                    Vec2::new(center.x + toward.x * (depth - 4.0) - 1.0, center.y - reach),
                    Vec2::new(2.0, reach * 2.0 + 1.0),
                )
            };
            rect(top_left, size, LINE_COLOR, ARROW_LAYER)
        })
        .collect()
}

fn arrows(grown: &Tree, pan: Vec2) -> Vec<Element> {
    let more = more_beyond(grown, pan);
    let view_end = VIEW_TOP_LEFT + VIEW_SIZE;
    let middle = VIEW_TOP_LEFT + VIEW_SIZE / 2.0;
    let mut elements = Vec::new();
    if more.up {
        elements.extend(arrow(
            Vec2::new(middle.x, VIEW_TOP_LEFT.y + ARROW_INSET),
            Vec2::new(0.0, -1.0),
        ));
    }
    if more.down {
        elements.extend(arrow(
            Vec2::new(middle.x, view_end.y - ARROW_INSET),
            Vec2::new(0.0, 1.0),
        ));
    }
    if more.left {
        elements.extend(arrow(
            Vec2::new(VIEW_TOP_LEFT.x + ARROW_INSET, middle.y),
            Vec2::new(-1.0, 0.0),
        ));
    }
    if more.right {
        elements.extend(arrow(
            Vec2::new(view_end.x - ARROW_INSET, middle.y),
            Vec2::new(1.0, 0.0),
        ));
    }
    elements
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
    inside(point, reroll_top_left(), DICE_SIZE).then_some(Target::Reroll)
}

fn on_buy_plate(point: Vec2) -> bool {
    (point - PLATE_CENTER).abs().cmple(PLATE_HIT_HALF).all()
}

fn damage_text(spec: &crate::guns::GunSpec) -> String {
    let bonus = ((spec.damage_scale - 1.0) * 100.0).round() as i32;
    if bonus > 0 {
        format!("{}-{} +{bonus}%", spec.min_damage, spec.max_damage)
    } else {
        format!("{}-{}", spec.min_damage, spec.max_damage)
    }
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

fn links(progress: &WeaponProgress, gold: u64, grown: &Tree, pan: Vec2) -> Vec<Element> {
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
        let lit = node_state(progress, gold, grown, index) == BuyState::Owned;
        draw_path(
            &mut elements,
            &points,
            if lit { LINE_COLOR } else { LINE_OFF },
            alpha,
        );
    }
    elements
}

fn dice_frame(hover: bool, roll: Option<f32>) -> (usize, f32) {
    match roll {
        Some(time) => {
            let spin = 2 + (time * ROLL_FPS) as usize % ROLL_FRAMES;
            let hop = -(time / ROLL_TIME * std::f32::consts::PI).sin().max(0.0) * ROLL_HOP;
            (spin, hop)
        }
        None => (usize::from(hover), 0.0),
    }
}

fn reroll_plate(hover: Hover, roll: Option<f32>, affordable: bool) -> Vec<Element> {
    let (row, hop) = dice_frame(hover.reroll && affordable, roll);
    let lit = hover.reroll && affordable && roll.is_none();
    vec![
        Element::Image {
            texture: tex::REROLL_DICE,
            top_left: reroll_top_left() + Vec2::new(0.0, hop),
            size: DICE_SIZE,
            cell: Some((row, DICE_ROWS)),
            layer: FRAME_LAYER,
            alpha: 1.0,
        },
        Element::Label {
            content: format!("{REROLL_COST} gold"),
            position: Vec2::new(REROLL_CENTER.x, REROLL_COST_BASELINE),
            color: if lit {
                HOVER_COLOR
            } else if affordable {
                COST_COLOR
            } else {
                BLOCKED_COLOR
            },
            size: FontSize::Size16,
            align: Align::Center,
        },
    ]
}

fn hint(move_keys: &str) -> Element {
    let keys = if move_keys.is_empty() {
        controls::movement_label(&controls::Bindings::default())
    } else {
        move_keys.to_owned()
    };
    Element::Label {
        content: format!("Drag or {keys} to move the tree"),
        position: Vec2::new(CENTER_X, HINT_BASELINE),
        color: LINE_OFF,
        size: FontSize::Size16,
        align: Align::Center,
    }
}

fn icon_of(node: &Node) -> Icon {
    node.mutation.map_or(Icon::Stat(node.stat), Icon::Mutation)
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
                        stat_of(node).amount() * multiplier(node),
                        stat.unit,
                        stat.noun
                    ),
                    stat.second.to_owned(),
                ],
            )
        }
    }
}

fn detail(
    progress: &WeaponProgress,
    gold: u64,
    grown: &Tree,
    index: usize,
    hover: bool,
) -> Vec<Element> {
    let centre_x = PANEL_TOP_LEFT.x + PANEL_SIZE.x / 2.0;
    let node = &grown.nodes[index];
    let state = node_state(progress, gold, grown, index);
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

#[derive(Debug, Clone, Default)]
pub(crate) struct Shown {
    pub(crate) first: usize,
    pub(crate) selected: usize,
    pub(crate) choice: Option<usize>,
    pub(crate) progress: WeaponProgress,
    pub(crate) pan: Vec2,
    pub(crate) hover: Hover,
    pub(crate) gold: u64,
    pub(crate) roll: Option<f32>,
    pub(crate) move_keys: String,
}

pub(crate) fn layout(shown: &Shown) -> Vec<Element> {
    let Shown {
        first,
        selected,
        choice,
        ref progress,
        pan,
        hover,
        gold,
        roll,
        ref move_keys,
    } = *shown;
    let kind = GunKind::ALL[selected.min(GunKind::ALL.len() - 1)];
    let spec = progress.mods(kind).apply(kind.spec());
    let grown = progress.tree(kind);
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
            content: other.name().to_owned(),
            position: Vec2::new(top_left.x + ROW_SIZE.x / 2.0, top_left.y + NAME_BASELINE),
            color: if chosen { HOVER_COLOR } else { MENU_COLOR },
            size: FontSize::Size22,
            align: Align::Center,
        });
    }
    elements.extend(scrollbar(first));
    elements.push(Element::Label {
        content: kind.name().to_owned(),
        position: Vec2::new(CENTER_X, TITLE_Y),
        color: MENU_COLOR,
        size: FontSize::Size27,
        align: Align::Center,
    });
    elements.push(Element::Label {
        content: format!(
            "Damage {}   Rate {}   Clip {}   Reload {:.1}s",
            damage_text(&spec),
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
    elements.extend(links(progress, gold, &grown, pan));
    for (index, node) in grown.nodes.iter().enumerate() {
        let top_left = cell_top_left(node.column, node.row, pan);
        elements.extend(node_elements(
            icon_of(node),
            quality_of(node),
            top_left,
            node_state(progress, gold, &grown, index),
            choice == Some(index),
            visibility(top_left, Vec2::splat(TALENT)),
            FRAME_LAYER,
        ));
    }
    elements.extend(arrows(&grown, pan));
    elements.push(hint(move_keys));
    elements.extend(reroll_plate(
        hover,
        roll,
        progress.gold_after_reroll(gold).is_some(),
    ));
    elements.push(Element::Image {
        texture: tex::MENU_PLATE,
        top_left: COUNTER_CENTER - COUNTER_SIZE / 2.0,
        size: COUNTER_SIZE,
        cell: Some((0, COUNTER_ROWS)),
        layer: PANEL_LAYER,
        alpha: 1.0,
    });
    elements.push(Element::Label {
        content: format!("Gold: {gold}"),
        position: Vec2::new(COUNTER_CENTER.x, COUNTER_BASELINE),
        color: MENU_COLOR,
        size: FontSize::Size27,
        align: Align::Center,
    });
    if let Some(index) = choice.filter(|index| *index < grown.nodes.len()) {
        elements.extend(detail(progress, gold, &grown, index, hover.buy));
    }
    elements
}

#[derive(Debug, Clone, Copy)]
struct Press {
    from: Vec2,
    pan: Vec2,
    moved: bool,
}

/// What happened at the weapon table that the lobby should know about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Outcome {
    Bought,
    Rerolled,
    /// Another gun was chosen from the list; the range hands it over.
    Picked(GunKind),
}

#[derive(Debug, Default)]
pub(crate) struct WeaponTable {
    open: bool,
    first: usize,
    selected: usize,
    choice: Option<usize>,
    progress: WeaponProgress,
    pan: Vec2,
    press: Option<Press>,
    was_down: bool,
    hover: Hover,
    gold: u64,
    roll: Option<f32>,
    move_keys: String,
    dirty: bool,
}

impl WeaponTable {
    pub(crate) fn is_open(&self) -> bool {
        self.open
    }

    /// Opens on the gun in hand, or the first one when the hands are empty.
    pub(crate) fn toggle(
        &mut self,
        world: &mut World,
        assets: &Assets,
        profile: &SharedProfile,
        held: Option<GunKind>,
    ) {
        if self.open {
            self.close(world);
        } else {
            let selected = held
                .and_then(|held| GunKind::ALL.iter().position(|kind| *kind == held))
                .unwrap_or(0);
            *self = Self {
                open: true,
                selected,
                first: selected.min(max_first()),
                ..Self::default()
            };
            self.refresh(profile);
            self.rebuild(world, assets);
        }
    }

    pub(crate) fn close(&mut self, world: &mut World) {
        self.open = false;
        self.press = None;
        self.hover = Hover::default();
        clear(world);
    }

    fn kind(&self) -> GunKind {
        GunKind::ALL[self.selected.min(GunKind::ALL.len() - 1)]
    }

    fn grown(&self) -> Tree {
        self.progress.tree(self.kind())
    }

    fn refresh(&mut self, profile: &SharedProfile) {
        self.progress = profile.weapon(self.kind());
        self.gold = profile.gold();
        self.dirty = true;
    }

    fn scroll(&mut self, rows: i32) {
        let moved = (self.first as i32 + rows).clamp(0, max_first() as i32) as usize;
        if moved != self.first {
            self.first = moved;
            self.dirty = true;
        }
    }

    fn can_reroll(&self) -> bool {
        self.progress.gold_after_reroll(self.gold).is_some()
    }

    fn tick_roll(&mut self, profile: &SharedProfile, dt: f32) -> Option<Outcome> {
        let time = self.roll?;
        self.dirty = true;
        if time + dt < ROLL_TIME {
            self.roll = Some(time + dt);
            return None;
        }
        self.roll = None;
        let rerolled = profile.reroll_weapon(self.kind());
        self.refresh(profile);
        self.regrow();
        rerolled.then_some(Outcome::Rerolled)
    }

    fn regrow(&mut self) {
        self.choice = None;
        self.pan = Vec2::ZERO;
        self.press = None;
        self.dirty = true;
    }

    fn can_buy(&self, grown: &Tree) -> bool {
        self.roll.is_none()
            && self.choice.is_some_and(|index| {
                index < grown.nodes.len()
                    && node_state(&self.progress, self.gold, grown, index) == BuyState::Available
            })
    }

    pub(crate) fn update(
        &mut self,
        world: &mut World,
        input: &Input,
        assets: &Assets,
        profile: &SharedProfile,
        sounds: &mut Vec<AudioCommand>,
        dt: f32,
    ) -> Option<Outcome> {
        if !self.open {
            return None;
        }
        let mut outcome = self.tick_roll(profile, dt);
        self.scroll(-input.scrolled());
        if input.just_pressed(KeyCode::ArrowDown) {
            self.scroll(1);
        }
        if input.just_pressed(KeyCode::ArrowUp) {
            self.scroll(-1);
        }
        let grown = self.grown();
        let point = input.mouse();
        let move_keys = controls::movement_label(input.bindings());
        if move_keys != self.move_keys {
            self.move_keys = move_keys;
            self.dirty = true;
        }
        let direction = input.direction();
        if direction != Vec2::ZERO {
            self.pan = clamp_pan(&grown, self.pan - direction * PAN_SPEED * dt);
            self.dirty = true;
        }
        let target = target_at(self.first, &grown, self.pan, point);
        let down = input.mouse_down();
        if input.clicked() {
            match target {
                _ if self.can_buy(&grown) && on_buy_plate(point) => {
                    if let Some(index) = self.choice {
                        if profile.buy_weapon_node(self.kind(), index) == Purchase::Bought {
                            sounds.push(AudioCommand::Effect(SoundEffect::TalentUnlock));
                            outcome = Some(Outcome::Bought);
                        }
                        self.refresh(profile);
                    }
                }
                Some(Target::Weapon(index)) if index != self.selected && self.roll.is_none() => {
                    self.selected = index;
                    self.refresh(profile);
                    self.regrow();
                    outcome = Some(Outcome::Picked(self.kind()));
                }
                Some(Target::Reroll) => {
                    if self.roll.is_none() && self.can_reroll() {
                        self.roll = Some(0.0);
                        self.dirty = true;
                        sounds.push(AudioCommand::Effect(SoundEffect::Click));
                    }
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
        let grown = self.grown();
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
        outcome
    }

    fn rebuild(&self, world: &mut World, assets: &Assets) {
        clear(world);
        for element in layout(&Shown {
            first: self.first,
            selected: self.selected,
            choice: self.choice,
            progress: self.progress.clone(),
            pan: self.pan,
            hover: self.hover,
            gold: self.gold,
            roll: self.roll,
            move_keys: self.move_keys.clone(),
        }) {
            spawn(world, element, assets);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weapon_tree;
    use crate::weapon_upgrades::MUTATION_COST;

    const TEST_GOLD: u64 = 1240;

    fn tree(kind: GunKind, rerolls: u32) -> Tree {
        weapon_tree::generate(kind, rerolls)
    }

    fn fresh(rerolls: u32) -> WeaponProgress {
        WeaponProgress {
            rerolls,
            ..WeaponProgress::default()
        }
    }

    fn base() -> Shown {
        Shown {
            gold: TEST_GOLD,
            ..Shown::default()
        }
    }

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
        layout(&Shown {
            first,
            selected,
            ..base()
        })
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
            assert!(labels(&elements).contains(&kind.name().to_string()));
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
        assert!(labels(&elements).contains(&last.name().to_string()));
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
                let elements = layout(&Shown {
                    selected,
                    progress: fresh(rerolls),
                    ..base()
                });
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
        let elements = layout(&Shown {
            pan: down,
            ..base()
        });
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
    fn arrows_appear_only_when_the_tree_runs_past_the_view() {
        let grown = tree(GunKind::ALL[0], 0);
        let (low, high) = pan_limits(&grown);

        assert_eq!(more_beyond(&grown, Vec2::ZERO), More::default());
        assert!(arrows(&grown, Vec2::ZERO).is_empty());
        let down = more_beyond(&grown, Vec2::new(0.0, low.y));
        assert!(down.up && !down.down);
        let up = more_beyond(&grown, Vec2::new(0.0, high.y));
        assert!(up.down && !up.up);
        let left = more_beyond(&grown, Vec2::new(low.x, 0.0));
        assert!(left.left);
        assert_eq!(arrows(&grown, Vec2::new(0.0, low.y)).len() % 5, 0);
        assert!(!arrows(&grown, Vec2::new(0.0, low.y)).is_empty());
    }

    #[test]
    fn deeper_nodes_are_rare_and_the_mutation_is_epic() {
        for kind in GunKind::ALL {
            for rerolls in 0..20 {
                for node in &tree(kind, rerolls).nodes {
                    let expected = if node.mutation.is_some() || node.row >= 3 {
                        Quality::Epic
                    } else if node.row >= 2 {
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
    fn every_tree_has_common_rare_and_epic_upgrades_besides_the_mutation() {
        for kind in GunKind::ALL {
            for rerolls in 0..20 {
                let grown = tree(kind, rerolls);
                let has = |wanted: Quality| {
                    grown
                        .nodes
                        .iter()
                        .any(|node| node.mutation.is_none() && quality_of(node) == wanted)
                };

                assert!(has(Quality::Common) && has(Quality::Rare) && has(Quality::Epic));
            }
        }
    }

    #[test]
    fn an_epic_upgrade_is_worth_three_common_ones() {
        let grown = tree(GunKind::ALL[0], 0);
        let epic = grown
            .nodes
            .iter()
            .find(|node| node.mutation.is_none() && quality_of(node) == Quality::Epic);
        let common = grown
            .nodes
            .iter()
            .find(|node| node.mutation.is_none() && quality_of(node) == Quality::Common);

        let (Some(epic), Some(common)) = (epic, common) else {
            panic!("a tree has both");
        };
        assert_eq!(multiplier(epic), 3 * multiplier(common));
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
    fn the_pan_hint_names_the_bound_movement_keys() {
        let rebound = layout(&Shown {
            move_keys: "ZQSD".to_owned(),
            ..base()
        });

        assert!(labels(&rebound).contains(&"Drag or ZQSD to move the tree".to_owned()));
    }

    #[test]
    fn rerolling_changes_the_tree_that_is_drawn() {
        let before = plain(0, 0);
        let after = layout(&Shown {
            progress: fresh(1),
            ..base()
        });

        assert_ne!(before, after);
        assert_eq!(images(&after, tex::REROLL_DICE), 1);
        assert!(labels(&after).contains(&format!("{REROLL_COST} gold")));
        assert!(labels(&after).contains(&"Drag or WASD to move the tree".to_owned()));
    }

    #[test]
    fn a_chosen_node_opens_the_detail_panel_with_its_description() {
        let grown = tree(GunKind::ALL[0], 0);
        let with = layout(&Shown {
            choice: Some(0),
            ..base()
        });
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
        let shown = labels(&layout(&Shown {
            choice: Some(index),
            ..base()
        }));

        assert!(shown.contains(&MUTATIONS[id].name.to_string()));
        assert!(shown.contains(&format!("Cost: {MUTATION_COST} gold")));
    }

    #[test]
    fn a_node_can_only_be_bought_after_its_parent_is_owned() {
        for kind in GunKind::ALL {
            for rerolls in 0..20 {
                let grown = tree(kind, rerolls);
                for (index, node) in grown.nodes.iter().enumerate() {
                    let mut progress = fresh(rerolls);
                    if let Some(parent) = node.parent {
                        progress.owned.insert(parent);
                    }
                    let state = node_state(&progress, TEST_GOLD, &grown, index);
                    if node.parent.is_none() || progress.owns(node.parent.unwrap_or(0)) {
                        assert_ne!(state, BuyState::Locked, "{kind:?} {rerolls} {index}");
                    }
                    let alone = node_state(&fresh(rerolls), TEST_GOLD, &grown, index);
                    if alone != BuyState::Available {
                        continue;
                    }
                    assert!(node.parent.is_none(), "{kind:?} {rerolls} {index}");
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
        let reroll_end = reroll_top_left() + DICE_SIZE;
        let widest = (2 * weapon_tree::HALF_COLUMNS) as f32 * STEP.x + TALENT;
        let tallest = weapon_tree::DEPTH as f32 * STEP.y + TALENT;

        assert!(last_row.y < list_end.y - 12.0);
        assert!(BAR_LEFT + BAR_WIDTH < list_end.x - 12.0);
        const { assert!(VIEW_TOP_LEFT.x >= TREE_TOP_LEFT.x + 18.0) };
        assert!(view_end.x <= tree_end.x - 18.0);
        assert!(view_end.y <= tree_end.y - 18.0);
        const { assert!(REROLL_COST_BASELINE < crate::viewport::TARGET.y) };
        assert!(reroll_end.y < REROLL_COST_BASELINE - 12.0);
        assert!(reroll_top_left().x >= PANEL_TOP_LEFT.x);
        assert!(reroll_top_left().y > PANEL_TOP_LEFT.y + PANEL_SIZE.y + 8.0);
        assert!(HINT_BASELINE < view_end.y);
        assert!(widest <= VIEW_SIZE.x && tallest <= VIEW_SIZE.y);
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

    #[test]
    fn a_reroll_spins_the_dice_first_and_regrows_the_tree_when_it_stops() {
        let profile = SharedProfile::in_memory();
        profile.top_up_gold(REROLL_COST);
        let mut table = WeaponTable {
            open: true,
            pan: Vec2::new(5.0, 5.0),
            ..WeaponTable::default()
        };
        table.refresh(&profile);
        table.roll = Some(0.0);

        assert_eq!(table.tick_roll(&profile, ROLL_TIME / 2.0), None);
        assert_eq!(table.progress.rerolls, 0);
        assert!(table.roll.is_some());

        assert_eq!(table.tick_roll(&profile, ROLL_TIME), Some(Outcome::Rerolled));
        assert_eq!(table.progress.rerolls, 1);
        assert_eq!(profile.weapon(GunKind::ALL[0]).rerolls, 1);
        assert_eq!(table.gold, 0);
        assert_eq!(table.roll, None);
        assert_eq!(table.pan, Vec2::ZERO);
        assert!(!table.can_reroll());
    }

    #[test]
    fn bought_nodes_are_drawn_owned_and_open_their_children() {
        let grown = tree(GunKind::ALL[0], 0);
        let root = grown
            .nodes
            .iter()
            .position(|node| node.parent.is_none())
            .unwrap_or(0);
        let mut progress = fresh(0);
        progress.owned.insert(root);

        assert_eq!(node_state(&progress, 0, &grown, root), BuyState::Owned);
        for (index, node) in grown.nodes.iter().enumerate() {
            if node.parent == Some(root) {
                assert_eq!(
                    node_state(&progress, TEST_GOLD, &grown, index),
                    BuyState::Available
                );
                assert_eq!(node_state(&progress, 0, &grown, index), BuyState::Locked);
            }
        }
        let shown = labels(&layout(&Shown {
            choice: Some(root),
            progress,
            ..base()
        }));
        assert!(shown.contains(&"Bought".to_string()));
    }

    #[test]
    fn the_header_shows_the_bonus_of_the_bought_nodes() {
        let kind = GunKind::ALL[0];
        let grown = tree(kind, 0);
        let mut progress = fresh(0);
        progress.owned = (0..grown.nodes.len()).collect();
        let spec = progress.mods(kind).apply(kind.spec());
        let shown = labels(&layout(&Shown {
            progress,
            ..base()
        }))
        .join(" ");

        assert!(shown.contains(&format!("Clip {}", spec.clip_size)));
        assert!(shown.contains(&format!("Reload {:.1}s", spec.reload_time)));
    }

    #[test]
    fn the_dice_rests_hovers_and_tumbles_with_a_hop() {
        assert_eq!(dice_frame(false, None), (0, 0.0));
        assert_eq!(dice_frame(true, None), (1, 0.0));
        let (row, hop) = dice_frame(true, Some(ROLL_TIME / 2.0));
        assert!((2..DICE_ROWS).contains(&row));
        assert!(hop < -ROLL_HOP / 2.0);
        let rows: std::collections::HashSet<usize> = (0..14)
            .map(|step| dice_frame(false, Some(step as f32 / ROLL_FPS)).0)
            .collect();
        assert_eq!(rows.len(), ROLL_FRAMES);
    }

    #[test]
    fn the_counter_and_the_prices_follow_the_real_gold() {
        let none = layout(&Shown { gold: 0, ..base() });
        let rich = layout(&Shown {
            gold: 99_999,
            ..base()
        });

        assert!(labels(&none).contains(&"Gold: 0".to_string()));
        assert!(labels(&rich).contains(&"Gold: 99999".to_string()));
        let grown = tree(GunKind::ALL[0], 0);
        assert!((0..grown.nodes.len()).all(|index| {
            node_state(&fresh(0), 0, &grown, index) != BuyState::Available
        }));
    }
}
