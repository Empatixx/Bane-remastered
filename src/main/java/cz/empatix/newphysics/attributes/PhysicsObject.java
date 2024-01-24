package cz.empatix.newphysics.attributes;

import cz.empatix.enemies.domain.bosses.BossImpl;
import org.joml.Vector2f;

public class PhysicsObject {
    private final static Runnable NOOP = () -> {};
    private Collisionable shape;
    private Vector2f position;
    private Vector2f velocity;
    private Vector2f force;
    private float linearDamping;
    private Runnable onCollision = NOOP;
    private Object owner;
    private boolean remove = false;
    public PhysicsObject(final Collisionable shape) {
        this.shape = shape;
        this.position = shape.getPosition();
        this.velocity = new Vector2f(0.0f, 0.0f);
        this.linearDamping = 0.0f;
        this.force = new Vector2f(0.0f, 0.0f);
    }
    public PhysicsObject(final Collisionable shape,
                         final Object owner) {
        this.shape = shape;
        this.position = shape.getPosition();
        this.velocity = new Vector2f(0.0f, 0.0f);
        this.linearDamping = 0.0f;
        this.force = new Vector2f(0.0f, 0.0f);
        this.owner = owner;
    }

    public void onCollision(final Runnable onCollision) {
        this.onCollision = onCollision;
    }

    public void setPosition(Vector2f position) {
        this.position = position;
    }
    public void setPosition(float x, float y) {
        this.position.set(x,y);
    }
    public void applyForce(Vector2f force) {
        this.force.add(force);
    }
    public void setVelocity(Vector2f velocity) {
        this.velocity = velocity;
    }

    public Object getOwner() {
        return owner;
    }

    public void update(double dt) {
        final float mass = getInversedMass();

        velocity.x += (float) (force.x * mass * dt);
        velocity.y += (float) (force.y * mass * dt);

        translate(velocity.x() * (float) dt, velocity.y() * (float) dt);

        force.zero();

        velocity.x *= linearDamping;
        velocity.y *= linearDamping;
    }

    private void translate(final float x, final float y) {
        position.add(x, y);
        shape.setPosition(position.x(), position.y());
    }

    private float getInversedMass() {
        return 1.0f / shape.getMass();
    }

    public Collisionable getShape() {
        return shape;
    }
    public void onCollision() {
        onCollision.run();
    }
    public void destroy(){
        remove = true;
    }

    public boolean isRemove() {
        return remove;
    }
    public Vector2f getPosition() {
        return position;
    }

    public void setOwner(Object owner) {
        this.owner = owner;
    }
}
