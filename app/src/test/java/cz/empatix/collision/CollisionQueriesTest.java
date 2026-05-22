package cz.empatix.collision;

import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.PhysicsService;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.geometry.MassType;
import org.dyn4j.geometry.Rectangle;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.util.EnumSet;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

class CollisionQueriesTest {

    private PhysicsService physics;

    @BeforeEach
    void setUp() {
        // Constructing a PhysicsService builds a headless dyn4j world and registers
        // itself as the static instance consulted by CollisionQueries.getInstance().
        physics = new PhysicsService();
    }

    /**
     * Adds a body of the given category occupying the axis-aligned box
     * [x, x + size] x [y, y + size] to the physics world.
     */
    private PhysicsObject addBody(final CollisionType type, final int x, final int y, final int size) {
        final PhysicsObject body = new PhysicsObject();
        final Rectangle rectangle = new Rectangle(size, size);
        // Match the engine's convention: the body's translation is the box corner.
        rectangle.translate(size / 2f, size / 2f);
        body.addFixture(new BodyFixture(rectangle));
        body.setMass(MassType.NORMAL);
        body.setType(type);
        body.setUserData(type);
        body.setTranslation(x, y);
        physics.add(body);
        physics.updateBroadphase();
        return body;
    }

    @Test
    void returnsEmptyWhenWorldHasNoBodies() {
        final List<PhysicsObject> result = CollisionQueries.queryType(
                new Vector2f(0, 0), new Vector2i(10, 10), EnumSet.of(CollisionType.ENEMY));

        assertTrue(result.isEmpty());
    }

    @Test
    void returnsBodyWhoseTypeIsInRequestedSet() {
        final PhysicsObject enemy = addBody(CollisionType.ENEMY, 0, 0, 20);

        final List<PhysicsObject> result = CollisionQueries.queryType(
                new Vector2f(0, 0), new Vector2i(20, 20), EnumSet.of(CollisionType.ENEMY));

        assertEquals(1, result.size());
        assertTrue(result.contains(enemy));
    }

    @Test
    void filtersOutBodiesWhoseTypeIsNotRequested() {
        final PhysicsObject enemy = addBody(CollisionType.ENEMY, 0, 0, 20);
        final PhysicsObject item = addBody(CollisionType.ITEM, 0, 0, 20);

        final List<PhysicsObject> result = CollisionQueries.queryType(
                new Vector2f(0, 0), new Vector2i(20, 20), EnumSet.of(CollisionType.ENEMY));

        assertTrue(result.contains(enemy));
        assertFalse(result.contains(item));
        assertEquals(1, result.size());
    }

    @Test
    void returnsMultipleBodiesWhenSeveralTypesRequested() {
        final PhysicsObject enemy = addBody(CollisionType.ENEMY, 0, 0, 20);
        final PhysicsObject item = addBody(CollisionType.ITEM, 0, 0, 20);

        final List<PhysicsObject> result = CollisionQueries.queryType(
                new Vector2f(0, 0), new Vector2i(20, 20),
                EnumSet.of(CollisionType.ENEMY, CollisionType.ITEM));

        assertEquals(2, result.size());
        assertTrue(result.contains(enemy));
        assertTrue(result.contains(item));
    }

    @Test
    void returnsEmptyWhenQueryRegionMissesTheBody() {
        // Body occupies [0,20]x[0,20]; query a far-away region.
        addBody(CollisionType.ENEMY, 0, 0, 20);

        final List<PhysicsObject> result = CollisionQueries.queryType(
                new Vector2f(1000, 1000), new Vector2i(20, 20), EnumSet.of(CollisionType.ENEMY));

        assertTrue(result.isEmpty());
    }
}
