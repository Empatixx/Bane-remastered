package cz.empatix.hud.domain;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

class StatusControllerTest {

    private static final double TWEEN = 0.05;

    @Test
    void freshControllerReportsFullDeltas() {
        final StatusController c = new StatusController();
        // maxValue == 0 -> delta() returns 1
        assertEquals(1f, c.delta());
        assertEquals(1f, c.delayedDelta());
    }

    @Test
    void deltaIsCurrentOverMax() {
        final StatusController c = new StatusController();
        c.update(0.016, 50, 100);
        assertEquals(0.5f, c.delta());
    }

    @Test
    void deltaIsOneWhenMaxIsZero() {
        final StatusController c = new StatusController();
        c.update(0.016, 7, 0);
        assertEquals(1f, c.delta());
    }

    @Test
    void delayedDeltaTweensTowardsDeltaOverTime() {
        final StatusController c = new StatusController();
        // first update: delta() still uses pre-update (max==0 -> 1), so delayedDelta stays 1
        c.update(0.1, 0, 100);
        assertEquals(1f, c.delayedDelta(), 1e-6);

        // second update with same dt: now delta() == 0, delayedDelta moves toward 0
        c.update(0.1, 0, 100);
        final float expected = (float) ((1.0 - 0.0) * Math.pow(TWEEN, 0.1) + 0.0);
        assertEquals(expected, c.delayedDelta(), 1e-6);

        // it should be strictly between the target (0) and the start (1)
        assertTrue(c.delayedDelta() > 0f && c.delayedDelta() < 1f);
    }

    @Test
    void delayedDeltaConvergesTowardsDeltaAcrossManyTicks() {
        final StatusController c = new StatusController();
        // drive current/max to a low ratio and tick many times
        for (int i = 0; i < 500; i++) {
            c.update(0.1, 0, 100);
        }
        // delta is 0; delayedDelta should have converged very close to 0
        assertEquals(0f, c.delayedDelta(), 1e-3);
    }
}
