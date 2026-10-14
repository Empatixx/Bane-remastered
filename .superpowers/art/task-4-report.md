# Task 4 report: Blood Chalice + Midas Coin (worktree bane-art-4, branch art-t4)

## Built
- systems/blood.rs: apply_heal_percent (ceil(max*p), capped, Heal sound, request removed, 14-drop burst ring), bloodlust_heals_on_kill (+1 HP per finished non-minion kill while Bloodlust), spawn_bloodlust_drops (14/s, stateless from Bloodlust remaining), tick_blood_drops (rise/drag, frame by lifetime progress, fade), pulse_bloodlust (2 Hz, peak rgb (1,0.55,0.55), BloodTint stores original rgb and restores it exactly, never touches Color.w).
- systems/midas.rs: convert_bullets (detach, despawn, Coin item despawns=true, 6-frame sparkle one-shot with Expiring so remove_expired despawns it), midas_kill_coins (2 coins, opposite scatter at 90 px/s, per finished kill without NoDrop while MidasTouch).
- Wired in scenes/ingame.rs update: kill hooks right before enemy_drops; heal/convert/drops/pulse right after tick_artefact_effects. Removed dead_code allows from Bloodlust, MidasTouch, ConvertBullets, HealPercent.

## Textures
- blood_drops.tga: 6 x 16px (tiny, small, big, stretched drop, splash, fade specks); midas_sparkle.tga: 6 x 32px star with glints.
- Generators: tools/make_blood_fx.py, tools/make_midas_fx.py. Pass an output dir as argv[1] to also get an 8x preview png.
- Previews: /private/tmp/claude-501/-Users-jirka-IdeaProjects-Bane-refactor-rust/1111d5e4-569e-437c-9ed7-9a60743473a1/scratchpad/t4/{blood_drops,midas_sparkle}_preview.png
- Registered ARTEFACTS_BLOOD_DROPS, ARTEFACTS_MIDAS_SPARKLE in assets.rs.

## Tests
cargo test: 728+ passed (new: heal cap, burst ring, bloodlust heal and expiry, no heal without bloodlust, colour pulse and exact restore, dodge alpha untouched, droplets lifecycle, bullets to coins and out of physics, sparkle despawn, midas 2 coins and expiry). clippy -D warnings clean.

## Concerns
- Bloodlust kill heal counts non-minion finished deaths (same predicate as kill stats); Midas bonus counts all non-NoDrop deaths incl. minions.
- Not visually checked in a running game; only sprite sheets previewed.
- Bloodlust timer ticks in update (frame dt) so the pulse phase derives from it.
