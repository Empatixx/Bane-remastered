use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::Assets;
use crate::components::{Color, Layer, Position, Shine, Size, Sky, Sprite};
use crate::input::Input;
use crate::render::TextureId;
use crate::settings::Settings;
use crate::viewport::TARGET;

use super::menu_logo::LogoAnim;
use super::{Scene, SceneId, Transition};

const LOGO_SIZE: Vec2 = Vec2::new(304.0, 144.0);
const LOGO_CENTER_Y: f32 = 190.0;
const BAR_SIZE: Vec2 = Vec2::new(360.0, 14.0);
const BAR_CENTER_Y: f32 = 390.0;
const BAR_RIM: f32 = 6.0;
const RIM_COLOR: Vec4 = Vec4::new(225.0 / 255.0, 152.0 / 255.0, 116.0 / 255.0, 1.0);
const BACK_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.75);
const FILL_COLOR: Vec4 = Vec4::new(222.0 / 255.0, 112.0 / 255.0, 38.0 / 255.0, 1.0);
const HIGHLIGHT_COLOR: Vec4 = Vec4::new(1.0, 0.78, 0.35, 1.0);
const HIGHLIGHT_HEIGHT: f32 = 4.0;
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
    logo: Option<LogoAnim>,
    fill: Option<Entity>,
    highlight: Option<Entity>,
}

fn rect(world: &mut World, center: Vec2, size: Vec2, color: Vec4, layer: i32) -> Entity {
    world.spawn((Position(center), Size(size), Color(color), Layer(layer)))
}

impl LoadingScene {
    fn left_edge() -> f32 {
        TARGET.x / 2.0 - BAR_SIZE.x / 2.0
    }

    fn set_bar(&mut self, shine: f32) {
        let width = (BAR_SIZE.x * self.shown.clamp(0.0, 1.0)).max(0.0);
        let center_x = Self::left_edge() + width / 2.0;
        if let Some(fill) = self.fill {
            if let Ok((position, size, glow)) = self
                .world
                .query_one_mut::<(&mut Position, &mut Size, &mut Shine)>(fill)
            {
                position.0 = Vec2::new(center_x, BAR_CENTER_Y);
                size.0 = Vec2::new(width, BAR_SIZE.y);
                glow.0 = if width > MIN_SHINE_WIDTH { shine } else { 0.0 };
            }
        }
        if let Some(highlight) = self.highlight {
            if let Ok((position, size)) = self
                .world
                .query_one_mut::<(&mut Position, &mut Size)>(highlight)
            {
                position.0 = Vec2::new(
                    center_x,
                    BAR_CENTER_Y - BAR_SIZE.y / 2.0 + HIGHLIGHT_HEIGHT / 2.0,
                );
                size.0 = Vec2::new(width, HIGHLIGHT_HEIGHT);
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
        self.sky = Some(self.world.spawn((
            Position(TARGET / 2.0),
            Size(TARGET),
            Color(Vec4::ONE),
            Sky(0.0),
            Layer(-5),
        )));
        self.logo = Some(LogoAnim::build(
            &mut self.world,
            &assets.textures,
            Vec2::new(TARGET.x / 2.0, LOGO_CENTER_Y),
            LOGO_SIZE,
        ));
        let bar = Vec2::new(TARGET.x / 2.0, BAR_CENTER_Y);
        rect(
            &mut self.world,
            bar,
            BAR_SIZE + Vec2::splat(BAR_RIM),
            RIM_COLOR,
            1,
        );
        rect(&mut self.world, bar, BAR_SIZE, BACK_COLOR, 2);
        self.fill = Some(self.world.spawn((
            Position(bar),
            Size(Vec2::new(0.0, BAR_SIZE.y)),
            Color(FILL_COLOR),
            Sprite::whole(TextureId::WHITE),
            Shine(0.0),
            Layer(3),
        )));
        self.highlight = Some(rect(
            &mut self.world,
            bar,
            Vec2::new(0.0, HIGHLIGHT_HEIGHT),
            HIGHLIGHT_COLOR,
            4,
        ));
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
        if let Some(logo) = &mut self.logo {
            logo.tick(&mut self.world, Vec2::ZERO, dt);
        }
        let shine = ((self.clock % SHINE_SECONDS) / SHINE_SECONDS).max(0.001);
        self.set_bar(shine);
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
    fn the_bar_starts_empty_and_grows_towards_the_real_progress() {
        let (mut scene, mut assets) = scene();
        let mut settings = Settings::in_memory();
        scene.update(&Input::default(), 0.016, &assets, &mut settings);
        assert!(fill_width(&scene) < 1.0);

        assets.set_progress(0.5);
        for _ in 0..120 {
            scene.update(&Input::default(), 0.016, &assets, &mut settings);
        }
        let half = fill_width(&scene);
        assert!((half - BAR_SIZE.x * 0.5).abs() < 4.0, "{half}");
    }

    #[test]
    fn the_scene_waits_for_the_assets_and_then_a_short_moment_with_a_full_bar() {
        let (mut scene, mut assets) = scene();
        let mut settings = Settings::in_memory();
        let flow = |scene: &mut LoadingScene, assets: &Assets, settings: &mut Settings, dt| {
            scene.update(&Input::default(), dt, assets, settings)
        };
        assert_eq!(
            flow(&mut scene, &assets, &mut settings, 1.0),
            Transition::None
        );

        assets.mark_loaded();
        assert_eq!(
            flow(&mut scene, &assets, &mut settings, 0.1),
            Transition::None
        );

        let mut result = Transition::None;
        for _ in 0..100 {
            result = flow(&mut scene, &assets, &mut settings, 0.05);
            if result != Transition::None {
                break;
            }
        }
        assert_eq!(result, Transition::Switch(SceneId::Menu));
        assert!(fill_width(&scene) > BAR_SIZE.x * FULL - 1.0);
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
}
