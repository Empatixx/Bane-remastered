package cz.empatix.utils;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

class InterpolatedAngleTest {

    private static final float PI = (float) Math.PI;

    @Test
    void startsAtInitial() {
        final InterpolatedAngle a = new InterpolatedAngle(1f, 5f);
        assertEquals(1f, a.get(), 1e-6);
        assertEquals(1f, a.getTarget(), 1e-6);
    }

    @Test
    void setTargetUpdatesTarget() {
        final InterpolatedAngle a = new InterpolatedAngle(0f, 5f);
        a.setTarget(2f);
        assertEquals(2f, a.getTarget(), 1e-6);
        assertEquals(0f, a.get(), 1e-6);
    }

    @Test
    void interpolatesHalfwayWithinSameRange() {
        // diff = 2-0 = 2 (within [-PI,PI]); t = 1*0.5 = 0.5 -> +1
        final InterpolatedAngle a = new InterpolatedAngle(0f, 1f);
        a.setTarget(2f);
        a.update(0.5f);
        assertEquals(1f, a.get(), 1e-5);
    }

    @Test
    void takesShortestPathWrappingDownward() {
        // current near -PI, target near +PI: naive diff ~ +2PI, shortest path goes
        // the negative (short) way. diff normalized into [-PI,PI] is small negative.
        final InterpolatedAngle a = new InterpolatedAngle(-PI + 0.1f, 1f);
        a.setTarget(PI - 0.1f);
        a.update(1f); // t clamped to 1, current += diff
        // shortest path: should land near +PI-0.1 by wrapping below -PI
        final float expected = (-PI + 0.1f) + (-0.2f); // diff = (PI-0.1)-(-PI+0.1)=2PI-0.2 -> -0.2
        assertEquals(expected, a.get(), 1e-4);
        // and the magnitude moved is small (shortest path), not ~2PI
        assertTrue(Math.abs(a.get() - (-PI + 0.1f)) < 0.5f);
    }

    @Test
    void takesShortestPathWrappingUpward() {
        // current near +PI, target near -PI: shortest path goes positive (wrap above +PI)
        final InterpolatedAngle a = new InterpolatedAngle(PI - 0.1f, 1f);
        a.setTarget(-PI + 0.1f);
        a.update(1f);
        // diff = (-PI+0.1)-(PI-0.1) = -2PI+0.2 -> +0.2 after wrap
        final float expected = (PI - 0.1f) + 0.2f;
        assertEquals(expected, a.get(), 1e-4);
    }

    @Test
    void speedClampedToOneReachesTargetInOneStep() {
        final InterpolatedAngle a = new InterpolatedAngle(0f, 100f);
        a.setTarget(1f);
        a.update(1f); // t clamped to 1
        assertEquals(1f, a.get(), 1e-5);
    }

    @Test
    void noTargetChangeMeansNoMovement() {
        final InterpolatedAngle a = new InterpolatedAngle(0.7f, 3f);
        a.update(0.5f);
        assertEquals(0.7f, a.get(), 1e-6);
    }

    @Test
    void convergesTowardTargetOverManySteps() {
        final InterpolatedAngle a = new InterpolatedAngle(-2f, 3f);
        a.setTarget(2f);
        for (int i = 0; i < 200; i++) {
            a.update(0.1f);
        }
        // diff = 2-(-2) = 4 > PI, so the shortest path wraps the negative way and
        // converges to an angle congruent to the target modulo 2*PI (here 2 - 2*PI).
        final double twoPi = 2 * Math.PI;
        double delta = a.get() - 2f;
        delta -= twoPi * Math.round(delta / twoPi);
        assertEquals(0d, delta, 1e-2);
    }
}
