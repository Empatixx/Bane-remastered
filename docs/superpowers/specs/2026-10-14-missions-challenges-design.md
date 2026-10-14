# Missions & Challenges — Design

Date: 2026-10-14
Status: approved in conversation, awaiting spec review

## Goal

Turn the lobby quest board (`rust/app/src/scenes/quest_board.rs`, today hard-coded
placeholder data) into two working systems:

- **Missions** (board tab "Tasks"): rotating goals whose progress accumulates across
  runs and pays talents when claimed.
- **Challenges** (board tab "Challenges"): a whole-run rule change picked on the board;
  reaching floor N with it active pays talents once. The lobby portal changes colour while
  a challenge is selected, and the selection can be cancelled.

The systems must be **decoupled from gameplay**: gameplay code never refers to missions or
challenges. It only reports what happened (`RunRecord`) and reads how the run is tuned
(`RunModifiers`).

## Player-facing behaviour

### Missions

- The board always shows **3 missions**, drawn at random from a pool of templates.
- A template is a goal kind plus a range of target values and a reward. Kinds:
  - kill N enemies (any)
  - kill N enemies of a given kind
  - kill boss X
  - reach floor N (in one run)
  - collect N gold (in one run)
  - collect N gold (in total)
  - pick up N items
  - clear a floor without taking damage
- Progress counts **from the moment a mission appears**, across any number of runs,
  including runs that end in death and runs with a challenge active.
- A finished mission shows the "done" stamp. **Clicking it claims** its talents
  (1–3, scaled with difficulty) and immediately pins a new random mission in that slot.
- A newly drawn mission never shares its goal kind with the two missions still on the board.
- "In one run" goals take the best single run since the mission appeared. Cumulative goals sum.

### Challenges

- A fixed list. Each challenge has a name, a description, modifiers, a target floor N,
  a talent reward and a portal colour.
- **Only one challenge can be active.** Clicking a challenge note selects it. Clicking the
  selected note again cancels it, and clicking another note switches to that one.
- The selected note is visibly marked, and the **lobby portal takes the challenge colour**.
  With nothing selected the portal looks as it does today.
- The selection is saved and survives restarts and finished runs, so a failed challenge can
  be retried straight away.
- **Completion = entering floor N** during a run with the challenge active. For N = 5
  (`FINAL_FLOOR`) it means defeating the final boss.
- The reward is paid **once only**, at the end of the run, and shown on the death/victory
  screen. Afterwards the note carries a permanent stamp. The challenge can still be played,
  but it pays nothing.

Initial list:

| Id | Name | Modifiers | Target | Reward |
|---|---|---|---|---|
| `glass_cannon` | Glass Cannon | player max health 1, player damage ×2 | floor 3 | 4 |
| `pistol_only` | Pistol Only | only the pistol; chests and shops never offer another gun | floor 3 | 3 |
| `horde` | Horde | enemy health ×1.6, enemy damage ×1.3 | floor 4 | 4 |
| `merchants_strike` | Merchant's Strike | no shops | floor 4 | 3 |
| `two_left_feet` | Two Left Feet | dodge disabled | floor 3 | 3 |
| `bane` | Bane | Horde + no armor regeneration | floor 5 | 6 |

Numbers are starting values to be tuned in play.

## Architecture

```
 InGame (gameplay)           progress/ (new module, pure logic)          Lobby
 ─────────────────           ──────────────────────────────────          ─────
 reads RunModifiers  ◄─────  ProgressState::modifiers()           ◄──── board: toggle / claim
 fills RunRecord     ─────►  ProgressState::finish_run(&RunRecord)       portal: accent()
                             → RunOutcome → death screen
```

### 1. `RunRecord`: what happened in a run

- Extends today's `RunStats` (`components/stats.rs`). Gameplay keeps reporting through the
  existing seam `systems::stats::record(world, Stat, amount)`.
- New `Stat` variants as needed: `Kill(EnemyKind)`, `BossKilled(EnemyKind)`, `Gold(u32)`,
  `KillWith(GunKind)`, plus floor events recorded by `InGameScene` (`FloorReached(n)`,
  `FloorUntouched` when a floor ends with no damage taken).
- Kills are not attributed to guns (bullets carry no gun), so there is no per-gun kill goal.
- `RunRecord` is plain data: per-kind kill counts, bosses killed, gold collected, items,
  deepest floor reached, untouched floors, and the challenge the run was
  started with (`Option<ChallengeId>`).
- It carries over between floors the same way `RunStats` does today (`carried_stats` /
  `restore_stats`).

### 2. `RunModifiers`: how the run is tuned

- A plain struct with a neutral `Default`:
  `player_max_health: Option<i32>`, `player_damage: f32`, `enemy_health: f32`,
  `enemy_damage: f32`, `allowed_guns: Option<Vec<GunKind>>`, `shops: bool`,
  `dodge: bool`, `armor_regen: bool`.
- `InGameScene` receives it when a run starts (`start_run`) and keeps it for the run.
- It is read only where it applies:
  - player stats in `systems::apply_run_perks` (health, damage, dodge, armor regen)
  - dungeon layout shop count (`dungeon/layout.rs`)
  - enemy scaling (`enemies.rs`, next to the per-floor growth)
  - weapon chest contents and the starting loadout (allowed guns)
- Gameplay never learns that a "challenge" exists.

### 3. `progress/` module: all mission and challenge logic

Pure Rust with no `hecs::World` and no rendering.

- `missions.rs`: the template pool, drawing without repeating a goal kind (seeded `Rng`),
  and applying a `RunRecord` to mission counters.
- `challenges.rs`: the static challenge table (`ChallengeId` → name, description,
  `RunModifiers`, target floor, reward, portal colour).
- `state.rs`: `ProgressState`, the serialisable state:
  - `missions: [ActiveMission; 3]` (template id, target, reward, progress counter)
  - `selected: Option<ChallengeId>`
  - `completed: BTreeSet<ChallengeId>`
  - `draws: u64` (counter that seeds the next draw so the rotation is deterministic)
- API:
  - `modifiers() -> RunModifiers` (neutral when nothing is selected)
  - `toggle_challenge(id)`
  - `claim(slot) -> Option<u32>` (talents, if the mission was done)
  - `finish_run(&RunRecord) -> RunOutcome` (missions newly finished, challenge reward if earned)
  - `board() -> BoardView` (what the quest board draws)
  - `portal_accent() -> Option<Color>`

### 4. Profile persistence

- `Profile` gains `progress: ProgressState` with `#[serde(default)]`. Old saves load
  with three freshly drawn missions and no challenges, without a version bump.
- Unknown mission templates or challenge ids in a save are dropped with a warning, the same
  way `known_upgrades` handles unknown upgrades. A dropped mission slot is redrawn.
- `SharedProfile` gains `progress()`, `toggle_challenge`, `claim_mission` and `finish_run`.
  Each saves the profile. Claimed talents go to `balance`.

### 5. Lobby

- `quest_board.rs` drops `const TASKS` / `CHALLENGES` and draws a `BoardView` snapshot.
- Clicks on notes become `Claim(slot)` on the Tasks tab and `Toggle(challenge)` on the
  Challenges tab. The board calls `SharedProfile` and rebuilds.
- Note states: in progress, hovered, done/claimable (stamp), plus a "selected" marker for
  the active challenge and a permanent stamp for completed challenges.
- The portal entity gets a `PortalAccent` component, refreshed whenever the selection
  changes. It tints the portal sprite and its ambience particles. `None` means today's look.

### 6. Run start and end

- On `EnterPortal`, the in-game scene starts the run with
  `profile.progress().modifiers()` and the selected challenge id stored in the `RunRecord`.
  The challenge is fixed for the whole run even if the save changes.
- `start_death_screen` keeps calling `record_run`, then calls `finish_run(&record)`.
  The death/victory screen lists missions finished this run and the challenge reward, if any.
- "Die" from the pause menu already goes through `start_death_screen`, so it also reaches
  `finish_run`. Quitting the game mid-run records nothing, the same as run stats today.

## Error handling

- A missing or partial `progress` in the profile gives the default state (fresh missions).
- A selected challenge id that no longer exists counts as nothing selected.
- `claim` on an unfinished or empty slot is a no-op and returns `None`.
- A profile that cannot be written keeps working in memory (existing `SharedProfile` behaviour).

## Testing

- `progress` unit tests:
  - counters advance from a `RunRecord`, both cumulative and best-single-run
  - claiming pays once and redraws without duplicating the kinds left on the board
  - a challenge pays only the first time and only when the target floor is reached
  - toggling selects, switches and cancels
  - modifiers are neutral with no selection
- Profile tests: an old save without `progress` loads, and progress survives a restart.
- Quest board layout tests (existing style): notes come from `BoardView`, the selected
  challenge is marked, and stamps follow the state.
- Gameplay: `RunModifiers::default()` leaves a seeded floor build identical, and each
  modifier changes only what it should (shops absent, enemy health scaled, etc.).

## Out of scope

- Several challenges at once, challenge tiers, repeatable challenge rewards.
- Daily/timed rotation, and mission rerolls for currency.
- Steam achievements tied to missions.
