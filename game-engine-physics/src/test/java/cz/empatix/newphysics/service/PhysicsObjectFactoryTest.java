package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.InterpolatedPhysicsObject;
import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.geometry.MassType;
import org.dyn4j.geometry.Rectangle;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * {@link PhysicsObjectFactory} calls {@code PhysicsService.getInstance().add(...)},
 * so a headless {@link PhysicsService} must exist first (its constructor registers
 * itself as the static instance and only builds a dyn4j world — no natives).
 */
class PhysicsObjectFactoryTest {

    private PhysicsService service;

    @BeforeEach
    void setUp() {
        service = new PhysicsService();
    }

    @Test
    void createFromRectangleProducesBodyWithZeroFrictionFixture() {
        final Rectangle rect = new Rectangle(3, 5);
        final PhysicsObject o = PhysicsObjectFactory.create(rect);

        assertNotNull(o);
        assertEquals(1, o.getFixtureCount());
        final BodyFixture f = o.getFixture(0);
        assertEquals(0.0, f.getFriction(), 1e-9);
        assertEquals(0.0, f.getRestitution(), 1e-9);
        assertEquals(1.0, f.getDensity(), 1e-9);
        assertEquals(rect, f.getShape());

        // It was added to the world: a second box at the same spot collides with it.
        o.setTranslation(0f, 0f);
        assertTrue(service.isColliding(secondBoxAt(0, 0)));
    }

    @Test
    void createFromDimensionsProducesCenteredRectangleFixture() {
        final PhysicsObject o = PhysicsObjectFactory.create(4, 6);

        assertNotNull(o);
        assertEquals(1, o.getFixtureCount());
        final BodyFixture f = o.getFixture(0);
        assertInstanceOf(Rectangle.class, f.getShape());
        final Rectangle shape = (Rectangle) f.getShape();
        assertEquals(4.0, shape.getWidth(), 1e-9);
        assertEquals(6.0, shape.getHeight(), 1e-9);
        // Rectangle was translated by (w/2, h/2).
        assertEquals(2.0, shape.getCenter().x, 1e-9);
        assertEquals(3.0, shape.getCenter().y, 1e-9);
    }

    @Test
    void createInterpolatedProducesInterpolatedBodyWithDampingAndMass() {
        final InterpolatedPhysicsObject o = PhysicsObjectFactory.createInterpolated(4, 4);

        assertNotNull(o);
        assertInstanceOf(InterpolatedPhysicsObject.class, o);
        assertEquals(1, o.getFixtureCount());
        assertEquals(5.0, o.getLinearDamping(), 1e-9);
        assertEquals(MassType.FIXED_ANGULAR_VELOCITY, o.getMass().getType());
    }

    @Test
    void createdBodiesAreRegisteredInTheWorld() {
        // Two stacked bodies must report a collision through the shared service/world.
        final PhysicsObject a = PhysicsObjectFactory.create(6, 6);
        final PhysicsObject b = PhysicsObjectFactory.create(6, 6);
        a.setTranslation(0f, 0f);
        b.setTranslation(0f, 0f);
        assertTrue(service.isColliding(a));
    }

    private PhysicsObject secondBoxAt(final float x, final float y) {
        final PhysicsObject o = new PhysicsObject();
        o.addFixture(new BodyFixture(new Rectangle(2, 2)));
        o.setMass(MassType.NORMAL);
        o.setTranslation(x, y);
        service.add(o);
        return o;
    }
}
