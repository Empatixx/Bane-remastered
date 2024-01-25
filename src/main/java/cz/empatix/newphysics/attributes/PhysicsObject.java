package cz.empatix.newphysics.attributes;

import cz.empatix.newphysics.service.PhysicsService;
import org.dyn4j.dynamics.Body;
import org.dyn4j.geometry.Mass;
import org.dyn4j.geometry.Rotation;
import org.dyn4j.geometry.Vector2;
import org.joml.Vector2f;

public class PhysicsObject extends Body {
    private Runnable onCollision = () -> {};
    public void onCollision(final Runnable onCollision) {
        this.onCollision = onCollision;
    }
    public void onCollision() {
        onCollision.run();
    }
    public PhysicsObject() {
    }

    public Vector2f getPosition() {
        return new Vector2f((float) getTransform().getTranslationX(), (float) getTransform().getTranslationY());
    }

    public void destroy() {
        PhysicsService.getInstance().remove(this);
    }
    public void applyForce(final float x, final float y) {
        applyForce(new Vector2(x, y));
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

    public Vector2f getTranslation() {
        return new Vector2f((float) getTransform().getTranslationX(), (float) getTransform().getTranslationY());
    }

    public void setPosition(float x, float y) {
        getTransform().setTranslation(x, y);
    }

    public void setVelocity(final float x, final float y) {
        setLinearVelocity(x, y);
    }

    public float getTranslationX() {
        return (float) getTransform().getTranslationX();
    }

    public float getTranslationY() {
        return (float) getTransform().getTranslationY();
    }

    public Vector2f getVelocity() {
        return new Vector2f((float) getLinearVelocity().x, (float) getLinearVelocity().y);
    }

    public Vector2f getPhysicsTranslation() {
        return new Vector2f((float) getTransform().getTranslationX(), (float) getTransform().getTranslationY());
    }

    public void update(float dt) {

    }

    @Override
    public void rotate(double theta) {

    }

    @Override
    public void rotate(Rotation rotation) {

    }

    @Override
    public void rotate(double theta, Vector2 point) {

    }

    @Override
    public void rotate(double theta, double x, double y) {

    }

    @Override
    public void rotate(Rotation rotation, Vector2 point) {

    }

    @Override
    public void rotate(Rotation rotation, double x, double y) {

    }

    @Override
    public void rotateAboutCenter(double theta) {

    }
}
