# Codebase notes for the new artefacts (paths relative to rust/app/src)

## Artefacts
- `artefacts.rs:14`: `enum ArtefactKind`. `ALL: [Self; N]` is a hard-coded length. `texture()` is an exhaustive match onto `tex::ARTEFACTS_*`. `icon()` has a `_ => 32x32` wildcard and multiplies by `ICON_SCALE = 2.0`.
  - Tests: `there_are_five_distinct_artefacts`, `icons_are_drawn_at_double_size`.
  - `artefacts.rs` may import only `assets` and `glam`. The `architecture.rs` test forbids module cycles.
- `components/artefacts.rs`: `ArtefactSlot { current: Option<ArtefactKind> }` (on the player), `ArtefactDrop`, `ArtefactIcon`, `Shielded`, `ShieldAura`, `Berserk { remaining }`.
- `systems/artefacts.rs:19`: `activate_artefact(world, physics, input, bullet_texture: TextureId) -> Option<ArtefactKind>`.
  - It needs `Action::UseArtefact` (key F) and runs `slot.current.take()`.
  - The match over kinds is exhaustive. It is called in `scenes/ingame.rs:~236` inside `update` (frame dt).
  - The scene special-cases `RevealMap` via the returned kind.
- `tick_berserk(world, dt)` (`:72`) runs in `update`. Berserk swaps the gaze-wind particle texture at `ingame.rs:262-277`: `systems::spawn_gaze_wind(world, physics, texture, dt, clock + phase)`. Particles are `Dust` entities with a 3-frame, 16px sheet; see `systems/dust.rs`.
- `unused_artefact(&ArtefactSlot, roll)` picks from `ALL`, so new kinds join the drop pools automatically.

## Textures
- To add a texture, add a line to `texture_keys!` in `assets.rs:~46` (path relative to `/assets/textures/`) and add the key to `GAME_TEXTURES` (around `:280-345`).
  - Tests: `every_texture_key_is_loaded_by_one_of_the_lists`, `every_listed_texture_exists_and_decodes`, `texture_names_are_unique`.
- Sheets carry no metadata. Use `Sprite::cell(tex, col, row, columns, rows)` and `Animation::new(columns, rows, Clip { row, frames, frame_time, looping, flipped })`. Sheet constants go in `sheets.rs`.
- Pixel-art generators live in `/tools/*.py` (PIL). Run them with `uv run --with pillow python -I tools/x.py`. Icon style: 32x32, 1px dark outline (24,18,26), 2–3 shades per material. See `tools/make_artefacts.py`.

## Post-processing (WGSL)
- Shaders: `render/post.wgsl`, `render/quad.wgsl`, `render/lightmap.wgsl`, loaded via `include_wgsl!`. The `/assets/shaders/*.glsl` files are legacy and unused.
- Data flow:
  - `Scene::fx() -> SceneFx { paused, low_health }` (`postfx.rs:13`). Struct literals live in `ingame.rs:104`, `lobby.rs:136` and 2 tests in `postfx.rs`.
  - `game.rs:111` calls `postfx.update(dt, fx, hit)`, then `PostFx::params(time) -> PostParams` (`render/post.rs:4`).
  - That goes into `PostUniform` (`#[repr(C)] Pod`, 32 bytes: `texel[2], blur, darkness, vignette, time, time_based, padding`; there is a size test). The WGSL `struct Params` in `post.wgsl` mirrors it.
- `render/gpu.rs`:
  - `struct Post` (`:69`) holds pipelines `blit, vignette, blur_horizontal, blur_vertical`, each built by the `pipeline("fs_x")` closure in `Post::new` (`:693`) with `vs_fullscreen`.
  - `render` (`:532`) takes the direct path when no effect is active (`direct` at `:533`). Otherwise it renders the scene to target 0, then vignette, then HUD, then blur, then blit, then overlay. `current = 1 - current` ping-pongs between targets.
  - Put a new scene-only pass right after the vignette block (`~:648`).

## Per-entity rendering
- There is one quad pipeline. `Quad` (`render/quad.rs:20`): `center, size, color, uv, lit, angle, fx[4], local[4], outline[4]`.
  - Builders: `outlined`, `frosted`, `shined`, `liquid`, `underworld`, `map_room`, `rotated`, `lit`.
  - `quad.wgsl fs_main` (`:330`) selects a mode from `fx.w` (>3.5 liquid, >2.5 underworld, >0.5 map-room). Otherwise it chains `outlined`, `frosted` and `shined`.
- `systems/draw.rs::collect_draws` builds the quads from `Position, Size, Color, Sprite, Layer, Lit` plus `Liquid`, `Frozen`/`Chilled`, `Shine`, `Heading` and `Outline`, and skips `Hidden`.
- Alpha works through `Color.w` with alpha blending.
- **Catch:** `systems/dodge.rs::show_dodge_visual` (`:135-166`) forces the player's `Color.w` to 1.0 when not dodging and 0.0 while dodging. Tests at `dodge.rs:404-409` cover this. During a dodge, the `DodgeVisual` and `Afterimage` entities are drawn instead of the player.

## Simulation
- `app.rs:163` runs `fixed_update(1/60)` zero or more times, then `update(frame_dt)`.
- `InGameScene::fixed_update` passes `dt` to many systems: `steer_player_on`, `tick_dodges`, `tick_spawning`, `dash`, `update_flow`, the boss ticks (`tick_golem/hound/ogre/argus`), `tick_laser`, `tick_flames`, `tick_orbiters`, `tick_smashes`, `tick_eye_shooters`, `steer_homing`, `burn_fuses`, `tick_hatchers`, `tick_web_spitters`, `tick_leapers`, `tick_armageddon`, `slow_bullets_in_webs`, `simulate`, `burn_out_rounds`, `pet_bites`, `tick_arrow_traps`, `tick_poison`, `tick_frozen`, `tick_chilled`.
- `update` uses frame dt for `animate` (`scenes/machine.rs:71`), `tick_berserk`, `shoot` (player shooting and reload), `tick_casings`, `glide` and `tick_dust`.
- `systems/simulation.rs::simulate(world, physics, dt)` pushes `Velocity` into bodies and steps physics. It already multiplies velocity for `Chilled` (`CHILL_SLOWDOWN`).
- Enemy targeting choke points: `player_placement` (`systems/enemies.rs:20`, private), `golem::player_at` (`golem.rs:233`, also used by ogre, hound and argus), `leapers::player_at`, `bosses::player_center`, `steer_homing` (`systems/skills.rs`), and `contact_damage` (`systems/damage.rs:157`). `chase_player` with no target sets velocity to zero.
- Enemy bullets:
  - `EnemyBullet` marker. `prefabs::enemy_bullet(world, physics, ProjectileKind, position, direction, &Textures)`.
  - To remove one: `physics.detach(world, e)` (`physics/mod.rs:200`), then `world.despawn(e)`.
- Items: `prefabs::item(world, ItemKind::Coin, center, despawns: bool, velocity, &Textures)`. Coins are collected automatically (`collect_items`, `use_item`).
- Enemy deaths: `enemy_drops` (`systems/items.rs:~182`) acts when the death animation of an `Enemy + Expiring` entity finishes. `resolve_deaths` / `reap_dead` insert `Expiring`.
- Damage to enemies: `systems::damage::deal(world, Damage { target, amount, source: DamageSource::{Bullet,Trap,Bite,Explosion,Poison} }) -> bool`. It must not target the player.
  - Damage to the player goes through `hurt_player(world, player, n, sounds)`.
- Pet as a model for summons:
  - `prefabs::pet` with `Pet` (`components/pet.rs`).
  - `systems/pet.rs`: `pick_pet_targets`, `nearest_enemy` (filters `Enemy`, `Health > 0`, not `Expiring`, not `Spawning`), `plan_pet_paths`, `steer_pets`, `pet_bites` (uses `damage::deal`).
- Player bullets: `prefabs::bullet(world, physics, position, velocity, Bullet::plain(dmg), texture)`. `Bullet` is in `components/combat.rs:28`.
  - `Friendly` is NOT a team flag; it only marks poison pools.
- Ammo:
  - `systems/shooting.rs::shoot` decrements `weapon.clip -= 1` at `:127` (burst) and `:168`. An empty clip returns at `:156`.
  - `Weapon { clip, ammo, reloading, ... }`. Reload goes through `start_reload` / `advance`.
- HUD: `systems/hud.rs::update_weapon_hud` (`:54`) and `show_weapon` (`:84`).
  - `AmmoText` is `"{clip}/{ammo}"`. `AmmoPip` sprites use `PIP_FULL`/`PIP_EMPTY`.
  - Spawned in `prefabs/hud.rs::weapon_hud` (`:210`). Callers are `ingame.rs:286` and `lobby.rs:350`.
  - The font has no ∞ glyph, so use a sprite.
- Healing: `Health { current, max }` on the player. Clamp to max. `SoundEffect::Heal` exists.

## Tests
- Fixtures: `prefabs::player(&mut world, &mut Physics::new(), Vec2, &Textures::default())`, `Input::default()` + `input.press(KeyCode::KeyF)` (see `press_f` in `systems/artefacts.rs` tests), `prefabs::ready_enemy(world, physics, EnemyKind, Vec2)`, `prefabs::enemy_bullet(..., &Textures::default())`, and `systems::simulate`.
- `postfx.rs` has tests, `render/post.rs` has a uniform size test, and `render/quad.rs` has builder tests.
- There are no WGSL tests; Task 0 adds naga validation.
- Workspace lints: `unreachable_pub` (use `pub(crate)`), `missing_debug_implementations`, `clippy::unwrap_used`.
- Gate: `cargo test` and `cargo clippy --all-targets -- -D warnings` must be clean. `cargo fmt --check` already shows unrelated diffs, so format only your own hunks.
