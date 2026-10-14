# Task 2 report: Phantom Cloak

Worktree /Users/jirka/IdeaProjects/bane-art-2, branch art-t2, commit 52e6ba29.

## Built
- **Shadow shader**
  - In `render/quad.rs`: `MODE_SHADOW = 5.0` goes in `fx.w`, and `Quad::shadowed(time)` writes `fx = [time, 0, 0, 5]` plus a cool rim outline `[0.55, 0.62, 1.0, 0.35]`. There is also a `#[cfg(test)] is_shadowed()`.
  - `quad.wgsl` gets `fn shadow`, which `fs_main` selects when `fx.w > 4.5`. The liquid branch is now `3.5 < w < 4.5`. The look:
    - The tint is rgb(0.12, 0.10, 0.20), and its brightness is modulated by a swirl noise.
    - Alpha is mix(0.3, 0.6) of two value-noise layers. The noise runs over the atlas texel cell (pixel-quantised) and the time.
    - Edge texels are those with a transparent 4-neighbour. They flicker out at 10 Hz (25–73% chance, more at corners) and the rest run at 60% alpha. Interior texels sometimes dim.
    - The rim reuses `outlined()`. The final alpha is multiplied by `in.color.a`.
- **collect_draws**
  - It reads the player's `Phantom.remaining` and uses it as the shader clock, so the signature is unchanged.
  - It shadows any sprite that is `Player`, `DodgeVisual` or `Afterimage` while the player is cloaked. These are matched through an `Option<Or<..>>` nested inside the query tuple.
- **show_dodge_visual** is unchanged on purpose. The shader multiplies by `Color.w`, so forcing alpha to 1 or 0 composes correctly (1 shows the shadow, 0 hides the player during the roll). The dodge tests are untouched and green.
- **Invisibility**
  - `player_placement`, `leapers::player_at`, `bosses::player_center` and `steer_homing` filter `.without::<Phantom>()`.
  - `golem::player_at` filters Phantom. A new `golem::player_body` keeps the unfiltered lookup.
- **contact_damage** skips non-trap sources while the player is cloaked. Spikes and geysers still hurt.
- **First shot**
  - In `shoot`, a trigger pull while `Phantom` is present marks the `Shot` as `ambush`, and `Phantom` is removed after the loop.
  - The ambush shot deals `AMBUSH_MULTIPLIER = 2` times damage, applied after perks and crits.
  - An empty-clip click does not break the cloak.

## Design calls
- Golem damage paths keep hitting a cloaked player through `player_body`: rock landing, `draw_beam` hit test, and magma tiles. Only targeting is blinded. Otherwise the cloak would have made beams and lava harmless.
- Traps still hurt through the cloak, because the design names only "touching enemies".
- With a burst gun, only the first round gets doubled damage. The follow-up rounds fire after the cloak has already been removed.
- Homing eye-shooter shots fired while the player is cloaked have zero velocity, so they hover in place until they expire.
- The player's DropShadow is still drawn under the shadow sprite.

## Textures
None needed. To check the shader I emulated it in Python on the player's idle frame. The preview shows the original sprite, then 5 time samples:
`/private/tmp/claude-501/-Users-jirka-IdeaProjects-Bane-refactor-rust/1111d5e4-569e-437c-9ed7-9a60743473a1/scratchpad/t2/shadow_preview.png`. It reads as a dark violet smoky silhouette with ragged, flickering edges and a faint blue rim.

## Tests
`cargo test`: 726 passed, 0 failed, 1 ignored. `cargo clippy --all-targets -D warnings` is clean. My hunks are rustfmt-clean; the remaining fmt diffs are import-order noise that was already there.

New tests:
- a_shadowed_quad_uses_the_shadow_mode_the_clock_and_a_cool_rim
- a_cloaked_player_and_its_dodge_pictures_are_drawn_as_shadows
- enemies_lose_track_of_a_cloaked_player_and_stand_still (covers player_placement, golem::player_at/player_body, chase velocity zero)
- a_leaper_cannot_pick_a_cloaked_player
- bosses_cannot_find_a_cloaked_player
- homing_shots_stop_steering_towards_a_cloaked_player
- touching_an_enemy_does_not_hurt_a_cloaked_player_but_spikes_still_do
- the_first_shot_from_the_cloak_ends_it_and_hits_twice_as_hard

`the_quad_shader_is_valid` (naga) passes.

## Concerns
- The shader has not been looked at in the running game, only through the Python emulation. In-game, the shadow ignores the lightmap, so in very dark rooms it may look slightly lighter than the surroundings.
- `fx.w` mode 5 is now taken. A later task adding a quad mode should use 6 or higher, and its liquid-style `>` check must stay ordered.
