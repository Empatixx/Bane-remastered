package cz.empatix.pets;

import cz.empatix.enemies.domain.Enemy;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Headless coverage for {@link AttackFollowingStrategy}: a pet that chases the nearest enemy. Enemy
 * discovery runs against the real headless {@link PhysicsService} (empty world -> no target) and the
 * follow target is a mocked {@link Enemy}. We assert acquisition flags, the dead-enemy abandon path,
 * and finiteness of the produced direction.
 */
@ExtendWith(HeadlessAssets.class)
class AttackFollowingStrategyTest {

    private static Enemy enemyAt(final float x, final float y, final boolean dead) {
        final Enemy enemy = mock(Enemy.class);
        when(enemy.getPosition()).thenReturn(new Vector2f(x, y));
        when(enemy.getX()).thenReturn(x);
        when(enemy.getY()).thenReturn(y);
        when(enemy.getWidth()).thenReturn(32);
        when(enemy.getHeight()).thenReturn(32);
        when(enemy.isDead()).thenReturn(dead);
        return enemy;
    }

    @Test
    void hasNoTargetBeforeFollowing() {
        final AttackFollowingStrategy strategy = new AttackFollowingStrategy(new Pet());

        assertFalse(strategy.hasTarget());
    }

    @Test
    void findEnemyDoesNothingInAnEmptyWorld() {
        final AttackFollowingStrategy strategy = new AttackFollowingStrategy(new Pet());

        strategy.findEnemy();

        assertFalse(strategy.hasTarget());
    }

    @Test
    void fixedUpdateWithoutTargetSearchesAndStaysTargetless() {
        final AttackFollowingStrategy strategy = new AttackFollowingStrategy(new Pet());

        strategy.fixedUpdate(0.016f);

        assertFalse(strategy.hasTarget());
    }

    @Test
    void hasTargetAfterFollowing() {
        final AttackFollowingStrategy strategy = new AttackFollowingStrategy(new Pet());

        strategy.follow(enemyAt(800f, 800f, false));

        assertTrue(strategy.hasTarget());
    }

    @Test
    void directionIsFiniteWhileChasingAnEnemy() {
        final Pet pet = new Pet();
        pet.setPosition(100f, 100f);
        final AttackFollowingStrategy strategy = new AttackFollowingStrategy(pet);
        strategy.follow(enemyAt(400f, 400f, false));

        strategy.fixedUpdate(0.016f);
        final Vector2f direction = strategy.getDirection();

        assertTrue(Float.isFinite(direction.x));
        assertTrue(Float.isFinite(direction.y));
    }

    @Test
    void updateAbandonsADeadEnemy() {
        final AttackFollowingStrategy strategy = new AttackFollowingStrategy(new Pet());
        strategy.follow(enemyAt(200f, 200f, true));
        assertTrue(strategy.hasTarget());

        strategy.update(0.016f);

        assertFalse(strategy.hasTarget(), "a dead enemy is dropped as a target");
    }

    @Test
    void updateKeepsALivingEnemy() {
        final Pet pet = new Pet();
        pet.setPosition(100f, 100f);
        final AttackFollowingStrategy strategy = new AttackFollowingStrategy(pet);
        strategy.follow(enemyAt(200f, 200f, false));

        strategy.update(0.016f);

        assertTrue(strategy.hasTarget());
    }
}
