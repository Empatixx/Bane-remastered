package cz.empatix.pets;

import cz.empatix.interaction.service.InteractionService;
import cz.empatix.items.domain.ItemDrop;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.Set;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Headless coverage for {@link DropFollowingStrategy}: a pet hunting nearby loot. Drop discovery is
 * driven through the real {@link InteractionService} singleton (cleared per test) and the A* target
 * is supplied as a mocked {@link ItemDrop}. We assert the target-acquisition flags and the abandon
 * conditions in {@code update} (item no longer interactable / removed), never exact physics.
 */
@ExtendWith(HeadlessAssets.class)
class DropFollowingStrategyTest {

    @BeforeEach
    void clearInteractionService() {
        InteractionService.getInstance().clear();
    }

    private static ItemDrop dropAt(final float x, final float y) {
        final ItemDrop drop = mock(ItemDrop.class);
        when(drop.getPosition()).thenReturn(new Vector2f(x, y));
        when(drop.getX()).thenReturn(x);
        when(drop.getY()).thenReturn(y);
        when(drop.getWidth()).thenReturn(32);
        when(drop.getHeight()).thenReturn(32);
        when(drop.canInteract(any())).thenReturn(true);
        when(drop.canRemove()).thenReturn(false);
        return drop;
    }

    @Test
    void hasNoTargetBeforeFollowing() {
        final DropFollowingStrategy strategy = new DropFollowingStrategy(new Pet());

        assertFalse(strategy.hasTarget());
    }

    @Test
    void findDropDoesNothingWhenNoDropsRegistered() {
        final DropFollowingStrategy strategy = new DropFollowingStrategy(new Pet());

        strategy.findDrop();

        assertFalse(strategy.hasTarget(), "an empty interaction service yields no drop to chase");
    }

    @Test
    void fixedUpdateWithoutTargetSearchesAndStaysTargetless() {
        final DropFollowingStrategy strategy = new DropFollowingStrategy(new Pet());

        strategy.fixedUpdate(0.016f);

        assertFalse(strategy.hasTarget());
    }

    @Test
    void hasTargetAfterFollowing() {
        final DropFollowingStrategy strategy = new DropFollowingStrategy(new Pet());

        strategy.follow(dropAt(800f, 800f));

        assertTrue(strategy.hasTarget());
    }

    @Test
    void directionIsFiniteWhileFollowingADrop() {
        final Pet pet = new Pet();
        pet.setPosition(100f, 100f);
        final DropFollowingStrategy strategy = new DropFollowingStrategy(pet);
        strategy.follow(dropAt(400f, 400f));

        strategy.fixedUpdate(0.016f);
        final Vector2f direction = strategy.getDirection();

        assertTrue(Float.isFinite(direction.x));
        assertTrue(Float.isFinite(direction.y));
    }

    @Test
    void updateDropsTargetWhenItCanNoLongerBeInteractedWith() {
        final Pet pet = new Pet();
        pet.setPosition(100f, 100f);
        final DropFollowingStrategy strategy = new DropFollowingStrategy(pet);
        final ItemDrop drop = dropAt(120f, 120f);
        when(drop.canInteract(Set.of())).thenReturn(false); // e.g. already collected
        strategy.follow(drop);
        assertTrue(strategy.hasTarget());

        strategy.update(0.016f);

        assertFalse(strategy.hasTarget(), "a no-longer-interactable drop is abandoned");
    }

    @Test
    void updateDropsTargetWhenItIsRemoved() {
        final Pet pet = new Pet();
        pet.setPosition(100f, 100f);
        final DropFollowingStrategy strategy = new DropFollowingStrategy(pet);
        final ItemDrop drop = dropAt(120f, 120f);
        when(drop.canRemove()).thenReturn(true); // despawned
        strategy.follow(drop);

        strategy.update(0.016f);

        assertFalse(strategy.hasTarget());
    }

    @Test
    void updateDropsTargetWhenItIsTooFarAway() {
        final Pet pet = new Pet();
        pet.setPosition(0f, 0f);
        final DropFollowingStrategy strategy = new DropFollowingStrategy(pet);
        // > AREA_LOOT (500) away from the pet
        final ItemDrop drop = dropAt(5000f, 5000f);
        strategy.follow(drop);

        strategy.update(0.016f);

        assertFalse(strategy.hasTarget(), "a drop beyond the loot radius is given up on");
    }

    @Test
    void updateKeepsTargetWhileItRemainsValidAndNear() {
        final Pet pet = new Pet();
        pet.setPosition(100f, 100f);
        final DropFollowingStrategy strategy = new DropFollowingStrategy(pet);
        strategy.follow(dropAt(150f, 150f));

        strategy.update(0.016f);

        assertTrue(strategy.hasTarget());
    }
}
