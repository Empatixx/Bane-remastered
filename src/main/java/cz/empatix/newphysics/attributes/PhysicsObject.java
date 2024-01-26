package cz.empatix.newphysics.attributes;

import cz.empatix.newphysics.service.PhysicsEngine;
import cz.empatix.newphysics.service.PhysicsService;
import org.dyn4j.dynamics.Body;
import org.dyn4j.geometry.Vector2;
import org.joml.Vector2f;

public class PhysicsObject extends Body {
    private final static Runnable NOOP = () -> {
    };
    private Runnable onCollision = NOOP;
    private boolean active = true;
    public void setActive(boolean active) {
        this.active = active;
    }
    public boolean isActive() {
        return active;
    }

    public void onCollision(final Runnable onCollision) {
        this.onCollision = onCollision;
    }

    public void onCollision() {
        onCollision.run();
    }

    public void setPhysicsTranslation(float x, float y) {
        getTransform().setTranslation(x, y);
    }

    public Vector2f getPhysicsTranslation() {
        return new Vector2f((float) getTransform().getTranslationX(), (float) getTransform().getTranslationY());
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
        return (float) getTransform().getTranslationX();
    }

    public float getPhysicsTranslationY() {
        return (float) getTransform().getTranslationY();
    }

    public void setTranslation(float x, float y) {
        getTransform().setTranslation(x, y);
    }

    public void destroy() {
        PhysicsService.getInstance().remove(this);
    }
    public void update(final float dt){

    }
}
