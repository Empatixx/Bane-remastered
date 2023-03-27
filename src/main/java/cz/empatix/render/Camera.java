package cz.empatix.render;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.domain.InterpolatedPosition;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector3f;

import static cz.empatix.common.handlers.WindowHandler.*;

public class Camera {
    private static float zoom = 1;
    private static final InterpolatedPosition interpolatedPosition = new InterpolatedPosition();

    public static Matrix4f getProjection() {
        return new Matrix4f().ortho2D(
                        TARGET_WIDTH * (1 - zoom) / 2,
                        TARGET_WIDTH - (TARGET_WIDTH * (1 - zoom)) / 2,
                        TARGET_HEIGHT - (TARGET_HEIGHT * (1 - zoom)) / 2,
                        WindowHandler.TARGET_HEIGHT * (1 - zoom) / 2)
                .translate(new Vector3f(interpolatedPosition.getInterpolatedPosition(), 0));
    }

    public static void update(final float dt) {
        interpolatedPosition.update(dt);
    }

    public static void setPosition(final float x, final float y) {
        interpolatedPosition.setPosition(Camera.getX() + x,Camera.getY() +  y);
    }

    public static float getX() {
        return interpolatedPosition.getInterpolatedX();
    }

    public static float getY() {
        return interpolatedPosition.getInterpolatedY();
    }

    public static Vector2f getPosition() {
        return new Vector2f(-interpolatedPosition.getInterpolatedX(), -interpolatedPosition.getInterpolatedY())
                .sub(-TARGET_WIDTH * (1 - zoom) / 2, -WindowHandler.TARGET_HEIGHT * (1 - zoom) / 2);
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

}
