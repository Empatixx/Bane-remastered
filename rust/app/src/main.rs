mod app;
mod artefacts;
#[cfg(test)]
mod architecture;
mod assets;
mod audio;
mod camera;
mod components;
mod controls;
mod dungeon;
mod enemies;
mod flow;
mod game;
mod geometry;
mod guns;
mod input;
mod items;
mod navigation;
mod objects;
mod palette;
mod perf;
mod physics;
mod postfx;
mod prefabs;
mod profile;
#[allow(dead_code, unused_imports)]
mod progress;
mod props;
mod render;
mod rng;
mod rules;
mod scenes;
mod settings;
mod storage;
mod systems;
mod tilemap;
mod upgrades;
mod viewport;
mod weapon_tree;
mod weapon_upgrades;

use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<(), winit::error::EventLoopError> {
    let (writer, _log_guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt().with_writer(writer).init();

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app::App::new())
}
