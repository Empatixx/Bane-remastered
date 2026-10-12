use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::{Assets, LOADING_SPINNER, LOADING_TITLE};
use crate::components::{Animation, Clip, Color, Glow, Layer, Position, Size, Sky, Sprite};
use crate::dungeon::Rng;
use crate::input::Input;
use crate::settings::Settings;
use crate::viewport::TARGET;

use super::menu_logo::LogoAnim;
use super::{Scene, SceneId, Transition};

const TITLE_SIZE: Vec2 = Vec2::new(285.0, 135.0);
const TITLE_CENTER: Vec2 = Vec2::new(TARGET.x / 2.0, 200.0);
const SPINNER_CENTER: Vec2 = Vec2::new(TARGET.x / 2.0, 365.0);
const GLOW_SIZE: Vec2 = Vec2::new(260.0, 260.0);
const SPARK_ORBIT: f32 = 44.0;
const SPARK_PERIOD: f32 = 0.045;
const SPARK_LIMIT: usize = 60;
const SPARK_LIFE: (f32, f32) = (0.9, 1.7);
const SPARK_SPEED: (f32, f32) = (18.0, 48.0);
const SPARK_SIZE: (f32, f32) = (2.0, 4.0);
const SPARK_SWIRL: f32 = 0.9;
const SPARK_COLORS: [Vec4; 3] = [
    Vec4::new(222.0 / 255.0, 112.0 / 255.0, 38.0 / 255.0, 1.0),
    Vec4::new(1.0, 0.78, 0.35, 1.0),
    Vec4::new(1.0, 1.0, 0.0, 1.0),
];
const SPARK_PEAK: f32 = 0.9;
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
    glow: Option<Entity>,
    title: Option<LogoAnim>,
    until_spark: f32,
    rng: Option<Rng>,
}

#[derive(Debug, Clone, Copy)]
struct Spark {
    velocity: Vec2,
    swirl: f32,
    age: f32,
    life: f32,
    color: Vec4,
}

fn between(rng: &mut Rng, low: f32, high: f32) -> f32 {
    low + (high - low) * rng.float()
}

fn emit_spark(world: &mut World, rng: &mut Rng) {
    let angle = between(rng, 0.0, std::f32::consts::TAU);
    let direction = Vec2::new(angle.cos(), angle.sin());
    let speed = between(rng, SPARK_SPEED.0, SPARK_SPEED.1);
    let size = between(rng, SPARK_SIZE.0, SPARK_SIZE.1);
    let color = SPARK_COLORS[rng.below(SPARK_COLORS.len())];
    world.spawn((
        Spark {
            velocity: direction * speed,
            swirl: if rng.below(2) == 0 {
                SPARK_SWIRL
            } else {
                -SPARK_SWIRL
            },
            age: 0.0,
            life: between(rng, SPARK_LIFE.0, SPARK_LIFE.1),
            color,
        },
        Position(SPINNER_CENTER + direction * SPARK_ORBIT),
        Size(Vec2::splat(size)),
        Color(Vec4::new(color.x, color.y, color.z, 0.0)),
        Layer(3),
    ));
}

fn run_sparks(world: &mut World, dt: f32) {
    let mut gone = Vec::new();
    for (entity, spark, position, color) in
        world.query_mut::<(Entity, &mut Spark, &mut Position, &mut Color)>()
    {
        spark.age += dt;
        let turn = spark.swirl * dt;
        spark.velocity = Vec2::from_angle(turn).rotate(spark.velocity);
        position.0 += spark.velocity * dt;
        let progress = (spark.age / spark.life).clamp(0.0, 1.0);
        color.0 = Vec4::new(
            spark.color.x,
            spark.color.y,
            spark.color.z,
            SPARK_PEAK * (progress * std::f32::consts::PI).sin(),
        );
        if spark.age >= spark.life {
            gone.push(entity);
        }
    }
    for entity in gone {
        let _ = world.despawn(entity);
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
        self.until_spark = 0.0;
        self.rng = Some(Rng::from_clock());
        self.finished_for = 0.0;
        let textures = &assets.textures;
        self.sky = Some(self.world.spawn((
            Position(TARGET / 2.0),
            Size(TARGET),
            Color(Vec4::ONE),
            Sky(0.0),
            Layer(-5),
        )));
        self.glow = Some(self.world.spawn((
            Position(SPINNER_CENTER),
            Size(GLOW_SIZE),
            Color(Vec4::ONE),
            Glow(0.0),
            Layer(1),
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
        if let Some(glow) = self.glow {
            if let Ok(mut state) = self.world.get::<&mut Glow>(glow) {
                state.0 = self.clock;
            }
        }
        self.until_spark -= dt;
        let count = self.world.query::<&Spark>().iter().count();
        if let (true, true, Some(rng)) = (
            self.until_spark <= 0.0,
            count < SPARK_LIMIT,
            self.rng.as_mut(),
        ) {
            self.until_spark = SPARK_PERIOD;
            emit_spark(&mut self.world, rng);
        }
        run_sparks(&mut self.world, dt);
        if let Some(title) = &mut self.title {
            title.tick(&mut self.world, Vec2::ZERO, dt);
        }
        if assets.is_loaded() {
            self.finished_for += dt;
            if self.finished_for >= HOLD_SECONDS {
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
    fn sparks_leave_the_ring_around_the_spinner_and_burn_out() {
        let (mut scene, assets) = scene();
        let mut settings = Settings::in_memory();
        for _ in 0..40 {
            scene.update(&Input::default(), 0.05, &assets, &mut settings);
        }
        let sparks: Vec<Vec2> = scene
            .world
            .query::<(&Spark, &Position)>()
            .iter()
            .map(|(_, position)| position.0)
            .collect();
        assert!(
            sparks.len() > 5 && sparks.len() <= SPARK_LIMIT,
            "{}",
            sparks.len()
        );
        assert!(sparks
            .iter()
            .all(|at| at.distance(SPINNER_CENTER) < SPARK_ORBIT + 120.0));

        for spark in scene.world.query_mut::<&mut Spark>() {
            spark.age = spark.life;
        }
        run_sparks(&mut scene.world, 0.0);
        assert_eq!(scene.world.query::<&Spark>().iter().count(), 0);
    }

    #[test]
    fn a_glow_disc_pulses_behind_the_spinner() {
        let (mut scene, assets) = scene();
        let mut settings = Settings::in_memory();
        scene.update(&Input::default(), 0.5, &assets, &mut settings);
        let time = scene
            .glow
            .and_then(|glow| scene.world.get::<&Glow>(glow).map(|g| g.0).ok())
            .unwrap_or(-1.0);
        assert!(time > 0.4);
    }

    #[test]
    fn the_title_is_half_the_size_of_the_original_picture() {
        assert_eq!(TITLE_SIZE, Vec2::new(285.0, 135.0));
    }
}
