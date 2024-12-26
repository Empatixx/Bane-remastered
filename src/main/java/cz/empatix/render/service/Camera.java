package cz.empatix.render.service;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.domain.InterpolatedPosition;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector3f;

import static cz.empatix.common.handlers.WindowHandler.TARGET_HEIGHT;
import static cz.empatix.common.handlers.WindowHandler.TARGET_WIDTH;

public class Camera {
    private static final float ZOOM = 1f;
    private static final float TWEEN = 5f;
    private final static Vector2f minDimensions = new Vector2f();
    private static final Vector2f maxDimensions = new Vector2f();
    private static final InterpolatedPosition interpolatedPosition = new InterpolatedPosition();

    // optimization so it is always one reference to shaderbindings!
    private final static Matrix4f PROJECTION_MATRIX = new Matrix4f().ortho(
            0,
            TARGET_WIDTH,
            TARGET_HEIGHT,
            0, -100, 100);
    private final static Matrix4f TRANSLATED_PROJECTION_MATRIX = new Matrix4f().ortho(
            0,
            TARGET_WIDTH,
            TARGET_HEIGHT,
            0, -100, 100);

    public static Matrix4f getProjection() {
        return TRANSLATED_PROJECTION_MATRIX;
    }
    public static Matrix4f getProjectionMatrix() {
        return PROJECTION_MATRIX;
    }

    public static Matrix4f getOriginProjection() {
        return PROJECTION_MATRIX;
    }

    public static void update(final float dt) {
        interpolatedPosition.update(dt);

        Matrix4f translate = getOriginProjection().translate(new Vector3f(-getX(), -getY(), 0), new Matrix4f());
        TRANSLATED_PROJECTION_MATRIX.set(translate);
    }

    public static void setPosition(final float x, final float y) {
        final float newX = Math.max(minDimensions.x(), Math.min(maxDimensions.x(), x));
        final float newY = Math.max(minDimensions.y(), Math.min(maxDimensions.y(), y));
        interpolatedPosition.setInterpolatedPosition(newX, newY);
        Camera.update(1);
    }

    public static void setSmoothlyPosition(final float x, final float y, final float dt) {
        float tween = TWEEN * dt;
        if(tween > 1){
            tween = 1;
        }
        final float newX = Math.max(minDimensions.x(), Math.min(maxDimensions.x(), (x - interpolatedPosition.getX()) * tween + interpolatedPosition.getX()));
        final float newY = Math.max(minDimensions.y(), Math.min(maxDimensions.y(), (y - interpolatedPosition.getY()) * tween + interpolatedPosition.getY()));
        interpolatedPosition.setInterpolatedPosition(newX, newY);
    }

    public static float getX() {
        return interpolatedPosition.getInterpolatedX() + TARGET_WIDTH * (1 - ZOOM) / 2;
    }

    public static float getY() {
        return interpolatedPosition.getInterpolatedY() + TARGET_HEIGHT * (1 - ZOOM) / 2;
    }

    public static Vector2f getPosition() {
        return new Vector2f(getX(), getY());
    }

    public static Vector2f centerX() {
        return new Vector2f(TARGET_WIDTH / 2, 0);
    }

    public static float getTargetWidth() {
        return WindowHandler.TARGET_WIDTH * ZOOM;
    }

    public static float getTargetHeight() {
        return WindowHandler.TARGET_HEIGHT * ZOOM;
    }

    public static void setSmoothlyMinDimensions(final int x, final int y, final float dt) {
        float tween = TWEEN * dt;
        if(tween > 1){
            tween = 1;
        }
        minDimensions.x = (x - minDimensions.x) * tween + minDimensions.x;
        minDimensions.y = (y - minDimensions.y) * tween + minDimensions.y;
    }

    public static void setSmoothlyMaxDimensions(final int x, final int y, final float dt) {
        float tween = TWEEN * dt;
        if(tween > 1){
            tween = 1;
        }
        maxDimensions.x = (x - maxDimensions.x - WindowHandler.TARGET_WIDTH) * tween + maxDimensions.x;
        maxDimensions.y = (y - maxDimensions.y - WindowHandler.TARGET_HEIGHT) * tween + maxDimensions.y;
    }

    public static void setMinDimensions(final int x, final int y) {
        minDimensions.set(x, y);
    }

    public static void setMaxDimensions(final int x, final int y) {
        maxDimensions.set(x - WindowHandler.TARGET_WIDTH, y - WindowHandler.TARGET_HEIGHT);
    }

    public static boolean isInProjection(final Vector2f position, final int width, final int height) {
        return position.x() + width > getX() && position.x() < getX() + getTargetWidth() &&
                position.y() + height > getY() && position.y() < getY() + getTargetHeight();
    }
}
