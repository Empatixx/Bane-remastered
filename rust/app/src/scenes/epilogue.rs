//! The ending at the bottom of the wound: the brothers waiting there, what the wound offers,
//! and what Elian remembers at last; then the choice. Let them go and close the wound, or stay
//! with them. Either way the lobby comes after, changed.

use glam::{Vec2, Vec4};
use hecs::{Entity, World};
use winit::keyboard::KeyCode;

use crate::assets::Assets;
use crate::components::{Color, Layer, Position, ScreenSpace, Size, Text};
use crate::input::Input;
use crate::profile::SharedProfile;
use crate::progress::Ending;
use crate::render::FontSize;
use crate::settings::Settings;
use crate::viewport::TARGET;

use super::{Cursor, Scene, SceneId, Transition};

/// What waits at the bottom, one line at a time.
pub(crate) const LINES: [&str; 4] = [
    "At the bottom of the wound, the brothers are waiting.",
    "Teodor. Cassian. Oswin. Changed, but there.",
    "\"Stay,\" it says. \"And you will never be alone again.\"",
    "Elian remembers that night. He fell asleep. It was him.",
];
const CHOOSING: &str = "Close the wound and let them go? Or stay with them?";
/// What follows each choice, before the lobby.
const CLOSED: &str = "The wound closes. For the first time since that night, Elian sleeps.";
const LEFT_OPEN: &str = "Elian stays. Above, the Scientist writes: Subject 41, lost.";
const CHOICES: [&str; 2] = ["Let them go", "Stay"];

const LINE_SECONDS: f32 = 5.0;
const FADE: f32 = 0.8;
const AFTER_SECONDS: f32 = 4.5;
const INK: Vec4 = Vec4::new(0.9, 0.86, 0.8, 1.0);
const CHOICE_INK: Vec4 = Vec4::new(0.62, 0.58, 0.66, 1.0);
const CHOSEN_INK: Vec4 = Vec4::new(0.84, 0.62, 1.0, 1.0);
const CHOICE_Y: f32 = 330.0;
const CHOICE_SPREAD: f32 = 170.0;
const CHOICE_SIZE: Vec2 = Vec2::new(300.0, 44.0);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Stage {
    /// Telling line `n`, for how long.
    Telling(usize, f32),
    /// Waiting for the choice, with the one under the pointer or the keys.
    Choosing(usize),
    /// The choice made, its words showing for how long.
    After(Ending, f32),
}

pub(crate) struct EpilogueScene {
    world: World,
    profile: Option<SharedProfile>,
    stage: Stage,
    words: Option<Entity>,
    choices: Vec<Entity>,
    /// How long the choice has waited.
    waited: f32,
}

impl Default for EpilogueScene {
    fn default() -> Self {
        Self {
            world: World::new(),
            profile: None,
            stage: Stage::Telling(0, 0.0),
            words: None,
            choices: Vec::new(),
            waited: 0.0,
        }
    }
}

impl EpilogueScene {
    pub(crate) fn attach_profile(&mut self, profile: SharedProfile) {
        self.profile = Some(profile);
    }

    fn say(&mut self, line: &str, color: Vec4) {
        if let Some(old) = self.words.take() {
            let _ = self.world.despawn(old);
        }
        self.words = Some(self.world.spawn((
            Position(Vec2::new(TARGET.x / 2.0, TARGET.y / 2.0 - 40.0)),
            Text::centered(line.to_owned(), color, FontSize::Size27),
            Color(color),
            Layer(2),
            ScreenSpace,
        )));
    }

    fn fade_words(&mut self, alpha: f32) {
        if let Some(words) = self.words {
            if let Ok(mut text) = self.world.get::<&mut Text>(words) {
                text.color.w = alpha;
            }
        }
    }

    fn offer(&mut self) {
        self.say(CHOOSING, INK);
        for (index, label) in CHOICES.into_iter().enumerate() {
            let entity = self.world.spawn((
                Position(choice_center(index)),
                Text::centered(label.to_owned(), CHOICE_INK, FontSize::Size27),
                Color(CHOICE_INK),
                Layer(2),
                ScreenSpace,
            ));
            self.choices.push(entity);
        }
        self.stage = Stage::Choosing(0);
        self.waited = 0.0;
    }

    fn choose(&mut self, ending: Ending) {
        for entity in self.choices.drain(..) {
            let _ = self.world.despawn(entity);
        }
        if let Some(profile) = &self.profile {
            profile.set_ending(ending);
        }
        let words = match ending {
            Ending::Closed => CLOSED,
            Ending::LeftOpen => LEFT_OPEN,
        };
        self.say(words, INK);
        self.stage = Stage::After(ending, 0.0);
    }
}

fn choice_center(index: usize) -> Vec2 {
    let side = if index == 0 { -1.0 } else { 1.0 };
    Vec2::new(TARGET.x / 2.0 + side * CHOICE_SPREAD, CHOICE_Y)
}

fn choice_at(point: Vec2) -> Option<usize> {
    (0..CHOICES.len()).find(|&index| {
        let offset = (point - choice_center(index) + Vec2::new(0.0, CHOICE_SIZE.y / 3.0)).abs();
        offset.x <= CHOICE_SIZE.x / 2.0 && offset.y <= CHOICE_SIZE.y / 2.0
    })
}

/// How visible a line is `time` into the `seconds` it shows.
fn fade(time: f32, seconds: f32) -> f32 {
    (time / FADE).min(1.0).min(((seconds - time) / FADE).max(0.0))
}

impl Scene for EpilogueScene {
    fn world(&self) -> &World {
        &self.world
    }

    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn cursor(&self) -> Cursor {
        if matches!(self.stage, Stage::Choosing(_)) { Cursor::Default } else { Cursor::Hidden }
    }

    fn enter(&mut self, _assets: &Assets) {
        self.world.clear();
        self.words = None;
        self.choices.clear();
        self.world.spawn((
            Position(TARGET / 2.0),
            Size(TARGET),
            Color(Vec4::new(0.0, 0.0, 0.0, 1.0)),
            Layer(0),
            ScreenSpace,
        ));
        self.stage = Stage::Telling(0, 0.0);
        self.say(LINES[0], INK);
        self.fade_words(0.0);
    }

    fn update(&mut self, input: &Input, dt: f32, _assets: &Assets, _settings: &mut Settings) -> Transition {
        let skip = [KeyCode::Space, KeyCode::Enter, KeyCode::Escape].into_iter().any(|key| input.just_pressed(key))
            || input.clicked();
        match self.stage {
            Stage::Telling(line, time) => {
                let time = time + dt;
                // A key moves on to the next line once this one is fully up.
                let done = time >= LINE_SECONDS || (skip && time > FADE);
                if done && line + 1 < LINES.len() {
                    self.say(LINES[line + 1], INK);
                    self.fade_words(0.0);
                    self.stage = Stage::Telling(line + 1, 0.0);
                } else if done {
                    self.offer();
                } else {
                    self.fade_words(fade(time, LINE_SECONDS));
                    self.stage = Stage::Telling(line, time);
                }
            }
            Stage::Choosing(current) => {
                // `BANE_CHOOSE=close` or `open` makes the choice itself, for recording the ending.
                self.waited += dt;
                if let Some(choice) = std::env::var("BANE_CHOOSE").ok().filter(|_| self.waited > 3.0) {
                    self.choose(if choice == "open" { Ending::LeftOpen } else { Ending::Closed });
                    return Transition::None;
                }
                let mut current = current;
                if let Some(hovered) = choice_at(input.mouse()) {
                    current = hovered;
                }
                if input.just_pressed(KeyCode::KeyA) || input.just_pressed(KeyCode::ArrowLeft) {
                    current = 0;
                }
                if input.just_pressed(KeyCode::KeyD) || input.just_pressed(KeyCode::ArrowRight) {
                    current = 1;
                }
                for (index, &entity) in self.choices.iter().enumerate() {
                    if let Ok(mut text) = self.world.get::<&mut Text>(entity) {
                        text.color = if index == current { CHOSEN_INK } else { CHOICE_INK };
                    }
                }
                let clicked = input.clicked() && choice_at(input.mouse()).is_some();
                if clicked || input.just_pressed(KeyCode::Enter) || input.just_pressed(KeyCode::Space) {
                    self.choose(if current == 0 { Ending::Closed } else { Ending::LeftOpen });
                } else {
                    self.stage = Stage::Choosing(current);
                }
            }
            Stage::After(ending, time) => {
                let time = time + dt;
                self.fade_words(fade(time, AFTER_SECONDS));
                if time >= AFTER_SECONDS {
                    return Transition::Switch(SceneId::Lobby);
                }
                self.stage = Stage::After(ending, time);
            }
        }
        Transition::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(scene: &mut EpilogueScene, input: &Input, seconds: f32) -> Transition {
        let assets = Assets::new().expect("assets");
        let mut settings = Settings::in_memory();
        let mut transition = Transition::None;
        let mut left = seconds;
        while left > 0.0 && transition == Transition::None {
            transition = scene.update(input, 0.1, &assets, &mut settings);
            left -= 0.1;
        }
        transition
    }

    #[test]
    fn the_truth_is_told_then_closing_the_wound_is_kept_and_leads_to_the_lobby() {
        let profile = SharedProfile::in_memory();
        let mut scene = EpilogueScene::default();
        scene.attach_profile(profile.clone());
        scene.enter(&Assets::new().expect("assets"));
        let idle = Input::default();

        run(&mut scene, &idle, LINE_SECONDS * LINES.len() as f32 + 0.5);
        assert_eq!(scene.stage, Stage::Choosing(0));

        let mut enter = Input::default();
        enter.press(KeyCode::Enter);
        run(&mut scene, &enter, 0.1);
        assert_eq!(profile.ending(), Some(Ending::Closed));

        assert_eq!(run(&mut scene, &idle, AFTER_SECONDS + 1.0), Transition::Switch(SceneId::Lobby));
    }

    #[test]
    fn leaving_it_open_is_chosen_with_the_right_key() {
        let profile = SharedProfile::in_memory();
        let mut scene = EpilogueScene::default();
        scene.attach_profile(profile.clone());
        scene.enter(&Assets::new().expect("assets"));
        run(&mut scene, &Input::default(), LINE_SECONDS * LINES.len() as f32 + 0.5);
        let mut right = Input::default();
        right.press(KeyCode::ArrowRight);
        run(&mut scene, &right, 0.1);
        let mut enter = Input::default();
        enter.press(KeyCode::Enter);
        run(&mut scene, &enter, 0.1);
        assert_eq!(profile.ending(), Some(Ending::LeftOpen));
    }
}
