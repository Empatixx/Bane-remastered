package cz.empatix.newphysics.attributes;

import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsEngine;
import cz.empatix.newphysics.service.PhysicsService;
import org.dyn4j.dynamics.Body;
import org.dyn4j.geometry.Vector2;
import org.joml.Vector2f;
import org.joml.Vector2i;

public class PhysicsObject extends Body {
    private final static Runnable NOOP = () -> {
    };
    private Runnable onCollision = NOOP;
    private CollisionType collisionType;
    private boolean remove;
    protected final Vector2i offset = new Vector2i();

    public void onCollision(final Runnable onCollision) {
        this.onCollision = onCollision;
    }

    public void onCollision() {
        onCollision.run();
    }

    public void setPhysicsTranslation(float x, float y) {
        getPreviousTransform().setTranslation(x + offset.x(), y + offset.y());
        getTransform().setTranslation(x + offset.x(), y + offset.y());
    }

    public Vector2f getPhysicsTranslation() {
        return new Vector2f(getPhysicsTranslationX(), getPhysicsTranslationY());
    }

    public void applyForce(final float x, final float y) {
        applyForce(new Vector2(x, y));
    }

    public void setVelocity(final float x, final float y) {
        getLinearVelocity().set(x, y);
    }

    public Vector2f getVelocity() {
        return new Vector2f((float) getLinearVelocity().x, (float) getLinearVelocity().y);
    }

    public float getPhysicsTranslationX() {
        return (float) getTransform().getTranslationX() - offset.x();
    }

    public float getPhysicsTranslationY() {
        return (float) getTransform().getTranslationY() - offset.y();
    }

    public void setTranslation(float x, float y) {
        getPreviousTransform().setTranslation(x + offset.x(), y + offset.y());
        getTransform().setTranslation(x + offset.x(), y + offset.y());
    }

    public void update(final float dt){

    }

    public CollisionType getType() {
        return collisionType;
    }

    public void setType(final CollisionType collisionType) {
        this.collisionType = collisionType;
    }
    public void destroy() {
        remove = true;
    }
    public boolean canRemove() {
        return remove;
    }
    public void setOffset(final int x, final int y) {
        offset.set(x, y);
    }
}
