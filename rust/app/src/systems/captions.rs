//! Words over the screen for a moment, fading in and out: a boss's name and who it was as the
//! fight starts, and what it says as it falls when no ghost comes back to say it. The game
//! tells its story in these few lines and in the scenes between runs, never in long text.

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::components::{Color, Layer, Position, ScreenSpace, Text};
use crate::render::FontSize;
use crate::viewport::TARGET;

const FADE: f32 = 0.6;
const INK: Vec4 = Vec4::new(0.93, 0.88, 0.78, 1.0);
const BOSS_NAME: Vec4 = Vec4::new(0.86, 0.3, 0.3, 1.0);
const LAST_WORDS: Vec4 = Vec4::new(0.82, 0.7, 0.95, 1.0);
const BOSS_TITLE_SECONDS: f32 = 4.0;
const LAST_WORDS_SECONDS: f32 = 6.0;

/// A line on the screen, and how long it has been up of how long it stays.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Caption {
    shown: f32,
    seconds: f32,
}

/// Lines over the screen for `seconds`: each its text, size, colour and height.
pub(crate) fn caption(world: &mut World, lines: &[(String, FontSize, Vec4, f32)], seconds: f32) {
    for (text, size, color, y) in lines {
        world.spawn((
            Caption { shown: 0.0, seconds },
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

/// Fades each line in, keeps it up a while, fades it out.
pub(crate) fn tick_captions(world: &mut World, dt: f32) {
    let mut done = Vec::new();
    for (entity, caption, text) in world.query_mut::<(Entity, &mut Caption, &mut Text)>() {
        caption.shown += dt;
        let t = caption.shown;
        text.color.w = (t / FADE).min(1.0).min(((caption.seconds - t) / FADE).max(0.0));
        if t >= caption.seconds {
            done.push(entity);
        }
    }
    for entity in done {
        let _ = world.despawn(entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_words_fade_in_and_are_gone_after_their_time() {
        let mut world = World::new();
        last_words(&mut world, crate::lore::identity(crate::enemies::EnemyKind::Argus).expect("the Eye"));
        let alpha = |world: &World| world.query::<&Text>().iter().next().map(|text| text.color.w);
        tick_captions(&mut world, 0.1);
        assert!(alpha(&world).is_some_and(|alpha| alpha > 0.0 && alpha < 1.0));
        tick_captions(&mut world, LAST_WORDS_SECONDS);
        assert_eq!(alpha(&world), None);
    }
}
