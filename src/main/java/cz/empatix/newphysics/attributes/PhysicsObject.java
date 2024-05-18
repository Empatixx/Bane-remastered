package cz.empatix.newphysics.attributes;

import cz.empatix.enemies.domain.bosses.GolemLaser;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsEngine;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.roomobjects.domain.RoomObject;
import org.dyn4j.collision.Fixture;
import org.dyn4j.dynamics.Body;
import org.dyn4j.geometry.Vector2;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.slf4j.LoggerFactory;

import java.util.function.Consumer;

public class PhysicsObject extends Body {
    private final static Consumer NOOP = r -> {
    };
    private Consumer<PhysicsObject> onCollision = NOOP;
    private Consumer<PhysicsObject> onEnter = NOOP;
    private Consumer<PhysicsObject> onLeave = NOOP;
    private CollisionType collisionType = CollisionType.NONE;
    private boolean remove;
    protected final Vector2i offset = new Vector2i();

    public void onCollision(final Consumer<PhysicsObject> onCollision) {
        this.onCollision = onCollision;
    }
    public void onEnter(final Consumer<PhysicsObject> onEnter) {
        this.onEnter = onEnter;
    }
    public void onLeave(final Consumer<PhysicsObject> onLeave) {
        this.onLeave = onLeave;
    }

    public void onCollision(final PhysicsObject physicsObject) {
        onCollision.accept(physicsObject);
    }
    public void onEnter(final PhysicsObject physicsObject) {
        onEnter.accept(physicsObject);
    }
    public void onLeave(final PhysicsObject physicsObject) {
        onLeave.accept(physicsObject);
    }

    public void setPhysicsTranslation(float x, float y) {
        setTranslation(x, y);
    }

    public Vector2f getPhysicsTranslation() {
        final Vector2f translation = new Vector2f();
        final Vector2 localCenter = getLocalCenter();
        final Vector2 worldCenter = getWorldCenter();
        translation.x = (float) (worldCenter.x - localCenter.x - offset.x());
        translation.y = (float) (worldCenter.y - localCenter.y - offset.y());
        return translation;
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
        return getPhysicsTranslation().x();
    }

    public float getPhysicsTranslationY() {
        return getPhysicsTranslation().y();
    }

    public void setTranslation(float x, float y) {
        getTransform().setTranslation(x + offset.x(), y + offset.y());
        getPreviousTransform().set(getTransform());

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
    public float getRotation() {
        return (float) getTransform().getRotationAngle();
    }

}
