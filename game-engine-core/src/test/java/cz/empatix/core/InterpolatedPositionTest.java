package cz.empatix.core;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class InterpolatedPositionTest {

    private static final float EPS = 1e-5f;

    @Test
    void setPositionSnapsCurrentAndNextImmediately() {
        final InterpolatedPosition pos = new InterpolatedPosition(1f);

        pos.setPosition(3f, 4f);

        // next ("logical") position
        assertEquals(3f, pos.getX(), EPS);
        assertEquals(4f, pos.getY(), EPS);
        // interpolated ("render") position is snapped, no easing needed
        assertEquals(3f, pos.getInterpolatedX(), EPS);
        assertEquals(4f, pos.getInterpolatedY(), EPS);
    }

    @Test
    void updateLerpsHalfwayThenReachesTarget() {
        // delta = 1 so update(dt) advances t by exactly dt
        final InterpolatedPosition pos = new InterpolatedPosition(1f);
        pos.setPosition(0f, 0f);

        pos.setInterpolatedPosition(10f, 0f);
        // before any update the render position still sits at the previous spot
        assertEquals(0f, pos.getInterpolatedX(), EPS);

        pos.update(0.5f);
        assertEquals(5f, pos.getInterpolatedX(), EPS);

        pos.update(0.5f);
        assertEquals(10f, pos.getInterpolatedX(), EPS);
    }

    @Test
    void updateClampsAndNeverOvershoots() {
        final InterpolatedPosition pos = new InterpolatedPosition(1f);
        pos.setPosition(0f, 0f);
        pos.setInterpolatedPosition(10f, 0f);

        // a dt larger than delta must clamp t at 1, landing exactly on the target
        pos.update(5f);
        assertEquals(10f, pos.getInterpolatedX(), EPS);

        // further updates keep it pinned on the target
        pos.update(5f);
        assertEquals(10f, pos.getInterpolatedX(), EPS);
    }
}
