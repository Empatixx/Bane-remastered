# Task 3 report: Infinity Quiver

Worktree /Users/jirka/IdeaProjects/bane-art-3, branch art-t3.

Built:
- systems/shooting.rs: with InfiniteAmmo on the player the clip never drops (burst and normal path), an empty clip still fires, any reload is cancelled each frame, the reload key is ignored.
- HUD: update_weapon_hud(world, textures, infinite: Option<f32>) where Some(clock) means active (ingame passes self.clock, lobby passes None). Pips tinted gold (1.0, 0.82, 0.25) with a sine brightness pulse (speed 5, +22%, offset 0.55 rad per pip) and always shown full; AmmoText gold and empty; AMMO_TEXT_COLOR moved to components/hud.rs so it can be restored.
- InfinityMark entity spawned hidden in prefabs/hud.rs::weapon_hud at (878, 503), 40x20 (2x of the 20x10 frame), layer 6.
- Texture assets/textures/ingame/artefacts/infinity_glint.tga, 80x10, 4 frames at 0.18 s, generator tools/make_infinity_fx.py; tex::ARTEFACTS_INFINITY_GLINT; sheets::infinity.
- Preview: /private/tmp/claude-501/-Users-jirka-IdeaProjects-Bane-refactor-rust/1111d5e4-569e-437c-9ed7-9a60743473a1/scratchpad/inf_preview.png (pre-final-glint tweak).
- Removed #[allow(dead_code)] from InfiniteAmmo. INFINITE_AMMO_SECONDS still has no allow.

Tests: 722 passed; clippy clean. New: clip does not drop while infinite and drops after expiry; reload cancelled and empty clip fires; HUD gold + infinity sprite only while active; shimmer travels.

Concerns: ∞ position/size not checked in the running game; the glint is subtle. Local rustfmt orders imports differently from the repo, so my import hunks follow the repo style.
