package cz.empatix.interaction.domain;

import org.joml.Vector2f;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

class CommonInteractEntityTest {

    @Test
    void exposesGeometryAndStartsWithNoKeys() {
        final Vector2f pos = new Vector2f(1f, 2f);
        final CommonInteractEntity e = new CommonInteractEntity(30, 40, pos);

        assertEquals(30, e.getWidth());
        assertEquals(40, e.getHeight());
        assertSame(pos, e.getPosition());
        assertTrue(e.getInteractKeys().isEmpty());
    }

    @Test
    void addKeyAccumulatesUniqueKeys() {
        final CommonInteractEntity e = new CommonInteractEntity(1, 1, new Vector2f());
        e.addKey(5);
        e.addKey(7);
        e.addKey(5); // duplicate ignored by the set

        assertEquals(2, e.getInteractKeys().size());
        assertTrue(e.getInteractKeys().contains(5));
        assertTrue(e.getInteractKeys().contains(7));
    }

    @Test
    void canRemoveThrows() {
        final CommonInteractEntity e = new CommonInteractEntity(1, 1, new Vector2f());
        assertThrows(IllegalStateException.class, e::canRemove);
    }
}
