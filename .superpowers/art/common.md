# Common rules for the artefact effect tasks

Read these first:
- Design: `docs/superpowers/specs/2026-10-14-new-artefacts-design.md`, in your worktree.
- Codebase notes, with file:line pointers: `/Users/jirka/IdeaProjects/Bane_refactor-rust/.superpowers/art/codebase-notes.md`.
- Foundation report: `/Users/jirka/IdeaProjects/Bane_refactor-rust/.superpowers/art/task-0-report.md`. It gives the exact names of the components and constants that are already in place.

The foundation already works like this. `activate_artefact` inserts a request or timer component on the player:
- timers: `TimeSlow`, `Bloodlust`, `Phantom`, `InfiniteAmmo`, `MidasTouch`, each `{ remaining }`
- one-shot requests: `SummonGhosts`, `CastLightning`, `PlaceTurret`, `ConvertBullets`, `HealPercent(f32)`

`tick_artefact_effects` counts the timers down. Your systems react to these components:
- Consume a one-shot request by removing it once handled.
- Remove the `#[allow(dead_code)]` from the components and constants you now use.

## Rules
- Work ONLY in your own worktree and branch (given in your prompt). Run cargo from `<worktree>/rust`. Other tasks run in parallel in other worktrees.
- Keep edits to the shared files minimal and local: `assets.rs` texture lists, `systems/mod.rs`, `scenes/ingame.rs` system calls. That keeps merges easy. Don't reformat or reorder unrelated lines.
- Pixel art:
  - Write your own generator script `tools/make_<name>_fx.py`, in the style of `tools/make_artefacts.py` (PIL; 1px dark outline (24,18,26) where it suits; 2–3 shades per material; no anti-aliasing; sprite sheets as a grid of equal cells).
  - Output goes to `assets/textures/ingame/artefacts/`. Run it with `uv run --with pillow python -I tools/<script>.py` from the worktree root.
  - Look at your art: render an 8x-scaled preview PNG into `/private/tmp/claude-501/-Users-jirka-IdeaProjects-Bane-refactor-rust/1111d5e4-569e-437c-9ed7-9a60743473a1/scratchpad/` and Read it. Iterate until it looks good. The user explicitly wants "pěkné" (nice) pixel-art animations.
  - Register textures in `assets.rs` (`texture_keys!` plus `GAME_TEXTURES`). Put sheet layout constants next to the code that uses them, or in `sheets.rs`.
- Shaders are WGSL. The naga validation tests in `render/mod.rs` must pass. Bind layouts and uniform structs must match on the Rust and WGSL sides, including 16-byte uniform size alignment.
- Tests: write unit tests for the behaviour (TDD where practical) with the fixtures listed in the notes.
- Gate before committing: `cargo test` and `cargo clippy --all-targets -- -D warnings` both clean. Keep only your own hunks rustfmt-clean.
- Lints: `unreachable_pub` (use `pub(crate)`), `missing_debug_implementations`, `clippy::unwrap_used`. Minimal comments; test names are sentences.
- End each commit message with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.
- Do all the work yourself; spawn no subagents. You run in the background, so make reasonable design calls and note them in the report.
- Report to `/Users/jirka/IdeaProjects/Bane_refactor-rust/.superpowers/art/task-<N>-report.md`. Cover what you built, the numbers, the textures (with the preview path), tests and output, and concerns. Then reply with ONLY (under 12 lines): Status, commits, a test summary, concerns.
