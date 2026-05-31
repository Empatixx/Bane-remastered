package cz.empatix.enemies.skills;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.collision.DamageSourceType;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.render.domain.RenderableState;
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
 * Headless coverage for the projectile-spawning skills and bullet entities. Bullets build a real
 * physics body (headless-safe {@link PhysicsService}) and a light source (headless-safe
 * LightningService) and fetch their texture through the stubbed {@link AssetsService}. We assert
 * damage values, removal/lifecycle flags and bullet-list accounting rather than dyn4j positions.
 */
class ProjectileSkillsHeadlessTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    @Test
    void slimebulletReportsEnemyDamageAndIsNotInitiallyRemovable() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final GreenSlimebullet bullet = new GreenSlimebullet();

            assertEquals(1, bullet.damage(), "slimebullets deal one point of damage");
            assertEquals(DamageSourceType.ENEMY, bullet.type());
            assertFalse(bullet.shouldRemove(), "a fresh bullet is alive");
        }
    }

    @Test
    void everySlimebulletColourVariantConstructsHeadlessly() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final BaseSlimebullet green = new GreenSlimebullet();
            final BaseSlimebullet red = new RedSlimebullet();
            final BaseSlimebullet purple = new PurpleSlimebullet();

            for (final BaseSlimebullet bullet : new BaseSlimebullet[]{green, red, purple}) {
                assertEquals(1, bullet.damage());
                assertFalse(bullet.shouldRemove());
            }
        }
    }

    @Test
    void slimebulletSetDirectionAndUpdateRunWithoutThrowing() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final RedSlimebullet bullet = new RedSlimebullet();
            bullet.setPosition(100f, 100f);

            bullet.setDirection(100f, 100f, 300f, 100f, 0f);
            bullet.fixedUpdate(0.016);
            bullet.update(0.016);

            assertFalse(bullet.shouldRemove(), "moving bullet that has not collided is still alive");
        }
    }

    @Test
    void slimeSmashShootsAFullVolleyOnceOffCooldown() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            // cooldown Timer is never started -> finished -> activate fires immediately.
            final SlimeSmash smash = new SlimeSmash(() -> new Vector2f(50f, 50f), GreenSlimebullet::new);

            assertTrue(smash.noBullets(), "no bullets before activation");
            smash.activate();
            assertFalse(smash.noBullets(), "a volley of bullets was spawned");

            // Second immediate activate is blocked by the now-running cooldown -> count unchanged.
            final RenderableState before = new RenderableState(32);
            smash.collectSubRenderables(before);
            final int countAfterFirst = before.getList().size();

            smash.activate();
            final RenderableState after = new RenderableState(32);
            smash.collectSubRenderables(after);
            assertEquals(countAfterFirst, after.getList().size(), "cooldown blocks a second volley");
        }
    }

    @Test
    void slimeSmashClearDestroysAndEmptiesBullets() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final SlimeSmash smash = new SlimeSmash(() -> new Vector2f(0f, 0f), 7, GreenSlimebullet::new);
            smash.activate();
            assertFalse(smash.noBullets());

            smash.clear();

            assertTrue(smash.noBullets(), "clear destroys and removes every bullet");
        }
    }

    @Test
    void kingSlimeArmageddonSpawnsOneBulletPerActivationAndClears() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final KingSlimeArmageddon armageddon =
                    new KingSlimeArmageddon(() -> new Vector2f(10f, 10f), PurpleSlimebullet::new);

            assertTrue(armageddon.noBullets());
            // First activate: cooldown never started -> fires; spawns exactly one bullet.
            armageddon.activate();
            assertFalse(armageddon.noBullets(), "one armageddon bullet spawned");

            armageddon.clear();
            assertTrue(armageddon.noBullets(), "clear empties the armageddon bullet list");
        }
    }

    @Test
    void eyeWatcherProjectileExposesDamageAndLifecycleFlags() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final EyeWatcherProjectile projectile = new EyeWatcherProjectile();
            projectile.setPosition(120f, 120f);

            assertEquals(1, projectile.damage());
            assertEquals(DamageSourceType.ENEMY, projectile.type());
            assertEquals(16 * 3, projectile.getWidth());
            assertEquals(16 * 3, projectile.getHeight());
            assertFalse(projectile.shouldRemove(), "fresh projectile is alive");

            projectile.update(0.016);
            assertFalse(projectile.shouldRemove(), "update without collision keeps it alive");
            projectile.destroy(); // tears down light source + physics body without throwing
        }
    }
}
