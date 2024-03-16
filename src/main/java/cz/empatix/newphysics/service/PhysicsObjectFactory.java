package cz.empatix.newphysics.service;


import cz.empatix.newphysics.attributes.InterpolatedPhysicsObject;
import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.collision.Fixture;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.geometry.*;
import org.joml.Vector2f;

public class PhysicsObjectFactory {
    private final static PhysicsService SERVICE = PhysicsService.getInstance();
    public static PhysicsObject create(Rectangle rect){
        final PhysicsObject physicsObject = new PhysicsObject();
        final BodyFixture fixture = new BodyFixture(rect);
        fixture.setFriction(0);
        fixture.setRestitution(0);
        fixture.setDensity(1);
        physicsObject.addFixture(fixture);

        //physicsObject.setAtRest(false);
        //physicsObject.setAtRestDetectionEnabled(false);

        SERVICE.add(physicsObject);
        return physicsObject;
    }
    public static PhysicsObject create(final int width, final int height) {
        final PhysicsObject physicsObject = new PhysicsObject();
        final Rectangle rectangle = new Rectangle(width, height);
        rectangle.translate(width/2f, height/2f);

        final BodyFixture fixture = new BodyFixture(rectangle);
        fixture.setFriction(0);
        fixture.setRestitution(0);
        fixture.setDensity(1);
        physicsObject.addFixture(fixture);

        //physicsObject.setAtRest(false);
        //physicsObject.setAtRestDetectionEnabled(false);

        SERVICE.add(physicsObject);
        return physicsObject;
    }
    public static InterpolatedPhysicsObject createInterpolated(final int width, final int height) {
        final InterpolatedPhysicsObject physicsObject = new InterpolatedPhysicsObject();

        final Rectangle rectangle = new Rectangle(width, height);
        rectangle.translate(width/2f, height/2f);

        final BodyFixture fixture = new BodyFixture(rectangle);

        fixture.setFriction(0);
        fixture.setRestitution(0);
        fixture.setDensity(1);

        physicsObject.addFixture(fixture);

        physicsObject.setAtRest(false);
        physicsObject.setAtRestDetectionEnabled(false);

        physicsObject.setMass(MassType.FIXED_ANGULAR_VELOCITY);
        physicsObject.setLinearDamping(3);


        SERVICE.add(physicsObject);
        return physicsObject;
    }
}
