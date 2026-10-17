//! The opening cinematic, played in the game's own engine so it looks like the game: rooms
//! built from the dungeon's tiles and props, lit by its lights, the camera gliding through
//! them with black bars above and below and a line of the story typed out at the bottom.
//!
//! For now it holds its first shot: the temple as it was before the wound, the camera moving
//! down the nave to the altar.

use std::sync::Arc;

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::Assets;
use crate::audio::{AudioCommand, Soundtrack};
use crate::camera::Camera;
use crate::components::{Color, Layer, Position, ScreenSpace, Size, Text};
use crate::dungeon::{
    Carpet, Decor, DecorKind, Dungeon, DungeonRoom, Hanging, HangingKind, Overhead, OverheadKind, RoomKind, Surface,
    Surfaces, Theme, Waterworks,
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
/// How long the shot runs, and the fade into it and out of it.
const SHOT_SECONDS: f32 = 11.0;
const FADE_SECONDS: f32 = 1.6;
/// The line, typed out a few letters a second once the shot has settled.
const LINE: &str = "Long ago, a temple stood on the hill.";
const LINE_FROM: f32 = 1.8;
const LETTERS_PER_SECOND: f32 = 18.0;
/// The black bars of a film frame, and where the line sits in the lower one.
const BAR: f32 = 40.0;
const LINE_AT: Vec2 = Vec2::new(TARGET.x / 2.0, TARGET.y - BAR / 2.0);
const LINE_COLOR: Vec4 = Vec4::new(0.93, 0.89, 0.82, 1.0);
const OVERLAY_LAYER: i32 = 20;

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
            decor.push(Decor { kind: DecorKind::FireBowl, tile: tile(column - 1, 1) });
            decor.push(Decor { kind: DecorKind::FireBowl, tile: tile(column + 2, rows - 2) });
        }
        // Banners on the top wall either side of each alcove.
        for &column in &[7usize, 11, 16, 20] {
            decor.push(Decor { kind: DecorKind::Banner, tile: (inside.0 + column, inside.1 - 1) });
        }
        let overheads = pillars
            .windows(2)
            .map(|pair| Overhead {
                kind: OverheadKind::Pennants,
                upright: false,
                first: tile(pair[0] + 2, rows - 2),
                last: tile(pair[1] - 1, rows - 2),
            })
            .collect();
        let hanging = [8.5f32, 17.5, 26.5]
            .iter()
            .map(|&column| Hanging { kind: HangingKind::Censer, tile: tile(column as usize, rows / 2 + 1) })
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
        let altar = at(columns as f32 - apse.0 as f32 / 2.0, rows as f32 / 2.0 - 0.4);
        let statue = Prop::AngelStatue.spec().size();
        prefabs::prop(&mut self.world, Prop::AngelStatue, altar - statue / 2.0, Look::DEFAULT, textures);
        let candle = Prop::Candlestick.spec().size();
        for dx in [-2.4, 2.4] {
            let spot = altar + Vec2::new(dx * TILE_SIZE, 1.8 * TILE_SIZE);
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
        self.build_temple(assets);
        self.frame();
    }

    fn leave(&mut self) {
        self.world.clear();
        self.occluders = None;
        self.line = None;
        self.fade = None;
    }

    fn soundtrack(&self) -> Option<Soundtrack> {
        Some(Soundtrack::ProgressRoom)
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
        self.camera.position()
    }

    fn occluders(&self) -> Option<Arc<Occluders>> {
        self.occluders.clone()
    }

    fn update(&mut self, input: &Input, dt: f32, assets: &Assets, _settings: &mut Settings) -> Transition {
        self.clock += dt;
        let next = self.next.unwrap_or(SceneId::Training);
        if self.clock > SHOT_SECONDS || input.any_key() && self.clock > 0.5 {
            return Transition::Switch(next);
        }
        // A slow glide down the nave, easing in and out.
        let t = (self.clock / SHOT_SECONDS).clamp(0.0, 1.0);
        let eased = t * t * (3.0 - 2.0 * t);
        self.camera.snap_to(self.glide.0.lerp(self.glide.1, eased));
        if let Some(line) = self.line {
            let shown = ((self.clock - LINE_FROM).max(0.0) * LETTERS_PER_SECOND) as usize;
            if let Ok(mut text) = self.world.get::<&mut Text>(line) {
                text.content = LINE.chars().take(shown).collect();
            }
        }
        if let Some(fade) = self.fade {
            let into = 1.0 - self.clock / FADE_SECONDS;
            let out = 1.0 - (SHOT_SECONDS - self.clock) / FADE_SECONDS;
            if let Ok(mut color) = self.world.get::<&mut Color>(fade) {
                color.0.w = into.max(out).clamp(0.0, 1.0);
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
