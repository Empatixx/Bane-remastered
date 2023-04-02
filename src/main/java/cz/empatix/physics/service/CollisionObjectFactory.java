package cz.empatix.physics.service;

import cz.empatix.physics.domain.CollisionObject;
import cz.empatix.render.domain.InterpolatedPosition;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.geometry.MassType;
import org.dyn4j.geometry.Rectangle;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;


public class CollisionObjectFactory {
    private static final Logger LOGGER = LoggerFactory.getLogger(CollisionObjectFactory.class);

    public static CollisionObject createDynamicObject(final int width,
                                                      final int height,
                                                      final InterpolatedPosition position) {
        final CollisionObject collisionObject = new CollisionObject(position);

        final Rectangle rectangle = new Rectangle(width, height);
        //rectangle.translate(width / 2f, height / 2f);

        final BodyFixture fixture = new BodyFixture(rectangle);
        fixture.setDensity(0.01f);
        fixture.setRestitution(0);

        collisionObject.addFixture(fixture);
        collisionObject.setPosition(position.getX(), position.getY());
        collisionObject.setMass(MassType.NORMAL);
        collisionObject.setAtRestDetectionEnabled(false);

        collisionObject.setAngularDamping(Double.MAX_VALUE);
        collisionObject.setLinearDamping(1);

        LOGGER.trace("Created dynamic object at position: {}", position);
        PhysicsEngine.addBody(collisionObject);

        return collisionObject;
    }

    public static CollisionObject createStaticObject(final int width,
                                                     final int height,
                                                     final Vector2f position) {
        final CollisionObject collisionObject = new CollisionObject();

        final Rectangle rectangle = new Rectangle(width, height);
        //rectangle.translate(width / 2f, height / 2f);

        final BodyFixture fixture = new BodyFixture(rectangle);
        fixture.setRestitution(0);

        collisionObject.addFixture(fixture);
        collisionObject.setPosition(position.x(), position.y());
        collisionObject.setMass(MassType.INFINITE);
        collisionObject.setAtRestDetectionEnabled(false);
        collisionObject.setAngularDamping(1000);
        collisionObject.setLinearDamping(1000);

        LOGGER.trace("Created static object at position: {}", position);
        PhysicsEngine.addBody(collisionObject);

        return collisionObject;
    }
}
