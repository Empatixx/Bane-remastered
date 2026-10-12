use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Assets, LOADING_SPINNER, LOADING_TITLE};
use crate::components::{Animation, Clip, Color, Layer, Position, Shine, Size, Sky, Sprite};
use crate::input::Input;
use crate::render::TextureId;
use crate::settings::Settings;
use crate::viewport::TARGET;

use super::menu_logo::LogoAnim;
use super::{Scene, SceneId, Transition};

const TITLE_SIZE: Vec2 = Vec2::new(570.0, 270.0);
const TITLE_CENTER: Vec2 = Vec2::new(TARGET.x / 2.0, 183.0);
const SPINNER_CENTER: Vec2 = Vec2::new(TARGET.x / 2.0, 395.0);
const SPINNER_SIZE: Vec2 = Vec2::new(68.0, 72.0);
const SPINNER_FRAMES: usize = 8;
const SPINNER_FRAME_TIME: f32 = 0.1;
const LINE_WIDTH: f32 = 240.0;
const LINE_HEIGHT: f32 = 2.0;
const LINE_Y: f32 = 450.0;
const LINE_TRACK: Vec4 = Vec4::new(1.0, 1.0, 1.0, 0.12);
const LINE_FILL: Vec4 = Vec4::new(222.0 / 255.0, 112.0 / 255.0, 38.0 / 255.0, 1.0);
const CATCH_UP: f32 = 6.0;
const SHINE_SECONDS: f32 = 1.4;
const MIN_SHINE_WIDTH: f32 = 8.0;
const HOLD_SECONDS: f32 = 0.5;
const FULL: f32 = 0.98;

#[derive(Default)]
pub(crate) struct LoadingScene {
    world: World,
    clock: f32,
    shown: f32,
    finished_for: f32,
    sky: Option<Entity>,
    title: Option<LogoAnim>,
    fill: Option<Entity>,
}

impl LoadingScene {
    fn left_edge() -> f32 {
        TARGET.x / 2.0 - LINE_WIDTH / 2.0
    }

    fn set_line(&mut self, shine: f32) {
        let width = (LINE_WIDTH * self.shown.clamp(0.0, 1.0)).max(0.0);
        if let Some(fill) = self.fill {
            if let Ok((position, size, glow)) = self
                .world
                .query_one_mut::<(&mut Position, &mut Size, &mut Shine)>(fill)
            {
                position.0 = Vec2::new(Self::left_edge() + width / 2.0, LINE_Y);
                size.0 = Vec2::new(width, LINE_HEIGHT);
                glow.0 = if width > MIN_SHINE_WIDTH { shine } else { 0.0 };
            }
        }
    }
}

impl Scene for LoadingScene {
    fn world(&self) -> &World {
        &self.world
    }

    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn enter(&mut self, assets: &Assets) {
        self.clock = 0.0;
        self.shown = 0.0;
        self.finished_for = 0.0;
        let textures = &assets.textures;
        self.sky = Some(self.world.spawn((
            Position(TARGET / 2.0),
            Size(TARGET),
            Color(Vec4::ONE),
            Sky(0.0),
            Layer(-5),
        )));
        self.title = Some(LogoAnim::build(
            &mut self.world,
            textures.get(LOADING_TITLE),
            TITLE_CENTER,
            TITLE_SIZE,
        ));
        let spin = Clip {
            row: 0,
            frames: SPINNER_FRAMES,
            frame_time: SPINNER_FRAME_TIME,
            looping: true,
            flipped: false,
        };
        self.world.spawn((
            Position(SPINNER_CENTER),
            Size(SPINNER_SIZE),
            Color(Vec4::ONE),
            Sprite::cell(textures.get(LOADING_SPINNER), 0, 0, SPINNER_FRAMES, 1),
            Animation::new(SPINNER_FRAMES, 1, spin),
            Layer(2),
        ));
        self.world.spawn((
            Position(Vec2::new(TARGET.x / 2.0, LINE_Y)),
            Size(Vec2::new(LINE_WIDTH, LINE_HEIGHT)),
            Color(LINE_TRACK),
            Layer(1),
        ));
        self.fill = Some(self.world.spawn((
            Position(Vec2::new(Self::left_edge(), LINE_Y)),
            Size(Vec2::new(0.0, LINE_HEIGHT)),
            Color(LINE_FILL),
            Sprite::whole(TextureId::WHITE),
            Shine(0.0),
            Layer(3),
        )));
    }

    fn update(
        &mut self,
        _input: &Input,
        dt: f32,
        assets: &Assets,
        _settings: &mut Settings,
    ) -> Transition {
        self.clock += dt;
        self.shown += (assets.progress() - self.shown) * (dt * CATCH_UP).min(1.0);
        if let Some(sky) = self.sky {
            if let Ok(mut state) = self.world.get::<&mut Sky>(sky) {
                state.0 = self.clock;
            }
        }
        if let Some(title) = &mut self.title {
            title.tick(&mut self.world, Vec2::ZERO, dt);
        }
        let shine = ((self.clock % SHINE_SECONDS) / SHINE_SECONDS).max(0.001);
        self.set_line(shine);
        if assets.is_loaded() {
            self.finished_for += dt;
            if self.shown >= FULL && self.finished_for >= HOLD_SECONDS {
                return Transition::Switch(SceneId::Menu);
            }
        }
        Transition::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scene() -> (LoadingScene, Assets) {
        let assets = Assets::new().expect("assets");
        let mut scene = LoadingScene::default();
        scene.enter(&assets);
        (scene, assets)
    }

    fn fill_width(scene: &LoadingScene) -> f32 {
        scene
            .fill
            .and_then(|fill| scene.world.get::<&Size>(fill).map(|s| s.0.x).ok())
            .unwrap_or(-1.0)
    }

    #[test]
    fn the_line_starts_empty_and_grows_towards_the_real_progress() {
        let (mut scene, mut assets) = scene();
        let mut settings = Settings::in_memory();
        scene.update(&Input::default(), 0.016, &assets, &mut settings);
        assert!(fill_width(&scene) < 1.0);

        assets.set_progress(0.5);
        for _ in 0..120 {
            scene.update(&Input::default(), 0.016, &assets, &mut settings);
        }
        let half = fill_width(&scene);
        assert!((half - LINE_WIDTH * 0.5).abs() < 3.0, "{half}");
    }

    #[test]
    fn the_scene_waits_for_the_assets_and_then_a_short_moment_with_a_full_line() {
        let (mut scene, mut assets) = scene();
        let mut settings = Settings::in_memory();
        let step = |scene: &mut LoadingScene, assets: &Assets, settings: &mut Settings, dt| {
            scene.update(&Input::default(), dt, assets, settings)
        };
        assert_eq!(
            step(&mut scene, &assets, &mut settings, 1.0),
            Transition::None
        );

        assets.mark_loaded();
        assert_eq!(
            step(&mut scene, &assets, &mut settings, 0.1),
            Transition::None
        );

        let mut result = Transition::None;
        for _ in 0..100 {
            result = step(&mut scene, &assets, &mut settings, 0.05);
            if result != Transition::None {
                break;
            }
        }
        assert_eq!(result, Transition::Switch(SceneId::Menu));
        assert!(fill_width(&scene) > LINE_WIDTH * FULL - 1.0);
    }

    #[test]
    fn the_sky_clock_advances_so_the_shader_effect_moves() {
        let (mut scene, assets) = scene();
        let mut settings = Settings::in_memory();
        for _ in 0..30 {
            scene.update(&Input::default(), 0.1, &assets, &mut settings);
        }
        let time = scene
            .sky
            .and_then(|sky| scene.world.get::<&Sky>(sky).map(|s| s.0).ok())
            .unwrap_or(0.0);
        assert!(time > 2.5, "{time}");
    }

    #[test]
    fn the_old_spinner_is_back_and_animated() {
        let (scene, _assets) = scene();
        let spinners = scene.world.query::<(&Animation, &Sprite)>().iter().count();
        assert_eq!(spinners, 1);
    }
}
