package cz.empatix.newphysics.attributes;

import cz.empatix.core.CollisionCategory;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.geometry.MassType;
import org.dyn4j.geometry.Rectangle;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;

import java.util.concurrent.atomic.AtomicReference;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

class PhysicsObjectTest {

    private static final float EPS = 1e-4f;

    @Test
    void translationRoundTripsWithoutOffset() {
        final PhysicsObject o = new PhysicsObject();
        o.setTranslation(5f, 7f);

        final Vector2f t = o.getPhysicsTranslation();
        assertEquals(5f, t.x, EPS);
        assertEquals(7f, t.y, EPS);
        assertEquals(5f, o.getPhysicsTranslationX(), EPS);
        assertEquals(7f, o.getPhysicsTranslationY(), EPS);
    }

    @Test
    void setPhysicsTranslationDelegatesToSetTranslation() {
        final PhysicsObject o = new PhysicsObject();
        o.setPhysicsTranslation(3f, -2f);

        assertEquals(3f, o.getPhysicsTranslationX(), EPS);
        assertEquals(-2f, o.getPhysicsTranslationY(), EPS);
    }

    @Test
    void offsetShiftsTransformButIsRemovedFromReportedTranslation() {
        final PhysicsObject o = new PhysicsObject();
        o.setOffset(10, 20);
        o.setTranslation(1f, 2f);

        // The transform translation includes the offset...
        assertEquals(11f, o.getTransform().getTranslationX(), EPS);
        assertEquals(22f, o.getTransform().getTranslationY(), EPS);

        // ...but getPhysicsTranslation subtracts it back out.
        final Vector2f t = o.getPhysicsTranslation();
        assertEquals(1f, t.x, EPS);
        assertEquals(2f, t.y, EPS);
    }

    @Test
    void velocityGetterAndSetter() {
        final PhysicsObject o = new PhysicsObject();
        o.setVelocity(4f, -8f);

        final Vector2f v = o.getVelocity();
        assertEquals(4f, v.x, EPS);
        assertEquals(-8f, v.y, EPS);
    }

    @Test
    void applyForceDoesNotThrow() {
        final PhysicsObject o = new PhysicsObject();
        // body needs mass for forces to accumulate; just verify the call is safe.
        o.addFixture(new BodyFixture(new Rectangle(1, 1)));
        o.setMass(MassType.NORMAL);
        o.applyForce(1f, 1f);
    }

    @Test
    void rotationReflectsTransformAngle() {
        final PhysicsObject o = new PhysicsObject();
        assertEquals(0f, o.getRotation(), EPS);

        o.getTransform().setRotation(Math.PI / 2);
        assertEquals((float) (Math.PI / 2), o.getRotation(), EPS);
    }

    @Test
    void typeDefaultsToNoneCollidesWithNothing() {
        final PhysicsObject o = new PhysicsObject();
        final CollisionCategory type = o.getType();
        assertNotNull(type);
        // The default NONE category collides with nothing.
        assertFalse(type.collidesWith(other -> true));
    }

    @Test
    void typeCanBeSetAndRetrieved() {
        final PhysicsObject o = new PhysicsObject();
        final CollisionCategory cat = other -> true;
        o.setType(cat);
        assertSame(cat, o.getType());
    }

    @Test
    void destroyFlipsRemovableFlag() {
        final PhysicsObject o = new PhysicsObject();
        assertFalse(o.canRemove());
        o.destroy();
        assertTrue(o.canRemove());
    }

    @Test
    void collisionCallbackIsInvokedWithGivenObject() {
        final PhysicsObject o = new PhysicsObject();
        final PhysicsObject other = new PhysicsObject();
        final AtomicReference<PhysicsObject> seen = new AtomicReference<>();
        o.onCollision(seen::set);

        o.onCollision(other);
        assertSame(other, seen.get());
    }

    @Test
    void enterCallbackIsInvokedWithGivenObject() {
        final PhysicsObject o = new PhysicsObject();
        final PhysicsObject other = new PhysicsObject();
        final AtomicReference<PhysicsObject> seen = new AtomicReference<>();
        o.onEnter(seen::set);

        o.onEnter(other);
        assertSame(other, seen.get());
    }

    @Test
    void leaveCallbackIsInvokedWithGivenObject() {
        final PhysicsObject o = new PhysicsObject();
        final PhysicsObject other = new PhysicsObject();
        final AtomicReference<PhysicsObject> seen = new AtomicReference<>();
        o.onLeave(seen::set);

        o.onLeave(other);
        assertSame(other, seen.get());
    }

    @Test
    void defaultCallbacksAreNoOpsAndDoNotThrow() {
        final PhysicsObject o = new PhysicsObject();
        // No callbacks registered: the NOOP default must just swallow these.
        o.onCollision(new PhysicsObject());
        o.onEnter(new PhysicsObject());
        o.onLeave(new PhysicsObject());
    }

    @Test
    void baseUpdateIsNoOp() {
        final PhysicsObject o = new PhysicsObject();
        o.update(0.5f); // no state, must not throw
    }
}
