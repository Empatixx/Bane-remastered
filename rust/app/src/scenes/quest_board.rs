use glam::Vec2;
use hecs::World;

use crate::assets::{Assets, tex};
use crate::input::Input;
use crate::render::{Align, FontSize};

use super::panel::{self, BOARD_CENTER_X, GREEN, INK, RED, SCALE, TAB_IDLE, TRACK_BROWN};
use super::upgrade_menu::{Element, clear, rect, spawn};

const COLUMNS: usize = 3;
const NOTE_SIZE: Vec2 = Vec2::new(44.0 * SCALE, 50.0 * SCALE);
const NOTE_ORIGIN: Vec2 = Vec2::new(218.0, 171.0);
const NOTE_STEP: Vec2 = Vec2::new(176.0, 198.0);
const NOTE_JITTER: [f32; 6] = [0.0, 3.0, -2.0, 3.0, -2.0, 4.0];
const NOTE_INSET: f32 = SCALE;
const VISIBLE: Vec2 = Vec2::new(160.0, 184.0);
const NOTE_ROWS: usize = 4;
const TAB_SIZE: Vec2 = Vec2::new(150.0, 34.0);
const TAB_TOP: f32 = 14.0;
const TAB_GAP: f32 = 24.0;
const BAR_SIZE: Vec2 = Vec2::new(128.0, 8.0);
const DETAIL_WIDTH: usize = 17;
const TAB_LAYER: i32 = 32;
const ICON_SIZE: Vec2 = Vec2::new(27.0, 27.0);

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
    talents: u32,
}

const TASKS: [Entry; 3] = [
    Entry {
        title: "Crypt Cleaner",
        detail: "Defeat 100 enemies",
        progress: 64,
        goal: 100,
        talents: 1,
    },
    Entry {
        title: "Gold Digger",
        detail: "Collect 500 gold in a single run",
        progress: 500,
        goal: 500,
        talents: 1,
    },
    Entry {
        title: "Deep Delver",
        detail: "Reach floor 5",
        progress: 3,
        goal: 5,
        talents: 1,
    },
];

const CHALLENGES: [Entry; 3] = [
    Entry {
        title: "Untouched",
        detail: "Clear a floor without taking damage",
        progress: 0,
        goal: 1,
        talents: 2,
    },
    Entry {
        title: "Pistol Only",
        detail: "Reach floor 3 using only the pistol",
        progress: 1,
        goal: 3,
        talents: 3,
    },
    Entry {
        title: "Boss Slayer",
        detail: "Defeat a boss without dodging",
        progress: 0,
        goal: 1,
        talents: 5,
    },
];

fn entries(tab: Tab) -> &'static [Entry] {
    match tab {
        Tab::Tasks => &TASKS,
        Tab::Challenges => &CHALLENGES,
    }
}

fn note_top_left(index: usize) -> Vec2 {
    let column = (index % COLUMNS) as f32;
    let row = (index / COLUMNS) as f32;
    NOTE_ORIGIN
        + Vec2::new(
            NOTE_STEP.x * column,
            NOTE_STEP.y * row + NOTE_JITTER[index % 6],
        )
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
    Note(usize),
}

pub(crate) fn target_at(tab: Tab, point: Vec2) -> Option<Target> {
    if let Some(found) = Tab::ALL
        .iter()
        .find(|other| panel::contains(tab_top_left(**other), TAB_SIZE, point))
    {
        return Some(Target::Tab(*found));
    }
    (0..entries(tab).len())
        .find(|index| {
            panel::contains(
                note_top_left(*index) + Vec2::splat(NOTE_INSET),
                VISIBLE,
                point,
            )
        })
        .map(Target::Note)
}

pub(crate) fn layout(tab: Tab, hover: Option<Target>) -> Vec<Element> {
    let mut elements = panel::frame(tex::BOARD_NOTICE);
    for other in Tab::ALL {
        let top_left = tab_top_left(other);
        let chosen = other == tab;
        let hovered = hover == Some(Target::Tab(other));
        elements.push(panel::label(
            other.name(),
            Vec2::new(top_left.x + TAB_SIZE.x / 2.0, top_left.y + 24.0),
            if chosen || hovered { INK } else { TAB_IDLE },
            FontSize::Size27,
            Align::Center,
        ));
        if chosen {
            elements.push(rect(
                top_left + Vec2::new(10.0, 29.0),
                Vec2::new(TAB_SIZE.x - 20.0, 3.0),
                INK,
                TAB_LAYER,
            ));
        }
    }
    for (index, entry) in entries(tab).iter().enumerate() {
        let corner = note_top_left(index);
        let origin = corner + Vec2::splat(NOTE_INSET);
        let done = entry.progress >= entry.goal;
        let hovered = hover == Some(Target::Note(index));
        let row = usize::from(hovered) + 2 * usize::from(done);
        elements.push(panel::note(
            tex::NOTE_PAPER,
            corner,
            NOTE_SIZE,
            row,
            NOTE_ROWS,
        ));
        elements.push(panel::label(
            entry.title,
            origin + Vec2::new(VISIBLE.x / 2.0, 50.0),
            INK,
            FontSize::Size22,
            Align::Center,
        ));
        for (line, text) in panel::wrap(entry.detail, DETAIL_WIDTH)
            .into_iter()
            .enumerate()
        {
            elements.push(panel::label(
                text,
                origin + Vec2::new(VISIBLE.x / 2.0, 76.0 + 17.0 * line as f32),
                INK,
                FontSize::Size16,
                Align::Center,
            ));
        }
        let shown = entry.progress.min(entry.goal);
        elements.extend(panel::bar(
            origin + Vec2::new((VISIBLE.x - BAR_SIZE.x) / 2.0, 118.0),
            BAR_SIZE,
            shown as f32 / entry.goal as f32,
            if done { GREEN } else { RED },
            TRACK_BROWN,
        ));
        elements.push(panel::label(
            format!("{shown}/{}", entry.goal),
            origin + Vec2::new(VISIBLE.x / 2.0, 146.0),
            INK,
            FontSize::Size16,
            Align::Center,
        ));
        let middle = origin.x + VISIBLE.x / 2.0;
        elements.push(panel::label(
            format!("+{}", entry.talents),
            Vec2::new(middle + 2.0, origin.y + 171.0),
            INK,
            FontSize::Size22,
            Align::Right,
        ));
        elements.push(panel::note(
            tex::TALENT_ICON,
            Vec2::new(middle + 8.0, origin.y + 148.0),
            ICON_SIZE,
            0,
            1,
        ));
    }
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

    fn papers(elements: &[Element]) -> Vec<usize> {
        elements
            .iter()
            .filter_map(|element| match element {
                Element::Image {
                    texture,
                    cell: Some((row, _)),
                    ..
                } if *texture == tex::NOTE_PAPER => Some(*row),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn each_tab_pins_its_own_papers() {
        let tasks = labels(&layout(Tab::Tasks, None));
        let challenges = labels(&layout(Tab::Challenges, None));

        assert!(tasks.contains(&"Crypt Cleaner".to_string()));
        assert!(!tasks.contains(&"Untouched".to_string()));
        assert!(challenges.contains(&"Untouched".to_string()));
        assert_eq!(papers(&layout(Tab::Tasks, None)).len(), TASKS.len());
    }

    #[test]
    fn finished_papers_carry_the_stamp_and_hovered_ones_are_lifted() {
        let calm = papers(&layout(Tab::Tasks, None));
        let lifted = papers(&layout(Tab::Tasks, Some(Target::Note(0))));

        assert_eq!(calm.iter().filter(|row| **row >= 2).count(), 1);
        assert_eq!(calm[0], 0);
        assert_eq!(lifted[0], 1);
    }

    #[test]
    fn every_paper_shows_its_reward_as_a_talent_icon() {
        let icons = |tab| {
            layout(tab, None)
                .iter()
                .filter(|element| matches!(element, Element::Image { texture, .. } if *texture == tex::TALENT_ICON))
                .count()
        };

        assert_eq!(icons(Tab::Tasks), TASKS.len());
        assert_eq!(icons(Tab::Challenges), CHALLENGES.len());
    }

    #[test]
    fn the_pointer_finds_tabs_and_papers() {
        let tab = tab_top_left(Tab::Challenges) + Vec2::splat(5.0);
        let paper = note_top_left(2) + Vec2::splat(20.0);

        assert_eq!(
            target_at(Tab::Tasks, tab),
            Some(Target::Tab(Tab::Challenges))
        );
        assert_eq!(target_at(Tab::Tasks, paper), Some(Target::Note(2)));
        assert_eq!(target_at(Tab::Tasks, Vec2::ZERO), None);
    }

    #[test]
    fn every_paper_fits_on_the_board() {
        let board = panel::BOARD_TOP_LEFT + panel::BOARD_SIZE;
        for index in 0..TASKS.len() {
            let far = note_top_left(index) + NOTE_SIZE;

            assert!(
                far.x < board.x && far.y < board.y - 12.0,
                "{index}: {far:?}"
            );
        }
        assert!(note_top_left(0).y > TAB_TOP + TAB_SIZE.y);
    }
}
