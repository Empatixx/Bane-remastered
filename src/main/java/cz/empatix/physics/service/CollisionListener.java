package cz.empatix.physics.service;

import cz.empatix.physics.domain.PhysicsBody;
import org.joml.Vector2f;

public class CollisionListener {
    private final PhysicsBody body;

    public CollisionListener(final PhysicsBody body) {
        this.body = body;
    }

    public void onCollision(final PhysicsBody other) {
        // Calculate the direction from body to other
        final Vector2f direction = new Vector2f(other.getPhysicsTranslation()).sub(body.getPhysicsTranslation()).normalize();

        // Project the body's velocity onto the direction vector
        float projectedVelocity = body.getVelocity().dot(direction);

        // If the projected velocity is positive, set the body's velocity to zero
        if (projectedVelocity > 0) {
            body.setVelocity(0, 0);
        }
    }
}
