package cz.empatix.interaction.service;

import cz.empatix.interaction.domain.Interactable;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Pure-logic coverage for {@link InteractionService}'s registry and AABB {@code query}. A fresh
 * instance is built per test (rather than the static singleton) so registration state is isolated;
 * interactables are mocks carrying only the geometry the spatial query reads.
 */
class InteractionServiceTest {

    private static Interactable interactableAt(final float x, final float y, final int w, final int h) {
        final Interactable interactable = mock(Interactable.class);
        when(interactable.getPosition()).thenReturn(new Vector2f(x, y));
        when(interactable.getWidth()).thenReturn(w);
        when(interactable.getHeight()).thenReturn(h);
        return interactable;
    }

    @Test
    void queryReturnsInteractablesInsideTheBox() {
        final InteractionService service = new InteractionService();
        final Interactable inside = interactableAt(50, 50, 10, 10);
        service.addInteractable(inside);

        final List<Interactable> hits = service.query(new Vector2f(0, 0), new Vector2i(100, 100));

        assertEquals(1, hits.size());
        assertSame(inside, hits.get(0));
    }

    @Test
    void queryExcludesInteractablesOutsideTheBox() {
        final InteractionService service = new InteractionService();
        service.addInteractable(interactableAt(5000, 5000, 10, 10));

        final List<Interactable> hits = service.query(new Vector2f(0, 0), new Vector2i(100, 100));

        assertTrue(hits.isEmpty());
    }

    @Test
    void queryDetectsOverlapAtTheBoxEdge() {
        final InteractionService service = new InteractionService();
        // right edge of the interactable just reaches the left edge of the query box
        final Interactable edge = interactableAt(-10, 50, 10, 10);
        service.addInteractable(edge);

        final List<Interactable> hits = service.query(new Vector2f(0, 0), new Vector2i(100, 100));

        assertEquals(1, hits.size());
    }

    @Test
    void removeInteractableTakesItOutOfQueries() {
        final InteractionService service = new InteractionService();
        final Interactable target = interactableAt(50, 50, 10, 10);
        service.addInteractable(target);

        service.removeInteractable(target);

        assertTrue(service.query(new Vector2f(0, 0), new Vector2i(100, 100)).isEmpty());
    }

    @Test
    void clearEmptiesTheRegistry() {
        final InteractionService service = new InteractionService();
        service.addInteractable(interactableAt(50, 50, 10, 10));

        service.clear();

        assertTrue(service.query(new Vector2f(0, 0), new Vector2i(100, 100)).isEmpty());
    }

    @Test
    void fixedUpdatePrunesRemovableInteractables() {
        final InteractionService service = new InteractionService();
        final Interactable stale = interactableAt(50, 50, 10, 10);
        when(stale.canRemove()).thenReturn(true);
        service.addInteractable(stale);

        service.fixedUpdate(0.016f);

        assertTrue(service.query(new Vector2f(0, 0), new Vector2i(100, 100)).isEmpty());
    }

    @Test
    void fixedUpdateKeepsLiveInteractables() {
        final InteractionService service = new InteractionService();
        final Interactable live = interactableAt(50, 50, 10, 10);
        when(live.canRemove()).thenReturn(false);
        service.addInteractable(live);

        service.fixedUpdate(0.016f);

        assertFalse(service.query(new Vector2f(0, 0), new Vector2i(100, 100)).isEmpty());
    }
}
