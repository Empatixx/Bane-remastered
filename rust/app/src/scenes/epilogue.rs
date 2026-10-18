//! The ending at the bottom of the wound, played in the game's own engine like the opening: the
//! temple's stone again, deep under the temple, and a rift at its head. Elian walks in up the
//! carpet; the brothers come out of the rift's light and stand before it, waiting; the wound
//! speaks; and Elian kneels as he knelt that night, nods off the way he did, and remembers.
//!
//! Then the choice. Let them go: the brothers fade one by one into sparks of light, the rift
//! closes tile by tile and Elian sleeps where he kneels. Stay: he gets up and walks into the
//! rift between them. Either way the lobby comes after, changed.

use std::sync::Arc;

use glam::{Vec2, Vec4};
use hecs::{Entity, World};
use winit::keyboard::KeyCode;

use crate::assets::{Assets, tex};
use crate::audio::Soundtrack;
use crate::camera::Camera;
use crate::components::{Animation, Clip, Color, Layer, Light, Lit, Position, ScreenSpace, Size, Sprite, Text, YSort};
use crate::dungeon::{Carpet, Decor, DecorKind, Dungeon, DungeonRoom, Mount, RoomKind, Surfaces, Theme, Waterworks};
use crate::input::Input;
use crate::postfx::SceneFx;
use crate::prefabs;
use crate::profile::SharedProfile;
use crate::progress::Ending;
use crate::render::{FontSize, Occluders, TextureId};
use crate::settings::Settings;
use crate::systems;
use crate::tilemap::{TILE_SIZE, TileGrid};
use crate::viewport::TARGET;

use super::{Cursor, Scene, SceneId, Transition};

/// What is told at the bottom, a shot each: how long it runs and its line.
pub(crate) const LINES: [(f32, &str); 4] = [
    (7.0, "At the bottom of the wound, the brothers are waiting."),
    (7.0, "Teodor. Oswin. Cassian. The Faithful. Changed, but there."),
    (7.0, "\"Stay,\" it says. \"And you will never be alone again.\""),
    (8.0, "Elian remembers that night. He fell asleep. It was him."),
];
const CHOOSING: &str = "Close the wound and let them go? Or stay with them?";
/// What follows each choice, before the lobby.
const CLOSED: &str = "The wound closes. For the first time since that night, Elian sleeps.";
const LEFT_OPEN: &str = "Elian stays. Above, the Scientist writes: Subject 41, lost.";
const CHOICES: [&str; 2] = ["Let them go", "Stay"];

/// The hall at the bottom, inside its walls, in tiles, and the dark around it.
const HALL: (usize, usize) = (15, 9);
const MARGIN: usize = 2;
/// The column down the middle, where the carpet runs and the doorway opens in the south wall.
const AISLE: f32 = 7.5;
/// The rift's tiles at the hall's head, inside the walls.
const RIFT: [(usize, usize); 10] = [(4, 1), (5, 1), (6, 1), (7, 1), (8, 1), (9, 1), (10, 1), (6, 2), (7, 2), (8, 2)];
const RIFT_FRAMES: usize = 4;
const RIFT_GLOW: (f32, f32, f32) = (150.0, 60.0, 230.0);

/// Who waits before the rift: each, the column it stands in, and whether it is one of the
/// brothers (the priests' sheet) or the Faithful (the pilgrims'). The middle is left open.
const WAITING: [(f32, bool); 4] = [(4.4, true), (5.9, true), (9.1, true), (10.6, false)];
const WAITING_ROW: f32 = 4.1;
const PRIEST_SIZE: Vec2 = Vec2::new(40.0, 76.0);
const GHOST: Vec4 = Vec4::new(0.72, 0.62, 1.0, 0.74);
const LET_GO: Vec4 = Vec4::new(1.0, 0.92, 0.7, 1.0);
/// The priests' and pilgrims' rows: standing, and kneeling at prayer.
const STANDING: Clip = Clip { row: 0, frames: 4, frame_time: 0.3, looping: true, flipped: false };
const PRAYING: Clip = Clip { row: 2, frames: 2, frame_time: 0.6, looping: true, flipped: false };

/// Elian's sheets: the hero's own (6x4; row 3 walks away from the camera) and kneeling
/// (tools/make_elian.py, 4x3: kneeling, nodding off, asleep).
const HERO: (usize, usize) = (6, 4);
const WALK_AWAY: Clip = Clip { row: 3, frames: 6, frame_time: 0.1, looping: true, flipped: false };
const KNEELING: (usize, usize) = (4, 3);
const KNEEL: Clip = Clip { row: 0, frames: 4, frame_time: 0.3, looping: true, flipped: false };
const NOD: Clip = Clip { row: 1, frames: 4, frame_time: 0.45, looping: false, flipped: false };
const ASLEEP: Clip = Clip { row: 2, frames: 4, frame_time: 0.5, looping: true, flipped: false };
/// Where Elian comes in from and stops, in rows, and how fast he walks (tiles a second).
const ENTERS_AT: f32 = 10.6;
const STOPS_AT: f32 = 6.7;
const WALKS_FROM: f32 = 0.8;
const PACE: f32 = 1.3;

/// When the waiting come out of the rift's light, one after another, and how long each takes.
const APPEAR_FROM: f32 = 5.0;
const APPEAR_EVERY: f32 = 1.1;
const APPEAR_SECONDS: f32 = 1.2;
/// When, in the last shot, Elian kneels, nods off, and how long the memory of that night lasts.
const KNEELS: f32 = 0.6;
const NODS: f32 = 2.4;
const MEMORY: (f32, f32) = (3.2, 6.6);
const NIGHT_TINT: Vec4 = Vec4::new(0.02, 0.03, 0.12, 0.5);

/// Letting them go: when each fades into light, how long it takes, when the rift starts to
/// close and how long that takes, when Elian falls asleep, and when it is over.
const RELEASE_FROM: f32 = 0.8;
const RELEASE_EVERY: f32 = 1.2;
const RELEASE_SECONDS: f32 = 1.6;
const CLOSE_FROM: f32 = 4.2;
const CLOSE_SECONDS: f32 = 2.6;
const SLEEPS: f32 = 6.0;
const CLOSED_SECONDS: f32 = 10.5;
/// Staying: when Elian gets up and walks to the rift, and when it is over.
const RISES: f32 = 0.7;
const WALKS_IN: f32 = 1.5;
const WALK_IN_PACE: f32 = 0.9;
const OPEN_SECONDS: f32 = 10.0;
const SWALLOW: Vec4 = Vec4::new(0.16, 0.03, 0.24, 0.7);

const FADE_SECONDS: f32 = 1.4;
const LETTERS_PER_SECOND: f32 = 18.0;
const LINE_FROM: f32 = 0.6;
const BAR: f32 = 40.0;
const OVERLAY_LAYER: i32 = 20;
const INK: Vec4 = Vec4::new(0.93, 0.89, 0.82, 1.0);
const CHOICE_INK: Vec4 = Vec4::new(0.62, 0.58, 0.66, 1.0);
const CHOSEN_INK: Vec4 = Vec4::new(0.84, 0.62, 1.0, 1.0);
const CHOICE_Y: f32 = TARGET.y - BAR - 36.0;
const CHOICE_SPREAD: f32 = 170.0;
const CHOICE_SIZE: Vec2 = Vec2::new(300.0, 44.0);
const MOTE_LAYER: i32 = 6;
const MOTE_BRIGHT: Vec4 = Vec4::new(0.92, 0.55, 1.0, 1.0);
const MOTE_DEEP: Vec4 = Vec4::new(0.62, 0.22, 0.9, 1.0);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Stage {
    /// Telling, how far in.
    Telling(f32),
    /// Waiting for the choice, with the one under the pointer or the keys.
    Choosing(usize),
    /// The choice made, how far into what follows.
    After(Ending, f32),
}

/// Sparks drifting up, from the rift or from a brother letting go: each, its drift, how long it
/// has left of how long.
struct Mote {
    entity: Entity,
    drift: Vec2,
    left: f32,
    life: f32,
}

pub(crate) struct EpilogueScene {
    world: World,
    camera: Camera,
    occluders: Option<Arc<Occluders>>,
    profile: Option<SharedProfile>,
    stage: Stage,
    line: Option<Entity>,
    fade: Option<Entity>,
    tint: Option<Entity>,
    choices: Vec<Entity>,
    /// How long the choice has waited.
    waited: f32,
    elian: Option<Entity>,
    /// Whether Elian is down on his knees yet, nodding off, asleep.
    knelt: bool,
    nodded: bool,
    slept: bool,
    /// Whether he has got up to walk into the rift.
    risen: bool,
    waiting: Vec<Entity>,
    rift: Vec<Entity>,
    rift_light: Option<Entity>,
    dawn: Option<Entity>,
    motes: Vec<Mote>,
    seed: u32,
    clock: f32,
    /// Where the camera starts, low over the doorway, and where it ends, on the rift.
    glide: (Vec2, Vec2),
    hero_sheet: Option<TextureId>,
    kneeling_sheet: Option<TextureId>,
}

impl Default for EpilogueScene {
    fn default() -> Self {
        Self {
            world: World::new(),
            camera: Camera::default(),
            occluders: None,
            profile: None,
            stage: Stage::Telling(0.0),
            line: None,
            fade: None,
            tint: None,
            choices: Vec::new(),
            waited: 0.0,
            elian: None,
            knelt: false,
            nodded: false,
            slept: false,
            risen: false,
            waiting: Vec::new(),
            rift: Vec::new(),
            rift_light: None,
            dawn: None,
            motes: Vec::new(),
            seed: 0x41,
            clock: 0.0,
            glide: (Vec2::ZERO, Vec2::ZERO),
            hero_sheet: None,
            kneeling_sheet: None,
        }
    }
}

/// Where a spot `column`, `row` tiles into the hall lies in the world.
fn at(column: f32, row: f32) -> Vec2 {
    (Vec2::new(column, row) + (MARGIN + 1) as f32) * TILE_SIZE
}

/// How long the telling runs before the choice.
fn telling_seconds() -> f32 {
    LINES.iter().map(|(seconds, _)| seconds).sum()
}

/// Which shot `time` falls in, and how far into it.
fn shot_at(time: f32) -> (usize, f32) {
    let mut shot = 0;
    let mut into = time;
    while shot + 1 < LINES.len() && into > LINES[shot].0 {
        into -= LINES[shot].0;
        shot += 1;
    }
    (shot, into)
}

fn smooth(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

impl EpilogueScene {
    pub(crate) fn attach_profile(&mut self, profile: SharedProfile) {
        self.profile = Some(profile);
    }

    fn roll(&mut self) -> f32 {
        self.seed = self.seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (self.seed >> 8) as f32 / (1u32 << 24) as f32
    }

    /// The hall at the bottom: the temple's stone, a carpet up the middle from a doorway in the
    /// south wall, torches either side of the head, and the rift there.
    fn build_hall(&mut self, assets: &Assets) {
        let (columns, rows) = HALL;
        let width = columns + 2 + 2 * MARGIN;
        let height = rows + 2 + 2 * MARGIN;
        let mut grid = TileGrid::filled(width, height);
        let inside = MARGIN + 1;
        for row in 0..rows {
            for column in 0..columns {
                grid.carve(inside + column, inside + row);
            }
        }
        // The doorway Elian comes in by, on down out of sight.
        for row in inside + rows..height {
            grid.carve(inside + AISLE as usize, row);
        }
        let theme = Theme::Temple;
        let wall = |column: usize| (inside + column, inside - 1);
        let tile = |column: usize, row: usize| (inside + column, inside + row);
        let decor = vec![
            Decor { kind: DecorKind::Torch { mount: Mount::Top, lit: true }, tile: wall(2) },
            Decor { kind: DecorKind::Torch { mount: Mount::Top, lit: true }, tile: wall(columns - 3) },
            Decor { kind: DecorKind::Cobweb { variant: 0, left: false }, tile: tile(0, 0) },
            Decor { kind: DecorKind::Cobweb { variant: 1, left: true }, tile: tile(columns - 1, 0) },
            Decor { kind: DecorKind::Bones { variant: 0 }, tile: tile(2, rows - 2) },
        ];
        let room = DungeonRoom {
            kind: RoomKind::Normal,
            cell: (0, 0),
            origin: (MARGIN, MARGIN),
            columns: columns + 2,
            rows: rows + 2,
            spawns: Vec::new(),
            doors: Vec::new(),
            decor,
            furnishing: None,
            overheads: Vec::new(),
            hanging: Vec::new(),
            track: None,
            rug: Some(Carpet::rect(tile(AISLE as usize - 1, 5), 3, rows - 5, 0)),
            tables: Vec::new(),
            waterworks: Waterworks::default(),
            pendulum: None,
        };
        let dungeon = Dungeon { grid, rooms: vec![room], theme, surfaces: Surfaces::default(), vestibules: Vec::new() };
        let textures = &assets.textures;
        let [red, green, blue] = theme.backdrop();
        prefabs::backdrop_colored(
            &mut self.world,
            Vec4::new(f32::from(red) / 255.0, f32::from(green) / 255.0, f32::from(blue) / 255.0, 1.0),
        );
        prefabs::tilemap(&mut self.world, &dungeon.grid, assets.tileset_of(theme.wide_tiles()), textures.get(theme.tileset_texture()));
        let grid = &dungeon.grid;
        self.occluders = Some(Arc::new(Occluders::new(grid.columns(), grid.rows(), TILE_SIZE, |column, row| {
            grid.is_wall(column as isize, row as isize)
        })));
        let mut physics = crate::physics::Physics::new();
        for decor in &dungeon.rooms[0].decor {
            prefabs::decor(&mut self.world, &mut physics, *decor, textures);
        }
        systems::spawn_furniture(&mut self.world, &mut physics, &dungeon, textures);
        systems::spawn_atmosphere(&mut self.world, &dungeon, 0x29);

        // The rift: purple chaos with a torn edge wherever it does not go on.
        for &(column, row) in &RIFT {
            let has = |dx: isize, dy: isize| {
                RIFT.contains(&(((column as isize) + dx) as usize, ((row as isize) + dy) as usize))
            };
            let mask = [(0, -1, 1), (1, 0, 2), (0, 1, 4), (-1, 0, 8)]
                .iter()
                .filter(|&&(dx, dy, _)| has(dx, dy))
                .map(|&(_, _, bit)| bit)
                .sum::<usize>();
            let clip = Clip { row: mask, frames: RIFT_FRAMES, frame_time: 0.14, looping: true, flipped: false };
            self.rift.push(self.world.spawn((
                Position(at(column as f32 + 0.5, row as f32 + 0.5)),
                Size(Vec2::splat(TILE_SIZE)),
                Color(Vec4::ONE),
                Sprite::cell(textures.get(tex::RIFT), 0, mask, RIFT_FRAMES, 16),
                Animation::new(RIFT_FRAMES, 16, clip),
                Layer(-1),
            )));
        }
        let (red, green, blue) = RIFT_GLOW;
        self.rift_light = Some(self.world.spawn((Position(at(AISLE, 2.0)), Light::rgb(Vec2::ZERO, red, green, blue, 330.0, 0.9))));
        self.dawn = Some(self.world.spawn((Position(at(AISLE, STOPS_AT)), Light::rgb(Vec2::ZERO, 255.0, 214.0, 150.0, 260.0, 0.0))));

        // The waiting, not yet come out of the light.
        for &(column, brother) in &WAITING {
            let sheet = if brother { tex::PRIEST } else { tex::PILGRIM };
            self.waiting.push(self.world.spawn((
                Position(at(column, WAITING_ROW)),
                Size(PRIEST_SIZE),
                Color(Vec4::new(GHOST.x, GHOST.y, GHOST.z, 0.0)),
                Sprite::cell(textures.get(sheet), 0, STANDING.row, 4, 4),
                Animation::new(4, 4, STANDING),
                Layer(1),
                YSort::bottom(PRIEST_SIZE),
                Light::rgb(Vec2::new(0.0, -10.0), 180.0, 140.0, 255.0, 90.0, 0.0),
            )));
        }

        // Elian, coming in at the doorway, his back to us.
        let size = prefabs::PLAYER_SIZE;
        let hero = textures.get(tex::PLAYER);
        self.hero_sheet = Some(hero);
        self.kneeling_sheet = Some(textures.get(tex::ELIAN_KNEELING));
        let elian = self.world.spawn((
            Position(at(AISLE, ENTERS_AT)),
            Size(size),
            Color(Vec4::ONE),
            Sprite::cell(hero, 0, WALK_AWAY.row, HERO.0, HERO.1),
            Animation::new(HERO.0, HERO.1, WALK_AWAY),
            Layer(1),
            YSort::bottom(size),
            Lit,
        ));
        if let Ok(mut animation) = self.world.get::<&mut Animation>(elian) {
            animation.hold(0);
        }
        self.elian = Some(elian);

        self.camera = Camera::new(dungeon.grid.pixel_size());
        self.glide = (at(AISLE, ENTERS_AT - 1.5) - TARGET / 2.0, at(AISLE, 4.3) - TARGET / 2.0);
        self.camera.snap_to(self.glide.0);
    }

    /// The film frame over it: black bars, the line in the lower one, a tint for the memory of
    /// that night, and the fade.
    fn frame(&mut self) {
        for y in [BAR / 2.0, TARGET.y - BAR / 2.0] {
            self.world.spawn((
                Position(Vec2::new(TARGET.x / 2.0, y)),
                Size(Vec2::new(TARGET.x, BAR)),
                Color(Vec4::new(0.0, 0.0, 0.0, 1.0)),
                Layer(OVERLAY_LAYER),
                ScreenSpace,
            ));
        }
        self.tint = Some(self.world.spawn((Position(TARGET / 2.0), Size(TARGET), Color(Vec4::ZERO), Layer(OVERLAY_LAYER - 1), ScreenSpace)));
        self.line = Some(self.world.spawn((
            Position(Vec2::new(TARGET.x / 2.0, TARGET.y - BAR / 2.0)),
            Text::centered(String::new(), INK, FontSize::Size22),
            Layer(OVERLAY_LAYER + 1),
            ScreenSpace,
        )));
        self.fade = Some(self.world.spawn((
            Position(TARGET / 2.0),
            Size(TARGET),
            Color(Vec4::new(0.0, 0.0, 0.0, 1.0)),
            Layer(OVERLAY_LAYER + 2),
            ScreenSpace,
        )));
    }

    /// Types `words` out, `time` after they started.
    fn type_line(&mut self, words: &str, time: f32) {
        let shown = ((time - LINE_FROM).max(0.0) * LETTERS_PER_SECOND) as usize;
        if let Some(line) = self.line {
            if let Ok(mut text) = self.world.get::<&mut Text>(line) {
                text.content = words.chars().take(shown).collect();
            }
        }
    }

    fn set_alpha(&mut self, entity: Option<Entity>, alpha: f32) {
        if let Some(entity) = entity {
            if let Ok(mut color) = self.world.get::<&mut Color>(entity) {
                color.0.w = alpha.clamp(0.0, 1.0);
            }
        }
    }

    fn set_light(&mut self, entity: Option<Entity>, intensity: f32) {
        if let Some(entity) = entity {
            if let Ok(mut light) = self.world.get::<&mut Light>(entity) {
                light.intensity = intensity.max(0.0);
            }
        }
    }

    fn play_elian(&mut self, sheet: Option<TextureId>, (columns, rows): (usize, usize), clip: Clip) {
        let (Some(elian), Some(sheet)) = (self.elian, sheet) else { return };
        let _ = self.world.insert(
            elian,
            (Sprite::cell(sheet, 0, clip.row, columns, rows), Animation::new(columns, rows, clip)),
        );
    }

    #[cfg(test)]
    fn elian_at(&self) -> Vec2 {
        self.elian.and_then(|elian| self.world.get::<&Position>(elian).ok().map(|position| position.0)).unwrap_or(Vec2::ZERO)
    }

    fn move_elian(&mut self, to: Vec2) {
        if let Some(elian) = self.elian {
            if let Ok(mut position) = self.world.get::<&mut Position>(elian) {
                position.0 = to;
            }
        }
    }

    /// A spark drifting up from `at`.
    fn spark(&mut self, at: Vec2, colour: Vec4, rise: f32) {
        let entity = self.world.spawn((Position(at), Size(Vec2::splat(4.0)), Color(colour), Layer(MOTE_LAYER)));
        let drift = Vec2::new((self.roll() - 0.5) * 30.0, -rise - self.roll() * rise);
        let life = 0.8 + self.roll() * 1.2;
        self.motes.push(Mote { entity, drift, left: life, life });
    }

    /// Sparks rising off what is left of the rift, `rate` a second, and the ones in the air
    /// drifting and fading.
    fn churn(&mut self, dt: f32, rate: f32) {
        let open: Vec<Vec2> = self
            .rift
            .iter()
            .filter_map(|&tile| {
                let color = self.world.get::<&Color>(tile).ok()?;
                (color.0.w > 0.3).then(|| self.world.get::<&Position>(tile).ok().map(|position| position.0)).flatten()
            })
            .collect();
        let mut wanted = rate * dt;
        while !open.is_empty() && (wanted >= 1.0 || self.roll() < wanted) {
            wanted -= 1.0;
            let pick = (self.roll() * open.len() as f32) as usize % open.len();
            let spot = open[pick] + (Vec2::new(self.roll(), self.roll()) - 0.5) * TILE_SIZE;
            let colour = if self.roll() > 0.5 { MOTE_BRIGHT } else { MOTE_DEEP };
            self.spark(spot, colour, 40.0);
        }
        let world = &mut self.world;
        self.motes.retain_mut(|mote| {
            mote.left -= dt;
            if mote.left <= 0.0 {
                let _ = world.despawn(mote.entity);
                return false;
            }
            if let Ok((position, color)) = world.query_one_mut::<(&mut Position, &mut Color)>(mote.entity) {
                position.0 += mote.drift * dt;
                color.0.w = (mote.left / mote.life).min(1.0);
            }
            true
        });
    }

    /// The telling: Elian walks in and stops, the waiting come out of the light, the wound
    /// speaks, and he kneels and remembers.
    fn tell(&mut self, time: f32, dt: f32) {
        let (shot, into) = shot_at(time);
        self.type_line(LINES[shot].1, into);
        let walked = ((time - WALKS_FROM) * PACE).max(0.0);
        let row = (ENTERS_AT - walked).max(STOPS_AT);
        self.move_elian(at(AISLE, row));
        let walking = walked > 0.0 && row > STOPS_AT;
        if !self.knelt {
            if let Some(elian) = self.elian {
                if let Ok(mut animation) = self.world.get::<&mut Animation>(elian) {
                    if walking {
                        animation.switch_to(WALK_AWAY);
                    } else {
                        // Standing: the first frame of walking away, held.
                        animation.play(WALK_AWAY);
                        animation.hold(0);
                    }
                }
            }
        }
        // The camera follows him up the hall to the rift.
        let up = smooth(time / (LINES[0].0 + 2.0));
        self.camera.snap_to(self.glide.0.lerp(self.glide.1, up));

        // The waiting come out of the light one by one, each with a burst of sparks.
        for (index, &entity) in self.waiting.clone().iter().enumerate() {
            let from = APPEAR_FROM + index as f32 * APPEAR_EVERY;
            let seen = ((time - from) / APPEAR_SECONDS).clamp(0.0, 1.0);
            let alpha = seen * GHOST.w;
            if seen > 0.0 && seen < 1.0 && self.roll() < 0.6 {
                let spot = at(WAITING[index].0, WAITING_ROW) + Vec2::new((self.roll() - 0.5) * 30.0, (self.roll() - 0.3) * 60.0);
                self.spark(spot, MOTE_BRIGHT, 30.0);
            }
            self.set_alpha(Some(entity), alpha);
        }

        // The wound speaks: the rift swells and the ground trembles under him.
        let speaking = if shot == 2 { (std::f32::consts::PI * (into / LINES[2].0)).sin() } else { 0.0 };
        self.set_light(self.rift_light, 0.9 + 0.8 * speaking);
        if shot == 2 && into > 1.0 && into < 2.0 {
            self.camera.add_trauma(0.4 * dt);
        }
        for &tile in &self.rift.clone() {
            if let Ok(mut color) = self.world.get::<&mut Color>(tile) {
                color.0 = Vec4::new(1.0 + 0.3 * speaking, 1.0, 1.0 + 0.2 * speaking, 1.0);
            }
        }

        // He kneels as he knelt that night, nods off as he did, and remembers: the night comes
        // back over the hall and the brothers kneel at their prayers again.
        if shot == 3 {
            if into >= KNEELS && !self.knelt {
                self.knelt = true;
                self.play_elian(self.kneeling_sheet, KNEELING, KNEEL);
            }
            if into >= NODS && !self.nodded {
                self.nodded = true;
                self.play_elian(self.kneeling_sheet, KNEELING, NOD);
            }
            let memory = smooth((into - MEMORY.0) / 0.8).min(smooth((MEMORY.1 - into) / 0.8));
            if let Some(tint) = self.tint {
                if let Ok(mut color) = self.world.get::<&mut Color>(tint) {
                    color.0 = NIGHT_TINT * Vec4::new(1.0, 1.0, 1.0, memory);
                }
            }
            let praying = into > MEMORY.0 && into < MEMORY.1;
            for &entity in &self.waiting {
                if let Ok(mut animation) = self.world.get::<&mut Animation>(entity) {
                    animation.switch_to(if praying { PRAYING } else { STANDING });
                }
            }
            if into >= MEMORY.1 && self.nodded {
                // Awake again, on his knees, his head bowed.
                self.nodded = false;
                self.play_elian(self.kneeling_sheet, KNEELING, KNEEL);
            }
        }
        self.churn(dt, 6.0 + 10.0 * speaking);
    }

    fn offer(&mut self) {
        self.type_line(CHOOSING, 99.0);
        for (index, label) in CHOICES.into_iter().enumerate() {
            let entity = self.world.spawn((
                Position(choice_center(index)),
                Text::centered(label.to_owned(), CHOICE_INK, FontSize::Size27),
                Color(CHOICE_INK),
                Layer(OVERLAY_LAYER + 1),
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
        self.type_line("", 0.0);
        self.stage = Stage::After(ending, 0.0);
    }

    /// Letting them go: each fades into light where it stands, the rift closes from its ends
    /// in, the warm light comes up, and Elian sleeps.
    fn let_go(&mut self, time: f32, dt: f32) {
        self.type_line(CLOSED, time - 1.2);
        for (index, &entity) in self.waiting.clone().iter().enumerate() {
            let from = RELEASE_FROM + index as f32 * RELEASE_EVERY;
            let gone = ((time - from) / RELEASE_SECONDS).clamp(0.0, 1.0);
            if gone > 0.0 && gone < 1.0 {
                let spot = at(WAITING[index].0, WAITING_ROW);
                for _ in 0..2 {
                    let jitter = Vec2::new((self.roll() - 0.5) * 34.0, (self.roll() - 0.5) * 70.0);
                    self.spark(spot + jitter, LET_GO, 60.0);
                }
            }
            if let Ok(mut color) = self.world.get::<&mut Color>(entity) {
                let warm = GHOST.lerp(LET_GO, smooth(gone * 2.0));
                color.0 = Vec4::new(warm.x, warm.y, warm.z, GHOST.w * (1.0 - smooth(gone)));
            }
        }
        // The rift closes from its ends inward.
        let closing = ((time - CLOSE_FROM) / CLOSE_SECONDS).clamp(0.0, 1.0);
        for (index, &tile) in self.rift.clone().iter().enumerate() {
            let (column, row) = RIFT[index];
            let from_middle = (column as f32 + 0.5 - AISLE).abs() / 3.5 + if row == 2 { -0.3 } else { 0.0 };
            let shut = smooth((closing - (1.0 - from_middle) * 0.6) / 0.4);
            self.set_alpha(Some(tile), 1.0 - shut);
        }
        self.set_light(self.rift_light, 0.9 * (1.0 - closing));
        self.set_light(self.dawn, 0.85 * smooth((time - CLOSE_FROM) / 3.0));
        if time >= SLEEPS && !self.slept {
            self.slept = true;
            self.play_elian(self.kneeling_sheet, KNEELING, ASLEEP);
        }
        self.churn(dt, 6.0 * (1.0 - closing));
    }

    /// Staying: Elian gets up and walks into the rift between them; they go into the light
    /// after him and the purple takes everything.
    fn stay(&mut self, time: f32, dt: f32) {
        self.type_line(LEFT_OPEN, time - 3.0);
        if time >= RISES && !self.risen {
            self.risen = true;
            self.play_elian(self.hero_sheet, HERO, WALK_AWAY);
            if let Some(elian) = self.elian {
                if let Ok(mut animation) = self.world.get::<&mut Animation>(elian) {
                    animation.hold(0);
                }
            }
        }
        let walked = ((time - WALKS_IN) * WALK_IN_PACE).max(0.0);
        let row = (STOPS_AT - walked).max(1.9);
        if self.risen {
            self.move_elian(at(AISLE, row));
            if let Some(elian) = self.elian {
                if let Ok(mut animation) = self.world.get::<&mut Animation>(elian) {
                    if walked > 0.0 && row > 1.9 {
                        animation.switch_to(WALK_AWAY);
                    } else if walked > 0.0 {
                        animation.hold(0);
                    }
                }
            }
        }
        // He fades into the rift once he steps onto it, and they after him.
        let into_rift = ((3.2 - row) / 1.2).clamp(0.0, 1.0);
        self.set_alpha(self.elian, 1.0 - into_rift);
        let after = ((time - WALKS_IN - 4.6) / 1.6).clamp(0.0, 1.0);
        for &entity in &self.waiting.clone() {
            self.set_alpha(Some(entity), GHOST.w * (1.0 - after));
        }
        let swallow = smooth((time - WALKS_IN - 4.0) / 3.0);
        self.set_light(self.rift_light, 0.9 + 1.4 * swallow);
        if let Some(tint) = self.tint {
            if let Ok(mut color) = self.world.get::<&mut Color>(tint) {
                color.0 = SWALLOW * Vec4::new(1.0, 1.0, 1.0, swallow);
            }
        }
        self.camera.snap_to(self.glide.1.lerp(at(AISLE, 3.2) - TARGET / 2.0, smooth(time / 6.0)));
        self.churn(dt, 8.0 + 30.0 * swallow);
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

    fn soundtrack(&self) -> Option<Soundtrack> {
        Some(Soundtrack::MainMenu)
    }

    fn fx(&self) -> SceneFx {
        SceneFx::default()
    }

    fn camera(&self) -> Vec2 {
        self.camera.view()
    }

    fn occluders(&self) -> Option<Arc<Occluders>> {
        self.occluders.clone()
    }

    fn enter(&mut self, assets: &Assets) {
        let profile = self.profile.take();
        *self = Self { profile, ..Self::default() };
        self.build_hall(assets);
        self.frame();
    }

    fn leave(&mut self) {
        self.world.clear();
        self.occluders = None;
        self.motes.clear();
        self.waiting.clear();
        self.rift.clear();
        self.choices.clear();
    }

    fn update(&mut self, input: &Input, dt: f32, _assets: &Assets, _settings: &mut Settings) -> Transition {
        self.clock += dt;
        self.camera.update_shake(dt);
        let mut fade = 0.0;
        match self.stage {
            Stage::Telling(time) => {
                let time = time + dt;
                fade = 1.0 - time / FADE_SECONDS;
                // Esc, Space or Enter skips the telling to the choice.
                let skip = [KeyCode::Escape, KeyCode::Space, KeyCode::Enter].into_iter().any(|key| input.just_pressed(key));
                if time >= telling_seconds() || skip && time > 0.5 {
                    // Skipped or not, Elian is on his knees for the choice.
                    self.move_elian(at(AISLE, STOPS_AT));
                    if !self.knelt {
                        self.knelt = true;
                        self.play_elian(self.kneeling_sheet, KNEELING, KNEEL);
                        for &entity in &self.waiting.clone() {
                            self.set_alpha(Some(entity), GHOST.w);
                        }
                        self.camera.snap_to(self.glide.1);
                    }
                    self.offer();
                } else {
                    self.tell(time, dt);
                    self.stage = Stage::Telling(time);
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
                self.churn(dt, 6.0);
            }
            Stage::After(ending, time) => {
                let time = time + dt;
                let seconds = match ending {
                    Ending::Closed => CLOSED_SECONDS,
                    Ending::LeftOpen => OPEN_SECONDS,
                };
                if time >= seconds {
                    return Transition::Switch(SceneId::Lobby);
                }
                match ending {
                    Ending::Closed => self.let_go(time, dt),
                    Ending::LeftOpen => self.stay(time, dt),
                }
                fade = 1.0 - (seconds - time) / FADE_SECONDS;
                self.stage = Stage::After(ending, time);
            }
        }
        self.set_alpha(self.fade, fade);
        // Each of the waiting glows as much as it is there.
        for &entity in &self.waiting {
            if let Ok((color, light)) = self.world.query_one_mut::<(&Color, &mut Light)>(entity) {
                light.intensity = color.0.w * 0.7;
            }
        }
        systems::flicker_flames(&mut self.world);
        systems::tick_dust(&mut self.world, self.clock, dt);
        systems::follow_backdrop(&mut self.world, self.camera.view());
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

        run(&mut scene, &idle, telling_seconds() + 0.5);
        assert_eq!(scene.stage, Stage::Choosing(0));

        let mut enter = Input::default();
        enter.press(KeyCode::Enter);
        run(&mut scene, &enter, 0.1);
        assert_eq!(profile.ending(), Some(Ending::Closed));

        assert_eq!(run(&mut scene, &idle, CLOSED_SECONDS + 1.0), Transition::Switch(SceneId::Lobby));
        assert!(scene.slept, "Elian sleeps once they are let go");
    }

    #[test]
    fn staying_is_chosen_with_the_right_key_and_elian_walks_into_the_rift() {
        let profile = SharedProfile::in_memory();
        let mut scene = EpilogueScene::default();
        scene.attach_profile(profile.clone());
        scene.enter(&Assets::new().expect("assets"));
        run(&mut scene, &Input::default(), telling_seconds() + 0.5);
        let mut right = Input::default();
        right.press(KeyCode::ArrowRight);
        run(&mut scene, &right, 0.1);
        let mut enter = Input::default();
        enter.press(KeyCode::Enter);
        run(&mut scene, &enter, 0.1);
        assert_eq!(profile.ending(), Some(Ending::LeftOpen));
        let kneeling = scene.elian_at();
        run(&mut scene, &Input::default(), OPEN_SECONDS - 1.5);
        assert!(scene.elian_at().y < kneeling.y - 3.0 * TILE_SIZE, "he walks up into the rift");
    }

    #[test]
    fn the_waiting_come_out_of_the_light_while_it_is_told() {
        let mut scene = EpilogueScene::default();
        scene.enter(&Assets::new().expect("assets"));
        let alpha = |scene: &EpilogueScene| scene.world.get::<&Color>(scene.waiting[0]).map(|color| color.0.w).unwrap();
        assert_eq!(alpha(&scene), 0.0);
        run(&mut scene, &Input::default(), APPEAR_FROM + APPEAR_SECONDS + 0.5);
        assert!(alpha(&scene) > 0.5);
    }
}
