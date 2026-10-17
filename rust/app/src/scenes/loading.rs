use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Assets, LOADING_SPINNER, LOADING_TITLE};
use crate::components::{Animation, Clip, Color, Layer, Position, Size, Sky, Sprite};
use crate::input::Input;
use crate::settings::Settings;
use crate::viewport::TARGET;

use super::menu_logo::LogoAnim;
use super::{Scene, SceneId, Transition};

pub(super) const TITLE_SIZE: Vec2 = Vec2::new(285.0, 135.0);
pub(super) const TITLE_CENTER: Vec2 = Vec2::new(TARGET.x / 2.0, 200.0);
const SPINNER_CENTER: Vec2 = Vec2::new(TARGET.x / 2.0, 365.0);
const SPINNER_SIZE: Vec2 = Vec2::new(68.0, 72.0);
const SPINNER_FRAMES: usize = 8;
const SPINNER_FRAME_TIME: f32 = 0.1;
const HOLD_SECONDS: f32 = 0.5;

#[derive(Default)]
pub(crate) struct LoadingScene {
    world: World,
    clock: f32,
    finished_for: f32,
    sky: Option<Entity>,
    title: Option<LogoAnim>,
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
    }

    fn update(
        &mut self,
        _input: &Input,
        dt: f32,
        assets: &Assets,
        _settings: &mut Settings,
    ) -> Transition {
        self.clock += dt;
        if let Some(sky) = self.sky {
            if let Ok(mut state) = self.world.get::<&mut Sky>(sky) {
                state.0 = self.clock;
            }
        }
        if let Some(title) = &mut self.title {
            title.tick(&mut self.world, Vec2::ZERO, dt);
        }
        if assets.is_loaded() {
            self.finished_for += dt;
            if self.finished_for >= HOLD_SECONDS {
                return Transition::Switch(first_scene());
            }
        }
        Transition::None
    }
}

/// `BANE_START=lobby` or `BANE_START=dungeon` skips straight there, for working on the game.
fn first_scene() -> SceneId {
    match std::env::var("BANE_START").as_deref() {
        Ok("lobby") => SceneId::Lobby,
        Ok("dungeon") => SceneId::InGame,
        Ok("training") => SceneId::Training,
        Ok("intro") => SceneId::Intro,
        Ok("epilogue") => SceneId::Epilogue,
        _ => SceneId::Menu,
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

    #[test]
    fn the_scene_waits_for_the_assets_and_then_a_short_moment() {
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

    #[test]
    fn the_title_is_half_the_size_of_the_original_picture() {
        assert_eq!(TITLE_SIZE, Vec2::new(285.0, 135.0));
    }
}
