package cz.empatix.newphysics.attributes;

import org.joml.Vector2f;

public class PhysicsObject {
    private Collisionable shape;
    private Vector2f position;
    private Vector2f velocity;
    public PhysicsObject(final Collisionable shape,
                         final Vector2f position,
                         final Vector2f velocity) {
        this.shape = shape;
        this.position = position;
        this.velocity = velocity;
    }

    public void setPosition(Vector2f position) {
        this.position = position;
    }
    public void setVelocity(Vector2f velocity) {
        this.velocity = velocity;
    }

    public void update(double dt) {
        position.add(velocity.x() * (float)dt, velocity.y() * (float)dt);
    }

    public Collisionable getShape() {
        return shape;
    }
    public void onCollision()
}
