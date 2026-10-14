# Task 6 report: Thunder Idol

Worktree /Users/jirka/IdeaProjects/bane-art-6, branch art-t6, commit b0208561.

## Built
- `systems/lightning.rs` (new):
  - `chain_targets(origin, &[(Entity, Vec2)]) -> Vec<(Entity, Vec2)>` is a pure function. It strikes the nearest enemy within 450 px of the origin, then the nearest unhit enemy within 300 px of the previous target, up to 6 hits.
  - `cast_lightning(world, &Textures, seed) -> usize` consumes `CastLightning`. It collects enemies with the same filters as the pet's `nearest_enemy` (`Enemy`, `Health > 0`, not `Expiring`, not `Spawning`). Each struck enemy takes 6 damage through `damage::deal` with `DamageSource::Explosion`. I did not add a `Lightning` source, to keep `damage.rs` untouched for merges. The function spawns the visuals and returns the hit count.
  - `tick_lightning(world, dt)` flickers and fades the bolts and flies the sparks with drag. Both despawn when spent.
- Visuals:
  - **Bolts.** Each jump is split into pieces of about 96 px (a 32 px cell at 3x), so the pixel density stays even instead of one quad stretched over up to 450 px. Each piece is a `Bolt { angle }` entity rotated along the jump. Every 0.05 s it picks a new random frame out of 4, and also randomly flips vertically, which gives 8 looks. A bolt lives 0.35 s with alpha `sqrt(1 - t)` times a brightness flicker between 0.75 and 1. The cell ends sit on the centre line, so the pieces join seamlessly.
  - **Impact.** Each hit gets a 6-frame burst at 0.06 s per frame, drawn at 96 px. It works like `frost_burst` (`Expiring` + `Animation`), plus a white-yellow `Light` with a radius of 220 and intensity 1.6, faded by `LightFade`.
  - **Sparks.** Each hit throws out 6–10 sparks, spread around the circle, at 140–360 px/s with drag `exp(-6 dt)`. Each lives 0.28–0.5 s, steps through the 3-frame sheet as it ages and fades with `1 - p²`.
  - **Shake.** The scene adds `LIGHTNING_SHAKE = 0.45` trauma when at least one enemy is hit. Bolts, bursts and sparks are unlit, so they glow in the dark.
- Wiring:
  - `draw.rs`: the main query gets `.without::<&Bolt>()`, plus a small loop that draws the rotated bolts.
  - `ingame.rs`: `cast_lightning` runs right after `activate_artefact`, and `tick_lightning` runs after `tick_dust`.
  - `systems/mod.rs`: one `mod` line and one `use` line.
  - `assets.rs`: 3 textures.
  - `#[allow(dead_code)]` is removed from `CastLightning`.

## Textures
`tools/make_lightning_fx.py` (PIL, no anti-aliasing) writes these to `assets/textures/ingame/artefacts/`:
- `lightning_bolt.tga`: 4 × 32×16. A white core with a pale yellow ring, a blue ring and a violet haze, plus small forks.
- `lightning_burst.tga`: 6 × 32×32. Flash, then a star of arcs, then a ring of arcs, then broken arcs and embers, then fading dots.
- `lightning_spark.tga`: 3 × 8×8. A star, a smaller star, then an ember.

The keys are `tex::ARTEFACTS_LIGHTNING_{BOLT,BURST,SPARK}`. There is no ink outline, because an outline reads wrong on light. The violet haze plays that role instead.

The preview is `/private/tmp/claude-501/-Users-jirka-IdeaProjects-Bane-refactor-rust/1111d5e4-569e-437c-9ed7-9a60743473a1/scratchpad/lightning_preview.png`. It shows the 4 bolt frames, a 4-piece chain, the 6 burst frames and the 3 sparks, at 8x. I went through 4 iterations.

## Tests
`cargo test`: 729 passed, 0 failed, 1 ignored. `cargo clippy --all-targets -D warnings`: clean.

The 11 new tests:
- nearest first, then nearest to each target
- never the same enemy twice
- strike range
- jump range
- the cap of 6
- damage applied and the request consumed
- nothing happens without a request
- dying and spawning enemies are skipped
- bolts, burst lights and sparks spawn and all expire
- a long jump is split into pieces that span it end to end at the right angle
- sparks slow down

## Concerns
- There is no line-of-sight check, so the chain can jump through walls within range.
- There is no thunder sound, because no `SoundEffect` fits and I didn't add audio.
- Damage, bolts and bursts all happen at once. There is no staggered propagation.
- `cargo fmt` here sorts imports in the 2021 style, but the codebase uses 2024 ordering (`{Textures, tex}`). I kept the codebase style in `lightning.rs`, and my other hunks are fmt-clean.
- The visuals were checked only in the preview, not in the running game. The 3x pixel scale is a guess based on `frost_burst`, which is drawn at about 3.5x.
