package cz.empatix.physics.service;

import cz.empatix.physics.domain.PhysicsBody;
import org.joml.Vector2f;

public class CollisionListener {
    public void onCollision(final PhysicsBody body) {
        final Vector2f velocity = body.getVelocity();

        final Vector2f position = body.getTranslation();
        final Vector2f other = body.getTranslation();

        Vector2f delta = other.sub(position);
        Vector2f overlap = new Vector2f(Math.abs(delta.x) - (size.x + other.size.x) * 0.5f, Math.abs(delta.y) - (size.y + other.size.y) * 0.5f);

        if (overlap.x < overlap.y) {
            float sign = delta.x > 0 ? 1 : -1;
            position.x -= overlap.x * sign;
            other.position.x += overlap.x * sign;

            // Update velocities based on the mass of the objects
            float impulse = (2 * other.mass) / (mass + other.mass) * (other.velocity.x - velocity.x);
            velocity.x += impulse * mass;
            other.velocity.x -= impulse * other.mass;
        } else {
            float sign = delta.y > 0 ? 1 : -1;
            position.y -= overlap.y * sign;
            other.position.y += overlap.y * sign;

            // Update velocities based on the mass of the objects
            float impulse = (2 * other.mass) / (mass + other.mass) * (other.velocity.y - velocity.y);
            velocity.y += impulse * mass;
            other.velocity.y -= impulse * other.mass;
        }
    }
}
