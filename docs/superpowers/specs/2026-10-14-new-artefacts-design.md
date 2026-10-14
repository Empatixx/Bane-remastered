# New Artefacts — Design & Plan

Date: 2026-10-14
Status: abilities and icon designs approved by the user in conversation; numbers below are starting values to tune in play.

## Goal

Add eight one-use artefacts to the existing artefact system (one slot, `F` activates and consumes):

| Kind (`ArtefactKind`) | Icon file | Effect on activation |
|---|---|---|
| `HourglassOfAsh` | `hourglass_of_ash.tga` | For 6 s everything hostile (enemies, enemy bullets, their timers and animations) runs at 35 % speed; the player is unaffected. A full-screen WGSL post effect (ash desaturation + drifting ash specks + a ripple at the edges) ramps in and out. |
| `BloodChalice` | `blood_chalice.tga` | Heals 50 % of max health at once. For 10 s ("Bloodlust") every kill heals 1 HP. While active the player pulses red and pixel-art blood droplets rise around them. |
| `PhantomCloak` | `phantom_cloak.tga` | For 5 s the player becomes a translucent dark shadow (per-entity WGSL shader mode: dark tint, smoky wavering alpha). Enemies lose track of the player (they stop chasing/aiming) and touching enemies does no damage. The first shot fired ends the cloak and deals double damage. |
| `GraveLantern` | `grave_lantern.tga` | Summons 3 friendly ghosts for 12 s. Ghosts float through walls towards the nearest enemy and hit it for 2 damage (0.5 s cooldown per ghost), then fade out. |
| `MidasCoin` | `midas_coin.tga` | Every enemy bullet currently flying turns into a coin with a golden sparkle. For 6 s every enemy killed drops 2 extra coins. |
| `ThunderIdol` | `thunder_idol.tga` | Chain lightning: strikes the nearest enemy within 450 px and jumps to up to 5 more enemies, each within 300 px of the previous one, 6 damage per hit. Animated pixel-art bolts flicker for 0.35 s along each jump, with spark particles at every hit. |
| `InfinityQuiver` | `infinity_quiver.tga` | For 8 s firing never uses ammo and there is no reload. The HUD ammo pips and text turn gold and an ∞ sprite replaces the ammo numbers. |
| `BoneTurret` | `bone_turret.tga` | Places a bone turret at the player's feet for 12 s. It turns to the nearest enemy within 600 px and fires a bone shard every 0.35 s (2 damage). It rises out of the ground on spawn and crumbles when it expires. |

The icons are drawn by `tools/make_artefacts.py` (already designed and approved). Each effect's extra textures (particles, sprite sheets, the ∞ glyph) are drawn by small generator scripts in `tools/` in the same pixel-art style: 1 px dark outline, 2–3 shades per material, no anti-aliasing. Run them with `uv run --with pillow python -I tools/<script>.py`.

## Architecture

- **Foundation (Task 0).** `ArtefactKind` gains the 8 variants (`ALL` grows to 13). Icons go into `assets/textures/ingame/artefacts/` and are registered in `assets.rs` (`texture_keys!` and `GAME_TEXTURES`). `activate_artefact` only *requests* effects: each new arm inserts a component on the player:
  - timed: `TimeSlow { remaining }`, `Bloodlust { remaining }`, `Phantom { remaining }`, `InfiniteAmmo { remaining }`, `MidasTouch { remaining }`
  - one-shot requests: `SummonGhosts`, `CastLightning`, `PlaceTurret`, `ConvertBullets`, `HealPercent(f32)`

  One generic `tick_artefact_effects(world, dt)` counts the timed ones down and removes them. Each effect task then owns the systems that read its components, so tasks never edit each other's match arms.
- **WGSL validation.** Task 0 adds a unit test that parses and validates every `render/*.wgsl` with naga (it is already in the dependency tree through wgpu; add a dev-dependency on the same version with `wgsl-in` if needed). Shader mistakes then fail `cargo test` instead of only crashing at runtime.
- **Time scale.** `InGameScene::fixed_update` computes `hostile_dt = dt * scale`, with the scale coming from `TimeSlow` on the player. Enemy, boss and enemy-bullet systems get `hostile_dt`. Player, friendly bullet and pet systems keep `dt`. `simulate` scales the `Velocity` of `Enemy`/`EnemyBullet` bodies the way `Chilled` already does. `animate` scales enemy animation.
- **Post effect.** `SceneFx` gains `time_slow: f32`. `PostFx` ramps it, `PostParams`/`PostUniform` carry it (32 → 48 bytes), `post.wgsl` gets `fs_slow`, and `gpu.rs` adds the pass after the vignette (scene only, HUD untouched).
- **Shadow shader.** `Quad` gains a `shadowed(time)` builder writing a free `fx` slot. `quad.wgsl` gets a shadow branch (dark tint, alpha 0.35–0.6 wavering with a noise of position and time). `collect_draws` applies it to entities with `Phantom`, including `DodgeVisual`/`Afterimage` while the player is cloaked. `show_dodge_visual` must stop forcing alpha back to 1 on a cloaked player.
- **Invisibility.** `player_placement` (`systems/enemies.rs`), `golem::player_at`, `leapers::player_at`, `bosses::player_center`, `steer_homing` and `contact_damage` ignore a player with `Phantom`. Enemies stop where they are.
- **Friendly summons.** The ghost and the turret are new components with their own systems, modelled on the pet (`nearest_enemy` filters). Ghosts move by position without a physics body. Turret shards are ordinary player bullets (`prefabs::bullet`).

## Tasks (execution order)

0. **Foundation:** kinds, icons, request components, generic tick, naga WGSL test. Serial; everything else builds on it.
1. **Hourglass of Ash:** time scale + `fs_slow` post shader + its textures (none needed beyond the shader).
2. **Phantom Cloak:** shadow quad shader mode + invisibility + first-shot double damage.
3. **Infinity Quiver:** infinite ammo in `shooting.rs` + gold HUD + ∞ sprite.
4. **Blood Chalice + Midas Coin:** healing, kill hooks, bullet-to-coin conversion, blood droplets and gold sparkle textures.
5. **Grave Lantern + Bone Turret:** friendly summons, ghost and turret sprite sheets, bone shard texture.
6. **Thunder Idol:** chain lightning system, bolt and spark textures.

Tasks 1–6 run in parallel in separate worktrees after Task 0 and are merged one by one. Expected merge conflicts are limited to the lists in `assets.rs`, `systems/mod.rs` and the system calls in `scenes/ingame.rs`.

## Testing

- Each effect has unit tests at the system level (`prefabs::player`, `Textures::default()`, `Physics::new()`):
  - the timer expires
  - enemies don't see a phantom player
  - ammo is not consumed while `InfiniteAmmo` is present
  - the lightning hits at most 6 distinct enemies within range
  - ghosts and the turret damage enemies and despawn on time
  - enemy bullets become coins
  - healing is capped at max
- The naga test validates the WGSL.
- `RunRules`/fingerprint tests stay green.
- The visual check is manual: the user runs the game.
