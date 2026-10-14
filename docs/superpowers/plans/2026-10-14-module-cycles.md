# Break the Module Dependency Cycles Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove both dependency cycles between the top-level modules of `rust/app/src` and leave `KNOWN_CYCLES` in `architecture.rs` empty, so any new cycle fails `cargo test`.

**Architecture:** The cycles are held up by a few "backward" imports, not by real two-way coupling. Each is fixed by moving a small data definition to the module that should own it, so dependencies point one way: `scenes → systems → prefabs → components → domain modules (guns, items, enemies, dungeon, …) → assets → leaf modules (clip, sheets)`. No behaviour changes; values move verbatim.

**Tech Stack:** Rust 2024, hecs ECS, `cargo test` (the guard lives in `rust/app/src/architecture.rs`).

**Spec:** No separate spec. The requirement is the user's: "decouple the modules so there are no circular dependencies". The current cycles come from `cargo test print_module_cycles -- --ignored --nocapture` (output pasted under *Starting point*).

## Starting point

```
cycle of 10 modules:
  assets -> components   (assets.rs:8)        BarKind
  assets -> dungeon      (assets.rs:9)        Theme
  enemies -> components  (enemies.rs:4)       Clip
  items -> components    (items.rs:4)         Clip
  components -> artefacts, assets, dungeon, enemies, guns, items, upgrades, weapon_upgrades
  dungeon/guns/items/upgrades/enemies/artefacts -> assets
  weapon_tree -> guns; weapon_upgrades -> guns, upgrades, weapon_tree

cycle of 2 modules:
  prefabs -> systems   (prefabs/ui.rs:12, prefabs/fixtures.rs:22, prefabs/actors.rs:19, :439, :489)
  systems -> prefabs   (many, the normal direction)
```

`components → domain` and `domain → assets` are the normal direction. Only four kinds of edges are backwards:

| Backward edge | What it carries | New home |
|---|---|---|
| `enemies/items → components` | `Clip` (plain animation data) | new leaf module `clip.rs` |
| `assets → components` | `BarKind` list for the loader | the caller (`app.rs`) passes `(Tex, hole)` pairs |
| `assets → dungeon` | `Theme` for tileset lookups | `Theme` methods in `dungeon/theme.rs` |
| `prefabs → systems` | sprite-sheet layouts, a few constants, `finish_spawning` | `sheets.rs`, `objects.rs`, `prefabs` |

## Global Constraints

- Values move byte-for-byte: no clip row, frame count, frame time, size or price may change.
- Keep `components::Clip` reachable as before (`pub(crate) use crate::clip::Clip;` in `components/animation.rs`) so the ~100 existing users do not churn.
- Match the surrounding style: `pub(crate)` items, no new comments beyond a one-line module doc where a new file is created.
- **Build and test in a worktree with its own `CARGO_TARGET_DIR`, or `touch app/src/main.rs` before every `cargo test`**: a shared `rust/target` gives both checkouts the same test-binary hash, so another checkout's build can replace yours.
- Run tests with `BANE_ASSETS=<worktree>/assets` so the asset tests read the worktree's files.

## Review Focus

1. HUD health/armor/boss bars are loaded through the changed loader path; a mistake shows as missing bar art at runtime. Task 2 adds a test that the loader queues exactly the bar textures.
2. Each dungeon theme must still pick its own tileset, and only Stone the narrow one. Task 2 adds a test for every theme.
3. Player walking clips (left is the flipped right row) must still match the sheet. Task 3 pins every player and pet clip in a test.
4. The slot machine must still charge 5 coins and start on the "lose" frame. The existing `systems/slot.rs` tests cover this; they must still pass unchanged apart from imports.
5. Enemies spawned with `prefabs::ready_enemy` must still get their physics body. The existing spawning tests cover this; Task 3 must keep them green.

---

### Task 1: `Clip` becomes a leaf module

**Files:**
- Create: `rust/app/src/clip.rs`
- Modify: `rust/app/src/main.rs` (add `mod clip;`)
- Modify: `rust/app/src/components/animation.rs:1-8`
- Modify: `rust/app/src/enemies.rs:4`, `rust/app/src/items.rs:4`

**Interfaces:**
- Produces: `crate::clip::Clip` with the same fields and derives as today. `crate::components::Clip` stays as a re-export.

- [ ] **Step 1: Create `rust/app/src/clip.rs`**

```rust
//! One animation of a sprite sheet: which row, how many frames and how fast.

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Clip {
    pub(crate) row: usize,
    pub(crate) frames: usize,
    pub(crate) frame_time: f32,
    pub(crate) looping: bool,
    pub(crate) flipped: bool,
}
```

- [ ] **Step 2: Replace the struct in `components/animation.rs`**

Delete the `Clip` definition (lines 1–8) and put this at the top of the file:

```rust
pub(crate) use crate::clip::Clip;
```

- [ ] **Step 3: Register the module and point the domain modules at it**

In `main.rs`, next to the other `mod` lines (alphabetical, after `mod camera;` or wherever `c` sorts), add `mod clip;`.
In `enemies.rs` and `items.rs` change `use crate::components::Clip;` to `use crate::clip::Clip;`.

- [ ] **Step 4: Run all tests**

Run: `cd rust && touch app/src/main.rs && BANE_ASSETS=$PWD/../assets cargo test`
Expected: all pass. `the_known_cycles_are_listed_exactly` still passes because the 10-module cycle is still there (`assets → components` holds it).

- [ ] **Step 5: Commit**

```bash
git add rust/app/src/clip.rs rust/app/src/main.rs rust/app/src/components/animation.rs rust/app/src/enemies.rs rust/app/src/items.rs
git commit -m "refactor: Clip lives in its own leaf module so enemies and items no longer depend on components"
```

---

### Task 2: `assets` no longer depends on `components` or `dungeon`

**Files:**
- Modify: `rust/app/src/assets.rs` (lines 8-9, 254-264, 491-497, 519-547, 568-574 and the tests at 600-640)
- Modify: `rust/app/src/dungeon/theme.rs` (`impl Theme`)
- Modify: `rust/app/src/components/hud.rs` (`impl BarKind`)
- Modify: `rust/app/src/app.rs:64`
- Modify: `rust/app/src/scenes/ingame.rs:8` and `:585-586`
- Modify: `rust/app/src/architecture.rs` (`KNOWN_CYCLES`)

**Interfaces:**
- Consumes: `crate::clip::Clip` from Task 1 (nothing else).
- Produces:
  - `Theme::tileset_texture(self) -> Tex` and `Theme::wide_tiles(self) -> bool` in `dungeon/theme.rs`
  - `Assets::tileset_of(&self, wide: bool) -> &Tileset`
  - `BarKind::holed_textures() -> [(Tex, [u32; 4]); 3]` in `components/hud.rs`
  - `Loader::start(gpu: &mut Gpu, assets: &mut Assets, holed: &[(Tex, [u32; 4])]) -> Result<Self, Box<dyn Error>>`

- [ ] **Step 1: Write the failing tests**

Add to the `#[cfg(test)] mod tests` of `dungeon/theme.rs` (create the module at the end of the file if it does not exist, with `use super::*;`):

```rust
#[test]
fn every_theme_has_its_own_tileset_and_only_stone_uses_narrow_tiles() {
    use crate::assets::tex;
    let all = [
        Theme::Stone,
        Theme::Sewer,
        Theme::Moss,
        Theme::Spider,
        Theme::Ice,
        Theme::Lava,
        Theme::Temple,
    ];
    let textures: std::collections::HashSet<_> =
        all.iter().map(|theme| theme.tileset_texture()).collect();

    assert_eq!(textures.len(), all.len());
    assert_eq!(Theme::Stone.tileset_texture(), tex::TILESET);
    assert_eq!(Theme::Lava.tileset_texture(), tex::TILESET_LAVA);
    assert!(!Theme::Stone.wide_tiles());
    assert!(all[1..].iter().all(|theme| theme.wide_tiles()));
}
```

Add to the tests of `components/hud.rs` (create `#[cfg(test)] mod tests { use super::*; … }` at the end if missing):

```rust
#[test]
fn the_loader_gets_every_bar_texture_with_its_hole() {
    let holed = BarKind::holed_textures();

    for (kind, (texture, hole)) in BarKind::ALL.iter().zip(holed) {
        assert_eq!(texture, kind.spec().texture);
        assert_eq!(hole, kind.spec().hole);
    }
}
```

- [ ] **Step 2: Run them to see them fail**

Run: `cd rust && touch app/src/main.rs && cargo test every_theme_has_its_own_tileset the_loader_gets_every_bar`
Expected: compile errors, `no method named tileset_texture` and `no function holed_textures`.

- [ ] **Step 3: Move the tileset lookup into `Theme`**

In `dungeon/theme.rs`, add `use crate::assets::{Tex, tex};` to the imports and these methods inside `impl Theme`:

```rust
pub(crate) fn tileset_texture(self) -> Tex {
    match self {
        Self::Stone => tex::TILESET,
        Self::Sewer => tex::TILESET_SEWER,
        Self::Moss => tex::TILESET_MOSS,
        Self::Spider => tex::TILESET_SPIDER,
        Self::Ice => tex::TILESET_ICE,
        Self::Lava => tex::TILESET_LAVA,
        Self::Temple => tex::TILESET_TEMPLE,
    }
}

/// Every theme but stone is drawn with the wide tileset layout.
pub(crate) fn wide_tiles(self) -> bool {
    self != Self::Stone
}
```

In `assets.rs`:
- delete `use crate::dungeon::Theme;` (line 9) and the whole `pub(crate) fn tileset_for(theme: Theme) -> Tex` (lines 254-264),
- replace `tileset_of` (lines 491-497) with:

```rust
pub(crate) fn tileset_of(&self, wide: bool) -> &Tileset {
    if wide {
        &self.wide_tileset
    } else {
        &self.tileset
    }
}
```

- in the test `every_theme_backdrop_matches_the_darkest_colour_of_its_tileset`, add `use crate::dungeon::Theme;` at the top of the tests module and change `tileset_for(theme).path()` to `theme.tileset_texture().path()`.

In `scenes/ingame.rs`, change the import on line 8 to `use crate::assets::{Assets, BULLET};` and lines 585-586 to:

```rust
            assets.tileset_of(dungeon.theme.wide_tiles()),
            assets.textures.get(dungeon.theme.tileset_texture()),
```

- [ ] **Step 4: Let the caller hand the bar textures to the loader**

In `components/hud.rs`, inside `impl BarKind`:

```rust
/// The bar textures with the hole the loader cuts out of them.
pub(crate) fn holed_textures() -> [(Tex, [u32; 4]); 3] {
    Self::ALL.map(|kind| {
        let spec = kind.spec();
        (spec.texture, spec.hole)
    })
}
```

In `assets.rs`:
- delete `use crate::components::BarKind;` (line 8),
- change the `Step` enum variant `Bar(BarKind)` to `Holed(Tex, [u32; 4])`,
- change `Loader::start` to:

```rust
pub(crate) fn start(
    gpu: &mut Gpu,
    assets: &mut Assets,
    holed: &[(Tex, [u32; 4])],
) -> Result<Self, Box<dyn Error>> {
    for key in EARLY_TEXTURES {
        load_texture(gpu, &mut assets.textures, *key)?;
    }
    let mut pending: VecDeque<Step> =
        GAME_TEXTURES.iter().copied().map(Step::Texture).collect();
    pending.extend(holed.iter().map(|(texture, hole)| Step::Holed(*texture, *hole)));
    pending.push_back(Step::Atlas);
    pending.extend(Audio::files().into_iter().map(Step::Sound));
    pending.extend(FontSize::ALL.map(Step::Font));
    Ok(Self {
        pending,
        decoded: Vec::new(),
    })
}
```

- change the `Step::Bar(kind)` arm in `Loader::step` to:

```rust
Step::Holed(texture, hole) => {
    let path = texture.path();
    let pixels = Pixels::load_with_hole(&root().join("textures").join(path), hole)?;
    self.decoded.push((path, pixels));
}
```

- in the test `every_texture_key_is_loaded_by_one_of_the_lists`, add `use crate::components::BarKind;` to the tests module (a test-only dependency, which the guard ignores).

In `app.rs:64`:

```rust
            let loader = Loader::start(
                &mut gpu,
                &mut assets,
                &crate::components::BarKind::holed_textures(),
            )?;
```

- [ ] **Step 5: Run all tests and read the guard's verdict**

Run: `cd rust && touch app/src/main.rs && BANE_ASSETS=$PWD/../assets cargo test`
Expected: everything passes except `architecture::the_known_cycles_are_listed_exactly`, which fails with "these known cycles shrank or broke up" and lists the remaining cycles as `[{"prefabs", "systems"}]`. If it lists any other group, a backward edge was missed; run `cargo test print_module_cycles -- --ignored --nocapture` and fix that edge before going on.

- [ ] **Step 6: Tighten the guard**

In `architecture.rs`, delete the ten-module entry so `KNOWN_CYCLES` reads:

```rust
const KNOWN_CYCLES: &[&[&str]] = &[&["prefabs", "systems"]];
```

Run: `cd rust && touch app/src/main.rs && BANE_ASSETS=$PWD/../assets cargo test`
Expected: all pass.

- [ ] **Step 7: Commit**

```bash
git add rust/app/src/assets.rs rust/app/src/dungeon/theme.rs rust/app/src/components/hud.rs rust/app/src/app.rs rust/app/src/scenes/ingame.rs rust/app/src/architecture.rs
git commit -m "refactor: assets is a leaf again, themes pick their tilesets and the app hands the bar textures to the loader"
```

---

### Task 3: `prefabs` no longer depends on `systems`

**Files:**
- Create: `rust/app/src/sheets.rs`
- Modify: `rust/app/src/main.rs` (add `mod sheets;`)
- Modify: `rust/app/src/systems/player_animation.rs:1-26`, `rust/app/src/systems/pet.rs:14-16, 27-45`, `rust/app/src/systems/slot.rs:16-46`
- Modify: `rust/app/src/systems/death_screen.rs:11, 21-22`
- Modify: `rust/app/src/systems/enemies.rs:109-125`, `rust/app/src/systems/bosses.rs:184`
- Modify: `rust/app/src/systems/mod.rs:75, 100-104, 115-117`
- Modify: `rust/app/src/objects.rs`
- Modify: `rust/app/src/prefabs/actors.rs:19-21, 58-62, 92-96, 439, 489`, `rust/app/src/prefabs/fixtures.rs:22, 668, 696`, `rust/app/src/prefabs/ui.rs:12`
- Modify: `rust/app/src/architecture.rs` (`KNOWN_CYCLES`)

**Interfaces:**
- Consumes: `crate::clip::Clip` from Task 1.
- Produces:
  - `crate::sheets::player::{COLUMNS, ROWS, IDLE, RIGHT, LEFT, BOTTOM, TOP}`
  - `crate::sheets::pet::{COLUMNS, ROWS, IDLE, BOTTOM, TOP, RIGHT, LEFT}`
  - `crate::sheets::slot::{COLUMNS, ROWS, SPIN, WIN, LOSE, IDLE}`
  - `crate::objects::SPIN_COST: i32`
  - `crate::prefabs::finish_spawning(world: &mut World, physics: &mut Physics, entity: Entity)`

- [ ] **Step 1: Write the failing test that pins the sheets**

Create `rust/app/src/sheets.rs` with only the test module for now:

```rust
//! Sprite-sheet layouts shared by the prefabs that spawn an entity and the systems that animate it.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clip::Clip;

    fn looping(row: usize, frames: usize, frame_time: f32, flipped: bool) -> Clip {
        Clip {
            row,
            frames,
            frame_time,
            looping: true,
            flipped,
        }
    }

    #[test]
    fn the_player_sheet_keeps_its_rows() {
        assert_eq!((player::COLUMNS, player::ROWS), (6, 4));
        assert_eq!(player::IDLE, looping(0, 6, 0.1, false));
        assert_eq!(player::RIGHT, looping(1, 6, 0.1, false));
        assert_eq!(player::LEFT, looping(1, 6, 0.1, true));
        assert_eq!(player::BOTTOM, looping(2, 6, 0.1, false));
        assert_eq!(player::TOP, looping(3, 6, 0.1, false));
    }

    #[test]
    fn the_pet_sheet_keeps_its_rows() {
        assert_eq!((pet::COLUMNS, pet::ROWS), (4, 4));
        assert_eq!(pet::IDLE, looping(3, 2, 0.25, false));
        assert_eq!(pet::BOTTOM, looping(0, 4, 0.1, false));
        assert_eq!(pet::TOP, looping(1, 4, 0.1, false));
        assert_eq!(pet::RIGHT, looping(2, 4, 0.1, false));
        assert_eq!(pet::LEFT, looping(2, 4, 0.1, true));
    }

    #[test]
    fn the_slot_machine_sheet_keeps_its_rows_and_rests_on_a_loss() {
        assert_eq!((slot::COLUMNS, slot::ROWS), (6, 3));
        assert_eq!(slot::SPIN, looping(0, 4, 0.1, false));
        assert_eq!(slot::WIN, looping(1, 6, 0.1, false));
        assert_eq!(slot::LOSE, looping(2, 1, 1.0, false));
        assert_eq!(slot::IDLE, slot::LOSE);
    }
}
```

Add `mod sheets;` to `main.rs`.

- [ ] **Step 2: Run it to see it fail**

Run: `cd rust && touch app/src/main.rs && cargo test sheets`
Expected: compile errors, `cannot find module player/pet/slot`.

- [ ] **Step 3: Move the sheet layouts into `sheets.rs`**

Put this above the test module in `sheets.rs`:

```rust
pub(crate) mod player {
    use crate::clip::Clip;

    const FRAME_TIME: f32 = 0.1;
    const FRAMES: usize = 6;

    pub(crate) const COLUMNS: usize = 6;
    pub(crate) const ROWS: usize = 4;

    pub(crate) const IDLE: Clip = clip(0, false);
    pub(crate) const RIGHT: Clip = clip(1, false);
    pub(crate) const LEFT: Clip = clip(1, true);
    pub(crate) const BOTTOM: Clip = clip(2, false);
    pub(crate) const TOP: Clip = clip(3, false);

    const fn clip(row: usize, flipped: bool) -> Clip {
        Clip {
            row,
            frames: FRAMES,
            frame_time: FRAME_TIME,
            looping: true,
            flipped,
        }
    }
}

pub(crate) mod pet {
    use crate::clip::Clip;

    const MOVE_FRAME_TIME: f32 = 0.1;
    const IDLE_FRAME_TIME: f32 = 0.25;

    pub(crate) const COLUMNS: usize = 4;
    pub(crate) const ROWS: usize = 4;

    pub(crate) const IDLE: Clip = clip(3, 2, IDLE_FRAME_TIME, false);
    pub(crate) const BOTTOM: Clip = clip(0, 4, MOVE_FRAME_TIME, false);
    pub(crate) const TOP: Clip = clip(1, 4, MOVE_FRAME_TIME, false);
    pub(crate) const RIGHT: Clip = clip(2, 4, MOVE_FRAME_TIME, false);
    pub(crate) const LEFT: Clip = clip(2, 4, MOVE_FRAME_TIME, true);

    const fn clip(row: usize, frames: usize, frame_time: f32, flipped: bool) -> Clip {
        Clip {
            row,
            frames,
            frame_time,
            looping: true,
            flipped,
        }
    }
}

pub(crate) mod slot {
    use crate::clip::Clip;

    pub(crate) const COLUMNS: usize = 6;
    pub(crate) const ROWS: usize = 3;

    pub(crate) const SPIN: Clip = Clip {
        row: 0,
        frames: 4,
        frame_time: 0.1,
        looping: true,
        flipped: false,
    };
    pub(crate) const WIN: Clip = Clip {
        row: 1,
        frames: 6,
        frame_time: 0.1,
        looping: true,
        flipped: false,
    };
    pub(crate) const LOSE: Clip = Clip {
        row: 2,
        frames: 1,
        frame_time: 1.0,
        looping: true,
        flipped: false,
    };
    pub(crate) const IDLE: Clip = LOSE;
}
```

Then remove the moved items from the systems and import them instead:
- `systems/player_animation.rs`: delete lines 6-26 (the constants and `clip`). Add `use crate::sheets::player::{BOTTOM, IDLE, LEFT, RIGHT, TOP};`. Change the import to `use crate::components::{Animation, Player};` (drop `Clip`).
- `systems/pet.rs`: delete `PET_COLUMNS`, `PET_ROWS`, `PET_GAZE_SPEED` (lines 14-16), `MOVE_FRAME_TIME`, `IDLE_FRAME_TIME`, the `clip` helper and the five clip constants. Add `use crate::sheets::pet::{BOTTOM, LEFT, RIGHT, TOP};`. Rename the one remaining use `PET_IDLE` (line ~299) to `crate::sheets::pet::IDLE`. Drop `Clip` from the components import if it is now unused (the compiler will say).
- `systems/slot.rs`: delete `SPIN_COST` (line 16), `SLOT_COLUMNS`, `SLOT_ROWS` (21-22), `SPIN`, `WIN`, `LOSE` and `SLOT_IDLE` (24-46). Add `use crate::objects::SPIN_COST;` and `use crate::sheets::slot::{LOSE, SPIN, WIN};`. In its tests, use `crate::sheets::slot::{COLUMNS, IDLE, ROWS}` where `SLOT_COLUMNS`, `SLOT_IDLE` and `SLOT_ROWS` were used.
- `systems/mod.rs`: remove `IDLE, PLAYER_COLUMNS, PLAYER_ROWS` from the `player_animation` re-export, `PET_COLUMNS, PET_GAZE_SPEED, PET_IDLE, PET_ROWS` from the `pet` re-export, and `SLOT_COLUMNS, SLOT_IDLE, SLOT_ROWS, SPIN_COST` from the `slot` re-export.

In `objects.rs`, after the other tuning constants:

```rust
pub(crate) const SPIN_COST: i32 = 5;
```

- [ ] **Step 4: Point the prefabs at the new homes**

`prefabs/actors.rs`:
- replace the `use crate::systems::{…}` block (lines 19-21) with `use crate::sheets::{pet, player};`,
- add `const PET_GAZE_SPEED: f32 = 150.0;` next to `PET_SIZE`,
- lines 58-62: `IDLE.row` → `player::IDLE.row`, `PLAYER_COLUMNS` → `player::COLUMNS`, `PLAYER_ROWS` → `player::ROWS`, `IDLE` → `player::IDLE`,
- lines 92-93: `PET_IDLE` → `pet::IDLE`, `PET_COLUMNS` → `pet::COLUMNS`, `PET_ROWS` → `pet::ROWS`,
- move `finish_spawning` from `systems/enemies.rs:113-125` into this file unchanged (add whatever `Spawning`, `Color`, `Vec4` imports the compiler asks for) and replace both `crate::systems::finish_spawning(world, physics, entity);` calls (lines 439, 489) with `finish_spawning(world, physics, entity);`.

`systems/enemies.rs`: delete `finish_spawning`, call `prefabs::finish_spawning(world, physics, entity)` on line 109 (add `use crate::prefabs;` if missing). `systems/mod.rs`: delete `pub(crate) use enemies::finish_spawning;`. `systems/bosses.rs:184` (test module): import `crate::prefabs::finish_spawning` instead of `crate::systems::finish_spawning`.

`prefabs/fixtures.rs`: replace line 22 with `use crate::objects::SPIN_COST;` and `use crate::sheets::slot;`, and line 668 with `Animation::new(slot::COLUMNS, slot::ROWS, slot::IDLE),`.

`prefabs/ui.rs`: delete line 12 and add the three death-screen layout constants there (nothing in `systems/death_screen.rs` uses them besides their definitions, which you delete):

```rust
const SKULL_SIZE: Vec2 = Vec2::new(102.0, 96.0);
const TITLE_BASELINE: f32 = 175.0;
const PROMPT_POSITION: Vec2 = Vec2::new(700.0, 500.0);
```

- [ ] **Step 5: Check that `prefabs` no longer reaches into `systems`**

Run: `grep -rn "systems" rust/app/src/prefabs/`
Expected: no output.

- [ ] **Step 6: Run all tests and read the guard's verdict**

Run: `cd rust && touch app/src/main.rs && BANE_ASSETS=$PWD/../assets cargo test`
Expected: everything passes except `architecture::the_known_cycles_are_listed_exactly`, which reports that `["prefabs", "systems"]` broke up and that the current cycles are `[]`.

- [ ] **Step 7: Empty the list of known cycles**

In `architecture.rs`:

```rust
const KNOWN_CYCLES: &[&[&str]] = &[];
```

Run: `cd rust && touch app/src/main.rs && BANE_ASSETS=$PWD/../assets cargo test && cargo clippy --all-targets`
Expected: all tests pass, no clippy warnings. `cargo test print_module_cycles -- --ignored --nocapture` prints nothing.

- [ ] **Step 8: Commit**

```bash
git add rust/app/src/sheets.rs rust/app/src/main.rs rust/app/src/objects.rs rust/app/src/systems rust/app/src/prefabs rust/app/src/architecture.rs
git commit -m "refactor: prefabs no longer reach into systems, sprite sheets live in sheets.rs and no module cycle is left"
```

---

## After the plan

- Run the game once: the health, armor and boss bars, a stone floor and a themed floor, the player walking in all four directions, the pet and a slot-machine spin. These are the paths no test draws (Review Focus 1-3).
- Not in scope, deliberately: the guard only looks at top-level modules. Cycles inside `systems` (for example `systems::slot → systems::loadout`) are allowed; tighten that later only if it starts to hurt.
