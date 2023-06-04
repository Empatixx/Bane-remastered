package cz.empatix.physics.service;

import cz.empatix.physics.domain.PhysicsBody;
import org.joml.Vector2f;

public class BodyUpdater {
    public void update(final PhysicsBody body, final float delta){
        final Vector2f velocity = body.getVelocity();
        final Vector2f force = body.getForce();
        final float mass = body.getMass();
        final float linearDamping = body.getLinearDamping();

        velocity.x += (force.x / mass) * delta;
        velocity.y += (force.y / mass) * delta;

        body.translate(new Vector2f(velocity).mul(delta));

        force.zero();

        velocity.x *= linearDamping;
        velocity.y *= linearDamping;

    }
}
