package cz.empatix.utils;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class InterpolatedFloatTest {

    @Test
    void startsAtInitialAndTargetEqualsInitial() {
        final InterpolatedFloat f = new InterpolatedFloat(3f, 5f);
        assertEquals(3f, f.get(), 1e-6);
        assertEquals(3f, f.getTarget(), 1e-6);
    }

    @Test
    void setTargetUpdatesTargetButNotCurrent() {
        final InterpolatedFloat f = new InterpolatedFloat(0f, 5f);
        f.setTarget(10f);
        assertEquals(10f, f.getTarget(), 1e-6);
        assertEquals(0f, f.get(), 1e-6);
    }

    @Test
    void updateMovesCurrentTowardTargetByLerpFactor() {
        // speed*dt = 2*0.25 = 0.5 -> halfway
        final InterpolatedFloat f = new InterpolatedFloat(0f, 2f);
        f.setTarget(10f);
        f.update(0.25f);
        assertEquals(5f, f.get(), 1e-5);
    }

    @Test
    void lerpFactorClampedToOneSnapsToTarget() {
        // speed*dt = 10*1 = 10 -> clamped to 1 -> reaches target in one step
        final InterpolatedFloat f = new InterpolatedFloat(0f, 10f);
        f.setTarget(7f);
        f.update(1f);
        assertEquals(7f, f.get(), 1e-6);
    }

    @Test
    void convergesTowardTargetOverManySteps() {
        final InterpolatedFloat f = new InterpolatedFloat(0f, 4f);
        f.setTarget(100f);
        for (int i = 0; i < 200; i++) {
            f.update(0.1f);
        }
        assertEquals(100f, f.get(), 1e-2);
    }

    @Test
    void noTargetChangeMeansNoMovement() {
        final InterpolatedFloat f = new InterpolatedFloat(42f, 3f);
        f.update(0.5f);
        assertEquals(42f, f.get(), 1e-6);
    }

    @Test
    void handlesDecreasingTarget() {
        final InterpolatedFloat f = new InterpolatedFloat(10f, 2f);
        f.setTarget(0f);
        f.update(0.25f); // halfway
        assertEquals(5f, f.get(), 1e-5);
    }
}
