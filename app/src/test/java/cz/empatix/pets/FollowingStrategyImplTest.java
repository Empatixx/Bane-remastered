package cz.empatix.pets;

import cz.empatix.enemies.targetting.Targetable;
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
 * Headless coverage for {@link FollowingStrategyImpl}: the plain A*-backed follow strategy used by
 * pets. We assert the target-state transitions and that the produced direction is finite — never the
 * exact path, which depends on the dyn4j world. Built against a real headless {@link PhysicsService}
 * (so the A* blocked-cell query resolves) with a real {@link Pet} as the owner.
 */
@ExtendWith(HeadlessAssets.class)
class FollowingStrategyImplTest {

    private static Targetable targetAt(final float x, final float y) {
        final Targetable target = mock(Targetable.class);
        when(target.getPosition()).thenReturn(new Vector2f(x, y));
        when(target.getX()).thenReturn(x);
        when(target.getY()).thenReturn(y);
        when(target.getWidth()).thenReturn(32);
        when(target.getHeight()).thenReturn(32);
        return target;
    }

    @Test
    void hasNoTargetBeforeFollowing() {
        final FollowingStrategyImpl strategy = new FollowingStrategyImpl(new Pet());

        assertFalse(strategy.hasTarget());
    }

    @Test
    void hasTargetAfterFollowing() {
        final FollowingStrategyImpl strategy = new FollowingStrategyImpl(new Pet());

        strategy.follow(targetAt(800f, 800f));

        assertTrue(strategy.hasTarget());
    }

    @Test
    void directionIsFiniteAfterAcquiringATarget() {
        final Pet pet = new Pet();
        pet.setPosition(100f, 100f);
        final FollowingStrategyImpl strategy = new FollowingStrategyImpl(pet);
        strategy.follow(targetAt(400f, 400f));

        strategy.fixedUpdate(0.016f);
        final Vector2f direction = strategy.getDirection();

        assertTrue(Float.isFinite(direction.x));
        assertTrue(Float.isFinite(direction.y));
    }

    @Test
    void updateAndFixedUpdateAreSafeWithoutATarget() {
        final FollowingStrategyImpl strategy = new FollowingStrategyImpl(new Pet());

        strategy.update(0.016f);
        strategy.fixedUpdate(0.016f);

        assertFalse(strategy.hasTarget());
    }
}
