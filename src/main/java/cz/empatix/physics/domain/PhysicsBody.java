package cz.empatix.physics.domain;

import cz.empatix.physics.domain.transform.InterpolatedTransform;
import cz.empatix.physics.domain.transform.Transform;
import cz.empatix.physics.service.CollisionListener;
import org.joml.Vector2f;

public record PhysicsBody(
        CollisionListener collisionListener,
        Geometry geometry,
        Transform transform,
        Vector2f velocity,
        Vector2f maxVelocity,
        int mass
) implements Moveable {
    public static PhysicsBody createDynamic(final Geometry geometry) {
        return new PhysicsBody(
                new CollisionListener(),
                geometry,
                new InterpolatedTransform(),
                new Vector2f(),
                new Vector2f(),
                geometry.getMass()
        );
    }
    public static PhysicsBody createStatic(final Geometry geometry){
        return new PhysicsBody(
                new CollisionListener(),
                geometry,
                new InterpolatedTransform(),
                new Vector2f(),
                new Vector2f(),
                geometry.getMass()
        );
    }
    public void fixedUpdate(final float dt) {
        move(dt);
    }
    public void update(final float dt) {
        transform.update(dt);
    }
    @Override
    public void move(float dt) {
        transform.translate(this.velocity.x() * dt, this.velocity.y() * dt);
    }
    @Override
    public Vector2f getVelocity() {
        return velocity();
    }
    public void collision(final PhysicsBody body) {
        collisionListener.onCollision(body);
    }
    public boolean isIntersecting(final PhysicsBody other) {
        return this.geometry.isIntersecting(other.geometry());
    }
    public float translationX(){
        return translation().x();
    }
    public float translationY(){
        return translation().y();
    }
    public float physicsTranslationX(){
        return physicsTranslation().x();
    }
    public float physicsTranslationY(){
        return physicsTranslation().y();
    }
    public Vector2f translation(){
        return transform.getTranslationToRender();
    }
    public Vector2f physicsTranslation(){
        return transform.getTranslation();
    }

    public void setVelocity(final float x, final float y) {
        this.velocity.x = x;
        this.velocity.y = y;
    }
    public void setTranslation(final float x, final float y) {
        this.transform.setTranslation(x, y);
    }
}
