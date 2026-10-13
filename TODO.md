# Bane — Roadmap (TODO)

> Feature roadmap toward a **commercial Steam release**. See [`FEATURE.md`](FEATURE.md) for
> the current state of the game.
>
> **Target identity:** a skill-based top-down shooter (*Enter the Gungeon* combat) with a
> *Hades*-style layer of per-room upgrade choices and meta-progression. **Single-player
> first**; online co-op is a later phase.

## What makes the loop "good" (every milestone is measured against this)

1. **Seconds** — combat is fun on its own: dodge skill, juice, readable attack patterns. ← *Gungeon*
2. **Minutes** — power grows within a run through choices that form a build with synergies. ← *Hades/Isaac*
3. **Hours** — runs differ (boons, biomes, elites, bosses) and build to a climax (a win condition).
4. **Days/weeks** — meta-progression, unlocks and hooks ("one more run", dailies, achievements) pull players back.

## Milestone map

| # | Milestone | Goal in one line | Pillar |
|---|-----------|------------------|--------|
| **M0** | Stabilize & scale | Fix what's broken, add difficulty scaling + seeded RNG — the loop can't hold without it. | Tech/Structure |
| **M1** | Combat feel | Dodge-roll + i-frames, juice, readable attack telegraphs — Gungeon-grade combat. | Combat |
| **M2** | In-run build system | Boons: choose-1-of-3 after rooms, stacking, synergies, rarities. **The biggest new system.** | Build |
| **M3** | Run structure & bosses | Win condition + final boss, more bosses, biomes/acts, elite (affixed) enemies. | Structure |
| **M4** | Content & meta & Steam | More weapons/enemies/biomes, deep meta-progression, hub/narrative, Steam, dailies. | Content/Meta |
| **M5** | Multiplayer (online co-op) | Break singletons, netcode (host-authoritative), Steam lobby. PvP out of scope. | MP |

## Tag legend

Each item: `- [ ] **Name** — `PRIORITY` · `EFFORT` · *Pillar* — note / acceptance criterion`

- **Priority:** `P0` (must-have for a good loop) · `P1` (important) · `P2` (nice-to-have)
- **Effort:** `S` · `M` · `L` · `XL`
- **Pillar:** Combat / Build / Structure / Content / Meta / MP / Tech

---

## M0 — Stabilize & scale
*Goal: the loop holds together and has a difficulty curve.*

- [ ] **Per-floor difficulty scaling** — `P0` · `M` · *Structure* — enemies do **not** scale today (stats are constructor literals; `EnemyFactory` only filters by floor). Add a per-floor multiplier for HP/damage/speed + rising spawn density.
- [ ] **Weapon balance pass** — `P0` · `M` · *Combat* — fix Shotgun `222–333` (debug values in `Shotgun.java:32-33`); set a DPS baseline and target time-to-kill per enemy tier.
- [ ] **Seeded deterministic RNG** — `P0` · `L` · *Tech* — centralize randomness behind one seeded service (today: scattered `Random` / `System.currentTimeMillis()`). Unlocks dailies (M4) and determinism for MP (M5).
- [ ] **Win condition (placeholder)** — `P1` · `S` · *Structure* — a run must be "finishable" (final floor/portal) so the loop has an end. Full final boss lands in M3.
- [ ] **Clean up WIP stubs** — `P1` · `S` · *Tech* — Golem melee (`if (true) return;`), PurpleSpiderling teleport, empty Controls settings tab, blocking Discord loop.
- [ ] **Save robustness** — `P2` · `S` · *Tech* — harden `upgrades.bin` load/save against corruption/version drift.

## M1 — Combat feel (Gungeon pillar)
*Goal: combat is fun on its own — skillful and readable.*

- [ ] **Dodge-roll / dash with i-frames** — `P0` · `M` · *Combat* — core skill defense; the `player_dash` audio asset already exists. Input + cooldown + brief invulnerability + animation. **(First M1 plan — see `docs/superpowers/plans/`.)**
- [ ] **Hitstop + screenshake** — `P0` · `S` · *Combat* — freeze-frame and camera shake on hits/explosions/deaths. Cheap, huge feel payoff.
- [ ] **Enemy attack telegraphs** — `P0` · `M` · *Combat* — wind-up animations/warnings before attacks (today enemies just chase and fire on cooldown). Required for a readable bullet-hell.
- [ ] **Shot-pattern system** — `P1` · `M` · *Combat* — aimed / fan / spiral as a reusable system (Slime's spiral exists), for enemies and bosses.
- [ ] **Weapon game-feel** — `P1` · `M` · *Combat* — recoil, muzzle flash, impact particles, better reload feedback, knockback tuning.
- [ ] **Gamepad / twin-stick controls** — `P1` · `L` · *Combat* — required for Steam / Steam Deck.
- [ ] **Active reload (optional)** — `P2` · `S` · *Combat* — Gears-style timing bonus.

## M2 — In-run build system (Hades pillar)
*Goal: the biggest new system — you assemble a build from choices during a run.*

- [ ] **Boon system (core)** — `P0` · `XL` · *Build* — after clearing a room / in reward rooms, offer **choose-1-of-3**; boons persist for the whole run and stack. Apply via the existing hook-composite pattern (`DamageEventHook` / `MoveForceEventHook`). Reuse/replace the unused `Affixes` enum.
- [ ] **Boon pool & categories** — `P0` · `L` · *Build* — stats (dmg/firerate/crit/move/HP), **on-hit elements** (burn/freeze/**chain-lightning** — note: today's `lightning` package is only scene lighting; build a real chaining attack), weapon modifiers (pierce/bounce/+projectiles), defensive/utility.
- [ ] **Boon rarities** — `P1` · `S` · *Build* — reuse `QualityLevel` (COMMON/RARE/EPIC).
- [ ] **Synergies between boons** — `P1` · `L` · *Build* — combinations unlock emergent effects (Isaac flavor).
- [ ] **Boon HUD + inspect screen** — `P0` · `M` · *Build/UI* — show the active build.
- [ ] **Vendors / "gods"** — `P1` · `M` · *Build* — different NPCs/altars offer different boon pools (Hades framing); ties shop + reward rooms into the build economy.
- [ ] **Choice economy** — `P2` · `S` · *Build* — re-roll for coins, banish, coin sink.

## M3 — Run structure & bosses
*Goal: runs vary and climax; the spine of replayability.*

- [ ] **Win condition + final boss + victory flow** — `P0` · `L` · *Structure* — a real run climax with a victory screen and rewards.
- [ ] **Boss roster expansion** — `P0` · `L` · *Structure* — today there are 3 bosses (Golem is unspawnable, Spider is floor-1-only). Add multi-phase, telegraphed bosses — ideally one per biome.
- [ ] **Biomes / acts** — `P1` · `L` · *Structure* — distinct tileset + enemy set + hazards per act (today: a single shared tileset, no biomes).
- [ ] **Act structure** — `P1` · `M` · *Structure* — e.g. 3 acts × N floors with escalating difficulty, tied to M0 scaling.
- [ ] **Elite / affixed enemies** — `P1` · `M` · *Structure* — enemy affixes (the unused `AffixType` enum names — `FORTIFIED`, `RAGING`, `REGENERATION` — fit exactly). Reuse that enum here.
- [ ] **Room variety & mini-events** — `P2` · `M` · *Structure* — challenge rooms, curse rooms, timed events.

## M4 — Content, meta-progression & Steam
*Goal: commercial depth and retention.*

- [ ] **Steam integration** — `P0` · `L` · *Meta* — achievements, stats, cloud saves, rich presence (Discord is already stubbed), trading cards. Required for a commercial release.
- [ ] **Deep meta-progression** — `P0` · `L` · *Meta* — today's talent tree is 15 single-level binary nodes. Add multi-level upgrades, more trees, and unlocking new weapons/boons/characters via a meta-currency (a Hades "mirror").
- [ ] **More weapons with real mechanics** — `P1` · `L` · *Content* — pierce / explosive / charge / beam, not just stat variants.
- [ ] **More enemies, room layouts, items, boons** — `P1` · `L` · *Content* — breadth for replayability.
- [ ] **Hub / meta-narrative** — `P1` · `L` · *Meta* — grow the lobby into a hub with NPCs, story beats, and "one more run" hooks (Hades model).
- [ ] **Retention features** — `P1` · `M` · *Meta* — daily runs (needs M0 seeded RNG), weekly challenges, leaderboards, run history, unlockable difficulty modifiers (Hades' Heat / Isaac hard mode).
- [ ] **Characters / classes** — `P2` · `L` · *Content* — distinct starting kits; a big replay multiplier.
- [ ] **Onboarding, accessibility, localization** — `P1` · `M` · *Meta* — tutorial, key rebinding (finish the Controls tab), CZ/EN localization.

## M5 — Multiplayer (online co-op)
*Goal: 2–4 player shared-dungeon co-op. PvP explicitly out of scope.*

- [ ] **De-singleton-ize core systems** — `P0` · `XL` · *Tech/MP* — replace singletons/global mutable state (`PhysicsService`, `GunService`, `GameStateController`, static `AssetsService`, …) with DI and per-player state + a player-id concept. The hardest prerequisite; the in-progress module split helps.
- [ ] **Netcode model decision + core** — `P0` · `XL` · *MP* — choose host-authoritative + client-side prediction (typical for co-op shooters) and build the sync layer (entities, bullets, RNG).
- [ ] **Steam networking transport** — `P1` · `L` · *MP* — Steamworks P2P / relay; ties to M4 Steam integration.
- [ ] **Real lobby & invites** — `P1` · `L` · *MP* — `LobbyState` becomes a true lobby; Steam lobbies, join/invite (Discord join is already stubbed).
- [ ] **Co-op design rules** — `P1` · `M` · *MP* — shared vs. individual loot/boons, revive mechanic, scaling by player count.

---

## Cross-cutting: tech debt & foundations
*Not a milestone — threads through everything and directly unlocks M5.*

- [ ] **Finish the module split** — `P1` · `L` · *Tech* — complete `game-engine-core` / `-physics` / `-render` / `-audio` extraction (in progress on this branch).
- [ ] **Replace singletons with DI** — `P1` · `XL` · *Tech* — needed for both MP and testability (overlaps M5).
- [ ] **Expand automated tests** — `P1` · `M` · *Tech* — build on the existing fast/integration split; cover combat, scaling, build system.
- [ ] **Performance budget** — `P2` · `M` · *Tech* — continue the render/shadow optimization already underway; set frame-time targets.
- [ ] **Seeded RNG service** — `P0` · `L` · *Tech* — foundational; also listed under M0. Feeds dailies and MP determinism.

---

## Suggested execution order

`M0 → M1 → M2 → M3 → M4 → (M5)`, with tech-debt foundations (seeded RNG, module split,
de-singleton-ization) pulled forward wherever they unblock the next milestone. Each milestone
should leave the game in a shippable, playtest-able state.
