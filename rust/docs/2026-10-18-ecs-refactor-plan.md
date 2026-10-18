# ECS refactor plan (2026-10-18)

Goal: make the Rust port follow ECS where the 2026-10-18 audit found it does not — implicit
cross-system communication, game state outside the World, duplicated boss scaffolding, an
undocumented 150-call schedule. Behaviour-preserving refactor: the game must play the same.

Spec: the audit summary in this file's "Findings" section (no separate spec doc). Rulings made
without a separate spec are provisional.

## Findings this plan answers

1. Deaths are detected implicitly: `blood.rs:95`, `items.rs:187`, `midas.rs:72` look for
   `Expiring` + `animation.finished()`; correctness depends on these running in `update()` before
   `remove_expired`. `resolve_deaths` (`systems/enemies.rs:548`) calls six modules' handlers.
2. `damage::deal` (`systems/damage.rs:62-100`) applies HP *and* runs side effects (skills hit
   counting, stats, flash, damage number) synchronously.
3. Screen shake is derived from the sound queue: `impact_shake` (`scenes/ingame.rs:38`) used by
   `feel_impacts` (`ingame.rs:~779`).
4. 175 player-lookup sites, ≥9 local `player_at`-style helpers that disagree (golem.rs:241 skips
   dead/Phantom players, prison.rs:123 does not).
5. `Loadout` (`components/combat.rs:335-520`) and `Weapon::advance/start_reload/count_shot`
   (`combat.rs:219-330`) hold gameplay rules inside components.
6. Run-level state lives on `InGameScene` (33 fields: rngs, `kills_without_drop`, `slay_in`…),
   as marker-entity singletons (`Ambience`, `Atmosphere`, `StirSeed`), no resource container.
7. Boss scaffolding is copy-pasted across golem/ogre/argus/mimic/hound: same brain fields
   (`components/skills.rs:115-220`), near-identical enrage block, `RAGE_TINT` ×4; `golem.rs` is a
   de-facto shared boss library; the same `without::<Spawning/Expiring/Frozen>` filter ×8.
8. The schedule is ~81 + 71 sequential calls in `ingame.rs:176-771` with ~25 repeats of the same
   `physics, textures, sounds, navigation, hostile_dt` tuple; lobby copies a subset by hand
   (`lobby.rs:521-631`); ~300 lines of `BANE_AT` debug spawn logic sit in `ingame.rs` (~870-1180).
9. Teleports/velocity writes bypass the single physics owner `simulation.rs:simulate` in ~13
   files; `finish_spawning` (`prefabs/actors.rs:597-609`) is a state transition inside a prefab.

## Global Constraints

- Work in `/Users/jirka/IdeaProjects/Bane_refactor-ecs` on branch `ecs-refactor`. Rust crate in
  `rust/`. hecs 0.11 (no built-in resources/events), rapier2d, wgpu.
- Every task ends with: `cd rust && cargo test --quiet` all green (baseline 984 passed,
  2 ignored — the count may only grow or shrink by moved/merged tests, explained in the report),
  `cargo clippy --all-targets --quiet` with no warnings beyond the 4 baseline ones
  (is_multiple_of, assertions_on_constants, unwrap on Result, very complex type).
- Behaviour-preserving. Where a behaviour change is unavoidable or fixes a hidden bug, name it in
  the report.
- Match surrounding code style: `pub(crate)`, doc comments in plain English describing what
  things are, small free-function systems over `&mut World`, tests in a trailing
  `#[cfg(test)] mod tests`.
- `architecture.rs` module-cycle guard must stay green (`KNOWN_CYCLES = []`).
- ECS event style chosen by the user discussion: entity lifecycle events are **one-tick tag
  components on the entity itself** (e.g. `Died`); free-standing gameplay events are **event
  entities** (e.g. a spawned `Hit`); non-entity signals (camera trauma, sounds) live in a
  **resource** struct. Every event tag/entity has exactly one documented cleanup point.
- Commit per logical step with conventional-commit messages ending with:
  `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>` and
  `Claude-Session: https://claude.ai/code/session_01EvRtUUEHWH7GpYYUm2qFFw`.
- Do not touch the `docs/` wiki or `assets/`.

## Tasks

### Task 1: Port the hygiene and `Hostile` tick context from `arch-review-rust`

Branch `arch-review-rust` (base 820103dc, 70 commits behind master) holds:
- `6850ede6` fix(app): cache boss debug env reads (OnceLock), share one per-entity PRNG
  (`rng::roll` replacing golem's own xorshift), drop dead volume arm.
- `1fabda46` refactor(bosses): pass a `Hostile<'a>` tick context to the enemy-side fixed systems
  (`systems/hostile.rs`; built twice in `fixed_update` because of physics borrows).
- `8e2adbf6` (Death events as `Vec<Death>` from `remove_expired`) — **do NOT port**; Task 2
  replaces it with the tag-component design.
Cherry-pick (or re-apply by hand where master moved) `6850ede6` and `1fabda46` onto
`ecs-refactor`. Any boss/enemy fixed system added on master since 820103dc that takes the same
`physics, textures, sounds, navigation, dt` tuple must also take `Hostile`. Optionally port the two
review docs (`rust/docs/2026-10-14-*.md`) as-is.

### Task 2: Death lifecycle as tag components

Replace implicit death detection with explicit one-tick tags:
- `Died` (components/death.rs or combat.rs): inserted by `reap_dead` on the fixed tick an enemy's
  HP reaches 0, alongside today's `Expiring`. Carries whatever the readers need that may not be
  on the entity (e.g. killer/source if available; otherwise a unit struct).
- `Gone`: inserted by `remove_expired` when an `Expiring` entity's animation finished, instead of
  despawning immediately. A single `despawn_gone` system despawns `Gone` entities at one documented
  point, after every reader has run in both loops (pick the point; document why).
- `Died` is removed at one documented point after all death readers ran.
- `resolve_deaths` stops calling other modules' handlers directly: `skills::death_bursts/
  death_clouds/death_heals`, `frost::death_splits`, `lava::death_ashes`,
  `bosses::clean_up_after_boss` become independent systems that query `With<Died>` and are listed
  in the scene schedule (or a `death_phase` fn in systems) in the same order.
- `blood.rs`, `items.rs` (both queries), `midas.rs` read `Gone` (or `Died`, whichever matches
  today's timing — keep the timing the player sees today) instead of `Expiring + finished()`.
- Physics `detach` on death stays where it is unless trivially centralised.
- Add a test that pins the cross-loop ordering: a kill in `fixed_update` produces exactly one drop
  / one blood burst / one midas effect regardless of how many `update()` calls happen in between
  (0, 1, 3), and the entity is despawned exactly once.

### Task 3: `Hit` event entities and `Fx` resource (trauma)

- `damage::deal` keeps applying HP immediately (other code reads HP the same tick) but its side
  effects (skills hit counting, stats recording, hit flash, floating damage number) move out: it
  spawns a `Hit { target, amount, source/kind, at }` event entity. Small systems read `Hit`
  entities and perform each side effect; one `clear_hits` despawns them at a documented point.
  Keep the existing `Damage` struct if it is the natural payload.
- Introduce a resource struct (e.g. `systems::Fx { trauma: f32 }` or a broader `Signals`) owned by
  the scene and passed explicitly. Systems that cause shake add trauma there
  (`LIGHTNING_SHAKE` path in ingame.rs:299 too). Delete `impact_shake`/`feel_impacts`; map each
  sound effect that used to shake to an explicit `trauma +=` at the place that queues that sound,
  same magnitude. The camera reads and drains trauma once per frame.
- Tests: a `deal` produces one `Hit` and the side-effect systems react once; shake magnitude for a
  former sound-derived impact is unchanged.

### Task 4: One player accessor

- New `systems/player.rs` (or `systems::player` module) with a small, documented API, e.g.
  `player(world) -> Option<Entity>`, `player_at(world) -> Option<Vec2>`,
  `living_player_at(world)` (skips dead and `Phantom`), `player_body(...)` if needed.
- Replace the local helpers: `golem::player_at/player_body`, `lava::player_at`,
  `prison::player_at`, `leapers::player_at`, `training::player`, `bosses::player_center`,
  `rooms::player_position`, `enemies::player_placement`, `ingame::player_tile`, and convert the
  most common inline `query::<…>().with::<&Player>()` single-player lookups where it reads cleaner.
- Each replaced helper keeps its current semantic (living vs any). Where two helpers used for the
  same purpose disagree (golem vs prison), report it; keep current behaviour unless clearly a bug,
  and if you change it, test it.

### Task 5: Move Loadout/Weapon rules out of the components

- Data stays in `Loadout`/`Weapon` (components/combat.rs). Gameplay rules move to free functions
  in `systems/loadout.rs` (exists): `Loadout::add/add_ammo/wants_ammo/neediest/restrict/
  apply_perks`, `Weapon::advance/start_reload/count_shot`. Pure accessors/derived values
  (`spec`, `max_ammo`, `fullness`, constructors) may stay as methods.
- Move their unit tests with them. No behaviour change.

### Task 6: `RunState` and scene resources

- Create a resource struct (e.g. `systems::RunState`) owned by `InGameScene` that holds run-level
  state now spread as scene fields: the three game RNGs (map/loot/combat), `kills_without_drop`,
  `slay_in`, and other gameplay (non-render, non-UI) fields — decide per field and list them in
  the report.
- Marker-entity singletons `Ambience`, `Atmosphere`, `StirSeed` (spawned lazily, found with
  `.iter().next()`) become fields of a resource struct instead of entities, if they carry no
  entity-only data (sprites, positions) — otherwise leave them and explain.
- Player-carried run data (`RunStats`, `PlayerPerks`, `Wallet`) may stay on the player entity
  this time (HUD and carry.rs depend on it); note it as a follow-up in the report.

### Task 7: Shared boss kit

- New `systems/boss_kit.rs`: move the shared helpers other bosses import from `golem.rs`
  (`play`, `unit`, `spawn_rock`, …) there; `golem.rs` keeps only golem behaviour.
- A shared `BossBrain` component (stage, attack, last, timer, cooldown, seed, idle — whatever is
  truly common) used by Hound/Ogre/Argus/Golem/Mimic boss components, which keep only their own
  fields.
- `Enrage { at, speed_mul/speed, tint }` component + `Enraged` tag + one `tick_enrage` system
  replacing the per-boss enrage blocks (golem.rs:299-314, ogre.rs:174-188, argus.rs:~275,
  hound.rs:225-232) — same thresholds, sounds, speed changes and tint per boss; `RAGE_TINT`
  defined once (or per-boss values carried in `Enrage`).
- A shared query filter for "active" hostiles (not Spawning/Expiring/Frozen) used by the eight
  systems that paste it.
- Per-boss `Cmd`/`step()` stay (pure, tested).

### Task 8: Named schedule phases

- `systems/schedule.rs` with a `Tick`-style context bundling `world`, `physics`, `assets/
  textures`, `sounds`, `navigation`, the Task 3/6 resources, `dt`/`hostile_dt`.
- Split `InGameScene::update`/`fixed_update` into named phase functions (e.g. player, hostile,
  physics, combat, deaths, fx, hud) called in order; document the ordering contract at the top of
  the module (what must precede what and why).
- Lobby calls the shared phase functions it needs instead of its hand-copied list
  (`lobby.rs:521-631`); keep lobby-only differences (`steer_player_on`) explicit.
- Move `build()`/`BANE_AT` debug spawn-point logic out of `ingame.rs` into a dungeon/debug module.
- No system call may be dropped or reordered relative to today except where the report explains
  it; add a test or assertion for any ordering the Task 2 test does not already cover.

### Task 9: Single physics owner for teleports; spawning transition

- A `Teleport(Vec2)` (and if needed `SetVelocity`) request component processed by `simulate`
  (or a system right before it) replaces direct rapier position writes in argus, pet, shooting,
  training, cart `move_to`, and the `set_velocity` calls in waterworks, hazards, effects,
  training that get overwritten anyway. Keep semantics identical.
- Move `finish_spawning` (prefabs/actors.rs:597-609) into `tick_spawning` in systems.
