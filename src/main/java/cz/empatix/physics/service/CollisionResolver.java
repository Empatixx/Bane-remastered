package cz.empatix.physics.service;

import cz.empatix.physics.domain.CollisionResult;
import cz.empatix.physics.domain.geometry.Circle;
import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.domain.geometry.Rectangle;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;


public class CollisionResolver {
    private final static Logger LOGGER = LoggerFactory.getLogger(CollisionResolver.class.getName());

    public CollisionResult resolve(final PhysicsBody bodyA, final PhysicsBody bodyB) {
        switch (bodyA.getGeometry().getType()) {
            case RECT -> {
                switch (bodyB.getGeometry().getType()) {
                    case RECT -> {
                        return resolveRectangleXRectangle(bodyA, bodyB);
                    }
                    case CIRCLE -> {
                        return resolveCircleXRectangle();
                    }
                }
            }
            case CIRCLE -> {
                switch (bodyB.getGeometry().getType()) {
                    case RECT -> {
                        return resolveCircleXRectangle();
                    }
                    case CIRCLE -> {
                        return resolveCircleXCircle(bodyA, bodyB);
                    }
                }
            }
            default -> throw new IllegalStateException("Unexpected value: " + bodyA.getGeometry().getType());
        }
        LOGGER.error("Could not resolve type {} and {}", bodyA.getGeometry().getType(), bodyB.getGeometry().getType());
        return null;
    }

    public CollisionResult resolveCircleXCircle(final PhysicsBody a, final PhysicsBody b) {
        final Circle circleA = (Circle) a.getGeometry();
        final Circle circleB = (Circle) b.getGeometry();

        final Vector2f positionA = a.getTranslation();
        final Vector2f positionB = b.getTranslation();

        final Vector2f centerA = new Vector2f(positionA).add(circleA.radius(), circleA.radius());
        final Vector2f centerB = new Vector2f(positionB).add(circleB.radius(), circleB.radius());

        final float distance = centerA.distance(centerB);
        float radiusSum = circleA.radius() + circleB.radius();

        if (distance >= radiusSum) {
            return null;
        }

        final Vector2f normal = new Vector2f(centerB).sub(centerA).normalize();
        final float depth = radiusSum - distance;
        return new CollisionResult(normal, depth);
    }

    public CollisionResult resolveCircleXRectangle() {
        //throw new UnsupportedOperationException("Not implemented yet");
        return null;
    }

    public CollisionResult resolveRectangleXRectangle(final PhysicsBody a, final PhysicsBody b) {
        final Rectangle rectangleA = (Rectangle) a.getGeometry();
        final Rectangle rectangleB = (Rectangle) b.getGeometry();

        final Vector2f positionA = a.getTranslation();
        final Vector2f positionB = b.getTranslation();

        if (positionA.x < positionB.x + rectangleB.width() &&
                positionA.x + rectangleA.width() > positionB.x &&
                positionA.y < positionB.y + rectangleB.height() &&
                positionA.y + rectangleA.height() > positionB.y) {

            float overlapX = Math.min(positionA.x + rectangleA.width() - positionB.x,
                    positionB.x + rectangleB.width() - positionA.x);
            float overlapY = Math.min(positionA.y + rectangleA.height() - positionB.y,
                    positionB.y + rectangleB.height() - positionA.y);

            if (overlapX < overlapY) {
                float sign = (positionA.x < positionB.x) ? 1.0f : -1.0f;
                return new CollisionResult(new Vector2f(sign, 0.0f), overlapX);
            } else {
                float sign = (positionA.y < positionB.y) ? 1.0f : -1.0f;
                return new CollisionResult(new Vector2f(0.0f, sign), overlapY);
            }
        }
        return null;
    }
}
