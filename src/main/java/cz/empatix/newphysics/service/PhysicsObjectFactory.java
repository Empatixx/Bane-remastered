package cz.empatix.newphysics.service;


import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.collision.Fixture;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.geometry.Convex;
import org.dyn4j.geometry.Rectangle;
import org.dyn4j.geometry.Shape;

public class PhysicsObjectFactory {
    private final static PhysicsService SERVICE = PhysicsService.getInstance();
    public static PhysicsObject create(final int width, final int height) {
        final PhysicsObject physicsObject = new PhysicsObject();
        final Rectangle rectangle = new Rectangle(width, height);
        final BodyFixture fixture = new BodyFixture(rectangle);
        fixture.setFriction(0);
        fixture.setRestitution(0);
        fixture.setDensity(1);

        physicsObject.addFixture(fixture);

        SERVICE.add(physicsObject);
        return physicsObject;
    }

}
