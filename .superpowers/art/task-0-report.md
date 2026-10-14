# Task 0 report

Done in /Users/jirka/IdeaProjects/bane-art (branch new-artefacts). 8 icons generated, tex::ARTEFACTS_{HOURGLASS_OF_ASH,BLOOD_CHALICE,PHANTOM_CLOAK,GRAVE_LANTERN,MIDAS_COIN,THUNDER_IDOL,INFINITY_QUIVER,BONE_TURRET} registered (texture_keys + GAME_TEXTURES).

## Names for later tasks
- ArtefactKind: HourglassOfAsh, BloodChalice, PhantomCloak, GraveLantern, MidasCoin, ThunderIdol, InfinityQuiver, BoneTurret; ALL: [Self; 13].
- Constants (crate::artefacts): TIME_SLOW_SECONDS 6.0, TIME_SLOW_SCALE 0.35, BLOODLUST_SECONDS 10.0, CHALICE_HEAL 0.5, PHANTOM_SECONDS 5.0, MIDAS_SECONDS 6.0, INFINITE_AMMO_SECONDS 8.0.
- Components (crate::components): TimeSlow/Bloodlust/Phantom/InfiniteAmmo/MidasTouch { remaining: f32 }; unit markers SummonGhosts, CastLightning, PlaceTurret, ConvertBullets; HealPercent(pub(crate) f32).
- systems::tick_artefact_effects(world: &mut World, dt: f32), called in InGameScene::update after tick_berserk.
- Activation arms only insert components on the player.

## Tests
cargo test: 718 passed, 0 failed. clippy --all-targets -D warnings clean.
New: every_new_artefact_inserts_its_components_and_consumes_the_slot, timed_effects_expire_after_their_duration, using_a_timed_artefact_again_resets_its_timer; renamed there_are_thirteen_distinct_artefacts.

## Notes
- WGSL naga tests (render/mod.rs shader_tests) and the naga 30.0.1 dev-dependency already existed in the branch; nothing added, tests pass.
- #[allow(dead_code)] on: all 10 new components (TimeSlow, Bloodlust, Phantom, InfiniteAmmo, MidasTouch, SummonGhosts, CastLightning, PlaceTurret, ConvertBullets, HealPercent) and const TIME_SLOW_SCALE. Remove as later tasks use them.
- Pre-existing rustfmt diff on artefacts.rs line 1 (import order) left untouched.
- Re-activation reset works because insert overwrites the component.
