package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.collision.DamageSourceType;
import cz.empatix.enemies.service.DifficultyService;
import cz.empatix.render.domain.Animation;
import cz.empatix.animation.AnimationState;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Exercises the shared {@link BaseEnemy} damage/heal/spawn state machine through a minimal real
 * subclass built headlessly. {@code spawnTime} and {@code trapInvincibility} are protected fields
 * declared in {@code BaseEnemy} (same package as this test), so time-gated branches are forced by
 * assignment rather than sleeping.
 */
@ExtendWith(HeadlessAssets.class)
class BaseEnemyHeadlessTest {

    /** Real BaseEnemy subclass with a tiny GL-free animation and tracked death/spawn callbacks. */
    private static final class TestEnemy extends BaseEnemy {
        private boolean died;
        private boolean spawned;

        TestEnemy(final int health, final int damage) {
            super(48, 48);
            animation = Animation.builder()
                    .setTotal(2, 4)
                    .addState(AnimationState.IDLE, 0, 4, 48, 48, 100)
                    .addState(AnimationState.DEATH, 1, 4, 48, 48, 100)
                    .texture(AssetsService.getTexture("ingame/test.tga"))
                    .build();
            animation.setState(AnimationState.IDLE);
            this.maxHealth = this.health = health;
            this.damage = damage;
            this.moveForce = 1_000_000;
        }

        /** Disable the 2s spawn-invincibility window so hit/heal logic is reachable. */
        void leaveSpawn() {
            spawnTime = 0;
        }

        @Override
        public void onDeath() {
            died = true;
            animation.setState(AnimationState.DEATH);
        }

        @Override
        public void onSpawn() {
            spawned = true;
        }
    }

    private static TestEnemy enemy(final int health, final int damage) {
        final TestEnemy enemy = new TestEnemy(health, damage);
        enemy.leaveSpawn();
        return enemy;
    }

    @Test
    void freshlyConstructedEnemyIsSpawningAndInvincible() {
        final TestEnemy enemy = new TestEnemy(10, 2);

        assertTrue(enemy.isSpawning(), "just-constructed enemy is within the spawn window");
        assertEquals(0, enemy.hit(5), "damage is ignored while spawning");
        assertEquals(10, enemy.getHealth(), "health untouched while spawning");
        assertEquals(0, enemy.damage(), "deals no contact damage while spawning");
    }

    @Test
    void hitReducesHealthOnceSpawnWindowHasPassed() {
        final TestEnemy enemy = enemy(10, 2);

        final int received = enemy.hit(3);

        assertEquals(3, received);
        assertEquals(7, enemy.getHealth());
        assertFalse(enemy.isDead());
        assertEquals(2, enemy.damage(), "exposes contact damage when not spawning");
    }

    @Test
    void lethalHitClampsHealthToZeroAndTriggersDeath() {
        final TestEnemy enemy = enemy(5, 2);

        enemy.hit(99);

        assertEquals(0, enemy.getHealth(), "health never goes negative");
        assertTrue(enemy.isDead());
        assertTrue(enemy.died, "onDeath fired exactly when health hit zero");
    }

    @Test
    void hittingADeadEnemyReturnsZero() {
        final TestEnemy enemy = enemy(5, 2);
        enemy.hit(5);

        assertEquals(0, enemy.hit(5), "no overkill damage is registered");
        assertTrue(enemy.isDead());
    }

    @Test
    void healIsCappedAtMaxHealthAndIgnoredWhenDead() {
        final TestEnemy enemy = enemy(10, 2);
        enemy.hit(4); // -> 6

        enemy.heal(100);
        assertEquals(10, enemy.getHealth(), "heal is clamped to maxHealth");

        enemy.hit(10); // kill
        enemy.heal(5);
        assertEquals(0, enemy.getHealth(), "dead enemies cannot be healed");
    }

    @Test
    void enemyDamageSourceIsRejectedButPlayerDamageApplies() {
        final TestEnemy enemy = enemy(10, 2);

        assertEquals(0, enemy.hit(DamageSourceType.ENEMY, 4), "enemy-sourced damage is ignored");
        assertEquals(10, enemy.getHealth());

        assertEquals(4, enemy.hit(DamageSourceType.PLAYER, 4), "player-sourced damage applies");
        assertEquals(6, enemy.getHealth());
    }

    @Test
    void trapDamageHonoursPerHitInvincibilityWindow() {
        final TestEnemy enemy = enemy(20, 2);

        final int first = enemy.hit(DamageSourceType.TRAP, 3);
        assertEquals(3, first, "first trap hit lands");
        assertEquals(17, enemy.getHealth());

        // Immediately re-applying trap damage is blocked by the 1s invincibility window.
        assertEquals(0, enemy.hit(DamageSourceType.TRAP, 3), "trap hit during invincibility ignored");
        assertEquals(17, enemy.getHealth());

        // Force the window open and the next trap hit lands again.
        enemy.trapInvincibility = 0;
        assertEquals(3, enemy.hit(DamageSourceType.TRAP, 3));
        assertEquals(14, enemy.getHealth());
    }

    @Test
    void typeAndDeadEnemyCannotRemoveUntilAnimationPlayedOut() {
        final TestEnemy enemy = enemy(3, 1);

        assertEquals(DamageSourceType.ENEMY, enemy.type());
        assertFalse(enemy.canRemove(), "alive enemy cannot be removed");

        enemy.hit(3);
        // Dead but the death animation has not finished a full cycle yet.
        assertTrue(enemy.isDead());
        assertFalse(enemy.canRemove(), "dead enemy stays until death animation completes");
    }

    @Test
    void positionAndDimensionGettersReflectState() {
        final TestEnemy enemy = enemy(5, 1);

        enemy.setPosition(150f, 275f);

        assertEquals(150f, enemy.getX(), 0.0001f);
        assertEquals(275f, enemy.getY(), 0.0001f);
        assertEquals(150f, enemy.getPosition().x(), 0.0001f);
        assertEquals(48, enemy.getWidth());
        assertEquals(48, enemy.getHeight());
        assertEquals(3, enemy.getZIndex());
    }

    @Test
    void despawnDefaultDealsCurrentHealthAsLethalDamage() {
        final TestEnemy enemy = enemy(7, 1);

        enemy.despawn(); // default Enemy.despawn -> hit(getHealth())

        assertTrue(enemy.isDead(), "despawn drops the enemy by dealing its remaining health");
        assertEquals(0, enemy.getHealth());
    }

    @Test
    void scaleToFloorRaisesHealthAndDamageByTheFloorMultipliers() {
        final TestEnemy enemy = enemy(10, 5); // past the spawn window

        enemy.scaleToFloor(new DifficultyService(), 5);

        // health: round(10 * (1 + 0.12*5)) = round(16.0) = 16, current health refilled to the new max
        assertEquals(16, enemy.getHealth());
        // damage: round(5 * (1 + 0.08*5)) = round(7.0) = 7
        assertEquals(7, enemy.damage());
    }

    @Test
    void scaleToFloorZeroLeavesStatsUnchanged() {
        final TestEnemy enemy = enemy(9, 2);

        enemy.scaleToFloor(new DifficultyService(), 0);

        assertEquals(9, enemy.getHealth());
        assertEquals(2, enemy.damage());
    }
}
