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
        final BodyFixture fixture = new BodyFixture(new Rectangle(width, height));

        collisionObject.addFixture(fixture);
        collisionObject.setPosition(position.getX(), position.getY());
        collisionObject.setMass(MassType.NORMAL);
        collisionObject.setLinearDamping(0);
        collisionObject.setAngularDamping(0);

        LOGGER.trace("Created dynamic object at position: {}", position);
        PhysicsEngine.addBody(collisionObject);

        return collisionObject;
    }

    public static CollisionObject createStaticObject(final int width,
                                                     final int height,
                                                     final Vector2f position) {
        final CollisionObject collisionObject = new CollisionObject();
        final BodyFixture fixture = new BodyFixture(new Rectangle(width, height));

        collisionObject.addFixture(fixture);
        collisionObject.setPosition(position.x(), position.y());
        collisionObject.setMass(MassType.INFINITE);
        collisionObject.setLinearDamping(0);
        collisionObject.setAngularDamping(0);

        LOGGER.trace("Created static object at position: {}", position);
        PhysicsEngine.addBody(collisionObject);

        return collisionObject;
    }
}
