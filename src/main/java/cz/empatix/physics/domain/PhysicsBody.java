package cz.empatix.physics.domain;

import cz.empatix.physics.service.CollisionListener;
import org.joml.Vector2f;

public class PhysicsBody implements Moveable {
    private final CollisionListener collisionListener;
    private final Geometry geometry;
    private final Vector2f transform;
    private final Vector2f velocity;
    private int mass;

    public PhysicsBody(final Geometry geometry) {
        this.collisionListener = new CollisionListener();
        this.geometry = geometry;
        this.transform = new Vector2f();
        this.velocity = new Vector2f();
        this.mass = geometry.getMass();

    }

    public void setMass(int mass) {
        this.mass = mass;
    }

    public int getMass() {
        return mass;
    }

    public void translate(final int x, final int y) {
        this.transform.add(x, y);
    }

    public void setTranslation(final int x, final int y) {
        this.transform.set(x, y);
    }

    public Vector2f getTranslation() {
        return transform;
    }

    public boolean isIntersecting(final PhysicsBody other) {
        return this.geometry.isIntersecting(other.geometry);
    }

    public void update(final float dt) {
        this.move(dt);
    }

    @Override
    public void move(double dt) {
        this.transform.add(this.velocity.mul((float) dt));
    }
    public void collision(final PhysicsBody body){
        collisionListener.onCollision(body);
    }

    @Override
    public Vector2f getVelocity() {
        return velocity;
    }
}
