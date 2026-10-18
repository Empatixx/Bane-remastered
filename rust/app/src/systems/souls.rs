//! What the wound gives back as a boss falls: the one it was, a ghost standing where it died,
//! who says its last words over its head and then fades into sparks of light. And on the way
//! down, a tear in the floor beside Elian that opens, whispers to him, and closes again.

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Textures, tex};
use crate::components::{Animation, Clip, Color, Layer, Light, Position, Size, Sprite, YSort};
use crate::lore::{Figure, Identity};
use crate::prefabs;
use crate::tilemap::TILE_SIZE;

const PRIEST_SIZE: Vec2 = Vec2::new(40.0, 76.0);
const GHOST: Vec4 = Vec4::new(0.72, 0.62, 1.0, 0.74);
const LET_GO: Vec4 = Vec4::new(1.0, 0.92, 0.7, 1.0);
const STANDING: Clip = Clip { row: 0, frames: 4, frame_time: 0.3, looping: true, flipped: false };
const PRAYING: Clip = Clip { row: 2, frames: 2, frame_time: 0.6, looping: true, flipped: false };
/// A ghost's life: coming out of the light, speaking, then going.
const APPEARS: f32 = 1.0;
const SPEAKS: f32 = 1.2;
const SPEAKS_FOR: f32 = 5.5;
const GOES: f32 = 7.2;
const GONE: f32 = 9.0;
/// How long a whispering tear stays open.
pub(crate) const WHISPER_SECONDS: f32 = 5.0;
/// A whispering tear: how long it takes to open and to close.
const TEAR_OPENS: f32 = 0.5;
const TEAR_CLOSES: f32 = 0.7;
const RIFT_FRAMES: usize = 4;
const SPARK_LAYER: i32 = 6;
const SPARK: Vec4 = Vec4::new(0.92, 0.55, 1.0, 1.0);

/// A fallen boss's ghost: how long it has been up, its words, and whether it has said them.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Ghost {
    age: f32,
    words: &'static str,
    said: bool,
    seed: u32,
}

/// A tear the wound whispers through: how long it has been open, of how long, and its words.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Tear {
    age: f32,
    seconds: f32,
    words: &'static str,
    said: bool,
    seed: u32,
}

/// A spark drifting up and fading: its drift, how long it has left of how long.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Spark {
    drift: Vec2,
    left: f32,
    life: f32,
}

fn roll(seed: &mut u32) -> f32 {
    *seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    (*seed >> 8) as f32 / (1u32 << 24) as f32
}

fn spark(world: &mut World, at: Vec2, colour: Vec4, rise: f32, seed: &mut u32) {
    let drift = Vec2::new((roll(seed) - 0.5) * 30.0, -rise - roll(seed) * rise);
    let life = 0.8 + roll(seed) * 1.0;
    world.spawn((Position(at), Size(Vec2::splat(4.0)), Color(colour), Layer(SPARK_LAYER), Spark { drift, left: life, life }));
}

fn smooth(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Raises the ghost of who a fallen boss was where it fell, if it was someone with a shape
/// to come back in and words to say aloud. Returns whether it did; if not, its last words are
/// for the caption.
pub(crate) fn raise_ghost(world: &mut World, identity: Identity, at: Vec2, textures: &Textures) -> bool {
    let Some(figure) = identity.figure else { return false };
    let sheet = match figure {
        Figure::Brother => tex::PRIEST,
        Figure::Pilgrim => tex::PILGRIM,
    };
    world.spawn((
        Position(at),
        Size(PRIEST_SIZE),
        Color(Vec4::new(GHOST.x, GHOST.y, GHOST.z, 0.0)),
        Sprite::cell(textures.get(sheet), 0, STANDING.row, 4, 4),
        Animation::new(4, 4, STANDING),
        Layer(1),
        YSort::bottom(PRIEST_SIZE),
        Light::rgb(Vec2::new(0.0, -10.0), 180.0, 140.0, 255.0, 110.0, 0.0),
        Ghost { age: 0.0, words: identity.last_words, said: false, seed: at.x.to_bits() ^ at.y.to_bits() },
    ));
    true
}

/// Opens a tear in the floor at `at` that whispers `words` and closes again after `seconds`.
pub(crate) fn open_tear(world: &mut World, at: Vec2, words: &'static str, seconds: f32, textures: &Textures) {
    // On its own, torn all round: the rift sheet's row for no neighbours.
    let clip = Clip { row: 0, frames: RIFT_FRAMES, frame_time: 0.14, looping: true, flipped: false };
    world.spawn((
        Position(at),
        Size(Vec2::splat(TILE_SIZE)),
        Color(Vec4::new(1.0, 1.0, 1.0, 0.0)),
        Sprite::cell(textures.get(tex::RIFT), 0, 0, RIFT_FRAMES, 16),
        Animation::new(RIFT_FRAMES, 16, clip),
        Layer(-1),
        Light::rgb(Vec2::ZERO, 150.0, 60.0, 230.0, 160.0, 0.0),
        Tear { age: 0.0, seconds, words, said: false, seed: at.x.to_bits().rotate_left(7) ^ at.y.to_bits() },
    ));
}

/// The ghosts coming, speaking and going; the tears opening, whispering and closing; the
/// sparks off both drifting up and fading.
pub(crate) fn tick_souls(world: &mut World, dt: f32) {
    let mut sparks: Vec<(Vec2, Vec4, f32)> = Vec::new();
    let mut speech: Option<(Vec2, &'static str, f32)> = None;
    let mut gone = Vec::new();
    for (entity, ghost, position, color, animation, light) in
        world.query_mut::<(Entity, &mut Ghost, &Position, &mut Color, &mut Animation, &mut Light)>()
    {
        ghost.age += dt;
        let age = ghost.age;
        let coming = smooth(age / APPEARS);
        let going = smooth((age - GOES) / (GONE - GOES));
        let tint = GHOST.lerp(LET_GO, smooth((age - GOES) / 0.6));
        color.0 = Vec4::new(tint.x, tint.y, tint.z, GHOST.w * coming * (1.0 - going));
        light.intensity = color.0.w * 0.8;
        if !ghost.said && age >= SPEAKS {
            ghost.said = true;
            let head = position.0 - Vec2::new(0.0, PRIEST_SIZE.y / 2.0);
            speech = Some((prefabs::speech_anchor(head), ghost.words, SPEAKS_FOR));
        }
        // It lifts its arms as it goes.
        if age >= GOES {
            animation.switch_to(PRAYING);
        }
        let bursting = (age < APPEARS && roll(&mut ghost.seed) < 0.5) || (age > GOES && age < GONE);
        if bursting {
            let jitter = Vec2::new((roll(&mut ghost.seed) - 0.5) * 34.0, (roll(&mut ghost.seed) - 0.5) * 70.0);
            let colour = if age > GOES { LET_GO } else { SPARK };
            sparks.push((position.0 + jitter, colour, if age > GOES { 60.0 } else { 30.0 }));
        }
        if age >= GONE {
            gone.push(entity);
        }
    }
    for (entity, tear, position, color, light) in world.query_mut::<(Entity, &mut Tear, &Position, &mut Color, &mut Light)>() {
        tear.age += dt;
        let open = smooth(tear.age / TEAR_OPENS).min(smooth((tear.seconds - tear.age) / TEAR_CLOSES));
        color.0.w = open;
        light.intensity = 0.9 * open;
        if !tear.said && tear.age >= TEAR_OPENS {
            tear.said = true;
            let over = position.0 - Vec2::new(0.0, TILE_SIZE * 0.6);
            speech = Some((prefabs::speech_anchor(over), tear.words, tear.seconds - TEAR_OPENS));
        }
        if open > 0.2 && roll(&mut tear.seed) < 0.35 {
            let jitter = Vec2::new(roll(&mut tear.seed) - 0.5, roll(&mut tear.seed) - 0.5) * TILE_SIZE * 0.7;
            sparks.push((position.0 + jitter, SPARK, 40.0));
        }
        if tear.age >= tear.seconds {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
    }
    let mut seed = (dt.to_bits()).wrapping_add(sparks.len() as u32);
    for (at, colour, rise) in sparks {
        seed = seed.wrapping_add(at.x.to_bits());
        spark(world, at, colour, rise, &mut seed);
    }
    if let Some((anchor, words, seconds)) = speech {
        super::say_at(world, anchor, words, seconds);
    }
    let mut faded = Vec::new();
    for (entity, spark, position, color) in world.query_mut::<(Entity, &mut Spark, &mut Position, &mut Color)>() {
        spark.left -= dt;
        if spark.left <= 0.0 {
            faded.push(entity);
            continue;
        }
        position.0 += spark.drift * dt;
        color.0.w = (spark.left / spark.life).min(1.0);
    }
    for entity in faded {
        let _ = world.despawn(entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::SpeechBubble;
    use crate::enemies::EnemyKind;

    #[test]
    fn a_fallen_brother_comes_back_says_his_last_words_and_goes() {
        let assets = crate::assets::Assets::new().expect("assets");
        let mut world = World::new();
        prefabs::minimal_speech(&mut world, Vec2::ZERO);
        let teodor = crate::lore::identity(EnemyKind::Spider).expect("Teodor");
        assert!(raise_ghost(&mut world, teodor, Vec2::new(300.0, 300.0), &assets.textures));
        let said = |world: &World| world.query::<&SpeechBubble>().iter().any(|bubble| bubble.message == teodor.last_words);
        tick_souls(&mut world, 0.5);
        assert!(!said(&world));
        tick_souls(&mut world, 1.0);
        assert!(said(&world), "he speaks once he has come out of the light");
        for _ in 0..20 {
            tick_souls(&mut world, 0.5);
        }
        assert_eq!(world.query::<&Ghost>().iter().count(), 0, "and then he is gone");
    }

    #[test]
    fn a_tear_opens_whispers_and_closes() {
        let assets = crate::assets::Assets::new().expect("assets");
        let mut world = World::new();
        prefabs::minimal_speech(&mut world, Vec2::ZERO);
        open_tear(&mut world, Vec2::new(100.0, 100.0), "Deeper.", 4.0, &assets.textures);
        tick_souls(&mut world, 0.6);
        assert!(world.query::<&SpeechBubble>().iter().any(|bubble| bubble.message == "Deeper."));
        for _ in 0..10 {
            tick_souls(&mut world, 0.5);
        }
        assert_eq!(world.query::<&Tear>().iter().count(), 0);
    }
}
