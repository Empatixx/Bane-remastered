package cz.empatix.interaction.domain;

import org.joml.Vector2f;
import org.junit.jupiter.api.Test;

import java.util.HashSet;
import java.util.Set;
import java.util.concurrent.atomic.AtomicReference;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

class CommonInteractableTest {

    @Test
    void delegatesInteractAndExposesGeometry() {
        final AtomicReference<Set<Integer>> received = new AtomicReference<>();
        final Vector2f pos = new Vector2f(3f, 4f);

        // constructor signature: (..., position, height, width)
        final CommonInteractable it = new CommonInteractable(
                received::set,
                keys -> keys.contains(42),
                () -> true,
                pos,
                10,   // height
                20);  // width

        assertEquals(20, it.getWidth());
        assertEquals(10, it.getHeight());
        assertSame(pos, it.getPosition());

        final Set<Integer> keys = new HashSet<>(Set.of(42));
        assertTrue(it.canInteract(keys));
        assertFalse(it.canInteract(new HashSet<>()));

        it.interact(keys);
        assertEquals(keys, received.get());
        assertTrue(it.canRemove());
    }

    @Test
    void supplierConstructorIgnoresKeysForCanInteract() {
        final CommonInteractable it = new CommonInteractable(
                keys -> { },
                (java.util.function.Supplier<Boolean>) () -> true,
                () -> false,
                new Vector2f(),
                5,
                6);

        // canInteract delegates to the supplier regardless of the keys passed in
        assertTrue(it.canInteract(new HashSet<>()));
        assertTrue(it.canInteract(new HashSet<>(Set.of(1, 2, 3))));
        assertFalse(it.canRemove());
    }

    @Test
    void setPositionMutatesUnderlyingVector() {
        final Vector2f pos = new Vector2f(0f, 0f);
        final CommonInteractable it = new CommonInteractable(
                keys -> { },
                keys -> true,
                () -> true,
                pos,
                1,
                1);
        it.setPosition(7.5f, -2.5f);
        assertEquals(7.5f, it.getPosition().x);
        assertEquals(-2.5f, it.getPosition().y);
    }
}
