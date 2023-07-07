package cz.empatix.physics.events.consumers;

import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.events.domain.CollisionEvent;
import cz.empatix.physics.events.domain.Event;
import cz.empatix.physics.events.domain.EventType;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;


public class CollisionConsumer implements EventConsumer {
    private final static Logger LOGGER = LoggerFactory.getLogger(CollisionConsumer.class);

    @Override
    public void consume(final Event event) {
        final CollisionEvent collisionEvent = (CollisionEvent) event;

        final PhysicsBody bodyA = collisionEvent.bodyA();
        final PhysicsBody bodyB = collisionEvent.bodyB();
        final Vector2f normal = collisionEvent.normal();
        final float depth = collisionEvent.depth();

        final Vector2f translateA = new Vector2f(normal).mul(depth / 2);
        final Vector2f translateB = new Vector2f(normal).mul(depth / 2);

        if (!bodyA.isImmmovable()) {
            bodyA.translate(translateA);
        }
        if (!bodyB.isImmmovable()) {
            bodyB.translate(translateB);
        }

        final Vector2f relativeVelocity = bodyB.getVelocity().sub(bodyA.getVelocity(), new Vector2f());
        if (relativeVelocity.dot(normal) > 0) {
            return;
        }
        final float e = Math.min(bodyA.getRestitution(), bodyB.getRestitution());
        final float j = (-(1f + e) * relativeVelocity.dot(normal)) / (bodyA.getInversedMass() + bodyB.getInversedMass());

        final Vector2f velocityA = bodyA.getVelocity();
        final Vector2f velocityB = bodyB.getVelocity();

        final Vector2f impulse = new Vector2f(normal).mul(j);

        velocityA.sub(impulse.mul(bodyA.getInversedMass(), new Vector2f()));
        velocityB.add(impulse.mul(bodyB.getInversedMass(), new Vector2f()));
    }

    @Override
    public EventType getType() {
        return EventType.COLLISION;
    }

}
