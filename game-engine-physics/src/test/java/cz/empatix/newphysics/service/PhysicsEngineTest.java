package cz.empatix.newphysics.service;

import cz.empatix.core.CollisionCategory;
import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.geometry.MassType;
import org.dyn4j.geometry.Rectangle;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

class PhysicsEngineTest {

    /** A trivial category that only collides with itself (by identity). */
    private static final class Category implements CollisionCategory {
        @Override
        public boolean collidesWith(final CollisionCategory other) {
            return other == this;
        }
    }

    private PhysicsEngine engine;

    @BeforeEach
    void setUp() {
        engine = new PhysicsEngine();
    }

    private static PhysicsObject box(final float x, final float y, final int w, final int h) {
        final PhysicsObject o = new PhysicsObject();
        o.addFixture(new BodyFixture(new Rectangle(w, h)));
        o.setMass(MassType.NORMAL);
        o.setTranslation(x, y);
        return o;
    }

    @Test
    void fixedUpdateStepsWithoutBodies() {
        engine.fixedUpdate(1.0 / 60.0); // empty world must step cleanly
    }

    @Test
    void fixedUpdateMovesABodyAccordingToVelocity() {
        final PhysicsObject o = box(0f, 0f, 2, 2);
        engine.add(o);
        o.setVelocity(60f, 0f); // 60 u/s; one 1/60 step ~= 1 unit

        engine.fixedUpdate(1.0 / 60.0);

        assertTrue(o.getPhysicsTranslationX() > 0.5f,
                "body should have moved along +x, was " + o.getPhysicsTranslationX());
    }

    @Test
    void fixedUpdateRemovesBodiesFlaggedForRemoval() {
        final PhysicsObject keep = box(0f, 0f, 2, 2);
        final PhysicsObject drop = box(100f, 100f, 2, 2);
        engine.add(keep);
        engine.add(drop);
        drop.destroy();

        engine.fixedUpdate(1.0 / 60.0);

        assertFalse(engine.isColliding(keep)); // drop is gone, far away anyway
        // removal: querying the dropped body's location finds nothing
        assertFalse(engine.queryExists(new Vector2f(99f, 99f), new Vector2i(2, 2), Set.of()));
    }

    @Test
    void removeTakesBodyOutOfWorld() {
        final PhysicsObject a = box(0f, 0f, 4, 4);
        final PhysicsObject b = box(0f, 0f, 4, 4);
        engine.add(a);
        engine.add(b);
        assertTrue(engine.isColliding(a));

        engine.remove(b);
        assertFalse(engine.isColliding(a));
    }

    @Test
    void clearRemovesAllBodies() {
        engine.add(box(0f, 0f, 4, 4));
        engine.add(box(0f, 0f, 4, 4));
        final PhysicsObject probe = box(0f, 0f, 4, 4);

        engine.clear();
        engine.add(probe);
        assertFalse(engine.isColliding(probe)); // nothing else left to overlap
    }

    @Test
    void isCollidingTrueForOverlappingBodiesFalseOtherwise() {
        final PhysicsObject a = box(0f, 0f, 4, 4);
        final PhysicsObject b = box(1f, 1f, 4, 4);
        final PhysicsObject far = box(1000f, 1000f, 2, 2);
        engine.add(a);
        engine.add(b);
        engine.add(far);

        assertTrue(engine.isColliding(a));
        assertFalse(engine.isColliding(far));
    }

    @Test
    void queryReturnsBodiesOverlappingTheRegion() {
        final PhysicsObject inside = box(5f, 5f, 4, 4);
        engine.add(inside);

        final List<PhysicsObject> hits = engine.query(
                new Vector2f(0f, 0f), new Vector2i(20, 20), Set.of());
        assertTrue(hits.contains(inside));
    }

    @Test
    void queryIgnoresRemovableBodies() {
        final PhysicsObject inside = box(5f, 5f, 4, 4);
        inside.destroy();
        engine.add(inside);

        final List<PhysicsObject> hits = engine.query(
                new Vector2f(0f, 0f), new Vector2i(20, 20), Set.of());
        assertFalse(hits.contains(inside));
    }

    @Test
    void queryFiltersOutIgnoredCategories() {
        final Category cat = new Category();
        final PhysicsObject inside = box(5f, 5f, 4, 4);
        inside.setType(cat);
        engine.add(inside);

        final List<PhysicsObject> withFilter = engine.query(
                new Vector2f(0f, 0f), new Vector2i(20, 20), Set.of(cat));
        assertFalse(withFilter.contains(inside));

        final List<PhysicsObject> withoutFilter = engine.query(
                new Vector2f(0f, 0f), new Vector2i(20, 20), Set.of());
        assertTrue(withoutFilter.contains(inside));
    }

    @Test
    void queryExistsTrueWhenSomethingInRegionFalseWhenIgnored() {
        final Category cat = new Category();
        final PhysicsObject inside = box(5f, 5f, 4, 4);
        inside.setType(cat);
        engine.add(inside);

        assertTrue(engine.queryExists(new Vector2f(0f, 0f), new Vector2i(20, 20), Set.of()));
        assertFalse(engine.queryExists(new Vector2f(0f, 0f), new Vector2i(20, 20), Set.of(cat)));
    }

    @Test
    void queryExistsFalseWhenRegionEmpty() {
        engine.add(box(500f, 500f, 2, 2));
        assertFalse(engine.queryExists(new Vector2f(0f, 0f), new Vector2i(4, 4), Set.of()));
    }

    @Test
    void updateAdvancesInterpolationOfBodies() {
        // Bodies' update(dt) is called; PhysicsObject base update is a no-op,
        // so this just verifies the loop runs without error over multiple bodies.
        engine.add(box(0f, 0f, 2, 2));
        engine.add(box(10f, 10f, 2, 2));
        engine.update(1.0 / 60.0);
    }

    @Test
    void updateBroadphaseDoesNotThrow() {
        engine.add(box(0f, 0f, 2, 2));
        engine.fixedUpdate(1.0 / 60.0);
        engine.updateBroadphase();
    }

    @Test
    void debugRenderEmitsVerticesPerBody() {
        engine.add(box(0f, 0f, 2, 2));
        engine.add(box(10f, 0f, 2, 2));

        final int[] bodies = {0};
        final int[] vertices = {0};
        final boolean[] open = {false};
        engine.debugRender(new PhysicsDebugRenderer() {
            @Override
            public void beginBody() {
                bodies[0]++;
                open[0] = true;
            }

            @Override
            public void vertex(final double worldX, final double worldY) {
                vertices[0]++;
            }

            @Override
            public void endBody() {
                open[0] = false;
            }
        });

        assertEquals(2, bodies[0]);
        assertEquals(8, vertices[0]); // 4 vertices per rectangle body
        assertFalse(open[0]); // every begin was closed
    }
}
