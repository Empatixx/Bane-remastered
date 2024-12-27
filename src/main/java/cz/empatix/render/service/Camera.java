package cz.empatix.render.service;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.domain.InterpolatedPosition;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.joml.Vector3f;

import java.util.ArrayList;
import java.util.List;

import static cz.empatix.common.handlers.WindowHandler.*;
import static org.lwjgl.opengl.GL11.glViewport;

public class Camera implements ObserveSubject{
    private static final Camera INSTANCE = new Camera();

    public static Camera getInstance() {
        return INSTANCE;
    }

    private static final float TWEEN = 5f;
    private final Matrix4f projectionMatrix;
    private final Matrix4f viewMatrix;
    private final Vector2f maxPosition = new Vector2f();
    private final Vector2f minPosition = new Vector2f();
    private final Vector2i resolution = new Vector2i();
    private final InterpolatedPosition interpolatedPosition;
    private final List<Observer> observers;

    public Camera() {
        projectionMatrix = new Matrix4f().ortho(0, TARGET_WIDTH, TARGET_HEIGHT, 0, -1000, 1000);
        resolution.set(TARGET_WIDTH, TARGET_HEIGHT);
        viewMatrix = new Matrix4f();
        interpolatedPosition = new InterpolatedPosition();
        observers = new ArrayList<>();
    }

    public Matrix4f getProjectionMatrix() {
        return projectionMatrix;
    }

    public Matrix4f getViewMatrix() {
        return viewMatrix;
    }

    public void update(final float dt) {
        interpolatedPosition.update(dt);
        notifyObservers(new CameraObservePayload(CameraChangeType.VIEW));
        viewMatrix.identity().translate(-getX(), -getY(), 0);

        final Matrix4f translate = viewMatrix.translate(new Vector3f(-getX(), -getY(), 0), new Matrix4f());
        viewMatrix.set(translate);
    }

    public void setViewport(final int width, final int height) {
        resolution.set(width, height);
        glViewport(0, 0, width, height);
    }

    public void setPosition(final float x, final float y) {
        final float newX = Math.max(minPosition.x(), Math.min(maxPosition.x(), x));
        final float newY = Math.max(minPosition.y(), Math.min(maxPosition.y(), y));

        interpolatedPosition.setInterpolatedPosition(newX, newY);
        update(1);
    }

    public void setSmoothlyPosition(final float x, final float y, final float dt) {
        float tween = TWEEN * dt;
        if(tween > 1){
            tween = 1;
        }
        final float newX = Math.max(minPosition.x(), Math.min(maxPosition.x(), (x - interpolatedPosition.getX()) * tween + interpolatedPosition.getX()));
        final float newY = Math.max(minPosition.y(), Math.min(maxPosition.y(), (y - interpolatedPosition.getY()) * tween + interpolatedPosition.getY()));
        interpolatedPosition.setInterpolatedPosition(newX, newY);
    }

    public float getX() {
        return interpolatedPosition.getInterpolatedX();
    }

    public float getY() {
        return interpolatedPosition.getInterpolatedY();
    }

    public Vector2f getPosition() {
        return new Vector2f(getX(), getY());
    }

    public static Vector2f centerX() {
        return new Vector2f(TARGET_WIDTH / 2, 0);
    }

    public static float getTargetWidth() {
        return WindowHandler.TARGET_WIDTH;
    }

    public static float getTargetHeight() {
        return WindowHandler.TARGET_HEIGHT;
    }

    public void setSmoothlyMinPosition(final float x, final float y, final float dt) {
        float tween = TWEEN * dt;
        if(tween > 1){
            tween = 1;
        }
        minPosition.x = (x - minPosition.x) * tween + minPosition.x;
        minPosition.y = (y - minPosition.y) * tween + minPosition.y;
    }

    public void setSmoothlyMaxPosition(final float x, final float y, final float dt) {
        float tween = TWEEN * dt;
        if(tween > 1){
            tween = 1;
        }
        maxPosition.x = (x - maxPosition.x - WindowHandler.TARGET_WIDTH) * tween + maxPosition.x;
        maxPosition.y = (y - maxPosition.y - WindowHandler.TARGET_HEIGHT) * tween + maxPosition.y;
    }

    public void setMinPosition(final float x, final float y) {
        minPosition.set(x, y);
    }
    public void setMaxPosition(final float x, final float y) {
        maxPosition.set(x - WindowHandler.TARGET_WIDTH, y - WindowHandler.TARGET_HEIGHT);
    }

    public boolean isInProjection(final Vector2f position, final int width, final int height) {
        return position.x() + width > getX() && position.x() < getX() + getTargetWidth() &&
                position.y() + height > getY() && position.y() < getY() + getTargetHeight();
    }

    @Override
    public void addObserver(Observer observer) {
        observers.add(observer);
        observer.update(this, new CameraObservePayload(CameraChangeType.VIEW));
        observer.update(this, new CameraObservePayload(CameraChangeType.VIEWPORT));
        observer.update(this, new CameraObservePayload(CameraChangeType.PROJECTION));
    }

    @Override
    public void removeObserver(Observer observer) {
        observers.remove(observer);
    }

    @Override
    public void notifyObservers(ObservePayload payload) {
        observers.forEach(observer -> observer.update(this, payload));
    }

    public Vector2i getResolution() {
        return resolution;
    }
}
