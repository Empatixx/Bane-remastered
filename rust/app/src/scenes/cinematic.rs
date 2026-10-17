//! The opening cinematic, played in the game's own engine so it looks like the game: rooms
//! built from the dungeon's tiles and props, lit by its lights, the camera gliding through
//! them with black bars above and below and a line of the story typed out at the bottom.
//!
//! Its shots so far: the temple as it was before the wound, the camera moving down the nave to
//! the altar; then the wound tearing open beneath it, a burning crack running from the altar
//! down the nave as the ground shakes and the angel falls; then what crawls out of it, the
//! first spiders, rats and bats, scattering into the dark as the temple's fires redden.

use std::sync::Arc;

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::Assets;
use crate::audio::{AudioCommand, Soundtrack};
use crate::camera::Camera;
use crate::components::{Color, Layer, Position, ScreenSpace, Size, Text};
use crate::enemies::EnemyKind;
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
/// The shots: how long each runs and its line, typed out a few letters a second once the shot
/// has settled. The cinematic fades in at the start and out at the end.
const SHOTS: [(f32, &str); 3] = [
    (11.0, "Long ago, a temple stood on the hill."),
    (10.0, "Then something tore open beneath it."),
    (10.0, "They called it Bane. The wound."),
];
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
/// The red the temple sinks into once the wound is open.
const DUSK: Vec4 = Vec4::new(0.18, 0.02, 0.03, 0.42);
const FADE_SECONDS: f32 = 1.6;
const LINE_FROM: f32 = 1.4;
/// The tear: the ground shakes for a moment, then the crack runs from the altar down the nave
/// over this long, the angel falling a little after it starts.
const RUMBLE_SECONDS: f32 = 1.2;
const CRACK_SECONDS: f32 = 3.6;
const ANGEL_FALLS: f32 = 1.9;
const SHAKE: f32 = 0.45;
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
    /// The nave's floor plan and what lies on it, for the crack to burn its way through.
    grid: Option<TileGrid>,
    surfaces: Surfaces,
    /// The crack's tiles, from the altar down the nave, and how many have opened.
    crack: Vec<(usize, usize)>,
    opened: usize,
    /// The tiles round the crack that already have their bank drawn.
    banks: std::collections::HashSet<(usize, usize)>,
    angel: Option<Entity>,
    altar: Vec2,
    /// Where the camera pulls back to as the crack runs past it.
    retreat: Vec2,
    /// What has climbed out of the wound so far: each one, where it came out and where it
    /// heads, and when.
    crawlers: Vec<(Entity, Vec2, Vec2, f32)>,
    dusk: Option<Entity>,
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
        self.angel = Some(prefabs::prop(&mut self.world, Prop::AngelStatue, altar - statue / 2.0, Look::DEFAULT, textures));
        self.altar = altar;
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
        // The whole crack is known from the start, so each tile joins its neighbours and the
        // edges get their banks as it opens.
        self.surfaces = dungeon.surfaces.clone();
        for &spot in &crack {
            self.surfaces.insert(spot, Surface::Lava);
        }
        self.crack = crack;
        self.opened = 0;
        self.banks.clear();
        self.grid = Some(dungeon.grid.clone());
    }

    /// The second shot: the ground shakes, a burning crack runs from the altar down the nave,
    /// the angel falls in pieces and the camera backs away along the crack.
    fn tear(&mut self, assets: &Assets, at: f32, dt: f32) {
        let textures = &assets.textures;
        if at > 0.3 {
            self.camera.add_trauma(SHAKE * dt * 4.0);
        }
        let running = ((at - RUMBLE_SECONDS) / CRACK_SECONDS).clamp(0.0, 1.0);
        let wanted = (running * self.crack.len() as f32) as usize;
        if let Some(grid) = &self.grid {
            while self.opened < wanted {
                let spot = self.crack[self.opened];
                prefabs::surface_tile(&mut self.world, grid, &self.surfaces, spot, Surface::Lava, textures);
                self.opened += 1;
                for dy in -1isize..=1 {
                    for dx in -1isize..=1 {
                        let near = ((spot.0 as isize + dx) as usize, (spot.1 as isize + dy) as usize);
                        if self.banks.insert(near) {
                            if let Some((surface, row)) = self.surfaces.shore_at(grid, near) {
                                prefabs::shore_tile(&mut self.world, surface, near, row, textures);
                            }
                        }
                    }
                }
                // The carpet tears where the crack runs through it.
                let centre = (Vec2::new(spot.0 as f32, spot.1 as f32) + 0.5) * TILE_SIZE;
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
        }
        if at > ANGEL_FALLS {
            if let Some(angel) = self.angel.take() {
                let _ = self.world.despawn(angel);
                self.camera.add_trauma(0.6);
                let mut physics = crate::physics::Physics::new();
                let base = (self.altar / TILE_SIZE).floor();
                for (variant, (dx, dy)) in [(1, (-1.0, 0.5)), (2, (1.0, -0.5)), (2, (0.0, 1.5))] {
                    let spot = ((base.x + dx) as usize, (base.y + dy) as usize);
                    prefabs::decor(&mut self.world, &mut physics, Decor { kind: DecorKind::StatuePiece { variant }, tile: spot }, textures);
                }
            }
        }
        // Hold on the altar while it breaks, then back away down the nave with the crack.
        let back = ((at - ANGEL_FALLS - 0.6) / 5.5).clamp(0.0, 1.0);
        let eased = back * back * (3.0 - 2.0 * back);
        self.camera.snap_to(self.glide.1.lerp(self.retreat, eased));
    }

    /// The third shot: spiders, rats and bats climb out of the crack and scatter, and the temple
    /// sinks into the wound's red.
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
            self.crawlers.push((entity, from, to, when));
        }
        for &(entity, from, to, when) in &self.crawlers {
            let since = at - when;
            let climbed = (since / CLIMB_SECONDS).clamp(0.0, 1.0);
            let walked = ((since - CLIMB_SECONDS * 0.5) * SCURRY * TILE_SIZE / from.distance(to).max(1.0)).clamp(0.0, 1.0);
            if let Ok(mut position) = self.world.get::<&mut Position>(entity) {
                position.0 = from.lerp(to, walked);
            }
            if let Ok(mut color) = self.world.get::<&mut Color>(entity) {
                color.0.w = climbed;
            }
        }
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
        match shot {
            0 => {
                // A slow glide down the nave, easing in and out.
                let t = (into_shot / SHOTS[0].0).clamp(0.0, 1.0);
                let eased = t * t * (3.0 - 2.0 * t);
                self.camera.snap_to(self.glide.0.lerp(self.glide.1, eased));
            }
            1 => self.tear(assets, into_shot, dt),
            _ => self.crawl(assets, into_shot, dt),
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
