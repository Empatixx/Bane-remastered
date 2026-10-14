# Task 1 report: Hourglass of Ash

Worktree /Users/jirka/IdeaProjects/bane-art-1, branch art-t1, commit 70e8f29a.

## Time slow
- New `systems/time_slow.rs`: `time_scale(world) -> f32`. It returns 1.0 without `TimeSlow` on the player and `TIME_SLOW_SCALE` (0.35) while the player has it. The value eases over 0.3 s at both ends, derived from `remaining` (no extra state).
- `simulate`: the `Velocity` of `Enemy`/`EnemyBullet` bodies is multiplied by `time_scale`, which stacks with `CHILL_SLOWDOWN`. The signature is unchanged.
- `animate`: `Enemy`/`EnemyBullet` animations advance by `dt * time_scale`. The signature and call site are unchanged.
- `InGameScene::fixed_update` computes `hostile_dt = dt * systems::time_scale(&self.world)`.
  - **hostile_dt:** tick_spawning, dash, tick_golem, tick_rock_falls, tick_hound, tick_magma_spread, tick_magma, tick_ogre, tick_argus, tick_laser, tick_flames, tick_orbiters, tick_smashes, tick_eye_shooters, steer_homing, burn_fuses (enemy-bullet lifetimes), tick_hatchers, tick_web_spitters, tick_leapers, tick_armageddon, watch_progress, tick_arrow_traps.
    - watch_progress has to use hostile_dt. Its expected distance is `|Velocity| * elapsed`, so with real dt slowed enemies would be judged stuck.
  - **real dt:** steer_player_on, tick_dodges, update_flow, pets, slow_bullets_in_webs, simulate (it scales internally), pet_bites, trap_hits_enemies, tick_poison, tick_frozen, tick_chilled, ripple_on_water.
- In `update`, `update_rooms` (the wave delay between waves) gets the scaled dt as well.
- `InGameScene::time_slow()` feeds `SceneFx.time_slow` (1.0 or 0.0).

## Post effect fs_slow
- `SceneFx.time_slow: f32`. `PostFx` ramps it: +1/0.4 s in, -1/0.6 s out.
- `PostParams` gains `time_slow` and `clock` (`time % 600`). `clock` is needed because `time` is wrapped to a 3 s pulse period, which would make the ash jump.
- `PostUniform` is now 48 bytes: `..., time_based, time_slow, clock, padding[3]`. WGSL `Params` has three scalar paddings, so both sides are 48 bytes. The size test is updated.
- `post.wgsl` `fs_slow`, scaled by smoothstep(time_slow):
  - concentric ripple (sin(r·38 − 3.5t), amplitude pulsing at 1.8 rad/s, 0.0045 uv, fading to zero at the center) plus a ±10 % brightness on the crests
  - 70 % warm ash/sepia desaturation
  - multiplicative dark-amber vignette (0.5, 0.3, 0.12) from r 0.3 to 0.95
  - two layers of pixel ash specks: an integer hash on a 480-cell and a 240-cell grid, drifting up-left with a sway, flickering, in light or dark ash
- `gpu.rs`: a `slow` pipeline, a scene-only pass right after the vignette (before the HUD), and the `direct` check includes `time_slow_active()`. The naga entry list includes `fs_slow`.
- Preview: a numpy port of the shader, applied to the menu bg, with the original on top and the effect below: `/private/tmp/claude-501/-Users-jirka-IdeaProjects-Bane-refactor-rust/1111d5e4-569e-437c-9ed7-9a60743473a1/scratchpad/t1/slow_preview.png`. The effect is clearly visible. There are no new textures.

## Tests
`cargo test`: 724 passed, 0 failed, 1 ignored. `cargo clippy --all-targets -D warnings` is clean.

New tests:
- time_runs_normally_without_the_hourglass
- the_hourglass_slows_time_to_its_scale_once_eased_in
- the_slowdown_eases_in_and_out
- the_hourglass_slows_enemies_and_their_bullets_but_not_the_player (simulate)
- the_hourglass_slows_enemy_animations_but_not_the_rest
- the_time_slow_effect_ramps_in_quickly_and_out_more_slowly

Updated: the uniform size test (48), effects_are_active…, post shader naga test (fs_slow).

## Concerns
- Using the hourglass again while it is active resets `remaining`. That re-runs the 0.3 s ease-in, so time pops briefly back towards 1.0.
- Animation scaling covers only `Enemy`/`EnemyBullet`. Boss effect entities (flames, laser, magma, falling rocks) keep animating at full speed even though their logic is slowed.
- Arrow traps count as hostile and are slowed. That is my call: they are environmental, not enemy-made.
- Poison pools tick on real dt because one system serves both sides.
- `ingame.rs` hunks: about 20 dt→hostile_dt edits. rustfmt split four long calls into multi-line calls, which may conflict with other tasks' system-call edits nearby.
- `render/post.rs` has a pre-existing rustfmt diff in its test (assert! layout). I left it alone and matched the existing style.
