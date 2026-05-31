package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.collision.DamageSourceType;
import cz.empatix.render.domain.Animation;
import cz.empatix.animation.AnimationState;
import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.concurrent.atomic.AtomicInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Headless coverage for the composition-based {@link EnemyImpl}: direct health/heal logic plus the
 * injected hit-callback routing and trap-source invincibility. EnemyImpl has no spawn window so
 * damage logic is reachable immediately.
 */
@ExtendWith(HeadlessAssets.class)
class EnemyImplHeadlessTest {

    private static Animation animation() {
        final Animation animation = Animation.builder()
                .setTotal(1, 4)
                .addState(AnimationState.IDLE, 0, 4, 32, 32, 100)
                .texture(AssetsService.getTexture("ingame/test.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        return animation;
    }

    private static EnemyImpl enemy(final int health, final int damage, final java.util.function.Consumer<Integer> hitCallback) {
        return new EnemyImpl(40, 40, health, animation(), damage, hitCallback,
                () -> false, () -> 1_000_000, 5);
    }

    @Test
    void directHitReducesHealthAndClampsAtZero() {
        final EnemyImpl enemy = enemy(10, 2, d -> { });

        enemy.hit(4);
        assertFalse(enemy.isDead());

        enemy.hit(100);
        assertTrue(enemy.isDead(), "health clamps to zero and reports dead");
    }

    @Test
    void healIsCappedAtMaxAndIgnoredWhenDead() {
        final EnemyImpl enemy = enemy(10, 2, d -> { });
        enemy.hit(6); // -> 4

        enemy.heal(50);
        // No public health getter; round-trip through a lethal hit proves it healed back to 10.
        enemy.hit(9);
        assertFalse(enemy.isDead(), "after heal-to-max a 9-point hit leaves 1 health");
        enemy.hit(1);
        assertTrue(enemy.isDead());

        enemy.heal(5);
        assertTrue(enemy.isDead(), "dead enemy stays dead after heal");
    }

    @Test
    void hitSourceCallbackFiresOnlyForNonEnemySources() {
        final AtomicInteger callbackTotal = new AtomicInteger();
        final EnemyImpl enemy = enemy(20, 2, callbackTotal::addAndGet);

        enemy.hit(DamageSourceType.ENEMY, 5);
        assertEquals(0, callbackTotal.get(), "enemy-sourced damage does not invoke the callback");

        enemy.hit(DamageSourceType.PLAYER, 5);
        assertEquals(5, callbackTotal.get(), "player-sourced damage routes through the callback");
    }

    @Test
    void trapSourceRespectsInvincibilityWindow() {
        final AtomicInteger callbackTotal = new AtomicInteger();
        final EnemyImpl enemy = enemy(20, 2, callbackTotal::addAndGet);

        enemy.hit(DamageSourceType.TRAP, 3);
        assertEquals(3, callbackTotal.get(), "first trap hit lands");

        enemy.hit(DamageSourceType.TRAP, 3);
        assertEquals(3, callbackTotal.get(), "second immediate trap hit is blocked by invincibility");
    }

    @Test
    void gettersAndPositionReflectConstruction() {
        final EnemyImpl enemy = enemy(10, 7, d -> { });

        enemy.setPosition(60f, 90f);

        assertEquals(40, enemy.getWidth());
        assertEquals(40, enemy.getHeight());
        assertEquals(7, enemy.damage());
        assertEquals(DamageSourceType.ENEMY, enemy.type());
        assertEquals(60f, enemy.getX(), 0.0001f);
        assertEquals(90f, enemy.getY(), 0.0001f);
        assertEquals(60f, enemy.getPosition().x(), 0.0001f);
    }

    @Test
    void collisionAwareConstructorBuildsWithOffsetCollider() {
        final EnemyImpl enemy = new EnemyImpl(40, 40, 8, animation(), 3, d -> { },
                () -> false, () -> 1_000_000, 20, 20, new Vector2i(5, 5), 5);

        assertFalse(enemy.isDead());
        enemy.hit(8);
        assertTrue(enemy.isDead(), "offset-collider variant shares the same damage logic");
    }

    @Test
    void canRemoveOnlyAfterDeathWithEmptyVitalityDisplay() {
        final EnemyImpl enemy = enemy(5, 1, d -> { });

        assertFalse(enemy.canRemove(), "alive enemy is not removable");
        enemy.hit(5);
        // Just died: the death animation and the hit's vitality popup are still pending (both
        // wall-clock gated), so removal is held off until they finish.
        assertTrue(enemy.isDead());
        assertFalse(enemy.canRemove(), "freshly-killed enemy is not removable until death anim/popups finish");
    }

    @Test
    void onDeathDestroysPhysicsBodyWithoutThrowing() {
        final EnemyImpl enemy = enemy(5, 1, d -> { });
        enemy.setPosition(0f, 0f);

        enemy.onDeath();
        // Position is still queryable after the body is destroyed.
        final Vector2f position = enemy.getPosition();
        assertTrue(Float.isFinite(position.x) && Float.isFinite(position.y));
    }
}
