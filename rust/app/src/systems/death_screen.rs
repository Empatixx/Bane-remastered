use glam::{Vec2, Vec4};
use hecs::World;

use crate::components::{Color, DeathPart, Hidden, Position, Size, Text};
use crate::systems::Summary;

pub(crate) const ROWS: usize = 4;
const SKULL_TIME: f32 = 2.5;
const SKULL_START: f32 = 200.0;
const SKULL_END: f32 = 50.0;
const SKULL_LEFT: f32 = 429.0;
const ROW_TIME: f32 = 0.5;
const ROW_STEP: f32 = 50.0;
const ICON_START: f32 = 250.0;
const ICON_END: f32 = 200.0;
const TEXT_END: f32 = 225.0;
const ICON_LEFT: f32 = 250.0;
const LABEL_LEFT: f32 = 300.0;
const REWARD_LEFT: f32 = 650.0;
const DIM_ALPHA: f32 = 0.7;
const DIM_FADE_IN: f32 = 1.0;
const FADE_OUT: f32 = 0.8;
const BLINK: f32 = 0.2;
/// Keys pressed this soon after dying (a dodge, say) do not skip the screen.
const SKIP_GUARD: f32 = 0.5;

#[derive(Debug, Clone)]
pub(crate) struct DeathScreen {
    pub(crate) elapsed: f32,
    pub(crate) summary: Summary,
    pub(crate) continued: Option<f32>,
    pub(crate) note: Option<String>,
}

impl DeathScreen {
    pub(crate) fn new(summary: Summary) -> Self {
        Self {
            elapsed: 0.0,
            summary,
            continued: None,
            note: None,
        }
    }

    pub(crate) fn with_note(mut self, note: Option<String>) -> Self {
        self.note = note;
        self
    }

    fn row_start(index: usize) -> f32 {
        SKULL_TIME + ROW_TIME * index as f32
    }

    fn reward_start(index: usize) -> f32 {
        SKULL_TIME + ROW_TIME * ROWS as f32 + ROW_TIME * index as f32
    }

    fn shown_in_full() -> f32 {
        Self::reward_start(ROWS - 1) + ROW_TIME
    }

    pub(crate) fn ready(&self) -> bool {
        self.elapsed > Self::shown_in_full()
    }

    pub(crate) fn finished(&self) -> bool {
        self.continued.is_some_and(|time| time >= FADE_OUT)
    }

    pub(crate) fn advance(&mut self, dt: f32, any_key: bool) {
        self.elapsed += dt;
        if let Some(time) = &mut self.continued {
            *time += dt;
        } else if any_key && self.ready() {
            self.continued = Some(0.0);
        } else if any_key && self.elapsed > SKIP_GUARD {
            // The first key skips the reveal, the next one continues.
            self.elapsed = Self::shown_in_full() + f32::EPSILON * 64.0;
        }
    }

    pub(crate) fn skull_top(&self) -> f32 {
        let progress = self.elapsed.min(SKULL_TIME) / SKULL_TIME;
        SKULL_START - (SKULL_START - SKULL_END) * progress
    }

    pub(crate) fn title_visible(&self) -> bool {
        self.elapsed > SKULL_TIME
    }

    pub(crate) fn row_offset(&self, index: usize) -> Option<f32> {
        let start = Self::row_start(index);
        (self.elapsed > start).then(|| (self.elapsed - start).min(ROW_TIME) / ROW_TIME)
    }

    pub(crate) fn icon_top(&self, index: usize) -> Option<f32> {
        self.row_offset(index)
            .map(|offset| ICON_START + ROW_STEP * index as f32 - (ICON_START - ICON_END) * offset)
    }

    pub(crate) fn text_baseline(&self, index: usize) -> Option<f32> {
        self.row_offset(index).map(|offset| {
            TEXT_END + ROW_STEP * index as f32 + (ICON_START - ICON_END) * (1.0 - offset)
        })
    }

    pub(crate) fn label(&self, index: usize) -> String {
        match index {
            0 => format!("Accuracy: {}%", self.summary.accuracy_percent),
            1 => format!("Kills: {}", self.summary.kills),
            2 => format!("Loot: {}", self.summary.items),
            _ => format!("Floor: {}", self.summary.floor),
        }
    }

    pub(crate) fn reward(&self, index: usize) -> Option<u32> {
        let start = Self::reward_start(index);
        (self.elapsed > start).then(|| {
            let progress = (self.elapsed - start).min(ROW_TIME) / ROW_TIME;
            (self.summary.rewards[index] as f32 * progress) as u32
        })
    }

    pub(crate) fn prompt_visible(&self) -> bool {
        self.continued.is_none()
            && self.ready()
            && ((self.elapsed / BLINK) as usize).is_multiple_of(2)
    }

    pub(crate) fn dim_alpha(&self) -> f32 {
        let fade_in = (self.elapsed / DIM_FADE_IN).min(1.0) * DIM_ALPHA;
        match self.continued {
            Some(time) => fade_in + (1.0 - fade_in) * (time / FADE_OUT).min(1.0),
            None => fade_in,
        }
    }
}

pub(crate) fn update_death_ui(world: &mut World, screen: &DeathScreen) {
    let hidden_by_continue = screen.continued.is_some();
    let mut hide = Vec::new();
    let mut show = Vec::new();
    for (entity, part, position, size, color, text) in world.query_mut::<(
        hecs::Entity,
        &DeathPart,
        &mut Position,
        &mut Size,
        &mut Color,
        Option<&mut Text>,
    )>() {
        let visible = match *part {
            DeathPart::Dim => {
                color.0 = Vec4::new(0.0, 0.0, 0.0, screen.dim_alpha());
                true
            }
            DeathPart::Skull => {
                position.0 = Vec2::new(SKULL_LEFT, screen.skull_top()) + size.0 / 2.0;
                !hidden_by_continue
            }
            DeathPart::Title => !hidden_by_continue && screen.title_visible(),
            DeathPart::Icon(index) => match screen.icon_top(index) {
                Some(top) if !hidden_by_continue => {
                    position.0 = Vec2::new(ICON_LEFT, top) + size.0 / 2.0;
                    true
                }
                _ => false,
            },
            DeathPart::Label(index) => match screen.text_baseline(index) {
                Some(y) if !hidden_by_continue => {
                    position.0 = Vec2::new(LABEL_LEFT, y);
                    if let Some(text) = text {
                        text.content = screen.label(index);
                    }
                    true
                }
                _ => false,
            },
            DeathPart::Reward(index) => match screen.reward(index) {
                Some(coins) if !hidden_by_continue => {
                    position.0 = Vec2::new(REWARD_LEFT, TEXT_END + ROW_STEP * index as f32);
                    if let Some(text) = text {
                        text.content = format!("+{coins}");
                    }
                    true
                }
                _ => false,
            },
            DeathPart::Prompt => screen.prompt_visible(),
            DeathPart::Note => match &screen.note {
                Some(note) if screen.ready() && screen.continued.is_none() => {
                    if let Some(text) = text {
                        text.content.clone_from(note);
                    }
                    true
                }
                _ => false,
            },
        };
        if visible {
            show.push(entity);
        } else {
            hide.push(entity);
        }
    }
    for entity in hide {
        let _ = world.insert_one(entity, Hidden);
    }
    for entity in show {
        let _ = world.remove_one::<Hidden>(entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::RunStats;
    use crate::systems::summarize;

    fn screen() -> DeathScreen {
        let stats = RunStats {
            fired: 20,
            hits: 10,
            kills: 8,
            items: 6,
            ..RunStats::default()
        };
        DeathScreen::new(summarize(stats, 3))
    }

    fn note_is_shown(screen: &DeathScreen) -> bool {
        let mut world = World::new();
        let entity = world.spawn((
            DeathPart::Note,
            Position(Vec2::ZERO),
            Size(Vec2::ONE),
            Color(Vec4::ONE),
            Text::new(String::new(), Vec4::ONE, crate::render::FontSize::Size22),
            Hidden,
        ));
        update_death_ui(&mut world, screen);
        let hidden = world.get::<&Hidden>(entity).is_ok();
        !hidden
    }

    #[test]
    fn the_note_stays_steady_while_the_prompt_blinks_and_only_when_there_is_one() {
        let mut with = screen().with_note(Some("done".to_owned()));
        let mut without = screen();
        with.advance(7.25, false);
        without.advance(7.25, false);
        assert!(with.prompt_visible());
        assert!(note_is_shown(&with));
        assert!(!note_is_shown(&without));

        with.advance(BLINK, false);
        assert!(!with.prompt_visible());
        assert!(note_is_shown(&with));
    }

    #[test]
    fn the_skull_rises_for_two_and_a_half_seconds_and_then_the_title_appears() {
        let mut screen = screen();
        assert_eq!(screen.skull_top(), 200.0);
        assert!(!screen.title_visible());
        screen.advance(1.25, false);
        assert!((screen.skull_top() - 125.0).abs() < 0.01);
        screen.advance(1.3, false);
        assert_eq!(screen.skull_top(), 50.0);
        assert!(screen.title_visible());
    }

    #[test]
    fn the_rows_slide_in_one_after_another() {
        let mut screen = screen();
        screen.advance(2.4, false);
        assert!(screen.icon_top(0).is_none());
        screen.advance(0.35, false);
        let first = screen.icon_top(0).expect("the first row started");
        assert!(first < 250.0 && first > 200.0, "{first}");
        assert!(screen.icon_top(1).is_none());
        screen.advance(3.0, false);
        assert_eq!(screen.icon_top(0), Some(200.0));
        assert_eq!(screen.icon_top(3), Some(350.0));
        assert_eq!(screen.text_baseline(3), Some(375.0));
    }

    #[test]
    fn labels_and_rewards_show_the_run() {
        let screen = screen();
        assert_eq!(screen.label(0), "Accuracy: 50%");
        assert_eq!(screen.label(1), "Kills: 8");
        assert_eq!(screen.label(2), "Loot: 6");
        assert_eq!(screen.label(3), "Floor: 3");
    }

    #[test]
    fn rewards_count_up_after_the_rows_are_in() {
        let mut screen = screen();
        screen.advance(4.4, false);
        assert_eq!(screen.reward(0), None);
        screen.advance(0.35, false);
        let partial = screen.reward(0).expect("started");
        assert!(partial <= screen.summary.rewards[0]);
        screen.advance(5.0, false);
        assert_eq!(screen.reward(3), Some(screen.summary.rewards[3]));
    }

    #[test]
    fn you_cannot_continue_before_everything_is_shown_and_then_any_key_does() {
        let mut screen = screen();
        screen.advance(3.0, false);
        assert!(screen.continued.is_none());
        screen.advance(4.0, false);
        assert!(screen.ready());
        screen.advance(0.016, true);
        assert!(screen.continued.is_some());
        assert!(!screen.finished());
        screen.advance(1.0, false);
        assert!(screen.finished());
    }

    #[test]
    fn the_first_key_skips_the_reveal_and_the_second_continues() {
        let mut screen = screen();
        screen.advance(1.0, true);
        assert!(screen.ready());
        assert!(screen.continued.is_none(), "skipping does not leave yet");
        for index in 0..ROWS {
            assert_eq!(screen.reward(index), Some(screen.summary.rewards[index]));
        }
        screen.advance(0.016, true);
        assert!(screen.continued.is_some());
    }

    #[test]
    fn a_key_pressed_right_as_you_die_does_not_skip() {
        let mut screen = screen();
        screen.advance(0.1, true);
        assert!(!screen.ready());
        assert!(!screen.title_visible());
    }

    #[test]
    fn the_prompt_blinks_and_the_dim_layer_fades_in_then_out() {
        let mut screen = screen();
        screen.advance(6.6, false);
        let states: Vec<bool> = (0..6)
            .map(|_| {
                screen.advance(0.2, false);
                screen.prompt_visible()
            })
            .collect();
        assert!(states.contains(&true) && states.contains(&false));
        assert!((screen.dim_alpha() - 0.7).abs() < 1e-4);
        screen.advance(0.0, true);
        screen.advance(0.4, false);
        assert!(screen.dim_alpha() > 0.7);
        screen.advance(1.0, false);
        assert_eq!(screen.dim_alpha(), 1.0);
    }
}
