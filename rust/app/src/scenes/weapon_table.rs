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
const ROOT_TOP: f32 = 94.0;
const TREE_STEP: Vec2 = Vec2::new(120.0, 72.0);
const NODE_TOP: f32 = 186.0;
const RULE_Y: f32 = 340.0;
const MUTATION_LABEL: f32 = 362.0;
const MUTATION_TOP: f32 = 372.0;
const MUTATION_NAME: f32 = 454.0;
const REROLL_TOP: f32 = 462.0;
const REROLL_SIZE: Vec2 = Vec2::new(108.0, 43.0);
const REROLL_BASELINE: f32 = 27.0;
const GLYPH: f32 = 32.0;
const TIERS: usize = 2;
const OFFERS: usize = 3;
const MUTATION_COST: u32 = 400;
const REROLL_COST: u32 = 100;
const SAMPLE_GOLD: u32 = 1240;
const ROW_FILL: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.5);
const ROW_BACK: Vec4 = Vec4::new(0.28, 0.2, 0.2, 0.6);
const ROW_RIM: Vec4 = Vec4::new(225.0 / 255.0, 152.0 / 255.0, 116.0 / 255.0, 1.0);
const EPIC_TEXT: Vec4 = Vec4::new(226.0 / 255.0, 150.0 / 255.0, 1.0, 1.0);
const ROW_LAYER: i32 = 31;
const GUN_LAYER: i32 = 33;
const BAR_LAYER: i32 = 32;
const RULE_LAYER: i32 = 31;

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

    fn description(self) -> [&'static str; 2] {
        match self {
            Self::Damage => ["+5% damage", "on every shot"],
            Self::FireRate => ["+4% fire rate", "less time between shots"],
            Self::Magazine => ["+1 round", "in the magazine"],
        }
    }
}

struct Mutation {
    short: &'static str,
    name: &'static str,
    description: [&'static str; 2],
}

const MUTATIONS: [Mutation; 6] = [
    Mutation {
        short: "Piercing",
        name: "Piercing Rounds",
        description: ["Bullets pierce", "the first enemy"],
    },
    Mutation {
        short: "Explosive",
        name: "Explosive Rounds",
        description: ["Hits explode and hurt", "enemies nearby"],
    },
    Mutation {
        short: "Venom",
        name: "Venom Rounds",
        description: ["Hits poison and leave", "a toxic pool"],
    },
    Mutation {
        short: "Frost",
        name: "Frost Rounds",
        description: ["Hits chill and", "slow enemies"],
    },
    Mutation {
        short: "Ricochet",
        name: "Ricochet",
        description: ["Bullets bounce", "off walls once"],
    },
    Mutation {
        short: "Deadeye",
        name: "Deadeye",
        description: ["Every fifth shot", "is a critical hit"],
    },
];

fn offer(kind: GunKind, rerolls: u32) -> [usize; OFFERS] {
    let gun = GunKind::ALL
        .iter()
        .position(|other| *other == kind)
        .unwrap_or(0) as u32;
    let mut state = gun
        .wrapping_mul(7919)
        .wrapping_add(rerolls.wrapping_mul(104_729))
        .wrapping_add(13);
    let mut pool: Vec<usize> = (0..MUTATIONS.len()).collect();
    let mut picked = [0; OFFERS];
    for slot in &mut picked {
        state = state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        *slot = pool.remove((state >> 16) as usize % pool.len());
    }
    picked
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
    ["I", "II"][tier.min(TIERS - 1)]
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

fn node_state(kind: GunKind, branch: Branch, tier: usize) -> BuyState {
    let level = sample_level(kind, branch);
    if tier < level {
        BuyState::Owned
    } else if tier == level && cost(tier) <= SAMPLE_GOLD {
        BuyState::Available
    } else {
        BuyState::Locked
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Choice {
    Node(usize, usize),
    Mutation(usize),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Hover {
    buy: bool,
    reroll: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Icon {
    Branch(usize),
    Mutation(usize),
}

fn max_first() -> usize {
    GunKind::ALL.len().saturating_sub(VISIBLE_ROWS)
}

fn row_top_left(slot: usize) -> Vec2 {
    Vec2::new(ROW_LEFT, ROW_TOP + ROW_STEP * slot as f32)
}

fn column_x(branch: usize) -> f32 {
    CENTER_X + (branch as f32 - 1.0) * TREE_STEP.x
}

fn node_top_left(branch: usize, tier: usize) -> Vec2 {
    Vec2::new(
        column_x(branch) - TALENT / 2.0,
        NODE_TOP + TREE_STEP.y * tier as f32,
    )
}

fn mutation_top_left(slot: usize) -> Vec2 {
    Vec2::new(column_x(slot) - TALENT / 2.0, MUTATION_TOP)
}

fn root_top_left() -> Vec2 {
    Vec2::new(CENTER_X - TALENT / 2.0, ROOT_TOP)
}

fn reroll_top_left() -> Vec2 {
    Vec2::new(CENTER_X - REROLL_SIZE.x / 2.0, REROLL_TOP)
}

fn inside(point: Vec2, top_left: Vec2, size: Vec2) -> bool {
    point.cmpge(top_left).all() && point.cmple(top_left + size).all()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Target {
    Weapon(usize),
    Choice(Choice),
    Reroll,
}

pub(crate) fn target_at(first: usize, point: Vec2) -> Option<Target> {
    let shown = GunKind::ALL.len().min(VISIBLE_ROWS);
    if let Some(slot) = (0..shown).find(|slot| inside(point, row_top_left(*slot), ROW_SIZE)) {
        return Some(Target::Weapon(first + slot));
    }
    for branch in 0..Branch::ALL.len() {
        for tier in 0..TIERS {
            if inside(point, node_top_left(branch, tier), Vec2::splat(TALENT)) {
                return Some(Target::Choice(Choice::Node(branch, tier)));
            }
        }
    }
    for slot in 0..OFFERS {
        if inside(point, mutation_top_left(slot), Vec2::splat(TALENT)) {
            return Some(Target::Choice(Choice::Mutation(slot)));
        }
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

fn node(
    icon: Icon,
    quality: Quality,
    top_left: Vec2,
    state: BuyState,
    selected: bool,
    layer: i32,
) -> Vec<Element> {
    let shown = if state == BuyState::Available && !selected {
        BuyState::Locked
    } else {
        state
    };
    let (texture, row, rows) = match icon {
        Icon::Branch(branch) => (tex::UPGRADE_ICONS, branch, Branch::ALL.len()),
        Icon::Mutation(index) => (tex::MUTATION_ICONS, index, MUTATIONS.len()),
    };
    let mut elements = vec![
        Element::Image {
            texture: tex::TALENT_FRAME,
            top_left,
            size: Vec2::splat(TALENT),
            cell: Some((frame_row(quality, shown), FRAME_ROWS)),
            layer,
            alpha: 1.0,
        },
        Element::Image {
            texture,
            top_left: top_left + Vec2::splat((TALENT - GLYPH) / 2.0),
            size: Vec2::splat(GLYPH),
            cell: Some((row, rows)),
            layer: layer + 1,
            alpha: if state == BuyState::Locked {
                LOCKED_ICON_ALPHA
            } else {
                1.0
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
            alpha: 1.0,
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

fn links(kind: GunKind) -> Vec<Element> {
    let mut elements = Vec::new();
    let near = FRAME_INSET;
    let far = TALENT - FRAME_INSET;
    for (branch, kind_of) in Branch::ALL.iter().enumerate() {
        let bottom = root_top_left().y + far;
        let top = node_top_left(branch, 0).y + near;
        let middle = (bottom + (top - bottom) / 2.0).floor();
        let mut points = vec![Vec2::new(CENTER_X, bottom)];
        if (column_x(branch) - CENTER_X).abs() > 1.0 {
            points.push(Vec2::new(CENTER_X, middle));
            points.push(Vec2::new(column_x(branch), middle));
        }
        points.push(Vec2::new(column_x(branch), top));
        draw_path(&mut elements, &points, LINE_COLOR, 1.0);
        for tier in 0..TIERS - 1 {
            let from = node_top_left(branch, tier).y + far;
            let to = node_top_left(branch, tier + 1).y + near;
            let color = if node_state(kind, *kind_of, tier) == BuyState::Owned {
                LINE_COLOR
            } else {
                LINE_OFF
            };
            draw_path(
                &mut elements,
                &[
                    Vec2::new(column_x(branch), from),
                    Vec2::new(column_x(branch), to),
                ],
                color,
                1.0,
            );
        }
    }
    elements
}

fn mutation_row(kind: GunKind, rerolls: u32, choice: Option<Choice>, hover: Hover) -> Vec<Element> {
    let mut elements = vec![
        rect(
            Vec2::new(TREE_TOP_LEFT.x + 40.0, RULE_Y),
            Vec2::new(TREE_SIZE.x - 80.0, 2.0),
            LINE_OFF,
            RULE_LAYER,
        ),
        Element::Label {
            content: "Choose a mutation".to_owned(),
            position: Vec2::new(CENTER_X, MUTATION_LABEL),
            color: EPIC_TEXT,
            size: FontSize::Size16,
            align: Align::Center,
        },
    ];
    for (slot, index) in offer(kind, rerolls).into_iter().enumerate() {
        elements.extend(node(
            Icon::Mutation(index),
            Quality::Epic,
            mutation_top_left(slot),
            BuyState::Owned,
            choice == Some(Choice::Mutation(slot)),
            FRAME_LAYER,
        ));
        elements.push(Element::Label {
            content: MUTATIONS[index].short.to_owned(),
            position: Vec2::new(column_x(slot), MUTATION_NAME),
            color: ROW_RIM,
            size: FontSize::Size16,
            align: Align::Center,
        });
    }
    let top_left = reroll_top_left();
    elements.push(Element::Image {
        texture: tex::MENU_PLATE_SMALL,
        top_left,
        size: REROLL_SIZE,
        cell: Some((usize::from(hover.reroll), PLATE_ROWS)),
        layer: FRAME_LAYER,
        alpha: 1.0,
    });
    elements.push(Element::Label {
        content: format!("Reroll {REROLL_COST} g"),
        position: Vec2::new(CENTER_X, top_left.y + REROLL_BASELINE),
        color: if hover.reroll {
            HOVER_COLOR
        } else {
            MENU_COLOR
        },
        size: FontSize::Size16,
        align: Align::Center,
    });
    elements
}

fn detail(kind: GunKind, choice: Choice, rerolls: u32, hover: bool) -> Vec<Element> {
    let centre_x = PANEL_TOP_LEFT.x + PANEL_SIZE.x / 2.0;
    let (icon, quality, state, title, lines, price) = match choice {
        Choice::Node(branch, tier) => {
            let kind_of = Branch::ALL[branch];
            (
                Icon::Branch(branch),
                Quality::Common,
                node_state(kind, kind_of, tier),
                format!("{} {}", kind_of.name(), roman(tier)),
                kind_of.description(),
                cost(tier),
            )
        }
        Choice::Mutation(slot) => {
            let index = offer(kind, rerolls)[slot];
            (
                Icon::Mutation(index),
                Quality::Epic,
                BuyState::Available,
                MUTATIONS[index].name.to_owned(),
                MUTATIONS[index].description,
                MUTATION_COST,
            )
        }
    };
    let mut elements = vec![Element::Image {
        texture: tex::PAUSEBG,
        top_left: PANEL_TOP_LEFT,
        size: PANEL_SIZE,
        cell: None,
        layer: PANEL_LAYER,
        alpha: 1.0,
    }];
    elements.extend(node(
        icon,
        quality,
        Vec2::new(centre_x - TALENT / 2.0, PANEL_TALENT_Y),
        state,
        true,
        PANEL_LAYER + 2,
    ));
    elements.push(Element::Label {
        content: title,
        position: Vec2::new(centre_x, TITLE_BASELINE),
        color: MENU_COLOR,
        size: FontSize::Size22,
        align: Align::Center,
    });
    for (index, line) in lines.iter().enumerate() {
        elements.push(Element::Label {
            content: (*line).to_owned(),
            position: Vec2::new(centre_x, DESCRIPTION_TOP + DESCRIPTION_STEP * index as f32),
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
            content: format!("Cost: {price} gold"),
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
    choice: Option<Choice>,
    rerolls: u32,
    hover: Hover,
) -> Vec<Element> {
    let kind = GunKind::ALL[selected.min(GunKind::ALL.len() - 1)];
    let spec = kind.spec();
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
    elements.push(Element::Image {
        texture: tex::TALENT_ROOT,
        top_left: root_top_left(),
        size: Vec2::splat(TALENT),
        cell: None,
        layer: FRAME_LAYER,
        alpha: 1.0,
    });
    elements.extend(links(kind));
    for (branch, kind_of) in Branch::ALL.iter().enumerate() {
        for tier in 0..TIERS {
            elements.extend(node(
                Icon::Branch(branch),
                Quality::Common,
                node_top_left(branch, tier),
                node_state(kind, *kind_of, tier),
                choice == Some(Choice::Node(branch, tier)),
                FRAME_LAYER,
            ));
        }
    }
    elements.extend(mutation_row(kind, rerolls, choice, hover));
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
    if let Some(choice) = choice {
        elements.extend(detail(kind, choice, rerolls, hover.buy));
    }
    elements
}

#[derive(Debug, Default)]
pub(crate) struct WeaponTable {
    open: bool,
    first: usize,
    selected: usize,
    choice: Option<Choice>,
    rerolls: u32,
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

    fn can_buy(&self) -> bool {
        match self.choice {
            Some(Choice::Node(branch, tier)) => {
                node_state(GunKind::ALL[self.selected], Branch::ALL[branch], tier)
                    == BuyState::Available
            }
            Some(Choice::Mutation(_)) => true,
            None => false,
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
        let point = input.mouse();
        let target = target_at(self.first, point);
        let hover = Hover {
            buy: self.can_buy() && on_buy_plate(point),
            reroll: target == Some(Target::Reroll),
        };
        if hover != self.hover {
            self.hover = hover;
            self.dirty = true;
        }
        if input.clicked() {
            match target {
                Some(Target::Weapon(index)) if index != self.selected => {
                    self.selected = index;
                    self.choice = None;
                    self.rerolls = 0;
                    self.dirty = true;
                }
                Some(Target::Choice(choice)) => {
                    self.choice = Some(choice);
                    self.dirty = true;
                }
                Some(Target::Reroll) => {
                    self.rerolls += 1;
                    if matches!(self.choice, Some(Choice::Mutation(_))) {
                        self.choice = None;
                    }
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
        for element in layout(
            self.first,
            self.selected,
            self.choice,
            self.rerolls,
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
        layout(first, selected, None, 0, Hover::default())
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
    fn the_tree_has_three_branches_of_two_tiers_and_three_offered_mutations() {
        for selected in 0..GunKind::ALL.len() {
            let elements = plain(0, selected);

            assert_eq!(images(&elements, tex::TALENT_FRAME), 9);
            assert_eq!(images(&elements, tex::UPGRADE_ICONS), 6);
            assert_eq!(images(&elements, tex::MUTATION_ICONS), OFFERS);
            assert_eq!(images(&elements, tex::TALENT_ROOT), 1);
        }
    }

    #[test]
    fn an_offer_has_distinct_mutations_and_is_stable() {
        for kind in GunKind::ALL {
            for rerolls in 0..20 {
                let picked = offer(kind, rerolls);

                assert!(picked.iter().all(|index| *index < MUTATIONS.len()));
                assert!(picked[0] != picked[1] && picked[1] != picked[2] && picked[0] != picked[2]);
                assert_eq!(picked, offer(kind, rerolls));
            }
        }
    }

    #[test]
    fn rerolling_changes_the_offer() {
        for kind in GunKind::ALL {
            let first = offer(kind, 0);

            assert!(
                (1..20).any(|rerolls| offer(kind, rerolls) != first),
                "{kind:?}"
            );
        }
    }

    #[test]
    fn the_offered_mutations_are_named_under_their_cards_and_a_reroll_button_is_shown() {
        let shown = labels(&plain(0, 0));

        for index in offer(GunKind::ALL[0], 0) {
            assert!(shown.contains(&MUTATIONS[index].short.to_string()));
        }
        assert!(shown.contains(&format!("Reroll {REROLL_COST} g")));
    }

    #[test]
    fn a_chosen_mutation_opens_the_detail_panel_with_its_description() {
        let index = offer(GunKind::ALL[0], 0)[1];
        let with = layout(0, 0, Some(Choice::Mutation(1)), 0, Hover::default());
        let shown = labels(&with);

        assert_eq!(images(&with, tex::PAUSEBG), 1);
        assert!(shown.contains(&MUTATIONS[index].name.to_string()));
        assert!(shown.contains(&format!("Cost: {MUTATION_COST} gold")));
        assert_eq!(images(&plain(0, 0), tex::PAUSEBG), 0);
    }

    #[test]
    fn a_chosen_node_fills_the_detail_panel() {
        let shown = labels(&layout(0, 0, Some(Choice::Node(0, 0)), 0, Hover::default()));

        assert!(shown.contains(&"Damage I".to_string()));
    }

    #[test]
    fn the_header_shows_the_real_stats_of_the_selected_weapon() {
        let spec = GunKind::Shotgun.spec();
        let shown = labels(&plain(0, 1)).join(" ");

        assert!(shown.contains(&format!("Damage {}-{}", spec.min_damage, spec.max_damage)));
        assert!(shown.contains(&format!("Clip {}", spec.clip_size)));
    }

    #[test]
    fn tiers_unlock_in_order() {
        let kind = GunKind::ALL[0];
        let level = sample_level(kind, Branch::Damage);

        for tier in 0..TIERS {
            let state = node_state(kind, Branch::Damage, tier);
            if tier < level {
                assert_eq!(state, BuyState::Owned);
            } else if tier > level {
                assert_eq!(state, BuyState::Locked);
            }
        }
    }

    #[test]
    fn the_pointer_finds_weapons_nodes_mutations_and_reroll() {
        let near = Vec2::splat(4.0);

        assert_eq!(
            target_at(0, row_top_left(3) + near),
            Some(Target::Weapon(3))
        );
        assert_eq!(
            target_at(1, row_top_left(3) + near),
            Some(Target::Weapon(4))
        );
        assert_eq!(
            target_at(0, node_top_left(1, 1) + near),
            Some(Target::Choice(Choice::Node(1, 1)))
        );
        assert_eq!(
            target_at(0, mutation_top_left(2) + near),
            Some(Target::Choice(Choice::Mutation(2)))
        );
        assert_eq!(target_at(0, reroll_top_left() + near), Some(Target::Reroll));
        assert_eq!(target_at(0, Vec2::ZERO), None);
        assert!(on_buy_plate(PLATE_CENTER));
    }

    #[test]
    fn everything_fits_inside_the_panels() {
        let list_end = LIST_TOP_LEFT + LIST_SIZE;
        let tree_end = TREE_TOP_LEFT + TREE_SIZE;
        let last_row = row_top_left(VISIBLE_ROWS - 1) + ROW_SIZE;
        let last_node = node_top_left(Branch::ALL.len() - 1, TIERS - 1) + Vec2::splat(TALENT);
        let last_card = mutation_top_left(OFFERS - 1) + Vec2::splat(TALENT);
        let reroll_end = reroll_top_left() + REROLL_SIZE;

        assert!(last_row.y < list_end.y - 12.0);
        assert!(BAR_LEFT + BAR_WIDTH < list_end.x - 12.0);
        assert!(last_node.y < RULE_Y);
        const { assert!(RULE_Y < MUTATION_LABEL - 12.0) };
        const { assert!(MUTATION_LABEL < MUTATION_TOP) };
        assert!(last_card.y < MUTATION_NAME - 12.0);
        assert!(MUTATION_NAME < reroll_top_left().y);
        assert!(reroll_end.y < tree_end.y - 18.0);
        assert!(last_card.x < tree_end.x - 18.0);
        assert!(list_end.x < TREE_TOP_LEFT.x);
        assert!(tree_end.x < PANEL_TOP_LEFT.x);
    }

    #[test]
    fn upgrade_costs_grow_with_the_tier() {
        assert!(cost(1) > cost(0));
        assert!(MUTATION_COST > cost(TIERS - 1));
    }
}
