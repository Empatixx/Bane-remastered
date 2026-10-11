use std::time::{Duration, Instant};

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::game::Game;

const STEP: Duration = Duration::from_nanos(1_000_000_000 / 60);
const MAX_FRAME: Duration = Duration::from_millis(250);

#[derive(Debug)]
pub(crate) struct App {
    window: Option<Window>,
    game: Game,
    last_frame: Instant,
    accumulator: Duration,
}

impl App {
    pub(crate) fn new() -> Self {
        Self {
            window: None,
            game: Game::default(),
            last_frame: Instant::now(),
            accumulator: Duration::ZERO,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes().with_title("Bane");
        match event_loop.create_window(attributes) {
            Ok(window) => self.window = Some(window),
            Err(error) => {
                tracing::error!(%error, "cannot create the window");
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {}
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        let frame_time = (now - self.last_frame).min(MAX_FRAME);
        self.last_frame = now;
        self.accumulator += frame_time;

        while self.accumulator >= STEP {
            self.game.fixed_update();
            self.accumulator -= STEP;
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
