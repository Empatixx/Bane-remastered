package cz.empatix.physics.service;

import cz.empatix.physics.domain.CollisionResult;
import cz.empatix.physics.domain.PhysicsBody;
import org.joml.Vector2f;

public class CollisionHandler {
    public void handle(final PhysicsBody bodyA, final PhysicsBody bodyB, final CollisionResult collisionResult){
        final Vector2f relativeVelocity = bodyB.getVelocity().sub(bodyA.getVelocity(), new Vector2f());
        if(relativeVelocity.dot(collisionResult.normal()) > 0){
            return;
        }
        final float e = Math.min(bodyA.getRestitution(), bodyB.getRestitution());
        final float j = (-(1f + e) * relativeVelocity.dot(collisionResult.normal())) / (bodyA.getInversedMass() + bodyB.getInversedMass());

        final Vector2f velocityA = bodyA.getVelocity();
        final Vector2f velocityB = bodyB.getVelocity();

        final Vector2f impulse = new Vector2f(collisionResult.normal()).mul(j);

        velocityA.sub(impulse.mul(bodyA.getInversedMass(), new Vector2f()));
        velocityB.add(impulse.mul(bodyB.getInversedMass(), new Vector2f()));

    }
}
