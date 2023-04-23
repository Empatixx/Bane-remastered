package cz.empatix.render.service;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.domain.InterpolatedPosition;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector3f;

import static cz.empatix.common.handlers.WindowHandler.TARGET_HEIGHT;
import static cz.empatix.common.handlers.WindowHandler.TARGET_WIDTH;

public class Camera {
    private static final float TWEEN = 0.1f;
    private static float zoom = 1;
    private final static Vector2f minDimensions = new Vector2f(0, 0);
    private static final Vector2f maxDimensions = new Vector2f(0, 0);
    private static final InterpolatedPosition interpolatedPosition = new InterpolatedPosition();

    public static Matrix4f getProjection() {
        final Vector2f position = interpolatedPosition.getInterpolatedPosition();
        return getOriginProjection().translate(new Vector3f(-position.x(), -position.y(), 0));
    }
    public static Matrix4f getOriginProjection(){
        return new Matrix4f().ortho2D(
                        TARGET_WIDTH * (1 - zoom) / 2,
                        TARGET_WIDTH - (TARGET_WIDTH * (1 - zoom)) / 2,
                        TARGET_HEIGHT - (TARGET_HEIGHT * (1 - zoom)) / 2,
                        TARGET_HEIGHT * (1 - zoom) / 2);
    }

    public static void update(final float dt) {
        interpolatedPosition.update(dt);
    }

    public static void setPosition(final float x, final float y) {
        final float newX = Math.max(minDimensions.x(), Math.min(maxDimensions.x(), x));
        final float newY = Math.max(minDimensions.y(), Math.min(maxDimensions.y(), y));
        interpolatedPosition.setPosition(newX, newY);
    }
    public static void setSmoothlyPosition(final float x, final float y) {
        final float newX = Math.max(minDimensions.x(), Math.min(maxDimensions.x(), (x - interpolatedPosition.getX()) * TWEEN + interpolatedPosition.getX()));
        final float newY = Math.max(minDimensions.y(), Math.min(maxDimensions.y(), (y - interpolatedPosition.getY()) * TWEEN + interpolatedPosition.getY()));
        interpolatedPosition.setPosition(newX, newY);
    }
    public static float getX() {
        return interpolatedPosition.getInterpolatedX();
    }

    public static float getY() {
        return interpolatedPosition.getInterpolatedY();
    }

    public static Vector2f getPosition() {
        return new Vector2f(interpolatedPosition.getInterpolatedX(), interpolatedPosition.getInterpolatedY())
                .add(TARGET_WIDTH * (1 - zoom) / 2, WindowHandler.TARGET_HEIGHT * (1 - zoom) / 2);
    }

    public static Vector2f centerX() {
        return new Vector2f(TARGET_WIDTH / 2, 0);
    }

    public static float getTargetWidth() {
        return TARGET_WIDTH * zoom;
    }

    public static float getTargetHeight() {
        return WindowHandler.TARGET_HEIGHT * zoom;
    }

    public static void zoomIn(final float zoom) {
        Camera.zoom += zoom;
    }

    public static void zoomOut(final float zoom) {
        Camera.zoom -= zoom;
    }
    public static void setMinDimensions(final float x, final float y){
        minDimensions.x = x;
        minDimensions.y = y;
    }
    public static void setMaxDimensions(final float x, final float y){
        maxDimensions.x = x - TARGET_WIDTH;
        maxDimensions.y = y - TARGET_HEIGHT;
    }

}
