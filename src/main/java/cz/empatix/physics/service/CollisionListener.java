package cz.empatix.physics.service;

import cz.empatix.physics.domain.PhysicsBody;
import org.joml.Vector2f;

public class CollisionListener {
    public void onCollision(final PhysicsBody body) {
        final Vector2f velocity = body.getVelocity();

        final Vector2f position = body.translation();
        final Vector2f other = body.translation();

        Vector2f delta = other.sub(position);
    }
}
