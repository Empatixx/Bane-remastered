use std::sync::Arc;
use std::time::{Duration, Instant};

use glam::Vec2;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Fullscreen, Window, WindowId};

use crate::assets::{Assets, Loader};
use crate::game::Game;
use crate::perf::PerfCounter;
use crate::profile::SharedProfile;
use crate::render::Gpu;
use crate::settings::{Change, Settings};
use crate::storage;
use crate::viewport::TARGET;

const SETTINGS_FILE: &str = "settings.json";
const PROFILE_FILE: &str = "profile.json";
const STEP: Duration = Duration::from_nanos(1_000_000_000 / 60);
const MAX_FRAME: Duration = Duration::from_millis(250);

#[derive(Debug)]
pub(crate) struct App {
    window: Option<Arc<Window>>,
    gpu: Option<Gpu>,
    game: Option<Game>,
    loader: Option<Loader>,
    last_frame: Instant,
    accumulator: Duration,
    perf: PerfCounter,
}

impl App {
    pub(crate) fn new() -> Self {
        Self {
            window: None,
            gpu: None,
            game: None,
            loader: None,
            last_frame: Instant::now(),
            accumulator: Duration::ZERO,
            perf: PerfCounter::default(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes().with_title("Bane");
        let window = match event_loop.create_window(attributes) {
            Ok(window) => Arc::new(window),
            Err(error) => {
                tracing::error!(%error, "cannot create the window");
                event_loop.exit();
                return;
            }
        };
        window.set_cursor_visible(false);
        let started = Gpu::new(Arc::clone(&window)).and_then(|mut gpu| {
            let mut assets = Assets::new()?;
            let loader = Loader::start(&mut gpu, &mut assets)?;
            Ok((gpu, assets, loader))
        });
        match started {
            Ok((mut gpu, assets, loader)) => {
                let mut settings = Settings::load(&storage::data_file(SETTINGS_FILE));
                settings.set_resolutions(monitor_resolutions(&window));
                settings.startup_changes();
                let profile = SharedProfile::load(&storage::data_file(PROFILE_FILE));
                let mut game = Game::new(assets, settings, profile);
                apply_changes(&window, &mut gpu, game.take_setting_changes());
                self.game = Some(game);
                self.loader = Some(loader);
                self.gpu = Some(gpu);
                self.window = Some(window);
            }
            Err(error) => {
                tracing::error!(%error, "cannot start the graphics");
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                if let Some(game) = &mut self.game {
                    game.handle_key(&event);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(game) = &mut self.game {
                    game.handle_click(button, state);
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let Some(game) = &mut self.game {
                    let lines = match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, y) => y,
                        winit::event::MouseScrollDelta::PixelDelta(position) => position.y as f32,
                    };
                    game.handle_scroll(lines);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let (Some(window), Some(game)) = (&self.window, &mut self.game) {
                    let size = window.inner_size();
                    if size.width > 0 && size.height > 0 {
                        let window_size = Vec2::new(size.width as f32, size.height as f32);
                        let pixels = Vec2::new(position.x as f32, position.y as f32);
                        game.set_mouse(pixels / window_size * TARGET);
                    }
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(gpu) = &mut self.gpu {
                    gpu.resize(size);
                }
            }
            WindowEvent::RedrawRequested => {
                self.perf.frame();
                if let (Some(gpu), Some(game)) = (&mut self.gpu, &self.game) {
                    gpu.render(wgpu::Color::BLACK, &game.frame());
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let (Some(game), Some(gpu)) = (&mut self.game, &mut self.gpu) else {
            return;
        };
        if let Some(loader) = &mut self.loader {
            if let Err(error) = game.load_step(gpu, loader) {
                tracing::error!(%error, "cannot load the assets");
                event_loop.exit();
                return;
            }
            if loader.is_done() {
                self.loader = None;
            }
        }
        if let Some(window) = &self.window {
            apply_changes(window, gpu, game.take_setting_changes());
        }
        let now = Instant::now();
        let frame_time = (now - self.last_frame).min(MAX_FRAME);
        self.last_frame = now;
        self.accumulator += frame_time;
        if let Some((fps, ups)) = self.perf.advance(frame_time) {
            tracing::info!(fps, ups, "performance");
        }

        while self.accumulator >= STEP {
            if game.fixed_update(STEP.as_secs_f32()).is_break() {
                event_loop.exit();
                return;
            }
            self.accumulator -= STEP;
            self.perf.update();
        }
        game.set_blend(self.accumulator.as_secs_f32() / STEP.as_secs_f32());
        if game.update(frame_time.as_secs_f32()).is_break() {
            event_loop.exit();
            return;
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn monitor_resolutions(window: &Window) -> Vec<(u32, u32)> {
    window
        .current_monitor()
        .map(|monitor| {
            monitor
                .video_modes()
                .map(|mode| (mode.size().width, mode.size().height))
                .collect()
        })
        .unwrap_or_default()
}

fn apply_changes(window: &Window, gpu: &mut Gpu, changes: Vec<Change>) {
    for change in changes {
        match change {
            Change::Fullscreen(fullscreen) => {
                window.set_fullscreen(fullscreen.then_some(Fullscreen::Borderless(None)));
            }
            Change::Resolution(width, height) => {
                let _ = window.request_inner_size(PhysicalSize::new(width, height));
            }
            Change::Vsync(vsync) => gpu.set_vsync(vsync),
            Change::Volume {
                overall,
                music,
                effects,
            } => tracing::info!(
                overall,
                music,
                effects,
                "volume changed (audio is not implemented yet)"
            ),
            Change::Controls(_) => {}
        }
    }
}
