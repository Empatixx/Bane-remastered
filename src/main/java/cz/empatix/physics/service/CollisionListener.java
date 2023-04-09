package cz.empatix.physics.service;

import cz.empatix.physics.domain.PhysicsBody;
import org.joml.Vector2f;

public class CollisionListener {
    private final PhysicsBody body;
    public CollisionListener(final PhysicsBody body) {
        this.body = body;
    }
    public void onCollision(final PhysicsBody other) {
        // Calculate relative velocity
        final Vector2f relativeVelocity = new Vector2f(body.getVelocity()).sub(other.getVelocity());

        // Calculate relative normal vector (from this body to other body)
        final Vector2f normal = new Vector2f(other.getPhysicsTranslation())
                .sub(body.getPhysicsTranslation())
                .normalize();

        // Calculate relative velocity along normal
        float velAlongNormal = relativeVelocity.dot(normal);

        // Check if bodies are moving away from each other
        if(velAlongNormal > 0) {
            return;
        }
        // Calculate restitution impulse
        float e = Math.min(body.getRestitution(), other.getRestitution());
        float jRestitution = -(1 + e) * velAlongNormal / (body.getMass() + other.getMass());

        // Apply impulses
        Vector2f impulse = new Vector2f(normal).mul(jRestitution);
        body.getVelocity().sub(impulse.mul(1.0f / this.body.getMass()));
        other.getVelocity().add(impulse.mul(1.0f / other.getMass()));
    }
}
