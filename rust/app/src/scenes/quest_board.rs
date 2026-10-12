use glam::Vec2;
use hecs::World;

use crate::assets::Assets;
use crate::input::Input;
use crate::render::{Align, FontSize};

use super::panel::{
    self, ACCENT, BOARD_CENTER_X, BOARD_TOP_LEFT, ButtonStyle, GOLD, GRAY, ORANGE, SALMON, YELLOW,
};
use super::upgrade_menu::{Element, clear, spawn};

const LIST_LEFT: f32 = 204.0;
const CARD_SIZE: Vec2 = Vec2::new(552.0, 66.0);
const CARD_TOP: f32 = 108.0;
const CARD_STEP: f32 = 76.0;
const TAB_SIZE: Vec2 = Vec2::new(150.0, 30.0);
const TAB_TOP: f32 = 62.0;
const TAB_GAP: f32 = 12.0;
const CLAIM_SIZE: Vec2 = Vec2::new(96.0, 28.0);
const BAR_SIZE: Vec2 = Vec2::new(300.0, 8.0);
const TITLE_BASELINE: f32 = 52.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tab {
    Tasks,
    Challenges,
}

impl Tab {
    const ALL: [Self; 2] = [Self::Tasks, Self::Challenges];

    fn name(self) -> &'static str {
        match self {
            Self::Tasks => "Tasks",
            Self::Challenges => "Challenges",
        }
    }
}

struct Entry {
    title: &'static str,
    detail: &'static str,
    progress: u32,
    goal: u32,
    reward: &'static str,
}

const TASKS: [Entry; 5] = [
    Entry {
        title: "Crypt Cleaner",
        detail: "Defeat 100 enemies",
        progress: 64,
        goal: 100,
        reward: "+1 talent",
    },
    Entry {
        title: "Gold Digger",
        detail: "Collect 500 gold in a single run",
        progress: 500,
        goal: 500,
        reward: "+1 talent",
    },
    Entry {
        title: "Deep Delver",
        detail: "Reach floor 5",
        progress: 3,
        goal: 5,
        reward: "+1 talent",
    },
    Entry {
        title: "Barrel Breaker",
        detail: "Destroy 30 barrels",
        progress: 12,
        goal: 30,
        reward: "+1 talent",
    },
    Entry {
        title: "Gun Collector",
        detail: "Pick up 5 different weapons",
        progress: 2,
        goal: 5,
        reward: "+1 talent",
    },
];

const CHALLENGES: [Entry; 5] = [
    Entry {
        title: "Untouched",
        detail: "Clear a floor without taking damage",
        progress: 0,
        goal: 1,
        reward: "+2 talents",
    },
    Entry {
        title: "Speed Runner",
        detail: "Clear a floor in under 90 seconds",
        progress: 0,
        goal: 1,
        reward: "+2 talents",
    },
    Entry {
        title: "Pistol Only",
        detail: "Reach floor 3 using only the pistol",
        progress: 1,
        goal: 3,
        reward: "+3 talents",
    },
    Entry {
        title: "Poisoned Well",
        detail: "Defeat 20 enemies with poison",
        progress: 20,
        goal: 20,
        reward: "+2 talents",
    },
    Entry {
        title: "Boss Slayer",
        detail: "Defeat a boss without dodging",
        progress: 0,
        goal: 1,
        reward: "+5 talents",
    },
];

fn entries(tab: Tab) -> &'static [Entry] {
    match tab {
        Tab::Tasks => &TASKS,
        Tab::Challenges => &CHALLENGES,
    }
}

fn card_top_left(index: usize) -> Vec2 {
    Vec2::new(LIST_LEFT, CARD_TOP + CARD_STEP * index as f32)
}

fn claim_top_left(index: usize) -> Vec2 {
    card_top_left(index) + Vec2::new(CARD_SIZE.x - CLAIM_SIZE.x - 12.0, 30.0)
}

fn tab_top_left(tab: Tab) -> Vec2 {
    let index = Tab::ALL.iter().position(|other| *other == tab).unwrap_or(0) as f32;
    let width = TAB_SIZE.x * Tab::ALL.len() as f32 + TAB_GAP * (Tab::ALL.len() - 1) as f32;
    Vec2::new(
        BOARD_CENTER_X - width / 2.0 + (TAB_SIZE.x + TAB_GAP) * index,
        TAB_TOP,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Target {
    Tab(Tab),
    Card(usize),
}

pub(crate) fn target_at(tab: Tab, point: Vec2) -> Option<Target> {
    if let Some(found) = Tab::ALL
        .iter()
        .find(|other| panel::contains(tab_top_left(**other), TAB_SIZE, point))
    {
        return Some(Target::Tab(*found));
    }
    (0..entries(tab).len())
        .find(|index| panel::contains(card_top_left(*index), CARD_SIZE, point))
        .map(Target::Card)
}

pub(crate) fn layout(tab: Tab, hover: Option<Target>) -> Vec<Element> {
    let mut elements = panel::frame();
    elements.push(panel::label(
        "Notice Board",
        Vec2::new(BOARD_CENTER_X, BOARD_TOP_LEFT.y + TITLE_BASELINE - 20.0),
        ORANGE,
        FontSize::Size32,
        Align::Center,
    ));
    for other in Tab::ALL {
        let style = if other == tab {
            ButtonStyle::Hover
        } else if hover == Some(Target::Tab(other)) {
            ButtonStyle::Normal
        } else {
            ButtonStyle::Disabled
        };
        elements.extend(panel::button(
            tab_top_left(other),
            TAB_SIZE,
            other.name(),
            style,
            FontSize::Size22,
        ));
    }
    for (index, entry) in entries(tab).iter().enumerate() {
        let top_left = card_top_left(index);
        let done = entry.progress >= entry.goal;
        let hovered = hover == Some(Target::Card(index));
        let rim = if hovered {
            YELLOW
        } else if done {
            GOLD
        } else {
            SALMON
        };
        elements.extend(panel::card(top_left, CARD_SIZE, rim, done));
        elements.push(panel::label(
            entry.title,
            top_left + Vec2::new(14.0, 24.0),
            if hovered { YELLOW } else { ORANGE },
            FontSize::Size22,
            Align::Left,
        ));
        elements.push(panel::label(
            entry.detail,
            top_left + Vec2::new(14.0, 44.0),
            GRAY,
            FontSize::Size16,
            Align::Left,
        ));
        let fraction = entry.progress.min(entry.goal) as f32 / entry.goal as f32;
        elements.extend(panel::bar(
            top_left + Vec2::new(14.0, 52.0),
            BAR_SIZE,
            fraction,
            if done { GOLD } else { ACCENT },
        ));
        elements.push(panel::label(
            format!("{}/{}", entry.progress.min(entry.goal), entry.goal),
            top_left + Vec2::new(14.0 + BAR_SIZE.x + 10.0, 60.0),
            GRAY,
            FontSize::Size16,
            Align::Left,
        ));
        elements.push(panel::label(
            entry.reward,
            top_left + Vec2::new(CARD_SIZE.x - 12.0 - CLAIM_SIZE.x / 2.0, 22.0),
            GOLD,
            FontSize::Size16,
            Align::Center,
        ));
        let style = match (done, hovered) {
            (true, true) => ButtonStyle::Hover,
            (true, false) => ButtonStyle::Normal,
            (false, _) => ButtonStyle::Disabled,
        };
        elements.extend(panel::button(
            claim_top_left(index),
            CLAIM_SIZE,
            if done { "Claim" } else { "Locked" },
            style,
            FontSize::Size16,
        ));
    }
    elements.push(panel::footer());
    elements
}

#[derive(Debug)]
pub(crate) struct QuestBoard {
    open: bool,
    tab: Tab,
    hover: Option<Target>,
    dirty: bool,
}

impl Default for QuestBoard {
    fn default() -> Self {
        Self {
            open: false,
            tab: Tab::Tasks,
            hover: None,
            dirty: false,
        }
    }
}

impl QuestBoard {
    pub(crate) fn is_open(&self) -> bool {
        self.open
    }

    pub(crate) fn toggle(&mut self, world: &mut World, assets: &Assets) {
        if self.open {
            self.close(world);
        } else {
            self.open = true;
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
        let hover = target_at(self.tab, input.mouse());
        if hover != self.hover {
            self.hover = hover;
            self.dirty = true;
        }
        if let (true, Some(Target::Tab(tab))) = (input.clicked(), hover) {
            if tab != self.tab {
                self.tab = tab;
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
        for element in layout(self.tab, self.hover) {
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
    fn each_tab_lists_its_own_entries() {
        let tasks = labels(&layout(Tab::Tasks, None));
        let challenges = labels(&layout(Tab::Challenges, None));

        assert!(tasks.contains(&"Crypt Cleaner".to_string()));
        assert!(!tasks.contains(&"Untouched".to_string()));
        assert!(challenges.contains(&"Untouched".to_string()));
    }

    #[test]
    fn only_finished_entries_offer_a_claim() {
        let shown = labels(&layout(Tab::Tasks, None));

        assert_eq!(shown.iter().filter(|text| *text == "Claim").count(), 1);
        assert_eq!(shown.iter().filter(|text| *text == "Locked").count(), 4);
    }

    #[test]
    fn the_pointer_finds_tabs_and_cards() {
        let tab = tab_top_left(Tab::Challenges) + Vec2::splat(5.0);
        let card = card_top_left(2) + Vec2::splat(5.0);

        assert_eq!(
            target_at(Tab::Tasks, tab),
            Some(Target::Tab(Tab::Challenges))
        );
        assert_eq!(target_at(Tab::Tasks, card), Some(Target::Card(2)));
        assert_eq!(target_at(Tab::Tasks, Vec2::ZERO), None);
    }

    #[test]
    fn every_card_fits_inside_the_board() {
        let last = card_top_left(TASKS.len() - 1) + CARD_SIZE;
        let board = panel::BOARD_TOP_LEFT + panel::BOARD_SIZE;

        assert!(last.y < board.y - 20.0);
        assert!(last.x < board.x);
    }
}
