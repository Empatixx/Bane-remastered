use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Assets, Tex, tex};
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Color, Layer, Overlay, Position, PulseUi, ScreenSpace, Size, Sprite, Text, UpgradeUi,
};
use crate::input::Input;
use crate::profile::SharedProfile;
use crate::render::{Align, FontSize};
use crate::rng::Rng;
use crate::upgrades::{Purchase, Quality, ROOT_CELL, Upgrade};

const TALENT: f32 = 64.0;
const ICON: f32 = 32.0;
const STEP: Vec2 = Vec2::new(80.0, 74.0);
const VIEW_TOP_LEFT: Vec2 = Vec2::new(30.0, 20.0);
const VIEW_SIZE: Vec2 = Vec2::new(600.0, 501.0);
const BOARD_MARGIN: f32 = 18.0;
const ROOT_DROP: f32 = 24.0;
const FADE: f32 = 20.0;
const DRAG_THRESHOLD: f32 = 5.0;
const PAN_SPEED: f32 = 380.0;
const PAN_MARGIN: f32 = 120.0;
const DIM_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.55);
const PANEL_TOP_LEFT: Vec2 = Vec2::new(675.0, 150.0);
const PANEL_SIZE: Vec2 = Vec2::new(240.0, 300.0);
const PANEL_TALENT_Y: f32 = 90.0;
const TITLE_BASELINE: f32 = 222.0;
const DESCRIPTION_TOP: f32 = 266.0;
const DESCRIPTION_STEP: f32 = 22.0;
const PLATE_CENTER: Vec2 = Vec2::new(795.0, 392.0);
const PLATE_SIZE: Vec2 = Vec2::new(144.0, 57.0);
const PLATE_HIT_HALF: Vec2 = Vec2::new(63.0, 19.5);
const PLATE_BASELINE: f32 = 400.0;
const COST_BASELINE: f32 = 348.0;
const COST_COLOR: Vec4 = Vec4::new(225.0 / 255.0, 152.0 / 255.0, 116.0 / 255.0, 1.0);
const MENU_COLOR: Vec4 = Vec4::new(222.0 / 255.0, 112.0 / 255.0, 38.0 / 255.0, 1.0);
const HOVER_COLOR: Vec4 = Vec4::new(1.0, 1.0, 0.0, 1.0);
const BLOCKED_COLOR: Vec4 = Vec4::new(112.0 / 255.0, 100.0 / 255.0, 100.0 / 255.0, 1.0);
const LINE_COLOR: Vec4 = Vec4::new(225.0 / 255.0, 152.0 / 255.0, 116.0 / 255.0, 1.0);
const LINE_OFF: Vec4 = Vec4::new(110.0 / 255.0, 84.0 / 255.0, 90.0 / 255.0, 1.0);
const LINE_INK: Vec4 = Vec4::new(14.0 / 255.0, 8.0 / 255.0, 12.0 / 255.0, 1.0);
const FRAME_INSET: f32 = 8.0;
const CAP_REACH: f32 = 4.0;
const PULSE_LAYER: i32 = 34;
const PULSE_SPEED: f32 = 64.0;
const PULSE_GRID: f32 = 2.0;
const PULSE_SIZE: f32 = 4.0;
const PULSE_GAP: f32 = 6.0;
const PULSE_TAIL: usize = 3;
const PULSE_PAUSE: (f32, f32) = (0.35, 1.1);
const PULSE_LIMIT: usize = 7;
const PULSE_HEAD: Vec4 = Vec4::new(1.0, 0.96, 0.78, 1.0);
const LOCKED_ICON_ALPHA: f32 = 0.6;
const DIM_LAYER: i32 = 29;
const BOARD_LAYER: i32 = 30;
const LINE_LAYER: i32 = 31;
const FRAME_LAYER: i32 = 32;
const PANEL_LAYER: i32 = 40;
const PLATE_ROWS: usize = 3;
const COUNTER_CENTER: Vec2 = Vec2::new(795.0, 50.0);
const COUNTER_SIZE: Vec2 = Vec2::new(243.0, 75.0);
const COUNTER_ROWS: usize = 2;
const COUNTER_BASELINE: f32 = 59.0;
const FRAME_ROWS: usize = 9;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Element {
    Image {
        texture: Tex,
        top_left: Vec2,
        size: Vec2,
        cell: Option<(usize, usize)>,
        layer: i32,
        alpha: f32,
    },
    Rect {
        top_left: Vec2,
        size: Vec2,
        color: Vec4,
        layer: i32,
    },
    Label {
        content: String,
        position: Vec2,
        color: Vec4,
        size: FontSize,
        align: Align,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BuyState {
    Available,
    Owned,
    Locked,
}

pub(crate) fn frame_row(quality: Quality, state: BuyState) -> usize {
    let base = match quality {
        Quality::Common => 0,
        Quality::Rare => 3,
        Quality::Epic => 6,
    };
    base + match state {
        BuyState::Owned => 0,
        BuyState::Locked => 1,
        BuyState::Available => 2,
    }
}

pub(crate) fn buy_state(profile: &SharedProfile, upgrade: Upgrade) -> BuyState {
    let snapshot = profile.snapshot();
    if snapshot.owns(upgrade) {
        BuyState::Owned
    } else if snapshot.can_buy(upgrade) {
        BuyState::Available
    } else {
        BuyState::Locked
    }
}

fn node_top_left(cell: (i32, i32), pan: Vec2) -> Vec2 {
    Vec2::new(
        VIEW_TOP_LEFT.x + VIEW_SIZE.x / 2.0 - TALENT / 2.0,
        VIEW_TOP_LEFT.y + ROOT_DROP,
    ) + Vec2::new(cell.0 as f32 * STEP.x, cell.1 as f32 * STEP.y)
        + pan
}

fn visibility(top_left: Vec2, size: Vec2) -> f32 {
    let inside = (top_left - VIEW_TOP_LEFT)
        .min(VIEW_TOP_LEFT + VIEW_SIZE - (top_left + size))
        .min_element();
    (inside / FADE).clamp(0.0, 1.0)
}

fn in_view(point: Vec2) -> bool {
    point.cmpge(VIEW_TOP_LEFT).all() && point.cmple(VIEW_TOP_LEFT + VIEW_SIZE).all()
}

fn pan_limits() -> (Vec2, Vec2) {
    let (mut low, mut high) = (ROOT_CELL, ROOT_CELL);
    for upgrade in Upgrade::ALL {
        let cell = upgrade.cell();
        low = (low.0.min(cell.0), low.1.min(cell.1));
        high = (high.0.max(cell.0), high.1.max(cell.1));
    }
    let origin = node_top_left(ROOT_CELL, Vec2::ZERO);
    let tree_low = origin + Vec2::new(low.0 as f32 * STEP.x, low.1 as f32 * STEP.y);
    let tree_high = origin + Vec2::new(high.0 as f32 * STEP.x, high.1 as f32 * STEP.y) + TALENT;
    let minimum = VIEW_TOP_LEFT + PAN_MARGIN - tree_high;
    let maximum = VIEW_TOP_LEFT + VIEW_SIZE - PAN_MARGIN - tree_low;
    (minimum.min(maximum), minimum.max(maximum))
}

fn clamp_pan(pan: Vec2) -> Vec2 {
    let (low, high) = pan_limits();
    pan.clamp(low, high)
}

pub(super) fn rect(top_left: Vec2, size: Vec2, color: Vec4, layer: i32) -> Element {
    Element::Rect {
        top_left,
        size,
        color,
        layer,
    }
}

fn segment(
    elements: &mut Vec<Element>,
    from: Vec2,
    to: Vec2,
    extend: (f32, f32),
    color: Vec4,
    alpha: f32,
) {
    let vertical = (from.x - to.x).abs() < 1.0;
    let direction = if vertical {
        Vec2::new(0.0, (to.y - from.y).signum())
    } else {
        Vec2::new((to.x - from.x).signum(), 0.0)
    };
    let start = from - direction * extend.0;
    let end = to + direction * extend.1;
    let low = start.min(end);
    let length = (start - end).abs().max_element() + 1.0;
    let (ink_low, ink_size, core_low, core_size) = if vertical {
        (
            Vec2::new(from.x - 2.0, low.y),
            Vec2::new(4.0, length),
            Vec2::new(from.x - 1.0, low.y),
            Vec2::new(2.0, length),
        )
    } else {
        (
            Vec2::new(low.x, from.y - 2.0),
            Vec2::new(length, 4.0),
            Vec2::new(low.x, from.y - 1.0),
            Vec2::new(length, 2.0),
        )
    };
    elements.push(rect(
        ink_low,
        ink_size,
        LINE_INK.truncate().extend(alpha),
        LINE_LAYER,
    ));
    elements.push(rect(
        core_low,
        core_size,
        color.truncate().extend(alpha),
        LINE_LAYER + 1,
    ));
}

fn cap(elements: &mut Vec<Element>, at: Vec2, inward: Vec2, color: Vec4, alpha: f32) {
    let bar = if inward.x != 0.0 {
        Vec2::new(CAP_REACH, 10.0)
    } else {
        Vec2::new(10.0, CAP_REACH)
    };
    let center = at + inward * (CAP_REACH / 2.0);
    elements.push(rect(
        (center - bar / 2.0).floor(),
        bar,
        LINE_INK.truncate().extend(alpha),
        LINE_LAYER,
    ));
    elements.push(rect(
        (center - (bar - Vec2::splat(2.0)) / 2.0).floor(),
        bar - Vec2::splat(2.0),
        color.truncate().extend(alpha),
        LINE_LAYER + 1,
    ));
}

pub(crate) fn edge_path(from: (i32, i32), to: (i32, i32)) -> Vec<Vec2> {
    let (a, b) = (
        node_top_left(from, Vec2::ZERO),
        node_top_left(to, Vec2::ZERO),
    );
    let near = FRAME_INSET;
    let far = TALENT - FRAME_INSET;
    let half = TALENT / 2.0;
    if from.1 == to.1 {
        let (start, end) = if to.0 < from.0 {
            (a.x + near, b.x + far)
        } else {
            (a.x + far, b.x + near)
        };
        return vec![Vec2::new(start, a.y + half), Vec2::new(end, a.y + half)];
    }
    let bottom = a.y + far;
    let top = b.y + near;
    let middle = (bottom + (top - bottom) / 2.0).floor();
    let mut points = vec![Vec2::new(a.x + half, bottom)];
    if (a.x - b.x).abs() > 1.0 {
        points.push(Vec2::new(a.x + half, middle));
        points.push(Vec2::new(b.x + half, middle));
    }
    points.push(Vec2::new(b.x + half, top));
    points
}

fn draw_path(elements: &mut Vec<Element>, points: &[Vec2], color: Vec4, alpha: f32) {
    if alpha <= 0.0 || points.len() < 2 {
        return;
    }
    let last = points.len() - 2;
    for (index, pair) in points.windows(2).enumerate() {
        let extend = (
            if index > 0 { 2.0 } else { 0.0 },
            if index < last { 2.0 } else { 0.0 },
        );
        segment(elements, pair[0], pair[1], extend, color, alpha);
    }
    let first = points[0];
    cap(
        elements,
        first,
        (points[1] - first).normalize_or_zero(),
        color,
        alpha,
    );
    let tail = points[points.len() - 1];
    cap(
        elements,
        tail,
        (points[points.len() - 2] - tail).normalize_or_zero(),
        color,
        alpha,
    );
}

fn path_length(points: &[Vec2]) -> f32 {
    points
        .windows(2)
        .map(|pair| pair[0].distance(pair[1]))
        .sum()
}

fn point_along(points: &[Vec2], distance: f32) -> Vec2 {
    let mut left = distance.max(0.0);
    for pair in points.windows(2) {
        let length = pair[0].distance(pair[1]);
        if left <= length {
            return pair[0].lerp(pair[1], if length > 0.0 { left / length } else { 0.0 });
        }
        left -= length;
    }
    points[points.len() - 1]
}

fn talent(
    upgrade: Upgrade,
    top_left: Vec2,
    state: BuyState,
    selected: bool,
    alpha: f32,
    layer: i32,
) -> Vec<Element> {
    let def = upgrade.def();
    let shown = if state == BuyState::Available && !selected {
        BuyState::Locked
    } else {
        state
    };
    let mut elements = vec![
        Element::Image {
            texture: tex::TALENT_FRAME,
            top_left,
            size: Vec2::splat(TALENT),
            cell: Some((frame_row(def.quality, shown), FRAME_ROWS)),
            layer,
            alpha,
        },
        Element::Image {
            texture: def.texture,
            top_left: top_left + Vec2::splat(TALENT / 4.0),
            size: Vec2::splat(ICON),
            cell: None,
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

pub(crate) fn layout(
    profile: &SharedProfile,
    selected: Option<Upgrade>,
    hover: bool,
    pan: Vec2,
) -> Vec<Element> {
    let mut elements = vec![
        rect(Vec2::ZERO, crate::viewport::TARGET, DIM_COLOR, DIM_LAYER),
        Element::Image {
            texture: tex::TALENT_BOARD,
            top_left: VIEW_TOP_LEFT - Vec2::splat(BOARD_MARGIN),
            size: VIEW_SIZE + Vec2::splat(BOARD_MARGIN * 2.0),
            cell: None,
            layer: BOARD_LAYER,
            alpha: 1.0,
        },
    ];
    let snapshot = profile.snapshot();
    let root = node_top_left(ROOT_CELL, pan);
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
    for upgrade in Upgrade::ALL {
        let top_left = node_top_left(upgrade.cell(), pan);
        let parent = node_top_left(upgrade.parent_cell(), pan);
        let alpha = visibility(top_left, Vec2::splat(TALENT));
        let line_alpha = alpha.min(visibility(parent, Vec2::splat(TALENT)));
        let path: Vec<Vec2> = edge_path(upgrade.parent_cell(), upgrade.cell())
            .into_iter()
            .map(|point| point + pan)
            .collect();
        let color = if snapshot.owns(upgrade) {
            LINE_COLOR
        } else {
            LINE_OFF
        };
        draw_path(&mut elements, &path, color, line_alpha);
        if alpha > 0.0 {
            elements.extend(talent(
                upgrade,
                top_left,
                buy_state(profile, upgrade),
                selected == Some(upgrade),
                alpha,
                FRAME_LAYER,
            ));
        }
    }
    elements.extend(counter(profile));
    if let Some(upgrade) = selected {
        elements.extend(panel(profile, upgrade, hover));
    }
    elements
}

fn counter(profile: &SharedProfile) -> Vec<Element> {
    vec![
        Element::Image {
            texture: tex::MENU_PLATE,
            top_left: COUNTER_CENTER - COUNTER_SIZE / 2.0,
            size: COUNTER_SIZE,
            cell: Some((0, COUNTER_ROWS)),
            layer: PANEL_LAYER,
            alpha: 1.0,
        },
        Element::Label {
            content: format!("Talents: {}", profile.balance()),
            position: Vec2::new(COUNTER_CENTER.x, COUNTER_BASELINE),
            color: MENU_COLOR,
            size: FontSize::Size27,
            align: Align::Center,
        },
    ]
}

fn panel(profile: &SharedProfile, upgrade: Upgrade, hover: bool) -> Vec<Element> {
    let def = upgrade.def();
    let state = buy_state(profile, upgrade);
    let centre_x = PANEL_TOP_LEFT.x + PANEL_SIZE.x / 2.0;
    let mut elements = vec![Element::Image {
        texture: tex::PAUSEBG,
        top_left: PANEL_TOP_LEFT,
        size: PANEL_SIZE,
        cell: None,
        layer: PANEL_LAYER,
        alpha: 1.0,
    }];
    elements.extend(talent(
        upgrade,
        Vec2::new(centre_x - TALENT / 2.0, PANEL_TALENT_Y),
        state,
        true,
        1.0,
        PANEL_LAYER + 2,
    ));
    elements.push(Element::Label {
        content: def.title.to_owned(),
        position: Vec2::new(centre_x, TITLE_BASELINE),
        color: MENU_COLOR,
        size: FontSize::Size22,
        align: Align::Center,
    });
    for (index, line) in def.description.iter().enumerate() {
        elements.push(Element::Label {
            content: line.trim().to_owned(),
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
    match state {
        BuyState::Available => {
            let color = if hover { HOVER_COLOR } else { MENU_COLOR };
            elements.push(Element::Label {
                content: "Buy".to_owned(),
                position: Vec2::new(PLATE_CENTER.x, PLATE_BASELINE),
                color,
                size: FontSize::Size22,
                align: Align::Center,
            });
        }
        BuyState::Owned | BuyState::Locked => {
            elements.push(Element::Label {
                content: if state == BuyState::Owned {
                    "Bought"
                } else {
                    "Can't buy"
                }
                .to_owned(),
                position: Vec2::new(PLATE_CENTER.x, PLATE_BASELINE),
                color: BLOCKED_COLOR,
                size: FontSize::Size22,
                align: Align::Center,
            });
        }
    }
    if state != BuyState::Owned {
        elements.push(Element::Label {
            content: format!(
                "Cost: {} talent{}",
                def.price,
                if def.price == 1 { "" } else { "s" }
            ),
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

pub(crate) fn talent_at(point: Vec2, pan: Vec2) -> Option<Upgrade> {
    if !in_view(point) {
        return None;
    }
    Upgrade::ALL.into_iter().find(|upgrade| {
        let top_left = node_top_left(upgrade.cell(), pan);
        point.cmpge(top_left).all() && point.cmple(top_left + TALENT).all()
    })
}

pub(crate) fn on_buy_plate(point: Vec2) -> bool {
    (point - PLATE_CENTER).abs().cmple(PLATE_HIT_HALF).all()
}

#[derive(Debug, Clone, Copy)]
struct Pulse {
    edge: usize,
    distance: f32,
}

#[derive(Debug, Clone, Copy)]
struct Press {
    from: Vec2,
    pan: Vec2,
    moved: bool,
}

#[derive(Debug, Default)]
pub(crate) struct UpgradeMenu {
    open: bool,
    selected: Option<Upgrade>,
    hover: bool,
    pan: Vec2,
    press: Option<Press>,
    was_down: bool,
    dirty: bool,
    pulses: Vec<Pulse>,
    until_pulse: f32,
    rng: Option<Rng>,
}

impl UpgradeMenu {
    pub(crate) fn is_open(&self) -> bool {
        self.open
    }

    pub(crate) fn toggle(&mut self, world: &mut World, assets: &Assets, profile: &SharedProfile) {
        if self.open {
            self.close(world);
        } else {
            self.open = true;
            self.selected = None;
            self.hover = false;
            self.pan = Vec2::ZERO;
            self.press = None;
            self.was_down = false;
            self.pulses.clear();
            self.until_pulse = 0.0;
            self.rebuild(world, assets, profile);
        }
    }

    pub(crate) fn close(&mut self, world: &mut World) {
        self.open = false;
        self.selected = None;
        self.press = None;
        self.pulses.clear();
        clear(world);
    }

    pub(crate) fn update(
        &mut self,
        world: &mut World,
        input: &Input,
        assets: &Assets,
        profile: &SharedProfile,
        sounds: &mut Vec<AudioCommand>,
        dt: f32,
    ) -> Option<Purchase> {
        if !self.open {
            return None;
        }
        let point = input.mouse();
        let direction = input.direction();
        if direction != Vec2::ZERO {
            self.pan = clamp_pan(self.pan - direction * PAN_SPEED * dt);
            self.dirty = true;
        }
        let down = input.mouse_down();
        if input.clicked() && in_view(point) {
            self.press = Some(Press {
                from: point,
                pan: self.pan,
                moved: false,
            });
        }
        if let (true, Some(press)) = (down, self.press.as_mut()) {
            if (point - press.from).length() > DRAG_THRESHOLD {
                press.moved = true;
            }
            if press.moved {
                self.pan = clamp_pan(press.pan + (point - press.from));
                self.dirty = true;
            }
        }
        if self.was_down && !down {
            if let Some(press) = self.press.take() {
                if !press.moved {
                    self.selected = talent_at(point, self.pan);
                    self.dirty = true;
                }
            }
        }
        self.was_down = down;
        let available = self
            .selected
            .is_some_and(|selected| buy_state(profile, selected) == BuyState::Available);
        let hover = available && on_buy_plate(point);
        if hover != self.hover {
            self.hover = hover;
            self.dirty = true;
        }
        let mut outcome = None;
        if let (true, true, Some(selected)) = (input.clicked(), on_buy_plate(point), self.selected)
        {
            if buy_state(profile, selected) == BuyState::Available {
                let result = profile.buy(selected);
                if result == Purchase::Bought {
                    sounds.push(AudioCommand::Effect(SoundEffect::TalentUnlock));
                }
                outcome = Some(result);
                self.dirty = true;
            }
        }
        if self.dirty {
            self.dirty = false;
            self.rebuild(world, assets, profile);
        }
        self.flow(world, profile, dt);
        outcome
    }

    fn flow(&mut self, world: &mut World, profile: &SharedProfile, dt: f32) {
        let active: Vec<usize> = {
            let snapshot = profile.snapshot();
            Upgrade::ALL
                .iter()
                .enumerate()
                .filter(|(_, upgrade)| snapshot.owns(**upgrade))
                .map(|(index, _)| index)
                .collect()
        };
        self.pulses.retain(|pulse| active.contains(&pulse.edge));
        let old: Vec<Entity> = world.query::<Entity>().with::<&PulseUi>().iter().collect();
        for entity in old {
            let _ = world.despawn(entity);
        }
        self.until_pulse -= dt;
        if self.until_pulse <= 0.0 && self.pulses.len() < PULSE_LIMIT && !active.is_empty() {
            let rng = self.rng.get_or_insert_with(Rng::from_clock);
            let edge = active[rng.below(active.len())];
            self.until_pulse = PULSE_PAUSE.0 + (PULSE_PAUSE.1 - PULSE_PAUSE.0) * rng.float();
            self.pulses.push(Pulse {
                edge,
                distance: 0.0,
            });
        }
        let tail = PULSE_GAP * PULSE_TAIL as f32;
        for pulse in &mut self.pulses {
            pulse.distance += PULSE_SPEED * dt;
        }
        self.pulses.retain(|pulse| {
            let upgrade = Upgrade::ALL[pulse.edge];
            let length = path_length(&edge_path(upgrade.parent_cell(), upgrade.cell()));
            pulse.distance - tail <= length
        });
        for pulse in &self.pulses {
            let upgrade = Upgrade::ALL[pulse.edge];
            let path = edge_path(upgrade.parent_cell(), upgrade.cell());
            let length = path_length(&path);
            let shown = visibility(node_top_left(upgrade.cell(), self.pan), Vec2::splat(TALENT))
                .min(visibility(
                    node_top_left(upgrade.parent_cell(), self.pan),
                    Vec2::splat(TALENT),
                ));
            for piece in 0..PULSE_TAIL {
                let at = pulse.distance - PULSE_GAP * piece as f32;
                if at < 0.0 || at > length || shown <= 0.0 {
                    continue;
                }
                let point = point_along(&path, at) + self.pan;
                let corner =
                    ((point - Vec2::splat(PULSE_SIZE / 2.0)) / PULSE_GRID).floor() * PULSE_GRID;
                let fade = [1.0, 0.7, 0.4][piece.min(2)];
                let color = if piece == 0 { PULSE_HEAD } else { LINE_COLOR };
                world.spawn((
                    PulseUi,
                    Position(corner + Vec2::splat(PULSE_SIZE / 2.0)),
                    Size(Vec2::splat(PULSE_SIZE)),
                    Color(color.truncate().extend(fade * shown)),
                    Layer(PULSE_LAYER),
                    Overlay,
                    ScreenSpace,
                ));
            }
        }
    }

    fn rebuild(&self, world: &mut World, assets: &Assets, profile: &SharedProfile) {
        clear(world);
        for element in layout(profile, self.selected, self.hover, self.pan) {
            spawn(world, element, assets);
        }
    }
}

pub(super) fn clear(world: &mut World) {
    let mut ui: Vec<Entity> = world
        .query::<Entity>()
        .with::<&UpgradeUi>()
        .iter()
        .collect();
    ui.extend(world.query::<Entity>().with::<&PulseUi>().iter());
    for entity in ui {
        let _ = world.despawn(entity);
    }
}

pub(super) fn spawn(world: &mut World, element: Element, assets: &Assets) {
    match element {
        Element::Image {
            texture,
            top_left,
            size,
            cell,
            layer,
            alpha,
        } => {
            if alpha <= 0.0 {
                return;
            }
            let id = assets.textures.get(texture);
            let sprite = match cell {
                Some((row, rows)) => Sprite::cell(id, 0, row, 1, rows),
                None => Sprite::whole(id),
            };
            world.spawn((
                UpgradeUi,
                Position(top_left + size / 2.0),
                Size(size),
                Color(Vec4::new(1.0, 1.0, 1.0, alpha)),
                sprite,
                Layer(layer),
                Overlay,
                ScreenSpace,
            ));
        }
        Element::Rect {
            top_left,
            size,
            color,
            layer,
        } => {
            if color.w <= 0.0 {
                return;
            }
            world.spawn((
                UpgradeUi,
                Position(top_left + size / 2.0),
                Size(size),
                Color(color),
                Layer(layer),
                Overlay,
                ScreenSpace,
            ));
        }
        Element::Label {
            content,
            position,
            color,
            size,
            align,
        } => {
            world.spawn((
                UpgradeUi,
                Position(position),
                Text::new(content, color, size).aligned(align),
                Layer(PANEL_LAYER + 5),
                Overlay,
                ScreenSpace,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use winit::event::{ElementState, MouseButton};

    use crate::components::RunStats;
    use crate::systems::summarize;

    use super::*;

    fn rich_profile() -> SharedProfile {
        let profile = SharedProfile::in_memory();
        let stats = RunStats {
            fired: 500,
            hits: 400,
            kills: 300,
            items: 200,
        };
        profile.record_run(stats, &summarize(stats, 9), true);
        profile
    }

    fn images(elements: &[Element], texture: Tex) -> Vec<(Vec2, Option<(usize, usize)>)> {
        elements
            .iter()
            .filter_map(|element| match element {
                Element::Image {
                    texture: found,
                    top_left,
                    cell,
                    ..
                } if *found == texture => Some((*top_left, *cell)),
                _ => None,
            })
            .collect()
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

    #[test]
    fn the_menu_shows_one_tree_with_a_root_and_fifteen_talents() {
        let profile = SharedProfile::in_memory();
        let elements = layout(&profile, None, false, Vec2::ZERO);
        assert_eq!(images(&elements, tex::TALENT_FRAME).len(), 15);
        assert_eq!(images(&elements, tex::TALENT_ROOT).len(), 1);
        assert_eq!(images(&elements, tex::TALENT_BOARD).len(), 1);
        let lines = elements
            .iter()
            .filter(
                |element| matches!(element, Element::Rect { layer, .. } if *layer == LINE_LAYER),
            )
            .count();
        assert!(lines > 15, "{lines}");
    }

    #[test]
    fn frames_show_quality_and_the_state_of_the_talent() {
        assert_eq!(frame_row(Quality::Common, BuyState::Owned), 0);
        assert_eq!(frame_row(Quality::Common, BuyState::Locked), 1);
        assert_eq!(frame_row(Quality::Common, BuyState::Available), 2);
        assert_eq!(frame_row(Quality::Rare, BuyState::Owned), 3);
        assert_eq!(frame_row(Quality::Epic, BuyState::Locked), 7);
        assert_eq!(frame_row(Quality::Epic, BuyState::Available), 8);
    }

    #[test]
    fn connections_to_owned_talents_are_lit_and_the_others_are_switched_off() {
        let profile = rich_profile();
        let elements = layout(&profile, None, false, Vec2::ZERO);
        let colours: Vec<Vec4> = elements
            .iter()
            .filter_map(|element| match element {
                Element::Rect { color, layer, .. } if *layer == LINE_LAYER + 1 => Some(*color),
                _ => None,
            })
            .collect();
        let lit = colours
            .iter()
            .filter(|c| (c.x, c.y, c.z) == (LINE_COLOR.x, LINE_COLOR.y, LINE_COLOR.z))
            .count();
        let off = colours
            .iter()
            .filter(|c| (c.x, c.y, c.z) == (LINE_OFF.x, LINE_OFF.y, LINE_OFF.z))
            .count();
        assert!(lit > 0 && off > lit, "{lit} {off}");
        assert_eq!(lit + off, colours.len());

        let fresh = SharedProfile::in_memory();
        let beginning = layout(&fresh, None, false, Vec2::ZERO);
        let lit_at_start = beginning
            .iter()
            .filter(|element| matches!(
                element,
                Element::Rect { color, layer, .. }
                    if *layer == LINE_LAYER + 1 && (color.x, color.y) == (LINE_COLOR.x, LINE_COLOR.y)
            ))
            .count();
        assert!(lit_at_start > 0 && lit_at_start <= lit);
    }

    #[test]
    fn connections_touch_the_visible_edge_of_the_squares_they_join() {
        let parent = node_top_left(Upgrade::MovementSpeed.cell(), Vec2::ZERO);
        let child = node_top_left(Upgrade::Discount.cell(), Vec2::ZERO);
        let path = edge_path(Upgrade::MovementSpeed.cell(), Upgrade::Discount.cell());
        assert_eq!(
            path.first().map(|p| p.y),
            Some(parent.y + TALENT - FRAME_INSET)
        );
        assert_eq!(path.last().map(|p| p.y), Some(child.y + FRAME_INSET));
        assert_eq!(path.first().map(|p| p.x), Some(parent.x + TALENT / 2.0));

        let beside = edge_path((-1, 2), (-2, 2));
        let left = node_top_left((-1, 2), Vec2::ZERO);
        let right = node_top_left((-2, 2), Vec2::ZERO);
        assert_eq!(beside.first().map(|p| p.x), Some(left.x + FRAME_INSET));
        assert_eq!(
            beside.last().map(|p| p.x),
            Some(right.x + TALENT - FRAME_INSET)
        );

        let diagonal = edge_path(ROOT_CELL, Upgrade::MovementSpeed.cell());
        assert!(diagonal.len() >= 3);
        assert!(
            diagonal
                .windows(2)
                .all(|pair| pair[0].x == pair[1].x || pair[0].y == pair[1].y)
        );
    }

    #[test]
    fn a_buyable_talent_shows_the_yellow_frame_only_while_it_is_selected() {
        let profile = rich_profile();
        let frame_of = |selected: Option<Upgrade>| {
            let elements = layout(&profile, selected, false, Vec2::ZERO);
            let wanted = node_top_left(Upgrade::Discount.cell(), Vec2::ZERO);
            images(&elements, tex::TALENT_FRAME)
                .into_iter()
                .find(|(at, _)| *at == wanted)
                .and_then(|(_, cell)| cell)
                .map(|(row, _)| row)
        };

        assert_eq!(
            frame_of(None),
            Some(frame_row(Quality::Common, BuyState::Locked))
        );
        assert_eq!(
            frame_of(Some(Upgrade::Discount)),
            Some(frame_row(Quality::Common, BuyState::Available))
        );
    }

    #[test]
    fn the_buy_plate_fits_inside_the_panel_between_the_text_and_the_bottom_rim() {
        let top = PLATE_CENTER.y - PLATE_SIZE.y / 2.0;
        let bottom = PLATE_CENTER.y + PLATE_SIZE.y / 2.0;
        let rim_bottom = PANEL_TOP_LEFT.y + 93.0 * 3.0 + 3.0;
        assert!(bottom < rim_bottom, "{bottom} {rim_bottom}");
        assert!(top > DESCRIPTION_TOP + DESCRIPTION_STEP * 3.0);
        assert!((PLATE_CENTER.x - (PANEL_TOP_LEFT.x + PANEL_SIZE.x / 2.0)).abs() < 3.0);
    }

    #[test]
    fn little_pulses_run_only_along_connections_between_active_talents() {
        let (mut menu, mut world, assets, profile) = opened();
        menu.rng = Some(Rng::new(5));
        let mut input = Input::default();
        let mut seen = 0;
        for _ in 0..300 {
            step(&mut menu, &mut world, &assets, &profile, &mut input);
            let pieces: Vec<Vec2> = world
                .query::<(&Position, &Size)>()
                .with::<&PulseUi>()
                .iter()
                .map(|(position, size)| position.0 - size.0 / 2.0)
                .collect();
            seen = seen.max(pieces.len());
            assert!(pieces.len() <= PULSE_LIMIT * PULSE_TAIL);
            assert!(
                pieces
                    .iter()
                    .all(|at| at.x % PULSE_GRID == 0.0 && at.y % PULSE_GRID == 0.0)
            );
            let snapshot = profile.snapshot();
            assert!(
                menu.pulses
                    .iter()
                    .all(|pulse| snapshot.owns(Upgrade::ALL[pulse.edge]))
            );
        }
        assert!(seen >= 2, "{seen}");

        menu.close(&mut world);
        assert_eq!(world.query::<&PulseUi>().iter().count(), 0);
    }

    #[test]
    fn at_the_start_only_the_free_first_talents_are_connected_to_the_root_and_flow() {
        let assets = Assets::new().expect("assets");
        let profile = SharedProfile::in_memory();
        let mut world = World::new();
        let mut menu = UpgradeMenu::default();
        menu.toggle(&mut world, &assets, &profile);
        menu.rng = Some(Rng::new(9));
        let mut input = Input::default();
        let mut edges = std::collections::BTreeSet::new();

        for _ in 0..600 {
            step(&mut menu, &mut world, &assets, &profile, &mut input);
            edges.extend(menu.pulses.iter().map(|pulse| pulse.edge));
        }

        assert!(!edges.is_empty());
        assert!(edges.iter().all(|edge| Upgrade::ALL[*edge].is_free()));
    }

    #[test]
    fn a_point_travels_along_a_path_with_corners() {
        let path = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 10.0),
            Vec2::new(10.0, 10.0),
        ];
        assert_eq!(path_length(&path), 20.0);
        assert_eq!(point_along(&path, 5.0), Vec2::new(0.0, 5.0));
        assert_eq!(point_along(&path, 15.0), Vec2::new(5.0, 10.0));
        assert_eq!(point_along(&path, 99.0), Vec2::new(10.0, 10.0));
    }

    #[test]
    fn clicking_a_talent_finds_it_and_empty_space_finds_nothing() {
        let first = node_top_left(Upgrade::MovementSpeed.cell(), Vec2::ZERO);
        assert_eq!(
            talent_at(first + Vec2::splat(10.0), Vec2::ZERO),
            Some(Upgrade::MovementSpeed)
        );
        let middle = node_top_left(Upgrade::AdditionalAmmo.cell(), Vec2::ZERO);
        assert_eq!(
            talent_at(middle + Vec2::splat(30.0), Vec2::ZERO),
            Some(Upgrade::AdditionalAmmo)
        );
        assert_eq!(talent_at(Vec2::new(5.0, 5.0), Vec2::ZERO), None);
        assert_eq!(talent_at(Vec2::new(900.0, 100.0), Vec2::ZERO), None);
    }

    #[test]
    fn panning_moves_where_the_talents_are() {
        let pan = Vec2::new(-30.0, 20.0);
        let spot = node_top_left(Upgrade::Discount.cell(), pan) + Vec2::splat(20.0);

        assert_eq!(talent_at(spot, pan), Some(Upgrade::Discount));
        assert_ne!(talent_at(spot, Vec2::ZERO), Some(Upgrade::Discount));
    }

    #[test]
    fn talents_fade_out_towards_the_edge_and_vanish_outside_the_board() {
        let inside = node_top_left(ROOT_CELL, Vec2::ZERO);
        assert_eq!(visibility(inside, Vec2::splat(TALENT)), 1.0);

        let half = Vec2::new(VIEW_TOP_LEFT.x + 10.0, inside.y);
        let partial = visibility(half, Vec2::splat(TALENT));
        assert!(partial > 0.0 && partial < 1.0, "{partial}");

        let outside = Vec2::new(VIEW_TOP_LEFT.x - 40.0, inside.y);
        assert_eq!(visibility(outside, Vec2::splat(TALENT)), 0.0);

        let profile = SharedProfile::in_memory();
        let far = layout(&profile, None, false, Vec2::new(-5000.0, 0.0));
        assert!(images(&far, tex::TALENT_FRAME).is_empty());
    }

    #[test]
    fn the_pan_cannot_leave_the_tree_behind() {
        let (low, high) = pan_limits();
        assert!(low.x < 0.0 && high.x > 0.0);
        assert_eq!(
            clamp_pan(Vec2::new(-9999.0, 9999.0)),
            Vec2::new(low.x, high.y)
        );
        assert_eq!(clamp_pan(Vec2::ZERO), Vec2::ZERO);
    }

    #[test]
    fn the_details_panel_names_the_talent_and_its_price_and_state() {
        let profile = rich_profile();
        let elements = layout(&profile, Some(Upgrade::Discount), false, Vec2::ZERO);
        let shown = labels(&elements);
        assert!(shown.contains(&"Discount".to_owned()));
        assert!(shown.contains(&"Buy".to_owned()));
        assert!(shown.contains(&"Cost: 1 talent".to_owned()));
        assert!(!shown.iter().any(|text| text.ends_with(",-")));
        assert_eq!(
            images(&elements, tex::MENU_PLATE_SMALL),
            vec![(PLATE_CENTER - PLATE_SIZE / 2.0, Some((0, PLATE_ROWS)))]
        );

        let owned = layout(&profile, Some(Upgrade::MovementSpeed), false, Vec2::ZERO);
        assert!(labels(&owned).contains(&"Bought".to_owned()));
        assert_eq!(
            images(&owned, tex::MENU_PLATE_SMALL)[0].1,
            Some((2, PLATE_ROWS))
        );

        let locked = layout(&profile, Some(Upgrade::TrapMaster), false, Vec2::ZERO);
        assert!(labels(&locked).contains(&"Can't buy".to_owned()));
        assert!(labels(&locked).contains(&"Cost: 1 talent".to_owned()));
        assert!(!labels(&owned).iter().any(|text| text.starts_with("Cost")));
    }

    #[test]
    fn the_buy_plate_lights_up_while_hovered() {
        let profile = rich_profile();
        let hovered = layout(&profile, Some(Upgrade::Discount), true, Vec2::ZERO);
        assert_eq!(
            images(&hovered, tex::MENU_PLATE_SMALL)[0].1,
            Some((1, PLATE_ROWS))
        );
    }

    #[test]
    fn the_selected_talent_gets_the_yellow_corners_and_without_one_there_is_no_panel() {
        let profile = SharedProfile::in_memory();
        let none = layout(&profile, None, false, Vec2::ZERO);
        assert!(images(&none, tex::PAUSEBG).is_empty());
        assert!(images(&none, tex::TALENT_SELECT).is_empty());

        let some = layout(&profile, Some(Upgrade::MovementSpeed), false, Vec2::ZERO);
        assert_eq!(images(&some, tex::TALENT_SELECT).len(), 2);
        assert_eq!(images(&some, tex::PAUSEBG).len(), 1);
    }

    #[test]
    fn the_buy_plate_sits_in_the_panel() {
        assert!(on_buy_plate(PLATE_CENTER));
        assert!(on_buy_plate(PLATE_CENTER + Vec2::new(60.0, 15.0)));
        assert!(!on_buy_plate(PLATE_CENTER + Vec2::new(90.0, 0.0)));
        assert!(!on_buy_plate(PLATE_CENTER - Vec2::new(0.0, 60.0)));
    }

    #[test]
    fn buy_state_follows_ownership_and_the_parent_chain() {
        let profile = rich_profile();
        assert_eq!(buy_state(&profile, Upgrade::MovementSpeed), BuyState::Owned);
        assert_eq!(buy_state(&profile, Upgrade::Discount), BuyState::Available);
        assert_eq!(buy_state(&profile, Upgrade::TrapMaster), BuyState::Locked);
        assert_eq!(profile.buy(Upgrade::Discount), Purchase::Bought);
        assert_eq!(
            buy_state(&profile, Upgrade::TrapMaster),
            BuyState::Available
        );
    }

    fn opened() -> (UpgradeMenu, World, Assets, SharedProfile) {
        let assets = Assets::new().expect("assets");
        let profile = rich_profile();
        let mut world = World::new();
        let mut menu = UpgradeMenu::default();
        menu.toggle(&mut world, &assets, &profile);
        (menu, world, assets, profile)
    }

    fn step(
        menu: &mut UpgradeMenu,
        world: &mut World,
        assets: &Assets,
        profile: &SharedProfile,
        input: &mut Input,
    ) -> Option<Purchase> {
        let outcome = menu.update(world, input, assets, profile, &mut Vec::new(), 1.0 / 60.0);
        input.end_frame();
        outcome
    }

    #[test]
    fn a_quick_click_selects_a_talent_and_a_drag_pans_without_selecting() {
        let (mut menu, mut world, assets, profile) = opened();
        let mut input = Input::default();
        let spot = node_top_left(Upgrade::Discount.cell(), Vec2::ZERO) + Vec2::splat(20.0);

        input.set_mouse(spot);
        input.handle_click(MouseButton::Left, ElementState::Pressed);
        step(&mut menu, &mut world, &assets, &profile, &mut input);
        input.handle_click(MouseButton::Left, ElementState::Released);
        step(&mut menu, &mut world, &assets, &profile, &mut input);
        assert_eq!(menu.selected, Some(Upgrade::Discount));

        let start = Vec2::new(300.0, 300.0);
        input.set_mouse(start);
        input.handle_click(MouseButton::Left, ElementState::Pressed);
        step(&mut menu, &mut world, &assets, &profile, &mut input);
        input.set_mouse(start + Vec2::new(-60.0, 25.0));
        step(&mut menu, &mut world, &assets, &profile, &mut input);
        assert!(menu.pan.x < -50.0 && menu.pan.y > 20.0, "{:?}", menu.pan);
        input.handle_click(MouseButton::Left, ElementState::Released);
        step(&mut menu, &mut world, &assets, &profile, &mut input);
        assert_eq!(menu.selected, Some(Upgrade::Discount));
    }

    #[test]
    fn clicking_empty_space_in_the_board_clears_the_selection() {
        let (mut menu, mut world, assets, profile) = opened();
        let mut input = Input::default();
        menu.selected = Some(Upgrade::Discount);

        input.set_mouse(Vec2::new(80.0, 300.0));
        input.handle_click(MouseButton::Left, ElementState::Pressed);
        step(&mut menu, &mut world, &assets, &profile, &mut input);
        input.handle_click(MouseButton::Left, ElementState::Released);
        step(&mut menu, &mut world, &assets, &profile, &mut input);

        assert_eq!(menu.selected, None);
    }

    #[test]
    fn the_movement_keys_pan_the_tree_and_stay_within_the_limits() {
        let (mut menu, mut world, assets, profile) = opened();
        let mut input = Input::default();
        let right = input.bindings().key(crate::controls::Action::MoveRight);

        for _ in 0..600 {
            input.press(right);
            step(&mut menu, &mut world, &assets, &profile, &mut input);
        }

        let (low, _) = pan_limits();
        assert!(menu.pan.x < -50.0);
        assert!(menu.pan.x >= low.x - 0.01);
    }

    #[test]
    fn the_talent_balance_is_shown_in_the_menu_and_follows_a_purchase() {
        let (mut menu, mut world, assets, profile) = opened();
        let shown = |world: &World| -> Vec<String> {
            world
                .query::<&Text>()
                .with::<&UpgradeUi>()
                .iter()
                .map(|text| text.content.clone())
                .filter(|content| content.starts_with("Talents"))
                .collect()
        };
        let before = profile.balance();
        assert_eq!(shown(&world), vec![format!("Talents: {before}")]);

        menu.selected = Some(Upgrade::Discount);
        let mut input = Input::default();
        input.set_mouse(PLATE_CENTER);
        input.handle_click(MouseButton::Left, ElementState::Pressed);
        let outcome = step(&mut menu, &mut world, &assets, &profile, &mut input);

        assert_eq!(outcome, Some(Purchase::Bought));
        assert_eq!(profile.balance(), before - 1);
        assert_eq!(shown(&world), vec![format!("Talents: {}", before - 1)]);
    }

    #[test]
    fn the_balance_plate_sits_above_the_panel_and_does_not_touch_the_talent_shown_in_it() {
        const BOTTOM: f32 = COUNTER_CENTER.y + COUNTER_SIZE.y / 2.0;
        const LEFT: f32 = COUNTER_CENTER.x - COUNTER_SIZE.x / 2.0;
        const RIGHT: f32 = COUNTER_CENTER.x + COUNTER_SIZE.x / 2.0;
        const {
            assert!(BOTTOM <= PANEL_TALENT_Y);
            assert!(LEFT >= PANEL_TOP_LEFT.x - 2.0);
            assert!(RIGHT <= PANEL_TOP_LEFT.x + PANEL_SIZE.x + 2.0);
        }
    }

    #[test]
    fn the_buy_plate_buys_the_selected_talent_and_nothing_else() {
        let (mut menu, mut world, assets, profile) = opened();
        let mut input = Input::default();
        menu.selected = Some(Upgrade::TrapMaster);
        input.set_mouse(PLATE_CENTER);
        input.handle_click(MouseButton::Left, ElementState::Pressed);
        assert_eq!(
            step(&mut menu, &mut world, &assets, &profile, &mut input),
            None
        );
        assert!(!profile.snapshot().owns(Upgrade::TrapMaster));

        menu.selected = Some(Upgrade::Discount);
        input.handle_click(MouseButton::Left, ElementState::Released);
        step(&mut menu, &mut world, &assets, &profile, &mut input);
        input.handle_click(MouseButton::Left, ElementState::Pressed);
        let outcome = step(&mut menu, &mut world, &assets, &profile, &mut input);

        assert_eq!(outcome, Some(Purchase::Bought));
        assert!(profile.snapshot().owns(Upgrade::Discount));
    }
}
