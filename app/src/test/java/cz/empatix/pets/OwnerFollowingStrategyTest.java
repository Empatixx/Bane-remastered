package cz.empatix.pets;

import cz.empatix.enemies.targetting.Targetable;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Headless coverage for {@link OwnerFollowingStrategy}: the strategy that keeps the pet glued to its
 * owner. We assert the {@code STOP_DISTANCE} dead-zone in {@code getDirection} (zero vector when the
 * pet is already on top of the owner) and that direction is finite when far, plus the null-target
 * guard in {@code fixedUpdate}. Built against a real headless {@link PhysicsService}.
 */
@ExtendWith(HeadlessAssets.class)
class OwnerFollowingStrategyTest {

    private static Targetable ownerAt(final float x, final float y) {
        final Targetable owner = mock(Targetable.class);
        when(owner.getPosition()).thenReturn(new Vector2f(x, y));
        when(owner.getX()).thenReturn(x);
        when(owner.getY()).thenReturn(y);
        when(owner.getWidth()).thenReturn(32);
        when(owner.getHeight()).thenReturn(32);
        return owner;
    }

    @Test
    void hasNoTargetBeforeFollowing() {
        final OwnerFollowingStrategy strategy = new OwnerFollowingStrategy(new Pet());

        assertFalse(strategy.hasTarget());
    }

    @Test
    void fixedUpdateIsSafeWithoutAnOwner() {
        final OwnerFollowingStrategy strategy = new OwnerFollowingStrategy(new Pet());

        strategy.fixedUpdate(0.016f);

        assertFalse(strategy.hasTarget());
    }

    @Test
    void hasTargetAfterFollowing() {
        final OwnerFollowingStrategy strategy = new OwnerFollowingStrategy(new Pet());

        strategy.follow(ownerAt(300f, 300f));

        assertTrue(strategy.hasTarget());
    }

    @Test
    void directionIsZeroWhenAlreadyOnTopOfTheOwner() {
        final Pet pet = new Pet();
        pet.setPosition(100f, 100f);
        final OwnerFollowingStrategy strategy = new OwnerFollowingStrategy(pet);
        // owner centred near the pet -> centre distance below STOP_DISTANCE (100)
        strategy.follow(ownerAt(100f, 100f));

        final Vector2f direction = strategy.getDirection();

        assertEquals(0f, direction.x, 0.0001f);
        assertEquals(0f, direction.y, 0.0001f);
    }

    @Test
    void directionIsFiniteWhenTheOwnerIsFarAway() {
        final Pet pet = new Pet();
        pet.setPosition(100f, 100f);
        final OwnerFollowingStrategy strategy = new OwnerFollowingStrategy(pet);
        strategy.follow(ownerAt(400f, 400f));
        strategy.fixedUpdate(0.016f);

        final Vector2f direction = strategy.getDirection();

        assertTrue(Float.isFinite(direction.x));
        assertTrue(Float.isFinite(direction.y));
    }
}
