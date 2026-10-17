//! The order's pages: Teodor's chronicle, Cassian's gate ledger, Oswin's bell log, Aldric's
//! oldest record, gone down into the wound with everything else (see `lore::PAGES`). One may lie on a floor, a book on the ground under a faint light;
//! reading it (E) keeps it in the profile and shows its words over the screen for a while.

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Color, InRange, Interactable, Interaction, Layer, Light, Position, Prompt, ScreenSpace, Size,
    Text,
};
use crate::prefabs;
use crate::props::{Look, Prop};
use crate::render::FontSize;
use crate::viewport::TARGET;

pub(crate) use crate::lore::PAGES;
/// How likely a floor is to hold a page while some are still unfound, in percent.
pub(crate) const PAGE_CHANCE: usize = 60;

const GLOW: (f32, f32, f32) = (255.0, 214.0, 150.0);
const READ_SECONDS: f32 = 9.0;
const READ_FADE: f32 = 0.6;
const PAPER: Vec4 = Vec4::new(0.08, 0.06, 0.07, 0.86);
const INK: Vec4 = Vec4::new(0.93, 0.88, 0.78, 1.0);
const HEADING: Vec4 = Vec4::new(0.85, 0.66, 0.36, 1.0);
const BOSS_NAME: Vec4 = Vec4::new(0.86, 0.3, 0.3, 1.0);
const LAST_WORDS: Vec4 = Vec4::new(0.82, 0.7, 0.95, 1.0);
const BOSS_TITLE_SECONDS: f32 = 4.0;
const LAST_WORDS_SECONDS: f32 = 6.0;
const LINE_CHARS: usize = 54;
const LINE_STEP: f32 = 26.0;
const PANEL_TOP: f32 = 286.0;

/// A page lying on the floor, by its number in `PAGES`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ChroniclePage(pub(crate) u8);

/// A part of the page shown on the screen, and how long it has been up.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Reading {
    pub(crate) shown: f32,
    /// Its colour's own opacity, which the fade scales.
    base: f32,
    /// How long it stays up.
    seconds: f32,
}

/// Lays page `page` on the floor at `center`.
pub(crate) fn lay_page(world: &mut World, center: Vec2, page: u8, textures: &Textures) -> Entity {
    let size = Prop::LayingBook.spec().size();
    let entity = prefabs::prop(world, Prop::LayingBook, center - size / 2.0, Look::DEFAULT, textures);
    let _ = world.insert(
        entity,
        (
            ChroniclePage(page),
            Interactable { action: Interaction::ReadPage },
            Prompt { text: "Press E to read", gap: 30.0, color: crate::palette::ORANGE, size: FontSize::Size22 },
            Light::rgb(Vec2::ZERO, GLOW.0, GLOW.1, GLOW.2, 150.0, 0.9),
        ),
    );
    entity
}

/// Reads the page in reach: it leaves the floor and its words come up over the screen.
/// Returns the page's number, for the profile to keep.
pub(crate) fn read_page(world: &mut World, sounds: &mut Vec<AudioCommand>) -> Option<u8> {
    let (entity, page) = world
        .query::<(Entity, &ChroniclePage)>()
        .with::<&InRange>()
        .iter()
        .next()
        .map(|(entity, page)| (entity, page.0))?;
    let _ = world.despawn(entity);
    sounds.push(AudioCommand::Effect(SoundEffect::Pickup));
    show_page(world, page);
    Some(page)
}

pub(crate) fn show_page(world: &mut World, page: u8) {
    clear(world);
    let (author, words) = PAGES[usize::from(page)];
    let lines = wrap(words, LINE_CHARS);
    let height = 70.0 + LINE_STEP * lines.len() as f32;
    let reading = Reading { shown: 0.0, base: 1.0, seconds: READ_SECONDS };
    world.spawn((
        Reading { shown: 0.0, base: PAPER.w, seconds: READ_SECONDS },
        Position(Vec2::new(TARGET.x / 2.0, PANEL_TOP + height / 2.0)),
        Size(Vec2::new(TARGET.x - 200.0, height)),
        Color(PAPER),
        Layer(15),
        ScreenSpace,
    ));
    let heading = format!("{author}  ({} of {})", page + 1, PAGES.len());
    world.spawn((
        reading,
        Position(Vec2::new(TARGET.x / 2.0, PANEL_TOP + 34.0)),
        Text::centered(heading, HEADING, FontSize::Size22),
        Color(HEADING),
        Layer(16),
        ScreenSpace,
    ));
    for (index, line) in lines.into_iter().enumerate() {
        world.spawn((
            reading,
            Position(Vec2::new(TARGET.x / 2.0, PANEL_TOP + 70.0 + LINE_STEP * index as f32)),
            Text::centered(line, INK, FontSize::Size22),
            Color(INK),
            Layer(16),
            ScreenSpace,
        ));
    }
}

/// Words over the screen for `seconds`, fading in and out: a boss's name as it rises, its last
/// words as it falls. Each line is its text, size, colour and height.
pub(crate) fn caption(world: &mut World, lines: &[(String, FontSize, Vec4, f32)], seconds: f32) {
    for (text, size, color, y) in lines {
        world.spawn((
            Reading { shown: 0.0, base: 1.0, seconds },
            Position(Vec2::new(TARGET.x / 2.0, *y)),
            Text::centered(text.clone(), *color, *size),
            Color(*color),
            Layer(16),
            ScreenSpace,
        ));
    }
}

/// A boss's name and who it was, high over the fight.
pub(crate) fn name_boss(world: &mut World, identity: crate::lore::Identity) {
    caption(
        world,
        &[
            (identity.name.to_uppercase(), FontSize::Size32, BOSS_NAME, 118.0),
            (identity.epithet.to_owned(), FontSize::Size22, INK, 152.0),
        ],
        BOSS_TITLE_SECONDS,
    );
}

/// What a boss says as it falls, low on the screen.
pub(crate) fn last_words(world: &mut World, identity: crate::lore::Identity) {
    caption(world, &[(identity.last_words.to_owned(), FontSize::Size22, LAST_WORDS, 420.0)], LAST_WORDS_SECONDS);
}

fn clear(world: &mut World) {
    let shown: Vec<Entity> = world.query::<Entity>().with::<&Reading>().iter().collect();
    for entity in shown {
        let _ = world.despawn(entity);
    }
}

/// Fades the page in, keeps it up a while, fades it out.
pub(crate) fn tick_reading(world: &mut World, dt: f32) {
    let mut done = Vec::new();
    for (entity, reading, color, text) in
        world.query_mut::<(Entity, &mut Reading, &mut Color, Option<&mut Text>)>()
    {
        reading.shown += dt;
        let t = reading.shown;
        let alpha = (t / READ_FADE).min(1.0).min(((reading.seconds - t) / READ_FADE).max(0.0));
        color.0.w = reading.base * alpha;
        if let Some(text) = text {
            text.color.w = alpha;
        }
        if t >= reading.seconds {
            done.push(entity);
        }
    }
    for entity in done {
        let _ = world.despawn(entity);
    }
}

/// Breaks `text` into lines of at most `width` characters, at spaces.
fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split(' ') {
        if !line.is_empty() && line.len() + 1 + word.len() > width {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::Player;

    #[test]
    fn every_page_fits_the_panel() {
        for (_, page) in PAGES {
            let lines = wrap(page, LINE_CHARS);
            assert!(lines.len() <= 3, "{page}");
            assert!(lines.iter().all(|line| line.len() <= LINE_CHARS));
        }
    }

    #[test]
    fn reading_a_page_in_reach_takes_it_and_shows_it_for_a_while() {
        let mut world = World::new();
        world.spawn((Player, Position(Vec2::ZERO)));
        world.spawn((ChroniclePage(5), Position(Vec2::ZERO), InRange));
        let mut sounds = Vec::new();

        assert_eq!(read_page(&mut world, &mut sounds), Some(5));
        assert_eq!(world.query::<&ChroniclePage>().iter().count(), 0);
        let texts = |world: &World| world.query::<&Text>().with::<&Reading>().iter().count();
        assert!(texts(&world) >= 2);

        tick_reading(&mut world, READ_SECONDS + 0.1);
        assert_eq!(texts(&world), 0);
        assert_eq!(read_page(&mut world, &mut sounds), None);
    }
}
