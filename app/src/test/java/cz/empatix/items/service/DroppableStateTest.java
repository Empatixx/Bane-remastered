package cz.empatix.items.service;

import org.junit.jupiter.api.Test;

import java.util.concurrent.atomic.AtomicBoolean;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/** Covers the edge-triggered behaviour of {@link DroppableState}. */
class DroppableStateTest {

    @Test
    void dropsOnceWhenConditionFirstBecomesTrue() {
        final AtomicBoolean condition = new AtomicBoolean(false);
        final DroppableState state = new DroppableState(condition::get);

        assertFalse(state.canDrop()); // condition false

        condition.set(true);
        assertTrue(state.canDrop());  // rising edge -> drop
        assertFalse(state.canDrop()); // still true, but already dropped -> no second drop
    }

    @Test
    void rearmsAfterConditionGoesFalseAgain() {
        final AtomicBoolean condition = new AtomicBoolean(true);
        final DroppableState state = new DroppableState(condition::get);

        assertTrue(state.canDrop());  // first true -> drop
        assertFalse(state.canDrop()); // latched

        condition.set(false);
        assertFalse(state.canDrop()); // resets the latch

        condition.set(true);
        assertTrue(state.canDrop());  // new rising edge -> drop again
    }
}
