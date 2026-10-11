mod app;
mod artefacts;
mod assets;
mod audio;
mod camera;
mod components;
mod dungeon;
mod enemies;
mod game;
mod geometry;
mod guns;
mod input;
mod items;
mod navigation;
mod objects;
mod perf;
mod physics;
mod postfx;
mod prefabs;
mod profile;
mod props;
mod render;
mod scenes;
mod settings;
mod systems;
mod tilemap;
mod upgrades;
mod viewport;

use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<(), winit::error::EventLoopError> {
    let (writer, _log_guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt().with_writer(writer).init();

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app::App::new())
}
