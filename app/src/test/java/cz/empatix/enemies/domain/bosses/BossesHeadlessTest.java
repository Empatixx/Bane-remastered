package cz.empatix.enemies.domain.bosses;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.collision.DamageSourceType;
import cz.empatix.enemies.targetting.Targetable;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.testsupport.MockedAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mockStatic;

/**
 * Headless coverage for the bosses. Construction is GL-free under a stubbed {@link AssetsService}:
 * the {@code StatusBarType} enum, {@code StatusBar} and {@code CommonText} only touch the mocked
 * texture/font in their constructors. Font drawables (which would NPE on a mock) are produced only
 * in {@code collectSubRenderables}, which these tests deliberately avoid. The {@code Spider}'s 1.5s
 * spawn window is forced open via the protected {@code spawnTime} field (same package).
 */
class BossesHeadlessTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    private static Targetable target(final float x, final float y) {
        return new Targetable() {
            @Override
            public Vector2f getPosition() {
                return new Vector2f(x, y);
            }

            @Override
            public int getWidth() {
                return 48;
            }

            @Override
            public int getHeight() {
                return 48;
            }
        };
    }

    // ---- Golem (BossImpl-backed) --------------------------------------------------------------

    @Test
    void golemConstructsAliveWithFullHealth() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Golem golem = new Golem();

            assertFalse(golem.isDead());
            assertEquals(100, golem.getHealth(), "Golem starts at its full 100 health");
            assertTrue(golem.getWidth() > 0 && golem.getHeight() > 0);
            assertFalse(golem.canDropItems(), "the Golem never drops items");
        }
    }

    @Test
    void golemTakesDamageAndDiesWhenDepleted() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Golem golem = new Golem();

            assertEquals(10, golem.hit(10), "Golem registers the damage dealt");
            assertEquals(90, golem.getHealth());
            assertFalse(golem.isDead());

            golem.hit(1000);
            assertEquals(0, golem.getHealth(), "health clamps to zero");
            assertTrue(golem.isDead());
            // canRemove still false until the death animation has played out.
            assertFalse(golem.canRemove());
        }
    }

    @Test
    void golemHealIsCappedAtMax() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Golem golem = new Golem();
            golem.hit(40); // -> 60

            golem.heal(1000);

            assertEquals(100, golem.getHealth(), "heal is capped at max health");
        }
    }

    @Test
    void golemTargetingAndFixedUpdateAreSafe() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Golem golem = new Golem();
            golem.setPosition(0f, 0f);
            golem.setTarget(null); // Golem.setTarget forwards null to BossImpl/golemLaser safely

            // fixedUpdate early-returns after ticking the laser (guarded by if(true){return;}).
            golem.fixedUpdate(0.016);
            golem.update(0.016);

            assertFalse(golem.isDead(), "benign ticks leave the Golem alive");
        }
    }

    // ---- KingSlime (BossImpl-backed) ----------------------------------------------------------

    @Test
    void kingSlimeConstructsAliveAndTakesDamage() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final KingSlime kingSlime = new KingSlime();

            assertEquals(90, kingSlime.getHealth(), "KingSlime starts at full 90 health");
            assertFalse(kingSlime.isDead());

            kingSlime.hit(30);
            assertEquals(60, kingSlime.getHealth());

            kingSlime.hit(1000);
            assertTrue(kingSlime.isDead());
            assertEquals(0, kingSlime.getHealth());
        }
    }

    @Test
    void kingSlimeDespawnClearsSkillBulletsWithoutThrowing() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final KingSlime kingSlime = new KingSlime();
            kingSlime.setPosition(0f, 0f);
            kingSlime.setTarget(null);

            // despawn() clears armageddon + smash bullet lists.
            kingSlime.despawn();

            assertFalse(kingSlime.canDropItems(), "KingSlime does not drop items");
        }
    }

    // ---- Spider (BossBaseEnemy-backed) --------------------------------------------------------

    @Test
    void spiderConstructsSpawningAndInvincible() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Spider spider = new Spider();

            assertTrue(spider.isSpawning(), "Spider is spawning immediately after construction");
            assertEquals(75, spider.getHealth(), "Spider starts at full 75 health");
            assertEquals(0, spider.hit(10), "damage is ignored during the spawn window");
            assertEquals(75, spider.getHealth());
            assertEquals(0, spider.damage(), "deals no contact damage while spawning");
        }
    }

    @Test
    void spiderTakesPlayerDamageOnceSpawnWindowPasses() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Spider spider = new Spider();
            spider.spawnTime = 0; // leave the spawn-invincibility window

            assertEquals(0, spider.hit(DamageSourceType.ENEMY, 5), "enemy-sourced damage ignored");
            assertEquals(75, spider.getHealth());

            assertEquals(5, spider.hit(DamageSourceType.PLAYER, 5), "player damage applies");
            assertEquals(70, spider.getHealth());
            assertEquals(2, spider.damage(), "exposes contact damage when active");
            assertEquals(DamageSourceType.ENEMY, spider.type());
        }
    }

    @Test
    void spiderDiesWhenHealthDepletedAndIsNotYetRemovable() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Spider spider = new Spider();
            spider.spawnTime = 0;

            spider.hit(DamageSourceType.PLAYER, 75);

            assertTrue(spider.isDead());
            assertEquals(0, spider.getHealth());
            assertFalse(spider.canRemove(), "dead boss waits for its death animation and spawned adds");
        }
    }

    @Test
    void spiderTargetingDirectionIsFiniteAfterTick() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Spider spider = new Spider();
            spider.spawnTime = 0;
            spider.setPosition(0f, 0f);
            spider.aStarTargetting.target(target(300f, 0f));

            spider.aStarTargetting.updatePath();
            spider.aStarTargetting.update(0.016f);
            final Vector2f direction = spider.aStarTargetting.getDirectionTarget();

            assertTrue(Float.isFinite(direction.x) && Float.isFinite(direction.y),
                    "spider direction is finite, got " + direction);
        }
    }
}
