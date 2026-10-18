# ECS refactor: merge notes (2026-10-18)

Branch `ecs-refactor`. Plan: `2026-10-18-ecs-refactor-plan.md`.

## What changed architecturally

- **Died / Gone tags**: a fallen enemy gets `Died` for the length of the death phase (every death
  effect reads it, it is cleared at the end); a finished corpse gets `Gone` in the frame, the payouts
  read it, then `despawn_gone` removes it.
- **Hit entities + `hit_phase`**: `deal` drops the health and spawns a `Hit` entity; `hit_phase`
  runs the reactions (hit count, run stats, flash, floating number) and then clears the hits.
- **Fx / Resources / RunState**: effects, shared services and per-run state are passed as bundles
  instead of loose argument lists.
- **`systems::player`**: the one place that finds the player (`player`, `player_at`, `player_body`,
  `uncloaked_player`, `living_player_at`, ...).
- **Loadout rules**: what a loadout allows is data-driven rules rather than scattered checks.
- **`boss_kit` / `BossBrain` / `Enrage` / `Inactive`**: boss behaviour is a kit plus a brain component,
  rage is its own component ticked before the bosses, and `Inactive` names "spawning, expiring or
  frozen" for queries.
- **Schedule phases + ordering contract**: the in-game step and frame are named phases in
  `systems/schedule.rs`; the ordering contract in its module doc says what must precede what.
- **Physics requests**: `Teleport`, `MoveTo` and `SetVelocity` are requests components carried out
  by `simulate`; `Physics::place` / `halt` are for writes that must be seen at once.

## Named behaviour changes

- `clean_up_after_boss` runs once, on the tick the boss falls (`Died`). Before, it ran every tick of
  the corpse animation, so minions born after that tick (a Hatcher in the room, or a second boss)
  were killed too; now they live.
- A cart hit's damage number spawns from the post-physics position.
- The cosmetic rng draw order of damage numbers changed (cosmetic only).
- Rage sounds are queued before boss sounds within a frame.
- `Atmosphere` is no longer spawned in the cinematic and epilogue scenes (it was never ticked there).

## Follow-ups

- `enemy_attacks_phase` still passes the physics/textures/sounds/dt tuple instead of `Hostile`.
- Six remaining `Spawning`/`Expiring`/`Frozen` triples could use `Inactive` (`bosses.rs`,
  `skills.rs`, `damage.rs`).
- `RunStats`, `PlayerPerks` and `Wallet` are still components on the player entity.
- The lobby frame is only partly on the schedule; the rest is hand-wired.
- `carry_out_requests` has a triplicated loop.
