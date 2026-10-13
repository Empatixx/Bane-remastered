package cz.empatix.interaction.service;

import org.junit.jupiter.api.Test;

import java.lang.reflect.Field;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Pure-logic coverage for {@link InteractionTracker}: a short-lived "is interacting" flag based on
 * the wall-clock time of the last interaction. The expiry window is forced by back-dating the
 * {@code lastInteraction} field via reflection rather than sleeping.
 */
class InteractionTrackerTest {

    private static void setLastInteraction(final InteractionTracker tracker, final long value) throws Exception {
        final Field field = InteractionTracker.class.getDeclaredField("lastInteraction");
        field.setAccessible(true);
        field.setLong(tracker, value);
    }

    @Test
    void freshTrackerIsNotInteracting() {
        // lastInteraction defaults to 0 (epoch) -> the window has long since elapsed
        assertFalse(new InteractionTracker().isInteracting());
    }

    @Test
    void isInteractingImmediatelyAfterAnInteraction() {
        final InteractionTracker tracker = new InteractionTracker();

        tracker.interaction();

        assertTrue(tracker.isInteracting(), "within the 100ms window the tracker reports interacting");
    }

    @Test
    void stopsInteractingOnceTheWindowElapses() throws Exception {
        final InteractionTracker tracker = new InteractionTracker();
        tracker.interaction();

        // force the last interaction to be well outside the 100ms window
        setLastInteraction(tracker, System.currentTimeMillis() - 10_000);

        assertFalse(tracker.isInteracting());
    }
}
