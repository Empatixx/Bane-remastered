use std::ops::ControlFlow;

use glam::Vec2;
use winit::event::{ElementState, KeyEvent, MouseButton};

use crate::assets::tex;
use crate::assets::{Assets, Loader};
use crate::audio::Audio;
use crate::components::Sprite;
use crate::controls::{self, Action};
use crate::input::Input;
use crate::postfx::PostFx;
use crate::profile::SharedProfile;
use crate::render::{Draw, Frame, Gpu, Quad};
use crate::scenes::{Cursor, Scenes};
use crate::settings::{Change, Settings};

const CURSOR_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const ARROW_HOTSPOT: Vec2 = Vec2::new(6.0, 3.5);
const AIM_HOTSPOT: Vec2 = Vec2::new(17.0, 17.0);
const RELOAD_FRAMES: usize = 3;
const RELOAD_FRAME_TIME: f32 = 0.1;

#[derive(Debug)]
pub(crate) struct Game {
    input: Input,
    scenes: Scenes,
    assets: Assets,
    settings: Settings,
    audio: Audio,
    postfx: PostFx,
    time: f32,
}

impl Game {
    pub(crate) fn new(assets: Assets, settings: Settings, profile: SharedProfile) -> Self {
        Self {
            input: Input::default(),
            scenes: Scenes::new(&assets, profile),
            assets,
            settings,
            audio: Audio::new(),
            postfx: PostFx::default(),
            time: 0.0,
        }
    }

    pub(crate) fn take_setting_changes(&mut self) -> Vec<Change> {
        let mut window_changes = Vec::new();
        for change in self.settings.take_changes() {
            match change {
                Change::Volume {
                    overall,
                    music,
                    effects,
                } => self.audio.set_volume(overall, music, effects),
                Change::Controls(bindings) => self.input.set_bindings(bindings),
                other => window_changes.push(other),
            }
        }
        window_changes
    }

    pub(crate) fn load_step(
        &mut self,
        gpu: &mut Gpu,
        loader: &mut Loader,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loader.step(gpu, &mut self.assets, &mut self.audio)
    }

    fn play_scene_sounds(&mut self) {
        for command in self.scenes.take_sounds() {
            self.audio.play(command);
        }
    }

    pub(crate) fn handle_key(&mut self, event: &KeyEvent) {
        self.input.handle_key(event);
    }

    pub(crate) fn handle_click(&mut self, button: MouseButton, state: ElementState) {
        self.input.handle_click(button, state);
    }

    pub(crate) fn handle_scroll(&mut self, lines: f32) {
        self.input.handle_scroll(lines);
    }

    pub(crate) fn set_mouse(&mut self, position: Vec2) {
        self.input.set_mouse(position);
    }

    pub(crate) fn set_blend(&mut self, blend: f32) {
        self.scenes.set_blend(blend);
    }

    pub(crate) fn fixed_update(&mut self, dt: f32) -> ControlFlow<()> {
        let flow = self.scenes.fixed_update(&self.input, dt, &self.assets);
        self.play_scene_sounds();
        flow
    }

    pub(crate) fn update(&mut self, dt: f32) -> ControlFlow<()> {
        self.time += dt;
        let flow = self
            .scenes
            .update(&self.input, dt, &self.assets, &mut self.settings);
        self.play_scene_sounds();
        let hit = self.scenes.take_hit();
        self.postfx.update(dt, self.scenes.fx(), hit);
        self.input.end_frame();
        flow
    }

    pub(crate) fn frame(&self) -> Frame {
        let mut layers = self.scenes.draws(
            &self.assets.fonts,
            controls::key_label(self.input.bindings().key(Action::Interact)),
        );
        if self.assets.is_loaded() {
            layers.overlay.push(self.cursor_draw());
        }
        Frame {
            layers,
            post: self.postfx.params(self.time),
            camera: self.scenes.camera(),
            lights: self.scenes.lights(),
        }
    }

    fn cursor_draw(&self) -> Draw {
        let textures = &self.assets.textures;
        let (name, hotspot, uv) = match self.scenes.cursor() {
            Cursor::Default => (tex::CURSOR, ARROW_HOTSPOT, Sprite::cell_uv(0, 0, 1, 1)),
            Cursor::Crosshair => (tex::CROSSHAIR, AIM_HOTSPOT, Sprite::cell_uv(0, 0, 1, 1)),
            Cursor::Reload => {
                let frame = (self.time / RELOAD_FRAME_TIME) as usize % RELOAD_FRAMES;
                (
                    tex::RELOAD_CURSOR,
                    AIM_HOTSPOT,
                    Sprite::cell_uv(frame, 0, RELOAD_FRAMES, 1),
                )
            }
        };
        let top_left = self.input.mouse() - hotspot;
        Draw {
            texture: textures.get(name),
            quad: Quad::with_uv(
                top_left + CURSOR_SIZE / 2.0,
                CURSOR_SIZE,
                glam::Vec4::ONE,
                uv,
            ),
        }
    }
}
