package cz.empatix.newphysics.service;

import cz.empatix.core.CollisionHandler;
import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.geometry.MassType;
import org.dyn4j.geometry.Rectangle;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Drives the dispatcher through a real {@link PhysicsEngine} step: two overlapping
 * bodies generate a narrow-phase collision which must fan out each body's user data
 * to every registered {@link CollisionHandler}.
 */
class CollisionHandlerDispatcherTest {

    private PhysicsEngine engine;

    @BeforeEach
    void setUp() {
        engine = new PhysicsEngine();
    }

    private PhysicsObject overlappingBox(final Object userData) {
        final PhysicsObject o = new PhysicsObject();
        o.addFixture(new BodyFixture(new Rectangle(4, 4)));
        o.setMass(MassType.NORMAL);
        o.setTranslation(0f, 0f); // all stacked on the origin -> they overlap
        o.setUserData(userData);
        engine.add(o);
        return o;
    }

    @Test
    void narrowphaseCollisionFansOutUserDataToHandlers() {
        final String udA = "A";
        final String udB = "B";
        overlappingBox(udA);
        overlappingBox(udB);

        final List<Object[]> pairs = new ArrayList<>();
        engine.addCollisionHandler((a, b) -> pairs.add(new Object[]{a, b}));

        engine.fixedUpdate(1.0 / 60.0);

        assertFalse(pairs.isEmpty(), "handler should have been invoked for the overlap");
        // Both user-data objects must appear in the reported pair.
        final Object[] pair = pairs.get(0);
        final List<Object> seen = List.of(pair[0], pair[1]);
        assertTrue(seen.contains(udA));
        assertTrue(seen.contains(udB));
    }

    @Test
    void multipleHandlersAllReceiveCollision() {
        overlappingBox("A");
        overlappingBox("B");

        final boolean[] first = {false};
        final boolean[] second = {false};
        engine.addCollisionHandler((a, b) -> first[0] = true);
        engine.addCollisionHandler((a, b) -> second[0] = true);

        engine.fixedUpdate(1.0 / 60.0);

        assertTrue(first[0]);
        assertTrue(second[0]);
    }

    @Test
    void noHandlersRegisteredIsANoOpDuringStep() {
        overlappingBox("A");
        overlappingBox("B");
        // No handler added; the empty-handlers branch must simply do nothing.
        engine.fixedUpdate(1.0 / 60.0);
    }

    @Test
    void dispatcherUnitHandlerListStartsEmpty() {
        // Direct construction is headless-safe and exercises add() in isolation.
        final CollisionHandlerDispatcher dispatcher = new CollisionHandlerDispatcher();
        final boolean[] called = {false};
        final CollisionHandler handler = (a, b) -> called[0] = true;
        dispatcher.add(handler);
        // The broadphase/manifold overloads always return true and never touch handlers.
        assertTrue(dispatcher.collision((org.dyn4j.world.BroadphaseCollisionData<PhysicsObject, BodyFixture>) null));
        assertFalse(called[0]);
    }
}
