# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Bane is a 2D top-down roguelike shooter in Rust, in the workspace `rust/` (the old Java version is
gone from `master`; it lives in the git history). See `README.md` for what the game contains and
`rust/docs/` for the ECS architecture notes.

## Commands

```bash
cd rust
cargo run --release          # play
cargo test --quiet           # tests
cargo clippy --all-targets   # lints (a few baseline warnings)

# headless screenshot, see rust/README.md for all flags
BANE_START=dungeon BANE_THEME=lava cargo run --release -- --shot x.png --seconds 4
```

## Layout

- `rust/app/src/` — the game: `scenes/` (state machine), `systems/` (run per frame by `schedule.rs`),
  `components/`, `prefabs/` (entity spawning), `dungeon/` (generation, themes), `physics/`
  (rapier2d wrapper), `assets/`.
- `rust/assets/` — textures, audio, shaders, fonts, room templates.
- `docs/` — the Bane wiki (Fumadocs, English), content in `docs/content/<topic>/`.
- `tools/` — Python asset generators.
- `media/` — pictures and GIFs used by `README.md`.

## Architecture

Fixed 60 UPS simulation with a variable render rate. State lives in a `hecs` `World` plus plain
resource structs (`RunState` and friends); systems are functions over the world, ordered in
`systems/schedule.rs`. `Velocity` is the source of truth for movement: `simulate` writes it onto the
physics body before every step. Dying and despawning go through the `Died`/`Gone` tags and
`Physics::despawn`, not a plain `world.despawn`.
