use std::sync::Arc;
use std::time::{Duration, Instant};

use glam::Vec2;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::assets::Assets;
use crate::game::Game;
use crate::render::Gpu;
use crate::viewport::TARGET;

const STEP: Duration = Duration::from_nanos(1_000_000_000 / 60);
const MAX_FRAME: Duration = Duration::from_millis(250);

#[derive(Debug)]
pub(crate) struct App {
    window: Option<Arc<Window>>,
    gpu: Option<Gpu>,
    game: Game,
    last_frame: Instant,
    accumulator: Duration,
}

impl App {
    pub(crate) fn new() -> Self {
        Self {
            window: None,
            gpu: None,
            game: Game::new(Assets::default()),
            last_frame: Instant::now(),
            accumulator: Duration::ZERO,
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
        let started = Gpu::new(Arc::clone(&window)).and_then(|mut gpu| {
            let assets = Assets::load(&mut gpu)?;
            Ok((gpu, assets))
        });
        match started {
            Ok((gpu, assets)) => {
                self.game = Game::new(assets);
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
            WindowEvent::KeyboardInput { event, .. } => self.game.handle_key(&event),
            WindowEvent::MouseInput { state, button, .. } => self.game.handle_click(button, state),
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(window) = &self.window {
                    let size = window.inner_size();
                    if size.width > 0 && size.height > 0 {
                        let window_size = Vec2::new(size.width as f32, size.height as f32);
                        let pixels = Vec2::new(position.x as f32, position.y as f32);
                        self.game.set_mouse(pixels / window_size * TARGET);
                    }
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(gpu) = &mut self.gpu {
                    gpu.resize(size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(gpu) = &mut self.gpu {
                    gpu.render(
                        wgpu::Color {
                            r: 0.05,
                            g: 0.08,
                            b: 0.12,
                            a: 1.0,
                        },
                        &self.game.draws(),
                    );
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        let frame_time = (now - self.last_frame).min(MAX_FRAME);
        self.last_frame = now;
        self.accumulator += frame_time;

        while self.accumulator >= STEP {
            if self.game.fixed_update(STEP.as_secs_f32()).is_break() {
                event_loop.exit();
                return;
            }
            self.accumulator -= STEP;
        }
        if self.game.update(frame_time.as_secs_f32()).is_break() {
            event_loop.exit();
            return;
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
