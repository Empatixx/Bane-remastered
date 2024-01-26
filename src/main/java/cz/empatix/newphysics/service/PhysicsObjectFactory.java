package cz.empatix.newphysics.service;


import cz.empatix.newphysics.attributes.InterpolatedPhysicsObject;
import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.collision.Fixture;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.geometry.*;

public class PhysicsObjectFactory {
    private final static PhysicsService SERVICE = PhysicsService.getInstance();
    public static PhysicsObject create(Rectangle rect){
        final PhysicsObject physicsObject = new PhysicsObject();
        final BodyFixture fixture = new BodyFixture(rect);
        fixture.setFriction(0);
        fixture.setRestitution(0);
        fixture.setDensity(1);
        physicsObject.addFixture(fixture);

        physicsObject.setAtRest(false);
        physicsObject.setAtRestDetectionEnabled(false);

        SERVICE.add(physicsObject);
        return physicsObject;
    }
    public static PhysicsObject create(final int width, final int height) {
        final PhysicsObject physicsObject = new PhysicsObject();
        final Rectangle rectangle = new Rectangle(width, height);
        rectangle.getVertices()[0].set(0,0);
        rectangle.getVertices()[1].set(width,0);
        rectangle.getVertices()[2].set(width,height);
        rectangle.getVertices()[3].set(0,height);

        final BodyFixture fixture = new BodyFixture(rectangle);
        fixture.setFriction(0);
        fixture.setRestitution(0);
        fixture.setDensity(1);
        physicsObject.addFixture(fixture);

        physicsObject.setAtRest(false);
        physicsObject.setAtRestDetectionEnabled(false);

        SERVICE.add(physicsObject);
        return physicsObject;
    }
    public static InterpolatedPhysicsObject createInterpolated(final int width, final int height) {
        final InterpolatedPhysicsObject physicsObject = new InterpolatedPhysicsObject();

        final Rectangle rectangle = new Rectangle(width, height);
        rectangle.getVertices()[0].set(0,0);
        rectangle.getVertices()[1].set(width,0);
        rectangle.getVertices()[2].set(width,height);
        rectangle.getVertices()[3].set(0,height);

        final BodyFixture fixture = new BodyFixture(rectangle);

        fixture.setFriction(0);
        fixture.setRestitution(0);
        fixture.setDensity(1);

        physicsObject.addFixture(fixture);

        physicsObject.setAtRest(false);
        physicsObject.setAtRestDetectionEnabled(false);

        physicsObject.setMass(MassType.NORMAL);
        physicsObject.setMass(new Mass(physicsObject.getMass().getCenter(),physicsObject.getMass().getMass(), 0)); // prevent rotations
        physicsObject.setLinearDamping(3);

        SERVICE.add(physicsObject);
        return physicsObject;
    }
}
