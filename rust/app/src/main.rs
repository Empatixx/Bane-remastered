mod app;
mod assets;
mod camera;
mod components;
mod game;
mod geometry;
mod input;
mod physics;
mod prefabs;
mod props;
mod render;
mod scenes;
mod systems;
mod tilemap;
mod viewport;

use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<(), winit::error::EventLoopError> {
    tracing_subscriber::fmt::init();

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app::App::new())
}
