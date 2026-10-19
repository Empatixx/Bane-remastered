# Bane — Feature Reference

> Auto-generated feature inventory of the **Bane** game, derived from a survey of the
> source tree (`app/src/main/java/cz/empatix/…`, 647 Java files). All stat values are
> read directly from the code; file references use `path:line` form where useful.
> Sections flagged **WIP / placeholder** describe systems that exist in code but are
> currently disabled, stubbed, or unused.

---

## 1. Overview

Bane is a **single-player 2D top-down roguelike shooter** built on Java 17, LWJGL 3
(OpenGL), and the dyn4j physics engine. A run consists of descending through
procedurally generated floors, clearing combat rooms, shopping, fighting a boss, and
taking a ladder to the next, harder floor. Death ends the run; the only thing that
carries between runs is **coins** and **permanently unlocked talent-tree upgrades**.

**Genre pillars**
- Twin-stick-style shooting (WASD move + mouse aim) with 7 distinct firearms.
- Procedural dungeon generation (≈9–10 rooms per floor).
- Roguelike run loop with persistent meta-progression between runs.
- Consumable artefacts, a combat pet, traps, destructibles, and a shop economy.

**Internal render resolution:** 960×540 (16:9), scaled to the window.
**Simulation:** fixed 60 UPS physics, variable render rate.

---

## 2. Game Flow & States

State machine (`gamestate/`): `LOADING → MENU → LOBBY → INGAME`, with death returning
to the lobby and the pause menu's *Exit* returning to the menu.

| State | Role |
|-------|------|
| **Loading** | Async asset loading with animated spinner. Advances to Menu when assets are ready. |
| **Menu** | Main menu — Play, Settings, Exit. |
| **Lobby** | Single-player staging room between runs. Contains the **Scientist** (upgrade NPC), coin display, and player status bars. Despite the name, **not multiplayer**. |
| **In-Game** | The actual run: procedural floors, combat, shops, bosses. |

**Run loop:** Lobby → enter portal/start → clear floor's rooms → defeat boss → take the
**Ladder** to generate the next floor (floor counter increments, difficulty scales) →
repeat until death → **Death/Game-Over screen** → back to Lobby with earned coins.

---

## 3. Player

`player/Player.java`

### Base stats

| Stat | Value | Notes |
|------|-------|-------|
| Max health | **10 HP** | Damage buffer order: armor first, then health. |
| Max armor | **5** | Absorbs hits; overflow spills to health. |
| Move force | **5,000,000** | Applied through the physics body. |
| Invulnerability window | **1000 ms** | After each hit; player sprite flickers while active. |
| Low-HP threshold | **30%** | Triggers red vignette post-process + `LOW_HEALTH` sound. |
| Sprite | 16×38 px @ 2× scale (32×76) | 4-direction animation, 6 frames/row, 100 ms/frame. |

- **No passive health regeneration**, no XP/leveling, **no dash/dodge** for the player
  (a `player_dash` sound exists but is unused; dashing is an *enemy* ability).
- Takes damage only from `ENEMY` and `TRAP` damage sources; ignores `PLAYER` sources
  (own bullets).
- Heal/armor pickups play `HEAL`; hits play `PLAYER_HURT` + red vignette.

### Modifier hooks (the "hook composite" pattern)

Damage and movement are run through ordered hook chains so upgrades/artefacts can
stack modifiers:

- **`DamageEventHook` → `DamageEventHookComposite`** — each hook transforms incoming
  damage (`int onDamage(int)`). E.g. *Kevlar* upgrade adds `d -> d * 0.9`.
- **`MoveForceEventHook` → `MoveForceEventHookComposite`** — each hook transforms move
  force (`long onMoveForce(long)`). E.g. *Swiftwind Steps* adds `f -> f * 1.1`;
  *Berserk Pot* artefact adds `f -> f * 1.3` for 15 s.

### Controls

| Input | Action |
|-------|--------|
| **W / A / S / D** | Move up / left / down / right |
| **Mouse** | Aim |
| **Left mouse** | Shoot |
| **Mouse wheel** | Cycle weapon slot |
| **1–9** | Select weapon slot directly |
| **R** | Reload |
| **Q** | Drop current weapon |
| **E** | Interact (pickups, portal, ladder, shop, scientist, slot machine) |
| **F** | Activate equipped artefact |
| **TAB** | Toggle full-screen minimap (ESC closes it) |
| **ESC** | Pause menu (Gaussian-blur overlay) |

---

## 4. Weapons

`guns/` — 7 firearms, clip-based ammo with reserve pool, per-weapon fire mode.
The player starts each run with a **Pistol** (2 slots, the second empty; expandable to 3 via the
*Additional Gun* upgrade). Other weapons are found as drops from chests/combat.

| Weapon | Damage | Fire rate | Reload | Clip | Reserve | Spread | Fire mode | Notes |
|--------|:------:|:---------:|:------:|:----:|:-------:|:------:|-----------|-------|
| **Pistol** | 1–3 | 250 ms | 700 ms | 7 | 120 | 0.10 | Semi-auto | Starter |
| **Luger** | 1–2 | 250 ms | 700 ms | 9 | 150 | 0.35 | Semi-auto | Inaccurate |
| **Revolver** | 4–6 | 450 ms | 700 ms | 6 | 60 | 0.20 | Semi-auto | High per-shot dmg |
| **Shotgun** | 222–333 | 500 ms | 700 ms | 6 | 48 | 0.10 | Semi-auto | **4 pellets/shot**, spread ⚠️ |
| **Uzi** | 1–2 | 100 ms | 700 ms | 20 | 200 | 0.40 | Full-auto | Fastest fire |
| **Thompson** | 1–3 | 200 ms | 850 ms | 30 | 300 | 0.05 | Full-auto | Accurate SMG |
| **M4** | 2 | 750 ms/burst | 1300 ms | 32 | 250 | 0.00 | 4-round burst | 50 ms between burst rounds; can't reload mid-burst |

> ⚠️ The Shotgun's 222–333 damage (vs. 1–6 for every other gun) is the literal value in
> `Shotgun.java:32-33` and looks like leftover debug/balancing — flagged here for review.

**Mechanics**
- Per-bullet damage is randomized: `min + rand*(max-min)`, then passed through the gun
  damage-hook chain (upgrades, crit).
- **Reload (R):** only when `reserve > 0 && clip < clipSize`; tops the clip from reserve
  after `reloadTime`. Each gun has its own reload/shoot SFX.
- **Spread/inaccuracy** offsets the aim angle per shot; the Shotgun fans its 4 pellets
  with alternating spread.
- **Bullets** (`guns/ammo/Bullet.java`): physics projectiles with a red glow light,
  no drag (constant velocity), pass through the player, and play a collision animation
  (~140 ms) with `WALL_HIT` / `ENEMY_HIT` SFX on impact.
- **Ammo box pickups** (`items/domain/AmmoBoxType`): PISTOL +10%, SHOTGUN +25%,
  SUBMACHINE +10%, UNIVERSAL +50% of that weapon's max reserve.

---

## 5. Enemies

`enemies/domain/` — 11 spawnable enemy types (annotated `@SpawnableEnemy(type, minFloor, maxFloor)`).
**Enemy stats do not scale per floor**; difficulty ramps via *which* enemies unlock at
higher floors and spawn density.

| Enemy | HP | Dmg | Move force | Floors | Special |
|-------|:--:|:---:|:----------:|:------:|---------|
| **Bat** | 9 | 2 | 14.5M | 0+ | Simple melee chaser |
| **Rat** | 12 | 2 | 6.5M | 0+ | Simple melee chaser |
| **Snake** | 8 | 3 | 8M | 0+ | Simple melee chaser |
| **Slime** | 9 | 1 | 1M | 0+ | *SlimeSmash*: 5-bullet spiral every 2 s |
| **RedSlime** | 9 | 1 | 1M | 0+ | On death spawns bullets ≈ hits taken |
| **Ghost** | 7 | 2 | 6.5M | 0+ | *Dash*: +24.5M force for 1 s, 3 s cooldown |
| **GreenSpiderling** | 5 | 2 | 5.5M | 2+ | On death heals nearby spiderlings (+1, r=200) |
| **RedSpiderling** | 3 | 2 | 5.5M | 2+ | Heals self +1 per hit taken |
| **PurpleSpiderling** | 3 | 1 | 6.5M | 2+ | *Teleport behind player* (**disabled in code**) |
| **DemonEye** | 11 | 3 | 14.5M | 3+ | Simple melee chaser |
| **EyeWatcher** | 4 | 1 | 0 (static) | 3+ | Fires homing projectile every 3 s |

**AI / targeting:** A* pathfinding (`AStarTargetting`) on a 16 px grid, recomputed when
the player moves more than one cell; capped at 25,000 cells. Sprites flip based on the
player's side. Enemies are spawn-invincible for 2000 ms and flash a red outline for
1000 ms after being hit.

**Loot drops (`EnemyDropService`):** pity system — a drop is guaranteed after ~4 kills
without one (random offset makes each kill more likely). Bosses do not drop pickups.

---

## 6. Bosses

`enemies/domain/bosses/` — 3 bosses. Bosses show a top-screen boss health bar with a
red name label and are spawn-invincible for 1500 ms.

| Boss | HP | Dmg | Move force | Floor | Mechanics |
|------|:--:|:---:|:----------:|:-----:|-----------|
| **Spider** | 75 | 2 | 32.5M | 1 only | *HatchSpiderlings* (one R→G→P spiderling every 2.5 s) + *SpitCobweb* (slows player: +5 linear damping, cap 15). Despawns its spiderlings on death. |
| **KingSlime** | 90 | 2 | 10M | 2+ | *SlimeSmash* (25-bullet spiral every 2 s) + *KingSlimeArmageddon* (continuous spray, 150 ms cadence, rising inaccuracy). |
| **Golem** | 100 | 4 | 12.5M | any* | *Healing* (+2 HP × 5 ticks = +10, 30 s cooldown), *ArmorStacking* (0→3 stacks), *Dash*, and a rotating **GolemLaser** beam (1 dmg/frame contact). Melee attack currently disabled. |

\* Golem has no `@SpawnableEnemy` annotation — it is spawned manually rather than via the
floor-spawn table.

---

## 7. Enemy Skills

`enemies/skills/` — cooldown-driven abilities shared across enemies/bosses.

| Skill | User(s) | Cooldown | Effect |
|-------|---------|:--------:|--------|
| SlimeSmash | Slime, KingSlime | 2000 ms | 5 (or 25) bullets in a spiral |
| Dash | Ghost, Golem | 3000 ms | +24.5M force for 1000 ms |
| Healing | Golem | 30000 ms | 5 ticks × +2 HP over 5 s |
| ArmorStacking | Golem | 4000 ms | Cycle 0→3 armor stacks |
| HatchSpiderlings | Spider | 2500 ms | Summon spiderling (R→G→P cycle) |
| SpitCobweb | Spider | 5000 ms | Cobweb projectile, slows player |
| KingSlimeArmageddon | KingSlime | 150 ms | Continuous bullet spray |
| EyeWatcherProjectile | EyeWatcher | 3000 ms | Homing projectile (self-destructs after 5 s) |
| SpiderlingTeleport | PurpleSpiderling | 8000 ms | Teleport behind player (**disabled**) |

All enemy projectiles deal **1 damage**.

---

## 8. Map Generation & Floors

`map/generators/ProceduralMapGenerator.java`, `rooms/`

Each floor is assembled from a fixed room budget plus an optional sacrifice room, then
connected with procedurally generated corridor (`PATH`) rooms via BFS with AABB
overlap-avoidance. Rooms are 15×20 tiles; tiles are 32 px @ 2× = 64 px on screen.

| Room type | Count/floor | Purpose |
|-----------|:-----------:|---------|
| **Start** | 1 | Player spawn, no enemies |
| **Normal** | 5 | Combat + loot, floor-appropriate enemies |
| **Shop** | 1 | Shopkeeper + 3–5 items for coins |
| **Treasure** | 1 | Chests (weapon/artefact) and breakables |
| **Boss** | 1 | Single boss; dead-end room |
| **Sacrifice** | 0–1 | Optional; items cost **health** instead of coins |
| **Path** | generated | Corridors connecting rooms |

**Floor progression:** clear the boss room, then interact with the **Ladder** to
generate a fresh floor and increment the floor counter. Higher floors unlock tougher
enemy types and raise shop prices. The tile set is a single shared atlas (no distinct
biomes); only row-0 tiles are walkable floor, the rest are solid walls.

---

## 9. Room Objects

`roomobjects/domain/`

**Interactables**
- **Portal** — returns to the lobby (E).
- **Ladder** — advances to the next floor (E).
- **Shopkeeper** — shop NPC with speech bubbles.
- **Scientist** — opens the persistent upgrade menu (E); present in the lobby.
- **Slot Machine** — gamble 5 coins/spin (2 s), unlocked by the *Gamble Machine* upgrade.
  Reward table: 40% lose · 20% 2× (10c) · 10% 3× (15c) · 10% cashout (10c) ·
  10% drop 5 common items · 10% rare item.

**Destructibles** (4 HP each; drop items on destruction)
- **Chest** — `WEAPON` or `ARTEFACT` type; ejects 5 items + 1 weapon/artefact radially.
- **Barrel**, **Pot** — drop 1 random item; can be pushed (Pot can be made immovable).
- **Explosive Barrel** — on death: AoE explosion, **3 dmg** in a 128×96 radius, plus a
  fading red light; damages items/traps/bullets in range.

**Traps** (`Trap` interface — can be disabled by the *Trap Master* upgrade)
- **Spike** — 1 dmg while active (animated on/off cycle).
- **Flamethrower** — 1 dmg on the firing frame; emits a red light.
- **Path Gate** — opens/closes; solid `TRAP` collision when closed.

**Decorations** (non-interactive theming): torches/candlesticks (lights), banners,
bookshelves, bones, chains, cobwebs, angel statue, pentagram circle, weapon racks,
crafting/potion/shop/map tables, boxes, rocks, task board, etc.

---

## 10. Items & Loot

`items/domain/`

**Rarity weights:** COMMON 60 · UNCOMMON 25 · RARE 15.

| Item | Effect |
|------|--------|
| **Recover Potion** | HEALTH or ARMOR; small = +1, large = +4. |
| **Ammo Box** | Refills reserve ammo by weapon-type % (see §4). |
| **Coin** | +1 coin; drops from enemies, chests, breakables, slot machine. |
| **Weapon Drop** | Pick up a gun (E); floats with outline highlight when near. |
| **Artefact Drop** | Pick up an artefact (E). |
| **Shop Item** | Any item wrapped with a coin price; supports discounts/sales. |
| **Sacrificial Item** | Shop item that costs **1–3 health** instead of coins. |

Dropped items can be physics-launched (e.g. from chests at ~20,000 N radial force) and
despawn after a timeout (blinking ~5 s before despawn) unless flagged persistent.

---

## 11. Economy

`player/CoinService.java`

- **Coins** are the run currency (gold text, `banecoin.tga`), shown bottom-left in the HUD.
- Earned from enemy kills, chests, breakables, and the slot machine; spent at shops and
  the slot machine. Start of run: 0.
- **Shop pricing:** `rand(1..5) × (floor+1) × typeMultiplier` — Offensive ×2.0,
  Defensive ×1.5, Ammunition ×1.0. Higher floors → pricier shops.
- **Sacrifice room** items cost health, not coins.
- Coins earned during a run also feed the post-run reward (see §14); permanent upgrades
  are paid for with coins from the Scientist menu.

---

## 12. Meta-Progression: Upgrades

`upgrades/` — persistent talent trees, saved to `upgrades.bin` (AES-encrypted JSON
`Map<PlayerUpgradeName, Boolean>`) at the repo root. Purchased via the **Scientist** UI,
gated by parent→child dependencies. 3 trees × 5 nodes = 15 upgrades.

| Tree | Upgrade | Quality | Cost | Effect |
|------|---------|---------|:----:|--------|
| **Utility** | Swiftwind Steps | Common | 20 | Move speed +10% |
| | Discount | Common | 60 | Chance to reduce shop prices |
| | Trap Master | Rare | 140 | 30% chance to disable each trap |
| | Additional Gun | Epic | 300 | +1 weapon slot |
| | Gamble Machine | Common | 500 | Unlocks the shop slot machine |
| **Defensive** | Hero's Heart | Common | 20 | Max health ×1.2 |
| | Improved Armor | Common | 60 | Max armor ×1.2 |
| | Armor Regeneration | Rare | 140 | +1 armor on clearing a room |
| | Titan's Heart | Epic | 300 | Max health ×1.25 |
| | Kevlar | Common | 500 | Damage taken ×0.9 |
| **Offensive** | Sharpened Bullets | Common | 20 | Damage ×1.2 |
| | Additional Ammo | Common | 60 | Max ammo ×1.1 |
| | Critical Hit | Rare | 140 | 10% chance for 2× damage |
| | Dangerous Bullets | Epic | 300 | Damage ×1.25 |
| | Pet | Common | 500 | Unlocks the combat pet |

Upgrades are applied at run start via `UpgradesApplicationService` (player, gun, and map
appliers). Effects use the hook composites from §3.

---

## 13. Artefacts & Pet

### Artefacts (`artefacts/`)
One artefact slot, activated with **F**. Default equipped at run start: *Reveal Map*.

| Artefact | Effect |
|----------|--------|
| **Ring of Death** | Fires 50 bullets radially around the player. |
| **Armor Refill** | Instantly refills armor to max. |
| **Shield Horn** | Magic shield blocks the next hit (0 damage), then expires. |
| **Berserk Pot** | +30% move force for 15 s, with speed particles. |
| **Reveal Map** | Reveals the entire minimap. |

### Pet (`pets/Pet.java`)
Unlocked by the *Pet* upgrade — a wolf companion that follows the player and fights.

| Stat | Value |
|------|-------|
| Damage | 1 (200 ms attack cooldown) |
| Move force | 12,500,000 |
| Size | 96×96 px |
| Detection radius | 500 px |

Behavior priority: **attack** nearby enemies > **collect** item drops > **follow** owner.

---

## 14. Stats & Death Screen

`death/` — tracked per run via `StatisticsService`:

- **Accuracy** = bullets hit / bullets fired
- **Kills** = enemies killed
- **Loot** = items collected
- **Floor** = deepest floor reached

On death, a skull animates in over 2.5 s, "Game Over" shows, and the four stats slide in,
each converting to a **coin reward**:

| Reward | Formula |
|--------|---------|
| Accuracy | `2.0 × floor^1.02 × (accuracy×100)^1.01` |
| Kills | `2.0 × floor^1.02 × kills^1.01` |
| Loot | `4.0 × floor^1.02 × loot^1.01` |
| Floor | `(floor − 1) × 5` |

"Press any key to continue" returns to the Lobby with the earned coins banked.

---

## 15. HUD, Audio & Visual Effects

**HUD**
- Health bar (top-left, ~25,25) and armor bar below it.
- Coin counter (bottom-left) with icon.
- Weapon slot + ammo readout `clip/reserve` (bottom area), per-weapon clip visualization.
- Artefact slot icon.
- Minimap — small always-on, big toggled with TAB (camera follows player, shooting
  disabled while open).
- On-screen keyboard key prompts (Q/W/E/R/A/S/D/F/TAB/1–5) for interaction hints.

**Audio** (`assets/audio/`)
- Music: `intro`, `menuclick`, `map_1`, `map_2`, `boss`, `ambient`.
- SFX: per-gun shoot/reload, `enemy_hit`, `wall_hit`, `explosion`, `heal`, `lowhealth`,
  `playerhurt`, `playerdeath`, chest opens, `coin`/`drop_coin`, `pickup`, `buy`,
  `sacrifice`, slot-machine win/lose/spin/cashout, `talent_unlock`, `switchgun`.
- Three-channel volume control: master / music / effects.

**Visual effects**
- **Particles:** gazewind speed trails when moving fast (>350 velocity).
- **Dynamic shadows:** player and per-object shadows cast from light sources
  (max length 400 px, smoothly interpolated).
- **Lighting:** dynamic `LightSource`s (position/color/radius/intensity) for bullets,
  explosions, torches, portals, etc., with camera-frustum culling.
- **Post-processing:** red vignette at low HP / on hit (3 s pulse), Gaussian blur during
  pause and death, fade on state transitions.

---

## 16. Settings & Integrations

**Settings** (`settings/`) — three tabs:
- **Video:** V-sync toggle, fullscreen/borderless toggle, resolution selector.
- **Audio:** master / music / effects volume sliders.
- **Controls:** present in the UI but **empty / not yet implemented** (key rebinding TODO).
- Apply / Revert / Exit buttons; settings persist.

**Integrations**
- **Discord Rich Presence** (`DiscordInit.java`, app id `805802430553653279`) — initialized
  with placeholder activity details and a blocking update loop; party/join secrets stubbed.
  **WIP.**
- **Steam:** mentioned in project docs but **no Steamworks integration exists** in the code
  (no achievements, stats, or cloud saves).
- **Multiplayer:** none — the "Lobby" is a single-player staging room only.

---

## 17. Architecture (for contributors)

Maven multi-module split:
- **`app`** — the game itself (`cz.empatix.*`: player, guns, enemies, map, items,
  upgrades, gamestate, hud, etc.).
- **`game-engine-core`**, **`game-engine-render`**, **`game-engine-physics`**,
  **`game-engine-audio`** — reusable engine layers being extracted from the monolith.

Consistent `domain/` (data/entities) + `service/` (logic) + `factory/` (creation)
package convention. Rendering is a layered `Renderable`/`Drawable` tree grouped by
`RenderScene` (SCENE/HUD/OVERLAYS) with per-scene post-processing. Physics wraps dyn4j
behind `PhysicsService`/`PhysicsEngine` with interpolated bodies for smooth rendering.

---

## Appendix — Known WIP / placeholder systems

| Area | Status |
|------|--------|
| **Affixes** (`affixes/`) | Enum of 5 types (RARE_POTIONS, INFLATION, FORTIFIED, RAGING, REGENERATION) defined; `AffixService` empty — **unused**. |
| **Shotgun damage** | 222–333 looks like debug values vs. 1–6 for other guns. |
| **PurpleSpiderling teleport** | Coded but disabled (TODO). |
| **Golem melee attack** | Early-returns (`if (true) return;`) — disabled. |
| **Settings → Controls tab** | Empty placeholder; no key rebinding. |
| **Discord presence** | Stub details + blocking loop; needs refinement. |
| **Steam / multiplayer** | Not implemented. |
