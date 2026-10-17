//! The opening cinematic, played in the game's own engine so it looks like the game: rooms
//! built from the dungeon's tiles and props, lit by its lights, the camera gliding through
//! them with black bars above and below and a line of the story typed out at the bottom.
//!
//! Its shots so far: the temple as it was before the wound, the camera moving down the nave to
//! the altar; then the wound tearing open beneath it, a rift of purple chaos running from the
//! altar down the nave as the ground shakes and the angel bursts apart; then what crawls out of it, the
//! first spiders, rats and bats, scattering into the dark as the temple sinks into the rift's purple.

use std::sync::Arc;

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::Assets;
use crate::audio::{AudioCommand, Soundtrack};
use crate::camera::Camera;
use crate::assets::tex;
use crate::components::{Animation, Clip, Color, Layer, Light, Position, ScreenSpace, Size, Sprite, Text};
use crate::enemies::EnemyKind;
use crate::dungeon::{
    Carpet, Decor, DecorKind, Dungeon, DungeonRoom, Hanging, HangingKind, Mount, Overhead, OverheadKind, RoomKind,
    Surface, Surfaces, Theme, Waterworks,
};
use crate::input::Input;
use crate::postfx::SceneFx;
use crate::prefabs;
use crate::props::{Look, Prop};
use crate::render::{FontSize, Occluders};
use crate::settings::Settings;
use crate::systems;
use crate::tilemap::{TILE_SIZE, TileGrid};
use crate::viewport::TARGET;

use super::{Cursor, Scene, SceneId, Transition};

/// The temple's nave, inside its walls, in tiles.
const NAVE: (usize, usize) = (34, 7);
const MARGIN: usize = 2;
/// The shots: how long each runs and its line, typed out a few letters a second once the shot
/// has settled. The cinematic fades in at the start and out at the end.
const SHOTS: [(f32, &str); 5] = [
    (11.0, "Long ago, a temple stood on the hill."),
    (14.0, "Then something tore open beneath it."),
    (10.0, "They called it Bane. The wound."),
    (9.5, "One acolyte escaped, and woke the temple's last guardian."),
    (5.5, "Rise. Close the wound."),
];
/// The cut from the temple to the crypt: the screen goes black for this long either side.
const CUT_SECONDS: f32 = 0.9;
/// The crypt: its inside in tiles; when light starts to leak from the sarcophagus, when its lid
/// shakes, slides and falls, and the white flash as the hero rises.
const CRYPT: (usize, usize) = (17, 9);
const GLOW_FROM: f32 = 1.4;
const LID_SHAKES: f32 = 2.8;
const LID_SLIDES: f32 = 3.8;
const LID_FALLS: f32 = 5.6;
const RISE_AT: f32 = 8.4;
const FLASH_SECONDS: f32 = 0.9;
/// The sarcophagus and its lid on screen (tools/make_bier.py draws them at 50x26 and 46x20).
const SARCOPHAGUS: Vec2 = Vec2::new(50.0 * 3.0, 26.0 * 3.0);
const LID_SIZE: Vec2 = Vec2::new(46.0 * 3.0, 20.0 * 3.0);
/// What crawls out of the wound in the third shot: what, how long into the shot it climbs out,
/// how far along the crack (0 at the altar, 1 at its far end), and where it heads, in tiles from
/// where it climbed out.
const CRAWLERS: [(EnemyKind, f32, f32, (f32, f32)); 14] = [
    (EnemyKind::RedSpiderling, 0.6, 0.62, (-2.5, -1.8)),
    (EnemyKind::Rat, 0.9, 0.7, (-4.0, 1.8)),
    (EnemyKind::PurpleSpiderling, 1.3, 0.55, (3.0, 1.8)),
    (EnemyKind::Bat, 1.6, 0.66, (-3.0, -5.0)),
    (EnemyKind::Spider, 2.0, 0.6, (-1.5, 1.8)),
    (EnemyKind::Rat, 2.3, 0.74, (2.5, -1.8)),
    (EnemyKind::RedSpiderling, 2.6, 0.68, (-5.0, -1.8)),
    (EnemyKind::Bat, 3.0, 0.58, (4.0, -5.5)),
    (EnemyKind::GreenSpiderling, 3.3, 0.72, (-3.5, 1.8)),
    (EnemyKind::Rat, 3.7, 0.64, (-6.0, -1.8)),
    (EnemyKind::PurpleSpiderling, 4.1, 0.78, (2.0, 1.8)),
    (EnemyKind::Bat, 4.4, 0.7, (-5.0, -6.0)),
    (EnemyKind::Rat, 4.8, 0.6, (5.5, 1.8)),
    (EnemyKind::RedSpiderling, 5.2, 0.66, (-6.5, 1.8)),
];
/// How fast they climb out (fading in from the glow) and how fast they scurry, tiles a second.
const CLIMB_SECONDS: f32 = 0.7;
const SCURRY: f32 = 1.6;
/// The purple the temple sinks into once the wound is open.
const DUSK: Vec4 = Vec4::new(0.12, 0.02, 0.18, 0.42);
const FADE_SECONDS: f32 = 1.6;
const LINE_FROM: f32 = 1.4;
/// The tear: the ground shakes for a moment, then the crack runs from the altar down the nave
/// over this long, the angel falling a little after it starts.
/// The narrator speaks the shot's line over a still temple first; the ground starts to shake only
/// this long into it, once the line is spoken.
const TEAR_FROM: f32 = 4.4;
const RUMBLE_SECONDS: f32 = 1.2;
const CRACK_SECONDS: f32 = 3.6;
const ANGEL_FALLS: f32 = 1.9;
const SHAKE: f32 = 0.45;
/// The angel shudders this long before it bursts.
const SHUDDER_SECONDS: f32 = 0.9;
/// How fast a thrown piece falls back, pixels a second a second.
const GRAVITY: f32 = 900.0;
/// The rift's sheet (see tools/make_rift.py): frames of churning across, a row for each set
/// of open sides down; how long a tile takes to spring open; the layers it and its sparks lie on.
const RIFT_FRAMES: usize = 4;
const OPEN_SECONDS: f32 = 0.25;
const RIFT_LAYER: i32 = -1;
const MOTE_LAYER: i32 = 6;
const MOTE_BRIGHT: Vec4 = Vec4::new(0.92, 0.55, 1.0, 1.0);
const MOTE_DEEP: Vec4 = Vec4::new(0.62, 0.22, 0.9, 1.0);

/// One of the temple's acolytes: where he kneels, where he hides once the ground shakes, and
/// whether something has caught him (and when).
struct Acolyte {
    entity: Entity,
    home: Vec2,
    hide: Vec2,
    caught: Option<f32>,
}

/// Where the acolytes of the Order of the Seal kneel along the carpet, in tiles inside the nave,
/// and which way they run: Teodor, Oswin, Cassian, Aldric, and Elian the novice, the last, who
/// gets away to the crypt (see story.md, chapter 9).
const ACOLYTES: [(f32, f32, f32); 5] =
    [(10.0, 1.3, -1.0), (14.0, 5.6, 1.0), (18.0, 1.3, -1.0), (22.0, 5.6, 1.0), (7.0, 5.6, 1.0)];
/// Elian, who escapes: he runs on out of the nave, and waits in the crypt.
const ELIAN: usize = 4;
/// When they start to run once the ground shakes, how fast, how fast the creatures hunt them,
/// and how long one takes to fall.
const FLEE_AFTER: f32 = 1.2;
const FLEE_SPEED: f32 = 2.6;
const HUNT_SPEED: f32 = 3.4;
const FALL_SECONDS: f32 = 0.35;

/// A piece of the broken angel, a chip of stone or a puff of dust in flight: the spot on the
/// floor under it and its speed across, its height and how fast that changes, and how long it
/// has left (pieces stay where they land).
struct Debris {
    entity: Entity,
    ground: Vec2,
    velocity: Vec2,
    height: f32,
    rise: f32,
    left: Option<f32>,
}
const LETTERS_PER_SECOND: f32 = 18.0;
/// The black bars of a film frame, and where the line sits in the lower one.
const BAR: f32 = 40.0;
const LINE_AT: Vec2 = Vec2::new(TARGET.x / 2.0, TARGET.y - BAR / 2.0);
const LINE_COLOR: Vec4 = Vec4::new(0.93, 0.89, 0.82, 1.0);
const OVERLAY_LAYER: i32 = 20;
/// The layer the carpet's tiles lie on (see `prefabs::carpet`), for tearing it.
const CARPET_LAYER: i32 = -2;

#[derive(Default)]
pub(crate) struct CinematicScene {
    world: World,
    camera: Camera,
    occluders: Option<Arc<Occluders>>,
    clock: f32,
    sounds: Vec<AudioCommand>,
    line: Option<Entity>,
    fade: Option<Entity>,
    /// Where the camera starts and ends its glide.
    glide: (Vec2, Vec2),
    /// The rift's tiles, from the altar down the nave, and how many have opened; each opened
    /// tile with the moment it opened, as it flares open.
    crack: Vec<(usize, usize)>,
    opened: usize,
    opening: Vec<(Entity, f32)>,
    /// Sparks of the chaos drifting up out of the rift: each, its drift, and how long it has
    /// left of how long.
    motes: Vec<(Entity, Vec2, f32, f32)>,
    /// The angel and where it stands; once it breaks, its pieces, chips and dust in flight.
    angel: Option<Entity>,
    angel_at: Vec2,
    debris: Vec<Debris>,
    seed: u32,
    altar: Vec2,
    /// Where the camera pulls back to as the crack runs past it.
    retreat: Vec2,
    /// What has climbed out of the wound so far: each one, where it came out and where it
    /// heads, and when.
    crawlers: Vec<(Entity, Vec2, Vec2, f32)>,
    /// The acolytes, and which crawler hunts which of them.
    acolytes: Vec<Acolyte>,
    hunters: Vec<(usize, usize)>,
    dusk: Option<Entity>,
    /// The crypt, once the cinematic has cut to it: the sarcophagus and its lid, the light
    /// leaking out of it, the white flash, and the hero once risen.
    crypt: bool,
    bier: Option<Entity>,
    lid: Option<Entity>,
    lid_at: Vec2,
    lid_landed: bool,
    glow: Option<Entity>,
    flash: Option<Entity>,
    hero: Option<Entity>,
    /// What comes after: the training the first time, otherwise the lobby.
    next: Option<SceneId>,
}

impl CinematicScene {
    /// The temple, whole, and nobody in it: a long nave of the temple's stone with pairs of
    /// blocks down it and pennants strung between them, censers swinging over a royal carpet,
    /// two alcoves where sand pours from the wall, fire bowls at the blocks' feet, and at the far
    /// end an apse with an angel on the altar between candles.
    fn build_temple(&mut self, assets: &Assets) {
        let (columns, rows) = NAVE;
        let apse = (7, 2);
        let width = columns + 2 + 2 * MARGIN;
        let height = rows + 2 + 2 * MARGIN + 2 * apse.1;
        let mut grid = TileGrid::filled(width, height);
        let inside = (MARGIN + 1, MARGIN + 1 + apse.1);
        let tile = |column: usize, row: usize| (inside.0 + column, inside.1 + row);
        for row in 0..rows {
            for column in 0..columns {
                grid.carve(inside.0 + column, inside.1 + row);
            }
        }
        // The apse: the nave's end opens up and down round the altar.
        for row in inside.1 - apse.1..inside.1 + rows + apse.1 {
            for column in inside.0 + columns - apse.0..inside.0 + columns {
                grid.carve(column, row);
            }
        }
        // Pairs of blocks down the nave, against both walls.
        let pillars = [4usize, 13, 22];
        for &column in &pillars {
            for row in [0, 1, rows - 2, rows - 1] {
                for dx in 0..2 {
                    let (x, y) = tile(column + dx, row);
                    grid.fill(x, y);
                }
            }
        }
        let theme = Theme::Temple;
        let mut surfaces = Surfaces::default();
        let mut waterworks = Waterworks::default();
        // Between the first pair and the second, and the second and the third, an alcove of
        // sand with a pipe pouring more into it, against the top wall.
        for &column in &[8usize, 17] {
            for dx in 0..3 {
                for row in 0..2 {
                    surfaces.insert(tile(column + dx, row), Surface::Sand);
                }
            }
            waterworks.pipes.push((inside.0 + column + 1, inside.1 - 1));
        }
        let mut decor = Vec::new();
        for &column in &pillars {
            decor.push(Decor { kind: DecorKind::FireBowl, tile: tile(column - 2, 1) });
            decor.push(Decor { kind: DecorKind::FireBowl, tile: tile(column + 2, rows - 2) });
        }
        // Banners on the top wall either side of each alcove.
        for &column in &[7usize, 11, 16, 20] {
            decor.push(Decor { kind: DecorKind::Banner, tile: (inside.0 + column, inside.1 - 1) });
        }
        // Pennants hanging straight down from the top wall either side of each block.
        let overheads = pillars
            .iter()
            .flat_map(|&column| [column - 1, column + 2])
            .map(|column| Overhead {
                kind: OverheadKind::Pennants,
                upright: true,
                first: tile(column, 0),
                last: tile(column, 1),
            })
            .collect();
        // Censers in pairs over the alcoves and the last bay, out of the way of the carpet.
        let hanging = [(8usize, 2usize), (10, 2), (17, 2), (19, 2), (26, 2), (29, 2)]
            .into_iter()
            .map(|(column, row)| Hanging { kind: HangingKind::Censer, tile: tile(column, row) })
            .collect();
        let room = DungeonRoom {
            kind: RoomKind::Normal,
            cell: (0, 0),
            origin: (MARGIN, MARGIN),
            columns: columns + 2,
            rows: rows + 2 + 2 * apse.1,
            spawns: Vec::new(),
            doors: Vec::new(),
            decor,
            furnishing: None,
            overheads,
            hanging,
            track: None,
            rug: Some(Carpet {
                origin: tile(0, rows / 2 - 1),
                columns: columns - apse.0 + 3,
                rows: 3,
                style: theme.style(),
            }),
            tables: Vec::new(),
            waterworks,
            pendulum: None,
        };
        let dungeon = Dungeon {
            grid,
            rooms: vec![room],
            theme,
            surfaces,
            vestibules: Vec::new(),
        };
        let textures = &assets.textures;
        let [red, green, blue] = theme.backdrop();
        prefabs::backdrop_colored(
            &mut self.world,
            Vec4::new(f32::from(red) / 255.0, f32::from(green) / 255.0, f32::from(blue) / 255.0, 1.0),
        );
        prefabs::tilemap(
            &mut self.world,
            &dungeon.grid,
            assets.tileset_of(theme.wide_tiles()),
            textures.get(theme.tileset_texture()),
        );
        let grid = &dungeon.grid;
        self.occluders = Some(Arc::new(Occluders::new(grid.columns(), grid.rows(), TILE_SIZE, |column, row| {
            grid.is_wall(column as isize, row as isize)
        })));
        for (spot, surface) in dungeon.surfaces.iter() {
            prefabs::surface_tile(&mut self.world, &dungeon.grid, &dungeon.surfaces, spot, surface, textures);
        }
        let mut shores = std::collections::BTreeSet::new();
        for (spot, _) in dungeon.surfaces.iter() {
            for dy in -1isize..=1 {
                for dx in -1isize..=1 {
                    shores.insert(((spot.0 as isize + dx) as usize, (spot.1 as isize + dy) as usize));
                }
            }
        }
        for spot in shores {
            if let Some((surface, row)) = dungeon.surfaces.shore_at(&dungeon.grid, spot) {
                prefabs::shore_tile(&mut self.world, surface, spot, row, textures);
            }
        }
        let mut physics = crate::physics::Physics::new();
        for room in &dungeon.rooms {
            for decor in &room.decor {
                prefabs::decor(&mut self.world, &mut physics, *decor, textures);
            }
        }
        systems::spawn_overheads(&mut self.world, &dungeon, textures);
        systems::spawn_hanging(&mut self.world, &dungeon, textures);
        systems::spawn_furniture(&mut self.world, &mut physics, &dungeon, textures);
        systems::spawn_waterworks(&mut self.world, &mut physics, &dungeon, textures);
        systems::spawn_atmosphere(&mut self.world, &dungeon, 0x7e3);
        let at = |column: f32, row: f32| (Vec2::new(inside.0 as f32 + column, inside.1 as f32 + row)) * TILE_SIZE;
        // The altar in the apse: an angel between two candlesticks, two fire bowls before it.
        let altar = at(columns as f32 - apse.0 as f32 / 2.0, rows as f32 / 2.0);
        let statue = Prop::AngelStatue.spec().size();
        self.angel = Some(prefabs::prop(&mut self.world, Prop::AngelStatue, altar - statue / 2.0, Look::DEFAULT, textures));
        self.angel_at = altar;
        self.altar = altar;
        let candle = Prop::Candlestick.spec().size();
        // The acolytes, kneeling either side of the carpet down the nave.
        let size = crate::prefabs::PLAYER_SIZE;
        for (index, &(column, row, run)) in ACOLYTES.iter().enumerate() {
            let home = at(column, row);
            let away = if index == ELIAN { 7.5 } else { 3.0 };
            let hide = home + Vec2::new(-away * TILE_SIZE, run * 0.6 * TILE_SIZE);
            let clip = Clip { row: 0, frames: 6, frame_time: 0.16 + index as f32 * 0.01, looping: true, flipped: index % 2 == 1 };
            let entity = self.world.spawn((
                Position(home),
                Size(size),
                Color(Vec4::ONE),
                Sprite::cell(textures.get(tex::ACOLYTE), 0, 0, 6, 4),
                Animation::new(6, 4, clip),
                Layer(1),
                crate::components::YSort::bottom(size),
                crate::components::Lit,
            ));
            self.acolytes.push(Acolyte { entity, home, hide, caught: None });
        }
        // Four candles round it, the same above as below.
        for (dx, dy) in [(-2.2, -1.9), (2.2, -1.9), (-2.2, 1.9), (2.2, 1.9)] {
            let spot = altar + Vec2::new(dx * TILE_SIZE, dy * TILE_SIZE);
            prefabs::prop(&mut self.world, Prop::Candlestick, spot - candle / 2.0, Look::DEFAULT, textures);
        }
        let bounds = dungeon.grid.pixel_size();
        self.camera = Camera::new(bounds);
        // The camera's position is its top left corner; it glides with the nave's middle row
        // in the middle of the screen.
        let middle = at(0.0, rows as f32 / 2.0).y;
        self.glide = (
            Vec2::new(at(6.0, 0.0).x, middle) - TARGET / 2.0,
            Vec2::new(at(columns as f32 - 5.5, 0.0).x, middle) - TARGET / 2.0,
        );
        self.camera.snap_to(self.glide.0);
        self.retreat = Vec2::new(at(15.0, 0.0).x, middle) - TARGET / 2.0;
        // The crack: from the altar's foot down the nave, wandering across the carpet a row at
        // a time, two tiles wide where it is oldest.
        let mut crack = Vec::new();
        let mut row = rows as i32 / 2;
        let mut seed = 0x5eed_u32;
        for column in (6..columns - 3).rev() {
            seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            match (seed >> 16) % 5 {
                0 => row -= 1,
                1 => row += 1,
                _ => {}
            }
            row = row.clamp(2, rows as i32 - 4);
            // A tile wide, two by the altar where it opened first.
            crack.push(tile(column, row as usize));
            if column > columns * 2 / 3 {
                crack.push(tile(column, row as usize + 1));
            }
        }
        crack.dedup();
        self.crack = crack;
        self.opened = 0;
        self.seed = 0x0b5e_55ed;
    }

    fn roll(&mut self) -> f32 {
        self.seed = self.seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (self.seed >> 8) as f32 / (1u32 << 24) as f32
    }

    /// Opens the next tile of the rift: purple chaos with a torn edge wherever the rift does not
    /// go on, the carpet torn away above it, a purple light every few tiles.
    fn open_rift(&mut self, at: f32, textures: &crate::assets::Textures) {
        let spot = self.crack[self.opened];
        let has = |dx: isize, dy: isize| {
            let near = ((spot.0 as isize + dx) as usize, (spot.1 as isize + dy) as usize);
            self.crack.contains(&near)
        };
        let mask = [(0, -1, 1), (1, 0, 2), (0, 1, 4), (-1, 0, 8)]
            .iter()
            .filter(|&&(dx, dy, _)| has(dx, dy))
            .map(|&(_, _, bit)| bit)
            .sum::<usize>();
        let centre = (Vec2::new(spot.0 as f32, spot.1 as f32) + 0.5) * TILE_SIZE;
        let clip = Clip { row: mask, frames: RIFT_FRAMES, frame_time: 0.14, looping: true, flipped: false };
        let entity = self.world.spawn((
            Position(centre),
            Size(Vec2::splat(TILE_SIZE)),
            Color(Vec4::new(1.0, 1.0, 1.0, 0.0)),
            Sprite::cell(textures.get(tex::RIFT), 0, mask, RIFT_FRAMES, 16),
            Animation::new(RIFT_FRAMES, 16, clip),
            Layer(RIFT_LAYER),
        ));
        if self.opened % 5 == 0 {
            let _ = self.world.insert_one(entity, Light::rgb(Vec2::ZERO, 150.0, 60.0, 230.0, 90.0, 0.8));
        }
        self.opening.push((entity, at));
        self.opened += 1;
        let torn: Vec<Entity> = self
            .world
            .query::<(Entity, &Position, &Layer)>()
            .iter()
            .filter(|(_, position, layer)| layer.0 == CARPET_LAYER && position.0.distance(centre) < 1.0)
            .map(|(entity, ..)| entity)
            .collect();
        for entity in torn {
            let _ = self.world.despawn(entity);
        }
    }

    /// Sparks of chaos rising from the open rift, and the ones already in the air drifting and
    /// fading.
    fn churn(&mut self, dt: f32, rate: f32) {
        let mut wanted = rate * dt;
        while self.opened > 0 && (wanted >= 1.0 || self.roll() < wanted) {
            wanted -= 1.0;
            let pick = (self.roll() * self.opened as f32) as usize % self.opened;
            let spot = self.crack[pick];
            let at = (Vec2::new(spot.0 as f32 + self.roll(), spot.1 as f32 + self.roll())) * TILE_SIZE;
            let colour = if self.roll() > 0.5 { MOTE_BRIGHT } else { MOTE_DEEP };
            let entity = self.world.spawn((Position(at), Size(Vec2::splat(4.0)), Color(colour), Layer(MOTE_LAYER)));
            let drift = Vec2::new((self.roll() - 0.5) * 30.0, -40.0 - self.roll() * 50.0);
            let life = 0.8 + self.roll() * 1.2;
            self.motes.push((entity, drift, life, life));
        }
        let world = &mut self.world;
        self.motes.retain_mut(|(entity, drift, left, life)| {
            *left -= dt;
            if *left <= 0.0 {
                let _ = world.despawn(*entity);
                return false;
            }
            if let Ok((position, color)) = world.query_one_mut::<(&mut Position, &mut Color)>(*entity) {
                position.0 += *drift * dt;
                color.0.w = (*left / *life).min(1.0);
            }
            true
        });
    }

    /// The angel breaking: it shudders and darkens, then bursts into pieces that fly and land,
    /// chips of stone that scatter and fade, and a cloud of dust.
    fn break_angel(&mut self, at: f32, textures: &crate::assets::Textures) {
        let Some(angel) = self.angel else { return };
        let shudder = ((at - (ANGEL_FALLS - SHUDDER_SECONDS)) / SHUDDER_SECONDS).clamp(0.0, 1.0);
        if shudder > 0.0 {
            let jolt = Vec2::new((at * 61.0).sin(), (at * 47.0).cos()) * 3.0 * shudder;
            if let Ok((position, color)) = self.world.query_one_mut::<(&mut Position, &mut Color)>(angel) {
                position.0 = self.angel_at + jolt.round();
                let dim = 1.0 - 0.35 * shudder * (0.5 + 0.5 * (at * 30.0).sin());
                color.0 = Vec4::new(dim, dim * 0.92, dim, 1.0);
            }
        }
        if at < ANGEL_FALLS {
            return;
        }
        let _ = self.world.despawn(angel);
        self.angel = None;
        self.camera.add_trauma(0.7);
        let origin = self.angel_at + Vec2::new(0.0, 60.0);
        // The pieces: its head, its body and a drum of its pedestal, thrown out and down.
        for (row, throw) in [(0usize, Vec2::new(-150.0, 30.0)), (1, Vec2::new(120.0, 70.0)), (2, Vec2::new(-40.0, 110.0))] {
            let size = Prop::StatuePiece.spec().size();
            let entity = prefabs::prop(&mut self.world, Prop::StatuePiece, origin - size / 2.0, Look::row(row, false), textures);
            self.debris.push(Debris { entity, ground: origin, velocity: throw, height: 90.0, rise: 160.0, left: None });
        }
        for _ in 0..22 {
            let angle = self.roll() * std::f32::consts::TAU;
            let speed = 90.0 + self.roll() * 180.0;
            let grey = 0.5 + self.roll() * 0.3;
            let big = self.roll() > 0.6;
            let entity = self.world.spawn((
                Position(origin),
                Size(Vec2::splat(if big { 8.0 } else { 4.0 })),
                Color(Vec4::new(grey, grey * 0.95, grey * 1.05, 1.0)),
                Layer(MOTE_LAYER),
            ));
            let velocity = Vec2::from_angle(angle) * Vec2::new(speed, speed * 0.6);
            let left = 0.8 + self.roll() * 0.8;
            let height = 60.0 + self.roll() * 60.0;
            let rise = 120.0 + self.roll() * 160.0;
            self.debris.push(Debris { entity, ground: origin, velocity, height, rise, left: Some(left) });
        }
        for i in 0..7 {
            let angle = i as f32 / 7.0 * std::f32::consts::TAU;
            let entity = self.world.spawn((
                Position(origin),
                Size(Vec2::splat(24.0)),
                Color(Vec4::new(0.55, 0.5, 0.48, 0.55)),
                Layer(MOTE_LAYER - 1),
            ));
            self.debris.push(Debris { entity, ground: origin, velocity: Vec2::from_angle(angle) * 70.0, height: 10.0, rise: 30.0, left: Some(1.4) });
        }
    }

    fn fly_debris(&mut self, dt: f32) {
        let world = &mut self.world;
        self.debris.retain_mut(|piece| {
            if let Some(left) = &mut piece.left {
                *left -= dt;
                if *left <= 0.0 {
                    let _ = world.despawn(piece.entity);
                    return false;
                }
            }
            if piece.height > 0.0 || piece.rise > 0.0 {
                piece.rise -= GRAVITY * dt;
                piece.height = (piece.height + piece.rise * dt).max(0.0);
                piece.ground += piece.velocity * dt;
                if piece.height <= 0.0 {
                    // A dull bounce, then it lies still.
                    piece.rise = if piece.rise < -120.0 { -piece.rise * 0.3 } else { 0.0 };
                    piece.velocity *= 0.4;
                }
            }
            if let Ok((position, size, color)) = world.query_one_mut::<(&mut Position, &mut Size, &mut Color)>(piece.entity) {
                position.0 = (piece.ground - Vec2::new(0.0, piece.height)).round();
                if let Some(left) = piece.left {
                    let dust = size.0.x > 16.0;
                    color.0.w = (left / 0.6).min(1.0) * if dust { 0.55 } else { 1.0 };
                    if dust {
                        size.0 += Vec2::splat(30.0 * dt);
                    }
                }
            }
            true
        });
    }

    /// The second shot: the ground shakes, a rift of purple chaos runs from the altar down the
    /// nave, tearing the carpet, the angel shudders and bursts into pieces, and the camera backs
    /// away along the rift.
    fn tear(&mut self, assets: &Assets, at: f32, dt: f32) {
        let textures = &assets.textures;
        let at = at - TEAR_FROM;
        if at < 0.0 {
            // Still, on the altar, while the line is spoken.
            self.camera.snap_to(self.glide.1);
            return;
        }
        if at > 0.3 {
            self.camera.add_trauma(SHAKE * dt * 4.0);
        }
        let running = ((at - RUMBLE_SECONDS) / CRACK_SECONDS).clamp(0.0, 1.0);
        let wanted = (running * self.crack.len() as f32) as usize;
        while self.opened < wanted {
            self.open_rift(at, textures);
        }
        // Each tile flares open out of the floor.
        let world = &mut self.world;
        self.opening.retain(|&(entity, since)| {
            let grown = ((at - since) / OPEN_SECONDS).clamp(0.0, 1.0);
            if let Ok(mut color) = world.get::<&mut Color>(entity) {
                color.0.w = grown;
            }
            grown < 1.0
        });
        self.churn(dt, 30.0 * running.min(1.0) + if self.opened > 0 { 8.0 } else { 0.0 });
        self.flee(at);
        self.break_angel(at, textures);
        self.fly_debris(dt);
        // Hold on the altar while it breaks, then back away down the nave with the rift.
        let back = ((at - ANGEL_FALLS - 0.6) / 5.5).clamp(0.0, 1.0);
        let eased = back * back * (3.0 - 2.0 * back);
        self.camera.snap_to(self.glide.1.lerp(self.retreat, eased));
    }

    /// The acolytes, once the ground shakes, get up and run for the walls, away from the altar.
    fn flee(&mut self, at: f32) {
        for acolyte in &self.acolytes {
            if acolyte.caught.is_some() {
                continue;
            }
            let since = at - FLEE_AFTER;
            let span = acolyte.home.distance(acolyte.hide).max(1.0);
            let run = (since * FLEE_SPEED * TILE_SIZE / span).clamp(0.0, 1.0);
            if let Ok(mut position) = self.world.get::<&mut Position>(acolyte.entity) {
                position.0 = acolyte.home.lerp(acolyte.hide, run);
            }
            if let Ok(mut animation) = self.world.get::<&mut Animation>(acolyte.entity) {
                // Running on their side, then cowering facing the room.
                let row = if since > 0.0 && run < 1.0 { 1 } else { 0 };
                if animation.clip().row != row {
                    animation.play(Clip { row, frames: 6, frame_time: if row == 1 { 0.08 } else { 0.16 }, looping: true, flipped: row == 1 });
                }
            }
        }
    }

    /// A creature falls on an acolyte: he flashes red and falls, and his blood spreads where he
    /// lay.
    fn kill(&mut self, index: usize, at: f32) {
        let acolyte = &mut self.acolytes[index];
        acolyte.caught = Some(at);
        tracing::info!(at, index, "an acolyte is caught");
        let spot = self.world.get::<&Position>(acolyte.entity).map(|p| p.0).unwrap_or(acolyte.hide);
        let entity = acolyte.entity;
        if let Ok(mut color) = self.world.get::<&mut Color>(entity) {
            color.0 = Vec4::new(1.0, 0.25, 0.25, 1.0);
        }
        self.camera.add_trauma(0.15);
        let foot = spot + Vec2::new(0.0, crate::prefabs::PLAYER_SIZE.y * 0.35);
        for _ in 0..7 {
            let offset = Vec2::new((self.roll() - 0.5) * 50.0, (self.roll() - 0.5) * 22.0);
            let side = 10.0 + self.roll() * 16.0;
            let shade = 0.32 + self.roll() * 0.2;
            self.world.spawn((
                Position((foot + offset).round()),
                Size(Vec2::new(side, side * 0.6).round()),
                Color(Vec4::new(shade, 0.02, 0.04, 1.0)),
                Layer(-1),
            ));
        }
        for _ in 0..14 {
            let angle = self.roll() * std::f32::consts::TAU;
            let speed = 60.0 + self.roll() * 120.0;
            let entity = self.world.spawn((
                Position(spot),
                Size(Vec2::splat(4.0)),
                Color(Vec4::new(0.7, 0.05, 0.08, 1.0)),
                Layer(MOTE_LAYER),
            ));
            let left = 0.5 + self.roll() * 0.5;
            let rise = 60.0 + self.roll() * 90.0;
            self.debris.push(Debris { entity, ground: foot, velocity: Vec2::from_angle(angle) * speed, height: 30.0, rise, left: Some(left) });
        }
    }

    /// The fallen: red, then fading into the floor.
    fn fall(&mut self, at: f32) {
        for acolyte in &self.acolytes {
            let Some(caught) = acolyte.caught else { continue };
            let since = at - caught;
            if let Ok(mut color) = self.world.get::<&mut Color>(acolyte.entity) {
                let gone = (since / FALL_SECONDS).clamp(0.0, 1.0);
                color.0 = Vec4::new(1.0, 0.25 + 0.2 * gone, 0.25, 1.0 - gone);
            }
        }
    }

    /// The third shot: spiders, rats and bats climb out of the rift and scatter, and the temple
    /// sinks into its purple.
    fn crawl(&mut self, assets: &Assets, at: f32, dt: f32) {
        let textures = &assets.textures;
        for &(kind, when, along, (dx, dy)) in &CRAWLERS[self.crawlers.len()..] {
            if at < when || self.crack.is_empty() {
                break;
            }
            let index = (along * (self.crack.len() - 1) as f32) as usize;
            let spot = self.crack[index.min(self.crack.len() - 1)];
            let from = (Vec2::new(spot.0 as f32, spot.1 as f32) + 0.5) * TILE_SIZE;
            let to = from + Vec2::new(dx, dy) * TILE_SIZE;
            let entity = prefabs::enemy(&mut self.world, kind, from, 1, textures);
            let _ = self.world.remove_one::<crate::components::Spawning>(entity);
            if let Ok(mut color) = self.world.get::<&mut Color>(entity) {
                color.0.w = 0.0;
            }
            if let Ok(mut animation) = self.world.get::<&mut crate::components::Animation>(entity) {
                let idle = kind.spec().idle_clip();
                animation.play(crate::components::Clip { flipped: to.x < from.x, ..idle });
            }
            let crawler = self.crawlers.len();
            self.crawlers.push((entity, from, to, when));
            // The first walkers out each go for the nearest acolyte nobody is hunting yet.
            if kind != EnemyKind::Bat {
                let prey = self
                    .acolytes
                    .iter()
                    .enumerate()
                    .filter(|(index, acolyte)| {
                        *index != ELIAN && acolyte.caught.is_none() && !self.hunters.iter().any(|&(_, hunted)| hunted == *index)
                    })
                    .min_by(|(_, a), (_, b)| a.hide.distance(from).total_cmp(&b.hide.distance(from)))
                    .map(|(index, _)| index);
                if let Some(prey) = prey {
                    self.hunters.push((crawler, prey));
                }
            }
        }
        // The hunters run their prey down.
        let mut caught = Vec::new();
        for &(crawler, prey) in &self.hunters {
            let (entity, _, _, when) = self.crawlers[crawler];
            if at - when < CLIMB_SECONDS * 0.5 || self.acolytes[prey].caught.is_some() {
                continue;
            }
            let target = self.world.get::<&Position>(self.acolytes[prey].entity).map(|p| p.0).unwrap_or_default();
            if let Ok(mut position) = self.world.get::<&mut Position>(entity) {
                let gap = target - position.0;
                if gap.length() < 26.0 {
                    caught.push(prey);
                } else {
                    position.0 += gap.normalize() * (HUNT_SPEED * TILE_SIZE * dt).min(gap.length());
                }
                let left = gap.x < 0.0;
                drop(position);
                if let Ok(mut animation) = self.world.get::<&mut Animation>(entity) {
                    if animation.clip().flipped != left {
                        let clip = animation.clip();
                        animation.play(Clip { flipped: left, ..clip });
                    }
                }
            }
        }
        for prey in caught {
            self.kill(prey, at);
        }
        self.fall(at);
        for (crawler, &(entity, from, to, when)) in self.crawlers.iter().enumerate() {
            let hunting = self.hunters.iter().any(|&(hunter, prey)| hunter == crawler && self.acolytes[prey].caught.is_none());
            let fed = self.hunters.iter().any(|&(hunter, prey)| hunter == crawler && self.acolytes[prey].caught.is_some());
            let since = at - when;
            let climbed = (since / CLIMB_SECONDS).clamp(0.0, 1.0);
            let walked = ((since - CLIMB_SECONDS * 0.5) * SCURRY * TILE_SIZE / from.distance(to).max(1.0)).clamp(0.0, 1.0);
            // A hunter moves by its own chase; once it has caught its prey it stays on it.
            if !hunting && !fed {
                if let Ok(mut position) = self.world.get::<&mut Position>(entity) {
                    position.0 = from.lerp(to, walked);
                }
            }
            if let Ok(mut color) = self.world.get::<&mut Color>(entity) {
                color.0.w = climbed;
            }
        }
        self.churn(dt, 26.0);
        self.fly_debris(dt);
        if let Some(dusk) = self.dusk {
            if let Ok(mut color) = self.world.get::<&mut Color>(dusk) {
                color.0 = DUSK * Vec4::new(1.0, 1.0, 1.0, (at / 4.0).min(1.0));
            }
        }
        // A last slow drift down the nave as they pour out.
        let drift = (at / SHOTS[2].0).clamp(0.0, 1.0);
        self.camera.snap_to(self.retreat.lerp(self.retreat + Vec2::new(-2.0 * TILE_SIZE, 0.0), drift));
        if at < 2.5 {
            self.camera.add_trauma(SHAKE * dt * 2.0);
        }
    }

    /// The crypt the hero sleeps in: a stone vault with blocks in its corners, his sarcophagus in
    /// the middle on a red carpet, candles round it, older sarcophagi closed along the
    /// walls, torches, shackles, chains, cobwebs and bones.
    fn build_crypt(&mut self, assets: &Assets) {
        let (columns, rows) = CRYPT;
        let width = columns + 2 + 2 * MARGIN;
        let height = rows + 2 + 2 * MARGIN;
        let mut grid = TileGrid::filled(width, height);
        let inside = (MARGIN + 1, MARGIN + 1);
        let tile = |column: usize, row: usize| (inside.0 + column, inside.1 + row);
        for row in 0..rows {
            for column in 0..columns {
                grid.carve(inside.0 + column, inside.1 + row);
            }
        }
        // Blocks in the four corners, as the dungeon's treasure rooms have.
        for (column, row) in [(1, 1), (columns - 3, 1), (1, rows - 3), (columns - 3, rows - 3)] {
            for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                let (x, y) = tile(column + dx, row + dy);
                grid.fill(x, y);
            }
        }
        let theme = Theme::Stone;
        let wall = |column: usize| (inside.0 + column, inside.1 - 1);
        let last = columns - 1;
        let middle = columns / 2;
        let decor = vec![
            Decor { kind: DecorKind::Torch { mount: Mount::Top, lit: true }, tile: wall(4) },
            Decor { kind: DecorKind::Torch { mount: Mount::Top, lit: true }, tile: wall(last - 4) },
            Decor { kind: DecorKind::Banner, tile: wall(middle) },
            Decor { kind: DecorKind::Shackles { variant: 1 }, tile: wall(middle - 2) },
            Decor { kind: DecorKind::Shackles { variant: 0 }, tile: wall(middle + 2) },
            Decor { kind: DecorKind::Cobweb { variant: 0, left: false }, tile: tile(0, 0) },
            Decor { kind: DecorKind::Cobweb { variant: 1, left: true }, tile: tile(last, 0) },
            Decor { kind: DecorKind::Bones { variant: 0 }, tile: tile(3, rows - 1) },
            Decor { kind: DecorKind::Bones { variant: 1 }, tile: tile(last - 3, 0) },
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
            // A red carpet in the middle of the vault, the sarcophagus on it.
            rug: Some(Carpet { origin: tile(middle - 3, rows / 2 - 2), columns: 7, rows: 5, style: 0 }),
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
        systems::spawn_atmosphere(&mut self.world, &dungeon, 0x41);
        let at = |column: f32, row: f32| Vec2::new(inside.0 as f32 + column, inside.1 as f32 + row) * TILE_SIZE;
        // Chains down the wall either side of the banner.
        for column in [middle as f32 - 4.0, middle as f32 + 4.0] {
            prefabs::prop(&mut self.world, Prop::Chains, at(column, -1.0), Look::DEFAULT, textures);
        }
        // Older sarcophagi, closed, between the corner blocks along both walls.
        for (column, row) in [(5.5, 0.9), (columns as f32 - 5.5, 0.9), (5.5, rows as f32 - 0.9), (columns as f32 - 5.5, rows as f32 - 0.9)] {
            let spot = at(column, row);
            self.world.spawn((Position(spot), Size(SARCOPHAGUS * 0.8), Color(Vec4::new(0.8, 0.8, 0.85, 1.0)),
                Sprite::cell(textures.get(tex::SARCOPHAGUS), 1, 0, 2, 1), Layer(0), crate::components::Lit));
            self.world.spawn((Position(spot - Vec2::new(0.0, 0.1 * SARCOPHAGUS.y * 0.8)), Size(LID_SIZE * 0.8),
                Color(Vec4::new(0.8, 0.8, 0.85, 1.0)), Sprite::whole(textures.get(tex::SARCOPHAGUS_LID)), Layer(0), crate::components::Lit));
        }
        // His own, in the middle, candles round it.
        // In the middle of the vault, on the middle of its carpet.
        let centre = at(columns as f32 / 2.0, rows as f32 / 2.0);
        self.bier = Some(self.world.spawn((
            Position(centre),
            Size(SARCOPHAGUS),
            Color(Vec4::ONE),
            Sprite::cell(textures.get(tex::SARCOPHAGUS), 0, 0, 2, 1),
            Layer(0),
            crate::components::Lit,
        )));
        self.lid_at = centre - Vec2::new(0.0, (SARCOPHAGUS.y - LID_SIZE.y) / 2.0 + 3.0 * 3.0);
        self.lid = Some(self.world.spawn((
            Position(self.lid_at),
            Size(LID_SIZE),
            Color(Vec4::ONE),
            Sprite::whole(textures.get(tex::SARCOPHAGUS_LID)),
            Layer(1),
            crate::components::Lit,
        )));
        // Elian, who ran down here, standing at its head.
        let size = crate::prefabs::PLAYER_SIZE;
        let clip = Clip { row: 0, frames: 6, frame_time: 0.16, looping: true, flipped: false };
        self.world.spawn((
            Position(centre + Vec2::new(-SARCOPHAGUS.x / 2.0 - 0.7 * TILE_SIZE, 0.1 * TILE_SIZE)),
            Size(size),
            Color(Vec4::ONE),
            Sprite::cell(textures.get(tex::ACOLYTE), 0, 0, 6, 4),
            Animation::new(6, 4, clip),
            Layer(1),
            crate::components::YSort::bottom(size),
            crate::components::Lit,
        ));
        let candle = Prop::Candlestick.spec().size();
        for (dx, dy) in [(-2.4, -1.4), (2.4, -1.4), (-2.4, 1.6), (2.4, 1.6)] {
            let spot = centre + Vec2::new(dx, dy) * TILE_SIZE;
            prefabs::prop(&mut self.world, Prop::Candlestick, spot - candle / 2.0, Look::DEFAULT, textures);
        }
        // The light that leaks out of it, and grows as the lid comes off.
        self.glow = Some(self.world.spawn((
            Position(centre),
            Light::rgb(Vec2::ZERO, 255.0, 226.0, 150.0, 40.0, 0.0),
        )));
        let bounds = dungeon.grid.pixel_size();
        self.camera = Camera::new(bounds);
        self.glide = (at(6.0, rows as f32 / 2.0) - TARGET / 2.0, centre - TARGET / 2.0);
        self.camera.snap_to(self.glide.0);
        self.flash = Some(self.world.spawn((
            Position(TARGET / 2.0),
            Size(TARGET),
            Color(Vec4::new(1.0, 0.97, 0.88, 0.0)),
            Layer(OVERLAY_LAYER - 1),
            ScreenSpace,
        )));
    }

    /// The fourth and fifth shots: the camera moves across the vault to the sarcophagus, light
    /// leaks round its lid, the lid shakes, grinds aside and falls, and in a white flash the
    /// hero stands beside it.
    fn revive(&mut self, assets: &Assets, at: f32, dt: f32) {
        let textures = &assets.textures;
        let push = (at / (LID_SLIDES + 1.0)).clamp(0.0, 1.0);
        let eased = push * push * (3.0 - 2.0 * push);
        self.camera.snap_to(self.glide.0.lerp(self.glide.1, eased));
        let centre = self.bier.and_then(|bier| self.world.get::<&Position>(bier).ok().map(|p| p.0)).unwrap_or_default();
        // The lid: still, then shaking, then grinding aside, then falling off onto the floor.
        if let Some(lid) = self.lid {
            let slide = ((at - LID_SLIDES) / (LID_FALLS - LID_SLIDES)).clamp(0.0, 1.0);
            let slide = slide * slide * (3.0 - 2.0 * slide);
            let fall = ((at - LID_FALLS) / 0.35).clamp(0.0, 1.0);
            let mut spot = self.lid_at + Vec2::new(slide * 0.55 * LID_SIZE.x, 0.0);
            spot += Vec2::new(fall * 0.3 * LID_SIZE.x, fall * fall * 0.55 * SARCOPHAGUS.y);
            if at > LID_SHAKES && at < LID_FALLS {
                spot += Vec2::new((at * 57.0).sin(), (at * 43.0).cos()) * 2.0;
                self.camera.add_trauma(0.25 * dt);
            }
            if let Ok(mut position) = self.world.get::<&mut Position>(lid) {
                position.0 = spot.round();
            }
            if at >= LID_FALLS + 0.35 && !self.lid_landed {
                self.lid_landed = true;
                self.camera.add_trauma(0.55);
                for i in 0..8 {
                    let angle = i as f32 / 8.0 * std::f32::consts::TAU;
                    let entity = self.world.spawn((
                        Position(spot + Vec2::new(0.0, 20.0)),
                        Size(Vec2::splat(24.0)),
                        Color(Vec4::new(0.55, 0.5, 0.52, 0.5)),
                        Layer(MOTE_LAYER - 1),
                    ));
                    self.debris.push(Debris { entity, ground: spot + Vec2::new(0.0, 20.0), velocity: Vec2::from_angle(angle) * 80.0, height: 6.0, rise: 30.0, left: Some(1.3) });
                }
            }
        }
        // Light leaking round the lid, then pouring out once it is off.
        let leak = ((at - GLOW_FROM) / (LID_SLIDES - GLOW_FROM)).clamp(0.0, 1.0) * 0.4;
        let open = ((at - LID_SLIDES) / (RISE_AT - LID_SLIDES)).clamp(0.0, 1.0) * 0.6;
        let after = if at > RISE_AT { (1.0 - (at - RISE_AT) / 3.0).max(0.3) } else { 1.0 };
        if let Some(glow) = self.glow {
            if let Ok(mut light) = self.world.get::<&mut Light>(glow) {
                light.radius = 40.0 + 260.0 * (leak + open);
                light.intensity = 2.4 * (leak + open) * after;
            }
        }
        // Gold motes rising out of the open sarcophagus.
        let rate = if at > LID_SLIDES && at < RISE_AT + 0.8 { 8.0 + 30.0 * open } else { 0.0 };
        let mut wanted = rate * dt;
        while wanted >= 1.0 || (wanted > 0.0 && self.roll() < wanted) {
            wanted -= 1.0;
            let spot = centre + Vec2::new((self.roll() - 0.5) * 120.0, (self.roll() - 0.6) * 40.0);
            let entity = self.world.spawn((Position(spot), Size(Vec2::splat(4.0)), Color(Vec4::new(1.0, 0.9, 0.55, 1.0)), Layer(MOTE_LAYER)));
            let drift = Vec2::new((self.roll() - 0.5) * 16.0, -30.0 - self.roll() * 40.0);
            let life = 1.0 + self.roll();
            self.motes.push((entity, drift, life, life));
        }
        self.churn(dt, 0.0);
        self.fly_debris(dt);
        if at > RISE_AT - 0.25 && self.hero.is_none() {
            self.camera.add_trauma(0.35);
            if let Some(bier) = self.bier {
                if let Ok(mut sprite) = self.world.get::<&mut Sprite>(bier) {
                    *sprite = Sprite::cell(textures.get(tex::SARCOPHAGUS), 1, 0, 2, 1);
                }
            }
            let size = crate::prefabs::PLAYER_SIZE;
            let clip = Clip { row: 0, frames: 6, frame_time: 0.16, looping: true, flipped: false };
            self.hero = Some(self.world.spawn((
                Position(centre + Vec2::new(0.0, SARCOPHAGUS.y / 2.0 + size.y / 2.0 - 4.0)),
                Size(size),
                Color(Vec4::ONE),
                Sprite::cell(textures.get(tex::PLAYER), 0, 0, 6, 4),
                Animation::new(6, 4, clip),
                Layer(1),
                crate::components::Lit,
            )));
        }
        if let Some(flash) = self.flash {
            let since = at - (RISE_AT - 0.25);
            let alpha = if since < 0.0 { 0.0 } else if since < 0.12 { since / 0.12 } else { (1.0 - (since - 0.12) / FLASH_SECONDS).max(0.0) };
            if let Ok(mut color) = self.world.get::<&mut Color>(flash) {
                color.0.w = alpha * 0.9;
            }
        }
    }

    /// Goes from the temple to the crypt: everything of the temple cleared, the crypt built and
    /// the film frame put back over it.
    fn cut_to_crypt(&mut self, assets: &Assets) {
        self.world.clear();
        self.crawlers.clear();
        self.acolytes.clear();
        self.hunters.clear();
        self.opening.clear();
        self.motes.clear();
        self.debris.clear();
        self.angel = None;
        self.dusk = None;
        self.build_crypt(assets);
        self.frame();
        self.crypt = true;
    }

    /// The film frame over it all: bars above and below, the line in the lower one, and the
    /// black it fades in from.
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
        self.dusk = Some(self.world.spawn((
            Position(TARGET / 2.0),
            Size(TARGET),
            Color(Vec4::ZERO),
            Layer(OVERLAY_LAYER - 1),
            ScreenSpace,
        )));
        self.line = Some(self.world.spawn((
            Position(LINE_AT),
            Text::centered(String::new(), LINE_COLOR, FontSize::Size22),
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
}

impl Scene for CinematicScene {
    fn world(&self) -> &World {
        &self.world
    }

    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn enter(&mut self, assets: &Assets) {
        self.clock = 0.0;
        self.crypt = false;
        self.lid = None;
        self.lid_landed = false;
        self.hero = None;
        self.build_temple(assets);
        self.frame();
    }

    fn leave(&mut self) {
        self.world.clear();
        self.occluders = None;
        self.line = None;
        self.fade = None;
        self.dusk = None;
        self.crawlers.clear();
        self.acolytes.clear();
        self.hunters.clear();
        self.opening.clear();
        self.motes.clear();
        self.debris.clear();
        self.lid = None;
        self.lid_landed = false;
        self.hero = None;
        self.crypt = false;
    }

    fn soundtrack(&self) -> Option<Soundtrack> {
        Some(Soundtrack::Cinematic)
    }

    fn take_sounds(&mut self) -> Vec<AudioCommand> {
        std::mem::take(&mut self.sounds)
    }

    fn fx(&self) -> SceneFx {
        SceneFx::default()
    }

    fn cursor(&self) -> Cursor {
        Cursor::Hidden
    }

    fn camera(&self) -> Vec2 {
        self.camera.view()
    }

    fn occluders(&self) -> Option<Arc<Occluders>> {
        self.occluders.clone()
    }

    fn update(&mut self, input: &Input, dt: f32, assets: &Assets, _settings: &mut Settings) -> Transition {
        self.clock += dt;
        let next = self.next.unwrap_or(SceneId::Training);
        let total: f32 = SHOTS.iter().map(|(seconds, _)| seconds).sum();
        if self.clock > total || input.any_key() && self.clock > 0.5 {
            return Transition::Switch(next);
        }
        // Which shot it is, and how far into it.
        let mut shot = 0;
        let mut into_shot = self.clock;
        while shot + 1 < SHOTS.len() && into_shot > SHOTS[shot].0 {
            into_shot -= SHOTS[shot].0;
            shot += 1;
        }
        self.camera.update_shake(dt);
        if shot >= 3 && !self.crypt {
            self.cut_to_crypt(assets);
        }
        match shot {
            0 => {
                // A slow glide down the nave, easing in and out.
                let t = (into_shot / SHOTS[0].0).clamp(0.0, 1.0);
                let eased = t * t * (3.0 - 2.0 * t);
                self.camera.snap_to(self.glide.0.lerp(self.glide.1, eased));
            }
            1 => self.tear(assets, into_shot, dt),
            2 => self.crawl(assets, into_shot, dt),
            // The crypt runs on through its two lines.
            3 => self.revive(assets, into_shot, dt),
            _ => self.revive(assets, into_shot + SHOTS[3].0, dt),
        }
        if let Some(line) = self.line {
            let from = if shot == 0 { LINE_FROM + 0.4 } else { LINE_FROM };
            let shown = ((into_shot - from).max(0.0) * LETTERS_PER_SECOND) as usize;
            if let Ok(mut text) = self.world.get::<&mut Text>(line) {
                text.content = SHOTS[shot].1.chars().take(shown).collect();
            }
        }
        if let Some(fade) = self.fade {
            let into = 1.0 - self.clock / FADE_SECONDS;
            let out = 1.0 - (total - self.clock) / FADE_SECONDS;
            let cut_at: f32 = SHOTS[..3].iter().map(|(seconds, _)| seconds).sum();
            let cut = 1.0 - (self.clock - cut_at).abs() / CUT_SECONDS;
            if let Ok(mut color) = self.world.get::<&mut Color>(fade) {
                color.0.w = into.max(out).max(cut).clamp(0.0, 1.0);
            }
        }
        systems::flicker_flames(&mut self.world);
        systems::tick_hanging(&mut self.world, self.clock, dt);
        systems::tick_overheads(&mut self.world, dt);
        systems::tick_drips(&mut self.world, &assets.textures, dt);
        systems::tick_dust(&mut self.world, self.clock, dt);
        Transition::None
    }
}
