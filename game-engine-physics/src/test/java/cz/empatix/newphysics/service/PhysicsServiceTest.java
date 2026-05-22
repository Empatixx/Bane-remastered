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

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

class PhysicsServiceTest {

    private static final class Category implements CollisionCategory {
        @Override
        public boolean collidesWith(final CollisionCategory other) {
            return other == this;
        }
    }

    private PhysicsService service;

    @BeforeEach
    void setUp() {
        service = new PhysicsService();
    }

    private PhysicsObject box(final float x, final float y, final int w, final int h) {
        final PhysicsObject o = new PhysicsObject();
        o.addFixture(new BodyFixture(new Rectangle(w, h)));
        o.setMass(MassType.NORMAL);
        o.setTranslation(x, y);
        return o;
    }

    @Test
    void constructorRegistersItselfAsTheStaticInstance() {
        assertSame(service, PhysicsService.getInstance());
    }

    @Test
    void getInstanceThrowsWhenNeverConstructed() {
        // Reset the static instance via reflection to simulate the uninitialized state.
        try {
            final var field = PhysicsService.class.getDeclaredField("instance");
            field.setAccessible(true);
            final Object previous = field.get(null);
            field.set(null, null);
            try {
                assertThrows(IllegalStateException.class, PhysicsService::getInstance);
            } finally {
                field.set(null, previous); // restore so other tests are unaffected
            }
        } catch (ReflectiveOperationException e) {
            throw new RuntimeException(e);
        }
    }

    @Test
    void addAndIsCollidingDelegateToEngine() {
        final PhysicsObject a = box(0f, 0f, 4, 4);
        final PhysicsObject b = box(0f, 0f, 4, 4);
        service.add(a);
        service.add(b);
        assertTrue(service.isColliding(a));
    }

    @Test
    void removeDelegatesToEngine() {
        final PhysicsObject a = box(0f, 0f, 4, 4);
        final PhysicsObject b = box(0f, 0f, 4, 4);
        service.add(a);
        service.add(b);
        service.remove(b);
        assertFalse(service.isColliding(a));
    }

    @Test
    void clearRemovesEverything() {
        service.add(box(0f, 0f, 4, 4));
        service.add(box(0f, 0f, 4, 4));
        service.clear();

        final PhysicsObject probe = box(0f, 0f, 4, 4);
        service.add(probe);
        assertFalse(service.isColliding(probe));
    }

    @Test
    void queryAndQueryExistsDelegateToEngine() {
        final PhysicsObject o = box(5f, 5f, 4, 4);
        service.add(o);

        final List<PhysicsObject> hits = service.query(
                new Vector2f(0f, 0f), new Vector2i(20, 20), Set.of());
        assertTrue(hits.contains(o));
        assertTrue(service.queryExists(new Vector2f(0f, 0f), new Vector2i(20, 20), Set.of()));
    }

    @Test
    void queryHonoursIgnoredCategoriesThroughService() {
        final Category cat = new Category();
        final PhysicsObject o = box(5f, 5f, 4, 4);
        o.setType(cat);
        service.add(o);

        assertFalse(service.query(new Vector2f(0f, 0f), new Vector2i(20, 20), Set.of(cat)).contains(o));
        assertFalse(service.queryExists(new Vector2f(0f, 0f), new Vector2i(20, 20), Set.of(cat)));
    }

    @Test
    void fixedUpdateAndUpdateAndBroadphaseDelegateWithoutError() {
        final PhysicsObject o = box(0f, 0f, 2, 2);
        service.add(o);
        o.setVelocity(60f, 0f);

        service.fixedUpdate(1.0 / 60.0);
        service.update(1.0 / 60.0);
        service.updateBroadphase();

        assertTrue(o.getPhysicsTranslationX() > 0.5f);
    }

    @Test
    void addCollisionHandlerWiresHandlerIntoTheEngine() {
        final boolean[] fired = {false};
        service.addCollisionHandler((a, b) -> fired[0] = true);

        final PhysicsObject a = box(0f, 0f, 4, 4);
        final PhysicsObject b = box(0f, 0f, 4, 4);
        a.setUserData("a");
        b.setUserData("b");
        service.add(a);
        service.add(b);

        service.fixedUpdate(1.0 / 60.0);
        assertTrue(fired[0]);
    }

    @Test
    void debugRenderDelegatesToEngine() {
        service.add(box(0f, 0f, 2, 2));
        final int[] bodies = {0};
        service.debugRender(new PhysicsDebugRenderer() {
            @Override
            public void beginBody() {
                bodies[0]++;
            }

            @Override
            public void vertex(final double worldX, final double worldY) {
            }

            @Override
            public void endBody() {
            }
        });
        assertTrue(bodies[0] >= 1);
    }
}
