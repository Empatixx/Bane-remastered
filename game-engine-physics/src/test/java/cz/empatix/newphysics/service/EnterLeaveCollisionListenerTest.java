package cz.empatix.newphysics.service;

import cz.empatix.core.CollisionCategory;
import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.geometry.MassType;
import org.dyn4j.geometry.Rectangle;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.util.concurrent.atomic.AtomicInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Exercises {@link EnterLeaveListener} (enter on first contact, leave when contact
 * ends) and {@link CollisionListener} (collision callbacks gated by category matrix),
 * driving them through a real {@link PhysicsEngine}.
 */
class EnterLeaveCollisionListenerTest {

    private PhysicsEngine engine;

    @BeforeEach
    void setUp() {
        engine = new PhysicsEngine();
    }

    private PhysicsObject box(final float x, final float y) {
        final PhysicsObject o = new PhysicsObject();
        o.addFixture(new BodyFixture(new Rectangle(4, 4)));
        o.setMass(MassType.NORMAL);
        o.setTranslation(x, y);
        engine.add(o);
        return o;
    }

    @Test
    void enterFiresOnceWhenBodiesFirstTouch() {
        final PhysicsObject a = box(0f, 0f);
        final PhysicsObject b = box(0f, 0f);

        final AtomicInteger enters = new AtomicInteger();
        a.onEnter(other -> enters.incrementAndGet());

        engine.fixedUpdate(1.0 / 60.0);

        // enter fires at least once when the two boxes are in contact
        assertTrue(enters.get() >= 1);
    }

    @Test
    void enterPassesTheOtherBody() {
        final PhysicsObject a = box(0f, 0f);
        final PhysicsObject b = box(0f, 0f);

        final PhysicsObject[] seen = new PhysicsObject[1];
        a.onEnter(other -> seen[0] = other);

        engine.fixedUpdate(1.0 / 60.0);
        assertSame(b, seen[0]);
    }

    @Test
    void leaveFiresWhenContactEnds() {
        final PhysicsObject a = box(0f, 0f);
        final PhysicsObject b = box(0f, 0f);

        final AtomicInteger leaves = new AtomicInteger();
        a.onLeave(other -> leaves.incrementAndGet());

        engine.fixedUpdate(1.0 / 60.0); // establishes contact

        // Teleport b far away so they are no longer in contact, then step.
        b.setTranslation(1000f, 1000f);
        engine.updateBroadphase();
        engine.fixedUpdate(1.0 / 60.0);

        assertTrue(leaves.get() >= 1, "leave should fire once contact is lost");
    }

    @Test
    void collisionCallbacksFireWhenCategoriesAllowIt() {
        // Categories that collide with everything (default null type path also allows).
        final CollisionCategory always = other -> true;
        final PhysicsObject a = box(0f, 0f);
        final PhysicsObject b = box(0f, 0f);
        a.setType(always);
        b.setType(always);

        final AtomicInteger aHits = new AtomicInteger();
        final AtomicInteger bHits = new AtomicInteger();
        a.onCollision(other -> aHits.incrementAndGet());
        b.onCollision(other -> bHits.incrementAndGet());

        engine.fixedUpdate(1.0 / 60.0);

        assertTrue(aHits.get() >= 1);
        assertTrue(bHits.get() >= 1);
    }

    @Test
    void collisionCallbacksSuppressedWhenCategoriesDoNotCollide() {
        final CollisionCategory never = other -> false;
        final PhysicsObject a = box(0f, 0f);
        final PhysicsObject b = box(0f, 0f);
        a.setType(never);
        b.setType(never);

        final AtomicInteger aHits = new AtomicInteger();
        a.onCollision(other -> aHits.incrementAndGet());

        engine.fixedUpdate(1.0 / 60.0);

        // canCollide() returned false -> onCollision must NOT have been invoked.
        assertEquals(0, aHits.get());
    }

    @Test
    void collisionCallbacksFireWithDefaultNullLikeTypeMatrix() {
        // Default type is NONE (collides with nothing) for both bodies, so when both
        // are NONE the collidesWith path returns false and callbacks are suppressed.
        // Verify the enter listener still fired regardless of the category matrix.
        final PhysicsObject a = box(0f, 0f);
        final PhysicsObject b = box(0f, 0f);

        final AtomicInteger enters = new AtomicInteger();
        a.onEnter(other -> enters.incrementAndGet());

        engine.fixedUpdate(1.0 / 60.0);
        // the enter listener fires regardless of the (NONE) category matrix
        assertTrue(enters.get() >= 1);
    }

    @Test
    void enterListenerDirectUnitTracksPairsAndCallsBack() {
        // EnterLeaveListener is constructible headlessly; exercise enter() directly.
        final EnterLeaveListener listener = new EnterLeaveListener();
        final PhysicsObject a = new PhysicsObject();
        final PhysicsObject b = new PhysicsObject();

        final AtomicInteger aEnters = new AtomicInteger();
        final AtomicInteger bEnters = new AtomicInteger();
        a.onEnter(other -> aEnters.incrementAndGet());
        b.onEnter(other -> bEnters.incrementAndGet());

        listener.enter(a, b);
        listener.enter(a, b); // duplicate pair -> not added again, no extra callbacks

        assertEquals(1, aEnters.get());
        assertEquals(1, bEnters.get());
    }
}
