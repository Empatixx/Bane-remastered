use glam::Vec2;
use hecs::World;

use crate::assets::{Assets, tex};
use crate::input::Input;
use crate::profile::SharedProfile;
use crate::progress::{BoardView, ChallengeId, NoteView};
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
pub(crate) const ACTIVE_MARK: &str = "Active";

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

fn notes(tab: Tab, view: &BoardView) -> &[NoteView] {
    match tab {
        Tab::Tasks => &view.missions,
        Tab::Challenges => &view.challenges,
    }
}

pub(crate) fn note_top_left(index: usize) -> Vec2 {
    let column = (index % COLUMNS) as f32;
    let row = (index / COLUMNS) as f32;
    NOTE_ORIGIN
        + Vec2::new(
            NOTE_STEP.x * column,
            NOTE_STEP.y * row + NOTE_JITTER[index % 6],
        )
}

pub(crate) fn tab_top_left(tab: Tab) -> Vec2 {
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

pub(crate) fn target_at(tab: Tab, view: &BoardView, point: Vec2) -> Option<Target> {
    if let Some(found) = Tab::ALL
        .iter()
        .find(|other| panel::contains(tab_top_left(**other), TAB_SIZE, point))
    {
        return Some(Target::Tab(*found));
    }
    (0..notes(tab, view).len())
        .find(|index| {
            panel::contains(
                note_top_left(*index) + Vec2::splat(NOTE_INSET),
                VISIBLE,
                point,
            )
        })
        .map(Target::Note)
}

pub(crate) fn layout(tab: Tab, view: &BoardView, hover: Option<Target>) -> Vec<Element> {
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
    for (index, note) in notes(tab, view).iter().enumerate() {
        let corner = note_top_left(index);
        let origin = corner + Vec2::splat(NOTE_INSET);
        let done = note.done;
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
            &note.title,
            origin + Vec2::new(VISIBLE.x / 2.0, 50.0),
            INK,
            FontSize::Size22,
            Align::Center,
        ));
        if note.active {
            elements.push(panel::label(
                ACTIVE_MARK,
                origin + Vec2::new(VISIBLE.x / 2.0, 28.0),
                GREEN,
                FontSize::Size16,
                Align::Center,
            ));
        }
        for (line, text) in panel::wrap(&note.detail, DETAIL_WIDTH)
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
        let shown = note.progress.min(note.goal);
        elements.extend(panel::bar(
            origin + Vec2::new((VISIBLE.x - BAR_SIZE.x) / 2.0, 118.0),
            BAR_SIZE,
            shown as f32 / note.goal as f32,
            if done { GREEN } else { RED },
            TRACK_BROWN,
        ));
        elements.push(panel::label(
            format!("{shown}/{}", note.goal),
            origin + Vec2::new(VISIBLE.x / 2.0, 146.0),
            INK,
            FontSize::Size16,
            Align::Center,
        ));
        let middle = origin.x + VISIBLE.x / 2.0;
        elements.push(panel::label(
            format!("+{}", note.talents),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BoardAction {
    Claimed(u32),
    Toggled,
}

#[derive(Debug)]
pub(crate) struct QuestBoard {
    open: bool,
    view: BoardView,
    tab: Tab,
    hover: Option<Target>,
    dirty: bool,
}

impl Default for QuestBoard {
    fn default() -> Self {
        Self {
            open: false,
            view: BoardView::default(),
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

    pub(crate) fn toggle(&mut self, world: &mut World, assets: &Assets, profile: &SharedProfile) {
        if self.open {
            self.close(world);
        } else {
            self.open = true;
            self.hover = None;
            self.view = profile.board();
            self.rebuild(world, assets);
        }
    }

    pub(crate) fn close(&mut self, world: &mut World) {
        self.open = false;
        self.hover = None;
        clear(world);
    }

    pub(crate) fn update(
        &mut self,
        world: &mut World,
        input: &Input,
        assets: &Assets,
        profile: &SharedProfile,
    ) -> Option<BoardAction> {
        if !self.open {
            return None;
        }
        let hover = target_at(self.tab, &self.view, input.mouse());
        if hover != self.hover {
            self.hover = hover;
            self.dirty = true;
        }
        let mut action = None;
        if let (true, Some(target)) = (input.clicked(), hover) {
            match (target, self.tab) {
                (Target::Tab(tab), _) => {
                    if tab != self.tab {
                        self.tab = tab;
                        self.dirty = true;
                    }
                }
                (Target::Note(index), Tab::Tasks) => {
                    action = profile.claim_mission(index).map(BoardAction::Claimed);
                }
                (Target::Note(index), Tab::Challenges) => {
                    if let Some(id) = ChallengeId::ALL.get(index) {
                        profile.toggle_challenge(*id);
                        action = Some(BoardAction::Toggled);
                    }
                }
            }
            if action.is_some() {
                self.view = profile.board();
                self.dirty = true;
            }
        }
        if self.dirty {
            self.dirty = false;
            self.rebuild(world, assets);
        }
        action
    }

    fn rebuild(&self, world: &mut World, assets: &Assets) {
        clear(world);
        for element in layout(self.tab, &self.view, self.hover) {
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

    fn view() -> BoardView {
        let note = |title: &str, done, active| NoteView {
            title: title.to_owned(),
            detail: "Defeat 100 enemies".to_owned(),
            progress: if done { 1 } else { 0 },
            goal: 1,
            talents: 2,
            done,
            active,
        };
        BoardView {
            missions: vec![
                note("Crypt Cleaner", false, false),
                note("Gold Digger", true, false),
                note("Deep Delver", false, false),
            ],
            challenges: vec![note("Horde", false, true), note("Bane", true, false)],
        }
    }

    #[test]
    fn each_tab_pins_its_own_papers() {
        let tasks = labels(&layout(Tab::Tasks, &view(), None));
        let challenges = labels(&layout(Tab::Challenges, &view(), None));

        assert!(tasks.contains(&"Crypt Cleaner".to_string()));
        assert!(!tasks.contains(&"Horde".to_string()));
        assert!(challenges.contains(&"Horde".to_string()));
        assert_eq!(
            papers(&layout(Tab::Tasks, &view(), None)).len(),
            view().missions.len()
        );
    }

    #[test]
    fn finished_papers_carry_the_stamp_and_hovered_ones_are_lifted() {
        let calm = papers(&layout(Tab::Tasks, &view(), None));
        let lifted = papers(&layout(Tab::Tasks, &view(), Some(Target::Note(0))));

        assert_eq!(calm.iter().filter(|row| **row >= 2).count(), 1);
        assert_eq!(calm[0], 0);
        assert_eq!(lifted[0], 1);
    }

    #[test]
    fn every_paper_shows_its_reward_as_a_talent_icon() {
        let icons = |tab| {
            layout(tab, &view(), None)
                .iter()
                .filter(|element| matches!(element, Element::Image { texture, .. } if *texture == tex::TALENT_ICON))
                .count()
        };

        assert_eq!(icons(Tab::Tasks), view().missions.len());
        assert_eq!(icons(Tab::Challenges), view().challenges.len());
    }

    #[test]
    fn the_pointer_finds_tabs_and_papers() {
        let tab = tab_top_left(Tab::Challenges) + Vec2::splat(5.0);
        let paper = note_top_left(2) + Vec2::splat(20.0);

        assert_eq!(
            target_at(Tab::Tasks, &view(), tab),
            Some(Target::Tab(Tab::Challenges))
        );
        assert_eq!(target_at(Tab::Tasks, &view(), paper), Some(Target::Note(2)));
        assert_eq!(target_at(Tab::Tasks, &view(), Vec2::ZERO), None);
    }

    #[test]
    fn every_paper_fits_on_the_board() {
        let board = panel::BOARD_TOP_LEFT + panel::BOARD_SIZE;
        for index in 0..view().missions.len() {
            let far = note_top_left(index) + NOTE_SIZE;

            assert!(
                far.x < board.x && far.y < board.y - 12.0,
                "{index}: {far:?}"
            );
        }
        assert!(note_top_left(0).y > TAB_TOP + TAB_SIZE.y);
    }

    #[test]
    fn the_active_challenge_is_marked_and_the_others_are_not() {
        let marks = labels(&layout(Tab::Challenges, &view(), None))
            .iter()
            .filter(|label| *label == ACTIVE_MARK)
            .count();
        assert_eq!(marks, 1);
        assert!(!labels(&layout(Tab::Tasks, &view(), None)).contains(&ACTIVE_MARK.to_owned()));
    }
}
