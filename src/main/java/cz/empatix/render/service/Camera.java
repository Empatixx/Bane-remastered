package cz.empatix.render.service;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.domain.InterpolatedPosition;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector3f;

import static cz.empatix.common.handlers.WindowHandler.TARGET_HEIGHT;
import static cz.empatix.common.handlers.WindowHandler.TARGET_WIDTH;

public class Camera {
    private static float zoom = 1f;
    private static final float TWEEN = 5f;
    private final static Vector2f minDimensions = new Vector2f();
    private static final Vector2f maxDimensions = new Vector2f();
    private static final InterpolatedPosition interpolatedPosition = new InterpolatedPosition();

    // optimization so it is always one reference to shaderbindings!
    private final static Matrix4f originWithoutZoomProjection = new Matrix4f().ortho2D(
            0,
            TARGET_WIDTH,
            TARGET_HEIGHT,
            0);
    private final static Matrix4f originProjection = new Matrix4f().ortho2D(
            TARGET_WIDTH * (1 - zoom) / 2,
            TARGET_WIDTH - (TARGET_WIDTH * (1 - zoom)) / 2,
            TARGET_HEIGHT - (TARGET_HEIGHT * (1 - zoom)) / 2,
            TARGET_HEIGHT * (1 - zoom) / 2);
    private final static Matrix4f projection = new Matrix4f(originProjection);

    public static Matrix4f getProjection() {
        return projection;
    }

    public static Matrix4f getOriginProjection() {
        return originProjection;
    }

    public static Matrix4f getOriginWithoutZoomProjection() {
        return originWithoutZoomProjection;
    }

    public static void update(final float dt) {
        interpolatedPosition.update(dt);

        final Vector2f position = interpolatedPosition.getInterpolatedPosition();
        Matrix4f translate = getOriginProjection().translate(new Vector3f(-position.x(), -position.y(), 0), new Matrix4f());
        projection.set(translate);
    }

    public static void setPosition(final float x, final float y) {
        final float newX = Math.max(minDimensions.x(), Math.min(maxDimensions.x(), x));
        final float newY = Math.max(minDimensions.y(), Math.min(maxDimensions.y(), y));
        interpolatedPosition.setPosition(newX, newY);
    }

    public static void setSmoothlyPosition(final float x, final float y, final float dt) {
        float tween = TWEEN * dt;
        if(tween > 1){
            tween = 1;
        }
        final float newX = Math.max(minDimensions.x(), Math.min(maxDimensions.x(), (x - interpolatedPosition.getX()) * tween + interpolatedPosition.getX()));
        final float newY = Math.max(minDimensions.y(), Math.min(maxDimensions.y(), (y - interpolatedPosition.getY()) * tween + interpolatedPosition.getY()));
        interpolatedPosition.setPosition(newX, newY);
    }

    public static float getX() {
        return interpolatedPosition.getInterpolatedX() + TARGET_WIDTH * (1 - zoom) / 2;
    }

    public static float getY() {
        return interpolatedPosition.getInterpolatedY() + TARGET_HEIGHT * (1 - zoom) / 2;
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

    public static float getZoom() {
        return zoom;
    }

    public static float getTargetHeight() {
        return WindowHandler.TARGET_HEIGHT * zoom;
    }

    public static void zoomIn(final float zoom) {
        Camera.zoom += zoom;
        zoomProjection();
    }

    public static void zoomOut(final float zoom) {
        Camera.zoom -= zoom;
        zoomProjection();
    }

    private static void zoomProjection() {
        final Matrix4f originProjection = new Matrix4f().ortho2D(
                TARGET_WIDTH * (1 - zoom) / 2,
                TARGET_WIDTH - (TARGET_WIDTH * (1 - zoom)) / 2,
                TARGET_HEIGHT - (TARGET_HEIGHT * (1 - zoom)) / 2,
                TARGET_HEIGHT * (1 - zoom) / 2);
        Camera.originProjection.set(originProjection);
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
        maxDimensions.x = (x - maxDimensions.x - TARGET_WIDTH) * tween + maxDimensions.x;
        maxDimensions.y = (y - maxDimensions.y - TARGET_HEIGHT) * tween + maxDimensions.y;
    }

    public static void setMinDimensions(final int x, final int y) {
        minDimensions.set(x, y);
    }

    public static void setMaxDimensions(final int x, final int y) {
        maxDimensions.set(x, y);
    }
}
