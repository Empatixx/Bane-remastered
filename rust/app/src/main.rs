mod app;
mod artefacts;
#[cfg(test)]
mod architecture;
mod assets;
mod audio;
mod camera;
mod clip;
mod components;
mod controls;
mod dungeon;
mod enemies;
mod flow;
mod game;
mod geometry;
mod guns;
mod headless;
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
mod progress;
mod props;
mod render;
mod rng;
mod rules;
mod scenes;
mod settings;
mod sheets;
mod storage;
mod systems;
mod tilemap;
mod upgrades;
mod viewport;
mod weapon_tree;
mod weapon_upgrades;

use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(options) = headless::Options::parse(&args) {
        let (writer, _log_guard) = tracing_appender::non_blocking(std::io::stderr());
        tracing_subscriber::fmt().with_writer(writer).init();
        return headless::run(&options?);
    }
    let (writer, _log_guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt().with_writer(writer).init();

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app::App::new())?;
    Ok(())
}
