# Missions & Challenges Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the lobby quest board real: three rotating missions that pay talents when claimed, and six selectable whole-run challenges that recolour the portal and pay talents once when the target floor is reached.

**Architecture:** Gameplay only knows two plain data types. `RunRules` (`rules.rs`) says how a run is tuned and is read through the existing `Perks` channel plus three narrow hooks (enemy health, shop count, allowed gun). `RunStats` (extended) says what happened. A new pure module `progress/` owns missions, challenges and their saved state. It turns `RunStats` into progress and a challenge into `RunRules`. The profile persists it, and the lobby board just draws a `BoardView` and sends clicks back.

**Tech Stack:** Rust 2021, hecs ECS, serde/serde_json, glam. Run tests from `rust/` with `cargo test`.

**Spec:** `docs/superpowers/specs/2026-10-14-missions-challenges-design.md`

## Global Constraints

- Work in a separate git worktree on branch `missions-challenges` cut from `master`. Other sessions commit to `master` concurrently, so never commit there directly.
- All commands run from `rust/`: `cargo test`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`.
- Workspace lints: `unreachable_pub` (use `pub(crate)`), `missing_debug_implementations` (derive `Debug` on every new type), `clippy::unwrap_used` (no `unwrap()` outside tests).
- Style: no comments unless one explains a non-obvious *why*, the same as the surrounding code. Test names are sentences (`fn a_challenge_pays_only_the_first_time()`).
- Only one challenge active at a time. A challenge reward is paid once. Missions: 3 on the board, progress counts from when a mission appeared, and clicking a finished one claims it.
- Old profiles without `progress` must load unchanged (no version bump; `PROFILE_VERSION` stays 1).
- `RunRules::default()` must leave gameplay byte-for-byte identical (the seeded-floor fingerprint test in `scenes/ingame.rs` must keep passing untouched).
- Commit after every task with a `feat:`/`test:` message ending with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

**Spec deviations (decided while planning, reflected in the spec):**
- The "kill N enemies with a given gun" mission kind is dropped, because bullets carry no gun attribution and adding it would couple shooting to progress.
- Pistol Only: weapon chests give only their normal item burst, and shops offer an artefact instead of a gun. No extra gold.

## Review Focus

1. **A boss kill must count as a kill of its kind.** Kills are counted in `systems::items::enemy_drops` when the death animation of an `Enemy + Expiring` (non-`Minion`) entity finishes. If a boss's death skips that path, "Defeat the X" missions never finish. Pinned by a test in Task 3.
2. **The profile file has garbage in `progress`** (wrong type, unknown mission kind, removed challenge id). Expected: the rest of the profile survives, unknown entries are dropped, and the board refills to 3. Pinned in Task 5.
3. **Glass Cannon on later floors.** Max health must stay 1 after `next_floor` (stats are carried, perks re-applied per floor), and health potions must not raise it above the cap. Pinned in Task 2.
4. **Pistol Only with the starting shotgun.** The starting loadout holds Pistol + Shotgun. The shotgun must be removed when the rule applies, and no chest or shop may offer another gun. Pinned in Task 2.
5. **Claiming twice / clicking a selected challenge while the board rebuilds.** Claim on an unfinished slot is a no-op, and the second toggle cancels. Pinned in Tasks 4 and 7.

---

## File Map

| File | Status | Responsibility |
|---|---|---|
| `rust/app/src/rules.rs` | create | `RunRules` (neutral `Default`) and `RunRules::shape(Perks) -> Perks` |
| `rust/app/src/upgrades.rs` | modify | `Perks` gains `max_health_cap`, `dodge`, `only_gun` |
| `rust/app/src/components/combat.rs` | modify | `Enemy.kind`; `Loadout` gun restriction |
| `rust/app/src/components/stats.rs` | modify | `RunStats` gains per-kind kills, gold, hurt counters, untouched floors |
| `rust/app/src/systems/stats.rs` | modify | `Stat::Kill(EnemyKind)`, `Stat::Gold`, `Stat::Hurt`, `close_floor` |
| `rust/app/src/systems/{items,damage,dodge,loadout,perks}.rs` | modify | report stats; honour dodge / gun rules |
| `rust/app/src/systems/rules.rs` | create | `toughen_enemies` (enemy health rule) |
| `rust/app/src/dungeon/build.rs` | modify | `generate_for_run(rng, theme, shops)` |
| `rust/app/src/enemies.rs` | modify | serde derive on `EnemyKind` |
| `rust/app/src/progress/{mod,challenges,missions,state}.rs` | create | pure mission/challenge logic and saved state |
| `rust/app/src/profile.rs` | modify | persist `ProgressState`; `SharedProfile` progress API |
| `rust/app/src/scenes/ingame.rs` | modify | use rules at run start/build, close floors, finish run |
| `rust/app/src/systems/death_screen.rs`, `prefabs/ui.rs`, `components/death.rs` | modify | one extra "progress" line on the death screen |
| `rust/app/src/scenes/quest_board.rs` | modify | draw `BoardView`, emit `BoardAction` |
| `rust/app/src/scenes/lobby.rs`, `systems/ambience.rs` or new `systems/portal.rs` | modify/create | wire board actions; tint the portal |

---

### Task 0: Worktree

- [ ] **Step 1: Create the worktree** (REQUIRED SUB-SKILL: superpowers:using-git-worktrees)

```bash
cd /Users/jirka/IdeaProjects/Bane_refactor-rust
git worktree add ../Bane_refactor-missions -b missions-challenges master
cd ../Bane_refactor-missions/rust && cargo test 2>&1 | tail -3
```
Expected: `test result: ok`. All later paths are relative to this worktree.

---

### Task 1: `RunRules` and `Perks` shaping

**Files:**
- Create: `rust/app/src/rules.rs`
- Modify: `rust/app/src/main.rs` (add `mod rules;` after `mod rng;`)
- Modify: `rust/app/src/upgrades.rs` (struct `Perks` ~line 286, `from_unlocked`, `max_health`)

**Interfaces:**
- Produces:
  - `crate::rules::RunRules { player_max_health: Option<i32>, player_damage: f32, enemy_health: f32, enemy_damage: f32, only_gun: Option<GunKind>, shops: bool, dodge: bool, armor_regen: bool }` (`Debug, Clone, Copy, PartialEq`, manual neutral `Default`)
  - `RunRules::shape(&self, perks: Perks) -> Perks`
  - `Perks` new pub(crate) fields: `max_health_cap: Option<i32>`, `dodge: bool`, `only_gun: Option<GunKind>`

- [ ] **Step 1: Write the failing tests** at the bottom of the new `rules.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_rules_leave_the_perks_untouched() {
        let perks = Perks::default();
        assert_eq!(RunRules::default().shape(perks), perks);
    }

    #[test]
    fn rules_scale_damage_cap_health_and_switch_things_off() {
        let rules = RunRules {
            player_max_health: Some(1),
            player_damage: 2.0,
            enemy_damage: 1.5,
            only_gun: Some(GunKind::Pistol),
            dodge: false,
            armor_regen: false,
            ..RunRules::default()
        };
        let mut owned = Perks::default();
        owned.armor_regeneration = true;
        let shaped = rules.shape(owned);

        assert_eq!(shaped.max_health(10), 1);
        assert_eq!(shaped.damage(10, false), 20);
        assert_eq!(shaped.damage_taken(2), 3);
        assert!(!shaped.dodge);
        assert!(!shaped.armor_regeneration);
        assert_eq!(shaped.only_gun, Some(GunKind::Pistol));
    }
}
```

Before writing this, check the name and signature of the player-damage helper in `upgrades.rs` (the fold over `damage_scales` near line 355). If it is not `damage(base, critical)`, use the real name in the test.

- [ ] **Step 2: Run them to see them fail**

Run: `cargo test rules::` → FAIL (module/type missing).

- [ ] **Step 3: Implement**

`upgrades.rs`: add to `Perks`:

```rust
    pub(crate) max_health_cap: Option<i32>,
    pub(crate) dodge: bool,
    pub(crate) only_gun: Option<crate::guns::GunKind>,
```
In `from_unlocked` set `max_health_cap: None, dodge: true, only_gun: None`. Change `max_health`:

```rust
    pub(crate) fn max_health(&self, base: i32) -> i32 {
        let scaled = self
            .health_scales
            .iter()
            .fold(base, |value, scale| (value as f32 * scale) as i32);
        self.max_health_cap.map_or(scaled, |cap| scaled.min(cap))
    }
```

`rules.rs`:

```rust
use crate::guns::GunKind;
use crate::upgrades::Perks;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct RunRules {
    pub(crate) player_max_health: Option<i32>,
    pub(crate) player_damage: f32,
    pub(crate) enemy_health: f32,
    pub(crate) enemy_damage: f32,
    pub(crate) only_gun: Option<GunKind>,
    pub(crate) shops: bool,
    pub(crate) dodge: bool,
    pub(crate) armor_regen: bool,
}

impl Default for RunRules {
    fn default() -> Self {
        Self {
            player_max_health: None,
            player_damage: 1.0,
            enemy_health: 1.0,
            enemy_damage: 1.0,
            only_gun: None,
            shops: true,
            dodge: true,
            armor_regen: true,
        }
    }
}

impl RunRules {
    pub(crate) fn shape(&self, mut perks: Perks) -> Perks {
        perks.damage_scales[0] *= self.player_damage;
        perks.damage_taken_scale *= self.enemy_damage;
        perks.max_health_cap = self.player_max_health;
        perks.only_gun = self.only_gun;
        perks.dodge &= self.dodge;
        perks.armor_regeneration &= self.armor_regen;
        perks
    }
}
```

- [ ] **Step 4: Run the tests**

Run: `cargo test` → all pass, including the existing `upgrades` and `perks` tests.

- [ ] **Step 5: Commit**

```bash
git add app/src/rules.rs app/src/main.rs app/src/upgrades.rs
git commit -m "feat: run rules that reshape the player's perks"
```

---

### Task 2: Gameplay honours the rules

**Files:**
- Modify: `rust/app/src/systems/dodge.rs` (`start_dodge`)
- Modify: `rust/app/src/components/combat.rs` (`Loadout`)
- Modify: `rust/app/src/systems/perks.rs` (`apply_floor_perks`)
- Modify: `rust/app/src/systems/loadout.rs` (`unused_gun`)
- Create: `rust/app/src/systems/rules.rs` (+ `mod rules;` and `pub(crate) use rules::toughen_enemies;` in `systems/mod.rs`)
- Modify: `rust/app/src/components/combat.rs` or `tags.rs` (marker `Toughened`)
- Modify: `rust/app/src/dungeon/build.rs`

**Interfaces:**
- Consumes: `Perks.dodge`, `Perks.only_gun`, `Perks.max_health_cap` (Task 1)
- Produces:
  - `Loadout::restrict(&mut self, only: Option<GunKind>)` and `Loadout::allows(&self, kind: GunKind) -> bool`
  - `systems::toughen_enemies(world: &mut World, factor: f32)`
  - `Dungeon::generate_for_run(rng: &mut Rng, theme: Theme, shops: bool) -> Result<Dungeon, Box<dyn Error>>`

- [ ] **Step 1: Write the failing tests**

In `systems/perks.rs` tests (reuse the existing `player()` and `vitals()` helpers):

```rust
    #[test]
    fn a_health_cap_keeps_the_player_at_one_heart() {
        let mut world = player();
        let mut capped = Perks::default();
        capped.max_health_cap = Some(1);
        apply_run_perks(&mut world, capped);
        let (current, max, _, _) = vitals(&world);
        assert_eq!((current, max), (1, 1));
    }

    #[test]
    fn pistol_only_takes_the_starting_shotgun_away() {
        let mut world = player();
        let mut only = Perks::default();
        only.only_gun = Some(crate::guns::GunKind::Pistol);
        apply_floor_perks(&mut world, only);
        let loadout = world
            .query::<&Loadout>()
            .with::<&Player>()
            .iter()
            .next()
            .cloned()
            .expect("a loadout");
        assert!(loadout.has(crate::guns::GunKind::Pistol));
        assert!(!loadout.has(crate::guns::GunKind::Shotgun));
        assert!(!loadout.allows(crate::guns::GunKind::Uzi));
    }
```
(If `Loadout` is not `Clone`, read `has`/`allows` inside the query instead of cloning.)

In `systems/loadout.rs` tests:

```rust
    #[test]
    fn a_restricted_loadout_is_never_offered_another_gun() {
        let mut loadout = Loadout::starting();
        loadout.restrict(Some(GunKind::Pistol));
        for roll in [0.0, 0.3, 0.6, 0.99] {
            assert_eq!(unused_gun(&loadout, roll), None);
        }
    }
```

In `systems/dodge.rs` tests (or create a `#[cfg(test)] mod tests` there): spawn a player with `prefabs::player`, insert `PlayerPerks(perks)` with `perks.dodge = false`, build an `Input` that has `Action::Dodge` pressed (copy how existing dodge tests build input; search `Action::Dodge` under `#[cfg(test)]`), call `start_dodge`, and assert that the player has no `Dodge` component. Add a twin case with `dodge = true` that asserts a `Dodge` component *is* present.

New `systems/rules.rs`:

```rust
#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::assets::Textures;
    use crate::components::Health;
    use crate::enemies::EnemyKind;
    use crate::prefabs;

    use super::*;

    #[test]
    fn new_enemies_are_toughened_once_and_neutral_rules_touch_nothing() {
        let mut world = World::new();
        let rat = prefabs::enemy(&mut world, EnemyKind::Rat, Vec2::ZERO, 1, &Textures::default());
        let base = world.get::<&Health>(rat).expect("health").max;

        toughen_enemies(&mut world, 1.0);
        assert_eq!(world.get::<&Health>(rat).expect("health").max, base);

        toughen_enemies(&mut world, 2.0);
        toughen_enemies(&mut world, 2.0);
        let health = world.get::<&Health>(rat).expect("health");
        assert_eq!((health.current, health.max), (base * 2, base * 2));
    }
}
```

In `dungeon/build.rs` tests (or `layout.rs` tests, next to `count(layout, RoomKind::Shop)`):

```rust
    #[test]
    fn a_run_without_shops_gets_no_shop_room_and_the_same_run_with_shops_matches_today() {
        let without = Dungeon::generate_for_run(&mut Rng::new(7), Theme::Stone, false).expect("a dungeon");
        assert!(without.rooms.iter().all(|room| room.kind != RoomKind::Shop));

        let with = Dungeon::generate_for_run(&mut Rng::new(7), Theme::Stone, true).expect("a dungeon");
        let today = Dungeon::generate_themed(&mut Rng::new(7), Theme::Stone).expect("a dungeon");
        let kinds = |dungeon: &Dungeon| dungeon.rooms.iter().map(|room| room.kind).collect::<Vec<_>>();
        assert_eq!(kinds(&with), kinds(&today));
    }
```

- [ ] **Step 2: Run them to see them fail**

Run: `cargo test` → the new tests fail to compile (`restrict`, `allows`, `toughen_enemies`, `generate_for_run` missing).

- [ ] **Step 3: Implement**

`components/combat.rs` `Loadout`: add a field `only: Option<GunKind>` (`None` in `starting()`) and:

```rust
    pub(crate) fn restrict(&mut self, only: Option<GunKind>) {
        self.only = only;
        if only.is_none() {
            return;
        }
        for slot in &mut self.slots {
            if slot.as_ref().is_some_and(|weapon| !self.only.map_or(true, |kind| kind == weapon.kind)) {
                *slot = None;
            }
        }
        if self.slots.get(self.current).is_none_or(Option::is_none) {
            self.current = self.slots.iter().position(Option::is_some).unwrap_or(0);
        }
    }

    pub(crate) fn allows(&self, kind: GunKind) -> bool {
        self.only.is_none_or(|only| only == kind)
    }
```
(Simplify the borrow if the compiler complains: compute `let only = self.only;` first. `Option::is_none_or` is stable since Rust 1.82. If the toolchain is older, use `map_or(true, ...)`.)

`systems/perks.rs` `apply_floor_perks`: after `loadout.apply_perks(...)` add `loadout.restrict(perks.only_gun);`.

`systems/loadout.rs` `unused_gun`: add `.filter(|kind| loadout.allows(*kind))` to the iterator. Shops and chests both go through it, so nothing else changes. Shop `Stock::Offensive` already falls back to an artefact when `unused_gun` is `None`.

`systems/dodge.rs` `start_dodge`: add `Option<&crate::components::PlayerPerks>` to the query and filter out players whose perks say `!dodge`:

```rust
        .query::<(Entity, &Position, &Health, Option<&crate::components::PlayerPerks>)>()
        ...
        .filter(|(_, _, health, perks)| health.current > 0 && perks.is_none_or(|perks| perks.0.dodge))
        .map(|(entity, position, _, _)| (entity, position.0))
```
The lobby player has no `PlayerPerks`, so dodge keeps working there.

Marker: add `#[derive(Debug, Clone, Copy)] pub(crate) struct Toughened;` next to `Enemy` in `components/combat.rs` and export it the same way `Enemy` is exported from `components/mod.rs`.

`systems/rules.rs`:

```rust
use hecs::{Entity, World};

use crate::components::{Enemy, Health, Toughened};
use crate::enemies::scale_stat;

pub(crate) fn toughen_enemies(world: &mut World, factor: f32) {
    if (factor - 1.0).abs() < f32::EPSILON {
        return;
    }
    let fresh: Vec<Entity> = world
        .query::<Entity>()
        .with::<(&Enemy, &Health)>()
        .without::<&Toughened>()
        .iter()
        .collect();
    for entity in fresh {
        if let Ok(mut health) = world.get::<&mut Health>(entity) {
            health.max = scale_stat(health.max, factor);
            health.current = scale_stat(health.current, factor);
        }
        let _ = world.insert_one(entity, Toughened);
    }
}
```

`dungeon/build.rs`:

```rust
    pub(crate) fn generate_themed(rng: &mut Rng, theme: Theme) -> Result<Self, Box<dyn Error>> {
        Self::generate_for_run(rng, theme, true)
    }

    pub(crate) fn generate_for_run(
        rng: &mut Rng,
        theme: Theme,
        shops: bool,
    ) -> Result<Self, Box<dyn Error>> {
        let templates = Templates::built_in()?;
        let mut spec = LayoutSpec::standard(rng);
        if !shops {
            spec.shops = 0;
        }
        let layout = Layout::generate(&spec, rng).ok_or("could not lay out the rooms")?;
        Self::build(&layout, &templates, theme, rng)
    }
```

- [ ] **Step 4: Run the tests**

Run: `cargo test` → all pass. Then also run `cargo clippy --all-targets -- -D warnings`.

- [ ] **Step 5: Commit**

```bash
git add -A app/src
git commit -m "feat: gameplay honours run rules for dodge, guns, enemy health and shops"
```

---

### Task 3: Richer run stats

**Files:**
- Modify: `rust/app/src/components/stats.rs`
- Modify: `rust/app/src/components/combat.rs` (`Enemy { death, kind }`)
- Modify: `rust/app/src/prefabs/actors.rs:161` (fill `kind`)
- Modify: `rust/app/src/systems/stats.rs`
- Modify: `rust/app/src/systems/items.rs` (`enemy_drops` ~line 160, coin pickup ~line 83)
- Modify: `rust/app/src/systems/damage.rs` (`hurt` ~line 210)
- Modify: `rust/app/src/systems/mod.rs` (export `close_floor as close_stats_floor`)

**Interfaces:**
- Produces:
  - `RunStats` fields (all `pub(crate)`, still `Copy + Default`): `fired, hits, kills, items` (unchanged) plus `kills_of: [u32; EnemyKind::ALL.len()]`, `gold: u32`, `hurt_this_floor: u32`, `untouched_floors: u32`
  - `RunStats::kills_of(&self, kind: EnemyKind) -> u32`
  - `Stat::Kill(EnemyKind)`, `Stat::Gold`, `Stat::Hurt` (and the existing `Fired`, `Hit`, `Item`)
  - `systems::close_stats_floor(world: &mut World)`: if no hurt this floor, `untouched_floors += 1`; resets `hurt_this_floor`
  - `EnemyKind::index(self) -> usize` (position in `ALL`)

- [ ] **Step 1: Write the failing tests** in `systems/stats.rs` tests (adapt the existing test that calls `record(&mut world, Stat::Kill, 2)` to `record(&mut world, Stat::Kill(EnemyKind::Rat), 2)`):

```rust
    #[test]
    fn kills_are_counted_per_kind_and_in_total() {
        let mut world = world_with_player();
        record(&mut world, Stat::Kill(EnemyKind::Rat), 2);
        record(&mut world, Stat::Kill(EnemyKind::Golem), 1);
        let stats = current(&world);
        assert_eq!(stats.kills, 3);
        assert_eq!(stats.kills_of(EnemyKind::Rat), 2);
        assert_eq!(stats.kills_of(EnemyKind::Golem), 1);
    }

    #[test]
    fn a_floor_without_hurt_counts_as_untouched_and_a_hurt_one_does_not() {
        let mut world = world_with_player();
        close_floor(&mut world);
        record(&mut world, Stat::Hurt, 1);
        close_floor(&mut world);
        close_floor(&mut world);
        let stats = current(&world);
        assert_eq!(stats.untouched_floors, 2);
        assert_eq!(stats.hurt_this_floor, 0);
    }

    #[test]
    fn coins_are_counted_as_gold() {
        let mut world = world_with_player();
        record(&mut world, Stat::Gold, 5);
        assert_eq!(current(&world).gold, 5);
    }
```
(`world_with_player` is whatever helper the existing stats test uses to spawn a `Player` with `RunStats`. Reuse it, or spawn `(Player, RunStats::default())` directly.)

In `systems/items.rs` tests (Review Focus #1), make sure a dying boss counts:

```rust
    #[test]
    fn a_boss_that_finishes_dying_counts_as_a_kill_of_its_kind() {
        // Spawn a player (prefabs::player) and a Golem via prefabs::enemy, then mark the golem
        // the way resolve_deaths does: insert Expiring and play its death clip to the end
        // (copy the setup used by the existing enemy_drops tests in this file).
        // Run enemy_drops once.
        // assert_eq!(current_stats(&world).kills_of(EnemyKind::Golem), 1);
    }
```
Fill the body from the closest existing `enemy_drops` test in `systems/items.rs` (search `enemy_drops(` under `#[cfg(test)]`). If bosses carry `Minion` or never reach `Expiring`, the test exposes it. In that case record the boss kill from `RoomEvent::BossDefeated` in Task 6 instead, and turn this into a test of that path.

In `systems/damage.rs` tests: after an existing "player takes damage" test passes, assert `current_stats(&world).hurt_this_floor == 1`. If no such test exists, call `hurt_player(&mut world, player, 1, &mut Vec::new())` on a freshly spawned player.

- [ ] **Step 2: Run them to see them fail**

Run: `cargo test stats` → compile errors.

- [ ] **Step 3: Implement**

`enemies.rs`, inside `impl EnemyKind`:

```rust
    pub(crate) fn index(self) -> usize {
        Self::ALL.iter().position(|kind| *kind == self).unwrap_or(0)
    }
```

`components/stats.rs`:

```rust
use crate::enemies::EnemyKind;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct RunStats {
    pub(crate) fired: u32,
    pub(crate) hits: u32,
    pub(crate) kills: u32,
    pub(crate) items: u32,
    pub(crate) kills_of: [u32; EnemyKind::ALL.len()],
    pub(crate) gold: u32,
    pub(crate) hurt_this_floor: u32,
    pub(crate) untouched_floors: u32,
}

impl RunStats {
    pub(crate) fn kills_of(&self, kind: EnemyKind) -> u32 {
        self.kills_of[kind.index()]
    }
}
```
Fix existing struct literals of `RunStats` (e.g. in `profile.rs` tests) by adding `..RunStats::default()`.

`systems/stats.rs`:

```rust
pub(crate) enum Stat {
    Fired,
    Hit,
    Kill(EnemyKind),
    Item,
    Gold,
    Hurt,
}
...
            Stat::Kill(kind) => {
                stats.kills += amount;
                stats.kills_of[kind.index()] += amount;
            }
            Stat::Gold => stats.gold += amount,
            Stat::Hurt => stats.hurt_this_floor += amount,
...
pub(crate) fn close_floor(world: &mut World) {
    for stats in world.query_mut::<&mut RunStats>().with::<&Player>() {
        if stats.hurt_this_floor == 0 {
            stats.untouched_floors += 1;
        }
        stats.hurt_this_floor = 0;
    }
}
```
`Stat` derives `Debug, Clone, Copy, PartialEq, Eq`, which still holds because `EnemyKind` is `Copy + Eq`.

`components/combat.rs`: `pub(crate) struct Enemy { pub(crate) death: Clip, pub(crate) kind: EnemyKind }`, and in `prefabs/actors.rs` `Enemy { death: spec.death_clip(), kind }`.

`systems/items.rs` `enemy_drops`: replace the `kills` count with the kinds:

```rust
    let killed: Vec<crate::enemies::EnemyKind> = world
        .query::<(&Enemy, &Animation)>()
        .with::<&Expiring>()
        .without::<&crate::components::Minion>()
        .iter()
        .filter(|(_, animation)| animation.finished())
        .map(|(enemy, _)| enemy.kind)
        .collect();
    for kind in killed {
        super::stats::record(world, super::stats::Stat::Kill(kind), 1);
    }
```
Coin pickup (`ItemKind::Coin` branch): after the wallet loop add `super::stats::record(world, super::stats::Stat::Gold, 1);`. If `world` is mutably borrowed by the loop, put the call after the loop.

`systems/damage.rs` `hurt`: just before `return true;` add `super::stats::record(world, super::stats::Stat::Hurt, 1);`. Use the same path style the file already uses for `stats::record`.

`systems/mod.rs`: `pub(crate) use stats::{Summary, close_floor as close_stats_floor, current as current_stats, summarize};`

- [ ] **Step 4: Run the tests**

Run: `cargo test` → all pass. Also run `cargo clippy --all-targets -- -D warnings`.

- [ ] **Step 5: Commit**

```bash
git add -A app/src
git commit -m "feat: run stats count kills per enemy kind, gold, hurt and untouched floors"
```

---

### Task 4: `progress` module (pure logic)

**Files:**
- Create: `rust/app/src/progress/mod.rs`, `challenges.rs`, `missions.rs`, `state.rs`
- Modify: `rust/app/src/main.rs` (`mod progress;`)
- Modify: `rust/app/src/enemies.rs` (`EnemyKind` gets `serde::Serialize, serde::Deserialize` and `#[serde(rename_all = "snake_case")]`, like `GunKind`)

**Interfaces:**
- Consumes: `RunRules` (Task 1), `RunStats::kills_of`, `RunStats.{kills, gold, items, untouched_floors}` (Task 3), `EnemyKind::is_boss`, `Role::Boss(name)`, `crate::enemies::FINAL_FLOOR`, `crate::rng::Rng { new, next_u64, below }`
- Produces (all re-exported from `progress/mod.rs`):
  - `ChallengeId` enum: `GlassCannon, PistolOnly, Horde, MerchantsStrike, TwoLeftFeet, Bane` (serde snake_case; `ALL: [Self; 6]`)
  - `Challenge { id, title: &'static str, detail: &'static str, target_floor: u32, reward: u32, accent: [f32; 3], rules: RunRules }` and `ChallengeId::spec(self) -> Challenge`
  - `Goal` enum and `ActiveMission { goal: Goal, target: u32, reward: u32, progress: u32 }` with `title() -> String`, `detail() -> String`, `done() -> bool`
  - `RunRecord { stats: RunStats, floor: u32, victory: bool, challenge: Option<ChallengeId> }`
  - `RunOutcome { missions_done: Vec<String>, challenge: Option<(&'static str, u32)> }`
  - `ProgressState` (serde) with `settle(&mut self, seed: u64)`, `rules() -> RunRules`, `selected() -> Option<ChallengeId>`, `toggle(id)`, `claim(slot) -> Option<u32>`, `finish_run(&RunRecord) -> RunOutcome`, `board() -> BoardView`, `accent() -> Option<[f32; 3]>`
  - `BoardView { missions: Vec<NoteView>, challenges: Vec<NoteView> }`, `NoteView { title: String, detail: String, progress: u32, goal: u32, talents: u32, done: bool, active: bool }`
  - `MISSION_SLOTS: usize = 3`

- [ ] **Step 1: Write `challenges.rs` with its test**

```rust
use serde::{Deserialize, Serialize};

use crate::guns::GunKind;
use crate::rules::RunRules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChallengeId {
    GlassCannon,
    PistolOnly,
    Horde,
    MerchantsStrike,
    TwoLeftFeet,
    Bane,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Challenge {
    pub(crate) id: ChallengeId,
    pub(crate) title: &'static str,
    pub(crate) detail: &'static str,
    pub(crate) target_floor: u32,
    pub(crate) reward: u32,
    pub(crate) accent: [f32; 3],
    pub(crate) rules: RunRules,
}

const HORDE: RunRules = RunRules {
    player_max_health: None,
    player_damage: 1.0,
    enemy_health: 1.6,
    enemy_damage: 1.3,
    only_gun: None,
    shops: true,
    dodge: true,
    armor_regen: true,
};
const NEUTRAL: RunRules = RunRules {
    enemy_health: 1.0,
    enemy_damage: 1.0,
    ..HORDE
};

impl ChallengeId {
    pub(crate) const ALL: [Self; 6] = [
        Self::GlassCannon,
        Self::PistolOnly,
        Self::Horde,
        Self::MerchantsStrike,
        Self::TwoLeftFeet,
        Self::Bane,
    ];

    pub(crate) fn spec(self) -> Challenge {
        let (title, detail, target_floor, reward, accent, rules) = match self {
            Self::GlassCannon => (
                "Glass Cannon",
                "One heart, double damage. Reach floor 3",
                3,
                4,
                [255.0, 70.0, 70.0],
                RunRules { player_max_health: Some(1), player_damage: 2.0, ..NEUTRAL },
            ),
            Self::PistolOnly => (
                "Pistol Only",
                "No gun but the pistol. Reach floor 3",
                3,
                3,
                [230.0, 200.0, 90.0],
                RunRules { only_gun: Some(GunKind::Pistol), ..NEUTRAL },
            ),
            Self::Horde => (
                "Horde",
                "Tougher, harder hitting enemies. Reach floor 4",
                4,
                4,
                [90.0, 200.0, 90.0],
                HORDE,
            ),
            Self::MerchantsStrike => (
                "Merchant's Strike",
                "No shops. Reach floor 4",
                4,
                3,
                [90.0, 160.0, 255.0],
                RunRules { shops: false, ..NEUTRAL },
            ),
            Self::TwoLeftFeet => (
                "Two Left Feet",
                "No dodging. Reach floor 3",
                3,
                3,
                [255.0, 150.0, 40.0],
                RunRules { dodge: false, ..NEUTRAL },
            ),
            Self::Bane => (
                "Bane",
                "Horde without armor regen. Beat the last boss",
                5,
                6,
                [200.0, 30.0, 30.0],
                RunRules { armor_regen: false, ..HORDE },
            ),
        };
        Challenge { id: self, title, detail, target_floor, reward, accent, rules }
    }

    pub(crate) fn reached(self, floor: u32, victory: bool) -> bool {
        let target = self.spec().target_floor;
        if target >= crate::enemies::FINAL_FLOOR {
            victory
        } else {
            floor >= target
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_challenge_changes_the_rules_and_has_a_reachable_target() {
        for id in ChallengeId::ALL {
            let spec = id.spec();
            assert_ne!(spec.rules, RunRules::default(), "{id:?}");
            assert!((2..=crate::enemies::FINAL_FLOOR).contains(&spec.target_floor), "{id:?}");
            assert!(spec.reward > 0);
        }
    }

    #[test]
    fn the_final_floor_target_needs_the_victory_and_others_need_only_the_floor() {
        assert!(ChallengeId::GlassCannon.reached(3, false));
        assert!(!ChallengeId::GlassCannon.reached(2, false));
        assert!(!ChallengeId::Bane.reached(5, false));
        assert!(ChallengeId::Bane.reached(5, true));
    }
}
```
`NEUTRAL` must equal `RunRules::default()`. Add a test assertion `assert_eq!(NEUTRAL, RunRules::default())`.

- [ ] **Step 2: Write `missions.rs` with its tests**

```rust
use serde::{Deserialize, Serialize};

use crate::components::RunStats;
use crate::enemies::{EnemyKind, Role};
use crate::rng::Rng;

const HUNTED: [(EnemyKind, &str); 5] = [
    (EnemyKind::Rat, "rats"),
    (EnemyKind::Bat, "bats"),
    (EnemyKind::Slime, "slimes"),
    (EnemyKind::Ghost, "ghosts"),
    (EnemyKind::Snake, "snakes"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "goal", rename_all = "snake_case")]
pub(crate) enum Goal {
    Kills,
    KillsOf { enemy: EnemyKind },
    Boss { enemy: EnemyKind },
    Floor,
    GoldInRun,
    Gold,
    Items,
    Untouched,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GoalKind {
    Kills,
    KillsOf,
    Boss,
    Floor,
    GoldInRun,
    Gold,
    Items,
    Untouched,
}

impl GoalKind {
    pub(crate) const ALL: [Self; 8] = [
        Self::Kills,
        Self::KillsOf,
        Self::Boss,
        Self::Floor,
        Self::GoldInRun,
        Self::Gold,
        Self::Items,
        Self::Untouched,
    ];
}

impl Goal {
    pub(crate) fn kind(self) -> GoalKind {
        match self {
            Self::Kills => GoalKind::Kills,
            Self::KillsOf { .. } => GoalKind::KillsOf,
            Self::Boss { .. } => GoalKind::Boss,
            Self::Floor => GoalKind::Floor,
            Self::GoldInRun => GoalKind::GoldInRun,
            Self::Gold => GoalKind::Gold,
            Self::Items => GoalKind::Items,
            Self::Untouched => GoalKind::Untouched,
        }
    }

    /// Best-of-a-run goals keep the best run; the rest add every run up.
    fn best_of_a_run(self) -> bool {
        matches!(self, Self::Floor | Self::GoldInRun)
    }

    fn measure(self, stats: &RunStats, floor: u32) -> u32 {
        match self {
            Self::Kills => stats.kills,
            Self::KillsOf { enemy } | Self::Boss { enemy } => stats.kills_of(enemy),
            Self::Floor => floor,
            Self::GoldInRun | Self::Gold => stats.gold,
            Self::Items => stats.items,
            Self::Untouched => stats.untouched_floors,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ActiveMission {
    #[serde(flatten)]
    pub(crate) goal: Goal,
    pub(crate) target: u32,
    pub(crate) reward: u32,
    #[serde(default)]
    pub(crate) progress: u32,
}

fn boss_name(enemy: EnemyKind) -> &'static str {
    match enemy.spec().role {
        Role::Boss(name) => name,
        _ => "boss",
    }
}

impl ActiveMission {
    pub(crate) fn done(&self) -> bool {
        self.progress >= self.target
    }

    pub(crate) fn advance(&mut self, stats: &RunStats, floor: u32) {
        let measured = self.goal.measure(stats, floor);
        self.progress = if self.goal.best_of_a_run() {
            self.progress.max(measured)
        } else {
            self.progress.saturating_add(measured)
        };
    }

    pub(crate) fn title(&self) -> String {
        match self.goal {
            Goal::Kills => "Crypt Cleaner".to_owned(),
            Goal::KillsOf { enemy } => format!("{} Hunter", hunted_name(enemy)),
            Goal::Boss { enemy } => format!("{} Slayer", boss_name(enemy)),
            Goal::Floor => "Deep Delver".to_owned(),
            Goal::GoldInRun => "Gold Digger".to_owned(),
            Goal::Gold => "Hoarder".to_owned(),
            Goal::Items => "Scavenger".to_owned(),
            Goal::Untouched => "Untouchable".to_owned(),
        }
    }

    pub(crate) fn detail(&self) -> String {
        let target = self.target;
        match self.goal {
            Goal::Kills => format!("Defeat {target} enemies"),
            Goal::KillsOf { enemy } => format!("Defeat {target} {}", hunted_name(enemy)),
            Goal::Boss { enemy } => format!("Defeat the {}", boss_name(enemy)),
            Goal::Floor => format!("Reach floor {target}"),
            Goal::GoldInRun => format!("Collect {target} gold in a single run"),
            Goal::Gold => format!("Collect {target} gold"),
            Goal::Items => format!("Pick up {target} items"),
            Goal::Untouched => format!("Clear {target} floors without taking damage"),
        }
    }
}

fn hunted_name(enemy: EnemyKind) -> &'static str {
    HUNTED
        .iter()
        .find(|(kind, _)| *kind == enemy)
        .map_or("enemies", |(_, name)| name)
}

fn tier(rng: &mut Rng, targets: [u32; 3]) -> (u32, u32) {
    let index = rng.below(3);
    (targets[index], index as u32 + 1)
}

pub(crate) fn draw(rng: &mut Rng, taken: &[GoalKind]) -> ActiveMission {
    let free: Vec<GoalKind> = GoalKind::ALL
        .into_iter()
        .filter(|kind| !taken.contains(kind))
        .collect();
    let kind = free[rng.below(free.len())];
    let (goal, target, reward) = match kind {
        GoalKind::Kills => {
            let (target, reward) = tier(rng, [50, 120, 250]);
            (Goal::Kills, target, reward)
        }
        GoalKind::KillsOf => {
            let enemy = HUNTED[rng.below(HUNTED.len())].0;
            let (target, reward) = tier(rng, [10, 25, 50]);
            (Goal::KillsOf { enemy }, target, reward)
        }
        GoalKind::Boss => {
            let bosses: Vec<EnemyKind> =
                EnemyKind::ALL.into_iter().filter(|kind| kind.is_boss()).collect();
            (Goal::Boss { enemy: bosses[rng.below(bosses.len())] }, 1, 2)
        }
        GoalKind::Floor => {
            let (target, reward) = tier(rng, [2, 3, 4]);
            (Goal::Floor, target, reward)
        }
        GoalKind::GoldInRun => {
            let (target, reward) = tier(rng, [40, 80, 140]);
            (Goal::GoldInRun, target, reward)
        }
        GoalKind::Gold => {
            let (target, reward) = tier(rng, [150, 300, 600]);
            (Goal::Gold, target, reward)
        }
        GoalKind::Items => {
            let (target, reward) = tier(rng, [15, 30, 60]);
            (Goal::Items, target, reward)
        }
        GoalKind::Untouched => {
            let (target, reward) = tier(rng, [1, 2, 3]);
            (Goal::Untouched, target, reward)
        }
    };
    ActiveMission { goal, target, reward, progress: 0 }
}
```
`taken` must leave at least one free kind. With 3 slots and 8 kinds it always does.

Tests at the bottom of `missions.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn stats(kills_of_rats: u32, gold: u32) -> RunStats {
        let mut stats = RunStats::default();
        stats.kills = kills_of_rats;
        stats.kills_of[EnemyKind::Rat.index()] = kills_of_rats;
        stats.gold = gold;
        stats
    }

    #[test]
    fn totals_add_up_across_runs_and_best_of_a_run_goals_keep_the_best() {
        let mut total = ActiveMission { goal: Goal::Gold, target: 100, reward: 1, progress: 0 };
        let mut best = ActiveMission { goal: Goal::GoldInRun, target: 100, reward: 1, progress: 0 };
        for gold in [60, 30] {
            total.advance(&stats(0, gold), 1);
            best.advance(&stats(0, gold), 1);
        }
        assert_eq!(total.progress, 90);
        assert_eq!(best.progress, 60);
        assert!(!total.done());
    }

    #[test]
    fn kind_goals_count_only_their_kind_and_the_floor_goal_reads_the_floor() {
        let mut rats = ActiveMission { goal: Goal::KillsOf { enemy: EnemyKind::Rat }, target: 3, reward: 1, progress: 0 };
        let mut bats = ActiveMission { goal: Goal::KillsOf { enemy: EnemyKind::Bat }, target: 3, reward: 1, progress: 0 };
        let mut floor = ActiveMission { goal: Goal::Floor, target: 3, reward: 1, progress: 0 };
        rats.advance(&stats(4, 0), 3);
        bats.advance(&stats(4, 0), 3);
        floor.advance(&stats(4, 0), 3);
        assert!(rats.done() && floor.done());
        assert_eq!(bats.progress, 0);
    }

    #[test]
    fn a_draw_never_repeats_a_kind_that_is_taken() {
        let mut rng = Rng::new(11);
        let taken = [GoalKind::Kills, GoalKind::Gold];
        for _ in 0..200 {
            assert!(!taken.contains(&draw(&mut rng, &taken).goal.kind()));
        }
    }

    #[test]
    fn hunted_enemies_turn_up_somewhere_in_the_dungeon() {
        for (kind, _) in HUNTED {
            let somewhere = (1..=crate::enemies::FINAL_FLOOR).any(|floor| {
                crate::dungeon::Theme::ALL.iter().any(|theme| kind.available_in(floor, *theme))
            });
            assert!(somewhere, "{kind:?}");
        }
    }

    #[test]
    fn missions_survive_a_round_trip_through_json() {
        let mission = ActiveMission { goal: Goal::Boss { enemy: EnemyKind::Golem }, target: 1, reward: 2, progress: 0 };
        let json = serde_json::to_string(&mission).expect("json");
        assert_eq!(serde_json::from_str::<ActiveMission>(&json).expect("parsed"), mission);
    }
}
```
If `Theme::ALL` does not exist, list the themes from `dungeon/theme.rs`. If a hunted kind fails the "turn up somewhere" test, swap it for one that passes.

- [ ] **Step 3: Write `state.rs` with its tests**

```rust
use std::collections::BTreeSet;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};

use super::challenges::ChallengeId;
use super::missions::{self, ActiveMission};
use crate::components::RunStats;
use crate::rng::Rng;
use crate::rules::RunRules;

pub(crate) const MISSION_SLOTS: usize = 3;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct ProgressState {
    #[serde(deserialize_with = "known")]
    missions: Vec<ActiveMission>,
    #[serde(deserialize_with = "known_option")]
    selected: Option<ChallengeId>,
    #[serde(deserialize_with = "known")]
    completed: BTreeSet<ChallengeId>,
    seed: u64,
    draws: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct RunRecord {
    pub(crate) stats: RunStats,
    pub(crate) floor: u32,
    pub(crate) victory: bool,
    pub(crate) challenge: Option<ChallengeId>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct RunOutcome {
    pub(crate) missions_done: Vec<String>,
    pub(crate) challenge: Option<(&'static str, u32)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NoteView {
    pub(crate) title: String,
    pub(crate) detail: String,
    pub(crate) progress: u32,
    pub(crate) goal: u32,
    pub(crate) talents: u32,
    pub(crate) done: bool,
    pub(crate) active: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct BoardView {
    pub(crate) missions: Vec<NoteView>,
    pub(crate) challenges: Vec<NoteView>,
}

fn known<'de, D, T, C>(deserializer: D) -> Result<C, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
    C: FromIterator<T>,
{
    let values = Vec::<serde_json::Value>::deserialize(deserializer)?;
    Ok(values
        .into_iter()
        .filter_map(|value| match serde_json::from_value(value.clone()) {
            Ok(parsed) => Some(parsed),
            Err(error) => {
                tracing::warn!(%error, %value, "skipping an unknown progress entry");
                None
            }
        })
        .collect())
}

fn known_option<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(value.and_then(|value| serde_json::from_value(value).ok()))
}

impl ProgressState {
    /// Gives a fresh profile its seed and keeps the board full; call after loading.
    pub(crate) fn settle(&mut self, seed: u64) {
        if self.seed == 0 {
            self.seed = seed.max(1);
        }
        self.missions.truncate(MISSION_SLOTS);
        while self.missions.len() < MISSION_SLOTS {
            let mission = self.draw();
            self.missions.push(mission);
        }
    }

    fn draw(&mut self) -> ActiveMission {
        let taken: Vec<_> = self.missions.iter().map(|mission| mission.goal.kind()).collect();
        let mut rng = Rng::new(self.seed ^ self.draws.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        self.draws += 1;
        missions::draw(&mut rng, &taken)
    }

    pub(crate) fn selected(&self) -> Option<ChallengeId> {
        self.selected
    }

    pub(crate) fn rules(&self) -> RunRules {
        self.selected.map_or_else(RunRules::default, |id| id.spec().rules)
    }

    pub(crate) fn accent(&self) -> Option<[f32; 3]> {
        self.selected.map(|id| id.spec().accent)
    }

    pub(crate) fn toggle(&mut self, id: ChallengeId) {
        self.selected = if self.selected == Some(id) { None } else { Some(id) };
    }

    pub(crate) fn claim(&mut self, slot: usize) -> Option<u32> {
        let mission = *self.missions.get(slot)?;
        if !mission.done() {
            return None;
        }
        self.missions.remove(slot);
        let fresh = self.draw();
        self.missions.insert(slot, fresh);
        Some(mission.reward)
    }

    pub(crate) fn finish_run(&mut self, record: &RunRecord) -> RunOutcome {
        let mut outcome = RunOutcome::default();
        for mission in &mut self.missions {
            let was_done = mission.done();
            mission.advance(&record.stats, record.floor);
            if mission.done() && !was_done {
                outcome.missions_done.push(mission.title());
            }
        }
        if let Some(id) = record.challenge {
            if !self.completed.contains(&id) && id.reached(record.floor, record.victory) {
                self.completed.insert(id);
                let spec = id.spec();
                outcome.challenge = Some((spec.title, spec.reward));
            }
        }
        outcome
    }

    pub(crate) fn board(&self) -> BoardView {
        BoardView {
            missions: self
                .missions
                .iter()
                .map(|mission| NoteView {
                    title: mission.title(),
                    detail: mission.detail(),
                    progress: mission.progress.min(mission.target),
                    goal: mission.target,
                    talents: mission.reward,
                    done: mission.done(),
                    active: false,
                })
                .collect(),
            challenges: ChallengeId::ALL
                .into_iter()
                .map(|id| {
                    let spec = id.spec();
                    let done = self.completed.contains(&id);
                    NoteView {
                        title: spec.title.to_owned(),
                        detail: spec.detail.to_owned(),
                        progress: u32::from(done),
                        goal: 1,
                        talents: spec.reward,
                        done,
                        active: self.selected == Some(id),
                    }
                })
                .collect(),
        }
    }
}
```

Tests at the bottom of `state.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::enemies::EnemyKind;

    fn settled() -> ProgressState {
        let mut state = ProgressState::default();
        state.settle(42);
        state
    }

    fn record(floor: u32, victory: bool, challenge: Option<ChallengeId>) -> RunRecord {
        let mut stats = RunStats::default();
        stats.kills = 500;
        stats.kills_of[EnemyKind::Rat.index()] = 500;
        stats.gold = 1000;
        stats.items = 100;
        stats.untouched_floors = 3;
        RunRecord { stats, floor, victory, challenge }
    }

    #[test]
    fn a_fresh_board_has_three_missions_of_different_kinds() {
        let state = settled();
        let kinds: Vec<_> = state.missions.iter().map(|m| m.goal.kind()).collect();
        assert_eq!(kinds.len(), MISSION_SLOTS);
        assert!(kinds.iter().enumerate().all(|(i, k)| !kinds[i + 1..].contains(k)));
    }

    #[test]
    fn claiming_pays_once_and_pins_a_fresh_mission_in_the_same_slot() {
        let mut state = settled();
        assert_eq!(state.claim(0), None, "nothing is done yet");
        state.missions[0].progress = state.missions[0].target;
        let reward = state.missions[0].reward;
        assert_eq!(state.claim(0), Some(reward));
        assert_eq!(state.missions.len(), MISSION_SLOTS);
        assert!(!state.missions[0].done());
        assert_eq!(state.claim(7), None);
    }

    #[test]
    fn a_finished_run_moves_the_missions_and_reports_the_new_ones() {
        let mut state = settled();
        let outcome = state.finish_run(&record(4, false, None));
        assert!(!outcome.missions_done.is_empty());
        assert_eq!(outcome.challenge, None);
    }

    #[test]
    fn a_challenge_pays_only_the_first_time_and_only_at_its_floor() {
        let mut state = settled();
        state.toggle(ChallengeId::Horde);
        assert_eq!(state.finish_run(&record(3, false, Some(ChallengeId::Horde))).challenge, None);
        assert_eq!(
            state.finish_run(&record(4, false, Some(ChallengeId::Horde))).challenge,
            Some(("Horde", 4))
        );
        assert_eq!(state.finish_run(&record(5, true, Some(ChallengeId::Horde))).challenge, None);
        assert!(state.board().challenges.iter().any(|note| note.title == "Horde" && note.done));
    }

    #[test]
    fn toggling_selects_switches_and_cancels() {
        let mut state = settled();
        assert_eq!(state.rules(), RunRules::default());
        assert_eq!(state.accent(), None);
        state.toggle(ChallengeId::Horde);
        assert_eq!(state.selected(), Some(ChallengeId::Horde));
        state.toggle(ChallengeId::Bane);
        assert_eq!(state.selected(), Some(ChallengeId::Bane));
        assert_eq!(state.rules(), ChallengeId::Bane.spec().rules);
        state.toggle(ChallengeId::Bane);
        assert_eq!(state.selected(), None);
        assert_eq!(state.board().challenges.iter().filter(|note| note.active).count(), 0);
    }

    #[test]
    fn unknown_entries_in_a_save_are_dropped_and_the_board_refills() {
        let json = r#"{
            "missions": [{"goal": "kills", "target": 50, "reward": 1, "progress": 7},
                         {"goal": "removed_goal", "target": 1, "reward": 1}],
            "selected": "removed_challenge",
            "completed": ["horde", "removed_challenge"],
            "seed": 5
        }"#;
        let mut state: ProgressState = serde_json::from_str(json).expect("parsed");
        state.settle(9);
        assert_eq!(state.missions.len(), MISSION_SLOTS);
        assert_eq!(state.missions[0].progress, 7);
        assert_eq!(state.selected(), None);
        assert_eq!(state.completed, BTreeSet::from([ChallengeId::Horde]));
    }
}
```

`progress/mod.rs`:

```rust
mod challenges;
mod missions;
mod state;

pub(crate) use challenges::ChallengeId;
pub(crate) use state::{BoardView, NoteView, ProgressState, RunOutcome, RunRecord};
```
If dead-code warnings show up because later tasks are not wired yet, add `#[allow(dead_code)]` to the `mod` lines in `main.rs`. Remove those attributes in Task 6.

- [ ] **Step 4: Run the tests**

Run: `cargo test progress::` → all pass. Then run `cargo test` and `cargo clippy --all-targets -- -D warnings`.

- [ ] **Step 5: Commit**

```bash
git add -A app/src
git commit -m "feat: progress module with rotating missions and one-time challenges"
```

---

### Task 5: Persist progress in the profile

**Files:**
- Modify: `rust/app/src/profile.rs`

**Interfaces:**
- Consumes: `ProgressState`, `RunRecord`, `RunOutcome`, `BoardView`, `ChallengeId` (Task 4)
- Produces on `SharedProfile`:
  - `rules(&self) -> RunRules`
  - `challenge(&self) -> Option<ChallengeId>`
  - `accent(&self) -> Option<[f32; 3]>`
  - `board(&self) -> BoardView`
  - `toggle_challenge(&self, id: ChallengeId)`
  - `claim_mission(&self, slot: usize) -> Option<u32>` (adds to `balance`)
  - `finish_run(&self, record: &RunRecord) -> RunOutcome` (adds the challenge reward to `balance`)

- [ ] **Step 1: Write the failing tests** in `profile.rs` tests (reuse `temp_path`, `run`):

```rust
    #[test]
    fn a_profile_from_before_progress_loads_with_a_full_board() {
        let path = temp_path("no-progress");
        fs::write(&path, r#"{"version": 1, "balance": 30}"#).expect("a temp file");
        let shared = SharedProfile::load(&path);
        assert_eq!(shared.balance(), 30);
        assert_eq!(shared.board().missions.len(), 3);
        assert_eq!(shared.challenge(), None);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn a_garbage_progress_keeps_the_rest_of_the_profile() {
        let path = temp_path("bad-progress");
        fs::write(&path, r#"{"version": 1, "balance": 30, "progress": 12}"#).expect("a temp file");
        let shared = SharedProfile::load(&path);
        assert_eq!(shared.balance(), 30);
        assert_eq!(shared.board().missions.len(), 3);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn the_selected_challenge_and_its_reward_survive_a_restart() {
        let path = temp_path("challenge");
        let _ = fs::remove_file(&path);
        let shared = SharedProfile::load(&path);
        shared.toggle_challenge(ChallengeId::Horde);
        let (stats, _) = run(1, 4);
        let outcome = shared.finish_run(&RunRecord { stats, floor: 4, victory: false, challenge: shared.challenge() });
        assert_eq!(outcome.challenge, Some(("Horde", 4)));
        assert_eq!(shared.balance(), 4);

        let reloaded = SharedProfile::load(&path);
        assert_eq!(reloaded.challenge(), Some(ChallengeId::Horde));
        assert_eq!(reloaded.balance(), 4);
        assert!(reloaded.board().challenges.iter().any(|note| note.title == "Horde" && note.done));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn claiming_a_mission_pays_talents_into_the_balance() {
        let shared = SharedProfile::in_memory();
        assert_eq!(shared.claim_mission(0), None);
        let mut stats = RunStats::default();
        stats.kills = 10_000;
        stats.gold = 10_000;
        stats.items = 10_000;
        stats.untouched_floors = 10;
        for kind in crate::enemies::EnemyKind::ALL {
            stats.kills_of[kind.index()] = 10_000;
        }
        shared.finish_run(&RunRecord { stats, floor: 5, victory: true, challenge: None });
        let paid = shared.claim_mission(0).expect("a finished mission");
        assert_eq!(shared.balance(), u64::from(paid));
    }
```

- [ ] **Step 2: Run them to see them fail**

Run: `cargo test profile::` → compile errors.

- [ ] **Step 3: Implement**

In `Profile` add:

```rust
    #[serde(deserialize_with = "lenient_progress")]
    pub(crate) progress: ProgressState,
```
with

```rust
fn lenient_progress<'de, D: Deserializer<'de>>(deserializer: D) -> Result<ProgressState, D::Error> {
    let value = serde_json::Value::deserialize(deserializer)?;
    Ok(serde_json::from_value(value).unwrap_or_else(|error| {
        tracing::warn!(%error, "the saved progress is unreadable, starting it afresh");
        ProgressState::default()
    }))
}
```
`Profile` derives `Eq`, and `ProgressState` derives `Eq` too, so the derive still works.

In `SharedProfile::load`, after `Profile::load(path)`: `profile.progress.settle(Rng::from_clock().next_u64());` (make `profile` `mut`). In `in_memory()`, use `settle(1)` so tests are deterministic.

Add a small private helper so every mutating method saves the same way:

```rust
    fn change<T>(&self, change: impl FnOnce(&mut Profile) -> T) -> T {
        let mut store = self.0.borrow_mut();
        let result = change(&mut store.profile);
        if let Some(path) = &store.path {
            store.profile.save(path);
        }
        result
    }
```
Then:

```rust
    pub(crate) fn rules(&self) -> RunRules { self.0.borrow().profile.progress.rules() }
    pub(crate) fn challenge(&self) -> Option<ChallengeId> { self.0.borrow().profile.progress.selected() }
    pub(crate) fn accent(&self) -> Option<[f32; 3]> { self.0.borrow().profile.progress.accent() }
    pub(crate) fn board(&self) -> BoardView { self.0.borrow().profile.progress.board() }

    pub(crate) fn toggle_challenge(&self, id: ChallengeId) {
        self.change(|profile| profile.progress.toggle(id));
    }

    pub(crate) fn claim_mission(&self, slot: usize) -> Option<u32> {
        self.change(|profile| {
            let paid = profile.progress.claim(slot)?;
            profile.balance += u64::from(paid);
            Some(paid)
        })
    }

    pub(crate) fn finish_run(&self, record: &RunRecord) -> RunOutcome {
        self.change(|profile| {
            let outcome = profile.progress.finish_run(record);
            if let Some((_, talents)) = outcome.challenge {
                profile.balance += u64::from(talents);
            }
            outcome
        })
    }
```
Format these one-liners the way `rustfmt` wants. Existing mutators may switch to `change` too, but don't refactor them in this task.

- [ ] **Step 4: Run the tests**

Run: `cargo test` → all pass (including the older profile tests). Also run `cargo clippy --all-targets -- -D warnings`.

- [ ] **Step 5: Commit**

```bash
git add app/src/profile.rs
git commit -m "feat: the profile keeps missions and challenges and pays their talents"
```

---

### Task 6: Wire runs to rules and progress

**Files:**
- Modify: `rust/app/src/scenes/ingame.rs` (`start_run` ~513, `build` ~530, `next_floor` ~670, `handle_room_events` BossDefeated ~720, `start_death_screen` ~523, simulation step ~470, struct fields ~50)
- Modify: `rust/app/src/systems/death_screen.rs`, `rust/app/src/prefabs/ui.rs` (`death_ui`), `rust/app/src/components/death.rs`

**Interfaces:**
- Consumes: `SharedProfile::{rules, challenge, finish_run}` (Task 5), `RunRules::shape` (Task 1), `systems::toughen_enemies`, `Dungeon::generate_for_run` (Task 2), `systems::close_stats_floor` (Task 3)
- Produces: `InGameScene` fields `rules: RunRules`, `challenge: Option<ChallengeId>`; `DeathScreen.note: Option<String>`; `DeathPart::Note`

- [ ] **Step 1: Write the failing tests** in `scenes/ingame.rs` tests (next to `fingerprint`):

```rust
    #[test]
    fn a_challenge_run_starts_with_its_rules_and_neutral_runs_match_today() {
        let assets = /* same Assets setup as the fingerprint test */;
        let profile = SharedProfile::in_memory();
        let mut plain = InGameScene::default();
        plain.attach_profile(profile.clone());
        assert_eq!(fingerprint_of(&mut plain, 7, &assets), fingerprint(7, &assets));

        profile.toggle_challenge(ChallengeId::GlassCannon);
        let mut scene = InGameScene::default();
        scene.attach_profile(profile);
        scene.start_run(7, &assets);
        let max = scene
            .world
            .query::<&Health>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|health| health.max);
        assert_eq!(max, Some(1));
        assert_eq!(scene.challenge, Some(ChallengeId::GlassCannon));
    }

    #[test]
    fn glass_cannon_stays_at_one_heart_on_the_next_floor() {
        // same setup with GlassCannon selected, then scene.next_floor(&assets)
        // and assert the player's Health.max is still 1
    }

    #[test]
    fn merchants_strike_floors_have_no_shop() {
        // select MerchantsStrike, start_run(7), and assert that
        // scene.world.query::<&Shop>().iter().count() == 0
    }
```
`fingerprint_of` is a small refactor of the existing `fingerprint` helper that takes an already prepared scene. Keep `fingerprint` itself unchanged so the old test is untouched. Fill in the two commented test bodies with the same setup code as the first test; they are three lines each.

- [ ] **Step 2: Run them to see them fail**

Run: `cargo test ingame` → fails (no `challenge` field, health not capped).

- [ ] **Step 3: Implement**

Fields on `InGameScene`: `rules: RunRules` and `challenge: Option<ChallengeId>`. Both have a `Default`, so `#[derive(Default)]` on the scene keeps working. Add a helper:

```rust
    fn perks(&self) -> Perks {
        let owned = self.profile.as_ref().map_or_else(Perks::default, SharedProfile::perks);
        self.rules.shape(owned)
    }
```

`start_run`: before `self.build(assets)` set

```rust
        self.rules = self.profile.as_ref().map_or_else(RunRules::default, SharedProfile::rules);
        self.challenge = self.profile.as_ref().and_then(SharedProfile::challenge);
```
and replace the `perks` computation with `let perks = self.perks();`.

`build`: replace `Dungeon::generate_themed(&mut self.map_rng, theme)` with `Dungeon::generate_for_run(&mut self.map_rng, theme, self.rules.shops)`. Replace the `let perks = self.profile...` block with `let perks = self.perks();`.

Simulation step (`fixed_update`, right before `systems::resolve_deaths(...)` near line 470): `systems::toughen_enemies(&mut self.world, self.rules.enemy_health);`

`next_floor`: first line `systems::close_stats_floor(&mut self.world);` (before `carried_stats`).

`handle_room_events`, `BossDefeated`, inside `if self.floor >= FINAL_FLOOR`: `systems::close_stats_floor(&mut self.world);` before `self.victory = true;`.

`start_death_screen`:

```rust
        let stats = systems::current_stats(&self.world);
        let summary = systems::summarize(stats, self.floor);
        let mut note = None;
        if let Some(profile) = &self.profile {
            profile.record_run(stats, &summary, self.victory);
            let outcome = profile.finish_run(&RunRecord {
                stats,
                floor: self.floor,
                victory: self.victory,
                challenge: self.challenge,
            });
            note = progress_note(&outcome);
        }
        prefabs::death_ui(&mut self.world, self.victory, &assets.textures);
        self.death = Some(systems::DeathScreen::new(summary).with_note(note));
```
with a free function in `ingame.rs`:

```rust
fn progress_note(outcome: &RunOutcome) -> Option<String> {
    let mut parts = Vec::new();
    if let Some((title, talents)) = outcome.challenge {
        parts.push(format!("{title} complete! +{talents} talents"));
    }
    match outcome.missions_done.len() {
        0 => {}
        1 => parts.push("1 mission ready on the board".to_owned()),
        count => parts.push(format!("{count} missions ready on the board")),
    }
    (!parts.is_empty()).then(|| parts.join("   "))
}
```
Add a unit test for `progress_note` (empty outcome gives `None`; a challenge plus 2 missions gives both parts).

Death screen: `components/death.rs` adds the variant `Note`. `DeathScreen` gets `pub(crate) note: Option<String>` (`None` in `new`) and

```rust
    pub(crate) fn with_note(mut self, note: Option<String>) -> Self {
        self.note = note;
        self
    }
```
`prefabs::death_ui` spawns one more text entity like the `Prompt` one, but with `DeathPart::Note`, position `Vec2::new(TARGET.x / 2.0, NOTE_BASELINE)`, the gold colour `Vec4::new(1.0, 216.0 / 255.0, 0.0, 1.0)`, `FontSize::Size22`, `.aligned(Align::Center)` and empty content. Add `pub(crate) const NOTE_BASELINE: f32 = 450.0;` in `death_screen.rs` next to `PROMPT_POSITION`. `update_death_ui` arm:

```rust
            DeathPart::Note => match &screen.note {
                Some(note) if screen.prompt_visible() => {
                    if let Some(text) = text {
                        text.content.clone_from(note);
                    }
                    true
                }
                _ => false,
            },
```
Add a test in `death_screen.rs` next to the existing timing tests: a screen with a note shows the `Note` part once `prompt_visible()`, and a screen without one never does.

- [ ] **Step 4: Run the tests**

Run: `cargo test` → all pass, especially the untouched `the_same_seed_builds_the_same_floor_with_the_same_shops_and_traps`. Also run `cargo clippy --all-targets -- -D warnings`, and remove any `#[allow(dead_code)]` added in Task 4.

- [ ] **Step 5: Commit**

```bash
git add -A app/src
git commit -m "feat: runs start with the chosen challenge's rules and report progress at the end"
```

---

### Task 7: Quest board and portal in the lobby

**Files:**
- Modify: `rust/app/src/scenes/quest_board.rs`
- Modify: `rust/app/src/scenes/lobby.rs`
- Create: `rust/app/src/systems/portal.rs` (+ `mod portal; pub(crate) use portal::tint_portal;` in `systems/mod.rs`)

**Interfaces:**
- Consumes: `SharedProfile::{board, toggle_challenge, claim_mission, accent}` (Task 5), `BoardView`, `NoteView`, `ChallengeId::ALL` (Task 4)
- Produces:
  - `quest_board::layout(tab: Tab, view: &BoardView, hover: Option<Target>) -> Vec<Element>`
  - `QuestBoard::update(&mut self, world, input, assets, profile: &SharedProfile) -> Option<BoardAction>`
  - `pub(crate) enum BoardAction { Claimed(u32), Toggled }`
  - `systems::tint_portal(world: &mut World, accent: Option<[f32; 3]>)`

- [ ] **Step 1: Write the failing tests**

`quest_board.rs`: replace `TASKS` / `CHALLENGES` in the existing tests with a fixture view:

```rust
    fn view() -> BoardView {
        let note = |title: &str, done, active| NoteView {
            title: title.to_owned(),
            detail: "Defeat 100 enemies".to_owned(),
            progress: if done { 1 } else { 0 },
            goal: 1,
            talents: 2,
            done,
            active,
        };
        BoardView {
            missions: vec![note("Crypt Cleaner", false, false), note("Gold Digger", true, false), note("Deep Delver", false, false)],
            challenges: vec![note("Horde", false, true), note("Bane", true, false)],
        }
    }
```
Update the five existing tests to call `layout(tab, &view(), hover)` and to compare counts with `view().missions.len()` / `view().challenges.len()`. Change `every_paper_fits_on_the_board` to loop over `0..6` (6 challenges = 2 rows). Add:

```rust
    #[test]
    fn the_active_challenge_is_marked_and_the_others_are_not() {
        let marks = labels(&layout(Tab::Challenges, &view(), None))
            .iter()
            .filter(|label| *label == ACTIVE_MARK)
            .count();
        assert_eq!(marks, 1);
        assert!(!labels(&layout(Tab::Tasks, &view(), None)).contains(&ACTIVE_MARK.to_owned()));
    }
```

`systems/portal.rs`:

```rust
#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::assets::Textures;
    use crate::components::{Color, Light};
    use crate::prefabs;

    use super::*;

    #[test]
    fn the_portal_takes_the_accent_and_returns_to_purple_without_one() {
        let mut world = World::new();
        let portal = prefabs::portal(&mut world, Vec2::ZERO, &Textures::default());
        tint_portal(&mut world, Some([255.0, 0.0, 0.0]));
        let red = world.get::<&Light>(portal).expect("light").color;
        assert!(red.x > red.z);
        tint_portal(&mut world, None);
        assert_eq!(world.get::<&Light>(portal).expect("light").color, PORTAL_LIGHT / 255.0);
        assert_eq!(world.get::<&Color>(portal).expect("color").0, glam::Vec4::ONE);
    }
}
```

`scenes/lobby.rs` test (follow the existing click test near line 487–540 that uses `SharedProfile::in_memory()` and a click `Input`): open the quests board, switch to the Challenges tab, click the first challenge note, and assert `profile.challenge() == Some(ChallengeId::ALL[0])`. Click it again and assert `None`. Look at how the existing test builds a click at a point and reuse it; the note centre is `note_top_left(0) + Vec2::splat(20.0)` (make `note_top_left` and `tab_top_left` `pub(crate)` for the test if needed).

- [ ] **Step 2: Run them to see them fail**

Run: `cargo test quest_board portal lobby` → compile errors.

- [ ] **Step 3: Implement**

`quest_board.rs`:
- Delete `struct Entry`, `TASKS`, `CHALLENGES` and `entries()`. Add `fn notes(tab: Tab, view: &BoardView) -> &[NoteView]`.
- `pub(crate) const ACTIVE_MARK: &str = "Active";`
- `target_at(tab, view, point)` iterates `0..notes(tab, view).len()`.
- `layout(tab, view, hover)` is the existing loop over `notes(tab, view)` with `entry.*` replaced by `note.*`. `done` is `note.done`. After the title, if `note.active` push `panel::label(ACTIVE_MARK, origin + Vec2::new(VISIBLE.x / 2.0, 28.0), GREEN, FontSize::Size16, Align::Center)`.
- `QuestBoard` keeps `view: BoardView` (fill it in `toggle` when opening and after every action). `toggle(&mut self, world, assets, profile: &SharedProfile)` loads `profile.board()` before `rebuild`.
- `update(..., profile: &SharedProfile) -> Option<BoardAction>`: on click,
  - `Target::Tab(tab)` → as today.
  - `Target::Note(index)` on `Tab::Tasks` → `profile.claim_mission(index).map(BoardAction::Claimed)`.
  - `Target::Note(index)` on `Tab::Challenges` → `profile.toggle_challenge(ChallengeId::ALL[index])` and `Some(BoardAction::Toggled)`.
  - After any action, set `self.view = profile.board()` and `self.dirty = true`.

`systems/portal.rs`:

```rust
use glam::{Vec3, Vec4};
use hecs::World;

use crate::components::{Color, Interactable, Interaction, Light};

pub(crate) const PORTAL_LIGHT: Vec3 = Vec3::new(128.0, 0.0, 128.0);

pub(crate) fn tint_portal(world: &mut World, accent: Option<[f32; 3]>) {
    let light = accent.map_or(PORTAL_LIGHT, Vec3::from) / 255.0;
    let tint = accent.map_or(Vec4::ONE, |[red, green, blue]| {
        Vec4::new(0.5 + red / 510.0, 0.5 + green / 510.0, 0.5 + blue / 510.0, 1.0)
    });
    for (interactable, color, glow) in world.query_mut::<(&Interactable, &mut Color, &mut Light)>() {
        if interactable.action == Interaction::EnterPortal {
            color.0 = tint;
            glow.color = light;
        }
    }
}
```
Change `prefabs::portal` to build its light from the constant: `Light::rgb(Vec2::ZERO, PORTAL_LIGHT.x, PORTAL_LIGHT.y, PORTAL_LIGHT.z, 50.0, 2.5)`. Check how `Interactable`/`Interaction` are imported in `prefabs/fixtures.rs` and use the same paths.

`scenes/lobby.rs`:
- In `enter`/build right after `prefabs::portal(...)`: `systems::tint_portal(&mut self.world, self.profile.as_ref().and_then(SharedProfile::accent));`
- `Some(Interaction::Quests)` branch: pass the profile to `toggle`. If there is no profile, do nothing (the board needs one).
- In `update` where `self.quests.update(...)` is called:

```rust
            if let Some(profile) = self.profile.clone() {
                match self.quests.update(&mut self.world, input, assets, &profile) {
                    Some(BoardAction::Claimed(talents)) => {
                        self.refresh_wallet();
                        self.refresh_counters();
                        systems::say(&mut self.world, &format!("+{talents} talents!"));
                        self.sounds.push(AudioCommand::Effect(SoundEffect::Coin));
                    }
                    Some(BoardAction::Toggled) => {
                        systems::tint_portal(&mut self.world, profile.accent());
                        self.sounds.push(AudioCommand::Effect(SoundEffect::Click));
                    }
                    None => {}
                }
            }
```
Check the signature of `systems::say`. If it takes `&'static str`, use a fixed line ("Talents earned!") instead of `format!`.

- [ ] **Step 4: Run the tests**

Run: `cargo test` → all pass. Also run `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`.

- [ ] **Step 5: Manual check** (REQUIRED SUB-SKILL: the `run` skill, or `cargo run --release` from `rust/`). Then:
  - The board shows 3 missions with real titles.
  - Clicking a challenge recolours the portal, clicking it again restores the purple, and the selection survives a game restart.
  - A Glass Cannon run starts with 1 heart.
  - The death screen shows the note line after a run that finished a mission.

- [ ] **Step 6: Commit**

```bash
git add -A app/src
git commit -m "feat: lobby quest board claims missions and picks challenges, the portal wears the challenge colour"
```

---

## Self-Review Notes

- **Spec coverage.** Missions are covered by Tasks 3–5 and 7, challenges by Tasks 1, 2 and 4–7, the portal by Task 7, persistence by Task 5, the death screen by Task 6, and decoupling by the `rules.rs`/`progress/` boundaries. The two spec deviations are listed at the top.
- **Type names used across tasks.** `RunRules`, `Perks.{max_health_cap,dodge,only_gun}`, `Loadout::{restrict,allows}`, `toughen_enemies`, `generate_for_run`, `RunStats.{kills_of,gold,hurt_this_floor,untouched_floors}`, `Stat::{Kill(EnemyKind),Gold,Hurt}`, `close_stats_floor`, `ChallengeId`, `RunRecord`, `RunOutcome`, `BoardView`, `NoteView`, `ProgressState::{settle,rules,selected,accent,toggle,claim,finish_run,board}`, `SharedProfile::{rules,challenge,accent,board,toggle_challenge,claim_mission,finish_run}`, `BoardAction::{Claimed,Toggled}`, `tint_portal`, `PORTAL_LIGHT`, `ACTIVE_MARK`, `DeathPart::Note`, `DeathScreen::with_note`.
