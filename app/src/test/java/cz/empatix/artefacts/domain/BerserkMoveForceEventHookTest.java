package cz.empatix.artefacts.domain;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

/** Covers the pure move-force scaling of {@link BerserkMoveForceEventHook} (no assets required). */
class BerserkMoveForceEventHookTest {

    @Test
    void increasesMoveForceByThirty() {
        final BerserkMoveForceEventHook hook = new BerserkMoveForceEventHook();

        // 1000 * 1.3 = 1300
        assertEquals(1300L, hook.onMoveForce(1000L));
    }

    @Test
    void zeroMoveForceStaysZero() {
        final BerserkMoveForceEventHook hook = new BerserkMoveForceEventHook();

        assertEquals(0L, hook.onMoveForce(0L));
    }

    @Test
    void truncatesTowardsZero() {
        final BerserkMoveForceEventHook hook = new BerserkMoveForceEventHook();

        // 7 * 1.3 = 9.1 -> truncated to 9
        assertEquals(9L, hook.onMoveForce(7L));
    }
}
