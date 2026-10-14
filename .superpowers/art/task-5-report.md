# Task 5 report: Grave Lantern + Bone Turret (branch art-t5, /Users/jirka/IdeaProjects/bane-art-5)

## Built
- `systems/summons.rs`: `spawn_summons` (consumes SummonGhosts/PlaceTurret), `tick_ghosts`, `tick_turrets`; `components/summons.rs`: Ghost, Turret, TurretStage; sheet layouts in `sheets.rs` (ghost, turret). Wired in `scenes/ingame.rs` (spawn after tick_artefact_effects in update; ticks in fixed_update after pet_bites).
- Ghosts: 3, 12 s, no body, 300 px/s smoothed steering to nearest live enemy (no range limit), sine bob 5 px, 2 damage (DamageSource::Bite) per 0.5 s cooldown on overlap, orbit the player at 110 px with no enemy, fade alpha over last 1 s, soft green Light (fades too). Hit flash row plays on a hit then ThenPlay float.
- Turret: no physics body (decision: doesn't block the player), 12 s active after 0.5 s rise, fires every 0.35 s at nearest enemy within 600 px via prefabs::bullet + Bullet::plain(2), 700 px/s, bone shard texture; 5 aim rows (down, 45, side, 135, up; flipped for left) with 3-frame recoil; crumble 0.6 s then despawn. Stage timers are tick-driven, not animation-driven.
- Size: ghost 72 px, turret 120 px (art 24 / 40 px at ~3x, like the pet).

## Textures (assets/textures/ingame/artefacts/, registered as ARTEFACTS_GHOST/TURRET/BONE_SHARD)
- ghost.tga 4x2 of 24px (float x4, hit x3); turret.tga 5x7 of 40px (rise, crumble, 5 aim rows with idle/shot-flash/recoil); bone_shard.tga 4x2 of 16px (spin row, impact row, matches BULLET sheet layout).
- Generators: tools/make_ghost_fx.py, tools/make_turret_fx.py. Previews: scratchpad ghost_prev.png, turret_prev.png, shard_prev.png.

## Tests
9 new in systems/summons.rs. cargo test: 727 passed, 0 failed; clippy --all-targets -D warnings clean.

## Concerns
- Front-facing barrel reads as a dark blob on the skull; not seen in-game.
- Bullet from turret may stop on walls between turret and enemy (no LOS check).
- Ghost damage uses DamageSource::Bite (flashes enemy; counts as a hit stat).
