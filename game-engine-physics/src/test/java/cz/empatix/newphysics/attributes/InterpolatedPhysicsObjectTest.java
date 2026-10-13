package cz.empatix.newphysics.attributes;

import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.geometry.MassType;
import org.dyn4j.geometry.Rectangle;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class InterpolatedPhysicsObjectTest {

    private static final float EPS = 1e-4f;

    /** delta is the fixed timestep (1/60); a dt of one full delta advances interpolation to the target. */
    private static final float ONE_STEP = 1f / 60f;

    private static InterpolatedPhysicsObject body() {
        final InterpolatedPhysicsObject o = new InterpolatedPhysicsObject();
        o.addFixture(new BodyFixture(new Rectangle(1, 1)));
        o.setMass(MassType.NORMAL);
        return o;
    }

    @Test
    void setTranslationSnapsInterpolatedPosition() {
        final InterpolatedPhysicsObject o = body();
        o.setTranslation(3f, 4f);

        assertEquals(3f, o.getTranslationX(), EPS);
        assertEquals(4f, o.getTranslationY(), EPS);
        final Vector2f t = o.getTranslation();
        assertEquals(3f, t.x, EPS);
        assertEquals(4f, t.y, EPS);
    }

    @Test
    void setPhysicsTranslationSnapsInterpolatedPosition() {
        final InterpolatedPhysicsObject o = body();
        o.setPhysicsTranslation(-1f, 9f);

        assertEquals(-1f, o.getTranslationX(), EPS);
        assertEquals(9f, o.getTranslationY(), EPS);
    }

    @Test
    void updateInterpolatedPositionSetsTargetButRenderStaysUntilUpdate() {
        final InterpolatedPhysicsObject o = body();
        o.setTranslation(0f, 0f);

        // Move the underlying dyn4j transform without touching interpolation state.
        o.getTransform().setTranslation(10f, 0f);
        o.updateInterpolatedPosition();

        // Render position has not advanced yet.
        assertEquals(0f, o.getTranslationX(), EPS);

        // After a full step it lands exactly on the target.
        o.update(ONE_STEP);
        assertEquals(10f, o.getTranslationX(), EPS);
    }

    @Test
    void updateLerpsHalfwayThenReachesTarget() {
        final InterpolatedPhysicsObject o = body();
        o.setTranslation(0f, 0f);

        o.getTransform().setTranslation(10f, 0f);
        o.updateInterpolatedPosition();

        o.update(ONE_STEP / 2f);
        assertEquals(5f, o.getTranslationX(), EPS);

        o.update(ONE_STEP / 2f);
        assertEquals(10f, o.getTranslationX(), EPS);
    }

    @Test
    void updateClampsAndDoesNotOvershoot() {
        final InterpolatedPhysicsObject o = body();
        o.setTranslation(0f, 0f);

        o.getTransform().setTranslation(10f, 0f);
        o.updateInterpolatedPosition();

        o.update(ONE_STEP * 5f);
        assertEquals(10f, o.getTranslationX(), EPS);
    }
}
