use glam::Vec2;
use hecs::World;

use crate::assets::Assets;
use crate::guns::GunKind;
use crate::input::Input;
use crate::render::{Align, FontSize};

use super::panel::{
    self, ACCENT, BOARD_CENTER_X, BOARD_TOP_LEFT, ButtonStyle, GOLD, GRAY, ORANGE, SALMON, YELLOW,
};
use super::upgrade_menu::{Element, clear, spawn};

const LIST_LEFT: f32 = 204.0;
const ROW_SIZE: Vec2 = Vec2::new(176.0, 46.0);
const ROW_TOP: f32 = 96.0;
const ROW_STEP: f32 = 54.0;
const DETAIL_LEFT: f32 = 404.0;
const DETAIL_WIDTH: f32 = 352.0;
const STAT_TOP: f32 = 204.0;
const STAT_STEP: f32 = 62.0;
const STAT_SIZE: Vec2 = Vec2::new(DETAIL_WIDTH, 54.0);
const BUY_SIZE: Vec2 = Vec2::new(86.0, 26.0);
const PIP_SIZE: Vec2 = Vec2::new(18.0, 8.0);
const PIP_GAP: f32 = 4.0;
const MAX_LEVEL: u32 = 5;
const SAMPLE_GOLD: u32 = 1240;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stat {
    Damage,
    FireRate,
    Magazine,
}

impl Stat {
    const ALL: [Self; 3] = [Self::Damage, Self::FireRate, Self::Magazine];

    fn name(self) -> &'static str {
        match self {
            Self::Damage => "Damage",
            Self::FireRate => "Fire rate",
            Self::Magazine => "Magazine",
        }
    }

    fn step(self) -> &'static str {
        match self {
            Self::Damage => "+5% per level",
            Self::FireRate => "+4% per level",
            Self::Magazine => "+1 round per level",
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

fn sample_level(kind: GunKind, stat: Stat) -> u32 {
    let gun = GunKind::ALL
        .iter()
        .position(|other| *other == kind)
        .unwrap_or(0) as u32;
    let stat = Stat::ALL
        .iter()
        .position(|other| *other == stat)
        .unwrap_or(0) as u32;
    (gun + stat * 2) % (MAX_LEVEL + 1) % 4
}

fn cost(level: u32) -> u32 {
    100 + level * 150
}

fn row_top_left(index: usize) -> Vec2 {
    Vec2::new(LIST_LEFT, ROW_TOP + ROW_STEP * index as f32)
}

fn stat_top_left(index: usize) -> Vec2 {
    Vec2::new(DETAIL_LEFT, STAT_TOP + STAT_STEP * index as f32)
}

fn buy_top_left(index: usize) -> Vec2 {
    stat_top_left(index) + Vec2::new(STAT_SIZE.x - BUY_SIZE.x - 10.0, 14.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Target {
    Weapon(usize),
    Upgrade(usize),
}

pub(crate) fn target_at(point: Vec2) -> Option<Target> {
    if let Some(index) =
        (0..GunKind::ALL.len()).find(|index| panel::contains(row_top_left(*index), ROW_SIZE, point))
    {
        return Some(Target::Weapon(index));
    }
    (0..Stat::ALL.len())
        .find(|index| panel::contains(buy_top_left(*index), BUY_SIZE, point))
        .map(Target::Upgrade)
}

fn per_second(interval: f32) -> String {
    format!("{:.1}/s", 1.0 / interval)
}

pub(crate) fn layout(selected: usize, hover: Option<Target>) -> Vec<Element> {
    let kind = GunKind::ALL[selected.min(GunKind::ALL.len() - 1)];
    let spec = kind.spec();
    let mut elements = panel::frame();
    elements.push(panel::label(
        "Weapon Table",
        Vec2::new(BOARD_CENTER_X, BOARD_TOP_LEFT.y + 32.0),
        ORANGE,
        FontSize::Size32,
        Align::Center,
    ));
    elements.push(panel::label(
        format!("Gold: {SAMPLE_GOLD}"),
        Vec2::new(
            BOARD_TOP_LEFT.x + panel::BOARD_SIZE.x - 24.0,
            BOARD_TOP_LEFT.y + 32.0,
        ),
        GOLD,
        FontSize::Size22,
        Align::Right,
    ));
    for (index, other) in GunKind::ALL.iter().enumerate() {
        let top_left = row_top_left(index);
        let chosen = index == selected;
        let hovered = hover == Some(Target::Weapon(index));
        let rim = if chosen || hovered { YELLOW } else { SALMON };
        elements.extend(panel::card(top_left, ROW_SIZE, rim, chosen));
        elements.push(panel::label(
            name(*other),
            top_left + Vec2::new(14.0, 29.0),
            if chosen || hovered { YELLOW } else { ORANGE },
            FontSize::Size22,
            Align::Left,
        ));
    }
    elements.push(panel::label(
        name(kind),
        Vec2::new(DETAIL_LEFT, 124.0),
        ORANGE,
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
            Vec2::new(
                DETAIL_LEFT + (index % 2) as f32 * 176.0,
                156.0 + (index / 2) as f32 * 24.0,
            ),
            GRAY,
            FontSize::Size16,
            Align::Left,
        ));
    }
    for (index, stat) in Stat::ALL.iter().enumerate() {
        let top_left = stat_top_left(index);
        let level = sample_level(kind, *stat);
        let maxed = level >= MAX_LEVEL;
        let hovered = hover == Some(Target::Upgrade(index));
        elements.extend(panel::card(top_left, STAT_SIZE, SALMON, false));
        elements.push(panel::label(
            stat.name(),
            top_left + Vec2::new(12.0, 22.0),
            ORANGE,
            FontSize::Size22,
            Align::Left,
        ));
        elements.push(panel::label(
            stat.step(),
            top_left + Vec2::new(12.0, 44.0),
            GRAY,
            FontSize::Size16,
            Align::Left,
        ));
        for pip in 0..MAX_LEVEL {
            let color = if pip < level { ACCENT } else { GRAY };
            let at = top_left + Vec2::new(150.0 + (PIP_SIZE.x + PIP_GAP) * pip as f32 - 40.0, 36.0);
            elements.extend(panel::bar(
                at,
                PIP_SIZE,
                if pip < level { 1.0 } else { 0.0 },
                color,
            ));
        }
        let (text, style) = if maxed {
            ("Maxed".to_string(), ButtonStyle::Disabled)
        } else if cost(level) > SAMPLE_GOLD {
            (format!("{} g", cost(level)), ButtonStyle::Disabled)
        } else if hovered {
            (format!("{} g", cost(level)), ButtonStyle::Hover)
        } else {
            (format!("{} g", cost(level)), ButtonStyle::Normal)
        };
        elements.extend(panel::button(
            buy_top_left(index),
            BUY_SIZE,
            &text,
            style,
            FontSize::Size16,
        ));
    }
    elements.push(panel::footer());
    elements
}

#[derive(Debug, Default)]
pub(crate) struct WeaponTable {
    open: bool,
    selected: usize,
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
        if let (true, Some(Target::Weapon(index))) = (input.clicked(), hover) {
            if index != self.selected {
                self.selected = index;
                self.dirty = true;
            }
        }
        if self.dirty {
            self.dirty = false;
            self.rebuild(world, assets);
        }
    }

    fn rebuild(&self, world: &mut World, assets: &Assets) {
        clear(world);
        for element in layout(self.selected, self.hover) {
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

    #[test]
    fn the_list_names_every_weapon() {
        let shown = labels(&layout(0, None));

        for kind in GunKind::ALL {
            assert!(shown.contains(&name(kind).to_string()), "{kind:?}");
        }
    }

    #[test]
    fn the_detail_shows_the_real_stats_of_the_selected_weapon() {
        let spec = GunKind::Shotgun.spec();
        let shown = labels(&layout(1, None));

        assert!(shown.contains(&format!("Damage {}-{}", spec.min_damage, spec.max_damage)));
        assert!(shown.contains(&format!("Clip {}", spec.clip_size)));
    }

    #[test]
    fn the_pointer_finds_weapons_and_upgrade_buttons() {
        assert_eq!(
            target_at(row_top_left(3) + Vec2::splat(4.0)),
            Some(Target::Weapon(3))
        );
        assert_eq!(
            target_at(buy_top_left(1) + Vec2::splat(4.0)),
            Some(Target::Upgrade(1))
        );
        assert_eq!(target_at(Vec2::ZERO), None);
    }

    #[test]
    fn everything_fits_inside_the_board() {
        let board = panel::BOARD_TOP_LEFT + panel::BOARD_SIZE;
        let list = row_top_left(GunKind::ALL.len() - 1) + ROW_SIZE;
        let stats = stat_top_left(Stat::ALL.len() - 1) + STAT_SIZE;

        assert!(list.y < board.y - 20.0);
        assert!(stats.y < board.y - 20.0);
        assert!(stats.x < board.x);
    }

    #[test]
    fn upgrade_costs_grow_with_the_level() {
        assert!(cost(1) > cost(0));
        assert!(cost(MAX_LEVEL) > cost(MAX_LEVEL - 1));
    }
}
