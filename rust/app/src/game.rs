use std::ops::ControlFlow;

use glam::Vec2;
use winit::event::{ElementState, KeyEvent, MouseButton};

use crate::assets::Assets;
use crate::components::Sprite;
use crate::input::Input;
use crate::render::{Draw, Quad};
use crate::scenes::{Cursor, Scenes};

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
    time: f32,
}

impl Game {
    pub(crate) fn new(assets: Assets) -> Self {
        Self {
            input: Input::default(),
            scenes: Scenes::new(&assets),
            assets,
            time: 0.0,
        }
    }

    pub(crate) fn assets_mut(&mut self) -> &mut Assets {
        &mut self.assets
    }

    pub(crate) fn handle_key(&mut self, event: &KeyEvent) {
        self.input.handle_key(event);
    }

    pub(crate) fn handle_click(&mut self, button: MouseButton, state: ElementState) {
        self.input.handle_click(button, state);
    }

    pub(crate) fn set_mouse(&mut self, position: Vec2) {
        self.input.set_mouse(position);
    }

    pub(crate) fn fixed_update(&mut self, dt: f32) -> ControlFlow<()> {
        self.scenes.fixed_update(&self.input, dt, &self.assets)
    }

    pub(crate) fn update(&mut self, dt: f32) -> ControlFlow<()> {
        self.time += dt;
        let flow = self.scenes.update(&self.input, dt, &self.assets);
        self.input.end_frame();
        flow
    }

    pub(crate) fn draws(&self) -> Vec<Draw> {
        let mut draws = self.scenes.draws(&self.assets.fonts);
        if self.assets.is_loaded() {
            draws.push(self.cursor_draw());
        }
        draws
    }

    fn cursor_draw(&self) -> Draw {
        let textures = &self.assets.textures;
        let (name, hotspot, uv) = match self.scenes.cursor() {
            Cursor::Default => (
                "ingame/cursor.tga",
                ARROW_HOTSPOT,
                Sprite::cell_uv(0, 0, 1, 1),
            ),
            Cursor::Crosshair => (
                "ingame/crosshair.tga",
                AIM_HOTSPOT,
                Sprite::cell_uv(0, 0, 1, 1),
            ),
            Cursor::Reload => {
                let frame = (self.time / RELOAD_FRAME_TIME) as usize % RELOAD_FRAMES;
                (
                    "ingame/reload_cursor.tga",
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
